use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::allocator::Extra;
use crate::drivers::open_render_node;
use crate::drm;
use crate::drm::DRM_COMMAND_BASE;
use crate::drm::DRM_IOCTL_BASE;
use crate::drm::DrmError;
use crate::drm::GemHandle;
use crate::errno::Errno;
use crate::fd::OwnedFd;
use crate::kernel::Kernel;
use crate::layout::DRM_FORMAT_MOD_LINEAR;
use crate::layout::DRM_FORMAT_MOD_VENDOR_APPLE;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::PlaneLayout;
use crate::layout::PlaneVec;
use crate::layout::fourcc_mod_code;
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use crate::vdrm::BlobCreate;
use crate::vdrm::VIRTGPU_BLOB_FLAG_USE_MAPPABLE;
use crate::vdrm::VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
use crate::vdrm::Vdrm;
use crate::vdrm::VdrmError;
use crate::vdrm::vdrm_ccmd_req;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::ffi::c_int;
use run_on_drop::on_drop;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AsahiError {
    #[error("could not open the render node")]
    RenderNode(#[source] Errno),
    #[error("could not create the buffer")]
    GemCreate(#[source] Errno),
    #[error("could not export the buffer")]
    Export(#[source] DrmError),
    #[error("could not connect to the virtio host")]
    VirtioConnect(#[source] VdrmError),
    #[error("could not create the virtio buffer")]
    VirtioBlob(#[source] VdrmError),
}

const DRM_FORMAT_MOD_APPLE_GPU_TILED: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_APPLE, 1);
const DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_APPLE, 2);

const AIL_CACHELINE: u32 = 0x80;
const AIL_PAGESIZE: u32 = 0x4000;
const MAX_EXTENT: u32 = 16384;
const MAX_SIZE: u64 = 1 << 32;
const MAX_TILE_WIDTH: u32 = 128;
const COMPRESSION_TILE: u32 = 16;
const METADATA_BYTES_PER_TILE: u32 = 8;

#[repr(C)]
struct drm_asahi_gem_create {
    size: u64,
    flags: u32,
    vm_id: u32,
    handle: GemHandle,
    pad: u32,
}

const DRM_IOCTL_ASAHI_GEM_CREATE: Ioctl =
    _IOWR::<drm_asahi_gem_create>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x05);

const ASAHI_CCMD_GEM_NEW: u32 = 4;

#[repr(C)]
struct asahi_ccmd_gem_new_req {
    hdr: vdrm_ccmd_req,
    flags: u32,
    bind_flags: u32,
    vm_id: u32,
    blob_id: u32,
    size: u64,
    addr: u64,
}

pub(crate) struct Device<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
    vdrm: Option<Box<Vdrm<K>>>,
}

impl<K: Kernel> Device<K> {
    pub(crate) fn new(fd: &Arc<OwnedFd<K>>) -> Result<Self, AsahiError> {
        Ok(Self {
            fd: open_render_node(fd).map_err(AsahiError::RenderNode)?,
            vdrm: None,
        })
    }

    pub(crate) fn virtio(fd: &Arc<OwnedFd<K>>) -> Result<Self, AsahiError> {
        let mut device = Self::new(fd)?;
        device.vdrm = Some(
            Vdrm::new(&device.fd)
                .map(Box::new)
                .map_err(AsahiError::VirtioConnect)?,
        );
        Ok(device)
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        if !config.format.0.asahi {
            return Err(LayoutError::Format);
        }
        let [block] = config.format.0.planes else {
            return Err(LayoutError::Format);
        };
        if block.block_width != 1 || block.block_height != 1 || !block.bits.is_multiple_of(8) {
            return Err(LayoutError::Format);
        }
        let bytes = block.bits / 8;
        let (width, height) = (config.width, config.height);
        if !matches!(bytes, 1 | 2 | 4 | 8 | 16 | 32 | 64) {
            return Err(LayoutError::Format);
        }
        if !(1..=MAX_EXTENT).contains(&width) || !(1..=MAX_EXTENT).contains(&height) {
            return Err(LayoutError::Size);
        }
        let tiled = !config.implicit_interop;
        let compressed = tiled && width >= COMPRESSION_TILE && height >= COMPRESSION_TILE;
        let candidates = [
            (compressed, DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED),
            (tiled, DRM_FORMAT_MOD_APPLE_GPU_TILED),
            (true, DRM_FORMAT_MOD_LINEAR),
        ];
        let (_, modifier) = candidates
            .into_iter()
            .find(|&(ok, modifier)| ok && config.modifiers.contains(&modifier))
            .ok_or(LayoutError::Modifier)?;
        let tile_width = MAX_TILE_WIDTH >> (bytes.ilog2() / 2);
        let tile_height = AIL_PAGESIZE / bytes / tile_width;
        let tiled_size = if width >= tile_width && height >= tile_height {
            width.div_ceil(tile_width) as u64
                * height.div_ceil(tile_height) as u64
                * AIL_PAGESIZE as u64
        } else {
            (width.next_power_of_two() * height.next_power_of_two() * bytes) as u64
        };
        let tiles = |px: u32| px.div_ceil(COMPRESSION_TILE).next_power_of_two();
        let mut stride = (width * bytes) as u64;
        let size = match modifier {
            DRM_FORMAT_MOD_APPLE_GPU_TILED => tiled_size.next_multiple_of(AIL_PAGESIZE as u64),
            DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED => {
                tiled_size
                    + (tiles(width) * tiles(height) * METADATA_BYTES_PER_TILE)
                        .next_multiple_of(AIL_CACHELINE) as u64
            }
            _ => {
                stride = stride.next_multiple_of(AIL_CACHELINE as u64);
                stride * height as u64
            }
        };
        if size >= MAX_SIZE {
            return Err(LayoutError::TooLarge);
        }
        Ok(DeviceLayout {
            layout: Layout {
                format: config.format.0,
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

    pub(crate) fn allocate(&self, device_layout: &DeviceLayout) -> Result<c_int, AsahiError> {
        let size = device_layout.layout.planes[0]
            .size
            .next_multiple_of(AIL_PAGESIZE as u64);
        let handle = match &self.vdrm {
            None => {
                let mut gem_create = drm_asahi_gem_create {
                    size,
                    flags: 0,
                    vm_id: 0,
                    handle: GemHandle(0),
                    pad: 0,
                };
                unsafe {
                    drm::ioctl(&self.fd, DRM_IOCTL_ASAHI_GEM_CREATE, &mut gem_create)
                        .map_err(AsahiError::GemCreate)?;
                }
                gem_create.handle
            }
            Some(vdrm) => {
                let size = vdrm.blob_size(size);
                let blob_id = vdrm.next_blob_id();
                let req = asahi_ccmd_gem_new_req {
                    hdr: vdrm_ccmd_req::new(ASAHI_CCMD_GEM_NEW),
                    flags: 0,
                    bind_flags: 0,
                    vm_id: 0,
                    blob_id: blob_id as u32,
                    size,
                    addr: 0,
                };
                let params = BlobCreate {
                    size,
                    blob_flags: VIRTGPU_BLOB_FLAG_USE_MAPPABLE | VIRTGPU_BLOB_FLAG_USE_SHAREABLE,
                    blob_id,
                    blob_hints: 0,
                };
                vdrm.bo_create(&params, &req, &[])
                    .map_err(AsahiError::VirtioBlob)?
                    .bo_handle
            }
        };
        let _close = on_drop(|| drm::gem_close(&self.fd, handle));
        drm::handle_to_fd(&self.fd, handle).map_err(AsahiError::Export)
    }
}
