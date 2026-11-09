use crate::bitfield::bitfields;
use crate::uapi::amdgpu::drm_amdgpu_info;
use crate::uapi::asahi::drm_asahi_get_params;
use core::ffi::c_char;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct amdgpu_buffer_size_alignments {
    pub size_local: u64,
    pub size_remote: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct amdgpu_heap_info {
    pub heap_size: u64,
    pub heap_usage: u64,
    pub max_allocation: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct amdgpu_gpu_info {
    pub asic_id: u32,
    pub chip_rev: u32,
    pub chip_external_rev: u32,
    pub family_id: u32,
    pub ids_flags: u64,
    pub max_engine_clk: u64,
    pub max_memory_clk: u64,
    pub num_shader_engines: u32,
    pub num_shader_arrays_per_engine: u32,
    pub avail_quad_shader_pipes: u32,
    pub max_quad_shader_pipes: u32,
    pub cache_entries_per_quad_pipe: u32,
    pub num_hw_gfx_contexts: u32,
    pub rb_pipes: u32,
    pub enabled_rb_pipes_mask: u32,
    pub gpu_counter_freq: u32,
    pub backend_disable: [u32; 4],
    pub mc_arb_ramcfg: u32,
    pub gb_addr_cfg: u32,
    pub gb_tile_mode: [u32; 32],
    pub gb_macro_tile_mode: [u32; 16],
    pub pa_sc_raster_cfg: [u32; 4],
    pub pa_sc_raster_cfg1: [u32; 4],
    pub cu_active_number: u32,
    pub cu_ao_mask: u32,
    pub cu_bitmap: [[u32; 4]; 4],
    pub vram_type: u32,
    pub vram_bit_width: u32,
    pub ce_ram_size: u32,
    pub vce_harvest_config: u32,
    pub pci_rev_id: u32,
}

pub const VIRTGPU_DRM_CONTEXT_AMDGPU: u32 = 2;
pub const VIRTGPU_DRM_CONTEXT_I915: u32 = 3;
pub const VIRTGPU_DRM_CONTEXT_ASAHI: u32 = 4;
pub const VIRTGPU_DRM_CONTEXT_PANFROST: u32 = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virgl_renderer_capset_drm__u__msm {
    pub has_cached_coherent: u32,
    pub priorities: u32,
    pub va_start: u64,
    pub va_size: u64,
    pub gpu_id: u32,
    pub gmem_size: u32,
    pub gmem_base: u64,
    pub chip_id: u64,
    pub max_freq: u32,
    pub highest_bank_bit: u32,
    pub ubwc_swizzle: u64,
    pub macrotile_mode: u64,
    pub has_raytracing: u32,
    pub has_preemption: u32,
    pub uche_trap_base: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct virgl_renderer_capset_drm__u__amdgpu {
    pub address32_hi: u32,
    pub _bitfield_1: u32,
    pub alignments: amdgpu_buffer_size_alignments,
    pub gpu_info: amdgpu_gpu_info,
    pub marketing_name: [c_char; 128],
}

impl virgl_renderer_capset_drm__u__amdgpu {
    bitfields! {
        _bitfield_1 {
            has_vm_always_valid, set_has_vm_always_valid: 0, 1;
            __pad, set___pad: 1, 31;
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct virgl_renderer_capset_drm__u__intel {
    pub pci_bus: u32,
    pub pci_dev: u32,
    pub pci_func: u32,
    pub pci_revision_id: u32,
    pub pci_domain: u32,
    pub pci_device_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union virgl_renderer_capset_drm__u {
    pub msm: virgl_renderer_capset_drm__u__msm,
    pub amdgpu: virgl_renderer_capset_drm__u__amdgpu,
    pub intel: virgl_renderer_capset_drm__u__intel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virgl_renderer_capset_drm {
    pub wire_format_version: u32,
    pub version_major: u32,
    pub version_minor: u32,
    pub version_patchlevel: u32,
    pub context_type: u32,
    pub pad: u32,
    pub u: virgl_renderer_capset_drm__u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdrm_shmem {
    pub seqno: u32,
    pub rsp_mem_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct vdrm_ccmd_req {
    pub cmd: u32,
    pub len: u32,
    pub seqno: u32,
    pub rsp_off: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdrm_ccmd_rsp {
    pub len: u32,
}

pub const AMDGPU_CCMD_QUERY_INFO: u32 = 1;
pub const AMDGPU_CCMD_GEM_NEW: u32 = 2;
pub const AMDGPU_CCMD_BO_VA_OP: u32 = 3;
pub const AMDGPU_CCMD_CS_SUBMIT: u32 = 4;
pub const AMDGPU_CCMD_SET_METADATA: u32 = 5;
pub const AMDGPU_CCMD_BO_QUERY_INFO: u32 = 6;
pub const AMDGPU_CCMD_CREATE_CTX: u32 = 7;
pub const AMDGPU_CCMD_RESERVE_VMID: u32 = 8;
pub const AMDGPU_CCMD_SET_PSTATE: u32 = 9;
pub const AMDGPU_CCMD_CS_QUERY_FENCE_STATUS: u32 = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ccmd_rsp {
    pub base: vdrm_ccmd_rsp,
    pub ret: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdvgpu_shmem {
    pub base: vdrm_shmem,
    pub async_error: u32,
    pub __pad: u32,
    pub gtt: amdgpu_heap_info,
    pub vram: amdgpu_heap_info,
    pub vis_vram: amdgpu_heap_info,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct amdgpu_ccmd_query_info_req {
    pub hdr: vdrm_ccmd_req,
    pub info: drm_amdgpu_info,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct amdgpu_ccmd_gem_new_req__r {
    pub alloc_size: u64,
    pub phys_alignment: u64,
    pub preferred_heap: u32,
    pub __pad: u32,
    pub flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct amdgpu_ccmd_gem_new_req {
    pub hdr: vdrm_ccmd_req,
    pub blob_id: u64,
    pub r: amdgpu_ccmd_gem_new_req__r,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct amdgpu_ccmd_bo_va_op_req {
    pub hdr: vdrm_ccmd_req,
    pub va: u64,
    pub vm_map_size: u64,
    pub flags: u64,
    pub flags2: u64,
    pub offset: u64,
    pub res_id: u32,
    pub op: u32,
}

pub const AMDGPU_CCMD_BO_VA_OP_SPARSE_BO: u64 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct amdgpu_ccmd_cs_submit_req {
    pub hdr: vdrm_ccmd_req,
    pub ctx_id: u32,
    pub num_chunks: u32,
    pub pad: u32,
    pub ring_idx: u32,
    pub payload: [u8; 0],
}

pub const AMDGPU_CCMD_CS_SUBMIT_MAX_NUM_CHUNKS: u32 = 128;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct amdgpu_ccmd_set_metadata_req {
    pub hdr: vdrm_ccmd_req,
    pub flags: u64,
    pub tiling_info: u64,
    pub res_id: u32,
    pub size_metadata: u32,
    pub umd_metadata: [u32; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ccmd_bo_query_info_req {
    pub hdr: vdrm_ccmd_req,
    pub res_id: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ccmd_bo_query_info_rsp__info__metadata {
    pub flags: u64,
    pub tiling_info: u64,
    pub size_metadata: u32,
    pub umd_metadata: [u32; 64],
    pub __pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ccmd_bo_query_info_rsp__info {
    pub alloc_size: u64,
    pub phys_alignment: u64,
    pub preferred_heap: u32,
    pub __pad: u32,
    pub alloc_flags: u64,
    pub metadata: amdgpu_ccmd_bo_query_info_rsp__info__metadata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ccmd_bo_query_info_rsp {
    pub hdr: amdgpu_ccmd_rsp,
    pub info: amdgpu_ccmd_bo_query_info_rsp__info,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub union amdgpu_ccmd_create_ctx_req__anon {
    pub priority: i32,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct amdgpu_ccmd_create_ctx_req {
    pub hdr: vdrm_ccmd_req,
    pub anon: amdgpu_ccmd_create_ctx_req__anon,
    pub flags: u32,
}

pub const AMDGPU_CCMD_CREATE_CTX_DESTROY: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ccmd_create_ctx_rsp {
    pub hdr: amdgpu_ccmd_rsp,
    pub ctx_id: u32,
    pub pad: u32,
}

pub const I915_CCMD_IOCTL_SIMPLE: u32 = 1;
pub const I915_CCMD_GETPARAM: u32 = 2;
pub const I915_CCMD_QUERYPARAM: u32 = 3;
pub const I915_CCMD_GEM_CREATE: u32 = 4;
pub const I915_CCMD_GEM_CREATE_EXT: u32 = 5;
pub const I915_CCMD_GEM_CONTEXT_CREATE: u32 = 6;
pub const I915_CCMD_GEM_EXECBUFFER2: u32 = 7;
pub const I915_CCMD_GEM_SET_MMAP_MODE: u32 = 8;
pub const I915_CCMD_GEM_BUSY: u32 = 9;
pub const I915_CCMD_GEM_EXECBUFFER2_V2: u32 = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_shmem {
    pub base: vdrm_shmem,
    pub banned_ctx_mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct i915_ccmd_ioctl_simple_req {
    pub hdr: vdrm_ccmd_req,
    pub cmd: u32,
    pub pad: u32,
    pub payload: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_ccmd_ioctl_simple_rsp {
    pub hdr: vdrm_ccmd_rsp,
    pub ret: i32,
    pub payload: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct i915_ccmd_getparam_req {
    pub hdr: vdrm_ccmd_req,
    pub param: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_ccmd_getparam_rsp {
    pub hdr: vdrm_ccmd_rsp,
    pub pad: u32,
    pub ret: i32,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct i915_ccmd_queryparam_req {
    pub hdr: vdrm_ccmd_req,
    pub query_id: u32,
    pub length: u32,
    pub flags: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_ccmd_queryparam_rsp {
    pub hdr: vdrm_ccmd_rsp,
    pub pad: u32,
    pub ret: i32,
    pub length: i32,
    pub payload: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct i915_ccmd_gem_create_req {
    pub hdr: vdrm_ccmd_req,
    pub size: u64,
    pub blob_id: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct i915_ccmd_gem_create_ext_req {
    pub hdr: vdrm_ccmd_req,
    pub size: u64,
    pub blob_id: u32,
    pub gem_flags: u32,
    pub ext_size: u32,
    pub pad: u32,
    pub payload: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct i915_ccmd_gem_set_mmap_mode_req {
    pub hdr: vdrm_ccmd_req,
    pub res_id: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asahi_shmem {
    pub base: vdrm_shmem,
    pub async_error: u32,
    pub global_faults: u32,
}

pub const ASAHI_CCMD_NOP: u32 = 1;
pub const ASAHI_CCMD_IOCTL_SIMPLE: u32 = 2;
pub const ASAHI_CCMD_GET_PARAMS: u32 = 3;
pub const ASAHI_CCMD_GEM_NEW: u32 = 4;
pub const ASAHI_CCMD_VM_BIND: u32 = 5;
pub const ASAHI_CCMD_SUBMIT: u32 = 6;
pub const ASAHI_CCMD_GEM_BIND_OBJECT: u32 = 7;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct asahi_ccmd_ioctl_simple_req {
    pub hdr: vdrm_ccmd_req,
    pub cmd: u32,
    pub payload: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asahi_ccmd_ioctl_simple_rsp {
    pub hdr: vdrm_ccmd_rsp,
    pub ret: i32,
    pub payload: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct asahi_ccmd_get_params_req {
    pub hdr: vdrm_ccmd_req,
    pub params: drm_asahi_get_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asahi_ccmd_get_params_rsp {
    pub hdr: vdrm_ccmd_rsp,
    pub ret: i32,
    pub payload: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct asahi_ccmd_gem_new_req {
    pub hdr: vdrm_ccmd_req,
    pub flags: u32,
    pub bind_flags: u32,
    pub vm_id: u32,
    pub blob_id: u32,
    pub size: u64,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct asahi_ccmd_vm_bind_req {
    pub hdr: vdrm_ccmd_req,
    pub vm_id: u32,
    pub stride: u32,
    pub count: u32,
    pub payload: [u8; 0],
}

pub const PANFROST_CCMD_SUBMIT: u32 = 1;
pub const PANFROST_CCMD_WAIT_BO: u32 = 2;
pub const PANFROST_CCMD_CREATE_BO: u32 = 3;
pub const PANFROST_CCMD_MMAP_BO: u32 = 4;
pub const PANFROST_CCMD_GET_PARAM: u32 = 5;
pub const PANFROST_CCMD_GET_BO_OFFSET: u32 = 6;
pub const PANFROST_CCMD_MADVISE: u32 = 7;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct panfrost_ccmd_create_bo_req {
    pub hdr: vdrm_ccmd_req,
    pub pad: u32,
    pub size: u32,
    pub blob_id: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct panfrost_ccmd_get_param_req {
    pub hdr: vdrm_ccmd_req,
    pub pad: u32,
    pub param: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_ccmd_get_param_rsp {
    pub hdr: vdrm_ccmd_rsp,
    pub ret: i32,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct panfrost_ccmd_get_bo_offset_req {
    pub hdr: vdrm_ccmd_req,
    pub pad: u32,
    pub res_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_ccmd_get_bo_offset_rsp {
    pub hdr: vdrm_ccmd_rsp,
    pub ret: i32,
    pub offset: u64,
}
