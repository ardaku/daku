//! Rust crate to interface with the [Daku API](https://github.com/ardaku/daku).
//! This crate only works/is sound on wasm32 platforms supporting the daku api.
//!
//! Functions are officially stabilized as they are added to this crate.
//!
//! In order for the async functions to be functional, you must call `run()` in
//! the async executor's main loop.

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/daku"
)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

static mut COMMANDS: Vec<Command> = Vec::new();
static mut DONE: Vec<usize> = Vec::new();
static mut NEXT_CHANNEL: usize = 0;
static mut NEXT_CHANNEL_TRASH: Vec<usize> = Vec::new();
static mut CHANNEL_WAKERS: Vec<Option<Waker>> = Vec::new();

// Daku async request ffi
#[link(wasm_import_module = "daku")]
extern "C" {
    fn ar(size: usize, data: *const Command, done: *mut usize) -> usize;
}

fn next_channel() -> usize {
    unsafe {
        if let Some(chan) = NEXT_CHANNEL_TRASH.pop() {
            chan
        } else {
            let chan = NEXT_CHANNEL;
            if NEXT_CHANNEL > DONE.capacity() {
                DONE.reserve(1);
            }
            NEXT_CHANNEL += 1;
            chan
        }
    }
}

fn drop_channel(chan: usize) {
    unsafe {
        NEXT_CHANNEL_TRASH.push(chan);
        COMMANDS.push(Command {
            channel: chan,
            new: 0,
            which: 0,
            data: &mut (),
        });
    }
}

struct Channel(usize);

impl Channel {
    fn new() -> Self {
        Self(next_channel())
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        drop_channel(self.0)
    }
}

/// Send queued commands, yield to the executor, and wake the tasks that need waking.
pub fn run() {
    unsafe {
        DONE.set_len(ar(COMMANDS.len(), COMMANDS.as_ptr(), DONE.as_mut_ptr()));
        for done in DONE.iter().cloned() {
            if let Some(waker) = CHANNEL_WAKERS[done].take() {
                waker.wake();
            }
        }
    }
}

struct Request(usize);

impl Future for Request {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let index = self.0;

        unsafe {
            CHANNEL_WAKERS.resize((index + 1).max(CHANNEL_WAKERS.len()), None);

            for (i, done) in DONE.iter().enumerate() {
                if *done == index {
                    DONE.swap_remove(i);
                    return Poll::Ready(());
                }
            }

            if CHANNEL_WAKERS[index].is_none() {
                CHANNEL_WAKERS[index] = Some(cx.waker().clone());
            }
        }

        Poll::Pending
    }
}

async fn request(command: Command) {
    let channel_id = command.new;
    unsafe {
        COMMANDS.push(command);
    }
    Request(channel_id).await;
}

#[repr(C, packed)]
struct Command {
    /// Channel to send a message on, 0 to open channel
    channel: usize,
    /// The new channel id
    ///  - Nop: Keep same as `channel`
    ///  - Modify: Non-Zero ID ≠ `channel`
    ///  - Discard: Zero (when `channel` is 0, exit)
    new: usize,
    /// Which command to run
    which: u32,
    /// Data to send and/or receive over channel
    data: *mut (),
}

/// CPU architecture list.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Arch {
    /// Web Assembly
    Wasm = 0,
    /// RISC-V
    RiscV = 1,
    /// ARM
    Arm = 2,
    /// MIPS
    Mips = 3,
    /// X86 AMD/Intel
    X86 = 4,
}

/// Get the CPU architecture of the underlying system.
pub async fn arch() -> Arch {
    let channel = Channel::new();
    let mut out = 0u32;
    request(Command {
        channel: 0,
        new: channel.0,
        which: 0,
        data: <*mut _>::cast(&mut out),
    })
    .await;
    match out {
        0 => Arch::Wasm,
        1 => Arch::RiscV,
        2 => Arch::Arm,
        3 => Arch::Mips,
        4 => Arch::X86,
        _ => Arch::Wasm,
    }
}
