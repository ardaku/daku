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
    sys::{self, Command, dbg},
};

// Task local command queue
static mut QUEUE: Vec<Command> = Vec::new();
// Pending wakers
static mut PENDING: Vec<Option<Waker>> = Vec::new();
// Pending drops
static mut DROPS: Vec<Box<dyn Any>> = Vec::new();

const FAKE_RAW_WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(fake_raw_waker, dont, dont, dont);

const unsafe fn dont(_: *const ()) {}

const unsafe fn fake_raw_waker(ptr: *const ()) -> RawWaker {
    RawWaker::new(ptr, &FAKE_RAW_WAKER_VTABLE)
}

/// Add a mock waker to be replaced
fn add_waker() -> usize {
    unsafe {
        let waker = Waker::from_raw(fake_raw_waker(ptr::null()));

        if let Some(index) = PENDING.iter().position(|w| w.is_none()) {
            PENDING[index] = Some(waker);
            index
        } else {
            let index = PENDING.len();
            PENDING.push(Some(waker));
            index
        }
    }
}

/// Defer drop(s) until next flush
pub fn defer<T: 'static, const N: usize>(items: [T; N]) {
    unsafe {
        let drops = &mut DROPS;
        drops
            .extend(items.into_iter().map(|x| -> Box<dyn Any> { Box::new(x) }));
    }
}

/// Queue a command
pub unsafe fn queue<const N: usize>(commands: [Command; N]) {
    let queue = &mut QUEUE;
    queue.extend(commands);
}

/// Flush commands
pub fn flush() {
    let queue = unsafe { &mut QUEUE };
    let drops = unsafe { &mut DROPS };
    unsafe {
        for ready in portal::ready_list(sys::ar(queue.len(), queue.as_ptr())) {
            if let Some(waker) = PENDING[*ready].take() {
                waker.wake();
            }
        }
    }
    queue.clear();
    drops.clear();
}

/// Queue and flush
pub unsafe fn until<const N: usize>(commands: [Command; N]) {
    queue(commands);
    flush();
}

/// Send a command on a channel
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
        unsafe {
            let text = "Poll Start";
            dbg(text.len(), text.as_ptr());
        }
        
        let waker = unsafe { &mut PENDING[self.0] };

        unsafe {
            let text = "Poll IND";
            dbg(text.len(), text.as_ptr());
        }

        if let Some(ref mut waker) = waker {
            *waker = cx.waker().clone();
            Pending
        } else {
            Ready(())
        }
    }
}
