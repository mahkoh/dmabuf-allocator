use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IO;
use crate::uapi::drm::DRM_IOW;
use crate::uapi::drm::DRM_IOWR;

pub const DRM_NVIDIA_GET_CRTC_CRC32: u32 = 0x00;
pub const DRM_NVIDIA_GEM_IMPORT_NVKMS_MEMORY: u32 = 0x01;
pub const DRM_NVIDIA_GEM_IMPORT_USERSPACE_MEMORY: u32 = 0x02;
pub const DRM_NVIDIA_GET_DEV_INFO: u32 = 0x03;
pub const DRM_NVIDIA_FENCE_SUPPORTED: u32 = 0x04;
pub const DRM_NVIDIA_PRIME_FENCE_CONTEXT_CREATE: u32 = 0x05;
pub const DRM_NVIDIA_GEM_PRIME_FENCE_ATTACH: u32 = 0x06;
pub const DRM_NVIDIA_GET_CLIENT_CAPABILITY: u32 = 0x08;
pub const DRM_NVIDIA_GEM_EXPORT_NVKMS_MEMORY: u32 = 0x09;
pub const DRM_NVIDIA_GEM_MAP_OFFSET: u32 = 0x0a;
pub const DRM_NVIDIA_GEM_ALLOC_NVKMS_MEMORY: u32 = 0x0b;
pub const DRM_NVIDIA_GET_CRTC_CRC32_V2: u32 = 0x0c;
pub const DRM_NVIDIA_GEM_EXPORT_DMABUF_MEMORY: u32 = 0x0d;
pub const DRM_NVIDIA_GEM_IDENTIFY_OBJECT: u32 = 0x0e;
pub const DRM_NVIDIA_DMABUF_SUPPORTED: u32 = 0x0f;
pub const DRM_NVIDIA_GET_DPY_ID_FOR_CONNECTOR_ID: u32 = 0x10;
pub const DRM_NVIDIA_GET_CONNECTOR_ID_FOR_DPY_ID: u32 = 0x11;
pub const DRM_NVIDIA_GRANT_PERMISSIONS: u32 = 0x12;
pub const DRM_NVIDIA_REVOKE_PERMISSIONS: u32 = 0x13;
pub const DRM_NVIDIA_SEMSURF_FENCE_CTX_CREATE: u32 = 0x14;
pub const DRM_NVIDIA_SEMSURF_FENCE_CREATE: u32 = 0x15;
pub const DRM_NVIDIA_SEMSURF_FENCE_WAIT: u32 = 0x16;
pub const DRM_NVIDIA_SEMSURF_FENCE_ATTACH: u32 = 0x17;
pub const DRM_NVIDIA_GET_DRM_FILE_UNIQUE_ID: u32 = 0x18;
pub const DRM_NVIDIA_SEMSURF_EXPORT_TO_SYNCOBJ_POINT: u32 = 0x1d;
pub const DRM_NVIDIA_SYNCOBJ_GET_SYNCFD: u32 = 0x1e;

pub const DRM_IOCTL_NVIDIA_GEM_IMPORT_NVKMS_MEMORY: u32 =
    DRM_IOWR::<drm_nvidia_gem_import_nvkms_memory_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_GEM_IMPORT_NVKMS_MEMORY,
    );
pub const DRM_IOCTL_NVIDIA_GEM_IMPORT_USERSPACE_MEMORY: u32 =
    DRM_IOWR::<drm_nvidia_gem_import_userspace_memory_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_GEM_IMPORT_USERSPACE_MEMORY,
    );
pub const DRM_IOCTL_NVIDIA_GET_DEV_INFO: u32 =
    DRM_IOWR::<drm_nvidia_get_dev_info_params>(DRM_COMMAND_BASE + DRM_NVIDIA_GET_DEV_INFO);
pub const DRM_IOCTL_NVIDIA_FENCE_SUPPORTED: u32 =
    DRM_IO(DRM_COMMAND_BASE + DRM_NVIDIA_FENCE_SUPPORTED);
pub const DRM_IOCTL_NVIDIA_DMABUF_SUPPORTED: u32 =
    DRM_IO(DRM_COMMAND_BASE + DRM_NVIDIA_DMABUF_SUPPORTED);
pub const DRM_IOCTL_NVIDIA_PRIME_FENCE_CONTEXT_CREATE: u32 =
    DRM_IOWR::<drm_nvidia_prime_fence_context_create_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_PRIME_FENCE_CONTEXT_CREATE,
    );
pub const DRM_IOCTL_NVIDIA_GEM_PRIME_FENCE_ATTACH: u32 =
    DRM_IOW::<drm_nvidia_gem_prime_fence_attach_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_GEM_PRIME_FENCE_ATTACH,
    );
