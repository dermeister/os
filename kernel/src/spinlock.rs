use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct Spinlock<T> {
    data: UnsafeCell<T>,
    locked: AtomicBool,
}

impl<T> Spinlock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> SpinlockGuard<T> {
        loop {
            let was_locked = self.locked.swap(true, Ordering::Acquire);
            if !was_locked { break; }
        }

        SpinlockGuard { spinlock: &self }
    }
}

pub struct SpinlockGuard<'a, T> {
    spinlock: &'a Spinlock<T>,
}

impl<'a, T> Deref for SpinlockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.spinlock.data.get() }
    }
}

impl<'a, T> DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.spinlock.data.get() }
    }
}

impl<'a, T> Drop for SpinlockGuard<'a, T> {
    fn drop(&mut self) {
        self.spinlock.locked.store(false, Ordering::Release);
    }
}
