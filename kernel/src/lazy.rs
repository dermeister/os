use core::cell::UnsafeCell;
use core::ops::Deref;
use crate::spinlock::Spinlock;

pub struct SyncLazy<T, F = fn() -> T> {
    data: Cell<Option<T>>,
    init: Spinlock<F>,
}

impl<T, F> SyncLazy<T, F> {
    pub const fn new(f: F) -> Self {
        Self { data: Cell::new(None), init: Spinlock::new(f) }
    }
}

unsafe impl<T> Sync for SyncLazy<T> {}

impl<T> Deref for SyncLazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if self.data.is_none() {
            let init = self.init.lock();

            if self.data.is_none() {
                self.data.set(Some(init()));
            }
        }

        self.data.as_ref().unwrap()
    }
}

pub struct Cell<T> {
    data: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub const fn new(data: T) -> Self {
        Self { data: UnsafeCell::new(data) }
    }

    pub fn set(&self, data: T) {
        unsafe { *self.data.get() = data; }
    }
}

impl<T> Deref for Cell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data.get() }
    }
}
