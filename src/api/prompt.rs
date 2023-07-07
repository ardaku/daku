//! Read lines from developer console

use alloc::string::String;

use crate::{cmd, sys, tls::Local};

struct State {
    channel: u32,
}

static STATE: Local<State> = Local::new(State { channel: u32::MAX });

#[inline(always)]
pub(crate) unsafe fn init(channel: u32) {
    STATE.with(|state| {
        state.channel = channel;
    })
}

/// Read a line, appending it to the provided buffer (not including the newline
/// character).
///
/// If the capacity of the string is sufficient this function will only take one
/// syscall, but if it's not it will require two.  Commands can never be more
/// than 65_536 bytes (size of one WebAssembly page).
#[inline(never)]
pub async fn read_line(buf: &mut String) {
    let channel = STATE.with(|state| state.channel);

    // Run command
    let mut capacity = buf.capacity();
    let mut text = sys::Text::new(buf.as_mut_str());
    let prompt = sys::Prompt {
        capacity: &mut capacity,
        text: &mut text,
    };
    unsafe { cmd::execute(channel, &prompt).await };

    let additional = capacity.saturating_sub(buf.capacity());
    if additional != 0 {
        buf.reserve(additional);
        // Re-run command with new values
        unsafe {
            text.set_addr(buf.as_mut_ptr());
            cmd::execute(channel, &prompt).await;
        }
    }

    unsafe { buf.as_mut_vec().set_len(text.len()) };
}
