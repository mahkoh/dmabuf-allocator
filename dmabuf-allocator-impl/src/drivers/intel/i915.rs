use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::drivers::intel::Vram;
use crate::drivers::intel::bo_size;
use crate::drivers::intel::choose_layout;
use crate::drivers::intel::device_info;
use crate::drivers::intel::device_info::DeviceInfo;
use crate::drivers::intel::heap;
use crate::drivers::intel::layout::I915_FORMAT_MOD_X_TILED;
use crate::drivers::intel::layout::I915_FORMAT_MOD_Y_TILED;
use crate::drivers::intel::layout::I915_FORMAT_MOD_Y_TILED_CCS;
use crate::drivers::intel::layout::I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS;
use crate::drivers::intel::layout::I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS;
use crate::drivers::intel::layout::I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC;
use crate::drivers::intel::records;
use crate::drivers::intel::zeroed_buffer;
use crate::drivers::open_render_node;
use crate::drivers::virtio_gpu::VirtioHandle;
use crate::drm;
use crate::drm::DRM_COMMAND_BASE;
use crate::drm::DRM_IOCTL_BASE;
use crate::drm::DrmError;
use crate::drm::GemHandle;
use crate::errno::Errno;
use crate::fd::OwnedFd;
use crate::kernel::Kernel;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::PAGE_SIZE;
use crate::sys::_IOW;
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use crate::vdrm::Vdrm;
use crate::vdrm::VdrmError;
use crate::vdrm::virgl_renderer_capset_drm;
use alloc::boxed::Box;
use alloc::sync::Arc;
use arrayvec::ArrayVec;
use core::ffi::c_int;
use core::ptr;
use run_on_drop::on_drop;
use thiserror::Error;

#[cfg(test)]
mod tests;
mod virtio;

