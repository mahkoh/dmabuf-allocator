use manual_tests::AllocatorError;
use manual_tests::Call;
use manual_tests::DEVICE_FD;
use manual_tests::Device;
use manual_tests::DriverName;
use manual_tests::DrmError;
use manual_tests::FakeDevice;
use manual_tests::Fault;
use manual_tests::Kernel;
use manual_tests::LINEAR;
use manual_tests::PAGE_SIZE;
use manual_tests::Plane;
use manual_tests::Request;
use manual_tests::Vc4Error;
use manual_tests::request;
use manual_tests::uapi::drm::DRM_IOCTL_GEM_CLOSE;
use manual_tests::uapi::drm::DRM_IOCTL_PRIME_HANDLE_TO_FD;
use manual_tests::uapi::drm::drm_gem_close;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED;
use manual_tests::uapi::vc4::DRM_IOCTL_VC4_CREATE_BO;
use manual_tests::uapi::vc4::DRM_IOCTL_VC4_GET_TILING;
use manual_tests::uapi::vc4::DRM_IOCTL_VC4_SET_TILING;
use manual_tests::uapi::vc4::drm_vc4_create_bo;
use std::ptr;

const BCM2837: &str = "vc4:gpu=bcm2837";

const MAX_EXTENT: u32 = 2048;
const AR24_BYTES: u64 = 4;
const AR24_UTILE: u32 = 4;
const R8_BYTES: u64 = 1;
const R8_UTILE: u32 = 8;
const RG16_BYTES: u64 = 2;
const RG16_UTILE_WIDTH: u32 = 8;
const RG16_UTILE_HEIGHT: u32 = 4;
const LT_UTILES: u32 = 4;
const T_UTILES: u32 = 8;

fn chosen(request: &Request) -> Option<u64> {
    Device::new(BCM2837).chosen(request)
}

fn planes(request: &Request) -> Vec<Plane> {
    let layout = Device::new(BCM2837).layout(request).unwrap();
    manual_tests::planes(&layout)
}

fn calls(device: &Device, request: &Request) -> Vec<Call> {
    device.allocate(request).unwrap().calls
}

fn create_bo(size: u64) -> Call {
    Call::Vc4CreateBo {
        size: u32::try_from(size).unwrap(),
        flags: 0,
    }
}

fn create_bo_ioctl(fake: &FakeDevice) -> u32 {
    let mut create = drm_vc4_create_bo {
        size: u32::try_from(PAGE_SIZE).unwrap(),
        flags: 0,
        handle: 0,
        pad: 0,
    };
    let request = DRM_IOCTL_VC4_CREATE_BO as libc::Ioctl;
    unsafe { fake.ioctl(DEVICE_FD, request, ptr::from_mut(&mut create).cast()) }.unwrap();
    create.handle
}

fn gem_close_ioctl(fake: &FakeDevice, handle: u32) {
    let mut close = drm_gem_close { handle, pad: 0 };
    let request = DRM_IOCTL_GEM_CLOSE as libc::Ioctl;
    unsafe { fake.ioctl(DEVICE_FD, request, ptr::from_mut(&mut close).cast()) }.unwrap();
}

#[test]
fn vc4_is_its_own_driver() {
    assert_eq!(Device::new(BCM2837).driver(), DriverName::Vc4);
}

