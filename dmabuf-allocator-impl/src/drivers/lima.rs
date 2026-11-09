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
use crate::layout::DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED;
use crate::layout::DRM_FORMAT_MOD_LINEAR;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::PAGE_SIZE;
use crate::layout::PlaneLayout;
use crate::layout::PlaneVec;
use crate::layout::align;
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use alloc::sync::Arc;
use core::ffi::c_int;
use run_on_drop::on_drop;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LimaError {
    #[error("could not create the buffer")]
    GemCreate(#[source] Errno),
    #[error("could not export the buffer")]
    Export(#[source] DrmError),
}

#[repr(C)]
struct drm_lima_gem_create {
    size: u32,
    flags: u32,
    handle: GemHandle,
    pad: u32,
}

const DRM_IOCTL_LIMA_GEM_CREATE: Ioctl =
    _IOWR::<drm_lima_gem_create>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x01);

pub(crate) struct Device<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
}

impl<K: Kernel> Device<K> {
    pub(crate) fn new(fd: &Arc<OwnedFd<K>>) -> Self {
        Self { fd: fd.clone() }
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        let fmt = config.format.0;
        let extents = 1..=4096;
        if !fmt.lima {
            return Err(LayoutError::Format);
        }
        if !extents.contains(&config.width) || !extents.contains(&config.height) {
            return Err(LayoutError::Size);
        }
        let requested = |modifier| config.modifiers.contains(&modifier);
        let modifier = if requested(DRM_FORMAT_MOD_LINEAR) {
            DRM_FORMAT_MOD_LINEAR
        } else if !config.implicit_interop
            && !config.scanout
            && requested(DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED)
        {
            DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED
        } else {
            return Err(LayoutError::Modifier);
        };
        let stride = align(config.width as u64, 16) * (fmt.planes[0].bits as u64 / 8);
        let size = stride * align(config.height as u64, 16);
        Ok(DeviceLayout {
            layout: Layout {
                format: fmt,
                modifier,
                planes: PlaneVec::from_iter([PlaneLayout {
                    offset: 0,
                    stride,
                    size,
                }]),
            },
            extra: Extra::None,
        })
    }

    pub(crate) fn allocate(&self, device_layout: &DeviceLayout) -> Result<c_int, LimaError> {
        let mut create = drm_lima_gem_create {
            size: align(device_layout.layout.planes[0].size, PAGE_SIZE) as u32,
            flags: 0,
            handle: GemHandle(0),
            pad: 0,
        };
        unsafe {
            drm::ioctl(&self.fd, DRM_IOCTL_LIMA_GEM_CREATE, &mut create)
                .map_err(LimaError::GemCreate)?;
        }
        let _close = on_drop(|| drm::gem_close(&self.fd, create.handle));
        drm::handle_to_fd(&self.fd, create.handle).map_err(LimaError::Export)
    }
}
