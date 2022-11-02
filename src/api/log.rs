//! Write application logs
//!
//! This is an abstraction over the Daku API using the [log] crate.
//!
//! ```rust,no_run
//! use log::Level;
//! use daku::{run, api::log as daku_log};
//!
//! run::spawn(async {
//!     daku_log::init(Level::Debug);
//!
//!     // Queue two log messages, and print at once with a single syscall
//!     log::info("=============");
//!     log::info("Hello, world!");
//!     // Without the call to flush, messages would print on next syscall
//!     log::logger().flush();
//! });
//! ```
//!
//! [log]: https://crates.io/crates/log

use alloc::{borrow::Cow, boxed::Box, string::ToString};
use core::mem;

use log::LevelFilter;

use crate::{
    cmd, portal,
    sys::{Command, Level, Log},
    tls::Local,
};

struct Logger;

struct State {
    channel: u32,
}

static STATE: Local<State> = Local::new(State { channel: u32::MAX });

impl From<log::Level> for Level {
    #[inline(always)]
    fn from(level: log::Level) -> Self {
        match level {
            log::Level::Trace => Level::Trace,
            log::Level::Debug => Level::Debug,
            log::Level::Info => Level::Info,
            log::Level::Warn => Level::Warn,
            log::Level::Error => Level::Error,
        }
    }
}

impl log::Log for Logger {
    #[inline(always)]
    fn enabled(&self, _metadata: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        const LOGSIZE: usize = mem::size_of::<Log>();

        let target = record.target().as_bytes();
        let args = record.args();
        let message: Cow<'_, str> = if let Some(message) = args.as_str() {
            message.into()
        } else {
            args.to_string().into()
        };
        let log = Box::new((
            Log {
                target_size: target.len()
                    | Level::from(record.level()) as usize,
                target_data: target.as_ptr(),
                message_size: message.len(),
                message_data: message.as_ptr(),
            },
            message,
        ));
        let log = cmd::defer(log);
        let cmd = Command {
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

/// Set logger to Daku.
///
/// # Panics
/// If logger is already set.
///
/// ```rust
/// use daku::api::log::{self, Level};
///
/// log::init(None); // Don't log anything
/// log::init(Level::Error); // Only log errors
/// log::init(Level::Warn); // Only log errors and warnings
/// log::init(Level::Info); // Only log errors, warnings and information
/// log::init(Level::Debug); // Log everything except trace logs
/// log::init(Level::Trace); // Log everything
/// ```
#[inline(always)]
pub fn init(level: impl Into<Option<log::Level>>) {
    STATE.with(|state| {
        state.channel = portal::log();
        log::set_max_level(
            level
                .into()
                .map(|level| level.to_level_filter())
                .unwrap_or(LevelFilter::Off),
        );
        unsafe { log::set_logger_racy(&Logger).unwrap() }
    })
}
