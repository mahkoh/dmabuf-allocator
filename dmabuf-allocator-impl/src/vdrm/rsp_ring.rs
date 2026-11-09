use crate::errno::Errno;
use crate::mutex::Mutex;
use crate::mutex::MutexGuard;
use crate::sys;
use crate::vdrm::RSP_ALIGNMENT;
use crate::vdrm::SHMEM_SIZE;
use crate::vdrm::VdrmError;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering::Relaxed;

#[cfg(test)]
mod tests;

const NO_WAITERS: u32 = 0;
const WAITERS: u32 = 1;
const ALL_WAITERS: u32 = i32::MAX as u32;

pub(crate) struct RspRing {
    units: usize,
    state: Mutex<State>,
    waiters: AtomicU32,
}

struct State {
    head: usize,
    free: usize,
    used: [bool; SHMEM_SIZE / RSP_ALIGNMENT],
    status: Result<(), Errno>,
}

pub(crate) struct RspSlot {
    head: usize,
    units: usize,
}

impl RspSlot {
    pub(crate) fn offset(&self) -> usize {
        self.head * RSP_ALIGNMENT
    }

    pub(crate) fn size(&self) -> usize {
        self.units * RSP_ALIGNMENT
    }
}

impl RspRing {
    pub(crate) fn new(units: usize) -> Self {
        Self {
            units,
            state: Mutex::new(State {
                head: 0,
                free: units,
                used: [false; SHMEM_SIZE / RSP_ALIGNMENT],
                status: Ok(()),
            }),
            waiters: AtomicU32::new(NO_WAITERS),
        }
    }

    fn wrap(&self, unit: usize) -> usize {
        match unit < self.units {
            true => unit,
            false => unit - self.units,
        }
    }

    fn acquire_locked(&self, state: &mut State, size: usize) -> Result<Option<RspSlot>, VdrmError> {
        let units = size.div_ceil(RSP_ALIGNMENT);
        if units > self.units {
            return Err(VdrmError::ResponseSize);
        }
        state.status.map_err(VdrmError::Broken)?;
        if state.free == self.units {
            state.head = 0;
        }
        let (head, skipped) = match state.head + units <= self.units {
            true => (state.head, 0),
            false => (0, self.units - state.head),
        };
        if skipped + units > state.free {
            return Ok(None);
        }
        state.used[head..head + units].fill(true);
        state.head = self.wrap(head + units);
        state.free -= skipped + units;
        Ok(Some(RspSlot { head, units }))
    }

    pub(crate) fn try_acquire(&self, size: usize) -> Result<Option<RspSlot>, VdrmError> {
        self.acquire_locked(&mut self.state.lock(), size)
    }

    pub(crate) fn acquire_or_wait(
        &self,
        size: usize,
        pending: impl FnOnce() -> bool,
    ) -> Result<Option<RspSlot>, VdrmError> {
        let mut state = self.state.lock();
        if let Some(slot) = self.acquire_locked(&mut state, size)? {
            return Ok(Some(slot));
        }
        self.waiters.store(WAITERS, Relaxed);
        drop(state);
        if !pending() {
            sys::futex_wait(&self.waiters, WAITERS);
        }
        Ok(None)
    }

    pub(crate) fn notify(&self) {
        if self.waiters.load(Relaxed) == WAITERS {
            self.wake(self.state.lock());
        }
    }

    pub(crate) fn check(&self) -> Result<(), VdrmError> {
        self.state.lock().status.map_err(VdrmError::Broken)
    }

    pub(crate) fn fail(&self, errno: Errno) {
        let mut state = self.state.lock();
        state.status = Err(errno);
        self.wake(state);
    }

    pub(crate) fn release(&self, slot: RspSlot) {
        let mut state = self.state.lock();
        state.used[slot.head..slot.head + slot.units].fill(false);
        let old = state.free;
        while state.free < self.units && !state.used[self.wrap(state.head + state.free)] {
            state.free += 1;
        }
        if state.free > old && self.waiters.load(Relaxed) == WAITERS {
            self.wake(state);
        }
    }

    fn wake(&self, state: MutexGuard<'_, State>) {
        self.waiters.store(NO_WAITERS, Relaxed);
        drop(state);
        sys::futex_wake(&self.waiters, ALL_WAITERS);
    }
}
