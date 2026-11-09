use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IOW;
use crate::uapi::drm::DRM_IOWR;

pub const DRM_PANFROST_SUBMIT: u32 = 0x00;
pub const DRM_PANFROST_WAIT_BO: u32 = 0x01;
pub const DRM_PANFROST_CREATE_BO: u32 = 0x02;
pub const DRM_PANFROST_MMAP_BO: u32 = 0x03;
pub const DRM_PANFROST_GET_PARAM: u32 = 0x04;
pub const DRM_PANFROST_GET_BO_OFFSET: u32 = 0x05;
pub const DRM_PANFROST_PERFCNT_ENABLE: u32 = 0x06;
pub const DRM_PANFROST_PERFCNT_DUMP: u32 = 0x07;
pub const DRM_PANFROST_MADVISE: u32 = 0x08;
pub const DRM_PANFROST_SET_LABEL_BO: u32 = 0x09;
pub const DRM_PANFROST_JM_CTX_CREATE: u32 = 0x0a;
pub const DRM_PANFROST_JM_CTX_DESTROY: u32 = 0x0b;
pub const DRM_PANFROST_SYNC_BO: u32 = 0x0c;
pub const DRM_PANFROST_QUERY_BO_INFO: u32 = 0x0d;

pub const DRM_IOCTL_PANFROST_SUBMIT: u32 =
    DRM_IOW::<drm_panfrost_submit>(DRM_COMMAND_BASE + DRM_PANFROST_SUBMIT);
pub const DRM_IOCTL_PANFROST_WAIT_BO: u32 =
    DRM_IOW::<drm_panfrost_wait_bo>(DRM_COMMAND_BASE + DRM_PANFROST_WAIT_BO);
pub const DRM_IOCTL_PANFROST_CREATE_BO: u32 =
    DRM_IOWR::<drm_panfrost_create_bo>(DRM_COMMAND_BASE + DRM_PANFROST_CREATE_BO);
pub const DRM_IOCTL_PANFROST_MMAP_BO: u32 =
    DRM_IOWR::<drm_panfrost_mmap_bo>(DRM_COMMAND_BASE + DRM_PANFROST_MMAP_BO);
pub const DRM_IOCTL_PANFROST_GET_PARAM: u32 =
    DRM_IOWR::<drm_panfrost_get_param>(DRM_COMMAND_BASE + DRM_PANFROST_GET_PARAM);
pub const DRM_IOCTL_PANFROST_GET_BO_OFFSET: u32 =
    DRM_IOWR::<drm_panfrost_get_bo_offset>(DRM_COMMAND_BASE + DRM_PANFROST_GET_BO_OFFSET);
pub const DRM_IOCTL_PANFROST_MADVISE: u32 =
    DRM_IOWR::<drm_panfrost_madvise>(DRM_COMMAND_BASE + DRM_PANFROST_MADVISE);
pub const DRM_IOCTL_PANFROST_SET_LABEL_BO: u32 =
    DRM_IOWR::<drm_panfrost_set_label_bo>(DRM_COMMAND_BASE + DRM_PANFROST_SET_LABEL_BO);
pub const DRM_IOCTL_PANFROST_JM_CTX_CREATE: u32 =
    DRM_IOWR::<drm_panfrost_jm_ctx_create>(DRM_COMMAND_BASE + DRM_PANFROST_JM_CTX_CREATE);
pub const DRM_IOCTL_PANFROST_JM_CTX_DESTROY: u32 =
    DRM_IOWR::<drm_panfrost_jm_ctx_destroy>(DRM_COMMAND_BASE + DRM_PANFROST_JM_CTX_DESTROY);
pub const DRM_IOCTL_PANFROST_SYNC_BO: u32 =
    DRM_IOWR::<drm_panfrost_sync_bo>(DRM_COMMAND_BASE + DRM_PANFROST_SYNC_BO);
pub const DRM_IOCTL_PANFROST_QUERY_BO_INFO: u32 =
    DRM_IOWR::<drm_panfrost_query_bo_info>(DRM_COMMAND_BASE + DRM_PANFROST_QUERY_BO_INFO);

