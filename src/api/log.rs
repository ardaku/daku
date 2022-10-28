//! Write application logs
//!
//! This is an abstraction over the Daku API using the [log] crate.
//!
//! ```rust,no_run
//! use log::Level;
//! use daku::{run, api::log as daku_log};
//!
//! run::spawn(async {
//!     daku_log::init(Level::Debug).await;
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

use alloc::{borrow::Cow, string::ToString, vec::Vec};
use core::{mem, slice};

use log::LevelFilter;

use crate::{
    cmd, portal,
    sys::{Command, Level, Log},
    tls::Local,
};

static CHANNEL: Local<u32> = Local::new(u32::MAX);

struct Logger;

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
        let mut log: Vec<u8> = Vec::with_capacity(LOGSIZE + target.len());
        let args = record.args();
        let message: Cow<'_, str> = if let Some(message) = args.as_str() {
            message.into()
        } else {
            args.to_string().into()
        };
        let send = Log {
            size: message.len(),
            data: message.as_ptr(),
            level: record.level().into(),
            target: (),
        };
        let send: *const Log = &send;
        let send: &[u8] =
            unsafe { slice::from_raw_parts(send.cast(), LOGSIZE) };

        log.extend(send);
        log.extend(target);

        let cmd = Command {
            ready: usize::MAX, // ignored because always immediately ready
            channel: CHANNEL.with(|channel| *channel),
            size: log.len(),
            data: log.as_ptr().cast(),
        };

        unsafe {
            cmd::queue([cmd]);
        }

        // Defer dropping of command data until flush
        cmd::defer([log]);
        cmd::defer([message]);
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
    CHANNEL.with(|channel| {
        *channel = portal::log();
        log::set_max_level(
            level
                .into()
                .map(|level| level.to_level_filter())
                .unwrap_or(LevelFilter::Off),
        );
        unsafe { log::set_logger_racy(&Logger).unwrap() }
    })
}
