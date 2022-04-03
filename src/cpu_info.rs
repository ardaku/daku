//! Portal for getting information about the CPU.

use std::mem::MaybeUninit;

use crate::ffi;

/// CPU architecture list.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
