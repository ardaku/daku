//! Write application logs
//!
//! This is an abstraction over the Daku API using the [log] crate (API is
//! re-exported here).
//!
//! ```rust,no_run
//! use daku::{run, api::log::{self, LevelFilter}};
//!
//! #[no_mangle]
//! unsafe extern "C" fn run() {
//!     run::start(async {
//!         log::set_max_level(LevelFilter::Debug);
//!
//!         // Queue two log messages, and print at once with a single syscall
//!         log::info("=============");
//!         log::info("Hello, world!");
//!         // Without the call to flush, messages would print on next syscall
//!         log::logger().flush();
//!     });
//! }
//! ```
//!
//! [log]: https://crates.io/crates/log

use alloc::{borrow::Cow, boxed::Box, string::ToString};
use core::mem;

use crate::{
    cmd,
    sys,
    tls::Local,
};

pub use log::*;

struct Logger;

struct State {
    channel: u32,
}

static STATE: Local<State> = Local::new(State { channel: u32::MAX });

#[inline(always)]
pub(crate) unsafe fn init(channel: u32) {
    STATE.with(|state| {
        state.channel = channel;
        set_logger_racy(&Logger).unwrap_unchecked()
    })
}

impl From<Level> for sys::Level {
    #[inline(always)]
    fn from(level: Level) -> Self {
        match level {
            Level::Trace => sys::Level::Trace,
            Level::Debug => sys::Level::Debug,
            Level::Info => sys::Level::Info,
            Level::Warn => sys::Level::Warn,
            Level::Error => sys::Level::Error,
        }
    }
}

impl Log for Logger {
    #[inline(always)]
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        const LOGSIZE: usize = mem::size_of::<sys::Log>();

        let target = record.target().as_bytes();
        let args = record.args();
        let message: Cow<'_, str> = if let Some(message) = args.as_str() {
            message.into()
        } else {
            args.to_string().into()
        };
        let log = Box::new((
            sys::Log {
                target_size: target.len()
                    | sys::Level::from(record.level()) as usize,
                target_data: target.as_ptr(),
                message_size: message.len(),
                message_data: message.as_ptr(),
            },
            message,
        ));
        let log = cmd::defer(log);
        let cmd = sys::Command {
            ready: usize::MAX, // ignored because always immediately ready
            channel: STATE.with(|state| state.channel),
            size: LOGSIZE,
            data: log.cast(),
        };

        unsafe { cmd::queue(cmd) };
    }

    #[inline(always)]
    fn flush(&self) {
        cmd::flush();
    }
}
