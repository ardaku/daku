//! Safe API for spawning asynchronous tasks

use alloc::{sync::Arc, boxed::Box, task::Wake};
use core::sync::atomic::{AtomicUsize, Ordering::{Relaxed}};
use core::task::{Context, Poll::{self, Ready}, RawWakerVTable, Waker};
use core::future::Future;
use core::pin::Pin;

static NEXT_TASK: AtomicUsize = AtomicUsize::new(0);

#[repr(C)]
struct TaskLocal {
    future: Box<dyn Future<Output = ()> + Send + 'static>,
}

impl Wake for TaskLocal {
    #[inline(always)]
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let task_id: *const TaskLocal = &**self;
        let task_id = task_id as usize;
    }
}

extern "C" fn executor(data: *mut ()) {
    let task_local: Box<TaskLocal> = unsafe { Box::from_raw(data.cast()) };
    let task_local: Arc<TaskLocal> = Arc::new(*task_local);
}

/// Spawn a task that may or may not run on another thread.
pub fn spawn<F: Future<Output = ()> + Send + 'static>(fut: impl Into<Box<F>>) {
    // Put the future on the heap
    let future = fut.into();
    // Get next task id
    let task_id = NEXT_TASK.fetch_add(1, Relaxed);

    let task_local = Arc::into_raw(Arc::new(TaskLocal {
        future,
    }));


}

/// Get the current task handle.
pub async fn current() -> Task {
    Task(Current.await)
}

/// Task handle
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Task(pub(crate) usize);

struct Current;

impl Future for Current {
    type Output = usize;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Copied from https://doc.rust-lang.org/src/core/task/wake.rs.html#15-24
        //
        // Removable once https://github.com/rust-lang/rust/issues/87021 resolves
        pub struct RawWaker {
            data: *const (),
            vtable: &'static RawWakerVTable,
        }

        let waker: *const Waker = cx.waker();
        let waker: *const RawWaker = waker.cast();

        let task_id = unsafe { (*waker).data } as usize;

        Ready(task_id)
    }
}
