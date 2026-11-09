use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IOW;
use crate::uapi::drm::DRM_IOWR;

pub const DRM_AMDGPU_GEM_CREATE: u32 = 0x00;
pub const DRM_AMDGPU_GEM_MMAP: u32 = 0x01;
pub const DRM_AMDGPU_CTX: u32 = 0x02;
pub const DRM_AMDGPU_BO_LIST: u32 = 0x03;
pub const DRM_AMDGPU_CS: u32 = 0x04;
pub const DRM_AMDGPU_INFO: u32 = 0x05;
pub const DRM_AMDGPU_GEM_METADATA: u32 = 0x06;
pub const DRM_AMDGPU_GEM_WAIT_IDLE: u32 = 0x07;
pub const DRM_AMDGPU_GEM_VA: u32 = 0x08;
pub const DRM_AMDGPU_WAIT_CS: u32 = 0x09;
pub const DRM_AMDGPU_GEM_OP: u32 = 0x10;
pub const DRM_AMDGPU_GEM_USERPTR: u32 = 0x11;
pub const DRM_AMDGPU_WAIT_FENCES: u32 = 0x12;
pub const DRM_AMDGPU_VM: u32 = 0x13;
pub const DRM_AMDGPU_FENCE_TO_HANDLE: u32 = 0x14;
pub const DRM_AMDGPU_SCHED: u32 = 0x15;
pub const DRM_AMDGPU_USERQ: u32 = 0x16;
pub const DRM_AMDGPU_USERQ_SIGNAL: u32 = 0x17;
pub const DRM_AMDGPU_USERQ_WAIT: u32 = 0x18;
pub const DRM_AMDGPU_GEM_LIST_HANDLES: u32 = 0x19;

pub const DRM_IOCTL_AMDGPU_GEM_CREATE: u32 =
    DRM_IOWR::<drm_amdgpu_gem_create>(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_CREATE);
pub const DRM_IOCTL_AMDGPU_GEM_MMAP: u32 =
    DRM_IOWR::<drm_amdgpu_gem_mmap>(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_MMAP);
pub const DRM_IOCTL_AMDGPU_CTX: u32 = DRM_IOWR::<drm_amdgpu_ctx>(DRM_COMMAND_BASE + DRM_AMDGPU_CTX);
pub const DRM_IOCTL_AMDGPU_BO_LIST: u32 =
    DRM_IOWR::<drm_amdgpu_bo_list>(DRM_COMMAND_BASE + DRM_AMDGPU_BO_LIST);
pub const DRM_IOCTL_AMDGPU_CS: u32 = DRM_IOWR::<drm_amdgpu_cs>(DRM_COMMAND_BASE + DRM_AMDGPU_CS);
pub const DRM_IOCTL_AMDGPU_INFO: u32 =
    DRM_IOW::<drm_amdgpu_info>(DRM_COMMAND_BASE + DRM_AMDGPU_INFO);
pub const DRM_IOCTL_AMDGPU_GEM_METADATA: u32 =
    DRM_IOWR::<drm_amdgpu_gem_metadata>(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_METADATA);
pub const DRM_IOCTL_AMDGPU_GEM_WAIT_IDLE: u32 =
    DRM_IOWR::<drm_amdgpu_gem_wait_idle>(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_WAIT_IDLE);
pub const DRM_IOCTL_AMDGPU_GEM_VA: u32 =
    DRM_IOW::<drm_amdgpu_gem_va>(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_VA);
pub const DRM_IOCTL_AMDGPU_WAIT_CS: u32 =
    DRM_IOWR::<drm_amdgpu_wait_cs>(DRM_COMMAND_BASE + DRM_AMDGPU_WAIT_CS);
pub const DRM_IOCTL_AMDGPU_GEM_OP: u32 =
    DRM_IOWR::<drm_amdgpu_gem_op>(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_OP);
pub const DRM_IOCTL_AMDGPU_GEM_USERPTR: u32 =
    DRM_IOWR::<drm_amdgpu_gem_userptr>(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_USERPTR);
pub const DRM_IOCTL_AMDGPU_WAIT_FENCES: u32 =
    DRM_IOWR::<drm_amdgpu_wait_fences>(DRM_COMMAND_BASE + DRM_AMDGPU_WAIT_FENCES);
