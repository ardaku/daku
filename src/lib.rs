//! Rust crate to interface with the [Daku API](https://github.com/ardaku/daku).
//! This crate only works/is sound on wasm32 platforms supporting the daku api.
//!
//! Functions are officially stabilized as they are added to this crate.
//!
//! Each module gives access to a safe portal API, so there is a module for each
//! portal.
//!
//! This crate supports joining syscalls together using the Daku command queue.
//! If you use `join!()` on two futures from this crate, it will combine the
//! two syscalls into one.  Some futures have a second syscall (for instance, if
//! they require allocation).  They also work with joining, so any number of
//! them joined together will result in two syscalls, so long as they become
//! ready at the same time.

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/daku"
)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

#[cfg(not(all(
    target_arch = "wasm32",
    target_endian = "little",
    target_env = "",
    target_family = "wasm",
    target_os = "daku",
    target_pointer_width = "32",
    target_vendor = "unknown",
)))]
compile_error!("Target is not wasm32-daku");

mod ffi;
mod types;

pub mod cpu_info;
pub mod log;

pub use ffi::block_on;