pub const DRM_IOCTL_NVIDIA_GET_CLIENT_CAPABILITY: u32 =
    DRM_IOWR::<drm_nvidia_get_client_capability_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_GET_CLIENT_CAPABILITY,
    );
pub const DRM_IOCTL_NVIDIA_GET_CRTC_CRC32: u32 =
    DRM_IOWR::<drm_nvidia_get_crtc_crc32_params>(DRM_COMMAND_BASE + DRM_NVIDIA_GET_CRTC_CRC32);
pub const DRM_IOCTL_NVIDIA_GET_CRTC_CRC32_V2: u32 = DRM_IOWR::<drm_nvidia_get_crtc_crc32_v2_params>(
    DRM_COMMAND_BASE + DRM_NVIDIA_GET_CRTC_CRC32_V2,
);
pub const DRM_IOCTL_NVIDIA_GEM_EXPORT_NVKMS_MEMORY: u32 =
    DRM_IOWR::<drm_nvidia_gem_export_nvkms_memory_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_GEM_EXPORT_NVKMS_MEMORY,
    );
pub const DRM_IOCTL_NVIDIA_GEM_MAP_OFFSET: u32 =
    DRM_IOWR::<drm_nvidia_gem_map_offset_params>(DRM_COMMAND_BASE + DRM_NVIDIA_GEM_MAP_OFFSET);
pub const DRM_IOCTL_NVIDIA_GEM_ALLOC_NVKMS_MEMORY: u32 =
    DRM_IOWR::<drm_nvidia_gem_alloc_nvkms_memory_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_GEM_ALLOC_NVKMS_MEMORY,
    );
pub const DRM_IOCTL_NVIDIA_GEM_EXPORT_DMABUF_MEMORY: u32 =
    DRM_IOWR::<drm_nvidia_gem_export_dmabuf_memory_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_GEM_EXPORT_DMABUF_MEMORY,
    );
pub const DRM_IOCTL_NVIDIA_GEM_IDENTIFY_OBJECT: u32 =
    DRM_IOWR::<drm_nvidia_gem_identify_object_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_GEM_IDENTIFY_OBJECT,
    );
pub const DRM_IOCTL_NVIDIA_GET_DPY_ID_FOR_CONNECTOR_ID: u32 =
    DRM_IOWR::<drm_nvidia_get_dpy_id_for_connector_id_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_GET_DPY_ID_FOR_CONNECTOR_ID,
    );
pub const DRM_IOCTL_NVIDIA_GET_CONNECTOR_ID_FOR_DPY_ID: u32 =
    DRM_IOWR::<drm_nvidia_get_connector_id_for_dpy_id_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_GET_CONNECTOR_ID_FOR_DPY_ID,
    );
pub const DRM_IOCTL_NVIDIA_GRANT_PERMISSIONS: u32 = DRM_IOWR::<drm_nvidia_grant_permissions_params>(
    DRM_COMMAND_BASE + DRM_NVIDIA_GRANT_PERMISSIONS,
);
pub const DRM_IOCTL_NVIDIA_REVOKE_PERMISSIONS: u32 = DRM_IOWR::<drm_nvidia_revoke_permissions_params>(
    DRM_COMMAND_BASE + DRM_NVIDIA_REVOKE_PERMISSIONS,
);
pub const DRM_IOCTL_NVIDIA_SEMSURF_FENCE_CTX_CREATE: u32 =
    DRM_IOWR::<drm_nvidia_semsurf_fence_ctx_create_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_SEMSURF_FENCE_CTX_CREATE,
    );
pub const DRM_IOCTL_NVIDIA_SEMSURF_FENCE_CREATE: u32 =
    DRM_IOWR::<drm_nvidia_semsurf_fence_create_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_SEMSURF_FENCE_CREATE,
    );
pub const DRM_IOCTL_NVIDIA_SEMSURF_FENCE_WAIT: u32 = DRM_IOW::<drm_nvidia_semsurf_fence_wait_params>(
    DRM_COMMAND_BASE + DRM_NVIDIA_SEMSURF_FENCE_WAIT,
);
pub const DRM_IOCTL_NVIDIA_SEMSURF_FENCE_ATTACH: u32 =
    DRM_IOW::<drm_nvidia_semsurf_fence_attach_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_SEMSURF_FENCE_ATTACH,
    );
pub const DRM_IOCTL_NVIDIA_GET_DRM_FILE_UNIQUE_ID: u32 =
    DRM_IOWR::<drm_nvidia_get_drm_file_unique_id_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_GET_DRM_FILE_UNIQUE_ID,
    );
