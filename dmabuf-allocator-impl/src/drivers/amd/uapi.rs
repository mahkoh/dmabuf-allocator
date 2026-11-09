use crate::drm::DRM_COMMAND_BASE;
use crate::drm::DRM_IOCTL_BASE;
use crate::drm::GemHandle;
use crate::sys::_IOW;
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use core::ffi::c_int;
use core::ops::Range;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct drm_amdgpu_gem_create_in {
    pub(crate) bo_size: u64,
    pub(crate) alignment: u64,
    pub(crate) domains: u64,
    pub(crate) domain_flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct drm_amdgpu_gem_create_out {
    pub(crate) handle: GemHandle,
    _pad: u32,
}

#[repr(C)]
pub(crate) union drm_amdgpu_gem_create {
    pub(crate) input: drm_amdgpu_gem_create_in,
    pub(crate) output: drm_amdgpu_gem_create_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct drm_amdgpu_gem_metadata__data {
    pub(crate) flags: u64,
    pub(crate) tiling_info: u64,
    pub(crate) data_size_bytes: u32,
    pub(crate) data: [u32; 64],
}

#[repr(C)]
pub(crate) struct drm_amdgpu_gem_metadata {
    pub(crate) handle: GemHandle,
    pub(crate) op: u32,
    pub(crate) data: drm_amdgpu_gem_metadata__data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct drm_amdgpu_query_fw {
    pub(crate) fw_type: u32,
    pub(crate) ip_instance: u32,
    pub(crate) index: u32,
    pub(crate) _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct drm_amdgpu_info__mode_crtc {
    pub(crate) id: u32,
    pub(crate) _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct drm_amdgpu_info__query_hw_ip {
    pub(crate) type_: u32,
    pub(crate) ip_instance: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct drm_amdgpu_info__read_mmr_reg {
    pub(crate) dword_offset: u32,
    pub(crate) count: u32,
    pub(crate) instance: u32,
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct drm_amdgpu_info__vbios_info {
    pub(crate) type_: u32,
    pub(crate) offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct drm_amdgpu_info__sensor_info {
    pub(crate) type_: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct drm_amdgpu_info__video_cap {
    pub(crate) type_: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union drm_amdgpu_info__anon {
    pub(crate) mode_crtc: drm_amdgpu_info__mode_crtc,
    pub(crate) query_hw_ip: drm_amdgpu_info__query_hw_ip,
    pub(crate) read_mmr_reg: drm_amdgpu_info__read_mmr_reg,
    pub(crate) query_fw: drm_amdgpu_query_fw,
    pub(crate) vbios_info: drm_amdgpu_info__vbios_info,
    pub(crate) sensor_info: drm_amdgpu_info__sensor_info,
    pub(crate) video_cap: drm_amdgpu_info__video_cap,
}

const _: () =
    assert!(size_of::<drm_amdgpu_info__read_mmr_reg>() == size_of::<drm_amdgpu_info__anon>());

impl drm_amdgpu_info__anon {
    pub(crate) const ZERO: Self = Self {
        read_mmr_reg: drm_amdgpu_info__read_mmr_reg {
            dword_offset: 0,
            count: 0,
            instance: 0,
            flags: 0,
        },
    };

    pub(crate) fn query_hw_ip(type_: u32) -> Self {
        let mut anon = Self::ZERO;
        anon.query_hw_ip = drm_amdgpu_info__query_hw_ip {
            type_,
            ip_instance: 0,
        };
        anon
    }
}

#[repr(C)]
pub(crate) struct drm_amdgpu_info {
    pub(crate) return_pointer: u64,
    pub(crate) return_size: u32,
    pub(crate) query: u32,
    pub(crate) anon: drm_amdgpu_info__anon,
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct drm_amdgpu_info_device {
    pub(crate) device_id: u32,
    chip_rev: u32,
    pub(crate) external_rev: u32,
    pci_rev: u32,
    pub(crate) family: u32,
    num_shader_engines: u32,
    num_shader_arrays_per_engine: u32,
    gpu_counter_freq: u32,
    max_engine_clock: u64,
    max_memory_clock: u64,
    cu_active_number: u32,
    cu_ao_mask: u32,
    cu_bitmap: [[u32; 4]; 4],
    enabled_rb_pipes_mask: u32,
    num_rb_pipes: u32,
    num_hw_gfx_contexts: u32,
    pcie_gen: u32,
    pub(crate) ids_flags: u64,
    virtual_address_offset: u64,
    virtual_address_max: u64,
    virtual_address_alignment: u32,
    pub(crate) pte_fragment_size: u32,
    pub(crate) gart_page_size: u32,
    ce_ram_size: u32,
    vram_type: u32,
    vram_bit_width: u32,
    vce_harvest_config: u32,
    gc_double_offchip_lds_buf: u32,
    prim_buf_gpu_addr: u64,
    pos_buf_gpu_addr: u64,
    cntl_sb_buf_gpu_addr: u64,
    param_buf_gpu_addr: u64,
    prim_buf_size: u32,
    pos_buf_size: u32,
    cntl_sb_buf_size: u32,
    param_buf_size: u32,
    wave_front_size: u32,
    num_shader_visible_vgprs: u32,
    num_cu_per_sh: u32,
    num_tcc_blocks: u32,
    gs_vgt_table_depth: u32,
    gs_prim_buffer_depth: u32,
    max_gs_waves_per_vgt: u32,
    pcie_num_lanes: u32,
    cu_ao_bitmap: [[u32; 4]; 4],
    high_va_offset: u64,
    high_va_max: u64,
    pa_sc_tile_steering_override: u32,
    tcc_disabled_mask: u64,
    min_engine_clock: u64,
    min_memory_clock: u64,
    tcp_cache_size: u32,
    num_sqc_per_wgp: u32,
    sqc_data_cache_size: u32,
    sqc_inst_cache_size: u32,
    gl1c_cache_size: u32,
    gl2c_cache_size: u32,
    mall_size: u64,
    enabled_rb_pipes_mask_hi: u32,
    shadow_size: u32,
    shadow_alignment: u32,
    csa_size: u32,
    csa_alignment: u32,
    userq_ip_mask: u32,
    pad: u32,
}

pub(crate) const DRM_IOCTL_AMDGPU_GEM_CREATE: Ioctl =
    _IOWR::<drm_amdgpu_gem_create>(DRM_IOCTL_BASE, DRM_COMMAND_BASE);
pub(crate) const DRM_IOCTL_AMDGPU_INFO: Ioctl =
    _IOW::<drm_amdgpu_info>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x05);
pub(crate) const DRM_IOCTL_AMDGPU_GEM_METADATA: Ioctl =
    _IOWR::<drm_amdgpu_gem_metadata>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x06);

pub(crate) const AMDGPU_INFO_HW_IP_INFO: u32 = 0x02;
pub(crate) const AMDGPU_INFO_READ_MMR_REG: u32 = 0x15;
pub(crate) const AMDGPU_INFO_DEV_INFO: u32 = 0x16;
pub(crate) const AMDGPU_INFO_MMR_BROADCAST: u32 = 0xffff_ffff;
pub(crate) const MM_GB_ADDR_CONFIG: u32 = 0x263e;
pub(crate) const MM_GB_TILE_MODE0: u32 = 0x2644;
pub(crate) const MM_GB_MACROTILE_MODE0: u32 = 0x2664;
pub(crate) const AMDGPU_DRM_MINOR_GFX12_DCC_WRITE_COMPRESS_DISABLE: c_int = 60;
pub(crate) const AMDGPU_IDS_FLAGS_FUSION: u64 = 0x1;
pub(crate) const FAMILY_CI: u32 = 0x78;
pub(crate) const FAMILY_AI: u32 = 0x8d;
pub(crate) const FAMILY_NV: u32 = 0x8f;
pub(crate) const AMDGPU_GFX1013_RANGE: Range<u32> = 0x82..0x86;

pub(crate) const AMDGPU_GEM_DOMAIN_GTT: u64 = 0x2;
pub(crate) const AMDGPU_GEM_DOMAIN_VRAM: u64 = 0x4;

pub(crate) const AMDGPU_GEM_CREATE_NO_CPU_ACCESS: u64 = 1 << 1;
pub(crate) const AMDGPU_GEM_CREATE_CPU_GTT_USWC: u64 = 1 << 2;
pub(crate) const AMDGPU_GEM_CREATE_VRAM_CLEARED: u64 = 1 << 3;
pub(crate) const AMDGPU_GEM_CREATE_EXPLICIT_SYNC: u64 = 1 << 7;
pub(crate) const AMDGPU_GEM_CREATE_GFX12_DCC: u64 = 1 << 16;

pub(crate) const AMDGPU_GEM_METADATA_OP_SET_METADATA: u32 = 1;

pub(crate) const AMDGPU_HW_IP_GFX: u32 = 0;
pub(crate) const AMDGPU_HW_IP_COMPUTE: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub(crate) struct drm_amdgpu_info_hw_ip {
    pub(crate) hw_ip_version_major: u32,
    pub(crate) hw_ip_version_minor: u32,
    capabilities_flags: u64,
    ib_start_alignment: u32,
    ib_size_alignment: u32,
    pub(crate) available_rings: u32,
    pub(crate) ip_discovery_version: u32,
    userq_num_slots: u32,
}
