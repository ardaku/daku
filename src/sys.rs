//! FFI bindings to the Daku API.

/// Portal IDs
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Portal {
    /// Task spawning API
    Spawn = 0,
    /// Blocking task spawning API
    SpawnBlocking = 1,
    /// Logging API (stdout/printf)
    Log = 2,
    /// Developer command API (stdin/scanf)
    Prompt = 3,
    /// MPMC Channel API
    Channel = 4,
}

/// Channel Zero Command
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Connect {
    /// The capacity of the ready list
    pub ready_capacity: usize,
    /// Reference to uninitialized ready list
    pub ready_data: *mut usize,
    /// The number of new portals
    pub portals_size: usize,
    /// in: List of new portal IDs - out: List of new portal channel IDs
    pub portals_data: *mut u32,
}

/// A queued command
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Command {
    /// Ready index for when command completes
    pub ready: usize,
    /// Channel id to use
    pub channel: u32,
    /// Data buffer size
    pub size: usize,
    /// Data buffer reference
    pub data: *const (),
}

#[link(wasm_import_module = "daku")]
extern "C" {
    /// Unsafe asynchronous request FFI call
    pub fn ar(size: usize, data: *const Command) -> usize;
}

/// Log level, pretty much copied from
/// [`log::Level`](https://docs.rs/log/0.4.17/log/enum.Level.html).
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Level {
    /// The “trace” level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace = 0,
    /// The “debug” level.
    ///
    /// Designates lower priority information.
    Debug = 1,
    /// The “info” level.
    ///
    /// Designates useful information.
    Info = 2,
    /// The “warn” level.
    ///
    /// Designates hazardous situations.
    Warn = 3,
    /// The “error” level.
    ///
    /// Designates very serious errors.
    Error = 4,
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
    /// Text size
    pub size: usize,
    /// Text bytes
    pub data: *const u8,
    /// Log level
    pub level: Level,
    /// Target bytes
    pub target: (),
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
