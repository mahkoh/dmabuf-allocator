use allocator_trace::Call;
use fake_kernel::Kernel;
use fake_kernel::KernelVersion;
use fake_kernel::Node;
use fake_kernel::uapi::drm::DRM_CLOEXEC;
use fake_kernel::uapi::drm::DRM_IOCTL_PRIME_HANDLE_TO_FD;
use fake_kernel::uapi::drm::DRM_IOCTL_VERSION;
use fake_kernel::uapi::drm::DRM_RDWR;
use fake_kernel::uapi::drm::drm_prime_handle;
use fake_kernel::uapi::drm::drm_version;
use fake_kernel::uapi::v3d::DRM_IOCTL_V3D_CREATE_BO;
use fake_kernel::uapi::v3d::drm_v3d_create_bo;
use fake_supervisor::DrmNode;
use fake_supervisor::client;
use std::ffi::OsString;
use std::fs::File;
use std::io::ErrorKind;
use std::os::fd::AsRawFd;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;

const DRM_MAJOR: u32 = 226;
const WORKER_TEST: &str = "worker";

fn entries(path: &str) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    names
}

fn ioctl<T>(file: &File, request: u32, arg: &mut T) {
    let ret = unsafe { libc::ioctl(file.as_raw_fd(), request as libc::Ioctl, arg) };
    assert_eq!(ret, 0, "ioctl {request:#x} failed");
}

fn driver_name(file: &File) -> String {
    let mut name = [0u8; 32];
    let mut version: drm_version = unsafe { std::mem::zeroed() };
    version.name_len = name.len();
    version.name = name.as_mut_ptr().cast();
    ioctl(file, DRM_IOCTL_VERSION, &mut version);
    String::from_utf8_lossy(&name[..version.name_len]).into_owned()
}

fn devices_are_listed() {
    assert_eq!(entries("/dev/dri"), ["card0", "card1", "renderD128"]);
    assert_eq!(
        entries("/sys/class/drm"),
        ["card0", "card1", "renderD128", "version"]
    );
    let version = std::fs::read_to_string("/sys/class/drm/version").unwrap();
    assert_eq!(version, "drm 1.1.0 20060810\n");
    let link = std::fs::read_link("/sys/dev/char/226:128").unwrap();
    assert_eq!(link.to_str(), Some("../../devices/fake0/drm/renderD128"));
    let uevent = std::fs::read_to_string("/sys/dev/char/226:1/device/uevent").unwrap();
    assert_eq!(uevent, "DRIVER=faux_driver\n");
}

fn nodes_are_character_devices() {
    let render = DrmNode {
        device: 0,
        kind: Node::Render,
    };
    let path = format!("/dev/dri/{}", render.name());
    let metadata = std::fs::metadata(&path).unwrap();
    assert!(metadata.file_type().is_char_device());
    assert_eq!(metadata.rdev(), libc::makedev(DRM_MAJOR, render.minor()));
    let missing = std::fs::metadata("/dev/dri/renderD129").unwrap_err();
    assert_eq!(missing.kind(), ErrorKind::NotFound);
}

fn ioctls_are_recorded() {
    let file = File::options()
        .read(true)
        .write(true)
        .open("/dev/dri/renderD128")
        .unwrap();
    assert_eq!(driver_name(&file), "v3d");
    client::start_recording();
    let mut create = drm_v3d_create_bo {
        size: 4096,
        flags: 0,
        handle: 0,
        offset: 0,
    };
    ioctl(&file, DRM_IOCTL_V3D_CREATE_BO, &mut create);
    let mut prime = drm_prime_handle {
        handle: create.handle,
        flags: DRM_CLOEXEC | DRM_RDWR,
        fd: -1,
    };
    ioctl(&file, DRM_IOCTL_PRIME_HANDLE_TO_FD, &mut prime);
    let traced = client::stop_recording();
    assert_eq!(unsafe { libc::close(prime.fd) }, 0);
    let calls: Vec<&Call> = traced.iter().map(|traced| &traced.call).collect();
    let expected = [
        &Call::V3dCreateBo {
            size: 4096,
            flags: 0,
        },
        &Call::PrimeHandleToFd {
            flags: DRM_CLOEXEC | DRM_RDWR,
        },
    ];
    assert_eq!(calls, expected);
    assert!(
        traced
            .iter()
            .all(|traced| traced.handle == Some(create.handle))
    );
    assert_eq!(client::stop_recording(), []);
}

fn hidden_devices_are_not_listed() {
    client::set_visible(Some(1..2));
    assert_eq!(entries("/dev/dri"), ["card1"]);
    assert_eq!(
        entries("/sys/class/drm"),
        ["card0", "card1", "renderD128", "version"]
    );
    let link = std::fs::read_link("/sys/dev/char/226:128").unwrap();
    assert_eq!(link.to_str(), Some("../../devices/fake0/drm/renderD128"));
    client::set_visible(None);
    assert_eq!(entries("/dev/dri"), ["card0", "card1", "renderD128"]);
}

fn other_threads_see_all_devices() {
    client::set_visible(Some(1..2));
    let listed = std::thread::spawn(|| entries("/dev/dri")).join().unwrap();
    client::set_visible(None);
    assert_eq!(listed, ["card0", "card1", "renderD128"]);
}

#[test]
fn worker() {
    if !client::is_worker() {
        return;
    }
    let kernel = KernelVersion::NEWEST;
    let ids = [
        fake_kernel::v3d::Device::new("bcm2711", kernel).id(),
        fake_kernel::vkms::Device::new(kernel).id(),
    ];
    assert_eq!(client::register(&ids), 0..2);
    devices_are_listed();
    nodes_are_character_devices();
    ioctls_are_recorded();
    hidden_devices_are_not_listed();
    other_threads_see_all_devices();
}

#[test]
fn supervised_worker_uses_the_fake_devices() {
    if client::is_worker() {
        return;
    }
    let program = std::env::current_exe().unwrap();
    let arguments: Vec<OsString> = ["--exact", WORKER_TEST, "--nocapture"]
        .map(OsString::from)
        .to_vec();
    let status = fake_supervisor::supervise(&program, &arguments);
    assert!(status.success(), "the worker failed: {status}");
}
