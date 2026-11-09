use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IOWR;

pub const DRM_VIRTGPU_MAP: u32 = 0x01;
pub const DRM_VIRTGPU_EXECBUFFER: u32 = 0x02;
pub const DRM_VIRTGPU_GETPARAM: u32 = 0x03;
pub const DRM_VIRTGPU_RESOURCE_CREATE: u32 = 0x04;
pub const DRM_VIRTGPU_RESOURCE_INFO: u32 = 0x05;
pub const DRM_VIRTGPU_TRANSFER_FROM_HOST: u32 = 0x06;
pub const DRM_VIRTGPU_TRANSFER_TO_HOST: u32 = 0x07;
pub const DRM_VIRTGPU_WAIT: u32 = 0x08;
pub const DRM_VIRTGPU_GET_CAPS: u32 = 0x09;
pub const DRM_VIRTGPU_RESOURCE_CREATE_BLOB: u32 = 0x0a;
pub const DRM_VIRTGPU_CONTEXT_INIT: u32 = 0x0b;

pub const VIRTGPU_EXECBUF_FENCE_FD_IN: u32 = 0x01;
pub const VIRTGPU_EXECBUF_FENCE_FD_OUT: u32 = 0x02;
pub const VIRTGPU_EXECBUF_RING_IDX: u32 = 0x04;
pub const VIRTGPU_EXECBUF_FLAGS: u32 =
    VIRTGPU_EXECBUF_FENCE_FD_IN | VIRTGPU_EXECBUF_FENCE_FD_OUT | VIRTGPU_EXECBUF_RING_IDX;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_map {
    pub offset: u64,
    pub handle: u32,
    pub pad: u32,
}

