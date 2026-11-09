use manual_tests::AllocatorError;
use manual_tests::Call;
use manual_tests::Device;
use manual_tests::DriverName;
use manual_tests::Fault;
use manual_tests::LINEAR;
use manual_tests::PAGE_SIZE;
use manual_tests::Plane;
use manual_tests::Request;
use manual_tests::UdmabufError;
use manual_tests::request;
use manual_tests::uapi::udmabuf::UDMABUF_CREATE;
use manual_tests::uapi::udmabuf::UDMABUF_FLAGS_CLOEXEC;

const UDMABUF: &str = "udmabuf";

const MAX_EXTENT: u32 = 65536;

fn linear_ar24(width: u32, height: u32) -> Request {
    request("AR24", &[LINEAR], width, height)
}

fn chosen(width: u32, height: u32) -> Option<u64> {
    Device::new(UDMABUF).chosen(&linear_ar24(width, height))
}

#[test]
fn driver_is_udmabuf() {
    assert_eq!(Device::new(UDMABUF).driver(), DriverName::Udmabuf);
}

#[test]
fn allocation_wraps_a_memfd() {
    let buffer = Device::new(UDMABUF).allocate(&linear_ar24(64, 64)).unwrap();
    let create = Call::UdmabufCreate {
        flags: UDMABUF_FLAGS_CLOEXEC,
        offset: 0,
        size: 16384,
    };
    assert_eq!(buffer.planes(), [Plane::first(256, 16384)]);
    assert_eq!(buffer.calls, [create]);
}

#[test]
fn allocation_is_rounded_up_to_a_page() {
    let buffer = Device::new(UDMABUF).allocate(&linear_ar24(65, 1)).unwrap();
    let create = Call::UdmabufCreate {
        flags: UDMABUF_FLAGS_CLOEXEC,
        offset: 0,
        size: PAGE_SIZE,
    };
    assert_eq!(buffer.calls, [create]);
}

#[test]
fn zero_extents_have_no_layout() {
    assert_eq!(chosen(0, 64), None);
    assert_eq!(chosen(64, 0), None);
}

#[test]
fn extents_up_to_65536_have_a_layout() {
    assert_eq!(chosen(MAX_EXTENT, 1), Some(LINEAR));
    assert_eq!(chosen(1, MAX_EXTENT), Some(LINEAR));
}

#[test]
fn extents_above_65536_have_no_layout() {
    assert_eq!(chosen(MAX_EXTENT + 1, 1), None);
    assert_eq!(chosen(1, MAX_EXTENT + 1), None);
}

#[test]
fn failed_create_closes_the_memfd() {
    let device = Device::new(UDMABUF);
    device.fail(Fault::new(UDMABUF_CREATE).errno(libc::ENOMEM));
    let error = device.allocate(&linear_ar24(64, 64)).err().unwrap();
    assert!(matches!(
        error,
        AllocatorError::UdmabufAllocate(UdmabufError::Create(errno)) if errno.0 == libc::ENOMEM
    ));
}
