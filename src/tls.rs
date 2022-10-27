//! Task local storage

use core::cell::UnsafeCell;

/// Similar to [`LocalKey`](https://doc.rust-lang.org/std/thread/struct.LocalKey.html)
/// but wrapping a [cell](https://doc.rust-lang.org/std/cell/index.html).
#[derive(Debug)]
pub struct Local<T> {
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Local<T> {}

impl<T> Local<T> {
    /// Create new task local storage
    pub const fn new(data: T) -> Self {
        let data = UnsafeCell::new(data);

        Self { data }
    }

    /// Similar to
    /// [`LocalKey::with_borrow_mut()`](https://doc.rust-lang.org/std/thread/struct.LocalKey.html#method.with_borrow_mut)
    #[inline(always)]
    pub fn with<R>(&'static self, f: impl FnOnce(&mut T) -> R) -> R {
        let data = unsafe { &mut *self.data.get() };

        f(data)
    }
}