pub const DRM_IOCTL_PANFROST_PERFCNT_ENABLE: u32 =
    DRM_IOW::<drm_panfrost_perfcnt_enable>(DRM_COMMAND_BASE + DRM_PANFROST_PERFCNT_ENABLE);
pub const DRM_IOCTL_PANFROST_PERFCNT_DUMP: u32 =
    DRM_IOW::<drm_panfrost_perfcnt_dump>(DRM_COMMAND_BASE + DRM_PANFROST_PERFCNT_DUMP);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_submit {
    pub jc: u64,
    pub in_syncs: u64,
    pub in_sync_count: u32,
    pub out_sync: u32,
    pub bo_handles: u64,
    pub bo_handle_count: u32,
    pub requirements: u32,
    pub jm_ctx_handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_wait_bo {
    pub handle: u32,
    pub pad: u32,
    pub timeout_ns: i64,
}

pub const PANFROST_BO_NOEXEC: u32 = 1;
pub const PANFROST_BO_HEAP: u32 = 2;
pub const PANFROST_BO_WB_MMAP: u32 = 4;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_panfrost_create_bo {
    pub size: u32,
    pub flags: u32,
    pub handle: u32,
    pub pad: u32,
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_mmap_bo {
    pub handle: u32,
    pub flags: u32,
    pub offset: u64,
}

pub const DRM_PANFROST_PARAM_GPU_PROD_ID: u32 = 0;
pub const DRM_PANFROST_PARAM_GPU_REVISION: u32 = 1;
pub const DRM_PANFROST_PARAM_SHADER_PRESENT: u32 = 2;
pub const DRM_PANFROST_PARAM_L2_FEATURES: u32 = 8;
pub const DRM_PANFROST_PARAM_TILER_FEATURES: u32 = 10;
pub const DRM_PANFROST_PARAM_MEM_FEATURES: u32 = 11;
pub const DRM_PANFROST_PARAM_MMU_FEATURES: u32 = 12;
pub const DRM_PANFROST_PARAM_THREAD_FEATURES: u32 = 13;
pub const DRM_PANFROST_PARAM_MAX_THREADS: u32 = 14;
pub const DRM_PANFROST_PARAM_THREAD_MAX_WORKGROUP_SZ: u32 = 15;
pub const DRM_PANFROST_PARAM_TEXTURE_FEATURES0: u32 = 18;
pub const DRM_PANFROST_PARAM_TEXTURE_FEATURES3: u32 = 21;
pub const DRM_PANFROST_PARAM_THREAD_TLS_ALLOC: u32 = 39;
pub const DRM_PANFROST_PARAM_AFBC_FEATURES: u32 = 40;
pub const DRM_PANFROST_PARAM_SYSTEM_TIMESTAMP: u32 = 41;
pub const DRM_PANFROST_PARAM_SYSTEM_TIMESTAMP_FREQUENCY: u32 = 42;
pub const DRM_PANFROST_PARAM_ALLOWED_JM_CTX_PRIORITIES: u32 = 43;
pub const DRM_PANFROST_PARAM_SELECTED_COHERENCY: u32 = 44;

pub const DRM_PANFROST_GPU_COHERENCY_NONE: u64 = 31;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_get_param {
    pub param: u32,
    pub pad: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_get_bo_offset {
    pub handle: u32,
    pub pad: u32,
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_perfcnt_enable {
    pub enable: u32,
    pub counterset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_perfcnt_dump {
    pub buf_ptr: u64,
}

pub const PANFROST_MADV_WILLNEED: u32 = 0;
pub const PANFROST_MADV_DONTNEED: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_madvise {
    pub handle: u32,
    pub madv: u32,
    pub retained: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_set_label_bo {
    pub handle: u32,
    pub pad: u32,
    pub label: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_sync_bo {
    pub ops: u64,
    pub op_count: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_query_bo_info {
    pub handle: u32,
    pub extra_flags: u32,
    pub create_flags: u32,
    pub pad: u32,
}

pub const PANFROST_JM_CTX_PRIORITY_LOW: u32 = 0;
pub const PANFROST_JM_CTX_PRIORITY_MEDIUM: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_jm_ctx_create {
    pub handle: u32,
    pub priority: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panfrost_jm_ctx_destroy {
    pub handle: u32,
    pub pad: u32,
}
