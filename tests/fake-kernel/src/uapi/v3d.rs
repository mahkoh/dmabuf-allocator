use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IOW;
use crate::uapi::drm::DRM_IOWR;

pub const DRM_V3D_SUBMIT_CL: u32 = 0x00;
pub const DRM_V3D_WAIT_BO: u32 = 0x01;
pub const DRM_V3D_CREATE_BO: u32 = 0x02;
pub const DRM_V3D_MMAP_BO: u32 = 0x03;
pub const DRM_V3D_GET_PARAM: u32 = 0x04;
pub const DRM_V3D_GET_BO_OFFSET: u32 = 0x05;
pub const DRM_V3D_SUBMIT_TFU: u32 = 0x06;
pub const DRM_V3D_SUBMIT_CSD: u32 = 0x07;
pub const DRM_V3D_PERFMON_CREATE: u32 = 0x08;
pub const DRM_V3D_PERFMON_DESTROY: u32 = 0x09;
pub const DRM_V3D_PERFMON_GET_VALUES: u32 = 0x0a;
pub const DRM_V3D_SUBMIT_CPU: u32 = 0x0b;
pub const DRM_V3D_PERFMON_GET_COUNTER: u32 = 0x0c;
pub const DRM_V3D_PERFMON_SET_GLOBAL: u32 = 0x0d;

pub const DRM_IOCTL_V3D_SUBMIT_CL: u32 =
    DRM_IOWR::<drm_v3d_submit_cl>(DRM_COMMAND_BASE + DRM_V3D_SUBMIT_CL);
pub const DRM_IOCTL_V3D_WAIT_BO: u32 =
    DRM_IOWR::<drm_v3d_wait_bo>(DRM_COMMAND_BASE + DRM_V3D_WAIT_BO);
pub const DRM_IOCTL_V3D_CREATE_BO: u32 =
    DRM_IOWR::<drm_v3d_create_bo>(DRM_COMMAND_BASE + DRM_V3D_CREATE_BO);
pub const DRM_IOCTL_V3D_MMAP_BO: u32 =
    DRM_IOWR::<drm_v3d_mmap_bo>(DRM_COMMAND_BASE + DRM_V3D_MMAP_BO);
pub const DRM_IOCTL_V3D_GET_PARAM: u32 =
    DRM_IOWR::<drm_v3d_get_param>(DRM_COMMAND_BASE + DRM_V3D_GET_PARAM);
pub const DRM_IOCTL_V3D_GET_BO_OFFSET: u32 =
    DRM_IOWR::<drm_v3d_get_bo_offset>(DRM_COMMAND_BASE + DRM_V3D_GET_BO_OFFSET);
pub const DRM_IOCTL_V3D_SUBMIT_TFU: u32 =
    DRM_IOW::<drm_v3d_submit_tfu>(DRM_COMMAND_BASE + DRM_V3D_SUBMIT_TFU);
pub const DRM_IOCTL_V3D_SUBMIT_CSD: u32 =
    DRM_IOW::<drm_v3d_submit_csd>(DRM_COMMAND_BASE + DRM_V3D_SUBMIT_CSD);
pub const DRM_IOCTL_V3D_PERFMON_CREATE: u32 =
    DRM_IOWR::<drm_v3d_perfmon_create>(DRM_COMMAND_BASE + DRM_V3D_PERFMON_CREATE);
pub const DRM_IOCTL_V3D_PERFMON_DESTROY: u32 =
    DRM_IOWR::<drm_v3d_perfmon_destroy>(DRM_COMMAND_BASE + DRM_V3D_PERFMON_DESTROY);
pub const DRM_IOCTL_V3D_PERFMON_GET_VALUES: u32 =
    DRM_IOWR::<drm_v3d_perfmon_get_values>(DRM_COMMAND_BASE + DRM_V3D_PERFMON_GET_VALUES);
pub const DRM_IOCTL_V3D_SUBMIT_CPU: u32 =
    DRM_IOW::<drm_v3d_submit_cpu>(DRM_COMMAND_BASE + DRM_V3D_SUBMIT_CPU);
pub const DRM_IOCTL_V3D_PERFMON_GET_COUNTER: u32 =
    DRM_IOWR::<drm_v3d_perfmon_get_counter>(DRM_COMMAND_BASE + DRM_V3D_PERFMON_GET_COUNTER);
