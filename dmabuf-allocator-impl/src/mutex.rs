use crate::sys;
use core::cell::Cell;
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ops::Deref;
use core::ops::DerefMut;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering::Acquire;
use core::sync::atomic::Ordering::Relaxed;
use core::sync::atomic::Ordering::Release;

const UNLOCKED: u32 = 0;
const LOCKED: u32 = 1;
const CONTENDED: u32 = 2;

pub(crate) struct Mutex<T> {
    state: AtomicU32,
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for Mutex<T> {}

pub(crate) struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
    _not_sync: PhantomData<Cell<()>>,
}

impl<T> Mutex<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(UNLOCKED),
            value: UnsafeCell::new(value),
        }
    }

    pub(crate) fn lock(&self) -> MutexGuard<'_, T> {
        if self
            .state
            .compare_exchange(UNLOCKED, LOCKED, Acquire, Relaxed)
            .is_err()
        {
            while self.state.swap(CONTENDED, Acquire) != UNLOCKED {
                sys::futex_wait(&self.state, CONTENDED);
            }
        }
        MutexGuard {
            mutex: self,
            _not_sync: PhantomData,
        }
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        if self.mutex.state.swap(UNLOCKED, Release) == CONTENDED {
            sys::futex_wake(&self.mutex.state, 1);
        }
    }
}
