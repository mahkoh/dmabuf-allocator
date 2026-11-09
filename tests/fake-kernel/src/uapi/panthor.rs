use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IOWR;

pub const DRM_PANTHOR_USER_MMIO_OFFSET_32BIT: u64 = 1 << 43;
pub const DRM_PANTHOR_USER_MMIO_OFFSET_64BIT: u64 = 1 << 56;

pub const DRM_PANTHOR_DEV_QUERY: u32 = 0x00;
pub const DRM_PANTHOR_VM_CREATE: u32 = 0x01;
pub const DRM_PANTHOR_VM_DESTROY: u32 = 0x02;
pub const DRM_PANTHOR_VM_BIND: u32 = 0x03;
pub const DRM_PANTHOR_VM_GET_STATE: u32 = 0x04;
pub const DRM_PANTHOR_BO_CREATE: u32 = 0x05;
pub const DRM_PANTHOR_BO_MMAP_OFFSET: u32 = 0x06;
pub const DRM_PANTHOR_GROUP_CREATE: u32 = 0x07;
pub const DRM_PANTHOR_GROUP_DESTROY: u32 = 0x08;
pub const DRM_PANTHOR_GROUP_SUBMIT: u32 = 0x09;
pub const DRM_PANTHOR_GROUP_GET_STATE: u32 = 0x0a;
pub const DRM_PANTHOR_TILER_HEAP_CREATE: u32 = 0x0b;
pub const DRM_PANTHOR_TILER_HEAP_DESTROY: u32 = 0x0c;
pub const DRM_PANTHOR_BO_SET_LABEL: u32 = 0x0d;
pub const DRM_PANTHOR_SET_USER_MMIO_OFFSET: u32 = 0x0e;
pub const DRM_PANTHOR_BO_SYNC: u32 = 0x0f;
pub const DRM_PANTHOR_BO_QUERY_INFO: u32 = 0x10;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_panthor_obj_array {
    pub stride: u32,
    pub count: u32,
    pub array: u64,
}

pub const DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_MASK: u32 = 0xff;
pub const DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_SYNCOBJ: u32 = 0;
pub const DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_TIMELINE_SYNCOBJ: u32 = 1;
pub const DRM_PANTHOR_SYNC_OP_SIGNAL: u32 = 1 << 31;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_panthor_sync_op {
    pub flags: u32,
    pub handle: u32,
    pub timeline_value: u64,
}

