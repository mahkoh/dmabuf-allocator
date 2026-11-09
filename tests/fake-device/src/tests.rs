use super::*;
use fake_kernel::KernelVersion;
use fake_kernel::uapi::drm::drm_gem_close;
use fake_kernel::uapi::drm::drm_mode_create_dumb;
use fake_kernel::uapi::drm::drm_version;
use fake_kernel::uapi::v3d::drm_v3d_create_bo;

const MAPPING_OFFSET: u64 = 0x10_0000;

fn v3d() -> FakeDevice {
    let kernel = fake_kernel::v3d::Device::new("bcm2711", KernelVersion::NEWEST);
    FakeDevice::new(Box::new(kernel), fake_kernel::Node::Render)
}

fn ioctl<T>(device: &FakeDevice, fd: c_int, request: u32, arg: &mut T) -> Result<c_int, i32> {
    let arg = ptr::from_mut(arg).cast();
    let ret = unsafe { Kernel::ioctl(device, fd, request as libc::Ioctl, arg) };
    ret.map_err(|errno| errno.0)
}

fn create_bo(device: &FakeDevice, fd: c_int) -> u32 {
    let mut create = drm_v3d_create_bo {
        size: 4096,
        flags: 0,
        handle: 0,
        offset: 0,
    };
    assert_eq!(
        ioctl(device, fd, DRM_IOCTL_V3D_CREATE_BO, &mut create),
        Ok(0)
    );
    create.handle
}

fn fill(mapping: *mut u8, len: usize, value: u8) {
    unsafe {
        mapping.write_bytes(value, len);
    }
}

fn first_byte(mapping: *mut u8) -> u8 {
    unsafe { mapping.read() }
}

#[test]
fn ioctls_of_other_drivers_are_unexpected() {
    let device = v3d();
    let mut create: drm_mode_create_dumb = unsafe { std::mem::zeroed() };
    let created = ioctl(&device, DEVICE_FD, DRM_IOCTL_MODE_CREATE_DUMB, &mut create);
    assert_eq!(created, Err(libc::ENOTTY));
    let expected = format!("unexpected: ioctl {DRM_IOCTL_MODE_CREATE_DUMB:#x}");
    assert_eq!(device.leak_error(), Some(expected));
}

#[test]
fn udmabuf_expects_no_gem_ioctls() {
    let device = FakeDevice::new(
        fake_kernel::from_id("udmabuf:kernel=7.2"),
        fake_kernel::Node::Primary,
    );
    let mut version: drm_version = unsafe { std::mem::zeroed() };
    let queried = ioctl(&device, DEVICE_FD, DRM_IOCTL_VERSION, &mut version);
    assert_eq!(queried, Err(libc::ENOTTY));
    assert_eq!(device.leak_error(), None);
    let mut close = drm_gem_close { handle: 1, pad: 0 };
    let closed = ioctl(&device, DEVICE_FD, DRM_IOCTL_GEM_CLOSE, &mut close);
    assert_eq!(closed, Err(libc::ENOTTY));
    assert!(device.leak_error().is_some());
}

#[test]
fn handles_left_on_a_closed_render_node_are_reported() {
    let device = v3d();
    let fd = device.open_render_node(DEVICE_FD).map_err(|e| e.0).unwrap();
    let handle = create_bo(&device, fd);
    assert_eq!(device.take_calls().len(), 1);
    device.close(fd);
    assert_eq!(device.leak_error(), None);
    let expected = format!("GEM handle {handle} left open");
    assert_eq!(device.handle_error(), Some(expected));
    assert!(device.0.handles.borrow().is_empty());
}

#[test]
fn closed_handles_are_not_reported() {
    let device = v3d();
    let fd = device.open_render_node(DEVICE_FD).map_err(|e| e.0).unwrap();
    let mut close = drm_gem_close {
        handle: create_bo(&device, fd),
        pad: 0,
    };
    assert_eq!(ioctl(&device, fd, DRM_IOCTL_GEM_CLOSE, &mut close), Ok(0));
    device.close(fd);
    assert_eq!(device.handle_error(), None);
}

#[test]
fn duplicated_render_nodes_share_their_handles() {
    let device = v3d();
    let fd = device.open_render_node(DEVICE_FD).map_err(|e| e.0).unwrap();
    let duplicate = device.dup(fd).map_err(|e| e.0).unwrap();
    let mut close = drm_gem_close {
        handle: create_bo(&device, fd),
        pad: 0,
    };
    device.close(fd);
    assert_eq!(device.0.leaked.borrow().len(), 0);
    let closed = ioctl(&device, duplicate, DRM_IOCTL_GEM_CLOSE, &mut close);
    assert_eq!(closed, Ok(0));
    device.close(duplicate);
    assert_eq!(device.handle_error(), None);
}

#[test]
fn mappings_keep_their_contents() {
    let device = v3d();
    let len = 2 * PAGE_SIZE;
    for round in 0..3u8 {
        let mapping = device
            .mmap(DEVICE_FD, len, MAPPING_OFFSET)
            .map_err(|e| e.0)
            .unwrap();
        assert_eq!(mapping.align_offset(PAGE_SIZE), 0);
        assert_eq!(first_byte(mapping), round);
        fill(mapping, len, round + 1);
        unsafe {
            device.munmap(mapping, len);
        }
    }
    assert_eq!(device.0.contents.borrow().len(), 1);
    assert_eq!(device.leak_error(), None);
}

#[test]
fn unknown_mappings_are_unexpected() {
    let device = v3d();
    let mut byte = 0u8;
    unsafe {
        device.munmap(&mut byte, 1);
    }
    let expected = "unexpected: munmap of 1 bytes that are not mapped";
    assert_eq!(device.leak_error(), Some(expected.to_string()));
}

#[test]
fn pages_have_the_size_of_the_fake_kernels() {
    assert_eq!(v3d().page_size(), 4096);
}

#[test]
fn overlaps_are_clipped_to_both_ranges() {
    let overlap = overlap(100, 50, 120, 100).unwrap();
    assert_eq!(overlap.target, 20);
    assert_eq!(overlap.source, 0);
    assert_eq!(overlap.len, 30);
    assert!(super::overlap(0, 10, 10, 10).is_none());
}
