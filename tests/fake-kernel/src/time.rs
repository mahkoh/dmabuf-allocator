pub(crate) const NSEC_PER_SEC: u64 = 1_000_000_000;

pub(crate) fn ktime_get_ns() -> u64 {
    let mut now = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let ret = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut now) };
    assert_eq!(ret, 0, "clock_gettime(CLOCK_MONOTONIC) failed");
    now.tv_sec as u64 * NSEC_PER_SEC + now.tv_nsec as u64
}
