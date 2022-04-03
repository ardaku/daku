//! Portal for getting information about the CPU.

use std::mem::MaybeUninit;

use crate::ffi;
use crate::types::TextMut;

/// CPU architecture list.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Arch {
    /// Web Assembly
    Wasm = 0,
    /// RISC-V
    RiscV = 1,
    /// ARM
    Arm = 2,
    /// MIPS
    Mips = 3,
    /// X86 AMD/Intel
    X86 = 4,
}

/// Get the CPU architecture of the underlying system.
pub async fn arch() -> Arch {
    let ready = ffi::allocate();
    let mut out = MaybeUninit::<u32>::uninit();
    let future = unsafe {
        ffi::request_future(ffi::Command {
            portal: ffi::Portal::CpuInfo,
            ready,
            command: 0,
            data: out.as_mut_ptr().cast(),
        })
    };
    future.await;
    match unsafe { out.assume_init() } {
        0 => Arch::Wasm,
        1 => Arch::RiscV,
        2 => Arch::Arm,
        3 => Arch::Mips,
        4 => Arch::X86,
        _ => Arch::Wasm,
    }
}

/// CPU pointer/reference/word width
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Width {
    /// 16-bit or lower
    Cpu16 = 0,
    /// 32-bit
    Cpu32 = 1,
    /// 64-bit
    Cpu64 = 2,
    /// 128-bit
    Cpu128 = 3,
}

/// Get the CPU pointer/reference/word width
pub async fn width() -> Width {
    let ready = ffi::allocate();
    let mut out = MaybeUninit::<u32>::uninit();
    let future = unsafe {
        ffi::request_future(ffi::Command {
            portal: ffi::Portal::CpuInfo,
            ready,
            command: 1,
            data: out.as_mut_ptr().cast(),
        })
    };
    future.await;
    match unsafe { out.assume_init() } {
        0 => Width::Cpu16,
        1 => Width::Cpu32,
        2 => Width::Cpu64,
        3 => Width::Cpu128,
        _ => Width::Cpu32,
    }
}

/// Get the CPU extensions (contents may vary, so shoudn't be relied on).
pub async fn extensions() -> String {
    // Get required length
    let ready = ffi::allocate();
    let mut out = TextMut { size: 0, data: std::ptr::null_mut() };
    let future = unsafe {
        ffi::request_future(ffi::Command {
            portal: ffi::Portal::CpuInfo,
            ready,
            command: 2,
            data: <*mut TextMut>::cast(&mut out),
        })
    };
    future.await;
    // Get the string
    let ready = ffi::allocate();
    let mut string = String::with_capacity(out.size);
    let mut out = TextMut { size: string.capacity(), data: string.as_mut_ptr() };
    let future = unsafe {
        ffi::request_future(ffi::Command {
            portal: ffi::Portal::CpuInfo,
            ready,
            command: 2,
            data: <*mut TextMut>::cast(&mut out),
        })
    };
    future.await;
    string
}
