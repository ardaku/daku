//! Task local command queue

use crate::{portal, sys::{self, Command}};
use alloc::vec::Vec;
use core::pin::Pin;
use core::future::Future;
use core::task::{Poll::{self, Pending, Ready}, Context, Waker};

// Task local command queue
static mut QUEUE: Vec<Command> = Vec::new();
// Pending wakers
// FIXME: Probably should be a hashmap with a custom minimal hasher
static mut PENDING: Vec<(usize, Waker)> = Vec::new();

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
            for i in (0..PENDING.len()).rev() {
                if PENDING[i].0 == *ready {
                    let (_, waker) = PENDING.remove(i);
                    waker.wake();
                }
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
    let ready = data as usize;
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
        unsafe {
            for (id, waker) in PENDING.iter_mut() {
                if *id == self.0 {
                    *waker = cx.waker().clone();
                    return Pending;
                }
            }
        }
        Ready(())
    }
}
