use crate::drm;
use allocator_trace::Call;
use allocator_trace::Traced;
use allocator_trace::vectors;
use allocator_trace::vectors::Flag;
use allocator_trace::vectors::Flags;
use fake_kernel::Node;
use fake_kernel::uapi::drm::DRM_IOCTL_VERSION;
use fake_kernel::uapi::drm::drm_version;
use std::os::fd::AsRawFd;
use std::os::fd::BorrowedFd;

#[cfg(test)]
mod tests;

const DRIVER_NAME_CAPACITY: usize = 32;

const VULKAN_FLAGS: [Flag; 9] = [
    Flag::AllowWriteCompressDisable,
    Flag::IgnoreDomains,
    Flag::IgnoreAlignment,
    Flag::IgnoreSize,
    Flag::IgnoreRegions,
    Flag::IgnoreSetTiling,
    Flag::IgnoreSetDomain,
    Flag::IgnoreUmdMetadata,
    Flag::IgnoreCpuAccess,
];

pub fn flags(list: &[Flag]) -> Flags {
    let mut flags = Flags::default();
    for &flag in list {
        flags.insert(flag);
    }
    flags
}

pub fn vulkan_flags() -> Flags {
    flags(&VULKAN_FLAGS)
}

fn driver_name(fd: BorrowedFd<'_>) -> String {
    let mut name = [0u8; DRIVER_NAME_CAPACITY];
    let mut version = drm_version {
        version_major: 0,
        version_minor: 0,
        version_patchlevel: 0,
        name_len: name.len(),
        name: name.as_mut_ptr().cast(),
        date_len: 0,
        date: std::ptr::null_mut(),
        desc_len: 0,
        desc: std::ptr::null_mut(),
    };
    let request = DRM_IOCTL_VERSION as libc::Ioctl;
    let result = unsafe { libc::ioctl(fd.as_raw_fd(), request, &mut version) };
    assert_eq!(
        result,
        0,
        "DRM_IOCTL_VERSION failed: {}",
        std::io::Error::last_os_error()
    );
    let len = version.name_len.min(name.len());
    String::from_utf8_lossy(&name[..len]).into_owned()
}

pub fn device(fd: BorrowedFd<'_>, node: u32, kind: Node) -> vectors::Device {
    vectors::Device {
        driver: driver_name(fd),
        fake: drm::kernel_id(node),
        node: kind,
    }
}

pub fn exported_calls(traced: Vec<Traced>) -> Vec<Call> {
    let exported = traced
        .iter()
        .rev()
        .find(|traced| matches!(traced.call, Call::PrimeHandleToFd { .. }))
        .expect("no export recorded")
        .handle;
    traced
        .into_iter()
        .filter(|traced| traced.handle == exported)
        .map(|traced| traced.call)
        .collect()
}
