# Daku v1.0.0-alpha.0
Daku is a system interface API similar to WASI with different goals.

Since it's currently alpha, things may change but changes are somewhat unlikely.
Alpha, beta and pre-release stages will not last very long (once there is an
implementation it will be fully stabilized).

## Goals
 - Async-First
 - Immediately Stable
 - Simple
 - Reduced Syscalls
 - Channel-Based
 - Security via Portals
 - Anti-POSIX
 - Full Multimedia Support

# API
The daku api exports a single function `ar()`:

```wat
(import "daku" "ar" (func $event
    (param $size i32)   ;; List[Command].size
    (param $data i32)   ;; List[Command].reference
    (param $done i32)   ;; List[Ready].reference (capacity=open channel count)
    (result i32)        ;; List[Ready].size (size truncated at 16384)
))
```

The function queues a number of asynchronous tasks, passed as a list (first two
parameters).  When any asynchronous task completes, the function returns the
new size of `$done` and it's contents are overwritten.

## `Command`
```rust
#[repr(C, packed)]
struct Command {
    /// Channel to send a message on, 0 to open channel
    channel: u32,
    /// The new channel id
    ///  - Nop: Keep same as `channel`
    ///  - Modify: Non-Zero ID ≠ `channel`
    ///  - Discard: Zero (when `channel` is 0, exit)
    new: u32,
    /// Which command to run
    which: u32,
    /// Data to send and/or receive over channel
    data: *mut (), 
}
```

## Types

### Time
```rust
#[repr(C, packed)]
struct Time {
    /// Range: 0 ~ 23 (always UTC, localized only for display)
    hour: u8,
    /// Range: 0 ~ 59
    minute: u8,
    /// Range: 0 ~ 60_999 (can represent leap seconds)
    millis: u16,
}
```

### Date
```rust
#[repr(C, packed)]
struct Date {
    /// Range: 0 ~ 65_535
    year: u16,
    /// Range: 1 ~ 12
    month: u8,
    /// Range: (of week: 1 ~ 7) << 5 | (of month: 1 ~ 31)
    day: u8,
}
```

### DateTime
```rust
#[repr(C, packed)]
struct DateTime {
    date: Date,
    time: Time,
}
```

### List
```rust
#[repr(C, packed)]
struct List<T> {
    size: u32,
    data: *mut T,
}
```

### Text
```rust
#[repr(C, packed)]
struct Text {
    /// Number of bytes
    size: u32,
    /// UTF-8 String
    data: *mut u8,
}
```

### Overclock
```rust
#[repr(C, packed)]
enum Overclock {
    Off = 0,
}
```

### TimeZone
```rust
#[repr(C, u32)]
enum TimeDesignation {
    Utc = 0,
}

#[repr(C, packed)]
struct TimeAdjustment {
    /// Time to replace
    when: DateTime,
    /// New time
    new: DateTime,
}

#[repr(C, packed)]
struct LeapSecond {
    /// Which year
    year: i16,
    /// Always the last day of the month
    month: u8,
    /// Direction: Either -1 or +1
    delta: i8,
}

#[repr(C, packed)]
struct TimeZone {
    /// Name of TimeZone (abbreviated, null terminated unless size 6)
    designation: TimeDesignation,
    /// List of adjustments made in this timezone
    deltas: List<TimeAdjustment>,
    /// Replace UTC jan 1 00:00:00:000 year 0 with Local time adjustments
    /// 
    /// This must be equivalent to all of the adjustments in `deltas` plus
    /// any daylight savings time modifications.
    offset: DateTime,
    /// List of leap seconds
    leap_seconds: List<LeapSecond>,
    /// Sum of leap seconds
    leap: i16,
    /// Is daylight savings time?
    is_dst: bool,
    /// Reserved for future use
    reserved: u8,
}
```

### Lang
```rust
#[repr(C, u32)]
enum Language {
    /// English United States
    EnUS = u32::from_ne_bytes(*b"enUS"),
    /// English Great Britain
    EnGB = u32::from_ne_bytes(*b"enGB"),
    /// Esperanto
    EoXX = u32::from_ne_bytes(*b"eoXX"),
}

#[repr(C, packed)]
struct Lang {
    /// List of languages in order of user preference (0 is most preferred)
    list: List<Language>,
}
```

