use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::allocator::Extra;
use crate::drivers::amd::modifier::compute_layout;
use crate::drivers::amd::modifier::has_dcc;
use crate::drivers::amd::modifier::rank;
use crate::drivers::amd::uapi::AMDGPU_DRM_MINOR_GFX12_DCC_WRITE_COMPRESS_DISABLE;
use crate::drivers::amd::uapi::AMDGPU_GEM_CREATE_CPU_GTT_USWC;
use crate::drivers::amd::uapi::AMDGPU_GEM_CREATE_EXPLICIT_SYNC;
use crate::drivers::amd::uapi::AMDGPU_GEM_CREATE_GFX12_DCC;
use crate::drivers::amd::uapi::AMDGPU_GEM_CREATE_NO_CPU_ACCESS;
use crate::drivers::amd::uapi::AMDGPU_GEM_CREATE_VRAM_CLEARED;
use crate::drivers::amd::uapi::AMDGPU_GEM_DOMAIN_GTT;
use crate::drivers::amd::uapi::AMDGPU_GEM_DOMAIN_VRAM;
use crate::drivers::amd::uapi::AMDGPU_GEM_METADATA_OP_SET_METADATA;
use crate::drivers::amd::uapi::AMDGPU_GFX1013_RANGE;
use crate::drivers::amd::uapi::AMDGPU_HW_IP_COMPUTE;
use crate::drivers::amd::uapi::AMDGPU_HW_IP_GFX;
use crate::drivers::amd::uapi::AMDGPU_IDS_FLAGS_FUSION;
use crate::drivers::amd::uapi::AMDGPU_INFO_DEV_INFO;
use crate::drivers::amd::uapi::AMDGPU_INFO_HW_IP_INFO;
use crate::drivers::amd::uapi::AMDGPU_INFO_MMR_BROADCAST;
use crate::drivers::amd::uapi::AMDGPU_INFO_READ_MMR_REG;
use crate::drivers::amd::uapi::DRM_IOCTL_AMDGPU_GEM_CREATE;
use crate::drivers::amd::uapi::DRM_IOCTL_AMDGPU_GEM_METADATA;
use crate::drivers::amd::uapi::DRM_IOCTL_AMDGPU_INFO;
use crate::drivers::amd::uapi::FAMILY_AI;
use crate::drivers::amd::uapi::FAMILY_CI;
use crate::drivers::amd::uapi::FAMILY_NV;
use crate::drivers::amd::uapi::MM_GB_ADDR_CONFIG;
use crate::drivers::amd::uapi::MM_GB_MACROTILE_MODE0;
use crate::drivers::amd::uapi::MM_GB_TILE_MODE0;
use crate::drivers::amd::uapi::drm_amdgpu_gem_create;
use crate::drivers::amd::uapi::drm_amdgpu_gem_create_in;
use crate::drivers::amd::uapi::drm_amdgpu_gem_metadata;
use crate::drivers::amd::uapi::drm_amdgpu_gem_metadata__data;
use crate::drivers::amd::uapi::drm_amdgpu_info;
use crate::drivers::amd::uapi::drm_amdgpu_info__anon;
use crate::drivers::amd::uapi::drm_amdgpu_info__read_mmr_reg;
use crate::drivers::amd::uapi::drm_amdgpu_info_device;
use crate::drivers::amd::uapi::drm_amdgpu_info_hw_ip;
use crate::drivers::amd::virtio;
use crate::drivers::amd::virtio::AmdgpuVirtioCommandError;
use crate::drivers::open_render_node;
use crate::drm;
use crate::drm::DrmError;
use crate::errno::Errno;
use crate::fd::OwnedFd;
use crate::kernel::Kernel;
use crate::layout::DRM_FORMAT_MOD_INVALID;
use crate::layout::DRM_FORMAT_MOD_LINEAR;
use crate::layout::Field;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::align;
use crate::layout::best_layout;
use crate::layout::field;
use crate::layout::linear;
use crate::vdrm::Vdrm;
use crate::vdrm::VdrmError;
use crate::vdrm::virgl_renderer_capset_drm;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::ffi::c_int;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::ptr;
use run_on_drop::on_drop;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("could not read {count} registers at {first:#x}")]
pub struct AmdgpuReadRegistersError {
    pub first: u32,
    pub count: u32,
    #[source]
    pub errno: Errno,
}

#[derive(Debug, Error)]
#[error("could not query the IP of type {ip_type}")]
pub struct AmdgpuQueryHwIpError {
    pub ip_type: u32,
    #[source]
    pub errno: Errno,
}

