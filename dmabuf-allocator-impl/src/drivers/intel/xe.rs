use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::drivers::intel::Vram;
use crate::drivers::intel::bo_size;
use crate::drivers::intel::choose_layout;
use crate::drivers::intel::device_info;
use crate::drivers::intel::device_info::DeviceInfo;
use crate::drivers::intel::heap;
use crate::drivers::intel::records;
use crate::drivers::intel::wants_compression;
use crate::drivers::intel::zeroed_buffer;
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
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use alloc::sync::Arc;
use alloc::vec::Vec;
use arrayvec::ArrayVec;
use core::ffi::c_int;
use run_on_drop::on_drop;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum XeError {
    #[error("iris does not support the intel device")]
    Device,
    #[error("could not query the size of the device query")]
    QuerySize(#[source] Errno),
    #[error("could not allocate the buffer for the device query")]
    QueryBuffer,
    #[error("could not run the device query")]
    Query(#[source] Errno),
    #[error("the memory region list is incomplete")]
    Regions,
    #[error("the device configuration is incomplete")]
    Config,
    #[error("could not create the buffer")]
    GemCreate(#[source] Errno),
    #[error("could not export the buffer")]
    Export(#[source] DrmError),
}

#[repr(C)]
struct drm_xe_device_query {
    extensions: u64,
    query: u32,
    size: u32,
    data: u64,
    reserved: [u64; 2],
}

#[repr(C)]
struct drm_xe_gem_create {
    extensions: u64,
    size: u64,
    placement: u32,
    flags: u32,
    vm_id: u32,
    handle: GemHandle,
    cpu_caching: u16,
    pad: [u16; 3],
    reserved: [u64; 2],
}

#[repr(C)]
struct drm_xe_query_mem_regions {
    num_mem_regions: u32,
    pad: u32,
}

#[repr(C)]
struct drm_xe_query_config {
    num_params: u32,
    pad: u32,
    info: [u64; 0],
}

#[repr(C)]
struct drm_xe_mem_region {
    mem_class: u16,
    instance: u16,
    min_page_size: u32,
    total_size: u64,
    used: u64,
    cpu_visible_size: u64,
    cpu_visible_used: u64,
    reserved: [u64; 6],
}

const DRM_IOCTL_XE_DEVICE_QUERY: Ioctl =
    _IOWR::<drm_xe_device_query>(DRM_IOCTL_BASE, DRM_COMMAND_BASE);
const DRM_IOCTL_XE_GEM_CREATE: Ioctl =
    _IOWR::<drm_xe_gem_create>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x01);

const DRM_XE_DEVICE_QUERY_MEM_REGIONS: u32 = 1;
const DRM_XE_DEVICE_QUERY_CONFIG: u32 = 2;
const DRM_XE_MEM_REGION_CLASS_SYSMEM: u16 = 0;
const DRM_XE_MEM_REGION_CLASS_VRAM: u16 = 1;
const DRM_XE_QUERY_CONFIG_REV_AND_DEVICE_ID: usize = 0;
const DRM_XE_QUERY_CONFIG_FLAGS: usize = 1;
const DRM_XE_QUERY_CONFIG_FLAG_HAS_VRAM: u64 = 1 << 0;
const DRM_XE_QUERY_CONFIG_FLAG_HAS_NO_COMPRESSION_HINT: u64 = 1 << 3;
const DRM_XE_QUERY_CONFIG_MIN_ALIGNMENT: usize = 2;
const CONFIG_PARAMS: usize = DRM_XE_QUERY_CONFIG_MIN_ALIGNMENT + 1;
const DRM_XE_GEM_CREATE_FLAG_SCANOUT: u32 = 1 << 1;
const DRM_XE_GEM_CREATE_FLAG_NEEDS_VISIBLE_VRAM: u32 = 1 << 2;
const DRM_XE_GEM_CREATE_FLAG_NO_COMPRESSION: u32 = 1 << 3;
const DRM_XE_GEM_CPU_CACHING_WC: u16 = 2;

pub(crate) struct Device<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
    info: Option<DeviceInfo>,
    system_placement: u32,
    vram_placement: u32,
    vram: Vram,
    mem_alignment: u64,
    no_compression_hint: bool,
}

fn query<K: Kernel>(fd: &OwnedFd<K>, query: u32) -> Result<Vec<u8>, XeError> {
    let mut device_query = drm_xe_device_query {
        extensions: 0,
        query,
        size: 0,
        data: 0,
        reserved: [0; 2],
    };
    unsafe {
        drm::ioctl(fd, DRM_IOCTL_XE_DEVICE_QUERY, &mut device_query).map_err(XeError::QuerySize)?;
    }
    let mut buffer = zeroed_buffer(device_query.size as usize).ok_or(XeError::QueryBuffer)?;
    device_query.data = buffer.as_mut_ptr().expose_provenance() as u64;
    unsafe {
        drm::ioctl(fd, DRM_IOCTL_XE_DEVICE_QUERY, &mut device_query).map_err(XeError::Query)?;
    }
    buffer.truncate(device_query.size as usize);
    Ok(buffer)
}

impl<K: Kernel> Device<K> {
    pub(crate) fn new(fd: &Arc<OwnedFd<K>>) -> Result<Self, XeError> {
        let regions = query(fd, DRM_XE_DEVICE_QUERY_MEM_REGIONS)?;
        let regions = unsafe {
            records::<drm_xe_query_mem_regions, drm_xe_mem_region>(&regions, |header| {
                header.num_mem_regions
            })
        }
        .ok_or(XeError::Regions)?;
        let mut system_placement = 1;
        let mut vram_placement = 0;
        let mut vram = Vram {
            size: 0,
            all_mappable: true,
        };
        for region in regions {
            let placement = 1u32.checked_shl(region.instance.into()).unwrap_or(0);
            match region.mem_class {
                DRM_XE_MEM_REGION_CLASS_SYSMEM => system_placement = placement,
                DRM_XE_MEM_REGION_CLASS_VRAM => {
                    vram_placement = placement;
                    vram = Vram {
                        size: region.total_size,
                        all_mappable: region.total_size == region.cpu_visible_size,
                    };
                }
                _ => {}
            }
        }
        let config = query(fd, DRM_XE_DEVICE_QUERY_CONFIG)?;
        let params: ArrayVec<u64, CONFIG_PARAMS> = unsafe {
            records::<drm_xe_query_config, u64>(&config, |header| header.num_params)
                .ok_or(XeError::Config)?
                .take(CONFIG_PARAMS)
                .collect()
        };
        let param = |index: usize| params.get(index).copied().ok_or(XeError::Config);
        let pci_id = param(DRM_XE_QUERY_CONFIG_REV_AND_DEVICE_ID)? as u16;
        let mut info = device_info(pci_id).map_err(|_| XeError::Device)?.copied();
        let flags = param(DRM_XE_QUERY_CONFIG_FLAGS)?;
        if let Some(info) = &mut info {
            info.has_local_mem |= flags & DRM_XE_QUERY_CONFIG_FLAG_HAS_VRAM != 0;
        }
        Ok(Device {
            fd: fd.clone(),
            info,
            system_placement,
            vram_placement,
            vram,
            mem_alignment: param(DRM_XE_QUERY_CONFIG_MIN_ALIGNMENT)?.max(1),
            no_compression_hint: flags & DRM_XE_QUERY_CONFIG_FLAG_HAS_NO_COMPRESSION_HINT != 0,
        })
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        choose_layout(config, self.info.as_ref(), false)
    }

    pub(crate) fn allocate(
        &self,
        device_layout: &DeviceLayout,
        config: &Config<'_>,
    ) -> Result<c_int, XeError> {
        let layout = &device_layout.layout;
        let info = self.info.as_ref();
        let heap = heap(info, &self.vram, layout.modifier);
        let mut placement = 0;
        if heap.vram {
            placement |= self.vram_placement;
        }
        if heap.system {
            placement |= self.system_placement;
        }
        let mut size = bo_size(layout, info, false, self.fd.kernel.page_size());
        let mut flags = 0;
        if config.scanout {
            flags |= DRM_XE_GEM_CREATE_FLAG_SCANOUT;
            if info
                .is_some_and(|info| info.has_local_mem && wants_compression(info, layout.modifier))
            {
                size = align(size, 64 * 1024);
            }
        }
        if heap.needs_visible_vram {
            flags |= DRM_XE_GEM_CREATE_FLAG_NEEDS_VISIBLE_VRAM;
        }
        if !heap.compressed && self.no_compression_hint {
            flags |= DRM_XE_GEM_CREATE_FLAG_NO_COMPRESSION;
        }
        let mut create = drm_xe_gem_create {
            extensions: 0,
            size: align(size, self.mem_alignment),
            placement,
            flags,
            vm_id: 0,
            handle: GemHandle(0),
            cpu_caching: DRM_XE_GEM_CPU_CACHING_WC,
            pad: [0; 3],
            reserved: [0; 2],
        };
        unsafe {
            drm::ioctl(&self.fd, DRM_IOCTL_XE_GEM_CREATE, &mut create)
                .map_err(XeError::GemCreate)?;
        }
        let handle = create.handle;
        let _close = on_drop(|| drm::gem_close(&self.fd, handle));
        drm::handle_to_fd(&self.fd, handle).map_err(XeError::Export)
    }
}
