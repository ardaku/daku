#[repr(C, packed)]
pub(crate) struct Text {
    /// Number of bytes
    pub(crate) size: usize,
    /// UTF-8 String
    pub(crate) data: *const u8,
}

impl From<&str> for Text {
    fn from(string_slice: &str) -> Self {
        let bytes = string_slice.as_bytes();
        let size = bytes.len();
        let data = bytes.as_ptr();

        Self { size, data }
    }
}

#[repr(C, packed)]
pub(crate) struct TextMut {
    /// Number of bytes
    pub(crate) size: usize,
    /// UTF-8 String
    pub(crate) data: *mut u8,
}
