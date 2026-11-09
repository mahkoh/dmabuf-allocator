use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::Relaxed;

const NAME: &[u8] = b"MESA_LOADER_DRIVER_OVERRIDE=";
const VALUE_CAPACITY: usize = 32;

static ENTRY: [AtomicU8; NAME.len() + VALUE_CAPACITY + 1] =
    [const { AtomicU8::new(0) }; NAME.len() + VALUE_CAPACITY + 1];
static INSTALLED: AtomicBool = AtomicBool::new(false);

fn value() -> &'static [AtomicU8] {
    &ENTRY[NAME.len()..]
}

pub unsafe fn install() {
    for (byte, value) in ENTRY.iter().zip(NAME) {
        byte.store(*value, Relaxed);
    }
    let result = unsafe { libc::putenv(ENTRY.as_ptr().cast_mut().cast()) };
    assert_eq!(
        result,
        0,
        "putenv failed: {}",
        std::io::Error::last_os_error()
    );
    INSTALLED.store(true, Relaxed);
}

struct Clear;

impl Drop for Clear {
    fn drop(&mut self) {
        value()[0].store(0, Relaxed);
    }
}

pub unsafe fn with<T>(driver: &str, f: impl FnOnce() -> T) -> T {
    assert!(
        INSTALLED.load(Relaxed),
        "the driver override is not installed"
    );
    assert!(
        driver.len() <= VALUE_CAPACITY,
        "the driver name {driver:?} is too long"
    );
    assert!(
        !driver.as_bytes().contains(&0),
        "the driver name {driver:?} contains a NUL byte"
    );
    let value = value();
    value[driver.len()].store(0, Relaxed);
    for (byte, value) in value.iter().zip(driver.bytes()) {
        byte.store(value, Relaxed);
    }
    let _clear = Clear;
    f()
}
