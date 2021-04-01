use core::cell::UnsafeCell;
use core::ops::Deref;

use crate::spinlock::Spinlock;

pub struct SyncLazy<T, F = fn() -> T> {
    data: UnsafeCell<Option<T>>,
    init: Spinlock<F>,
}

impl<T, F> SyncLazy<T, F> {
    pub const fn new(f: F) -> Self {
        Self { data: UnsafeCell::new(None), init: Spinlock::new(f) }
    }
}

unsafe impl<T> Sync for SyncLazy<T> {}

impl<T> Deref for SyncLazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let data = unsafe { &*self.data.get() };
        if data.is_none() {
            let init = self.init.lock();

            if data.is_none() {
                unsafe { *self.data.get() = Some(init()) }
            }
        }

        data.as_ref().unwrap()
    }
}
