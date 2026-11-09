use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::drivers::nvidia;
use crate::drivers::nvidia::COMPRESSION;
use crate::drivers::nvidia::GOB_WIDTH_BYTES;
use crate::drivers::nvidia::LOG2_GOBS;
use crate::drivers::nvidia::PAGE_KIND;
use crate::drivers::nvidia::gob_height;
use crate::drivers::nvidia::log2_gobs;
use crate::drivers::open_render_node;
use crate::drm;
use crate::drm::DRM_COMMAND_BASE;
use crate::drm::DRM_IOCTL_BASE;
use crate::drm::DrmError;
use crate::drm::GemHandle;
use crate::errno::Errno;
use crate::fd::OwnedFd;
use crate::kernel::Kernel;
use crate::layout::LayoutError;
use crate::layout::align;
use crate::layout::end;
use crate::layout::field;
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use alloc::sync::Arc;
use core::ffi::c_int;
use run_on_drop::on_drop;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NouveauError {
    #[error("could not open the render node")]
    RenderNode(#[source] Errno),
    #[error("could not query the chipset")]
    GetParam(#[source] Errno),
    #[error("could not create the buffer")]
    GemNew(#[source] Errno),
    #[error("could not export the buffer")]
    Export(#[source] DrmError),
}

#[repr(C)]
struct drm_nouveau_gem_info {
    handle: GemHandle,
    domain: u32,
    size: u64,
    offset: u64,
    map_handle: u64,
    tile_mode: u32,
    tile_flags: u32,
}

#[repr(C)]
struct drm_nouveau_gem_new {
    info: drm_nouveau_gem_info,
    channel_hint: u32,
    align: u32,
}

#[repr(C)]
struct drm_nouveau_vm_init {
    kernel_managed_addr: u64,
    kernel_managed_size: u64,
}

#[repr(C)]
struct drm_nouveau_getparam {
    param: u64,
    value: u64,
}

const COMPRESSED_BONUS: u32 = 10;

const DRM_NOUVEAU_GETPARAM: u32 = 0x00;
const DRM_NOUVEAU_VM_INIT: u32 = 0x10;
const DRM_NOUVEAU_GEM_NEW: u32 = 0x40;

const DRM_IOCTL_NOUVEAU_GETPARAM: Ioctl =
    _IOWR::<drm_nouveau_getparam>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + DRM_NOUVEAU_GETPARAM);
const DRM_IOCTL_NOUVEAU_VM_INIT: Ioctl =
    _IOWR::<drm_nouveau_vm_init>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + DRM_NOUVEAU_VM_INIT);
const DRM_IOCTL_NOUVEAU_GEM_NEW: Ioctl =
    _IOWR::<drm_nouveau_gem_new>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_NEW);

const NOUVEAU_GEM_DOMAIN_VRAM: u32 = 1 << 1;
const NOUVEAU_GEM_DOMAIN_GART: u32 = 1 << 2;
const NOUVEAU_GETPARAM_CHIPSET_ID: u64 = 11;

const KERNEL_MANAGED_START: u64 = 1 << 39;
const VA_SPACE_END: u64 = 1 << 40;

const MAX_EXTENT: u32 = 16384;
const MAX_EXTENT_PASCAL_B: u32 = 32768;
const PASCAL_B_CHIPSETS: [u64; 5] = [0x132, 0x134, 0x136, 0x137, 0x138];
const GV100_CHIPSET: u64 = 0x140;

fn rank(modifier: u64) -> u32 {
    let log2_gobs = field(modifier, LOG2_GOBS) as u32;
    match field(modifier, COMPRESSION) {
        0 => log2_gobs,
        _ => log2_gobs + COMPRESSED_BONUS,
    }
}

pub(crate) struct Device<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
    max_extent: u32,
}

impl<K: Kernel> Device<K> {
    pub(crate) fn new(fd: &Arc<OwnedFd<K>>) -> Result<Self, NouveauError> {
        let fd = open_render_node(fd).map_err(NouveauError::RenderNode)?;
        let mut init = drm_nouveau_vm_init {
            kernel_managed_addr: KERNEL_MANAGED_START,
            kernel_managed_size: VA_SPACE_END - KERNEL_MANAGED_START,
        };
        let _ = unsafe { drm::ioctl(&fd, DRM_IOCTL_NOUVEAU_VM_INIT, &mut init) };
        let mut chipset = drm_nouveau_getparam {
            param: NOUVEAU_GETPARAM_CHIPSET_ID,
            value: 0,
        };
        unsafe {
            drm::ioctl(&fd, DRM_IOCTL_NOUVEAU_GETPARAM, &mut chipset)
                .map_err(NouveauError::GetParam)?;
        }
        let max_extent =
            match PASCAL_B_CHIPSETS.contains(&chipset.value) || chipset.value >= GV100_CHIPSET {
                true => MAX_EXTENT_PASCAL_B,
                false => MAX_EXTENT,
            };
        Ok(Self { fd, max_extent })
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        nvidia::layout(config, config.format.0.nouveau, self.max_extent, rank)
    }

    pub(crate) fn allocate(
        &self,
        device_layout: &DeviceLayout,
        config: &Config<'_>,
    ) -> Result<c_int, NouveauError> {
        let layout = &device_layout.layout;
        let log2_gobs = log2_gobs(config, layout.modifier, &layout.format.planes[0]);
        let alignment = (GOB_WIDTH_BYTES * (gob_height(layout.modifier) << log2_gobs))
            .max(self.fd.kernel.page_size());
        let mut new = drm_nouveau_gem_new {
            info: drm_nouveau_gem_info {
                handle: GemHandle(0),
                domain: NOUVEAU_GEM_DOMAIN_VRAM | NOUVEAU_GEM_DOMAIN_GART,
                size: align(end(&layout.planes), alignment),
                offset: 0,
                map_handle: 0,
                tile_mode: (log2_gobs as u32) << 4,
                tile_flags: (field(layout.modifier, PAGE_KIND) as u32) << 8,
            },
            channel_hint: 0,
            align: alignment as u32,
        };
        unsafe {
            drm::ioctl(&self.fd, DRM_IOCTL_NOUVEAU_GEM_NEW, &mut new)
                .map_err(NouveauError::GemNew)?;
        }
        let handle = new.info.handle;
        let _close = on_drop(|| drm::gem_close(&self.fd, handle));
        drm::handle_to_fd(&self.fd, handle).map_err(NouveauError::Export)
    }
}
