use manual_tests::AllocatorError;
use manual_tests::Call;
use manual_tests::Device;
use manual_tests::DriverName;
use manual_tests::DrmError;
use manual_tests::Fault;
use manual_tests::LINEAR;
use manual_tests::LimaError;
use manual_tests::PAGE_SIZE;
use manual_tests::Plane;
use manual_tests::Request;
use manual_tests::request;
use manual_tests::uapi::drm::DRM_IOCTL_PRIME_HANDLE_TO_FD;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_VIVANTE_TILED;
use manual_tests::uapi::lima::DRM_IOCTL_LIMA_GEM_CREATE;

const A64: &str = "lima:gpu=a64";
const RK3328: &str = "lima:gpu=rk3328";

const MAX_EXTENT: u32 = 4096;
const TILE: u64 = 16;
const AR24_BYTES: u64 = 4;
const R8_BYTES: u64 = 1;

fn chosen(request: &Request) -> Option<u64> {
    Device::new(A64).chosen(request)
}

fn planes(request: &Request) -> Vec<Plane> {
    let layout = Device::new(A64).layout(request).unwrap();
    manual_tests::planes(&layout)
}

fn create(request: &Request) -> Call {
    let buffer = Device::new(A64).allocate(request).unwrap();
    buffer.only(|call| matches!(call, Call::LimaGemCreate { .. }).then(|| call.clone()))
}

fn gem_create(size: u64) -> Call {
    Call::LimaGemCreate {
        size: u32::try_from(size).unwrap(),
        flags: 0,
    }
}

#[test]
fn lima_is_its_own_driver() {
    assert_eq!(Device::new(A64).driver(), DriverName::Lima);
    assert_eq!(Device::new(RK3328).driver(), DriverName::Lima);
}

#[test]
fn extents_are_limited_to_4096() {
    assert_eq!(
        chosen(&request("AR24", &[LINEAR], MAX_EXTENT, 1)),
        Some(LINEAR)
    );
    assert_eq!(
        chosen(&request("AR24", &[LINEAR], 1, MAX_EXTENT)),
        Some(LINEAR)
    );
    assert_eq!(chosen(&request("AR24", &[LINEAR], MAX_EXTENT + 1, 1)), None);
    assert_eq!(chosen(&request("AR24", &[LINEAR], 1, MAX_EXTENT + 1)), None);
    assert_eq!(chosen(&request("AR24", &[LINEAR], 0, 1)), None);
    assert_eq!(chosen(&request("AR24", &[LINEAR], 1, 0)), None);
}

#[test]
fn formats_without_lima_support_have_no_layout() {
    assert_eq!(chosen(&request("AB30", &[LINEAR], 64, 64)), None);
    assert_eq!(chosen(&request("NV12", &[LINEAR], 64, 64)), None);
    assert_eq!(chosen(&request("R  H", &[LINEAR], 64, 64)), Some(LINEAR));
}

#[test]
fn linear_wins_whenever_it_is_requested() {
    let both = [DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED, LINEAR];
    assert_eq!(chosen(&request("AR24", &both, 64, 64)), Some(LINEAR));
    assert_eq!(
        chosen(&request("AR24", &both, 64, 64).scanout()),
        Some(LINEAR)
    );
}

#[test]
fn u_interleaved_is_used_without_linear() {
    let tiled = [DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED];
    assert_eq!(
        chosen(&request("AR24", &tiled, 64, 64)),
        Some(DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED)
    );
}

#[test]
fn scanout_and_implicit_interop_reject_u_interleaved() {
    let tiled = [DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED];
    assert_eq!(chosen(&request("AR24", &tiled, 64, 64).scanout()), None);
    assert_eq!(
        chosen(&request("AR24", &tiled, 64, 64).implicit_interop()),
        None
    );
}

#[test]
fn foreign_modifiers_have_no_layout() {
    let foreign = [DRM_FORMAT_MOD_VIVANTE_TILED];
    assert_eq!(chosen(&request("AR24", &foreign, 64, 64)), None);
}

#[test]
fn extents_are_aligned_to_16_pixels() {
    for modifier in [LINEAR, DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED] {
        let stride = TILE * AR24_BYTES;
        assert_eq!(
            planes(&request("AR24", &[modifier], 1, 1)),
            [Plane::first(stride, stride * TILE)]
        );
        let stride = 2 * TILE * AR24_BYTES;
        assert_eq!(
            planes(&request("AR24", &[modifier], 17, 17)),
            [Plane::first(stride, stride * 2 * TILE)]
        );
        let stride = TILE * R8_BYTES;
        assert_eq!(
            planes(&request("R8  ", &[modifier], 16, 16)),
            [Plane::first(stride, stride * TILE)]
        );
    }
}

#[test]
fn buffers_are_page_aligned() {
    let small = request("AR24", &[LINEAR], 1, 1);
    assert_eq!(create(&small), gem_create(PAGE_SIZE));
    let size = 3 * TILE * AR24_BYTES * 3 * TILE;
    assert!(size > 2 * PAGE_SIZE && size < 3 * PAGE_SIZE);
    let large = request("AR24", &[LINEAR], 33, 33);
    assert_eq!(create(&large), gem_create(3 * PAGE_SIZE));
}

#[test]
fn failed_gem_create_fails_the_allocation() {
    let device = Device::new(A64);
    device.fail(Fault::new(DRM_IOCTL_LIMA_GEM_CREATE).errno(libc::ENOMEM));
    let error = device
        .allocate(&request("AR24", &[LINEAR], 64, 64))
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::LimaAllocate(LimaError::GemCreate(errno)) if errno.0 == libc::ENOMEM
    ));
}

#[test]
fn failed_export_closes_the_gem_handle() {
    let device = Device::new(A64);
    device.fail(Fault::new(DRM_IOCTL_PRIME_HANDLE_TO_FD).errno(libc::EMFILE));
    let error = device
        .allocate(&request("AR24", &[LINEAR], 64, 64))
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::LimaAllocate(LimaError::Export(DrmError::PrimeHandleToFd(errno)))
            if errno.0 == libc::EMFILE
    ));
}