#[derive(Debug, Error)]
pub enum AmdgpuGfxVersionError<E> {
    #[error("could not query the gfx IP version")]
    Query(#[source] E),
    #[error("unknown gfx IP version {major}.{minor}")]
    Unknown { major: u32, minor: u32 },
}

#[derive(Debug, Error)]
pub enum AmdgpuOpenError {
    #[error("could not open the render node")]
    RenderNode(#[source] Errno),
    #[error("could not query the device")]
    DevInfo(#[source] Errno),
    #[error("could not determine the gfx IP version")]
    GfxVersion(#[source] AmdgpuGfxVersionError<AmdgpuQueryHwIpError>),
    #[error("could not read the registers")]
    Registers(#[source] AmdgpuReadRegistersError),
    #[error("could not query the driver version")]
    VersionMinor(#[source] DrmError),
}

#[derive(Debug, Error)]
pub enum AmdgpuVirtioOpenError {
    #[error("could not open the render node for virtio")]
    RenderNode(#[source] Errno),
    #[error("could not connect to the virtio host")]
    Connect(#[source] VdrmError),
    #[error("could not determine the gfx IP version")]
    GfxVersion(#[source] AmdgpuGfxVersionError<AmdgpuVirtioCommandError>),
}

#[derive(Debug, Error)]
pub enum AmdgpuAllocateError {
    #[error("could not create the buffer")]
    GemCreate(#[source] Errno),
    #[error("could not set the metadata")]
    GemMetadata(#[source] Errno),
    #[error("could not export the buffer")]
    Export(#[source] DrmError),
    #[error("could not create the virtio buffer")]
    VirtioBlob(#[source] VdrmError),
    #[error("could not set the virtio buffer metadata")]
    VirtioSetMetadata(#[source] AmdgpuVirtioCommandError),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct GfxVersion {
    major: u32,
    minor: u32,
}

impl Debug for GfxVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

const fn version(major: u32, minor: u32) -> GfxVersion {
    GfxVersion { major, minor }
}

pub(super) const GFX6: GfxVersion = version(6, 0);
pub(super) const GFX7: GfxVersion = version(7, 0);
pub(super) const GFX8: GfxVersion = version(8, 0);
pub(super) const GFX9: GfxVersion = version(9, 0);
pub(super) const GFX10: GfxVersion = version(10, 1);
pub(super) const GFX10_3: GfxVersion = version(10, 3);
pub(super) const GFX11: GfxVersion = version(11, 0);
pub(super) const GFX11_5: GfxVersion = version(11, 5);
pub(super) const GFX11_7: GfxVersion = version(11, 7);
pub(super) const GFX12: GfxVersion = version(12, 0);

const IP_DISCOVERY_MAJOR: Field = Field { shift: 16, bits: 8 };
const IP_DISCOVERY_MINOR: Field = Field { shift: 8, bits: 8 };

const MAX_EXTENT: u32 = 16384;

fn ip_version(ip: &drm_amdgpu_info_hw_ip) -> GfxVersion {
    if ip.ip_discovery_version != 0 {
        let discovery = ip.ip_discovery_version as u64;
        return version(
            field(discovery, IP_DISCOVERY_MAJOR) as u32,
            field(discovery, IP_DISCOVERY_MINOR) as u32,
        );
    }
    version(ip.hw_ip_version_major, ip.hw_ip_version_minor)
}

fn gfx_ip<E>(
    family: u32,
    external_rev: u32,
    mut query: impl FnMut(u32) -> Result<drm_amdgpu_info_hw_ip, E>,
) -> Result<GfxIp, AmdgpuGfxVersionError<E>> {
    let gfx = query(AMDGPU_HW_IP_GFX);
    let has_graphics = gfx.as_ref().is_ok_and(|ip| ip.available_rings != 0);
    let gfx1013 = family == FAMILY_NV && AMDGPU_GFX1013_RANGE.contains(&external_rev);
    let mut compute_is_gfx9 = || {
        query(AMDGPU_HW_IP_COMPUTE).is_ok_and(|ip| {
            !(gfx1013 && ip.available_rings != 0) && ip_version(&ip).major == GFX9.major
        })
    };
    let version = match gfx.map(|ip| ip_version(&ip)) {
        Ok(ver) if ver >= GFX12 => GFX12,
        Ok(ver) if [GFX11_7, GFX11_5, GFX11, GFX10_3, GFX10].contains(&ver) => ver,
        Ok(ver) if ver.major == GFX9.major => GFX9,
        _ if compute_is_gfx9() => GFX9,
        Ok(ver) if (GFX6.major..=GFX8.major).contains(&ver.major) => version(ver.major, 0),
        Ok(ver) => {
            return Err(AmdgpuGfxVersionError::Unknown {
                major: ver.major,
                minor: ver.minor,
            });
        }
        Err(e) => return Err(AmdgpuGfxVersionError::Query(e)),
    };
    Ok(GfxIp {
        version,
        has_graphics,
    })
}

struct GfxIp {
    version: GfxVersion,
    has_graphics: bool,
}

pub(crate) struct Bo {
    pub(crate) size: u64,
    pub(crate) alignment: u64,
    pub(crate) linear: bool,
    pub(crate) metadata: drm_amdgpu_gem_metadata__data,
}

pub(crate) struct Device<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
    vdrm: Option<Box<Vdrm<K>>>,
    pub(super) gfx_version: GfxVersion,
    pub(super) has_graphics: bool,
    ids_flags: u64,
    pub(super) pci_id: u32,
    pte_fragment_size: u64,
    page_size: u64,
    version_minor: c_int,
    pub(super) addr_config: u32,
    pub(super) tile_mode: [u32; 32],
    pub(super) macrotile_mode: [u32; 16],
}

fn info<K: Kernel, T: Default>(
    fd: &OwnedFd<K>,
    query: u32,
    anon: drm_amdgpu_info__anon,
) -> Result<T, Errno> {
    let mut out = T::default();
    let mut info = drm_amdgpu_info {
        return_size: size_of::<T>() as u32,
        return_pointer: ptr::from_mut(&mut out).expose_provenance() as u64,
        query,
        anon,
    };
    unsafe {
        drm::ioctl(fd, DRM_IOCTL_AMDGPU_INFO, &mut info)?;
    }
    Ok(out)
}

fn read_registers<K: Kernel, const N: usize>(
    fd: &OwnedFd<K>,
    first: u32,
) -> Result<[u32; N], AmdgpuOpenError>
where
    [u32; N]: Default,
{
    let count = N as u32;
    let anon = drm_amdgpu_info__anon {
        read_mmr_reg: drm_amdgpu_info__read_mmr_reg {
            dword_offset: first,
            count,
            instance: AMDGPU_INFO_MMR_BROADCAST,
            flags: 0,
        },
    };
    info(fd, AMDGPU_INFO_READ_MMR_REG, anon).map_err(|errno| {
        AmdgpuOpenError::Registers(AmdgpuReadRegistersError {
            first,
            count,
            errno,
        })
    })
}

impl<K: Kernel> Device<K> {
    pub(crate) fn new(fd: &Arc<OwnedFd<K>>) -> Result<Self, AmdgpuOpenError> {
        let fd = open_render_node(fd).map_err(AmdgpuOpenError::RenderNode)?;
        let device: drm_amdgpu_info_device =
            info(&fd, AMDGPU_INFO_DEV_INFO, drm_amdgpu_info__anon::ZERO)
                .map_err(AmdgpuOpenError::DevInfo)?;
        let gfx = gfx_ip(device.family, device.external_rev, |ip_type| {
            info(
                &fd,
                AMDGPU_INFO_HW_IP_INFO,
                drm_amdgpu_info__anon::query_hw_ip(ip_type),
            )
            .map_err(|errno| AmdgpuQueryHwIpError { ip_type, errno })
        })
        .map_err(AmdgpuOpenError::GfxVersion)?;
        let [addr_config] = read_registers(&fd, MM_GB_ADDR_CONFIG)?;
        let mut tile_mode = [0; 32];
        let mut macrotile_mode = [0; 16];
        if device.family < FAMILY_AI {
            tile_mode = read_registers(&fd, MM_GB_TILE_MODE0)?;
            if device.family >= FAMILY_CI {
                macrotile_mode = read_registers(&fd, MM_GB_MACROTILE_MODE0)?;
            }
        }
        let version_minor = drm::version_minor(&fd).map_err(AmdgpuOpenError::VersionMinor)?;
        Ok(Device {
            fd,
            vdrm: None,
            gfx_version: gfx.version,
            has_graphics: gfx.has_graphics,
            ids_flags: device.ids_flags,
            pci_id: device.device_id,
            pte_fragment_size: device.pte_fragment_size as u64,
            page_size: (device.gart_page_size as u64).max(1),
            version_minor,
            addr_config,
            tile_mode,
            macrotile_mode,
        })
    }

    pub(crate) fn virtio(
        fd: &Arc<OwnedFd<K>>,
        caps: &virgl_renderer_capset_drm,
    ) -> Result<Self, AmdgpuVirtioOpenError> {
        let amdgpu = unsafe { caps.u.amdgpu };
        let info = &amdgpu.gpu_info;
        let fd = open_render_node(fd).map_err(AmdgpuVirtioOpenError::RenderNode)?;
        let vdrm = Vdrm::new(&fd).map_err(AmdgpuVirtioOpenError::Connect)?;
        let gfx = gfx_ip(info.family_id, info.chip_external_rev, |ip_type| {
            virtio::query_hw_ip(&vdrm, ip_type)
        })
        .map_err(AmdgpuVirtioOpenError::GfxVersion)?;
        Ok(Device {
            fd,
            vdrm: Some(Box::new(vdrm)),
            gfx_version: gfx.version,
            has_graphics: gfx.has_graphics,
            ids_flags: info.ids_flags,
            pci_id: info.asic_id,
            pte_fragment_size: amdgpu.alignments.size_local,
            page_size: amdgpu.alignments.size_remote.max(1),
            version_minor: caps.version_minor as c_int,
            addr_config: info.gb_addr_cfg,
            tile_mode: info.gb_tile_mode,
            macrotile_mode: info.gb_macro_tile_mode,
        })
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        let fmt = config.format.0;
        let extents = 1..=MAX_EXTENT;
        if !fmt.amd {
            return Err(LayoutError::Format);
        }
        if !extents.contains(&config.width) || !extents.contains(&config.height) {
            return Err(LayoutError::Size);
        }
        let explicit = best_layout(
            config,
            |layout| self.explicit_layout(layout, linear::ALIGNMENT, config),
            |modifier| {
                let planes = compute_layout(fmt, modifier, config.width, config.height)?;
                let layout = Layout {
                    format: fmt,
                    modifier,
                    planes: planes.planes,
                };
                Some(self.explicit_layout(layout, planes.alignment, config))
            },
            |modifier| rank(self, modifier, config),
        );
        let linear = match &explicit {
            Ok(explicit) => explicit.layout.modifier == DRM_FORMAT_MOD_LINEAR,
            Err(_) => true,
        };
        if self.gfx_version < GFX9 && config.modifiers.contains(&DRM_FORMAT_MOD_INVALID) && linear {
            return self.implicit_layout(config).or(explicit);
        }
        explicit
    }

    pub(crate) fn allocate(
        &self,
        device_layout: &DeviceLayout,
        config: &Config<'_>,
    ) -> Result<c_int, AmdgpuAllocateError> {
        let Extra::Amd(bo) = &device_layout.extra else {
            unreachable!();
        };
        let mut domains = AMDGPU_GEM_DOMAIN_VRAM;
        if self.ids_flags & AMDGPU_IDS_FLAGS_FUSION != 0 {
            domains |= AMDGPU_GEM_DOMAIN_GTT;
        }
        let mut flags = AMDGPU_GEM_CREATE_VRAM_CLEARED | AMDGPU_GEM_CREATE_CPU_GTT_USWC;
        if !bo.linear {
            flags |= AMDGPU_GEM_CREATE_NO_CPU_ACCESS;
        }
        if self.gfx_version >= GFX12
            && self.version_minor >= AMDGPU_DRM_MINOR_GFX12_DCC_WRITE_COMPRESS_DISABLE
            && has_dcc(device_layout.layout.modifier)
        {
            flags |= AMDGPU_GEM_CREATE_GFX12_DCC;
        }
        if !config.implicit_sync {
            flags |= AMDGPU_GEM_CREATE_EXPLICIT_SYNC;
        }
        let size = align(bo.size, self.page_size);
        let fragment = match size >= self.pte_fragment_size {
            true => self.pte_fragment_size,
            false => size.isolate_highest_one(),
        };
        let input = drm_amdgpu_gem_create_in {
            bo_size: size,
            alignment: align(bo.alignment, self.page_size).max(fragment),
            domains,
            domain_flags: flags,
        };
        if let Some(vdrm) = &self.vdrm {
            return virtio::allocate(vdrm, &input, &bo.metadata);
        }
        let fd = &self.fd;
        let mut create = drm_amdgpu_gem_create { input };
        unsafe {
            drm::ioctl(fd, DRM_IOCTL_AMDGPU_GEM_CREATE, &mut create)
                .map_err(AmdgpuAllocateError::GemCreate)?;
        }
        let handle = unsafe { create.output.handle };
        let _close = on_drop(|| drm::gem_close(fd, handle));
        let mut request = drm_amdgpu_gem_metadata {
            handle,
            op: AMDGPU_GEM_METADATA_OP_SET_METADATA,
            data: bo.metadata,
        };
        unsafe {
            drm::ioctl(fd, DRM_IOCTL_AMDGPU_GEM_METADATA, &mut request)
                .map_err(AmdgpuAllocateError::GemMetadata)?;
        }
        drm::handle_to_fd(fd, handle).map_err(AmdgpuAllocateError::Export)
    }
}