pub const DRM_IOCTL_AMDGPU_VM: u32 = DRM_IOWR::<drm_amdgpu_vm>(DRM_COMMAND_BASE + DRM_AMDGPU_VM);
pub const DRM_IOCTL_AMDGPU_FENCE_TO_HANDLE: u32 =
    DRM_IOWR::<drm_amdgpu_fence_to_handle>(DRM_COMMAND_BASE + DRM_AMDGPU_FENCE_TO_HANDLE);
pub const DRM_IOCTL_AMDGPU_SCHED: u32 =
    DRM_IOW::<drm_amdgpu_sched>(DRM_COMMAND_BASE + DRM_AMDGPU_SCHED);
pub const DRM_IOCTL_AMDGPU_USERQ: u32 =
    DRM_IOWR::<drm_amdgpu_userq>(DRM_COMMAND_BASE + DRM_AMDGPU_USERQ);
pub const DRM_IOCTL_AMDGPU_USERQ_SIGNAL: u32 =
    DRM_IOWR::<drm_amdgpu_userq_signal>(DRM_COMMAND_BASE + DRM_AMDGPU_USERQ_SIGNAL);
pub const DRM_IOCTL_AMDGPU_USERQ_WAIT: u32 =
    DRM_IOWR::<drm_amdgpu_userq_wait>(DRM_COMMAND_BASE + DRM_AMDGPU_USERQ_WAIT);
pub const DRM_IOCTL_AMDGPU_GEM_LIST_HANDLES: u32 =
    DRM_IOWR::<drm_amdgpu_gem_list_handles>(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_LIST_HANDLES);

pub const AMDGPU_GEM_DOMAIN_CPU: u64 = 0x1;
pub const AMDGPU_GEM_DOMAIN_GTT: u64 = 0x2;
pub const AMDGPU_GEM_DOMAIN_VRAM: u64 = 0x4;
pub const AMDGPU_GEM_DOMAIN_GDS: u64 = 0x8;
pub const AMDGPU_GEM_DOMAIN_GWS: u64 = 0x10;
pub const AMDGPU_GEM_DOMAIN_OA: u64 = 0x20;
pub const AMDGPU_GEM_DOMAIN_DOORBELL: u64 = 0x40;
pub const AMDGPU_GEM_DOMAIN_MASK: u64 = AMDGPU_GEM_DOMAIN_CPU
    | AMDGPU_GEM_DOMAIN_GTT
    | AMDGPU_GEM_DOMAIN_VRAM
    | AMDGPU_GEM_DOMAIN_GDS
    | AMDGPU_GEM_DOMAIN_GWS
    | AMDGPU_GEM_DOMAIN_OA
    | AMDGPU_GEM_DOMAIN_DOORBELL;

