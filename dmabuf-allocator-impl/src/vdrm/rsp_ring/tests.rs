extern crate std;

use super::*;
use std::thread;

fn try_acquire(ring: &RspRing, units: usize) -> Option<RspSlot> {
    ring.try_acquire(units * RSP_ALIGNMENT).unwrap()
}

fn acquire(ring: &RspRing) -> Result<RspSlot, VdrmError> {
    loop {
        if let Some(slot) = ring.acquire_or_wait(RSP_ALIGNMENT, || false)? {
            return Ok(slot);
        }
    }
}

#[test]
fn out_of_order_release() {
    let ring = RspRing::new(4);
    let a = try_acquire(&ring, 1).unwrap();
    let b = try_acquire(&ring, 1).unwrap();
    let c = try_acquire(&ring, 1).unwrap();
    let d = try_acquire(&ring, 1).unwrap();
    assert_eq!([a.head, b.head, c.head, d.head], [0, 1, 2, 3]);
    ring.release(b);
    ring.release(c);
    assert!(try_acquire(&ring, 1).is_none());
    ring.release(a);
    let e = try_acquire(&ring, 1).unwrap();
    let f = try_acquire(&ring, 1).unwrap();
    let g = try_acquire(&ring, 1).unwrap();
    assert_eq!([e.head, f.head, g.head], [0, 1, 2]);
    assert!(try_acquire(&ring, 1).is_none());
    ring.release(d);
    assert_eq!(try_acquire(&ring, 1).unwrap().head, 3);
}

#[test]
fn slot_that_does_not_fit_starts_at_zero() {
    let ring = RspRing::new(4);
    ring.release(try_acquire(&ring, 3).unwrap());
    assert_eq!(try_acquire(&ring, 2).unwrap().head, 0);
}

#[test]
fn slot_that_does_not_fit_waits_for_the_head() {
    let ring = RspRing::new(4);
    let a = try_acquire(&ring, 3).unwrap();
    let b = try_acquire(&ring, 1).unwrap();
    ring.release(a);
    ring.release(try_acquire(&ring, 3).unwrap());
    assert_eq!(b.head, 3);
    assert!(try_acquire(&ring, 2).is_none());
    ring.release(b);
    assert_eq!(try_acquire(&ring, 2).unwrap().head, 0);
}

#[test]
fn full_ring_sleeps_until_release() {
    let ring = RspRing::new(1);
    let slot = try_acquire(&ring, 1).unwrap();
    thread::scope(|scope| {
        let waiter = scope.spawn(|| acquire(&ring).unwrap().head);
        while ring.waiters.load(Relaxed) != WAITERS {
            thread::yield_now();
        }
        ring.release(slot);
        assert_eq!(waiter.join().unwrap(), 0);
    });
}

#[test]
fn failed_ring_wakes_waiters() {
    let ring = RspRing::new(1);
    let _slot = try_acquire(&ring, 1).unwrap();
    thread::scope(|scope| {
        let waiter = scope.spawn(|| acquire(&ring).map(|slot| slot.head));
        while ring.waiters.load(Relaxed) != WAITERS {
            thread::yield_now();
        }
        ring.fail(Errno(libc::EIO));
        assert!(matches!(
            waiter.join().unwrap(),
            Err(VdrmError::Broken(Errno(libc::EIO)))
        ));
    });
    assert!(matches!(
        acquire(&ring),
        Err(VdrmError::Broken(Errno(libc::EIO)))
    ));
}

#[test]
fn slot_that_fits_follows_the_previous_slot() {
    let ring = RspRing::new(8);
    assert_eq!(try_acquire(&ring, 3).unwrap().head, 0);
    assert_eq!(try_acquire(&ring, 3).unwrap().head, 3);
}

#[test]
fn slot_that_does_not_fit_skips_the_end() {
    let ring = RspRing::new(8);
    let a = try_acquire(&ring, 5).unwrap();
    let _b = try_acquire(&ring, 1).unwrap();
    ring.release(a);
    assert_eq!(try_acquire(&ring, 3).unwrap().head, 0);
    assert!(try_acquire(&ring, 4).is_none());
}

#[test]
fn free_units_never_exceed_the_ring() {
    let ring = RspRing::new(8);
    ring.release(try_acquire(&ring, 1).unwrap());
    assert_eq!(try_acquire(&ring, 8).unwrap().head, 0);
    assert!(try_acquire(&ring, 1).is_none());
}

#[test]
fn contended_ring_hands_out_each_unit_once() {
    const THREADS: usize = 4;
    const ITERATIONS: usize = 10000;
    let ring = RspRing::new(2);
    let owned = [const { AtomicU32::new(0) }; 2];
    thread::scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|| {
                for _ in 0..ITERATIONS {
                    let slot = acquire(&ring).unwrap();
                    assert_eq!(owned[slot.head].swap(1, Relaxed), 0);
                    owned[slot.head].store(0, Relaxed);
                    ring.release(slot);
                }
            });
        }
    });
    assert_eq!(ring.state.lock().free, 2);
}

#[test]
fn empty_ring_starts_at_zero() {
    let ring = RspRing::new(2046);
    ring.release(try_acquire(&ring, 1023).unwrap());
    assert_eq!(try_acquire(&ring, 1025).unwrap().head, 0);
}

#[test]
fn skipped_units_are_freed_by_a_later_release() {
    let ring = RspRing::new(100);
    let a = try_acquire(&ring, 50).unwrap();
    let b = try_acquire(&ring, 10).unwrap();
    ring.release(a);
    let c = try_acquire(&ring, 45).unwrap();
    assert_eq!([b.head, c.head], [50, 0]);
    ring.release(b);
    assert_eq!(try_acquire(&ring, 30).unwrap().head, 45);
}

#[test]
fn responses_are_rounded_up_to_whole_units() {
    let ring = RspRing::new(4);
    let slot = ring.try_acquire(RSP_ALIGNMENT + 1).unwrap().unwrap();
    assert_eq!(slot.units, 2);
}

#[test]
fn response_larger_than_the_ring_is_rejected() {
    let ring = RspRing::new(4);
    let slot = ring.try_acquire(RSP_ALIGNMENT * 5);
    assert!(matches!(slot, Err(VdrmError::ResponseSize)));
}

#[test]
fn skipped_units_are_not_free() {
    let ring = RspRing::new(10);
    let a = try_acquire(&ring, 6).unwrap();
    let _b = try_acquire(&ring, 1).unwrap();
    ring.release(a);
    assert_eq!(try_acquire(&ring, 5).unwrap().head, 0);
    assert!(try_acquire(&ring, 2).is_none());
}

#[test]
fn pending_slots_prevent_sleeping() {
    let ring = RspRing::new(1);
    let _slot = try_acquire(&ring, 1).unwrap();
    let slot = ring.acquire_or_wait(RSP_ALIGNMENT, || true).unwrap();
    assert!(slot.is_none());
}

#[test]
fn notify_wakes_waiters() {
    let ring = RspRing::new(1);
    let _slot = try_acquire(&ring, 1).unwrap();
    thread::scope(|scope| {
        let waiter = scope.spawn(|| ring.acquire_or_wait(RSP_ALIGNMENT, || false).unwrap());
        while ring.waiters.load(Relaxed) != WAITERS {
            thread::yield_now();
        }
        ring.notify();
        assert!(waiter.join().unwrap().is_none());
    });
}
