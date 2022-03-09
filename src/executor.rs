use std::{
    future::Future,
    sync::Arc,
    task::{
        Context,
        Poll::{Pending, Ready},
        Wake, Waker,
    },
};

use super::{ar, CHANNEL_WAKERS, COMMANDS, DONE};

static mut SHOULD_WAKE: bool = false;

/// Send queued commands, yield to the executor, and wake the tasks that need waking.
fn run() {
    unsafe {
        // If the waker has awoken, early return and run executor again.
        if SHOULD_WAKE {
            SHOULD_WAKE = false;
            return;
        }

        // Call into system and wait
        DONE.set_len(ar(COMMANDS.len(), COMMANDS.as_ptr(), DONE.as_mut_ptr()));
        for done in DONE.iter().cloned() {
            if let Some(waker) = CHANNEL_WAKERS[done].take() {
                waker.wake();
            }
        }
    }
}

struct Woke;

impl Wake for Woke {
    #[inline(always)]
    fn wake(self: Arc<Self>) {
        unsafe { SHOULD_WAKE = true }
    }

    #[inline(always)]
    fn wake_by_ref(self: &Arc<Self>) {
        unsafe { SHOULD_WAKE = true }
    }
}

/// Create a waker
fn waker() -> Waker {
    Arc::new(Woke).into()
}

/// Block on a future
pub fn block_on<T, F: Future<Output = T>>(future: F) -> T {
    let waker = waker();
    let mut future = Box::pin(future);

    loop {
        match future.as_mut().poll(&mut Context::from_waker(&waker)) {
            Pending => run(),
            Ready(ret) => break ret,
        }
    }
}
