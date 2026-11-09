use crate::errno::Errno;
use crate::fd::OwnedFd;
use crate::kernel::Kernel;
use crate::sys::_IOW;
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use core::ffi::c_int;
use core::ptr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DrmError {
    #[error("the DRM_IOCTL_VERSION ioctl failed")]
    Version(#[source] Errno),
    #[error("the DRM_IOCTL_PRIME_HANDLE_TO_FD ioctl failed")]
    PrimeHandleToFd(#[source] Errno),
    #[error("the DRM_IOCTL_MODE_CREATE_DUMB ioctl failed")]
    CreateDumb(#[source] Errno),
}

pub(crate) const DRM_IOCTL_BASE: u32 = b'd' as u32;
pub(crate) const DRM_COMMAND_BASE: u32 = 0x40;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub(crate) struct GemHandle(pub(crate) u32);

#[repr(C)]
struct drm_version {
    version_major: c_int,
    version_minor: c_int,
    version_patchlevel: c_int,
    name_len: usize,
    name: *mut u8,
    date_len: usize,
    date: *mut u8,
    desc_len: usize,
    desc: *mut u8,
}

#[repr(C)]
struct drm_gem_close {
    handle: GemHandle,
    pad: u32,
}

#[repr(C)]
struct drm_prime_handle {
    handle: GemHandle,
    flags: u32,
    fd: i32,
}

#[repr(C)]
struct drm_mode_create_dumb {
    height: u32,
    width: u32,
    bpp: u32,
    flags: u32,
    handle: GemHandle,
    pitch: u32,
    size: u64,
}

const DRM_IOCTL_VERSION: Ioctl = _IOWR::<drm_version>(DRM_IOCTL_BASE, 0x00);
const DRM_IOCTL_GEM_CLOSE: Ioctl = _IOW::<drm_gem_close>(DRM_IOCTL_BASE, 0x09);
const DRM_IOCTL_PRIME_HANDLE_TO_FD: Ioctl = _IOWR::<drm_prime_handle>(DRM_IOCTL_BASE, 0x2d);
const DRM_IOCTL_MODE_CREATE_DUMB: Ioctl = _IOWR::<drm_mode_create_dumb>(DRM_IOCTL_BASE, 0xb2);

pub(crate) unsafe fn ioctl<K: Kernel, T>(
    fd: &OwnedFd<K>,
    request: Ioctl,
    arg: *mut T,
) -> Result<(), Errno> {
    unsafe { fd.kernel.ioctl(fd.fd, request, arg.cast()).map(drop) }
}

fn version<K: Kernel>(fd: &OwnedFd<K>, name: &mut [u8]) -> Result<drm_version, Errno> {
    let mut version = drm_version {
        version_major: 0,
        version_minor: 0,
        version_patchlevel: 0,
        name_len: name.len(),
        name: name.as_mut_ptr(),
        date_len: 0,
        date: ptr::null_mut(),
        desc_len: 0,
        desc: ptr::null_mut(),
    };
    unsafe {
        ioctl(fd, DRM_IOCTL_VERSION, &mut version)?;
    }
    Ok(version)
}

pub(crate) fn driver_name<'a, K: Kernel>(
    fd: &OwnedFd<K>,
    name: &'a mut [u8; 32],
) -> Result<&'a [u8], DrmError> {
    let version = version(fd, name).map_err(DrmError::Version)?;
    Ok(&name[..version.name_len.min(name.len())])
}

pub(crate) fn version_minor<K: Kernel>(fd: &OwnedFd<K>) -> Result<c_int, DrmError> {
    let version = version(fd, &mut []).map_err(DrmError::Version)?;
    Ok(version.version_minor)
}

pub(crate) fn handle_to_fd<K: Kernel>(
    fd: &OwnedFd<K>,
    handle: GemHandle,
) -> Result<c_int, DrmError> {
    let mut prime = drm_prime_handle {
        handle,
        flags: (libc::O_CLOEXEC | libc::O_RDWR) as u32,
        fd: -1,
    };
    unsafe {
        ioctl(fd, DRM_IOCTL_PRIME_HANDLE_TO_FD, &mut prime).map_err(DrmError::PrimeHandleToFd)?;
    }
    Ok(prime.fd)
}

pub(crate) struct DumbBuffer {
    pub(crate) handle: GemHandle,
    pub(crate) pitch: u32,
    pub(crate) size: u64,
}

pub(crate) fn create_dumb<K: Kernel>(
    fd: &OwnedFd<K>,
    width: u32,
    height: u32,
    bpp: u32,
) -> Result<DumbBuffer, DrmError> {
    let mut create = drm_mode_create_dumb {
        height,
        width,
        bpp,
        flags: 0,
        handle: GemHandle(0),
        pitch: 0,
        size: 0,
    };
    unsafe {
        ioctl(fd, DRM_IOCTL_MODE_CREATE_DUMB, &mut create).map_err(DrmError::CreateDumb)?;
    }
    Ok(DumbBuffer {
        handle: create.handle,
        pitch: create.pitch,
        size: create.size,
    })
}

pub(crate) fn gem_close<K: Kernel>(fd: &OwnedFd<K>, handle: GemHandle) {
    let mut close = drm_gem_close { handle, pad: 0 };
    let _ = unsafe { ioctl(fd, DRM_IOCTL_GEM_CLOSE, &mut close) };
}
