//! FFI bindings to the Daku API.

/// Portal IDs
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Portal {
    /// Logging API (stdout/printf)
    Log = 0x00,
    /// Developer command API (stdin/scanf)
    Prompt = 0x01,
    ///
    Fetch = 0x02,
    ///
    Serve = 0x03,
    ///
    Speakers = 0x04,
    ///
    Microphone = 0x05,
    ///
    Screen = 0x06,
    ///
    Camera = 0x07,
    ///
    Window = 0x08,
    ///
    Spawn = 0x09,
    /// Set user information API (username, display name, localization)
    User = 0x0A,
    /// Get user information API (username, display name, localization)
    Preferences = 0x0B,
    /// Create new users, settings for all users
    System = 0x0C,
    /// Get system information and settings
    About = 0x0D,
    ///
    File = 0x0E,
    ///
    Hid = 0x0F,
    ///
    Timer = 0x10,
    ///
    Clock = 0x11,
    ///
    Gpu = 0x12,
    ///
    Location = 0x13,
}

/// Channel Zero Command
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Connect {
    /// The number of new portals
    pub portals_size: usize,
    /// in: List of new portal IDs - out: List of new portal channel IDs
    pub portals_data: *mut u32,
    /// The capacity of the ready list
    pub ready_capacity: usize,
    /// Reference to uninitialized ready list
    pub ready_data: *mut usize,
}

/// A queued command
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Command {
    /// Data buffer size
    pub size: usize,
    /// Data buffer reference
    pub data: *const (),
    /// Channel id to use
    pub channel: u32,
    /// Ready index for when command completes
    pub ready: usize,
}

#[link(wasm_import_module = "daku")]
extern "C" {
    /// Unsafe asynchronous request FFI call
    pub fn ar(size: usize, data: *const Command) -> usize;
    /// Unstable debug call
    pub fn dbg(size: usize, text: *const u8);
}

/// Log level, pretty much copied from
/// [`log::Level`](https://docs.rs/log/0.4.17/log/enum.Level.html).
#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum Level {
    /// The “trap” level.
    ///
    /// Trigger a trap without panicking.
    Fatal = 0,
    /// The “error” level.
    ///
    /// Designates very serious errors.
    Error = 1,
    /// The “warn” level.
    ///
    /// Designates hazardous situations.
    Warn = 2,
    /// The “info” level.
    ///
    /// Designates useful information.
    Info = 3,
    /// The “debug” level.
    ///
    /// Designates lower priority information.
    Debug = 4,
    /// The “trace” level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace = 5,
}

/// Portal Log Command
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Log {
    /// Target size
    pub target_size: u16,
    /// Log level
    pub level: Level,
    /// Target bytes
    pub target_data: *const u8,
    /// Message size
    pub message_size: usize,
    /// Message bytes
    pub message_data: *const u8,
}

/// UTF-8 text
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Text {
    /// Length of string
    pub size: usize,
    /// UTF-8 String
    pub data: *mut u8,
}

/// Developer console prompt
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Prompt {
    /// Capacity (in/out)
    pub capacity: *mut usize,
    /// Text (in/out)
    pub text: *mut Text,
}
