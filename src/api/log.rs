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

use crate::{cmd, sys::{Log, Level, Command}};
use core::{mem, sync::atomic::{AtomicBool, Ordering::Relaxed}, fmt::Write};
use alloc::{string::String, format, borrow::Cow};
use log::LevelFilter;

static ONCE: AtomicBool = AtomicBool::new(true);

struct Logger;

impl From<log::Level> for Level {
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
    fn enabled(&self, _metadata: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        let level = record.level();
        let target = record.target();
        let args = record.args();

        let message: Cow<'_, str> = if let Some(message) = args.as_str() {
            message.into()
        } else {
            format!("{}", args).into()
        };
        let length = message.len();

        let logsize = mem::size_of::<Log>();
        let mut log = String::with_capacity(logsize + target.len());
        let send = Log {
            size: length,
            data: message.as_ptr(),
            level: level.into(),
            target: (),
        };
        let send: *const Log = &send;
        let send: &[u8] = unsafe { core::slice::from_raw_parts(send.cast(), logsize) };
        log.extend(['\0'; mem::size_of::<Log>()]);
        write!(&mut log, "{target}").ok();
        let mut log = log.into_bytes();
        for (l, s) in log.iter_mut().zip(send.into_iter().cloned()) {
            *l = s;
        }
        
        unsafe { 
            cmd::queue([Command {
                ready: usize::MAX, // ignored because always immediately ready
                channel: 0, // FIXME: Get global log channel
                size: log.len(),
                data: log.as_ptr().cast(),
            }]);
        }
    }

    // Data is never buffered
    fn flush(&self) {
        cmd::flush();
    }
}

/// Set logger to Daku.  Doesn't do anything if logger already set.
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
pub async fn init(level: impl Into<Option<log::Level>>) {
    if ONCE.swap(false, Relaxed) {
        log::set_max_level(level.into().map(|level| level.to_level_filter()).unwrap_or(LevelFilter::Off));
        unsafe { log::set_logger_racy(&Logger).unwrap() };
    }
}