## Commands
Commands can be added in new versions of the Daku spec, and can be deprecated,
but never removed.  Because of strict backwards-compatibility their spec should
aim to be as mathematical as possible rather than engineered.  Command ids are
always allocated starting from 0, then incrementing by 1.

When you open a channel, you will receive data whenever it's ready.  Some data
doesn't change, so you can close the channel after the first receive.

Receiving commands must send a capacity in a `size` field for variable length
items, and if the capacity is not met, the `size` field gets set to the required
length and data set to NULL.

 0. Get CPU Architecture `recv: u32`
    ```rust
    #[repr(C, u32)]
    enum Arch {
        Wasm = 0,
        RiscV = 1,
        Arm = 2,
        Mips = 3,
        X86 = 4,
        Unknown = u32::MAX,
    }
    ```
 1. Get CPU Reference Size `recv: u32`
    ```rust
    #[repr(C, u32)]
    enum RefSize {
        Ref16 = 0,
        Ref32 = 1,
        Ref64 = 2,
        Ref128 = 3,
        Unknown = u32::MAX,
    }
    ```
 2. Query CPU Extensions description `recv: Text`.
    Results should never be relied on for app features.
 3. Get overclocking setting `recv: Overclock`
 4. Set overclocking setting `send: Overclock`
 5. Get current UTC DateTime `recv: DateTime`
 6. Get current Time Zone `recv: TimeZone`
 7. Get Kernel Type `recv: Platform`
    ```rust
    #[repr(C, u32)]
    enum Platform {
        Linux = 0,
        Bsd = 1,
        Windows = 2,
        MacOS = 3,
        Ios = 4,
        Android = 5,
        Nintendo = 6,
        Xbox = 7,
        PlayStation = 8,
        Fuchsia = 9,
        Redox = 10,
        Novusk = 11,
        Unknown = u32::MAX,
    }
    ```
 8. Get OS Name `recv: Text`
 9. Get (Desktop) Environment `recv: Environment`
    ```rust
    #[repr(C, u32)]
    enum Environment {
        Gnome = 0,
        Windows = 1,
        Lxde = 2,
        Openbox = 3,
        Mate = 4,
        Xfce = 5,
        Kde = 6,
        Cinnamon = 7,
        I3 = 8,
        Aqua = 9,
        Ios = 10,
        Android = 11,
        WebBrowser = 12,
        Console = 13,
        Ubuntu = 14,
        Ermine = 15,
        Orbital = 16,
       _quantii_env = 17,
        Unknown = u32::MAX,
    }
    ```
 10. Get Wasm Runtime `recv: Runtime`
     ```rust
     #[repr(C, u32)]
     enum Runtime {
         Wasmtime = 0,
         Wasmer = 1,
         Wasmi = 2,
         Wari = 3,
         Unknown = u32::MAX,
     }
     ```
 11. Get User:Username `recv: Text`
 12. Get User:FullName `recv: Text`
 13. Get User:Language `recv: Lang`
 14. Set User:Username `send: Text`
 15. Set User:FullName `send: Text`
 16. Set User:Language `send: Lang`
 17. Spawn child process / task `send: Task`.
     Becomes "ready" when task sends a message, and shared_memory is written
     then sent back on next call to `ar()`.  If another task is sent, replaces
     the child task.  Close the channel to stop the child task.
     ```rust
     #[repr(C, packed)]
     struct Task {
         /// A WebAssembly file
         spawn_wasm_len: u32,
         spawn_wasm_data: *mut u8,
         /// Share memory with child task (mmap-like)
         shared_memory_size: u32,
         shared_memory_data: *mut u8,
     }
     ```
 18. Connect to parent channel `send: Message`
     ```rust
     #[repr(C, packed)]
     struct Message {
         /// Share memory with parent task (mmap-like)
         shared_memory_size: u32,
         shared_memory_data: *mut u8,
     }
     ```
     Once connected, send an empty (null) command to wake parent task and
     transfer message.
 19. Log `send: Log`
     ```rust
     #[repr(C, packed)]
     struct Print {
         /// Log index (which log to print to)
         which: u16,
         /// Which levels to print to (bit flags):
         ///  - 0 DEBUG
         ///  - 1 INFO
         ///  - 2 WARNING
         ///  - 3 ERROR
         /// Which levels are enabled
         ///  - 4 DEBUG
         ///  - 5 INFO
         ///  - 6 WARNING
         ///  - 7 ERROR
         level: u8,
         /// Set name for this log index instead of printing
         set_name: bool,
         /// Text to print (on it's own line)
         text: Text,
     }
     ```
 20. Debug: `recv: Text`
 21. Set window actions "toolbar" `send: WindowActions, recv: WindowActionEvent`
     ```rust
     #[repr(C, packed)]
     struct WindowActions {
         /// Number of actions (between 0 and 3)
         len: u32,
         /// List of icons for window actions
         [Icon; 3],
     }
     ```

     ```rust
     #[repr(C, packed)]
     struct WindowActionEvent {
         /// Action number (0, 1 or 2)
         action: u32,
         _reserved_a: u32,
         _reserved_b: u64,
     }
     ```
 22. Set tab navigation "toolbar" `send: NavigationActions, recv: NavigationActionEvent`
     ```rust
     #[repr(C, packed)]
     struct NavigationActions {
         /// Number of actions (between 0 and 3)
         len: u32,
         /// List of icons for window actions
         [Icon; 3],
     }
     ```

     ```rust
     #[repr(C, packed)]
     struct WindowActionEvent {
         /// Action number (0, 1 or 2)
         action: u32,
         _reserved_a: u32,
         _reserved_b: u64,
     }
     ```
 23. Set HUD menu / keyboard shortcuts, and receive events `send: Menu, recv: u32`
     ```rust
     #[repr(C, packed)]
     struct Menu {
	 options: List<Action>,
     }

     #[repr(C, packed)]
     struct Action {
         /// "Copy", "Select All", etc.
         name: Text,
         /// "Copy Selection To The Clipboard", etc.
         description: Text,
         /// Additional keywords for searching ["Clone", "Edit"]
         search_tags: List<Text>,
         /// Shortcut Modifier (255 for none)
         modifier: u8,
         /// Shortcut Key (255 for none)
         key: u8,
         /// True for on press, false for on release
         pressed: bool,
         /// Option should be disabled?
         disabled: bool,
         /// Only display when mode is equal (default mode is 0)
         mode: u32,
     }
     ```
 24. Set vertical tabs hamburger menu
 25. Set search enabled/disabled for each mode
 26. Get keycode input
 27. Get text input
 28. Get cursor input
 29. Get controller input
 30. Set GPIO Interrupts
 31. Set GPIO State
 32. Record audio (microphone)
 33. Play audio (speakers)
 34. Record video (webcam)
 35. Play video (framebuffer)
 36. Serve over HTTP
 37. Connect over HTTP
 38. Bluetooth device
 39. Bluetooth connect
 40. Persistent Storage Open File
 41. Persistent Storage Share File
 42. GPU Compute
 43. GPU Command `send: GpuCommand, recv: u32`
     ```rust
     #[repr(C, packed)]
     enum GpuCommandId {
         /// () => (count: u32)
         GetNumberOfDisplays = 0u32,
         /// (index: u32) => (width: u16, height: u16)
         GetDisplaySize = 1,
         /// (width: u16, height: u16) => (framebuffer: u32)
         AllocateFramebuffer = 2,
         /// (size: u32) => (buffer: u32)
         AllocateBufferF32 = 3,
         /// (width: u16, height: u16) => (framebuffer: u32)
         AllocateTexture = 4,
         /// (size: u32) => (buffer: u32)
         AllocateBufferI32 = 5,
     }

     #[repr(C, packed)]
     struct GpuCommand {
         id: GpuCommandId,
         value: u32,
         map_size: u32,
         map_data: *mut (),
     }
     ```
 44. Switch Mode `send: u32`
