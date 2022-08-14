//! Task local command queue

use crate::{portal, sys::{self, Command}};
use alloc::vec::Vec;
use core::pin::Pin;
use core::future::Future;
use core::task::{Poll::{self, Pending, Ready}, Context, Waker, RawWaker, RawWakerVTable};
use core::ptr;

// Task local command queue
static mut QUEUE: Vec<Command> = Vec::new();
// Pending wakers
static mut PENDING: Vec<Option<Waker>> = Vec::new();

const FAKE_RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    fake_raw_waker,
    dont,
    dont,
    dont,
);
        
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

/// Queue a command
pub unsafe fn queue<const N: usize>(commands: [Command; N]) {
    let queue = &mut QUEUE;
    queue.extend(commands);
}

/// Flush commands
pub fn flush() {
    let queue = unsafe { &mut QUEUE };
    unsafe {
        for ready in portal::ready_list(sys::ar(queue.len(), queue.as_ptr())) {
            if let Some(waker) = PENDING[*ready].take() {
                waker.wake();
            }
        }
    }
    queue.clear();
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
    until([Command { ready, channel, size, data }]);
    // Wait until ready
    Request(ready).await
}

// An asynchronous request
struct Request(usize);

impl Future for Request {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = unsafe { &mut PENDING[self.0] };

        if let Some(ref mut waker) = waker {
            *waker = cx.waker().clone();
            Pending
        } else {
            Ready(())
        }
    }
}