#[derive(Debug, Error)]
pub enum I915Error {
    #[error("could not query the chipset")]
    GetParam(#[source] Errno),
    #[error("iris does not support the intel device")]
    Device,
    #[error("could not create the buffer with GEM_CREATE")]
    GemCreate(#[source] Errno),
    #[error("could not create the buffer with GEM_CREATE_EXT")]
    GemCreateExt(#[source] Errno),
    #[error("could not export the buffer")]
    Export(#[source] DrmError),
    #[error("could not open the render node")]
    RenderNode(#[source] Errno),
    #[error("could not connect to the virtio host")]
    VirtioConnect(#[source] VdrmError),
    #[error("could not create the virtio buffer")]
    VirtioBlob(#[source] VdrmError),
    #[error("could not send the queued commands to the virtio host")]
    VirtioFlush(#[source] VdrmError),
    #[error("could not query the local memory regions")]
    MemoryRegions(#[source] I915QueryError),
}

#[derive(Debug, Error)]
pub enum I915QueryError {
    #[error("the query ioctl failed")]
    Ioctl(#[source] Errno),
    #[error("the kernel rejected the query")]
    Rejected(#[source] Errno),
    #[error("could not send the query to the virtio host")]
    VirtioSend(#[source] VdrmError),
    #[error("the virtio host could not run the query")]
    VirtioHost(#[source] Errno),
    #[error("could not allocate the buffer for the query")]
    Buffer,
    #[error("the query result is incomplete")]
    Incomplete,
}

#[repr(C)]
struct drm_i915_getparam {
    param: i32,
    value: *mut c_int,
}

#[repr(C)]
struct drm_i915_gem_create {
    size: u64,
    handle: GemHandle,
    pad: u32,
}

#[repr(C)]
struct drm_i915_gem_create_ext {
    size: u64,
    handle: GemHandle,
    flags: u32,
    extensions: u64,
}

#[repr(C)]
struct i915_user_extension {
    next_extension: u64,
    name: u32,
    flags: u32,
    rsvd: [u32; 4],
}

#[repr(C)]
struct drm_i915_gem_create_ext_memory_regions {
    base: i915_user_extension,
    pad: u32,
    num_regions: u32,
    regions: u64,
}

#[repr(C)]
struct drm_i915_gem_create_ext_set_pat {
    base: i915_user_extension,
    pat_index: u32,
    rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct drm_i915_gem_memory_class_instance {
    memory_class: u16,
    memory_instance: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct drm_i915_gem_set_tiling {
    handle: IoctlHandle,
    tiling_mode: u32,
    stride: u32,
    swizzle_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct drm_i915_gem_get_tiling {
    handle: IoctlHandle,
    tiling_mode: u32,
    swizzle_mode: u32,
    phys_swizzle_mode: u32,
}

#[repr(C)]
struct drm_i915_gem_set_domain {
    handle: IoctlHandle,
    read_domains: u32,
    write_domain: u32,
}

#[repr(C)]
struct drm_i915_query {
    num_items: u32,
    flags: u32,
    items_ptr: u64,
}

#[repr(C)]
struct drm_i915_query_item {
    query_id: u64,
    length: i32,
    flags: u32,
    data_ptr: u64,
}

#[repr(C)]
struct drm_i915_query_memory_regions {
    num_regions: u32,
    rsvd: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct drm_i915_memory_region_info_cpu_visible {
    probed_cpu_visible_size: u64,
    unallocated_cpu_visible_size: u64,
}

#[repr(C)]
union drm_i915_memory_region_info_tail {
    rsvd1: [u64; 8],
    cpu_visible: drm_i915_memory_region_info_cpu_visible,
}

#[repr(C)]
struct drm_i915_memory_region_info {
    region: drm_i915_gem_memory_class_instance,
    rsvd0: u32,
    probed_size: u64,
    unallocated_size: u64,
    tail: drm_i915_memory_region_info_tail,
}

const DRM_IOCTL_I915_GETPARAM: Ioctl =
    _IOWR::<drm_i915_getparam>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x06);
const DRM_IOCTL_I915_GEM_CREATE: Ioctl =
    _IOWR::<drm_i915_gem_create>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x1b);
const DRM_IOCTL_I915_GEM_SET_TILING: Ioctl =
    _IOWR::<drm_i915_gem_set_tiling>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x21);
const DRM_IOCTL_I915_GEM_GET_TILING: Ioctl =
    _IOWR::<drm_i915_gem_get_tiling>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x22);
const DRM_IOCTL_I915_GEM_SET_DOMAIN: Ioctl =
    _IOW::<drm_i915_gem_set_domain>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x1f);
const DRM_IOCTL_I915_QUERY: Ioctl =
    _IOWR::<drm_i915_query>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x39);
const DRM_IOCTL_I915_GEM_CREATE_EXT: Ioctl =
    _IOWR::<drm_i915_gem_create_ext>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x3c);

const I915_PARAM_CHIPSET_ID: i32 = 4;
const I915_GEM_DOMAIN_CPU: u32 = 0x00000001;
const DRM_I915_QUERY_MEMORY_REGIONS: u64 = 4;
const I915_MEMORY_CLASS_SYSTEM: u16 = 0;
const I915_MEMORY_CLASS_DEVICE: u16 = 1;
const I915_GEM_CREATE_EXT_MEMORY_REGIONS: u32 = 0;
const I915_GEM_CREATE_EXT_SET_PAT: u32 = 2;
const I915_GEM_CREATE_EXT_FLAG_NEEDS_CPU_ACCESS: u32 = 1 << 0;
const I915_TILING_NONE: u32 = 0;
const I915_TILING_X: u32 = 1;
const I915_TILING_Y: u32 = 2;

struct Regions {
    system: drm_i915_gem_memory_class_instance,
    device: Option<drm_i915_gem_memory_class_instance>,
    vram: Vram,
}

enum Transport<K: Kernel> {
    Native,
    Vdrm(Box<Vdrm<K>>),
}

#[repr(C)]
#[derive(Copy, Clone)]
union IoctlHandle {
    gem: GemHandle,
    virtio: VirtioHandle,
}

struct Bo {
    gem_handle: GemHandle,
    ioctl_handle: IoctlHandle,
}

pub(crate) struct Device<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
    transport: Transport<K>,
    info: Option<&'static DeviceInfo>,
    regions: Option<Regions>,
    has_tiling_uapi: bool,
}

fn query_native<K: Kernel>(
    fd: &OwnedFd<K>,
    query_id: u64,
    data: &mut [u8],
) -> Result<i32, I915QueryError> {
    let data_ptr = match data.len() {
        0 => 0,
        _ => data.as_mut_ptr().expose_provenance() as u64,
    };
    let mut item = drm_i915_query_item {
        query_id,
        length: data.len() as i32,
        flags: 0,
        data_ptr,
    };
    let mut query = drm_i915_query {
        num_items: 1,
        flags: 0,
        items_ptr: ptr::from_mut(&mut item).expose_provenance() as u64,
    };
    unsafe {
        drm::ioctl(fd, DRM_IOCTL_I915_QUERY, &mut query).map_err(I915QueryError::Ioctl)?;
    }
    Ok(item.length)
}

fn memory_regions(
    mut query: impl FnMut(&mut [u8]) -> Result<i32, I915QueryError>,
) -> Result<Regions, I915QueryError> {
    let mut query = |data: &mut [u8]| {
        let length = query(data)?;
        usize::try_from(length).map_err(|_| I915QueryError::Rejected(Errno(length.wrapping_neg())))
    };
    let length = query(&mut [])?;
    let mut buffer = zeroed_buffer(length).ok_or(I915QueryError::Buffer)?;
    let length = query(&mut buffer)?.min(length);
    let infos = unsafe {
        records::<drm_i915_query_memory_regions, drm_i915_memory_region_info>(
            &buffer[..length],
            |header| header.num_regions,
        )
        .ok_or(I915QueryError::Incomplete)?
    };
    let mut regions = Regions {
        system: drm_i915_gem_memory_class_instance {
            memory_class: I915_MEMORY_CLASS_SYSTEM,
            memory_instance: 0,
        },
        device: None,
        vram: Vram {
            size: 0,
            all_mappable: true,
        },
    };
    for info in infos {
        match info.region.memory_class {
            I915_MEMORY_CLASS_SYSTEM => regions.system = info.region,
            I915_MEMORY_CLASS_DEVICE => {
                let visible = unsafe { info.tail.cpu_visible.probed_cpu_visible_size };
                regions.device = Some(info.region);
                regions.vram = Vram {
                    size: info.probed_size,
                    all_mappable: visible == 0 || visible == info.probed_size,
                };
            }
            _ => {}
        }
    }
    Ok(regions)
}

fn tiling(modifier: u64) -> u32 {
    match modifier {
        I915_FORMAT_MOD_X_TILED => I915_TILING_X,
        I915_FORMAT_MOD_Y_TILED
        | I915_FORMAT_MOD_Y_TILED_CCS
        | I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS
        | I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS
        | I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC => I915_TILING_Y,
        _ => I915_TILING_NONE,
    }
}

fn user_extension(next_extension: u64, name: u32) -> i915_user_extension {
    i915_user_extension {
        next_extension,
        name,
        flags: 0,
        rsvd: [0; 4],
    }
}

fn regions_ext(
    num_regions: u32,
    next_extension: u64,
    regions: u64,
) -> drm_i915_gem_create_ext_memory_regions {
    drm_i915_gem_create_ext_memory_regions {
        base: user_extension(next_extension, I915_GEM_CREATE_EXT_MEMORY_REGIONS),
        pad: 0,
        num_regions,
        regions,
    }
}

fn set_pat_ext(pat_index: u32) -> drm_i915_gem_create_ext_set_pat {
    drm_i915_gem_create_ext_set_pat {
        base: user_extension(0, I915_GEM_CREATE_EXT_SET_PAT),
        pat_index,
        rsvd: 0,
    }
}

fn gem_create_ext<K: Kernel>(
    fd: &OwnedFd<K>,
    size: u64,
    flags: u32,
    instances: &[drm_i915_gem_memory_class_instance],
    pat_index: Option<u32>,
) -> Result<GemHandle, I915Error> {
    let mut set_pat = pat_index.map(set_pat_ext);
    let mut regions = regions_ext(
        instances.len() as u32,
        set_pat
            .as_mut()
            .map_or(0, |ext| ptr::from_mut(ext).expose_provenance() as u64),
        instances.as_ptr().expose_provenance() as u64,
    );
    let mut create = drm_i915_gem_create_ext {
        size,
        handle: GemHandle(0),
        flags,
        extensions: ptr::from_mut(&mut regions).expose_provenance() as u64,
    };
    unsafe {
        drm::ioctl(fd, DRM_IOCTL_I915_GEM_CREATE_EXT, &mut create)
            .map_err(I915Error::GemCreateExt)?;
    }
    Ok(create.handle)
}

fn native_bo(handle: GemHandle) -> Bo {
    Bo {
        gem_handle: handle,
        ioctl_handle: IoctlHandle { gem: handle },
    }
}

impl<K: Kernel> Device<K> {
    pub(crate) fn new(fd: &Arc<OwnedFd<K>>) -> Result<Self, I915Error> {
        let mut chipset_id = 0;
        let mut getparam = drm_i915_getparam {
            param: I915_PARAM_CHIPSET_ID,
            value: &mut chipset_id,
        };
        unsafe {
            drm::ioctl(fd, DRM_IOCTL_I915_GETPARAM, &mut getparam).map_err(I915Error::GetParam)?;
        }
        let info = device_info(chipset_id as u16).map_err(|_| I915Error::Device)?;
        Self::probe(fd.clone(), Transport::Native, info)
    }

    pub(crate) fn virtio(
        fd: &Arc<OwnedFd<K>>,
        caps: &virgl_renderer_capset_drm,
    ) -> Result<Self, I915Error> {
        let pci_id = unsafe { caps.u.intel.pci_device_id as u16 };
        let info = device_info(pci_id).map_err(|_| I915Error::Device)?;
        let fd = open_render_node(fd).map_err(I915Error::RenderNode)?;
        let vdrm = Vdrm::new(&fd).map_err(I915Error::VirtioConnect)?;
        Self::probe(fd, Transport::Vdrm(Box::new(vdrm)), info)
    }

    fn probe(
        fd: Arc<OwnedFd<K>>,
        transport: Transport<K>,
        info: Option<&'static DeviceInfo>,
    ) -> Result<Self, I915Error> {
        let mut device = Device {
            fd,
            transport,
            info,
            regions: None,
            has_tiling_uapi: false,
        };
        let regions = memory_regions(|data| device.query(DRM_I915_QUERY_MEMORY_REGIONS, data));
        let regions_error = match regions {
            Ok(regions) => {
                device.regions = Some(regions);
                None
            }
            Err(error) => Some(error),
        };
        device.has_tiling_uapi = device.has_get_tiling()?;
        if let Some(error) = regions_error
            && info.is_some_and(|info| info.has_local_mem)
        {
            return Err(I915Error::MemoryRegions(error));
        }
        Ok(device)
    }

    fn query(&self, query_id: u64, data: &mut [u8]) -> Result<i32, I915QueryError> {
        match &self.transport {
            Transport::Native => query_native(&self.fd, query_id, data),
            Transport::Vdrm(vdrm) => virtio::query(vdrm, query_id, data),
        }
    }

    fn has_get_tiling(&self) -> Result<bool, I915Error> {
        let bo = self.gem_create(PAGE_SIZE)?;
        let _close = on_drop(|| drm::gem_close(&self.fd, bo.gem_handle));
        let mut get_tiling = drm_i915_gem_get_tiling {
            handle: bo.ioctl_handle,
            tiling_mode: 0,
            swizzle_mode: 0,
            phys_swizzle_mode: 0,
        };
        let ok = match &self.transport {
            Transport::Native => unsafe {
                drm::ioctl(&self.fd, DRM_IOCTL_I915_GEM_GET_TILING, &mut get_tiling).is_ok()
            },
            Transport::Vdrm(vdrm) => virtio::get_tiling(vdrm, &mut get_tiling),
        };
        Ok(ok)
    }

    fn gem_create(&self, size: u64) -> Result<Bo, I915Error> {
        if let Transport::Vdrm(vdrm) = &self.transport {
            return virtio::gem_create(vdrm, size);
        }
        let mut create = drm_i915_gem_create {
            size,
            handle: GemHandle(0),
            pad: 0,
        };
        unsafe {
            drm::ioctl(&self.fd, DRM_IOCTL_I915_GEM_CREATE, &mut create)
                .map_err(I915Error::GemCreate)?;
        }
        Ok(native_bo(create.handle))
    }

    fn create(&self, layout: &Layout, scanout: bool) -> Result<Bo, I915Error> {
        let size = bo_size(layout, self.info, true, self.fd.kernel.page_size());
        let Some(regions) = &self.regions else {
            return self.gem_create(size);
        };
        let heap = heap(self.info, &regions.vram, layout.modifier);
        let mut instances = ArrayVec::<_, 2>::new();
        if heap.vram {
            instances.extend(regions.device);
        }
        if heap.system {
            instances.push(regions.system);
        }
        let flags = match heap.vram && heap.system && !regions.vram.all_mappable {
            true => I915_GEM_CREATE_EXT_FLAG_NEEDS_CPU_ACCESS,
            false => 0,
        };
        let pat_index = self
            .info
            .filter(|info| info.ver > 12 || info.mtl_or_arl)
            .map(|info| match (heap.compressed, scanout) {
                (false, false) => info.pat.writecombining.index,
                (false, true) => info.pat.scanout.index,
                (true, false) => info.pat.compressed.index,
                (true, true) => info.pat.compressed_scanout.index,
            });
        let bo = match &self.transport {
            Transport::Native => native_bo(gem_create_ext(
                &self.fd, size, flags, &instances, pat_index,
            )?),
            Transport::Vdrm(vdrm) => {
                virtio::gem_create_ext(vdrm, size, flags, &instances, pat_index)?
            }
        };
        if !heap.vram {
            let set_domain = drm_i915_gem_set_domain {
                handle: bo.ioctl_handle,
                read_domains: I915_GEM_DOMAIN_CPU,
                write_domain: 0,
            };
            unsafe {
                self.simple_ioctl(DRM_IOCTL_I915_GEM_SET_DOMAIN, set_domain, 0);
            }
        }
        Ok(bo)
    }

    unsafe fn simple_ioctl<T>(&self, cmd: Ioctl, mut arg: T, rsp_payload_len: usize) {
        match &self.transport {
            Transport::Native => unsafe {
                let _ = drm::ioctl(&self.fd, cmd, &mut arg);
            },
            Transport::Vdrm(vdrm) => virtio::simple_ioctl(vdrm, cmd, &arg, rsp_payload_len),
        }
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        choose_layout(config, self.info, self.has_tiling_uapi)
    }

    pub(crate) fn allocate(
        &self,
        device_layout: &DeviceLayout,
        config: &Config<'_>,
    ) -> Result<c_int, I915Error> {
        let layout = &device_layout.layout;
        let bo = self.create(layout, config.scanout)?;
        let _close = on_drop(|| drm::gem_close(&self.fd, bo.gem_handle));
        if self.has_tiling_uapi {
            let set_tiling = drm_i915_gem_set_tiling {
                handle: bo.ioctl_handle,
                tiling_mode: tiling(layout.modifier),
                stride: layout.planes[0].stride as u32,
                swizzle_mode: 0,
            };
            unsafe {
                self.simple_ioctl(
                    DRM_IOCTL_I915_GEM_SET_TILING,
                    set_tiling,
                    size_of::<drm_i915_gem_set_tiling>(),
                );
            }
        }
        if let Transport::Vdrm(vdrm) = &self.transport {
            vdrm.flush().map_err(I915Error::VirtioFlush)?;
        }
        drm::handle_to_fd(&self.fd, bo.gem_handle).map_err(I915Error::Export)
    }
}