pub const DRM_IOCTL_NVIDIA_SEMSURF_EXPORT_TO_SYNCOBJ_POINT: u32 =
    DRM_IOW::<drm_nvidia_semsurf_export_to_syncobj_point_params>(
        DRM_COMMAND_BASE + DRM_NVIDIA_SEMSURF_EXPORT_TO_SYNCOBJ_POINT,
    );
pub const DRM_IOCTL_NVIDIA_SYNCOBJ_GET_SYNCFD: u32 = DRM_IOWR::<drm_nvidia_syncobj_get_syncfd_params>(
    DRM_COMMAND_BASE + DRM_NVIDIA_SYNCOBJ_GET_SYNCFD,
);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_gem_import_nvkms_memory_params {
    pub mem_size: u64,
    pub nvkms_params_ptr: u64,
    pub nvkms_params_size: u64,
    pub handle: u32,
    pub __pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_gem_import_userspace_memory_params {
    pub size: u64,
    pub address: u64,
    pub handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_get_dev_info_params {
    pub gpu_id: u32,
    pub mig_device: u32,
    pub primary_index: u32,
    pub supports_alloc: u32,
    pub generic_page_kind: u32,
    pub page_kind_generation: u32,
    pub sector_layout: u32,
    pub supports_sync_fd: u32,
    pub supports_semsurf: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_prime_fence_context_create_params {
    pub handle: u32,
    pub index: u32,
    pub size: u64,
    pub import_mem_nvkms_params_ptr: u64,
    pub import_mem_nvkms_params_size: u64,
    pub event_nvkms_params_ptr: u64,
    pub event_nvkms_params_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_gem_prime_fence_attach_params {
    pub handle: u32,
    pub fence_context_handle: u32,
    pub sem_thresh: u32,
    pub __pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_get_client_capability_params {
    pub capability: u64,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_crtc_crc32 {
    pub value: u32,
    pub supported: u8,
    pub __pad0: u8,
    pub __pad1: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_crtc_crc32_v2_out {
    pub compositorCrc32: drm_nvidia_crtc_crc32,
    pub rasterGeneratorCrc32: drm_nvidia_crtc_crc32,
    pub outputCrc32: drm_nvidia_crtc_crc32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_get_crtc_crc32_v2_params {
    pub crtc_id: u32,
    pub crc32: drm_nvidia_crtc_crc32_v2_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_get_crtc_crc32_params {
    pub crtc_id: u32,
    pub crc32: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_gem_export_nvkms_memory_params {
    pub handle: u32,
    pub __pad: u32,
    pub nvkms_params_ptr: u64,
    pub nvkms_params_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_gem_map_offset_params {
    pub handle: u32,
    pub __pad: u32,
    pub offset: u64,
}

pub const NV_GEM_ALLOC_NO_SCANOUT: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_nvidia_gem_alloc_nvkms_memory_params {
    pub handle: u32,
    pub block_linear: u8,
    pub compressible: u8,
    pub __pad0: u16,
    pub memory_size: u64,
    pub flags: u32,
    pub __pad1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_gem_export_dmabuf_memory_params {
    pub handle: u32,
    pub __pad: u32,
    pub nvkms_params_ptr: u64,
    pub nvkms_params_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_gem_identify_object_params {
    pub handle: u32,
    pub object_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_get_dpy_id_for_connector_id_params {
    pub connectorId: u32,
    pub dpyId: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_get_connector_id_for_dpy_id_params {
    pub dpyId: u32,
    pub connectorId: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_grant_permissions_params {
    pub fd: i32,
    pub dpyId: u32,
    pub type_: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_revoke_permissions_params {
    pub dpyId: u32,
    pub type_: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_semsurf_fence_ctx_create_params {
    pub index: u64,
    pub nvkms_params_ptr: u64,
    pub nvkms_params_size: u64,
    pub handle: u32,
    pub __pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_semsurf_fence_create_params {
    pub fence_context_handle: u32,
    pub timeout_value_ms: u32,
    pub wait_value: u64,
    pub fd: i32,
    pub __pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_semsurf_fence_wait_params {
    pub fence_context_handle: u32,
    pub fd: i32,
    pub pre_wait_value: u64,
    pub post_wait_value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_semsurf_fence_attach_params {
    pub handle: u32,
    pub fence_context_handle: u32,
    pub timeout_value_ms: u32,
    pub shared: u32,
    pub wait_value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_get_drm_file_unique_id_params {
    pub id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_semsurf_export_to_syncobj_point_params {
    pub fence_context_handle: u32,
    pub syncobj_handle: u32,
    pub wait_value: u64,
    pub syncobj_point: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nvidia_syncobj_get_syncfd_params {
    pub syncobj_handle: u32,
    pub fd: i32,
    pub syncobj_point: u64,
}
