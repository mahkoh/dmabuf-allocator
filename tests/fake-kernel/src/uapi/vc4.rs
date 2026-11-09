use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IOWR;

pub const DRM_VC4_SUBMIT_CL: u32 = 0x00;
pub const DRM_VC4_WAIT_SEQNO: u32 = 0x01;
pub const DRM_VC4_WAIT_BO: u32 = 0x02;
pub const DRM_VC4_CREATE_BO: u32 = 0x03;
pub const DRM_VC4_MMAP_BO: u32 = 0x04;
pub const DRM_VC4_CREATE_SHADER_BO: u32 = 0x05;
pub const DRM_VC4_GET_HANG_STATE: u32 = 0x06;
pub const DRM_VC4_GET_PARAM: u32 = 0x07;
pub const DRM_VC4_SET_TILING: u32 = 0x08;
pub const DRM_VC4_GET_TILING: u32 = 0x09;
pub const DRM_VC4_LABEL_BO: u32 = 0x0a;
pub const DRM_VC4_GEM_MADVISE: u32 = 0x0b;
pub const DRM_VC4_PERFMON_CREATE: u32 = 0x0c;
pub const DRM_VC4_PERFMON_DESTROY: u32 = 0x0d;
pub const DRM_VC4_PERFMON_GET_VALUES: u32 = 0x0e;

pub const DRM_IOCTL_VC4_SUBMIT_CL: u32 =
    DRM_IOWR::<drm_vc4_submit_cl>(DRM_COMMAND_BASE + DRM_VC4_SUBMIT_CL);
pub const DRM_IOCTL_VC4_WAIT_SEQNO: u32 =
    DRM_IOWR::<drm_vc4_wait_seqno>(DRM_COMMAND_BASE + DRM_VC4_WAIT_SEQNO);
pub const DRM_IOCTL_VC4_WAIT_BO: u32 =
    DRM_IOWR::<drm_vc4_wait_bo>(DRM_COMMAND_BASE + DRM_VC4_WAIT_BO);
pub const DRM_IOCTL_VC4_CREATE_BO: u32 =
    DRM_IOWR::<drm_vc4_create_bo>(DRM_COMMAND_BASE + DRM_VC4_CREATE_BO);
pub const DRM_IOCTL_VC4_MMAP_BO: u32 =
    DRM_IOWR::<drm_vc4_mmap_bo>(DRM_COMMAND_BASE + DRM_VC4_MMAP_BO);
pub const DRM_IOCTL_VC4_CREATE_SHADER_BO: u32 =
    DRM_IOWR::<drm_vc4_create_shader_bo>(DRM_COMMAND_BASE + DRM_VC4_CREATE_SHADER_BO);
pub const DRM_IOCTL_VC4_GET_HANG_STATE: u32 =
    DRM_IOWR::<drm_vc4_get_hang_state>(DRM_COMMAND_BASE + DRM_VC4_GET_HANG_STATE);
pub const DRM_IOCTL_VC4_GET_PARAM: u32 =
    DRM_IOWR::<drm_vc4_get_param>(DRM_COMMAND_BASE + DRM_VC4_GET_PARAM);
pub const DRM_IOCTL_VC4_SET_TILING: u32 =
    DRM_IOWR::<drm_vc4_set_tiling>(DRM_COMMAND_BASE + DRM_VC4_SET_TILING);
pub const DRM_IOCTL_VC4_GET_TILING: u32 =
    DRM_IOWR::<drm_vc4_get_tiling>(DRM_COMMAND_BASE + DRM_VC4_GET_TILING);
pub const DRM_IOCTL_VC4_LABEL_BO: u32 =
    DRM_IOWR::<drm_vc4_label_bo>(DRM_COMMAND_BASE + DRM_VC4_LABEL_BO);
pub const DRM_IOCTL_VC4_GEM_MADVISE: u32 =
    DRM_IOWR::<drm_vc4_gem_madvise>(DRM_COMMAND_BASE + DRM_VC4_GEM_MADVISE);
pub const DRM_IOCTL_VC4_PERFMON_CREATE: u32 =
    DRM_IOWR::<drm_vc4_perfmon_create>(DRM_COMMAND_BASE + DRM_VC4_PERFMON_CREATE);
pub const DRM_IOCTL_VC4_PERFMON_DESTROY: u32 =
    DRM_IOWR::<drm_vc4_perfmon_destroy>(DRM_COMMAND_BASE + DRM_VC4_PERFMON_DESTROY);
pub const DRM_IOCTL_VC4_PERFMON_GET_VALUES: u32 =
    DRM_IOWR::<drm_vc4_perfmon_get_values>(DRM_COMMAND_BASE + DRM_VC4_PERFMON_GET_VALUES);

pub const VC4_SUBMIT_RCL_SURFACE_READ_IS_FULL_RES: u16 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_submit_rcl_surface {
    pub hindex: u32,
    pub offset: u32,
    pub bits: u16,
    pub flags: u16,
}

