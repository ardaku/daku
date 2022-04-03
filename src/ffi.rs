use std::{
    cell::Cell,
    fmt,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

#[repr(u32)]
pub(crate) enum Portal {
    CpuInfo = 0,
    Log = 4,
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub(crate) struct Ready(usize);

impl fmt::Debug for Ready {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ready")
    }
}

#[repr(C, packed)]
pub(crate) struct Command {
    /// Which portal to use
    pub(crate) portal: Portal,
    /// Ready index for when command completes
    pub(crate) ready: Ready,
    /// Command ID
    pub(crate) command: u32,
    /// Data buffer to memory-map
    pub(crate) data: *mut (),
}

struct Global {
    /// Command queue
    commands: Cell<Vec<Command>>,
    /// Ready list
    ready: Cell<Vec<usize>>,
    /// Ready state
    state: Cell<Vec<bool>>,
    /// Unused indices for `state`
    discard: Cell<Vec<Ready>>,
    /// Whether should re-wake.
    should_wake: Cell<bool>,
}

impl Global {
    fn new() -> Self {
        Self {
            commands: Cell::new(Vec::new()),
            ready: Cell::new(Vec::with_capacity(32)),
            state: Cell::new(Vec::new()),
            discard: Cell::new(Vec::new()),
            should_wake: Cell::new(false),
        }
    }
}

thread_local! { static GLOBAL: Global = Global::new(); }

// Daku async request ffi
#[link(wasm_import_module = "daku")]
extern "C" {
    fn ar(
        cmd_size: usize,
        cmd_data: *const Command,
        ready_size: usize,
        ready_data: *mut usize,
    ) -> usize;
}

struct RequestStream(Ready);

impl Future for RequestStream {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let is_ready = GLOBAL.with(|ctx| {
            let state = ctx.state.take();
            let is_ready = state[self.0 .0];
            ctx.state.set(state);
            is_ready
        });

        if is_ready {
            GLOBAL.with(|ctx| {
                let mut state = ctx.state.take();
                state[self.0 .0] = false;
                ctx.state.set(state);
                Poll::Ready(())
            })
        } else {
            Poll::Pending
        }
    }
}

struct RequestFuture(Ready);

impl Future for RequestFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let is_ready = GLOBAL.with(|ctx| {
            let state = ctx.state.take();
            let is_ready = state[self.0 .0];
            ctx.state.set(state);
            is_ready
        });

        if is_ready {
            GLOBAL.with(|ctx| {
                let mut discard = ctx.discard.take();
                discard.push(self.0);
                ctx.discard.set(discard);
                Poll::Ready(())
            })
        } else {
            Poll::Pending
        }
    }
}

/// Send command to `.await`, ready index must already be allocated.
///
/// Unlike `request_stream()`, the ready index is de-allocated on completion.
pub(crate) async unsafe fn request_future(command: Command) {
    let ready = command.ready;
    GLOBAL.with(|ctx| {
        let mut commands = ctx.commands.take();
        commands.push(command);
        ctx.commands.set(commands);
    });
    RequestFuture(ready).await
}

/// Send command to `.await`, ready index must already be allocated.
///
/// Unlike `request_future()`, the ready index isn't de-allocated on completion.
pub(crate) async unsafe fn request_stream(command: Command) {
    let ready = command.ready;
    GLOBAL.with(|ctx| {
        let mut commands = ctx.commands.take();
        commands.push(command);
        ctx.commands.set(commands);
    });
    RequestStream(ready).await
}

/// Allocate a ready index.
pub(crate) fn allocate() -> Ready {
    GLOBAL.with(|ctx| {
        let mut discard = ctx.discard.take();
        let mut state = ctx.state.take();
        let discarded = discard.pop();

        let ready = if let Some(discarded) = discarded {
            state[discarded.0] = false;
            discarded
        } else {
            let ready = state.len();
            state.push(false);
            Ready(ready)
        };

        ctx.discard.set(discard);
        ctx.state.set(state);

        ready
    })
}

#[inline(always)]
fn sleep() {
    GLOBAL.with(|ctx| unsafe {
        let commands = ctx.commands.take();
        let mut ready = ctx.ready.take();

        let new_len = ar(
            commands.len(),
            commands.as_ptr(),
            ready.capacity(),
            ready.as_mut_ptr(),
        );
        ready.set_len(new_len);

        ctx.commands.set(commands);
        ctx.ready.set(ready);
    });
}

#[inline(always)]
fn respond() {
    sleep();
    GLOBAL.with(|ctx| {
        let ready_list = ctx.ready.take();
        let mut ready_state = ctx.state.take();

        for ready in ready_list.iter().cloned() {
            ready_state[ready] = true;
        }

        ctx.ready.set(ready_list);
        ctx.state.set(ready_state);
    });
}

struct Woke;

impl Wake for Woke {
    #[inline(always)]
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    #[inline(always)]
    fn wake_by_ref(self: &Arc<Self>) {
        GLOBAL.with(|ctx| ctx.should_wake.set(true));
    }
}

/// Create a waker
#[inline(always)]
fn waker() -> Waker {
    Arc::new(Woke).into()
}

#[inline(always)]
fn run() {
    // If the waker has awoken, early return and run executor again.
    if GLOBAL.with(|ctx| ctx.should_wake.replace(false)) {
        return;
    }

    // Call into system and wait
    respond();
}

/// Block on a future and return its output.
pub fn block_on<T, F: Future<Output = T>>(future: F) -> T {
    let waker = waker();
    let mut future = Box::pin(future);

    loop {
        match future.as_mut().poll(&mut Context::from_waker(&waker)) {
            Poll::Pending => run(),
            Poll::Ready(ret) => break ret,
        }
    }
}
