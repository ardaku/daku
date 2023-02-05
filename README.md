# Daku v1.0.0-beta.0
Daku is a system interface API similar to WASI with different goals.

Since Daku is currently in beta, things may change based on feedback but large
changes are unlikely.

The [daku specification](https://ardaku.org/daku) is currently in draft, so some
remnants of the old spec may remain in this README and other files in the
repository for now.

## Goals
 - Async-first
 - Early stability of base API
 - As simple as possible
 - Reduced syscalls
 - Channel-based
 - Security via portals
 - Non-POSIX-y
 - Prioritize multimedia support

## Types

### Timestamp
```rust
#[repr(transparent)]
struct Timestamp {
    /// The number of TAI microseconds since Jan 1 00:00:00.000_000, year 0 in
    /// [ISO 8601:2004](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// This gives about a range of ±292_470 years since year 0.
    ///
    /// This differs from Unix time in 3 ways:
    ///  - Epoch is 0000-01-01T00:00:00.000_000 TAI instead of
    ///    1970-01-01T00:00:00.000_000 UTC
    ///  - Precision is microseconds instead of seconds
    ///  - TAI not UTC, meaning that a leap second gets representation, rather
    ///    than being represented as repeat of previous second as it would be in
    ///    unix time.
    micros: i64,
}
```

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
