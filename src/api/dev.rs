//! Read lines from developer console

use alloc::string::String;
use core::mem;
use crate::sys::{Dev, Text};

/// Read a line, appending it to the provided buffer (not including the newline
/// character).
///
/// If the capacity of the string is sufficient this function will only take one
/// syscall, but if it's not it will require two.  Commands can never be more
/// than 65_536 bytes (size of one WebAssembly page).
pub async fn read_line(buf: &mut String) {
    let mut buffer = String::new();
    mem::swap(&mut buffer, buf);

    // Get raw parts and forget in order to leak memory
    let mut capacity = buffer.capacity();
    let mut size = buffer.len();
    let mut data = buffer.as_mut_ptr();
    mem::forget(buffer);

    // Build a Text type
    let mut text = Text { size, data };
    let mut new_capacity = capacity;
    let dev = Dev {
        text: &mut text,
        capacity: &mut new_capacity,
    };

    // Run command FIXME

    if capacity != new_capacity {
        // Not enough space!
        let mut buffer = unsafe {
            String::from_raw_parts(text.data, text.size, capacity)
        };
        buffer.reserve(new_capacity - capacity);
        capacity = buffer.capacity();
        size = buffer.len();
        data = buffer.as_mut_ptr();
        mem::forget(buffer);
        text.size = size;
        text.data = data;
        new_capacity = capacity;
        let dev = Dev {
            text: &mut text,
            capacity: &mut new_capacity,
        };

        // Re-run command FIXME

        assert_eq!(capacity, new_capacity);
    }
        
    *buf = unsafe {
        String::from_raw_parts(text.data, text.size, capacity)
    };
}
