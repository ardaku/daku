//! FFI bindings to the Daku API.

/// Portal IDs
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Portal {
    /// Logging API (stdout/printf)
    Log = 0,
    /// Developer command API (stdin/scanf)
    Prompt = 1,
    /// Set user information API (username, display name)
    Account = 2,
    /// Get user information API (username, display name)
    User = 3,
    /// Set system information API (system nickname, hostname)
    System = 4,
    /// Get system information API (system nickname, hostname)
    Host = 5,
    /// Set hardware features API (overclock, hardware time)
    Hardware = 6,
    /// Get hardware features API (cpu / gpu specs)
    Platform = 7,
    /// Task spawning API
    Spawn = 8,
    /// Blocking task spawning API
    SpawnBlocking = 9,
    /// MPMC Channel API
    Channel = 10,
    /// Account API (create / delete users)
    Admin = 11,
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
#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum Level {
    /// The “trace” level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace = 0b0000_0000_0000_0000_0000_0000_0000_0000,
    /// The “debug” level.
    ///
    /// Designates lower priority information.
    Debug = 0b0010_0000_0000_0000_0000_0000_0000_0000,
    /// The “info” level.
    ///
    /// Designates useful information.
    Info = 0b0100_0000_0000_0000_0000_0000_0000_0000,
    /// The “warn” level.
    ///
    /// Designates hazardous situations.
    Warn = 0b0110_0000_0000_0000_0000_0000_0000_0000,
    /// The “error” level.
    ///
    /// Designates very serious errors.
    Error = 0b1000_0000_0000_0000_0000_0000_0000_0000,
    /// The “trap” level.
    ///
    /// Trigger a trap without panicking.
    Trap = 0b1010_0000_0000_0000_0000_0000_0000_0000,
    /// Bit Mask.
    Mask = 0b1110_0000_0000_0000_0000_0000_0000_0000,
}

/// Spawn a task with state
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Spawn {
    /// Task's asynchronous function (calls to `ar()` act as `.await` points)
    pub func: extern "C" fn(data: *mut ()),
    /// State to pass to asynchronous task function
    pub data: *mut (),
}

/// Portal Log Command
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Log {
    /// Target size / log level
    pub target_size: usize,
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
    /// Text (in/out)
    pub text: *mut Text,
    /// Capacity (in/out)
    pub capacity: *mut usize,
}
