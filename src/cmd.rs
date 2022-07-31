//! Task local command queue

use crate::sys::{self, Command};
use alloc::vec::Vec;

// Task local command queue
static mut QUEUE: Vec<Command> = Vec::new();
// Ready queue
static mut READY: Vec<usize> = Vec::new();

/// Queue a command
pub unsafe fn queue<const N: usize>(commands: [Command; N]) {
    let queue = &mut QUEUE;
    queue.extend(commands);
}

/// Flush commands
pub fn flush() {
    let queue = unsafe { &mut QUEUE };
    unsafe { sys::ar(queue.len(), queue.as_ptr()) };
    // FIXME: Waking
    queue.clear();
}

/// Queue and flush
pub unsafe fn until<const N: usize>(commands: [Command; N]) {
    queue(commands);
    flush();
}
