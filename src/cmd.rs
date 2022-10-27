//! Task local command queue

use alloc::{boxed::Box, vec::Vec};
use core::{
    any::Any,
    future::Future,
    pin::Pin,
    ptr,
    task::{
        Context,
        Poll::{self, Pending, Ready},
        RawWaker, RawWakerVTable, Waker,
    },
};

use crate::{
    portal,
    sys::{self, Command},
    tls::Local,
};

struct State {
    // Task local command queue
    queue: Vec<Command>,
    // Pending wakers
    pending: Vec<Option<Waker>>,
    // Pending drops
    drops: Vec<Box<dyn Any>>,
}

static STATE: Local<State> = Local::new(State {
    queue: Vec::new(),
    pending: Vec::new(),
    drops: Vec::new(),
});

const FAKE_RAW_WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(fake_raw_waker, dont, dont, dont);

const unsafe fn dont(_: *const ()) {}

const unsafe fn fake_raw_waker(ptr: *const ()) -> RawWaker {
    RawWaker::new(ptr, &FAKE_RAW_WAKER_VTABLE)
}

/// Add a mock waker to be replaced
fn add_waker() -> usize {
    STATE.with(|state| {
        let waker = unsafe { Waker::from_raw(fake_raw_waker(ptr::null())) };

        if let Some(index) = state.pending.iter().position(|w| w.is_none()) {
            state.pending[index] = Some(waker);
            index
        } else {
            let index = state.pending.len();
            state.pending.push(Some(waker));
            index
        }
    })
}

/// Defer drop(s) until next flush
pub fn defer<T: 'static, const N: usize>(items: [T; N]) {
    STATE.with(|state| {
        state
            .drops
            .extend(items.into_iter().map(|x| -> Box<dyn Any> { Box::new(x) }));
    })
}

/// Queue a command
///
/// # Safety
/// Commands must be valid according to the Daku spec.  Failure to pass in valid
/// `Command` struct may cause undefined behavior.
pub unsafe fn queue<const N: usize>(commands: [Command; N]) {
    STATE.with(|state| state.queue.extend(commands));
}

/// Flush commands
pub fn flush() {
    STATE.with(|state| {
        for ready in unsafe {
            portal::ready_list(sys::ar(state.queue.len(), state.queue.as_ptr()))
        } {
            if *ready == usize::MAX {
                // Special value to ignore
                continue;
            }
            if let Some(waker) = state.pending[*ready].take() {
                waker.wake();
            }
        }
        state.queue.clear();
        state.drops.clear();
    });
}

/// Queue and flush
///
/// # Safety
/// Commands must be valid according to the Daku spec.  Failure to pass in valid
/// `Command` struct may cause undefined behavior.
pub unsafe fn until<const N: usize>(commands: [Command; N]) {
    queue(commands);
    flush();
}

/// Send a command on a channel
///
/// # Safety
/// `data` must be valid according to the Daku spec.  Failure to pass in valid
/// `data` may cause undefined behavior.
pub async unsafe fn execute<T>(channel: u32, data: &T) {
    // Data can't move since it's borrowed
    let data: *const T = data;
    let data = data.cast();
    let ready = add_waker();
    let size = core::mem::size_of::<T>();
    // Queue command and flush
    until([Command {
        ready,
        channel,
        size,
        data,
    }]);
    // Wait until ready
    Request(ready).await
}

// An asynchronous request
struct Request(usize);

impl Future for Request {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        STATE.with(|state| {
            if let Some(ref mut waker) = state.pending[self.0] {
                *waker = cx.waker().clone();
                Pending
            } else {
                Ready(())
            }
        })
    }
}
