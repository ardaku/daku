#[repr(C, packed)]
pub(crate) struct Text {
    /// Number of bytes
    size: u32,
    /// UTF-8 String
    data: *const u8,
}

impl From<&str> for Text {
    fn from(string_slice: &str) -> Self {
        let bytes = string_slice.as_bytes();
        let size = u32::from_ne_bytes(bytes.len().to_ne_bytes());
        let data = bytes.as_ptr();

        Self { size, data }
    }
}
