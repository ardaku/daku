//! Daku primitive FFI types for Rust, and asynchronous request function

use core::mem;

/// 32-bit floating point value
#[allow(nonstandard_style)]
pub type num = f32;

/// 32-bit integer value
#[allow(nonstandard_style)]
pub type int = u32;

/// 32-bit integer length value
#[allow(nonstandard_style)]
pub type len = usize;

/// 64-bit integer value
#[allow(nonstandard_style)]
pub type long = u64;

/// 16-bit integer value
#[allow(nonstandard_style)]
pub type half = u16;

/// 8-bit integer value
#[allow(nonstandard_style)]
pub type byte = u8;

/// Optional pointer value
#[allow(nonstandard_style)]
pub type opt<T> = *mut T;

/// List of elements of type `T`
///
/// This is always safe to construct with [`From`] from slice of `&[T]` or
/// `&mut [T]`, but when using, must make sure to keep the slice around at least
/// until the `List<T>` is passed over FFI.
#[repr(C, packed)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct List<T> {
    /// Number of elements pointed to at `addr`
    pub size: len,
    /// Packed list of `size` elements
    pub addr: opt<T>,
}

impl<T> From<List<T>> for [u8; 8] {
    #[inline(always)]
    fn from(list: List<T>) -> Self {
        // SAFETY: wasm32 int and ptr can be safely turned into a byte array
        // since endianness is set, and there's no strict provenance on wasm
        unsafe { mem::transmute(list) }
    }
}

impl<T> From<&[T]> for List<T> {
    fn from(slice: &[T]) -> Self {
        List {
            size: slice.len(),
            addr: slice.as_ptr().cast_mut(),
        }
    }
}

impl<T> From<&mut [T]> for List<T> {
    fn from(slice: &mut [T]) -> Self {
        List {
            size: slice.len(),
            addr: slice.as_mut_ptr(),
        }
    }
}

/// List of bytes
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Buffer(pub List<byte>);

impl From<Buffer> for [u8; 8] {
    #[inline(always)]
    fn from(buf: Buffer) -> Self {
        List::from(buf).into()
    }
}

impl From<Buffer> for List<byte> {
    #[inline(always)]
    fn from(buf: Buffer) -> Self {
        buf.0
    }
}

impl From<List<byte>> for Buffer {
    #[inline(always)]
    fn from(buf: List<byte>) -> Self {
        Self(buf)
    }
}

/// Command sent from the guest to the host
#[repr(C, packed)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Command {
    /// (in) Which channel is being used, (out) number of channels opened
    pub channel: len,
    /// Capacity of `buffer`
    pub capacity: len,
    /// Data buffer
    pub buffer: Buffer,
}

#[link(wasm_import_module = "daku")]
extern "C" {
    /// Asynchronous request FFI call
    pub fn ar(size: len, data: opt<Command>);
    /// Unstable debug call
    pub fn dbg(size: len, text: opt<byte>);
}
