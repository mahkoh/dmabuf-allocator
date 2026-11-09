use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IOW;
use crate::uapi::drm::DRM_IOWR;

pub const DRM_XE_DEVICE_QUERY: u32 = 0x00;
pub const DRM_XE_GEM_CREATE: u32 = 0x01;
pub const DRM_XE_GEM_MMAP_OFFSET: u32 = 0x02;
pub const DRM_XE_VM_CREATE: u32 = 0x03;
pub const DRM_XE_VM_DESTROY: u32 = 0x04;
pub const DRM_XE_VM_BIND: u32 = 0x05;
pub const DRM_XE_EXEC_QUEUE_CREATE: u32 = 0x06;
pub const DRM_XE_EXEC_QUEUE_DESTROY: u32 = 0x07;
pub const DRM_XE_EXEC_QUEUE_GET_PROPERTY: u32 = 0x08;
pub const DRM_XE_EXEC: u32 = 0x09;
pub const DRM_XE_WAIT_USER_FENCE: u32 = 0x0a;
pub const DRM_XE_OBSERVATION: u32 = 0x0b;
pub const DRM_XE_MADVISE: u32 = 0x0c;
pub const DRM_XE_VM_QUERY_MEM_RANGE_ATTRS: u32 = 0x0d;
pub const DRM_XE_EXEC_QUEUE_SET_PROPERTY: u32 = 0x0e;
pub const DRM_XE_VM_GET_PROPERTY: u32 = 0x0f;

pub const DRM_IOCTL_XE_DEVICE_QUERY: u32 =
    DRM_IOWR::<drm_xe_device_query>(DRM_COMMAND_BASE + DRM_XE_DEVICE_QUERY);
pub const DRM_IOCTL_XE_GEM_CREATE: u32 =
    DRM_IOWR::<drm_xe_gem_create>(DRM_COMMAND_BASE + DRM_XE_GEM_CREATE);
pub const DRM_IOCTL_XE_GEM_MMAP_OFFSET: u32 =
    DRM_IOWR::<drm_xe_gem_mmap_offset>(DRM_COMMAND_BASE + DRM_XE_GEM_MMAP_OFFSET);
pub const DRM_IOCTL_XE_VM_CREATE: u32 =
    DRM_IOWR::<drm_xe_vm_create>(DRM_COMMAND_BASE + DRM_XE_VM_CREATE);
pub const DRM_IOCTL_XE_VM_DESTROY: u32 =
    DRM_IOW::<drm_xe_vm_destroy>(DRM_COMMAND_BASE + DRM_XE_VM_DESTROY);
pub const DRM_IOCTL_XE_VM_BIND: u32 = DRM_IOW::<drm_xe_vm_bind>(DRM_COMMAND_BASE + DRM_XE_VM_BIND);
pub const DRM_IOCTL_XE_EXEC_QUEUE_CREATE: u32 =
    DRM_IOWR::<drm_xe_exec_queue_create>(DRM_COMMAND_BASE + DRM_XE_EXEC_QUEUE_CREATE);
pub const DRM_IOCTL_XE_EXEC_QUEUE_DESTROY: u32 =
    DRM_IOW::<drm_xe_exec_queue_destroy>(DRM_COMMAND_BASE + DRM_XE_EXEC_QUEUE_DESTROY);
pub const DRM_IOCTL_XE_EXEC_QUEUE_GET_PROPERTY: u32 =
    DRM_IOWR::<drm_xe_exec_queue_get_property>(DRM_COMMAND_BASE + DRM_XE_EXEC_QUEUE_GET_PROPERTY);
pub const DRM_IOCTL_XE_EXEC: u32 = DRM_IOW::<drm_xe_exec>(DRM_COMMAND_BASE + DRM_XE_EXEC);
pub const DRM_IOCTL_XE_WAIT_USER_FENCE: u32 =
    DRM_IOWR::<drm_xe_wait_user_fence>(DRM_COMMAND_BASE + DRM_XE_WAIT_USER_FENCE);