pub const DRM_PANTHOR_DEV_QUERY_GPU_INFO: u32 = 0;
pub const DRM_PANTHOR_DEV_QUERY_CSIF_INFO: u32 = 1;
pub const DRM_PANTHOR_DEV_QUERY_TIMESTAMP_INFO: u32 = 2;
pub const DRM_PANTHOR_DEV_QUERY_GROUP_PRIORITIES_INFO: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_panthor_gpu_info {
    pub gpu_id: u32,
    pub gpu_rev: u32,
    pub csf_id: u32,
    pub l2_features: u32,
    pub tiler_features: u32,
    pub mem_features: u32,
    pub mmu_features: u32,
    pub thread_features: u32,
    pub max_threads: u32,
    pub thread_max_workgroup_size: u32,
    pub thread_max_barrier_size: u32,
    pub coherency_features: u32,
    pub texture_features: [u32; 4],
    pub as_present: u32,
    pub selected_coherency: u32,
    pub shader_present: u64,
    pub l2_present: u64,
    pub tiler_present: u64,
    pub core_features: u32,
    pub pad: u32,
    pub gpu_features: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_panthor_csif_info {
    pub csg_slot_count: u32,
    pub cs_slot_count: u32,
    pub cs_reg_count: u32,
    pub scoreboard_slot_count: u32,
    pub unpreserved_cs_reg_count: u32,
    pub pad: u32,
}

pub const DRM_PANTHOR_TIMESTAMP_GPU: u32 = 1 << 0;
pub const DRM_PANTHOR_TIMESTAMP_GPU_OFFSET: u32 = 1 << 4;
pub const DRM_PANTHOR_TIMESTAMP_FREQ: u32 = 1 << 6;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_panthor_timestamp_info {
    pub timestamp_frequency: u64,
    pub current_timestamp: u64,
    pub timestamp_offset: u64,
    pub flags: u32,
    pub duration_nsec: u32,
    pub cycle_count: u64,
    pub cpu_timestamp_sec: u64,
    pub cpu_timestamp_nsec: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_panthor_group_priorities_info {
    pub allowed_mask: u8,
    pub pad: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_dev_query {
    pub type_: u32,
    pub size: u32,
    pub pointer: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_vm_create {
    pub flags: u32,
    pub id: u32,
    pub user_va_range: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_vm_destroy {
    pub id: u32,
    pub pad: u32,
}

pub const DRM_PANTHOR_VM_BIND_OP_MAP_READONLY: u32 = 1 << 0;
pub const DRM_PANTHOR_VM_BIND_OP_MAP_NOEXEC: u32 = 1 << 1;
pub const DRM_PANTHOR_VM_BIND_OP_MAP_UNCACHED: u32 = 1 << 2;
pub const DRM_PANTHOR_VM_BIND_OP_TYPE_MASK: u32 = 0xf << 28;
pub const DRM_PANTHOR_VM_BIND_OP_TYPE_MAP: u32 = 0 << 28;
pub const DRM_PANTHOR_VM_BIND_OP_TYPE_UNMAP: u32 = 1 << 28;
pub const DRM_PANTHOR_VM_BIND_OP_TYPE_SYNC_ONLY: u32 = 2 << 28;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_panthor_vm_bind_op {
    pub flags: u32,
    pub bo_handle: u32,
    pub bo_offset: u64,
    pub va: u64,
    pub size: u64,
    pub syncs: drm_panthor_obj_array,
}

pub const DRM_PANTHOR_VM_BIND_ASYNC: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_vm_bind {
    pub vm_id: u32,
    pub flags: u32,
    pub ops: drm_panthor_obj_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_vm_get_state {
    pub vm_id: u32,
    pub state: u32,
}

pub const DRM_PANTHOR_BO_NO_MMAP: u32 = 1 << 0;
pub const DRM_PANTHOR_BO_WB_MMAP: u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_panthor_bo_create {
    pub size: u64,
    pub flags: u32,
    pub exclusive_vm_id: u32,
    pub handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_bo_mmap_offset {
    pub handle: u32,
    pub pad: u32,
    pub offset: u64,
}

pub const PANTHOR_GROUP_PRIORITY_LOW: u8 = 0;
pub const PANTHOR_GROUP_PRIORITY_MEDIUM: u8 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_group_create {
    pub queues: drm_panthor_obj_array,
    pub max_compute_cores: u8,
    pub max_fragment_cores: u8,
    pub max_tiler_cores: u8,
    pub priority: u8,
    pub pad: u32,
    pub compute_core_mask: u64,
    pub fragment_core_mask: u64,
    pub tiler_core_mask: u64,
    pub vm_id: u32,
    pub group_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_group_destroy {
    pub group_handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_group_submit {
    pub group_handle: u32,
    pub pad: u32,
    pub queue_submits: drm_panthor_obj_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_group_get_state {
    pub group_handle: u32,
    pub state: u32,
    pub fatal_queues: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_tiler_heap_create {
    pub vm_id: u32,
    pub initial_chunk_count: u32,
    pub chunk_size: u32,
    pub max_chunks: u32,
    pub target_in_flight: u32,
    pub handle: u32,
    pub tiler_heap_ctx_gpu_va: u64,
    pub first_heap_chunk_gpu_va: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_tiler_heap_destroy {
    pub handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_bo_set_label {
    pub handle: u32,
    pub pad: u32,
    pub label: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_set_user_mmio_offset {
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_bo_sync {
    pub ops: drm_panthor_obj_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_bo_query_info {
    pub handle: u32,
    pub extra_flags: u32,
    pub create_flags: u32,
    pub pad: u32,
}

pub const DRM_IOCTL_PANTHOR_DEV_QUERY: u32 =
    DRM_IOWR::<drm_panthor_dev_query>(DRM_COMMAND_BASE + DRM_PANTHOR_DEV_QUERY);
pub const DRM_IOCTL_PANTHOR_VM_CREATE: u32 =
    DRM_IOWR::<drm_panthor_vm_create>(DRM_COMMAND_BASE + DRM_PANTHOR_VM_CREATE);
pub const DRM_IOCTL_PANTHOR_VM_DESTROY: u32 =
    DRM_IOWR::<drm_panthor_vm_destroy>(DRM_COMMAND_BASE + DRM_PANTHOR_VM_DESTROY);
pub const DRM_IOCTL_PANTHOR_VM_BIND: u32 =
    DRM_IOWR::<drm_panthor_vm_bind>(DRM_COMMAND_BASE + DRM_PANTHOR_VM_BIND);
pub const DRM_IOCTL_PANTHOR_VM_GET_STATE: u32 =
    DRM_IOWR::<drm_panthor_vm_get_state>(DRM_COMMAND_BASE + DRM_PANTHOR_VM_GET_STATE);
pub const DRM_IOCTL_PANTHOR_BO_CREATE: u32 =
    DRM_IOWR::<drm_panthor_bo_create>(DRM_COMMAND_BASE + DRM_PANTHOR_BO_CREATE);
pub const DRM_IOCTL_PANTHOR_BO_MMAP_OFFSET: u32 =
    DRM_IOWR::<drm_panthor_bo_mmap_offset>(DRM_COMMAND_BASE + DRM_PANTHOR_BO_MMAP_OFFSET);
pub const DRM_IOCTL_PANTHOR_GROUP_CREATE: u32 =
    DRM_IOWR::<drm_panthor_group_create>(DRM_COMMAND_BASE + DRM_PANTHOR_GROUP_CREATE);
pub const DRM_IOCTL_PANTHOR_GROUP_DESTROY: u32 =
    DRM_IOWR::<drm_panthor_group_destroy>(DRM_COMMAND_BASE + DRM_PANTHOR_GROUP_DESTROY);
pub const DRM_IOCTL_PANTHOR_GROUP_SUBMIT: u32 =
    DRM_IOWR::<drm_panthor_group_submit>(DRM_COMMAND_BASE + DRM_PANTHOR_GROUP_SUBMIT);
pub const DRM_IOCTL_PANTHOR_GROUP_GET_STATE: u32 =
    DRM_IOWR::<drm_panthor_group_get_state>(DRM_COMMAND_BASE + DRM_PANTHOR_GROUP_GET_STATE);
pub const DRM_IOCTL_PANTHOR_TILER_HEAP_CREATE: u32 =
    DRM_IOWR::<drm_panthor_tiler_heap_create>(DRM_COMMAND_BASE + DRM_PANTHOR_TILER_HEAP_CREATE);
pub const DRM_IOCTL_PANTHOR_TILER_HEAP_DESTROY: u32 =
    DRM_IOWR::<drm_panthor_tiler_heap_destroy>(DRM_COMMAND_BASE + DRM_PANTHOR_TILER_HEAP_DESTROY);
pub const DRM_IOCTL_PANTHOR_BO_SET_LABEL: u32 =
    DRM_IOWR::<drm_panthor_bo_set_label>(DRM_COMMAND_BASE + DRM_PANTHOR_BO_SET_LABEL);
pub const DRM_IOCTL_PANTHOR_SET_USER_MMIO_OFFSET: u32 = DRM_IOWR::<drm_panthor_set_user_mmio_offset>(
    DRM_COMMAND_BASE + DRM_PANTHOR_SET_USER_MMIO_OFFSET,
);
pub const DRM_IOCTL_PANTHOR_BO_SYNC: u32 =
    DRM_IOWR::<drm_panthor_bo_sync>(DRM_COMMAND_BASE + DRM_PANTHOR_BO_SYNC);
pub const DRM_IOCTL_PANTHOR_BO_QUERY_INFO: u32 =
    DRM_IOWR::<drm_panthor_bo_query_info>(DRM_COMMAND_BASE + DRM_PANTHOR_BO_QUERY_INFO);
