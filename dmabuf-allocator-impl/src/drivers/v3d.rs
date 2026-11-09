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
pub enum V3dError {
    #[error("could not create the buffer")]
    CreateBo(#[source] Errno),
    #[error("could not export the buffer")]
    Export(#[source] DrmError),
}

const DRM_FORMAT_MOD_BROADCOM_UIF: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_BROADCOM, 6);

const V3D_TFU_READAHEAD_SIZE: u64 = 64;
const PAGE_UB_ROWS_TIMES_1_5: u64 = 6;
const PAGE_CACHE_UB_ROWS: u64 = 32;
const PAGE_CACHE_MINUS_1_5_UB_ROWS: u64 = PAGE_CACHE_UB_ROWS - PAGE_UB_ROWS_TIMES_1_5;

#[repr(C)]
struct drm_v3d_create_bo {
    size: u32,
    flags: u32,
    handle: GemHandle,
    offset: u32,
}

const DRM_IOCTL_V3D_CREATE_BO: Ioctl =
    _IOWR::<drm_v3d_create_bo>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x02);

pub(crate) struct Device<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
}

impl<K: Kernel> Device<K> {
    pub(crate) fn new(fd: &Arc<OwnedFd<K>>) -> Self {
        Self { fd: fd.clone() }
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        let format = config.format.0;
        let extents = 1..=4096;
        if !format.v3d {
            return Err(LayoutError::Format);
        }
        if !extents.contains(&config.width) || !extents.contains(&config.height) {
            return Err(LayoutError::Size);
        }
        let [plane] = format.planes else {
            return Err(LayoutError::Format);
        };
        let cpp = u64::from(plane.bits / 8);
        let (block_w, block_h) = match cpp {
            1 => (64, 16),
            2 => (64, 8),
            4 => (32, 8),
            8 => (32, 4),
            16 => (16, 4),
            _ => return Err(LayoutError::Format),
        };
        let mut width = u64::from(config.width).div_ceil(u64::from(plane.block_width));
        let mut height = u64::from(config.height).div_ceil(u64::from(plane.block_height));
        let requested = |modifier| config.modifiers.contains(&modifier);
        let tile = !(config.scanout || config.implicit_interop);
        let modifier = if tile && requested(DRM_FORMAT_MOD_BROADCOM_UIF) {
            width = align(width, block_w);
            let rows = height.div_ceil(block_h);
            height = block_h
                * match rows % PAGE_CACHE_UB_ROWS {
                    offset @ 1..PAGE_UB_ROWS_TIMES_1_5 if rows > PAGE_CACHE_UB_ROWS => {
                        rows - offset + PAGE_UB_ROWS_TIMES_1_5
                    }
                    offset if offset > PAGE_CACHE_MINUS_1_5_UB_ROWS => {
                        align(rows, PAGE_CACHE_UB_ROWS)
                    }
                    _ => rows,
                };
            DRM_FORMAT_MOD_BROADCOM_UIF
        } else if requested(DRM_FORMAT_MOD_LINEAR) {
            DRM_FORMAT_MOD_LINEAR
        } else {
            return Err(LayoutError::Modifier);
        };
        let stride = width * cpp;
        let plane = PlaneLayout {
            offset: 0,
            stride,
            size: height * stride,
        };
        Ok(DeviceLayout {
            layout: Layout {
                format,
                modifier,
                planes: PlaneVec::from_iter([plane]),
            },
            extra: Extra::None,
        })
    }

    pub(crate) fn allocate(&self, device_layout: &DeviceLayout) -> Result<c_int, V3dError> {
        let size = align(
            device_layout.layout.planes[0].size + V3D_TFU_READAHEAD_SIZE,
            PAGE_SIZE,
        );
        let mut create = drm_v3d_create_bo {
            size: size as u32,
            flags: 0,
            handle: GemHandle(0),
            offset: 0,
        };
        unsafe {
            drm::ioctl(&self.fd, DRM_IOCTL_V3D_CREATE_BO, &mut create)
                .map_err(V3dError::CreateBo)?;
        }
        let _close = on_drop(|| drm::gem_close(&self.fd, create.handle));
        drm::handle_to_fd(&self.fd, create.handle).map_err(V3dError::Export)
    }
}
