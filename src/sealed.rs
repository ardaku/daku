pub trait Addr<T> {
    fn as_usize(&self) -> usize;
}

impl<T> Addr<T> for *mut T {
    #[inline(always)]
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl<T> Addr<T> for *const T {
    #[inline(always)]
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

pub trait Str {
    type Address: Addr<u8>;

    fn len(&self) -> usize;
    fn to_addr(self) -> Self::Address;
}

impl Str for &str {
    type Address = *const u8;

    #[inline(always)]
    fn len(&self) -> usize {
        str::len(self)
    }

    #[inline(always)]
    fn to_addr(self) -> Self::Address {
        self.as_ptr()
    }
}

impl Str for &mut str {
    type Address = *mut u8;

    #[inline(always)]
    fn len(&self) -> usize {
        str::len(self)
    }

    #[inline(always)]
    fn to_addr(self) -> Self::Address {
        self.as_mut_ptr()
    }
}

pub trait Float {
    fn clean(self) -> Self;
}

impl Float for f32 {
    #[inline(always)]
    fn clean(self) -> Self {
        if self.is_nan() {
            Self::from_bits(u32::MAX)
        } else {
            self
        }
    }
}
