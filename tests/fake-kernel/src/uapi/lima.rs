use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IOR;
use crate::uapi::drm::DRM_IOW;
use crate::uapi::drm::DRM_IOWR;

pub const DRM_LIMA_PARAM_GPU_ID_UNKNOWN: u32 = 0;
pub const DRM_LIMA_PARAM_GPU_ID_MALI400: u32 = 1;
pub const DRM_LIMA_PARAM_GPU_ID_MALI450: u32 = 2;

pub const DRM_LIMA_PARAM_GPU_ID: u32 = 0;
pub const DRM_LIMA_PARAM_NUM_PP: u32 = 1;
pub const DRM_LIMA_PARAM_GP_VERSION: u32 = 2;
pub const DRM_LIMA_PARAM_PP_VERSION: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_get_param {
    pub param: u32,
    pub pad: u32,
    pub value: u64,
}

pub const LIMA_BO_FLAG_HEAP: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_lima_gem_create {
    pub size: u32,
    pub flags: u32,
    pub handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_info {
    pub handle: u32,
    pub va: u32,
    pub offset: u64,
}

pub const LIMA_SUBMIT_BO_READ: u32 = 0x01;
pub const LIMA_SUBMIT_BO_WRITE: u32 = 0x02;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_submit_bo {
    pub handle: u32,
    pub flags: u32,
}

pub const LIMA_GP_FRAME_REG_NUM: usize = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gp_frame {
    pub frame: [u32; LIMA_GP_FRAME_REG_NUM],
}

pub const LIMA_PP_FRAME_REG_NUM: usize = 23;
pub const LIMA_PP_WB_REG_NUM: usize = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_m400_pp_frame {
    pub frame: [u32; LIMA_PP_FRAME_REG_NUM],
    pub num_pp: u32,
    pub wb: [u32; 3 * LIMA_PP_WB_REG_NUM],
    pub plbu_array_address: [u32; 4],
    pub fragment_stack_address: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_lima_m450_pp_frame__anon {
    pub plbu_array_address: [u32; 8],
    pub dlbu_regs: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_m450_pp_frame {
    pub frame: [u32; LIMA_PP_FRAME_REG_NUM],
    pub num_pp: u32,
    pub wb: [u32; 3 * LIMA_PP_WB_REG_NUM],
    pub use_dlbu: u32,
    pub _pad: u32,
    pub anon: drm_lima_m450_pp_frame__anon,
    pub fragment_stack_address: [u32; 8],
}

pub const LIMA_PIPE_GP: u32 = 0x00;
pub const LIMA_PIPE_PP: u32 = 0x01;

pub const LIMA_SUBMIT_FLAG_EXPLICIT_FENCE: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_submit {
    pub ctx: u32,
    pub pipe: u32,
    pub nr_bos: u32,
    pub frame_size: u32,
    pub bos: u64,
    pub frame: u64,
    pub flags: u32,
    pub out_sync: u32,
    pub in_sync: [u32; 2],
}

pub const LIMA_GEM_WAIT_READ: u32 = 0x01;
pub const LIMA_GEM_WAIT_WRITE: u32 = 0x02;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_wait {
    pub handle: u32,
    pub op: u32,
    pub timeout_ns: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_ctx_create {
    pub id: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_ctx_free {
    pub id: u32,
    pub _pad: u32,
}

pub const DRM_LIMA_GET_PARAM: u32 = 0x00;
pub const DRM_LIMA_GEM_CREATE: u32 = 0x01;
pub const DRM_LIMA_GEM_INFO: u32 = 0x02;
pub const DRM_LIMA_GEM_SUBMIT: u32 = 0x03;
pub const DRM_LIMA_GEM_WAIT: u32 = 0x04;
pub const DRM_LIMA_CTX_CREATE: u32 = 0x05;
pub const DRM_LIMA_CTX_FREE: u32 = 0x06;

pub const DRM_IOCTL_LIMA_GET_PARAM: u32 =
    DRM_IOWR::<drm_lima_get_param>(DRM_COMMAND_BASE + DRM_LIMA_GET_PARAM);
pub const DRM_IOCTL_LIMA_GEM_CREATE: u32 =
    DRM_IOWR::<drm_lima_gem_create>(DRM_COMMAND_BASE + DRM_LIMA_GEM_CREATE);
pub const DRM_IOCTL_LIMA_GEM_INFO: u32 =
    DRM_IOWR::<drm_lima_gem_info>(DRM_COMMAND_BASE + DRM_LIMA_GEM_INFO);
pub const DRM_IOCTL_LIMA_GEM_SUBMIT: u32 =
    DRM_IOW::<drm_lima_gem_submit>(DRM_COMMAND_BASE + DRM_LIMA_GEM_SUBMIT);
pub const DRM_IOCTL_LIMA_GEM_WAIT: u32 =
    DRM_IOW::<drm_lima_gem_wait>(DRM_COMMAND_BASE + DRM_LIMA_GEM_WAIT);
pub const DRM_IOCTL_LIMA_CTX_CREATE: u32 =
    DRM_IOR::<drm_lima_ctx_create>(DRM_COMMAND_BASE + DRM_LIMA_CTX_CREATE);
pub const DRM_IOCTL_LIMA_CTX_FREE: u32 =
    DRM_IOW::<drm_lima_ctx_free>(DRM_COMMAND_BASE + DRM_LIMA_CTX_FREE);
