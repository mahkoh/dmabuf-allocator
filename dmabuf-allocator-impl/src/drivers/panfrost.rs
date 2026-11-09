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
use crate::layout::DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED;
use crate::layout::DRM_FORMAT_MOD_LINEAR;
use crate::layout::DRM_FORMAT_MOD_VENDOR_ARM;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::PAGE_SIZE;
use crate::layout::PlaneLayout;
use crate::layout::PlaneVec;
use crate::layout::align;
use crate::layout::fourcc_mod_code;
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use crate::vdrm::BlobCreate;
use crate::vdrm::VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE;
use crate::vdrm::VIRTGPU_BLOB_FLAG_USE_MAPPABLE;
use crate::vdrm::VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
use crate::vdrm::Vdrm;
use crate::vdrm::VdrmError;
use crate::vdrm::vdrm_ccmd_req;
use crate::vdrm::vdrm_ccmd_rsp;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::ffi::c_int;
use core::ptr;
use run_on_drop::on_drop;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PanfrostError {
    #[error("could not query the GPU")]
    Query(#[source] Errno),
    #[error("the buffer is too large")]
    Size,
    #[error("could not create the buffer")]
    Create(#[source] Errno),
    #[error("could not export the buffer")]
    Export(#[source] DrmError),
    #[error("could not open the render node for virtio")]
    VirtioRenderNode(#[source] Errno),
    #[error("could not connect to the virtio host")]
    VirtioConnect(#[source] VdrmError),
    #[error("could not send the GPU query to the virtio host")]
    VirtioGetParamSend(#[source] VdrmError),
    #[error("the virtio host could not query the GPU")]
    VirtioGetParam(#[source] Errno),
    #[error("could not create the virtio buffer")]
    VirtioBlob(#[source] VdrmError),
}

#[derive(Copy, Clone)]
pub(crate) enum AfbcMode {
    R8,
    R8G8,
    R5G6B5,
    R5G5B5A1,
    R8G8B8,
    R8G8B8A8,
    R10G10B10A2,
    R16,
    R16G16,
    R16G16B16A16,
}

#[derive(Copy, Clone)]
pub(crate) struct PanfrostAfbc {
    pub(crate) mode: AfbcMode,
    pub(crate) min_arch: u32,
    pub(crate) no_v7: bool,
}

#[derive(Copy, Clone)]
pub(crate) struct PanfrostFormat {
    pub(crate) afbc: Option<PanfrostAfbc>,
}

const AFBC: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_ARM, 0);
const U_INTERLEAVED: u64 = DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED;
const BLOCK_16X16: u64 = 1;
const BLOCK_32X8: u64 = 2;
const YTR: u64 = 1 << 4;
const SPLIT: u64 = 1 << 5;
const SPARSE: u64 = 1 << 6;
const TILED: u64 = 1 << 8;
const SC: u64 = 1 << 9;

const MODIFIERS: [u64; 10] = [
    AFBC | BLOCK_32X8 | SPARSE,
    AFBC | BLOCK_32X8 | SPARSE | SPLIT,
    AFBC | BLOCK_32X8 | SPARSE | SPLIT | YTR,
    AFBC | BLOCK_16X16 | TILED | SC | SPARSE | YTR,
    AFBC | BLOCK_16X16 | TILED | SC | SPARSE,
    AFBC | BLOCK_16X16 | SPARSE | YTR,
    AFBC | BLOCK_16X16 | SPARSE,
    AFBC | BLOCK_16X16 | SPARSE | SPLIT,
    U_INTERLEAVED,
    DRM_FORMAT_MOD_LINEAR,
];

const MAX_EXTENT: u32 = 16384;
const CACHELINE: u64 = 64;
const U_INTERLEAVED_TILE: u64 = 16;
const RENDER_TILE: u64 = 16;
const AFBC_HEADER_BYTES_PER_TILE: u64 = 16;
const AFBC_HEADER_ALIGNMENT: u64 = 64;
const AFBC_HEADER_ALIGNMENT_BIFROST: u64 = 128;
const AFBC_HEADER_ALIGNMENT_TILED: u64 = 4096;
const AFBC_TILE_SIZE: u64 = 8;
const AFBC_TILE_SIZE_WIDE: u64 = 4;
const AFBC_TILE_SIZE_WIDE_MIN_BYTES: u64 = 5;
const PAN_CRC_HEADER_SIZE_B: u64 = 64;
const CHECKSUM_TILE_WIDTH: u64 = 16;
const CHECKSUM_TILE_HEIGHT: u64 = 16;
const CHECKSUM_BYTES_PER_TILE: u64 = 8;
const CHECKSUM_REGION_SIZE: u64 = 32;
const CHECKSUM_REGION_SIZE_V12: u64 = 64;
const CHECKSUM_MAX_BYTES: u64 = 4;
const CHECKSUM_MAX_BYTES_V6: u64 = 6;

const T600: u64 = 0x600;
const T620: u64 = 0x620;
const T720: u64 = 0x720;
const T760: u64 = 0x750;
const T820: u64 = 0x820;
const T830: u64 = 0x830;
const T860: u64 = 0x860;
const T880: u64 = 0x880;

const PANFROST_BO_NOEXEC: u32 = 1;
const DRM_PANFROST_PARAM_GPU_PROD_ID: u32 = 0;
const DRM_PANFROST_PARAM_AFBC_FEATURES: u32 = 40;

#[repr(C)]
struct drm_panfrost_create_bo {
    size: u32,
    flags: u32,
    handle: GemHandle,
    pad: u32,
    offset: u64,
}

#[repr(C)]
struct drm_panfrost_get_param {
    param: u32,
    pad: u32,
    value: u64,
}

const DRM_IOCTL_PANFROST_CREATE_BO: Ioctl =
    _IOWR::<drm_panfrost_create_bo>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x02);
const DRM_IOCTL_PANFROST_GET_PARAM: Ioctl =
    _IOWR::<drm_panfrost_get_param>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x04);

const DRM_PANTHOR_DEV_QUERY_GPU_INFO: u32 = 0;

#[repr(C)]
struct drm_panthor_dev_query {
    type_: u32,
    size: u32,
    pointer: u64,
}

#[repr(C)]
#[derive(Default)]
struct drm_panthor_gpu_info {
    gpu_id: u32,
    gpu_rev: u32,
    csf_id: u32,
    l2_features: u32,
    tiler_features: u32,
    mem_features: u32,
    mmu_features: u32,
    thread_features: u32,
    max_threads: u32,
    thread_max_workgroup_size: u32,
    thread_max_barrier_size: u32,
    coherency_features: u32,
    texture_features: [u32; 4],
    as_present: u32,
    selected_coherency: u32,
    shader_present: u64,
    l2_present: u64,
    tiler_present: u64,
    core_features: u32,
    pad: u32,
    gpu_features: u64,
}

#[repr(C)]
struct drm_panthor_bo_create {
    size: u64,
    flags: u32,
    exclusive_vm_id: u32,
    handle: GemHandle,
    pad: u32,
}

const DRM_IOCTL_PANTHOR_DEV_QUERY: Ioctl =
    _IOWR::<drm_panthor_dev_query>(DRM_IOCTL_BASE, DRM_COMMAND_BASE);
const DRM_IOCTL_PANTHOR_BO_CREATE: Ioctl =
    _IOWR::<drm_panthor_bo_create>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x05);

const PANFROST_CCMD_CREATE_BO: u32 = 3;
const PANFROST_CCMD_GET_PARAM: u32 = 5;

#[repr(C)]
struct panfrost_ccmd_create_bo_req {
    hdr: vdrm_ccmd_req,
    pad: u32,
    size: u32,
    blob_id: u32,
    flags: u32,
}

#[repr(C)]
struct panfrost_ccmd_get_param_req {
    hdr: vdrm_ccmd_req,
    pad: u32,
    param: u32,
}

#[repr(C)]
struct panfrost_ccmd_get_param_rsp {
    hdr: vdrm_ccmd_rsp,
    ret: i32,
    value: u64,
}

enum Kmod<K: Kernel> {
    Panfrost,
    PanfrostVirtioGpu(Box<Vdrm<K>>),
    Panthor,
}

pub(crate) struct Device<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
    kmod: Kmod<K>,
    arch: u32,
    afbc: bool,
}

fn get_param<K: Kernel>(fd: &OwnedFd<K>, param: u32) -> Result<u64, PanfrostError> {
    let mut get = drm_panfrost_get_param {
        param,
        pad: 0,
        value: 0,
    };
    unsafe {
        drm::ioctl(fd, DRM_IOCTL_PANFROST_GET_PARAM, &mut get).map_err(PanfrostError::Query)?;
    }
    Ok(get.value)
}

fn virtio_get_param<K: Kernel>(vdrm: &Vdrm<K>, param: u32) -> Result<u64, PanfrostError> {
    let req = panfrost_ccmd_get_param_req {
        hdr: vdrm_ccmd_req::new(PANFROST_CCMD_GET_PARAM),
        pad: 0,
        param,
    };
    let unanswered = panfrost_ccmd_get_param_rsp {
        hdr: vdrm_ccmd_rsp { len: 0 },
        ret: -1,
        value: 0,
    };
    let rsp = unsafe {
        vdrm.send_sync(&req, &[], Some(unanswered))
            .map_err(PanfrostError::VirtioGetParamSend)?
    };
    match rsp.ret {
        0 => Ok(rsp.value),
        ret => Err(PanfrostError::VirtioGetParam(Errno(ret.wrapping_neg()))),
    }
}

impl<K: Kernel> Device<K> {
    fn new(prod_id: u64, afbc_features: u64, fd: Arc<OwnedFd<K>>, kmod: Kmod<K>) -> Self {
        let arch = match prod_id & 0xffff {
            T600 | T620 | T720 => 4,
            T760 | T820 | T830 | T860 | T880 => 5,
            id => (id >> 12) as u32,
        };
        Self {
            fd,
            kmod,
            arch,
            afbc: arch >= 5 && afbc_features == 0,
        }
    }

    pub(crate) fn panfrost(fd: &Arc<OwnedFd<K>>) -> Result<Self, PanfrostError> {
        Ok(Self::new(
            get_param(fd, DRM_PANFROST_PARAM_GPU_PROD_ID)?,
            get_param(fd, DRM_PANFROST_PARAM_AFBC_FEATURES)?,
            fd.clone(),
            Kmod::Panfrost,
        ))
    }

    pub(crate) fn virtio(fd: &Arc<OwnedFd<K>>) -> Result<Self, PanfrostError> {
        let fd = open_render_node(fd).map_err(PanfrostError::VirtioRenderNode)?;
        let vdrm = Vdrm::new(&fd).map_err(PanfrostError::VirtioConnect)?;
        Ok(Self::new(
            virtio_get_param(&vdrm, DRM_PANFROST_PARAM_GPU_PROD_ID)?,
            virtio_get_param(&vdrm, DRM_PANFROST_PARAM_AFBC_FEATURES)?,
            fd,
            Kmod::PanfrostVirtioGpu(Box::new(vdrm)),
        ))
    }

    pub(crate) fn panthor(fd: &Arc<OwnedFd<K>>) -> Result<Self, PanfrostError> {
        let mut info = drm_panthor_gpu_info::default();
        let mut query = drm_panthor_dev_query {
            type_: DRM_PANTHOR_DEV_QUERY_GPU_INFO,
            size: size_of::<drm_panthor_gpu_info>() as u32,
            pointer: ptr::from_mut(&mut info).expose_provenance() as u64,
        };
        unsafe {
            drm::ioctl(fd, DRM_IOCTL_PANTHOR_DEV_QUERY, &mut query)
                .map_err(PanfrostError::Query)?;
        }
        Ok(Self::new(
            (info.gpu_id >> 16).into(),
            0,
            fd.clone(),
            Kmod::Panthor,
        ))
    }

    fn supports(&self, afbc: PanfrostAfbc, modifier: u64) -> bool {
        use AfbcMode::*;
        let split = modifier & SPLIT != 0;
        self.afbc
            && self.arch >= afbc.min_arch
            && !(self.arch == 7 && afbc.no_v7)
            && (modifier & TILED == 0 || self.arch >= 7)
            && (!split || self.arch >= 6)
            && (modifier & YTR == 0 || !matches!(afbc.mode, R8 | R8G8 | R16 | R16G16))
            && (modifier & BLOCK_32X8 == 0
                || split
                    == matches!(
                        afbc.mode,
                        R8G8B8 | R8G8B8A8 | R10G10B10A2 | R16G16 | R16G16B16A16
                    ))
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        let extents = 1..=MAX_EXTENT;
        if !extents.contains(&config.width) || !extents.contains(&config.height) {
            return Err(LayoutError::Size);
        }
        let format = config.format.0;
        let afbc = format.panfrost.ok_or(LayoutError::Format)?.afbc;
        let [plane] = format.planes else {
            return Err(LayoutError::Format);
        };
        if plane.block_width != 1 || plane.block_height != 1 || !plane.bits.is_multiple_of(8) {
            return Err(LayoutError::Format);
        }
        let modifier = MODIFIERS.into_iter().find(|&modifier| {
            config.modifiers.contains(&modifier)
                && (modifier == DRM_FORMAT_MOD_LINEAR
                    || !config.implicit_interop
                        && (modifier == U_INTERLEAVED
                            || afbc.is_some_and(|afbc| self.supports(afbc, modifier))))
        });
        let modifier = modifier.ok_or(LayoutError::Modifier)?;
        let bytes = plane.bits as u64 / 8;
        let (width, height) = (config.width as u64, config.height as u64);
        let (stride, rows, header) = match modifier {
            DRM_FORMAT_MOD_LINEAR => (align(width * bytes, CACHELINE), height, 0),
            U_INTERLEAVED => (
                align(width, U_INTERLEAVED_TILE) * bytes,
                align(height, U_INTERLEAVED_TILE),
                0,
            ),
            _ => {
                let (block_width, block_height) = if modifier & BLOCK_32X8 != 0 {
                    (32, 8)
                } else {
                    (16, 16)
                };
                let render_height = align(block_height, RENDER_TILE);
                let (align_width, align_height, header_align) = if modifier & TILED != 0 {
                    let tile_size = match bytes >= AFBC_TILE_SIZE_WIDE_MIN_BYTES {
                        true => AFBC_TILE_SIZE_WIDE,
                        false => AFBC_TILE_SIZE,
                    };
                    (
                        block_width * tile_size,
                        render_height * tile_size,
                        AFBC_HEADER_ALIGNMENT_TILED,
                    )
                } else if self.arch >= 6 {
                    (block_width, render_height, AFBC_HEADER_ALIGNMENT_BIFROST)
                } else {
                    (block_width, render_height, AFBC_HEADER_ALIGNMENT)
                };
                let width = align(width, align_width);
                let height = align(height, align_height);
                let header =
                    AFBC_HEADER_BYTES_PER_TILE * (width / block_width) * (height / block_height);
                (width * bytes, height, align(header, header_align))
            }
        };
        let checksum_max_bytes = match self.arch {
            6 => CHECKSUM_MAX_BYTES_V6,
            _ => CHECKSUM_MAX_BYTES,
        };
        let crc = if bytes <= checksum_max_bytes {
            let region = match self.arch >= 12 {
                true => CHECKSUM_REGION_SIZE_V12,
                false => CHECKSUM_REGION_SIZE,
            };
            let tiles_x = align(width, region) / CHECKSUM_TILE_WIDTH;
            let tiles_y = align(height, region) / CHECKSUM_TILE_HEIGHT;
            PAN_CRC_HEADER_SIZE_B + align(tiles_x * CHECKSUM_BYTES_PER_TILE, CACHELINE) * tiles_y
        } else {
            0
        };
        let size = align(header + stride * rows + crc, PAGE_SIZE);
        if !matches!(self.kmod, Kmod::Panthor) && u32::try_from(size).is_err() {
            return Err(LayoutError::TooLarge);
        }
        Ok(DeviceLayout {
            layout: Layout {
                format,
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

    pub(crate) fn allocate(&self, device_layout: &DeviceLayout) -> Result<c_int, PanfrostError> {
        let size = device_layout.layout.planes[0].size;
        let handle = match &self.kmod {
            Kmod::Panfrost => {
                let mut create = drm_panfrost_create_bo {
                    size: size as u32,
                    flags: PANFROST_BO_NOEXEC,
                    handle: GemHandle(0),
                    pad: 0,
                    offset: 0,
                };
                unsafe {
                    drm::ioctl(&self.fd, DRM_IOCTL_PANFROST_CREATE_BO, &mut create)
                        .map_err(PanfrostError::Create)?;
                }
                create.handle
            }
            Kmod::PanfrostVirtioGpu(vdrm) => {
                let size = u32::try_from(vdrm.blob_size(size)).map_err(|_| PanfrostError::Size)?;
                let blob_id = vdrm.next_blob_id();
                let req = panfrost_ccmd_create_bo_req {
                    hdr: vdrm_ccmd_req::new(PANFROST_CCMD_CREATE_BO),
                    pad: 0,
                    size,
                    blob_id: blob_id as u32,
                    flags: PANFROST_BO_NOEXEC,
                };
                let params = BlobCreate {
                    size: size.into(),
                    blob_flags: VIRTGPU_BLOB_FLAG_USE_MAPPABLE
                        | VIRTGPU_BLOB_FLAG_USE_SHAREABLE
                        | if vdrm.cross_device() {
                            VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE
                        } else {
                            0
                        },
                    blob_id,
                    blob_hints: 0,
                };
                vdrm.bo_create(&params, &req, &[])
                    .map_err(PanfrostError::VirtioBlob)?
                    .bo_handle
            }
            Kmod::Panthor => {
                let mut create = drm_panthor_bo_create {
                    size,
                    flags: 0,
                    exclusive_vm_id: 0,
                    handle: GemHandle(0),
                    pad: 0,
                };
                unsafe {
                    drm::ioctl(&self.fd, DRM_IOCTL_PANTHOR_BO_CREATE, &mut create)
                        .map_err(PanfrostError::Create)?;
                }
                create.handle
            }
        };
        let _close = on_drop(|| drm::gem_close(&self.fd, handle));
        drm::handle_to_fd(&self.fd, handle).map_err(PanfrostError::Export)
    }
}