pub const DRM_IOCTL_XE_OBSERVATION: u32 =
    DRM_IOW::<drm_xe_observation_param>(DRM_COMMAND_BASE + DRM_XE_OBSERVATION);
pub const DRM_IOCTL_XE_MADVISE: u32 = DRM_IOW::<drm_xe_madvise>(DRM_COMMAND_BASE + DRM_XE_MADVISE);
pub const DRM_IOCTL_XE_VM_QUERY_MEM_RANGE_ATTRS: u32 =
    DRM_IOWR::<drm_xe_vm_query_mem_range_attr>(DRM_COMMAND_BASE + DRM_XE_VM_QUERY_MEM_RANGE_ATTRS);
pub const DRM_IOCTL_XE_EXEC_QUEUE_SET_PROPERTY: u32 =
    DRM_IOW::<drm_xe_exec_queue_set_property>(DRM_COMMAND_BASE + DRM_XE_EXEC_QUEUE_SET_PROPERTY);
pub const DRM_IOCTL_XE_VM_GET_PROPERTY: u32 =
    DRM_IOWR::<drm_xe_vm_get_property>(DRM_COMMAND_BASE + DRM_XE_VM_GET_PROPERTY);

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_xe_user_extension {
    pub next_extension: u64,
    pub name: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub union drm_xe_ext_set_property__anon {
    pub value: u64,
    pub ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_xe_ext_set_property {
    pub base: drm_xe_user_extension,
    pub property: u32,
    pub pad: u32,
    pub anon: drm_xe_ext_set_property__anon,
    pub reserved: [u64; 2],
}

pub const DRM_XE_ENGINE_CLASS_RENDER: u16 = 0;
pub const DRM_XE_ENGINE_CLASS_COPY: u16 = 1;
pub const DRM_XE_ENGINE_CLASS_VIDEO_DECODE: u16 = 2;
pub const DRM_XE_ENGINE_CLASS_VIDEO_ENHANCE: u16 = 3;
pub const DRM_XE_ENGINE_CLASS_COMPUTE: u16 = 4;
pub const DRM_XE_ENGINE_CLASS_VM_BIND: u16 = 5;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_xe_engine_class_instance {
    pub engine_class: u16,
    pub engine_instance: u16,
    pub gt_id: u16,
    pub pad: u16,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_xe_engine {
    pub instance: drm_xe_engine_class_instance,
    pub reserved: [u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_xe_query_engines {
    pub num_engines: u32,
    pub pad: u32,
    pub engines: [drm_xe_engine; 0],
}

pub const DRM_XE_MEM_REGION_CLASS_SYSMEM: u16 = 0;
pub const DRM_XE_MEM_REGION_CLASS_VRAM: u16 = 1;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_xe_mem_region {
    pub mem_class: u16,
    pub instance: u16,
    pub min_page_size: u32,
    pub total_size: u64,
    pub used: u64,
    pub cpu_visible_size: u64,
    pub cpu_visible_used: u64,
    pub reserved: [u64; 6],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_xe_query_mem_regions {
    pub num_mem_regions: u32,
    pub pad: u32,
    pub mem_regions: [drm_xe_mem_region; 0],
}

pub const DRM_XE_QUERY_CONFIG_REV_AND_DEVICE_ID: usize = 0;
pub const DRM_XE_QUERY_CONFIG_FLAGS: usize = 1;
pub const DRM_XE_QUERY_CONFIG_FLAG_HAS_VRAM: u64 = 1 << 0;
pub const DRM_XE_QUERY_CONFIG_FLAG_HAS_LOW_LATENCY: u64 = 1 << 1;
pub const DRM_XE_QUERY_CONFIG_FLAG_HAS_CPU_ADDR_MIRROR: u64 = 1 << 2;
pub const DRM_XE_QUERY_CONFIG_FLAG_HAS_NO_COMPRESSION_HINT: u64 = 1 << 3;
pub const DRM_XE_QUERY_CONFIG_FLAG_HAS_DISABLE_STATE_CACHE_PERF_FIX: u64 = 1 << 4;
pub const DRM_XE_QUERY_CONFIG_FLAG_HAS_PURGING_SUPPORT: u64 = 1 << 5;
pub const DRM_XE_QUERY_CONFIG_MIN_ALIGNMENT: usize = 2;
pub const DRM_XE_QUERY_CONFIG_VA_BITS: usize = 3;
pub const DRM_XE_QUERY_CONFIG_MAX_EXEC_QUEUE_PRIORITY: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_xe_query_config {
    pub num_params: u32,
    pub pad: u32,
    pub info: [u64; 0],
}

pub const DRM_XE_QUERY_GT_TYPE_MAIN: u16 = 0;
pub const DRM_XE_QUERY_GT_TYPE_MEDIA: u16 = 1;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_xe_gt {
    pub type_: u16,
    pub tile_id: u16,
    pub gt_id: u16,
    pub pad: [u16; 3],
    pub reference_clock: u32,
    pub near_mem_regions: u64,
    pub far_mem_regions: u64,
    pub ip_ver_major: u16,
    pub ip_ver_minor: u16,
    pub ip_ver_rev: u16,
    pub pad2: u16,
    pub reserved: [u64; 7],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_xe_query_gt_list {
    pub num_gt: u32,
    pub pad: u32,
    pub gt_list: [drm_xe_gt; 0],
}

pub const DRM_XE_TOPO_DSS_GEOMETRY: u16 = 1;
pub const DRM_XE_TOPO_DSS_COMPUTE: u16 = 2;
pub const DRM_XE_TOPO_L3_BANK: u16 = 3;
pub const DRM_XE_TOPO_EU_PER_DSS: u16 = 4;
pub const DRM_XE_TOPO_SIMD16_EU_PER_DSS: u16 = 5;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_xe_query_topology_mask {
    pub gt_id: u16,
    pub type_: u16,
    pub num_bytes: u32,
    pub mask: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_xe_query_engine_cycles {
    pub eci: drm_xe_engine_class_instance,
    pub clockid: i32,
    pub width: u32,
    pub engine_cycles: u64,
    pub cpu_timestamp: u64,
    pub cpu_delta: u64,
}

pub const XE_QUERY_UC_TYPE_GUC_SUBMISSION: u16 = 0;
pub const XE_QUERY_UC_TYPE_HUC: u16 = 1;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_xe_query_uc_fw_version {
    pub uc_type: u16,
    pub pad: u16,
    pub branch_ver: u32,
    pub major_ver: u32,
    pub minor_ver: u32,
    pub patch_ver: u32,
    pub pad2: u32,
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_query_pxp_status {
    pub status: u32,
    pub supported_session_types: u32,
}

pub const DRM_XE_DEVICE_QUERY_ENGINES: u32 = 0;
pub const DRM_XE_DEVICE_QUERY_MEM_REGIONS: u32 = 1;
pub const DRM_XE_DEVICE_QUERY_CONFIG: u32 = 2;
pub const DRM_XE_DEVICE_QUERY_GT_LIST: u32 = 3;
pub const DRM_XE_DEVICE_QUERY_HWCONFIG: u32 = 4;
pub const DRM_XE_DEVICE_QUERY_GT_TOPOLOGY: u32 = 5;
pub const DRM_XE_DEVICE_QUERY_ENGINE_CYCLES: u32 = 6;
pub const DRM_XE_DEVICE_QUERY_UC_FW_VERSION: u32 = 7;
pub const DRM_XE_DEVICE_QUERY_OA_UNITS: u32 = 8;
pub const DRM_XE_DEVICE_QUERY_PXP_STATUS: u32 = 9;
pub const DRM_XE_DEVICE_QUERY_EU_STALL: u32 = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_device_query {
    pub extensions: u64,
    pub query: u32,
    pub size: u32,
    pub data: u64,
    pub reserved: [u64; 2],
}

pub const DRM_XE_GEM_CREATE_FLAG_DEFER_BACKING: u32 = 1 << 0;
pub const DRM_XE_GEM_CREATE_FLAG_SCANOUT: u32 = 1 << 1;
pub const DRM_XE_GEM_CREATE_FLAG_NEEDS_VISIBLE_VRAM: u32 = 1 << 2;
pub const DRM_XE_GEM_CREATE_FLAG_NO_COMPRESSION: u32 = 1 << 3;

pub const DRM_XE_GEM_CPU_CACHING_WB: u16 = 1;
pub const DRM_XE_GEM_CPU_CACHING_WC: u16 = 2;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_xe_gem_create {
    pub extensions: u64,
    pub size: u64,
    pub placement: u32,
    pub flags: u32,
    pub vm_id: u32,
    pub handle: u32,
    pub cpu_caching: u16,
    pub pad: [u16; 3],
    pub reserved: [u64; 2],
}

pub const DRM_XE_MMAP_OFFSET_FLAG_PCI_BARRIER: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_gem_mmap_offset {
    pub extensions: u64,
    pub handle: u32,
    pub flags: u32,
    pub offset: u64,
    pub reserved: [u64; 2],
}

pub const DRM_XE_VM_CREATE_FLAG_SCRATCH_PAGE: u32 = 1 << 0;
pub const DRM_XE_VM_CREATE_FLAG_LR_MODE: u32 = 1 << 1;
pub const DRM_XE_VM_CREATE_FLAG_FAULT_MODE: u32 = 1 << 2;
pub const DRM_XE_VM_CREATE_FLAG_NO_VM_OVERCOMMIT: u32 = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_vm_create {
    pub extensions: u64,
    pub flags: u32,
    pub vm_id: u32,
    pub reserved: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_vm_destroy {
    pub vm_id: u32,
    pub pad: u32,
    pub reserved: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_xe_vm_bind_op__anon {
    pub obj_offset: u64,
    pub userptr: u64,
    pub cpu_addr_mirror_offset: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_vm_bind_op {
    pub extensions: u64,
    pub obj: u32,
    pub pat_index: u16,
    pub pad: u16,
    pub anon: drm_xe_vm_bind_op__anon,
    pub range: u64,
    pub addr: u64,
    pub op: u32,
    pub flags: u32,
    pub prefetch_mem_region_instance: u32,
    pub pad2: u32,
    pub reserved: [u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_xe_vm_bind__anon {
    pub bind: drm_xe_vm_bind_op,
    pub vector_of_binds: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_vm_bind {
    pub extensions: u64,
    pub vm_id: u32,
    pub exec_queue_id: u32,
    pub pad: u32,
    pub num_binds: u32,
    pub anon: drm_xe_vm_bind__anon,
    pub pad2: u32,
    pub num_syncs: u32,
    pub syncs: u64,
    pub reserved: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_xe_vm_get_property__anon {
    pub data: u64,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_vm_get_property {
    pub extensions: u64,
    pub vm_id: u32,
    pub property: u32,
    pub size: u32,
    pub pad: u32,
    pub anon: drm_xe_vm_get_property__anon,
    pub reserved: [u64; 3],
}

pub const DRM_XE_EXEC_QUEUE_EXTENSION_SET_PROPERTY: u32 = 0;
pub const DRM_XE_EXEC_QUEUE_SET_PROPERTY_PRIORITY: u32 = 0;
pub const DRM_XE_EXEC_QUEUE_SET_PROPERTY_TIMESLICE: u32 = 1;
pub const DRM_XE_EXEC_QUEUE_SET_PROPERTY_PXP_TYPE: u32 = 2;
pub const DRM_XE_EXEC_QUEUE_SET_HANG_REPLAY_STATE: u32 = 3;
pub const DRM_XE_EXEC_QUEUE_SET_PROPERTY_MULTI_GROUP: u32 = 4;
pub const DRM_XE_EXEC_QUEUE_SET_PROPERTY_MULTI_QUEUE_PRIORITY: u32 = 5;
pub const DRM_XE_EXEC_QUEUE_SET_DISABLE_STATE_CACHE_PERF_FIX: u32 = 6;

pub const DRM_XE_EXEC_QUEUE_LOW_LATENCY_HINT: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_exec_queue_create {
    pub extensions: u64,
    pub width: u16,
    pub num_placements: u16,
    pub vm_id: u32,
    pub flags: u32,
    pub exec_queue_id: u32,
    pub instances: u64,
    pub reserved: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_exec_queue_destroy {
    pub exec_queue_id: u32,
    pub pad: u32,
    pub reserved: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_exec_queue_get_property {
    pub extensions: u64,
    pub exec_queue_id: u32,
    pub property: u32,
    pub value: u64,
    pub reserved: [u64; 2],
}

pub const DRM_XE_SYNC_TYPE_SYNCOBJ: u32 = 0x0;
pub const DRM_XE_SYNC_TYPE_TIMELINE_SYNCOBJ: u32 = 0x1;
pub const DRM_XE_SYNC_TYPE_USER_FENCE: u32 = 0x2;

pub const DRM_XE_SYNC_FLAG_SIGNAL: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub union drm_xe_sync__anon {
    pub handle: u32,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_xe_sync {
    pub extensions: u64,
    pub type_: u32,
    pub flags: u32,
    pub anon: drm_xe_sync__anon,
    pub timeline_value: u64,
    pub reserved: [u64; 2],
}

pub const DRM_XE_MAX_SYNCS: u32 = 1024;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_exec {
    pub extensions: u64,
    pub exec_queue_id: u32,
    pub num_syncs: u32,
    pub syncs: u64,
    pub address: u64,
    pub num_batch_buffer: u16,
    pub pad: [u16; 3],
    pub reserved: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_wait_user_fence {
    pub extensions: u64,
    pub addr: u64,
    pub op: u16,
    pub flags: u16,
    pub pad: u32,
    pub value: u64,
    pub mask: u64,
    pub timeout: i64,
    pub exec_queue_id: u32,
    pub pad2: u32,
    pub reserved: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_observation_param {
    pub extensions: u64,
    pub observation_type: u64,
    pub observation_op: u64,
    pub param: u64,
}

pub const DRM_XE_PXP_TYPE_HWDRM: u32 = 1;

pub const DRM_XE_VMA_ATTR_PURGEABLE_STATE: u32 = 3;

pub const DRM_XE_VMA_PURGEABLE_STATE_WILLNEED: u32 = 0;
pub const DRM_XE_VMA_PURGEABLE_STATE_DONTNEED: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_madvise__anon__preferred_mem_loc {
    pub devmem_fd: u32,
    pub migration_policy: u16,
    pub region_instance: u16,
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_madvise__anon__atomic {
    pub val: u32,
    pub pad: u32,
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_madvise__anon__pat_index {
    pub val: u32,
    pub pad: u32,
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_madvise__anon__purge_state_val {
    pub val: u32,
    pub pad: u32,
    pub retained_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_xe_madvise__anon {
    pub preferred_mem_loc: drm_xe_madvise__anon__preferred_mem_loc,
    pub atomic: drm_xe_madvise__anon__atomic,
    pub pat_index: drm_xe_madvise__anon__pat_index,
    pub purge_state_val: drm_xe_madvise__anon__purge_state_val,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_madvise {
    pub extensions: u64,
    pub start: u64,
    pub range: u64,
    pub vm_id: u32,
    pub type_: u32,
    pub anon: drm_xe_madvise__anon,
    pub reserved: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_vm_query_mem_range_attr {
    pub extensions: u64,
    pub vm_id: u32,
    pub num_mem_ranges: u32,
    pub start: u64,
    pub range: u64,
    pub sizeof_mem_range_attr: u64,
    pub vector_of_mem_attr: u64,
    pub reserved: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_xe_exec_queue_set_property {
    pub extensions: u64,
    pub exec_queue_id: u32,
    pub property: u32,
    pub value: u64,
    pub reserved: [u64; 2],
}
