//! Safe API for spawning asynchronous tasks

use alloc::{boxed::Box, sync::Arc, task::Wake};
use core::{
    future::Future,
    task::{Context, Waker},
};

use crate::cmd;

static mut ASLEEP: bool = false;

/// Sleep until wake
pub fn sleep() {
    unsafe {
        if ASLEEP {
            cmd::flush();
        } else {
            ASLEEP = true;
        }
    }
}

/// Wake
pub fn wake() {
    unsafe { ASLEEP = false };
}

/// Execute a future
pub fn block_on<F: Future<Output = ()>>(future: F) {
    struct Executor;

    impl Wake for Executor {
        fn wake(self: Arc<Self>) {
            wake();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            wake();
        }
    }

    let waker: Waker = Arc::new(Executor).into();
    let mut future = Box::pin(future);
    let mut cx = Context::from_waker(&waker);

    while future.as_mut().poll(&mut cx).is_pending() {
        sleep();
    }
    cmd::flush();
}
