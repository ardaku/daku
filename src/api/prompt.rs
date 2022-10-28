//! Read lines from developer console

use alloc::string::String;

use crate::{
    cmd, portal,
    sys::{Prompt, Text},
};

/// Read a line, appending it to the provided buffer (not including the newline
/// character).
///
/// If the capacity of the string is sufficient this function will only take one
/// syscall, but if it's not it will require two.  Commands can never be more
/// than 65_536 bytes (size of one WebAssembly page).
#[inline(never)]
pub async fn read_line(buf: &mut String) {
    let channel = portal::prompt();

    // Run command
    let mut capacity = buf.capacity();
    let mut text = Text {
        size: buf.len(),
        data: buf.as_mut_ptr(),
    };
    let prompt = Prompt {
        text: &mut text,
        capacity: &mut capacity,
    };
    unsafe { cmd::execute(channel, &prompt).await };

    let additional = capacity.saturating_sub(buf.capacity());
    if additional != 0 {
        buf.reserve(additional);
        // Re-run command with new values
        unsafe {
            text.data = buf.as_mut_ptr();
            cmd::execute(channel, &prompt).await
        };
    }

    unsafe { buf.as_mut_vec().set_len(text.size) }
}
