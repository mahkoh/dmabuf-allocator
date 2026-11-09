use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::allocator::Extra;
use crate::drm;
use crate::drm::DRM_COMMAND_BASE;
use crate::drm::DRM_IOCTL_BASE;
use crate::drm::DrmError;
use crate::drm::GemHandle;
use crate::errno::Errno;
use crate::fd::OwnedFd;
use crate::kernel::Kernel;
use crate::layout::DRM_FORMAT_MOD_LINEAR;
use crate::layout::DRM_FORMAT_MOD_VENDOR_BROADCOM;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::PAGE_SIZE;
use crate::layout::PlaneLayout;
use crate::layout::PlaneVec;
use crate::layout::align;
use crate::layout::fourcc_mod_code;
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use alloc::sync::Arc;
use core::ffi::c_int;
use run_on_drop::on_drop;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Vc4Error {
    #[error("could not create the buffer")]
    CreateBo(#[source] Errno),
    #[error("could not set the tiling of the buffer")]
    SetTiling(#[source] Errno),
    #[error("could not export the buffer")]
    Export(#[source] DrmError),
}

const DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_BROADCOM, 1);

#[repr(C)]
struct drm_vc4_create_bo {
    size: u32,
    flags: u32,
    handle: GemHandle,
    pad: u32,
}

#[repr(C)]
struct drm_vc4_get_tiling {
    handle: GemHandle,
    flags: u32,
    modifier: u64,
}

#[repr(C)]
struct drm_vc4_set_tiling {
    handle: GemHandle,
    flags: u32,
    modifier: u64,
}

const DRM_IOCTL_VC4_CREATE_BO: Ioctl =
    _IOWR::<drm_vc4_create_bo>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x03);
const DRM_IOCTL_VC4_SET_TILING: Ioctl =
    _IOWR::<drm_vc4_set_tiling>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x08);
const DRM_IOCTL_VC4_GET_TILING: Ioctl =
    _IOWR::<drm_vc4_get_tiling>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x09);

pub(crate) struct Device<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
    has_tiling_ioctl: bool,
}

impl<K: Kernel> Device<K> {
    pub(crate) fn new(fd: &Arc<OwnedFd<K>>) -> Self {
        let mut get_tiling = drm_vc4_get_tiling {
            handle: GemHandle(0),
            flags: 0,
            modifier: 0,
        };
        let probe = unsafe { drm::ioctl(fd, DRM_IOCTL_VC4_GET_TILING, &mut get_tiling) };
        Self {
            fd: fd.clone(),
            has_tiling_ioctl: probe == Err(Errno(libc::ENOENT)),
        }
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        let fmt = config.format.0;
        let extents = 1..=2048;
        if !fmt.vc4 {
            return Err(LayoutError::Format);
        }
        if !extents.contains(&config.width) || !extents.contains(&config.height) {
            return Err(LayoutError::Size);
        }
        let [plane] = fmt.planes else {
            return Err(LayoutError::Format);
        };
        let cpp = u64::from(plane.bits / 8);
        let (utile_w, utile_h) = match cpp {
            1 => (8, 8),
            2 => (8, 4),
            4 => (4, 4),
            8 => (2, 4),
            _ => return Err(LayoutError::Format),
        };
        let width = u64::from(config.width);
        let height = u64::from(config.height);
        let requested = |modifier| config.modifiers.contains(&modifier);
        let (modifier, width, height) = if requested(DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED)
            && self.has_tiling_ioctl
            && !config.implicit_interop
            && width > 4 * utile_w
            && height > 4 * utile_h
        {
            (
                DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED,
                align(width, 8 * utile_w),
                align(height, 8 * utile_h),
            )
        } else if requested(DRM_FORMAT_MOD_LINEAR) {
            (DRM_FORMAT_MOD_LINEAR, align(width, utile_w), height)
        } else {
            return Err(LayoutError::Modifier);
        };
        let stride = width * cpp;
        Ok(DeviceLayout {
            layout: Layout {
                format: fmt,
                modifier,
                planes: PlaneVec::from_iter([PlaneLayout {
                    offset: 0,
                    stride,
                    size: height * stride,
                }]),
            },
            extra: Extra::None,
        })
    }

    pub(crate) fn allocate(&self, device_layout: &DeviceLayout) -> Result<c_int, Vc4Error> {
        let mut create = drm_vc4_create_bo {
            size: align(device_layout.layout.planes[0].size, PAGE_SIZE) as u32,
            flags: 0,
            handle: GemHandle(0),
            pad: 0,
        };
        unsafe {
            drm::ioctl(&self.fd, DRM_IOCTL_VC4_CREATE_BO, &mut create)
                .map_err(Vc4Error::CreateBo)?;
        }
        let handle = create.handle;
        let _close = on_drop(|| drm::gem_close(&self.fd, handle));
        if self.has_tiling_ioctl {
            let mut set_tiling = drm_vc4_set_tiling {
                handle,
                flags: 0,
                modifier: device_layout.layout.modifier,
            };
            unsafe {
                drm::ioctl(&self.fd, DRM_IOCTL_VC4_SET_TILING, &mut set_tiling)
                    .map_err(Vc4Error::SetTiling)?;
            }
        }
        drm::handle_to_fd(&self.fd, handle).map_err(Vc4Error::Export)
    }
}
