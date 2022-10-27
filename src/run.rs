//! Safe API for spawning asynchronous tasks

use core::{
    future::Future,
    pin::Pin,
    ptr,
    task::{Context, RawWaker, RawWakerVTable, Waker},
};

use crate::{cmd, tls::Local};

const VTABLE: RawWakerVTable =
    RawWakerVTable::new(clone, wake_any, wake_any, drop);

static ASLEEP: Local<bool> = Local::new(false);

/// Sleep until wake
#[inline(always)]
pub fn sleep() {
    ASLEEP.with(|asleep| {
        if *asleep {
            cmd::flush();
        } else {
            *asleep = true;
        }
    })
}

/// Wake
#[inline(always)]
pub fn wake() {
    ASLEEP.with(|asleep| *asleep = false)
}

unsafe fn clone(ptr: *const ()) -> RawWaker {
    RawWaker::new(ptr, &VTABLE)
}

unsafe fn wake_any(_: *const ()) {
    wake()
}

unsafe fn drop(_: *const ()) {}

/// Execute a future
#[inline(always)]
pub fn block_on<F: Future<Output = ()>>(future: F) {
    let waker = unsafe { Waker::from_raw(clone(ptr::null())) };
    let mut future = future;
    let mut future = unsafe { Pin::new_unchecked(&mut future) };
    let mut cx = Context::from_waker(&waker);

    while future.as_mut().poll(&mut cx).is_pending() {
        sleep();
    }

    cmd::flush();
}