pub const VIRTGPU_EXECBUF_SYNCOBJ_RESET: u32 = 0x01;
pub const VIRTGPU_EXECBUF_SYNCOBJ_FLAGS: u32 = VIRTGPU_EXECBUF_SYNCOBJ_RESET;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_virtgpu_execbuffer_syncobj {
    pub handle: u32,
    pub flags: u32,
    pub point: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_virtgpu_execbuffer {
    pub flags: u32,
    pub size: u32,
    pub command: u64,
    pub bo_handles: u64,
    pub num_bo_handles: u32,
    pub fence_fd: i32,
    pub ring_idx: u32,
    pub syncobj_stride: u32,
    pub num_in_syncobjs: u32,
    pub num_out_syncobjs: u32,
    pub in_syncobjs: u64,
    pub out_syncobjs: u64,
}

pub const VIRTGPU_PARAM_3D_FEATURES: u64 = 1;
pub const VIRTGPU_PARAM_CAPSET_QUERY_FIX: u64 = 2;
pub const VIRTGPU_PARAM_RESOURCE_BLOB: u64 = 3;
pub const VIRTGPU_PARAM_HOST_VISIBLE: u64 = 4;
pub const VIRTGPU_PARAM_CROSS_DEVICE: u64 = 5;
pub const VIRTGPU_PARAM_CONTEXT_INIT: u64 = 6;
pub const VIRTGPU_PARAM_SUPPORTED_CAPSET_IDs: u64 = 7;
pub const VIRTGPU_PARAM_EXPLICIT_DEBUG_NAME: u64 = 8;
pub const VIRTGPU_PARAM_BLOB_ALIGNMENT: u64 = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_getparam {
    pub param: u64,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_virtgpu_resource_create {
    pub target: u32,
    pub format: u32,
    pub bind: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub array_size: u32,
    pub last_level: u32,
    pub nr_samples: u32,
    pub flags: u32,
    pub bo_handle: u32,
    pub res_handle: u32,
    pub size: u32,
    pub stride: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_resource_info {
    pub bo_handle: u32,
    pub res_handle: u32,
    pub size: u32,
    pub blob_mem: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_3d_box {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
    pub h: u32,
    pub d: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_3d_transfer_to_host {
    pub bo_handle: u32,
    pub box_: drm_virtgpu_3d_box,
    pub level: u32,
    pub offset: u32,
    pub stride: u32,
    pub layer_stride: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_3d_transfer_from_host {
    pub bo_handle: u32,
    pub box_: drm_virtgpu_3d_box,
    pub level: u32,
    pub offset: u32,
    pub stride: u32,
    pub layer_stride: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_3d_wait {
    pub handle: u32,
    pub flags: u32,
}

pub const VIRTGPU_DRM_CAPSET_VIRGL: u32 = 1;
pub const VIRTGPU_DRM_CAPSET_VIRGL2: u32 = 2;
pub const VIRTGPU_DRM_CAPSET_VENUS: u32 = 4;
pub const VIRTGPU_DRM_CAPSET_DRM: u32 = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_get_caps {
    pub cap_set_id: u32,
    pub cap_set_ver: u32,
    pub addr: u64,
    pub size: u32,
    pub pad: u32,
}

pub const VIRTGPU_BLOB_MEM_GUEST: u32 = 0x0001;
pub const VIRTGPU_BLOB_MEM_HOST3D: u32 = 0x0002;
pub const VIRTGPU_BLOB_MEM_HOST3D_GUEST: u32 = 0x0003;
pub const VIRTGPU_BLOB_FLAG_USE_MAPPABLE: u32 = 0x0001;
pub const VIRTGPU_BLOB_FLAG_USE_SHAREABLE: u32 = 0x0002;
pub const VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE: u32 = 0x0004;
pub const DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING: u32 = 0x0001;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_virtgpu_resource_create_blob {
    pub blob_mem: u32,
    pub blob_flags: u32,
    pub bo_handle: u32,
    pub res_handle: u32,
    pub size: u64,
    pub pad: u32,
    pub cmd_size: u32,
    pub cmd: u64,
    pub blob_id: u64,
    pub blob_hints: u32,
    pub pad2: u32,
}

pub const VIRTGPU_CONTEXT_PARAM_CAPSET_ID: u64 = 0x0001;
pub const VIRTGPU_CONTEXT_PARAM_NUM_RINGS: u64 = 0x0002;
pub const VIRTGPU_CONTEXT_PARAM_POLL_RINGS_MASK: u64 = 0x0003;
pub const VIRTGPU_CONTEXT_PARAM_DEBUG_NAME: u64 = 0x0004;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_virtgpu_context_set_param {
    pub param: u64,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_context_init {
    pub num_params: u32,
    pub pad: u32,
    pub ctx_set_params: u64,
}

pub const DRM_IOCTL_VIRTGPU_MAP: u32 =
    DRM_IOWR::<drm_virtgpu_map>(DRM_COMMAND_BASE + DRM_VIRTGPU_MAP);
pub const DRM_IOCTL_VIRTGPU_EXECBUFFER: u32 =
    DRM_IOWR::<drm_virtgpu_execbuffer>(DRM_COMMAND_BASE + DRM_VIRTGPU_EXECBUFFER);
pub const DRM_IOCTL_VIRTGPU_GETPARAM: u32 =
    DRM_IOWR::<drm_virtgpu_getparam>(DRM_COMMAND_BASE + DRM_VIRTGPU_GETPARAM);
pub const DRM_IOCTL_VIRTGPU_RESOURCE_CREATE: u32 =
    DRM_IOWR::<drm_virtgpu_resource_create>(DRM_COMMAND_BASE + DRM_VIRTGPU_RESOURCE_CREATE);
pub const DRM_IOCTL_VIRTGPU_RESOURCE_INFO: u32 =
    DRM_IOWR::<drm_virtgpu_resource_info>(DRM_COMMAND_BASE + DRM_VIRTGPU_RESOURCE_INFO);
pub const DRM_IOCTL_VIRTGPU_TRANSFER_FROM_HOST: u32 = DRM_IOWR::<drm_virtgpu_3d_transfer_from_host>(
    DRM_COMMAND_BASE + DRM_VIRTGPU_TRANSFER_FROM_HOST,
);
pub const DRM_IOCTL_VIRTGPU_TRANSFER_TO_HOST: u32 =
    DRM_IOWR::<drm_virtgpu_3d_transfer_to_host>(DRM_COMMAND_BASE + DRM_VIRTGPU_TRANSFER_TO_HOST);
pub const DRM_IOCTL_VIRTGPU_WAIT: u32 =
    DRM_IOWR::<drm_virtgpu_3d_wait>(DRM_COMMAND_BASE + DRM_VIRTGPU_WAIT);
pub const DRM_IOCTL_VIRTGPU_GET_CAPS: u32 =
    DRM_IOWR::<drm_virtgpu_get_caps>(DRM_COMMAND_BASE + DRM_VIRTGPU_GET_CAPS);
pub const DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB: u32 = DRM_IOWR::<drm_virtgpu_resource_create_blob>(
    DRM_COMMAND_BASE + DRM_VIRTGPU_RESOURCE_CREATE_BLOB,
);
pub const DRM_IOCTL_VIRTGPU_CONTEXT_INIT: u32 =
    DRM_IOWR::<drm_virtgpu_context_init>(DRM_COMMAND_BASE + DRM_VIRTGPU_CONTEXT_INIT);
