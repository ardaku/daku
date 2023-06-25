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

use alloc::{boxed::Box, string::String};
use core::{fmt::Write, mem};

pub use log::*;

use crate::{cmd, sys, tls::Local};

const LOGSIZE: usize = mem::size_of::<sys::Log>();

static STATE: Local<State> = Local::new(State { channel: u32::MAX });

struct Logger;

struct State {
    channel: u32,
}

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
        let target = record.target().as_bytes();
        let args = record.args();
        let mut message = String::new();
        message.push(char::from(record.level() as u8));
        write!(&mut message, "{args}").ok();

        let log = Box::new((
            sys::Log {
                target: sys::Text {
                    size: target.len(),
                    addr: target.as_ptr() as usize,
                },
                message: sys::Text {
                    size: message.len(),
                    addr: message.as_ptr() as usize,
                },
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

/// Logs a message at the fail level.
///
/// Triggers a guest trap.
///
/// # Examples
///
/// ```
/// use daku::log::fail;
///
/// fail("Subsystem", "Unrecoverable error!");
/// unreachable!()
/// ```
pub fn fail(target: impl AsRef<str>, message: impl AsRef<str>) {
    let target = target.as_ref().as_bytes();
    let msg = message.as_ref();
    let mut message = String::new();
    message.push(char::from(sys::Level::Fail as u8));
    write!(&mut message, "{msg}").ok();

    let log = Box::new((
        sys::Log {
            target: sys::Text {
                size: target.len(),
                addr: target.as_ptr() as usize,
            },
            message: sys::Text {
                size: message.len(),
                addr: message.as_ptr() as usize,
            },
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

    unsafe { cmd::until(cmd) };
}
