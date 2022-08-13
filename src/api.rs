//! Stable and safe portal APIs
//!
//! Each module provides a safe API for the portal it's named after.  Portals
//! have to be enabled by features of the same name.

#[cfg(feature = "log")]
pub mod log;

#[cfg(feature = "prompt")]
pub mod prompt;
