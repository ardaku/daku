//! Write messages to the log.
//!
//! # Usage
//! These are macros only so that they can accept variadic arguments.
//! Here's a readable version of the macro definitions:
//!
//! ```rust
//! pub async fn info!(target: Target, message: ... impl Display)
//! pub async fn warn!(target: Target, message: ... impl Display)
//! pub async fn error!(target: Target, message: ... impl Display)
//! pub async fn debug!(target: Target, message: ... impl Display)
//! ```
//!
//! Simple example:
//!
//! ```rust
//! use daku::log;
//!
//! let a = 15;
//! async {
//!     let log = Target::new("Log").await;
//!
//!     log::info!(log, "a = ", a, ", isn't that cool‽").await;
//! }
//! ```
//!
//! Making a target:
//!
//! ```rust
//! use daku::log::{self, Target};
//!
//! // Target for a networking task
//! let net = Target::new("Net").await;
//!
//! // Fire and forget (won't print until the event loop)
//! let _ = log::info!(net, "Hello, world!");
//! ```

use crate::{ffi, types::Text};

mod inner {
    use super::*;

    #[repr(u32)]
    #[derive(Debug, Copy, Clone)]
    #[doc(hidden)]
    pub enum Level {
        Info = 0,
        Debug = 1,
        Error = 2,
        Warn = 3,
    }

    #[inline(always)]
    #[doc(hidden)]
    pub async fn _log(level: Level, target: Target, text: String) {
        let mut text = Text::from(text.as_str());
        let future = unsafe {
            ffi::request_stream(ffi::Command {
                portal: ffi::Portal::Log,
                ready: target.0,
                command: level as u32,
                data: <*mut Text>::cast(&mut text),
            })
        };
        future.await;
    }
}

/// A log target.
#[derive(Debug, Copy, Clone)]
pub struct Target(ffi::Ready);

impl Target {
    /// Create a new log target.
    pub async fn new(name: &str) -> Self {
        let ready = ffi::allocate();
        let mut text = Text::from(name);
        let future = unsafe {
            ffi::request_stream(ffi::Command {
                portal: ffi::Portal::Log,
                ready,
                command: 4,
                data: <*mut Text>::cast(&mut text),
            })
        };
        future.await;

        Target(ready)
    }
}

#[doc(hidden)]
pub use self::inner::{
    Level::{Debug as _Debug, Error as _Error, Info as _Info, Warn as _Warn},
    _log,
};

#[doc(hidden)]
#[macro_export]
macro_rules! _log {
    ($level:expr, $target:expr, $($message:expr),+) => ( {
        use std::fmt::Write;

        let mut temp_buffer = String::new();

        $(
            let _ = temp_buffer.write_fmt(format_args!("{}", $message));
        )+

        $crate::log::_log(
            $crate::log::_Info,
            $target,
            temp_buffer,
        )
    } );
}

#[doc(hidden)]
#[macro_export]
macro_rules! _info {
    ($target:expr, $($message:expr),+) => ( {
        $crate::_log!($crate::log::_Info, $target, $($message),+)
    } );
    ($($data:expr)+) => (
        info!(MAIN: $($data)+)
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! _warn {
    ($target:expr, $($message:expr),+) => ( {
        $crate::_log!($crate::log::_Warn, $target, $($message),+)
    } );
    ($($data:expr)+) => (
        info!(MAIN: $($data)+)
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! _debug {
    ($target:expr, $($message:expr),+) => ( {
        $crate::_log!($crate::log::_Debug, $target, $($message),+)
    } );
    ($($data:expr)+) => (
        info!(MAIN: $($data)+)
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! _error {
    ($target:expr, $($message:expr),+) => ( {
        $crate::_log!($crate::log::_Error, $target, $($message),+)
    } );
    ($($data:expr)+) => (
        info!(MAIN: $($data)+)
    );
}

#[doc(inline)]
pub use crate::{
    _debug as debug, _error as error, _info as info, _warn as warn,
};
