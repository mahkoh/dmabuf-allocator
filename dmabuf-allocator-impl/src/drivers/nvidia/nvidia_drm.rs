use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::drivers::nvidia;
use crate::drivers::nvidia::BLOCK_LINEAR;
use crate::drivers::nvidia::COMPRESSION;
use crate::drivers::nvidia::LOG2_GOBS;
use crate::drivers::nvidia::PLANE_SIZE_ALIGNMENT;
use crate::drm;
use crate::drm::DRM_COMMAND_BASE;
use crate::drm::DRM_IOCTL_BASE;
use crate::drm::DrmError;
use crate::drm::GemHandle;
use crate::errno::Errno;
use crate::fd::OwnedFd;
use crate::kernel::Kernel;
use crate::layout::LayoutError;
use crate::layout::end;
use crate::layout::field;
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use alloc::sync::Arc;
use core::ffi::c_int;
use run_on_drop::on_drop;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NvidiaDrmError {
    #[error("could not create the buffer")]
    GemAlloc(#[source] Errno),
    #[error("could not export the buffer")]
    Export(#[source] DrmError),
    #[error("the buffer is too large")]
    Size,
}

#[repr(C)]
struct drm_nvidia_gem_alloc_nvkms_memory_params {
    handle: GemHandle,
    block_linear: u8,
    compressible: u8,
    pad0: u16,
    memory_size: u64,
    flags: u32,
    pad1: u32,
}

const DRM_NVIDIA_GEM_ALLOC_NVKMS_MEMORY: u32 = 0x0b;

const DRM_IOCTL_NVIDIA_GEM_ALLOC_NVKMS_MEMORY: Ioctl =
    _IOWR::<drm_nvidia_gem_alloc_nvkms_memory_params>(
        DRM_IOCTL_BASE,
        DRM_COMMAND_BASE + DRM_NVIDIA_GEM_ALLOC_NVKMS_MEMORY,
    );

const NV_GEM_ALLOC_NO_SCANOUT: u32 = 1 << 0;

fn rank(config: &Config<'_>, modifier: u64) -> u32 {
    let preferred = match config.height {
        86.. => 4,
        43.. => 3,
        22.. => 2,
        11.. => 1,
        _ => 0,
    };
    if field(modifier, LOG2_GOBS) != preferred {
        return 0;
    }
    match (field(modifier, COMPRESSION), config.scanout) {
        (0, _) => 1,
        (_, false) => 2,
        (_, true) => 0,
    }
}

pub(crate) struct Device<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
}

impl<K: Kernel> Device<K> {
    pub(crate) fn new(fd: &Arc<OwnedFd<K>>) -> Self {
        Device { fd: fd.clone() }
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        if !config.format.0.nvidia {
            return Err(LayoutError::Format);
        }
        nvidia::layout(config, true, u32::MAX, |modifier| rank(config, modifier))
    }

    pub(crate) fn allocate(
        &self,
        device_layout: &DeviceLayout,
        config: &Config<'_>,
    ) -> Result<c_int, NvidiaDrmError> {
        let layout = &device_layout.layout;
        let memory_size = end(&layout.planes)
            .checked_next_multiple_of(PLANE_SIZE_ALIGNMENT)
            .ok_or(NvidiaDrmError::Size)?;
        let flags = match config.scanout {
            true => 0,
            false => NV_GEM_ALLOC_NO_SCANOUT,
        };
        let mut alloc = drm_nvidia_gem_alloc_nvkms_memory_params {
            handle: GemHandle(0),
            block_linear: field(layout.modifier, BLOCK_LINEAR) as u8,
            compressible: (field(layout.modifier, COMPRESSION) != 0) as u8,
            pad0: 0,
            memory_size,
            flags,
            pad1: 0,
        };
        unsafe {
            drm::ioctl(
                &self.fd,
                DRM_IOCTL_NVIDIA_GEM_ALLOC_NVKMS_MEMORY,
                &mut alloc,
            )
            .map_err(NvidiaDrmError::GemAlloc)?;
        }
        let handle = alloc.handle;
        let _close = on_drop(|| drm::gem_close(&self.fd, handle));
        drm::handle_to_fd(&self.fd, handle).map_err(NvidiaDrmError::Export)
    }
}