#[test]
fn extents_are_limited_to_2048() {
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
fn formats_without_vc4_support_have_no_layout() {
    assert_eq!(chosen(&request("AB4H", &[LINEAR], 64, 64)), None);
    assert_eq!(chosen(&request("AR15", &[LINEAR], 64, 64)), None);
    assert_eq!(chosen(&request("GR88", &[LINEAR], 64, 64)), Some(LINEAR));
}

#[test]
fn t_tiling_is_preferred() {
    let both = [LINEAR, DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED];
    assert_eq!(
        chosen(&request("AR24", &both, 64, 64)),
        Some(DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED)
    );
    assert_eq!(
        chosen(&request("AR24", &both, 64, 64).scanout()),
        Some(DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED)
    );
}

#[test]
fn small_images_are_linear() {
    let lt = LT_UTILES * AR24_UTILE;
    let both = [LINEAR, DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED];
    let tiled = [DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED];
    assert_eq!(chosen(&request("AR24", &both, lt, 64)), Some(LINEAR));
    assert_eq!(chosen(&request("AR24", &both, 64, lt)), Some(LINEAR));
    assert_eq!(
        chosen(&request("AR24", &both, lt + 1, lt + 1)),
        Some(DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED)
    );
    assert_eq!(chosen(&request("AR24", &tiled, lt, 64)), None);
    let lt = LT_UTILES * R8_UTILE;
    assert_eq!(chosen(&request("R8  ", &both, lt, 64)), Some(LINEAR));
    assert_eq!(
        chosen(&request("R8  ", &both, lt + 1, lt + 1)),
        Some(DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED)
    );
}

#[test]
fn implicit_interop_is_linear() {
    let both = [DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED, LINEAR];
    let base = || request("AR24", &both, 64, 64);
    assert_eq!(chosen(&base().implicit_interop()), Some(LINEAR));
}

#[test]
fn foreign_modifiers_have_no_layout() {
    let foreign = [DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED];
    assert_eq!(chosen(&request("AR24", &foreign, 64, 64)), None);
}

#[test]
fn linear_width_is_aligned_to_a_utile() {
    let linear = request("AR24", &[LINEAR], 1, 3);
    let stride = u64::from(AR24_UTILE) * AR24_BYTES;
    assert_eq!(planes(&linear), [Plane::first(stride, stride * 3)]);
    let linear = request("R8  ", &[LINEAR], 9, 3);
    let stride = 2 * u64::from(R8_UTILE) * R8_BYTES;
    assert_eq!(planes(&linear), [Plane::first(stride, stride * 3)]);
}

#[test]
fn t_tiles_are_8_utiles() {
    let extent = LT_UTILES * AR24_UTILE + 1;
    let tiled = request(
        "AR24",
        &[DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED],
        extent,
        extent,
    );
    let tile = u64::from(T_UTILES * AR24_UTILE);
    let stride = tile * AR24_BYTES;
    assert_eq!(planes(&tiled), [Plane::first(stride, stride * tile)]);
    let extent = LT_UTILES * RG16_UTILE_WIDTH + 1;
    let tiled = request(
        "RG16",
        &[DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED],
        extent,
        extent,
    );
    let stride = u64::from(T_UTILES * RG16_UTILE_WIDTH) * RG16_BYTES;
    let rows = 2 * u64::from(T_UTILES * RG16_UTILE_HEIGHT);
    assert_eq!(planes(&tiled), [Plane::first(stride, stride * rows)]);
}

#[test]
fn allocation_sets_the_tiling() {
    let device = Device::new(BCM2837);
    let tiled = calls(
        &device,
        &request("AR24", &[DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED], 64, 64),
    );
    assert_eq!(
        tiled[..2],
        [
            create_bo(64 * AR24_BYTES * 64),
            Call::Vc4SetTiling {
                flags: 0,
                modifier: DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED,
            },
        ]
    );
    let linear = calls(&device, &request("AR24", &[LINEAR], 1, 1));
    assert_eq!(
        linear[..2],
        [
            create_bo(PAGE_SIZE),
            Call::Vc4SetTiling {
                flags: 0,
                modifier: LINEAR,
            },
        ]
    );
}

#[test]
fn without_the_tiling_ioctl_buffers_are_linear_and_untagged() {
    let device = Device::builder(BCM2837)
        .fail(Fault::new(DRM_IOCTL_VC4_GET_TILING).errno(libc::EINVAL))
        .open()
        .unwrap();
    let both = [DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED, LINEAR];
    let layout = device.layout(&request("AR24", &both, 64, 64)).unwrap();
    assert_eq!(layout.modifier, LINEAR);
    let calls = calls(&device, &request("AR24", &both, 64, 64));
    assert!(
        !calls
            .iter()
            .any(|call| matches!(call, Call::Vc4SetTiling { .. }))
    );
}

#[test]
fn failed_create_bo_fails_the_allocation() {
    let device = Device::new(BCM2837);
    device.fail(Fault::new(DRM_IOCTL_VC4_CREATE_BO).errno(libc::ENOMEM));
    let error = device
        .allocate(&request("AR24", &[LINEAR], 64, 64))
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::Vc4Allocate(Vc4Error::CreateBo(errno)) if errno.0 == libc::ENOMEM
    ));
}

#[test]
fn failed_set_tiling_fails_the_allocation() {
    let device = Device::new(BCM2837);
    device.fail(Fault::new(DRM_IOCTL_VC4_SET_TILING).errno(libc::ENOENT));
    let error = device
        .allocate(&request("AR24", &[LINEAR], 64, 64))
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::Vc4Allocate(Vc4Error::SetTiling(errno)) if errno.0 == libc::ENOENT
    ));
}

#[test]
fn failed_export_closes_the_gem_handle() {
    let device = Device::new(BCM2837);
    device.fail(Fault::new(DRM_IOCTL_PRIME_HANDLE_TO_FD).errno(libc::EMFILE));
    let error = device
        .allocate(&request("AR24", &[LINEAR], 64, 64))
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::Vc4Allocate(Vc4Error::Export(DrmError::PrimeHandleToFd(errno)))
            if errno.0 == libc::EMFILE
    ));
}

#[test]
fn tiling_probe_uses_an_impossible_handle() {
    let device = Device::builder(BCM2837)
        .before_open(|fake| assert_eq!(create_bo_ioctl(fake), 1))
        .open()
        .unwrap();
    let both = [DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED, LINEAR];
    let chosen = device.chosen(&request("AR24", &both, 64, 64));
    gem_close_ioctl(device.fake(), 1);
    assert_eq!(chosen, Some(DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED));
}
