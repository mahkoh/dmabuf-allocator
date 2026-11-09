use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IOW;
use crate::uapi::drm::DRM_IOWR;

pub const DRM_ASAHI_GET_PARAMS: u32 = 0x00;
pub const DRM_ASAHI_GET_TIME: u32 = 0x01;
pub const DRM_ASAHI_VM_CREATE: u32 = 0x02;
pub const DRM_ASAHI_VM_DESTROY: u32 = 0x03;
pub const DRM_ASAHI_VM_BIND: u32 = 0x04;
pub const DRM_ASAHI_GEM_CREATE: u32 = 0x05;
pub const DRM_ASAHI_GEM_MMAP_OFFSET: u32 = 0x06;
pub const DRM_ASAHI_GEM_BIND_OBJECT: u32 = 0x07;
pub const DRM_ASAHI_QUEUE_CREATE: u32 = 0x08;
pub const DRM_ASAHI_QUEUE_DESTROY: u32 = 0x09;
pub const DRM_ASAHI_SUBMIT: u32 = 0x0a;

pub const DRM_ASAHI_MAX_CLUSTERS: usize = 64;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_asahi_params_global {
    pub features: u64,
    pub gpu_generation: u32,
    pub gpu_variant: u32,
    pub gpu_revision: u32,
    pub chip_id: u32,
    pub num_dies: u32,
    pub num_clusters_total: u32,
    pub num_cores_per_cluster: u32,
    pub max_frequency_khz: u32,
    pub core_masks: [u64; DRM_ASAHI_MAX_CLUSTERS],
    pub vm_start: u64,
    pub vm_end: u64,
    pub vm_kernel_min_size: u64,
    pub max_commands_per_submission: u32,
    pub max_attachments: u32,
    pub command_timestamp_frequency_hz: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_asahi_get_params {
    pub param_group: u32,
    pub pad: u32,
    pub pointer: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_asahi_vm_create {
    pub kernel_start: u64,
    pub kernel_end: u64,
    pub vm_id: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_asahi_vm_destroy {
    pub vm_id: u32,
    pub pad: u32,
}

pub const DRM_ASAHI_GEM_WRITEBACK: u32 = 1 << 0;
pub const DRM_ASAHI_GEM_VM_PRIVATE: u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_asahi_gem_create {
    pub size: u64,
    pub flags: u32,
    pub vm_id: u32,
    pub handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_asahi_gem_mmap_offset {
    pub handle: u32,
    pub flags: u32,
    pub offset: u64,
}

pub const DRM_ASAHI_BIND_UNBIND: u32 = 1 << 0;
pub const DRM_ASAHI_BIND_READ: u32 = 1 << 1;
pub const DRM_ASAHI_BIND_WRITE: u32 = 1 << 2;
pub const DRM_ASAHI_BIND_SINGLE_PAGE: u32 = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_asahi_gem_bind_op {
    pub flags: u32,
    pub handle: u32,
    pub offset: u64,
    pub range: u64,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_asahi_vm_bind {
    pub vm_id: u32,
    pub num_binds: u32,
    pub stride: u32,
    pub pad: u32,
    pub userptr: u64,
}

pub const DRM_ASAHI_BIND_OBJECT_OP_BIND: u32 = 0;
pub const DRM_ASAHI_BIND_OBJECT_OP_UNBIND: u32 = 1;

pub const DRM_ASAHI_BIND_OBJECT_USAGE_TIMESTAMPS: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_asahi_gem_bind_object {
    pub op: u32,
    pub flags: u32,
    pub handle: u32,
    pub vm_id: u32,
    pub offset: u64,
    pub range: u64,
    pub object_handle: u32,
    pub pad: u32,
}

pub const DRM_ASAHI_PRIORITY_LOW: u32 = 0;
pub const DRM_ASAHI_PRIORITY_MEDIUM: u32 = 1;
pub const DRM_ASAHI_PRIORITY_HIGH: u32 = 2;
pub const DRM_ASAHI_PRIORITY_REALTIME: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_asahi_queue_create {
    pub flags: u32,
    pub vm_id: u32,
    pub priority: u32,
    pub queue_id: u32,
    pub usc_exec_base: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_asahi_queue_destroy {
    pub queue_id: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_asahi_submit {
    pub syncs: u64,
    pub cmdbuf: u64,
    pub flags: u32,
    pub queue_id: u32,
    pub in_sync_count: u32,
    pub out_sync_count: u32,
    pub cmdbuf_size: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_asahi_get_time {
    pub flags: u64,
    pub gpu_timestamp: u64,
}

pub const DRM_IOCTL_ASAHI_GET_PARAMS: u32 =
    DRM_IOW::<drm_asahi_get_params>(DRM_COMMAND_BASE + DRM_ASAHI_GET_PARAMS);
pub const DRM_IOCTL_ASAHI_GET_TIME: u32 =
    DRM_IOWR::<drm_asahi_get_time>(DRM_COMMAND_BASE + DRM_ASAHI_GET_TIME);
pub const DRM_IOCTL_ASAHI_VM_CREATE: u32 =
    DRM_IOWR::<drm_asahi_vm_create>(DRM_COMMAND_BASE + DRM_ASAHI_VM_CREATE);
pub const DRM_IOCTL_ASAHI_VM_DESTROY: u32 =
    DRM_IOW::<drm_asahi_vm_destroy>(DRM_COMMAND_BASE + DRM_ASAHI_VM_DESTROY);
pub const DRM_IOCTL_ASAHI_VM_BIND: u32 =
    DRM_IOW::<drm_asahi_vm_bind>(DRM_COMMAND_BASE + DRM_ASAHI_VM_BIND);
pub const DRM_IOCTL_ASAHI_GEM_CREATE: u32 =
    DRM_IOWR::<drm_asahi_gem_create>(DRM_COMMAND_BASE + DRM_ASAHI_GEM_CREATE);
pub const DRM_IOCTL_ASAHI_GEM_MMAP_OFFSET: u32 =
    DRM_IOWR::<drm_asahi_gem_mmap_offset>(DRM_COMMAND_BASE + DRM_ASAHI_GEM_MMAP_OFFSET);
pub const DRM_IOCTL_ASAHI_GEM_BIND_OBJECT: u32 =
    DRM_IOWR::<drm_asahi_gem_bind_object>(DRM_COMMAND_BASE + DRM_ASAHI_GEM_BIND_OBJECT);
pub const DRM_IOCTL_ASAHI_QUEUE_CREATE: u32 =
    DRM_IOWR::<drm_asahi_queue_create>(DRM_COMMAND_BASE + DRM_ASAHI_QUEUE_CREATE);
pub const DRM_IOCTL_ASAHI_QUEUE_DESTROY: u32 =
    DRM_IOW::<drm_asahi_queue_destroy>(DRM_COMMAND_BASE + DRM_ASAHI_QUEUE_DESTROY);
pub const DRM_IOCTL_ASAHI_SUBMIT: u32 =
    DRM_IOW::<drm_asahi_submit>(DRM_COMMAND_BASE + DRM_ASAHI_SUBMIT);