pub const AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED: u64 = 1 << 0;
pub const AMDGPU_GEM_CREATE_NO_CPU_ACCESS: u64 = 1 << 1;
pub const AMDGPU_GEM_CREATE_CPU_GTT_USWC: u64 = 1 << 2;
pub const AMDGPU_GEM_CREATE_VRAM_CLEARED: u64 = 1 << 3;
pub const AMDGPU_GEM_CREATE_VRAM_CONTIGUOUS: u64 = 1 << 5;
pub const AMDGPU_GEM_CREATE_VM_ALWAYS_VALID: u64 = 1 << 6;
pub const AMDGPU_GEM_CREATE_EXPLICIT_SYNC: u64 = 1 << 7;
pub const AMDGPU_GEM_CREATE_CP_MQD_GFX9: u64 = 1 << 8;
pub const AMDGPU_GEM_CREATE_VRAM_WIPE_ON_RELEASE: u64 = 1 << 9;
pub const AMDGPU_GEM_CREATE_ENCRYPTED: u64 = 1 << 10;
pub const AMDGPU_GEM_CREATE_PREEMPTIBLE: u64 = 1 << 11;
pub const AMDGPU_GEM_CREATE_DISCARDABLE: u64 = 1 << 12;
pub const AMDGPU_GEM_CREATE_COHERENT: u64 = 1 << 13;
pub const AMDGPU_GEM_CREATE_UNCACHED: u64 = 1 << 14;
pub const AMDGPU_GEM_CREATE_EXT_COHERENT: u64 = 1 << 15;
pub const AMDGPU_GEM_CREATE_GFX12_DCC: u64 = 1 << 16;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_amdgpu_gem_create_in {
    pub bo_size: u64,
    pub alignment: u64,
    pub domains: u64,
    pub domain_flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_gem_create_out {
    pub handle: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub union drm_amdgpu_gem_create {
    pub in_: drm_amdgpu_gem_create_in,
    pub out: drm_amdgpu_gem_create_out,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_bo_list_in {
    pub operation: u32,
    pub list_handle: u32,
    pub bo_number: u32,
    pub bo_info_size: u32,
    pub bo_info_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_bo_list_entry {
    pub bo_handle: u32,
    pub bo_priority: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_bo_list_out {
    pub list_handle: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_bo_list {
    pub in_: drm_amdgpu_bo_list_in,
    pub out: drm_amdgpu_bo_list_out,
}

pub const AMDGPU_CTX_OP_ALLOC_CTX: u32 = 1;
pub const AMDGPU_CTX_OP_FREE_CTX: u32 = 2;
pub const AMDGPU_CTX_OP_QUERY_STATE: u32 = 3;
pub const AMDGPU_CTX_OP_QUERY_STATE2: u32 = 4;
pub const AMDGPU_CTX_OP_GET_STABLE_PSTATE: u32 = 5;
pub const AMDGPU_CTX_OP_SET_STABLE_PSTATE: u32 = 6;

pub const AMDGPU_CTX_PRIORITY_VERY_LOW: i32 = -1023;
pub const AMDGPU_CTX_PRIORITY_LOW: i32 = -512;
pub const AMDGPU_CTX_PRIORITY_NORMAL: i32 = 0;
pub const AMDGPU_CTX_PRIORITY_HIGH: i32 = 512;
pub const AMDGPU_CTX_PRIORITY_VERY_HIGH: i32 = 1023;

pub const AMDGPU_CTX_STABLE_PSTATE_FLAGS_MASK: u32 = 0xf;
pub const AMDGPU_CTX_STABLE_PSTATE_PEAK: u32 = 4;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_ctx_in {
    pub op: u32,
    pub flags: u32,
    pub ctx_id: u32,
    pub priority: i32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_ctx_out__alloc {
    pub ctx_id: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_ctx_out__state {
    pub flags: u64,
    pub hangs: u32,
    pub reset_status: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_ctx_out__pstate {
    pub flags: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub union drm_amdgpu_ctx_out {
    pub alloc: drm_amdgpu_ctx_out__alloc,
    pub state: drm_amdgpu_ctx_out__state,
    pub pstate: drm_amdgpu_ctx_out__pstate,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub union drm_amdgpu_ctx {
    pub in_: drm_amdgpu_ctx_in,
    pub out: drm_amdgpu_ctx_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_userq_in {
    pub op: u32,
    pub queue_id: u32,
    pub ip_type: u32,
    pub doorbell_handle: u32,
    pub doorbell_offset: u32,
    pub flags: u32,
    pub queue_va: u64,
    pub queue_size: u64,
    pub rptr_va: u64,
    pub wptr_va: u64,
    pub mqd: u64,
    pub mqd_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_userq_out {
    pub queue_id: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_userq {
    pub in_: drm_amdgpu_userq_in,
    pub out: drm_amdgpu_userq_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_userq_signal {
    pub queue_id: u32,
    pub pad: u32,
    pub syncobj_handles: u64,
    pub num_syncobj_handles: u16,
    pub pad0: u16,
    pub pad1: u32,
    pub bo_read_handles: u64,
    pub bo_write_handles: u64,
    pub num_bo_read_handles: u32,
    pub num_bo_write_handles: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_userq_wait {
    pub waitq_id: u32,
    pub pad: u32,
    pub syncobj_handles: u64,
    pub syncobj_timeline_handles: u64,
    pub syncobj_timeline_points: u64,
    pub bo_read_handles: u64,
    pub bo_write_handles: u64,
    pub num_syncobj_timeline_handles: u16,
    pub num_fences: u16,
    pub num_syncobj_handles: u16,
    pub pad0: u16,
    pub num_bo_read_handles: u32,
    pub num_bo_write_handles: u32,
    pub out_fences: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_vm_in {
    pub op: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_vm_out {
    pub flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_vm {
    pub in_: drm_amdgpu_vm_in,
    pub out: drm_amdgpu_vm_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_sched_in {
    pub op: u32,
    pub fd: u32,
    pub priority: i32,
    pub ctx_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_sched {
    pub in_: drm_amdgpu_sched_in,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_userptr {
    pub addr: u64,
    pub size: u64,
    pub flags: u32,
    pub handle: u32,
}

pub const AMDGPU_TILING_TILE_SPLIT_SHIFT: u64 = 9;
pub const AMDGPU_TILING_TILE_SPLIT_MASK: u64 = 0x7;

pub const AMDGPU_GEM_METADATA_OP_SET_METADATA: u32 = 1;
pub const AMDGPU_GEM_METADATA_OP_GET_METADATA: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_gem_metadata__data {
    pub flags: u64,
    pub tiling_info: u64,
    pub data_size_bytes: u32,
    pub data: [u32; 64],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_gem_metadata {
    pub handle: u32,
    pub op: u32,
    pub data: drm_amdgpu_gem_metadata__data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_mmap_in {
    pub handle: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_mmap_out {
    pub addr_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_gem_mmap {
    pub in_: drm_amdgpu_gem_mmap_in,
    pub out: drm_amdgpu_gem_mmap_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_wait_idle_in {
    pub handle: u32,
    pub flags: u32,
    pub timeout: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_wait_idle_out {
    pub status: u32,
    pub domain: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_gem_wait_idle {
    pub in_: drm_amdgpu_gem_wait_idle_in,
    pub out: drm_amdgpu_gem_wait_idle_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_wait_cs_in {
    pub handle: u64,
    pub timeout: u64,
    pub ip_type: u32,
    pub ip_instance: u32,
    pub ring: u32,
    pub ctx_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_wait_cs_out {
    pub status: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_wait_cs {
    pub in_: drm_amdgpu_wait_cs_in,
    pub out: drm_amdgpu_wait_cs_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_fence {
    pub ctx_id: u32,
    pub ip_type: u32,
    pub ip_instance: u32,
    pub ring: u32,
    pub seq_no: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_wait_fences_in {
    pub fences: u64,
    pub fence_count: u32,
    pub wait_all: u32,
    pub timeout_ns: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_wait_fences_out {
    pub status: u32,
    pub first_signaled: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_wait_fences {
    pub in_: drm_amdgpu_wait_fences_in,
    pub out: drm_amdgpu_wait_fences_out,
}

pub const AMDGPU_GEM_OP_GET_GEM_CREATE_INFO: u32 = 0;
pub const AMDGPU_GEM_OP_SET_PLACEMENT: u32 = 1;
pub const AMDGPU_GEM_OP_GET_MAPPING_INFO: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_op {
    pub handle: u32,
    pub op: u32,
    pub value: u64,
    pub num_entries: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_list_handles {
    pub entries: u64,
    pub num_entries: u32,
    pub padding: u32,
}

pub const AMDGPU_VA_OP_MAP: u32 = 1;
pub const AMDGPU_VA_OP_UNMAP: u32 = 2;
pub const AMDGPU_VA_OP_CLEAR: u32 = 3;
pub const AMDGPU_VA_OP_REPLACE: u32 = 4;

pub const AMDGPU_VM_DELAY_UPDATE: u32 = 1 << 0;

pub const AMDGPU_VM_PAGE_READABLE: u32 = 1 << 1;
pub const AMDGPU_VM_PAGE_WRITEABLE: u32 = 1 << 2;
pub const AMDGPU_VM_PAGE_EXECUTABLE: u32 = 1 << 3;
pub const AMDGPU_VM_PAGE_PRT: u32 = 1 << 4;
pub const AMDGPU_VM_MTYPE_MASK: u32 = 0xf << 5;
pub const AMDGPU_VM_MTYPE_DEFAULT: u32 = 0 << 5;
pub const AMDGPU_VM_MTYPE_UC: u32 = 4 << 5;
pub const AMDGPU_VM_PAGE_NOALLOC: u32 = 1 << 9;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_gem_va {
    pub handle: u32,
    pub _pad: u32,
    pub operation: u32,
    pub flags: u32,
    pub va_address: u64,
    pub offset_in_bo: u64,
    pub map_size: u64,
    pub vm_timeline_point: u64,
    pub vm_timeline_syncobj_out: u32,
    pub num_syncobj_handles: u32,
    pub input_fence_syncobj_handles: u64,
}

pub const AMDGPU_HW_IP_GFX: u32 = 0;
pub const AMDGPU_HW_IP_COMPUTE: u32 = 1;
pub const AMDGPU_HW_IP_NUM: u32 = 10;

pub const AMDGPU_HW_IP_INSTANCE_MAX_COUNT: u32 = 1;

pub const AMDGPU_CHUNK_ID_IB: u32 = 0x01;
pub const AMDGPU_CHUNK_ID_FENCE: u32 = 0x02;
pub const AMDGPU_CHUNK_ID_DEPENDENCIES: u32 = 0x03;
pub const AMDGPU_CHUNK_ID_SYNCOBJ_IN: u32 = 0x04;
pub const AMDGPU_CHUNK_ID_SYNCOBJ_OUT: u32 = 0x05;
pub const AMDGPU_CHUNK_ID_BO_HANDLES: u32 = 0x06;
pub const AMDGPU_CHUNK_ID_SCHEDULED_DEPENDENCIES: u32 = 0x07;
pub const AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_WAIT: u32 = 0x08;
pub const AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_SIGNAL: u32 = 0x09;
pub const AMDGPU_CHUNK_ID_CP_GFX_SHADOW: u32 = 0x0a;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_cs_chunk {
    pub chunk_id: u32,
    pub length_dw: u32,
    pub chunk_data: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_cs_in {
    pub ctx_id: u32,
    pub bo_list_handle: u32,
    pub num_chunks: u32,
    pub flags: u32,
    pub chunks: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_cs_out {
    pub handle: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub union drm_amdgpu_cs {
    pub in_: drm_amdgpu_cs_in,
    pub out: drm_amdgpu_cs_out,
}

pub const AMDGPU_IB_FLAG_CE: u32 = 1 << 0;

pub const AMDGPU_IB_FLAG_PREAMBLE: u32 = 1 << 1;

pub const AMDGPU_IB_FLAG_PREEMPT: u32 = 1 << 2;

pub const AMDGPU_IB_FLAG_TC_WB_NOT_INVALIDATE: u32 = 1 << 3;

pub const AMDGPU_IB_FLAG_EMIT_MEM_SYNC: u32 = 1 << 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_chunk_ib {
    pub _pad: u32,
    pub flags: u32,
    pub va_start: u64,
    pub ib_bytes: u32,
    pub ip_type: u32,
    pub ip_instance: u32,
    pub ring: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_cs_chunk_fence {
    pub handle: u32,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_chunk_sem {
    pub handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_chunk_syncobj {
    pub handle: u32,
    pub flags: u32,
    pub point: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_fence_to_handle__in {
    pub fence: drm_amdgpu_fence,
    pub what: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_fence_to_handle__out {
    pub handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_fence_to_handle {
    pub in_: drm_amdgpu_fence_to_handle__in,
    pub out: drm_amdgpu_fence_to_handle__out,
}

pub const AMDGPU_INFO_ACCEL_WORKING: u32 = 0x00;
pub const AMDGPU_INFO_CRTC_FROM_ID: u32 = 0x01;
pub const AMDGPU_INFO_HW_IP_INFO: u32 = 0x02;
pub const AMDGPU_INFO_HW_IP_COUNT: u32 = 0x03;
pub const AMDGPU_INFO_TIMESTAMP: u32 = 0x05;
pub const AMDGPU_INFO_FW_VERSION: u32 = 0x0e;
pub const AMDGPU_INFO_FW_VCE: u32 = 0x1;
pub const AMDGPU_INFO_FW_UVD: u32 = 0x2;
pub const AMDGPU_INFO_FW_GMC: u32 = 0x03;
pub const AMDGPU_INFO_FW_GFX_ME: u32 = 0x04;
pub const AMDGPU_INFO_FW_GFX_PFP: u32 = 0x05;
pub const AMDGPU_INFO_FW_GFX_CE: u32 = 0x06;
pub const AMDGPU_INFO_FW_GFX_RLC: u32 = 0x07;
pub const AMDGPU_INFO_FW_GFX_MEC: u32 = 0x08;
pub const AMDGPU_INFO_FW_SMC: u32 = 0x0a;
pub const AMDGPU_INFO_FW_SDMA: u32 = 0x0b;
pub const AMDGPU_INFO_FW_SOS: u32 = 0x0c;
pub const AMDGPU_INFO_FW_ASD: u32 = 0x0d;
pub const AMDGPU_INFO_FW_VCN: u32 = 0x0e;
pub const AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_CNTL: u32 = 0x0f;
pub const AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_GPM_MEM: u32 = 0x10;
pub const AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_SRM_MEM: u32 = 0x11;
pub const AMDGPU_INFO_FW_DMCU: u32 = 0x12;
pub const AMDGPU_INFO_FW_TA: u32 = 0x13;
pub const AMDGPU_INFO_FW_DMCUB: u32 = 0x14;
pub const AMDGPU_INFO_FW_TOC: u32 = 0x15;
pub const AMDGPU_INFO_FW_CAP: u32 = 0x16;
pub const AMDGPU_INFO_FW_GFX_RLCP: u32 = 0x17;
pub const AMDGPU_INFO_FW_GFX_RLCV: u32 = 0x18;
pub const AMDGPU_INFO_FW_MES_KIQ: u32 = 0x19;
pub const AMDGPU_INFO_FW_MES: u32 = 0x1a;
pub const AMDGPU_INFO_FW_IMU: u32 = 0x1b;
pub const AMDGPU_INFO_FW_VPE: u32 = 0x1c;

pub const AMDGPU_INFO_NUM_BYTES_MOVED: u32 = 0x0f;
pub const AMDGPU_INFO_VRAM_USAGE: u32 = 0x10;
pub const AMDGPU_INFO_GTT_USAGE: u32 = 0x11;
pub const AMDGPU_INFO_GDS_CONFIG: u32 = 0x13;
pub const AMDGPU_INFO_VRAM_GTT: u32 = 0x14;
pub const AMDGPU_INFO_READ_MMR_REG: u32 = 0x15;
pub const AMDGPU_INFO_DEV_INFO: u32 = 0x16;
pub const AMDGPU_INFO_VIS_VRAM_USAGE: u32 = 0x17;
pub const AMDGPU_INFO_NUM_EVICTIONS: u32 = 0x18;
pub const AMDGPU_INFO_MEMORY: u32 = 0x19;
pub const AMDGPU_INFO_VCE_CLOCK_TABLE: u32 = 0x1A;
pub const AMDGPU_INFO_VBIOS: u32 = 0x1B;
pub const AMDGPU_INFO_NUM_HANDLES: u32 = 0x1C;
pub const AMDGPU_INFO_SENSOR: u32 = 0x1D;
pub const AMDGPU_INFO_NUM_VRAM_CPU_PAGE_FAULTS: u32 = 0x1E;
pub const AMDGPU_INFO_VRAM_LOST_COUNTER: u32 = 0x1F;
pub const AMDGPU_INFO_RAS_ENABLED_FEATURES: u32 = 0x20;
pub const AMDGPU_INFO_VIDEO_CAPS: u32 = 0x21;
pub const AMDGPU_INFO_VIDEO_CAPS_DECODE: u32 = 0;
pub const AMDGPU_INFO_VIDEO_CAPS_ENCODE: u32 = 1;
pub const AMDGPU_INFO_MAX_IBS: u32 = 0x22;
pub const AMDGPU_INFO_GPUVM_FAULT: u32 = 0x23;
pub const AMDGPU_INFO_UQ_FW_AREAS: u32 = 0x24;

pub const AMDGPU_INFO_MMR_SE_INDEX_SHIFT: u32 = 0;
pub const AMDGPU_INFO_MMR_SE_INDEX_MASK: u32 = 0xff;
pub const AMDGPU_INFO_MMR_SH_INDEX_SHIFT: u32 = 8;
pub const AMDGPU_INFO_MMR_SH_INDEX_MASK: u32 = 0xff;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_query_fw {
    pub fw_type: u32,
    pub ip_instance: u32,
    pub index: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_info__anon__mode_crtc {
    pub id: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_info__anon__query_hw_ip {
    pub type_: u32,
    pub ip_instance: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_info__anon__read_mmr_reg {
    pub dword_offset: u32,
    pub count: u32,
    pub instance: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_info__anon__vbios_info {
    pub type_: u32,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_info__anon__sensor_info {
    pub type_: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_info__anon__video_cap {
    pub type_: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub union drm_amdgpu_info__anon {
    pub mode_crtc: drm_amdgpu_info__anon__mode_crtc,
    pub query_hw_ip: drm_amdgpu_info__anon__query_hw_ip,
    pub read_mmr_reg: drm_amdgpu_info__anon__read_mmr_reg,
    pub query_fw: drm_amdgpu_query_fw,
    pub vbios_info: drm_amdgpu_info__anon__vbios_info,
    pub sensor_info: drm_amdgpu_info__anon__sensor_info,
    pub video_cap: drm_amdgpu_info__anon__video_cap,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_amdgpu_info {
    pub return_pointer: u64,
    pub return_size: u32,
    pub query: u32,
    pub anon: drm_amdgpu_info__anon,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_amdgpu_info_vram_gtt {
    pub vram_size: u64,
    pub vram_cpu_accessible_size: u64,
    pub gtt_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_amdgpu_heap_info {
    pub total_heap_size: u64,
    pub usable_heap_size: u64,
    pub heap_usage: u64,
    pub max_allocation: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_amdgpu_memory_info {
    pub vram: drm_amdgpu_heap_info,
    pub cpu_accessible_vram: drm_amdgpu_heap_info,
    pub gtt: drm_amdgpu_heap_info,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_amdgpu_info_firmware {
    pub ver: u32,
    pub feature: u32,
}

pub const AMDGPU_VRAM_TYPE_GDDR6: u32 = 9;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_amdgpu_info_device {
    pub device_id: u32,
    pub chip_rev: u32,
    pub external_rev: u32,
    pub pci_rev: u32,
    pub family: u32,
    pub num_shader_engines: u32,
    pub num_shader_arrays_per_engine: u32,
    pub gpu_counter_freq: u32,
    pub max_engine_clock: u64,
    pub max_memory_clock: u64,
    pub cu_active_number: u32,
    pub cu_ao_mask: u32,
    pub cu_bitmap: [[u32; 4]; 4],
    pub enabled_rb_pipes_mask: u32,
    pub num_rb_pipes: u32,
    pub num_hw_gfx_contexts: u32,
    pub pcie_gen: u32,
    pub ids_flags: u64,
    pub virtual_address_offset: u64,
    pub virtual_address_max: u64,
    pub virtual_address_alignment: u32,
    pub pte_fragment_size: u32,
    pub gart_page_size: u32,
    pub ce_ram_size: u32,
    pub vram_type: u32,
    pub vram_bit_width: u32,
    pub vce_harvest_config: u32,
    pub gc_double_offchip_lds_buf: u32,
    pub prim_buf_gpu_addr: u64,
    pub pos_buf_gpu_addr: u64,
    pub cntl_sb_buf_gpu_addr: u64,
    pub param_buf_gpu_addr: u64,
    pub prim_buf_size: u32,
    pub pos_buf_size: u32,
    pub cntl_sb_buf_size: u32,
    pub param_buf_size: u32,
    pub wave_front_size: u32,
    pub num_shader_visible_vgprs: u32,
    pub num_cu_per_sh: u32,
    pub num_tcc_blocks: u32,
    pub gs_vgt_table_depth: u32,
    pub gs_prim_buffer_depth: u32,
    pub max_gs_waves_per_vgt: u32,
    pub pcie_num_lanes: u32,
    pub cu_ao_bitmap: [[u32; 4]; 4],
    pub high_va_offset: u64,
    pub high_va_max: u64,
    pub pa_sc_tile_steering_override: u32,
    pub _padding0: u32,
    pub tcc_disabled_mask: u64,
    pub min_engine_clock: u64,
    pub min_memory_clock: u64,
    pub tcp_cache_size: u32,
    pub num_sqc_per_wgp: u32,
    pub sqc_data_cache_size: u32,
    pub sqc_inst_cache_size: u32,
    pub gl1c_cache_size: u32,
    pub gl2c_cache_size: u32,
    pub mall_size: u64,
    pub enabled_rb_pipes_mask_hi: u32,
    pub shadow_size: u32,
    pub shadow_alignment: u32,
    pub csa_size: u32,
    pub csa_alignment: u32,
    pub userq_ip_mask: u32,
    pub pad: u32,
    pub _padding1: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_amdgpu_info_hw_ip {
    pub hw_ip_version_major: u32,
    pub hw_ip_version_minor: u32,
    pub capabilities_flags: u64,
    pub ib_start_alignment: u32,
    pub ib_size_alignment: u32,
    pub available_rings: u32,
    pub ip_discovery_version: u32,
    pub userq_num_slots: u32,
    pub _padding0: u32,
}

pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_COUNT: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_video_codec_info {
    pub valid: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub max_pixels_per_frame: u32,
    pub max_level: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_video_caps {
    pub codec_info: [drm_amdgpu_info_video_codec_info; AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_COUNT],
}

pub const AMDGPU_FAMILY_SI: u32 = 110;
pub const AMDGPU_FAMILY_CZ: u32 = 135;
pub const AMDGPU_FAMILY_AI: u32 = 141;