pub const DRM_IOCTL_V3D_PERFMON_SET_GLOBAL: u32 =
    DRM_IOW::<drm_v3d_perfmon_set_global>(DRM_COMMAND_BASE + DRM_V3D_PERFMON_SET_GLOBAL);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_submit_cl {
    pub bcl_start: u32,
    pub bcl_end: u32,
    pub rcl_start: u32,
    pub rcl_end: u32,
    pub in_sync_bcl: u32,
    pub in_sync_rcl: u32,
    pub out_sync: u32,
    pub qma: u32,
    pub qms: u32,
    pub qts: u32,
    pub bo_handles: u64,
    pub bo_handle_count: u32,
    pub flags: u32,
    pub perfmon_id: u32,
    pub pad: u32,
    pub extensions: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_wait_bo {
    pub handle: u32,
    pub pad: u32,
    pub timeout_ns: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_v3d_create_bo {
    pub size: u32,
    pub flags: u32,
    pub handle: u32,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_mmap_bo {
    pub handle: u32,
    pub flags: u32,
    pub offset: u64,
}

pub const DRM_V3D_PARAM_V3D_UIFCFG: u32 = 0;
pub const DRM_V3D_PARAM_V3D_HUB_IDENT1: u32 = 1;
pub const DRM_V3D_PARAM_V3D_HUB_IDENT2: u32 = 2;
pub const DRM_V3D_PARAM_V3D_HUB_IDENT3: u32 = 3;
pub const DRM_V3D_PARAM_V3D_CORE0_IDENT0: u32 = 4;
pub const DRM_V3D_PARAM_V3D_CORE0_IDENT1: u32 = 5;
pub const DRM_V3D_PARAM_V3D_CORE0_IDENT2: u32 = 6;
pub const DRM_V3D_PARAM_SUPPORTS_TFU: u32 = 7;
pub const DRM_V3D_PARAM_SUPPORTS_CSD: u32 = 8;
pub const DRM_V3D_PARAM_SUPPORTS_CACHE_FLUSH: u32 = 9;
pub const DRM_V3D_PARAM_SUPPORTS_PERFMON: u32 = 10;
pub const DRM_V3D_PARAM_SUPPORTS_MULTISYNC_EXT: u32 = 11;
pub const DRM_V3D_PARAM_SUPPORTS_CPU_QUEUE: u32 = 12;
pub const DRM_V3D_PARAM_MAX_PERF_COUNTERS: u32 = 13;
pub const DRM_V3D_PARAM_SUPPORTS_SUPER_PAGES: u32 = 14;
pub const DRM_V3D_PARAM_GLOBAL_RESET_COUNTER: u32 = 15;
pub const DRM_V3D_PARAM_CONTEXT_RESET_COUNTER: u32 = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_get_param {
    pub param: u32,
    pub pad: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_get_bo_offset {
    pub handle: u32,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_submit_tfu__v71 {
    pub ioc: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_submit_tfu {
    pub icfg: u32,
    pub iia: u32,
    pub iis: u32,
    pub ica: u32,
    pub iua: u32,
    pub ioa: u32,
    pub ios: u32,
    pub coef: [u32; 4],
    pub bo_handles: [u32; 4],
    pub in_sync: u32,
    pub out_sync: u32,
    pub flags: u32,
    pub extensions: u64,
    pub v71: drm_v3d_submit_tfu__v71,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_submit_csd {
    pub cfg: [u32; 7],
    pub coef: [u32; 4],
    pub bo_handles: u64,
    pub bo_handle_count: u32,
    pub in_sync: u32,
    pub out_sync: u32,
    pub perfmon_id: u32,
    pub extensions: u64,
    pub flags: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_submit_cpu {
    pub bo_handles: u64,
    pub bo_handle_count: u32,
    pub flags: u32,
    pub extensions: u64,
}

pub const DRM_V3D_MAX_PERF_COUNTERS: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_perfmon_create {
    pub id: u32,
    pub ncounters: u32,
    pub counters: [u8; DRM_V3D_MAX_PERF_COUNTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_perfmon_destroy {
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_perfmon_get_values {
    pub id: u32,
    pub pad: u32,
    pub values_ptr: u64,
}

pub const DRM_V3D_PERFCNT_MAX_NAME: usize = 64;
pub const DRM_V3D_PERFCNT_MAX_CATEGORY: usize = 32;
pub const DRM_V3D_PERFCNT_MAX_DESCRIPTION: usize = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_perfmon_get_counter {
    pub counter: u8,
    pub name: [u8; DRM_V3D_PERFCNT_MAX_NAME],
    pub category: [u8; DRM_V3D_PERFCNT_MAX_CATEGORY],
    pub description: [u8; DRM_V3D_PERFCNT_MAX_DESCRIPTION],
    pub reserved: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_perfmon_set_global {
    pub flags: u32,
    pub id: u32,
}