pub const VC4_SUBMIT_CL_USE_CLEAR_COLOR: u32 = 1 << 0;
pub const VC4_SUBMIT_CL_FIXED_RCL_ORDER: u32 = 1 << 1;
pub const VC4_SUBMIT_CL_RCL_ORDER_INCREASING_X: u32 = 1 << 2;
pub const VC4_SUBMIT_CL_RCL_ORDER_INCREASING_Y: u32 = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_submit_cl {
    pub bin_cl: u64,
    pub shader_rec: u64,
    pub uniforms: u64,
    pub bo_handles: u64,
    pub bin_cl_size: u32,
    pub shader_rec_size: u32,
    pub shader_rec_count: u32,
    pub uniforms_size: u32,
    pub bo_handle_count: u32,
    pub width: u16,
    pub height: u16,
    pub min_x_tile: u8,
    pub min_y_tile: u8,
    pub max_x_tile: u8,
    pub max_y_tile: u8,
    pub color_read: drm_vc4_submit_rcl_surface,
    pub color_write: drm_vc4_submit_rcl_surface,
    pub zs_read: drm_vc4_submit_rcl_surface,
    pub zs_write: drm_vc4_submit_rcl_surface,
    pub msaa_color_write: drm_vc4_submit_rcl_surface,
    pub msaa_zs_write: drm_vc4_submit_rcl_surface,
    pub clear_color: [u32; 2],
    pub clear_z: u32,
    pub clear_s: u8,
    pub pad: [u8; 3],
    pub flags: u32,
    pub seqno: u64,
    pub perfmonid: u32,
    pub in_sync: u32,
    pub out_sync: u32,
    pub pad2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_wait_seqno {
    pub seqno: u64,
    pub timeout_ns: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_wait_bo {
    pub handle: u32,
    pub pad: u32,
    pub timeout_ns: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_vc4_create_bo {
    pub size: u32,
    pub flags: u32,
    pub handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_mmap_bo {
    pub handle: u32,
    pub flags: u32,
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_create_shader_bo {
    pub size: u32,
    pub flags: u32,
    pub data: u64,
    pub handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_get_hang_state_bo {
    pub handle: u32,
    pub paddr: u32,
    pub size: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_get_hang_state {
    pub bo: u64,
    pub bo_count: u32,
    pub start_bin: u32,
    pub start_render: u32,
    pub ct0ca: u32,
    pub ct0ea: u32,
    pub ct1ca: u32,
    pub ct1ea: u32,
    pub ct0cs: u32,
    pub ct1cs: u32,
    pub ct0ra0: u32,
    pub ct1ra0: u32,
    pub bpca: u32,
    pub bpcs: u32,
    pub bpoa: u32,
    pub bpos: u32,
    pub vpmbase: u32,
    pub dbge: u32,
    pub fdbgo: u32,
    pub fdbgb: u32,
    pub fdbgr: u32,
    pub fdbgs: u32,
    pub errstat: u32,
    pub pad: [u32; 16],
}

pub const DRM_VC4_PARAM_V3D_IDENT0: u32 = 0;
pub const DRM_VC4_PARAM_V3D_IDENT1: u32 = 1;
pub const DRM_VC4_PARAM_V3D_IDENT2: u32 = 2;
pub const DRM_VC4_PARAM_SUPPORTS_BRANCHES: u32 = 3;
pub const DRM_VC4_PARAM_SUPPORTS_ETC1: u32 = 4;
pub const DRM_VC4_PARAM_SUPPORTS_THREADED_FS: u32 = 5;
pub const DRM_VC4_PARAM_SUPPORTS_FIXED_RCL_ORDER: u32 = 6;
pub const DRM_VC4_PARAM_SUPPORTS_MADVISE: u32 = 7;
pub const DRM_VC4_PARAM_SUPPORTS_PERFMON: u32 = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_get_param {
    pub param: u32,
    pub pad: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_get_tiling {
    pub handle: u32,
    pub flags: u32,
    pub modifier: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_vc4_set_tiling {
    pub handle: u32,
    pub flags: u32,
    pub modifier: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_label_bo {
    pub handle: u32,
    pub len: u32,
    pub name: u64,
}

pub const VC4_MADV_WILLNEED: u32 = 0;
pub const VC4_MADV_DONTNEED: u32 = 1;
pub const __VC4_MADV_PURGED: u32 = 2;
pub const __VC4_MADV_NOTSUPP: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_gem_madvise {
    pub handle: u32,
    pub madv: u32,
    pub retained: u32,
    pub pad: u32,
}

pub const VC4_PERFCNT_NUM_EVENTS: u32 = 30;
pub const DRM_VC4_MAX_PERF_COUNTERS: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_perfmon_create {
    pub id: u32,
    pub ncounters: u32,
    pub events: [u8; DRM_VC4_MAX_PERF_COUNTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_perfmon_destroy {
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_perfmon_get_values {
    pub id: u32,
    pub values_ptr: u64,
}
