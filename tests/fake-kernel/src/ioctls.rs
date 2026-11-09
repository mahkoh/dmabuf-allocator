use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_BO_LIST;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_CS;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_CTX;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_FENCE_TO_HANDLE;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_CREATE;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_LIST_HANDLES;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_METADATA;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_MMAP;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_OP;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_USERPTR;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_VA;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_WAIT_IDLE;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_INFO;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_SCHED;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_USERQ;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_USERQ_SIGNAL;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_USERQ_WAIT;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_VM;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_WAIT_CS;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_WAIT_FENCES;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_GEM_BIND_OBJECT;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_GEM_CREATE;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_GEM_MMAP_OFFSET;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_GET_PARAMS;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_GET_TIME;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_QUEUE_CREATE;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_QUEUE_DESTROY;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_SUBMIT;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_VM_BIND;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_VM_CREATE;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_VM_DESTROY;
use crate::uapi::drm::DRM_IOCTL_ADD_DRAW;
use crate::uapi::drm::DRM_IOCTL_AUTH_MAGIC;
use crate::uapi::drm::DRM_IOCTL_BLOCK;
use crate::uapi::drm::DRM_IOCTL_CRTC_GET_SEQUENCE;
use crate::uapi::drm::DRM_IOCTL_CRTC_QUEUE_SEQUENCE;
use crate::uapi::drm::DRM_IOCTL_DROP_MASTER;
use crate::uapi::drm::DRM_IOCTL_FINISH;
use crate::uapi::drm::DRM_IOCTL_GEM_CHANGE_HANDLE;
use crate::uapi::drm::DRM_IOCTL_GEM_CLOSE;
use crate::uapi::drm::DRM_IOCTL_GEM_FLINK;
use crate::uapi::drm::DRM_IOCTL_GEM_OPEN;
use crate::uapi::drm::DRM_IOCTL_GET_CAP;
use crate::uapi::drm::DRM_IOCTL_GET_CLIENT;
use crate::uapi::drm::DRM_IOCTL_GET_MAGIC;
use crate::uapi::drm::DRM_IOCTL_GET_STATS;
use crate::uapi::drm::DRM_IOCTL_GET_UNIQUE;
use crate::uapi::drm::DRM_IOCTL_MODE_ADDFB;
use crate::uapi::drm::DRM_IOCTL_MODE_ADDFB2;
use crate::uapi::drm::DRM_IOCTL_MODE_ATOMIC;
use crate::uapi::drm::DRM_IOCTL_MODE_ATTACHMODE;
use crate::uapi::drm::DRM_IOCTL_MODE_CLOSEFB;
use crate::uapi::drm::DRM_IOCTL_MODE_CREATE_DUMB;
use crate::uapi::drm::DRM_IOCTL_MODE_CREATE_LEASE;
use crate::uapi::drm::DRM_IOCTL_MODE_CREATEPROPBLOB;
use crate::uapi::drm::DRM_IOCTL_MODE_CURSOR;
use crate::uapi::drm::DRM_IOCTL_MODE_CURSOR2;
use crate::uapi::drm::DRM_IOCTL_MODE_DESTROY_DUMB;
use crate::uapi::drm::DRM_IOCTL_MODE_DESTROYPROPBLOB;
use crate::uapi::drm::DRM_IOCTL_MODE_DETACHMODE;
use crate::uapi::drm::DRM_IOCTL_MODE_DIRTYFB;
use crate::uapi::drm::DRM_IOCTL_MODE_GET_LEASE;
use crate::uapi::drm::DRM_IOCTL_MODE_GETCONNECTOR;
use crate::uapi::drm::DRM_IOCTL_MODE_GETCRTC;
use crate::uapi::drm::DRM_IOCTL_MODE_GETENCODER;
use crate::uapi::drm::DRM_IOCTL_MODE_GETFB;
use crate::uapi::drm::DRM_IOCTL_MODE_GETFB2;
use crate::uapi::drm::DRM_IOCTL_MODE_GETGAMMA;
use crate::uapi::drm::DRM_IOCTL_MODE_GETPLANE;
use crate::uapi::drm::DRM_IOCTL_MODE_GETPLANERESOURCES;
use crate::uapi::drm::DRM_IOCTL_MODE_GETPROPBLOB;
use crate::uapi::drm::DRM_IOCTL_MODE_GETPROPERTY;
use crate::uapi::drm::DRM_IOCTL_MODE_GETRESOURCES;
use crate::uapi::drm::DRM_IOCTL_MODE_LIST_LESSEES;
use crate::uapi::drm::DRM_IOCTL_MODE_MAP_DUMB;
use crate::uapi::drm::DRM_IOCTL_MODE_OBJ_GETPROPERTIES;
use crate::uapi::drm::DRM_IOCTL_MODE_OBJ_SETPROPERTY;
use crate::uapi::drm::DRM_IOCTL_MODE_PAGE_FLIP;
use crate::uapi::drm::DRM_IOCTL_MODE_REVOKE_LEASE;
use crate::uapi::drm::DRM_IOCTL_MODE_RMFB;
use crate::uapi::drm::DRM_IOCTL_MODE_SETCRTC;
use crate::uapi::drm::DRM_IOCTL_MODE_SETGAMMA;
use crate::uapi::drm::DRM_IOCTL_MODE_SETPLANE;
use crate::uapi::drm::DRM_IOCTL_MODE_SETPROPERTY;
use crate::uapi::drm::DRM_IOCTL_PRIME_FD_TO_HANDLE;
use crate::uapi::drm::DRM_IOCTL_PRIME_HANDLE_TO_FD;
use crate::uapi::drm::DRM_IOCTL_RM_DRAW;
use crate::uapi::drm::DRM_IOCTL_SET_CLIENT_CAP;
use crate::uapi::drm::DRM_IOCTL_SET_CLIENT_NAME;
use crate::uapi::drm::DRM_IOCTL_SET_MASTER;
use crate::uapi::drm::DRM_IOCTL_SET_UNIQUE;
use crate::uapi::drm::DRM_IOCTL_SET_VERSION;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_CREATE;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_DESTROY;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_EVENTFD;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_FD_TO_HANDLE;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_HANDLE_TO_FD;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_QUERY;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_RESET;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_SIGNAL;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_TIMELINE_SIGNAL;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_TIMELINE_WAIT;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_TRANSFER;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_WAIT;
use crate::uapi::drm::DRM_IOCTL_UNBLOCK;
use crate::uapi::drm::DRM_IOCTL_UPDATE_DRAW;
use crate::uapi::drm::DRM_IOCTL_VERSION;
use crate::uapi::drm::DRM_IOCTL_WAIT_VBLANK;
use crate::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GEM_CPU_FINI;
use crate::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GEM_CPU_PREP;
use crate::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GEM_INFO;
use crate::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GEM_NEW;
use crate::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GEM_SUBMIT;
use crate::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GEM_USERPTR;
use crate::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GEM_WAIT;
use crate::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GET_PARAM;
use crate::uapi::etnaviv::DRM_IOCTL_ETNAVIV_PM_QUERY_DOM;
use crate::uapi::etnaviv::DRM_IOCTL_ETNAVIV_PM_QUERY_SIG;
use crate::uapi::etnaviv::DRM_IOCTL_ETNAVIV_WAIT_FENCE;
use crate::uapi::i915::DRM_IOCTL_I915_ALLOC;
use crate::uapi::i915::DRM_IOCTL_I915_BATCHBUFFER;
use crate::uapi::i915::DRM_IOCTL_I915_CMDBUFFER;
use crate::uapi::i915::DRM_IOCTL_I915_DESTROY_HEAP;
use crate::uapi::i915::DRM_IOCTL_I915_FLIP;
use crate::uapi::i915::DRM_IOCTL_I915_FLUSH;
use crate::uapi::i915::DRM_IOCTL_I915_FREE;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_BUSY;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_CONTEXT_CREATE_EXT;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_CONTEXT_DESTROY;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_CONTEXT_GETPARAM;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_CONTEXT_SETPARAM;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_CREATE;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_CREATE_EXT;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_ENTERVT;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_EXECBUFFER;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_EXECBUFFER2_WR;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_GET_APERTURE;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_GET_CACHING;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_GET_TILING;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_INIT;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_LEAVEVT;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_MADVISE;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_MMAP;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_MMAP_OFFSET;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_PIN;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_PREAD;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_PWRITE;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_SET_CACHING;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_SET_DOMAIN;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_SET_TILING;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_SW_FINISH;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_THROTTLE;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_UNPIN;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_USERPTR;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_VM_CREATE;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_VM_DESTROY;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_WAIT;
use crate::uapi::i915::DRM_IOCTL_I915_GET_PIPE_FROM_CRTC_ID;
use crate::uapi::i915::DRM_IOCTL_I915_GET_RESET_STATS;
use crate::uapi::i915::DRM_IOCTL_I915_GET_SPRITE_COLORKEY;
use crate::uapi::i915::DRM_IOCTL_I915_GET_VBLANK_PIPE;
use crate::uapi::i915::DRM_IOCTL_I915_GETPARAM;
use crate::uapi::i915::DRM_IOCTL_I915_HWS_ADDR;
use crate::uapi::i915::DRM_IOCTL_I915_INIT;
use crate::uapi::i915::DRM_IOCTL_I915_INIT_HEAP;
use crate::uapi::i915::DRM_IOCTL_I915_IRQ_EMIT;
use crate::uapi::i915::DRM_IOCTL_I915_IRQ_WAIT;
use crate::uapi::i915::DRM_IOCTL_I915_OVERLAY_ATTRS;
use crate::uapi::i915::DRM_IOCTL_I915_OVERLAY_PUT_IMAGE;
use crate::uapi::i915::DRM_IOCTL_I915_PERF_ADD_CONFIG;
use crate::uapi::i915::DRM_IOCTL_I915_PERF_OPEN;
use crate::uapi::i915::DRM_IOCTL_I915_PERF_REMOVE_CONFIG;
use crate::uapi::i915::DRM_IOCTL_I915_QUERY;
use crate::uapi::i915::DRM_IOCTL_I915_REG_READ;
use crate::uapi::i915::DRM_IOCTL_I915_SET_SPRITE_COLORKEY;
use crate::uapi::i915::DRM_IOCTL_I915_SET_VBLANK_PIPE;
use crate::uapi::i915::DRM_IOCTL_I915_SETPARAM;
use crate::uapi::i915::DRM_IOCTL_I915_VBLANK_SWAP;
use crate::uapi::lima::DRM_IOCTL_LIMA_CTX_CREATE;
use crate::uapi::lima::DRM_IOCTL_LIMA_CTX_FREE;
use crate::uapi::lima::DRM_IOCTL_LIMA_GEM_CREATE;
use crate::uapi::lima::DRM_IOCTL_LIMA_GEM_INFO;
use crate::uapi::lima::DRM_IOCTL_LIMA_GEM_SUBMIT;
use crate::uapi::lima::DRM_IOCTL_LIMA_GEM_WAIT;
use crate::uapi::lima::DRM_IOCTL_LIMA_GET_PARAM;
use crate::uapi::msm::DRM_IOCTL_MSM_GEM_CPU_FINI;
use crate::uapi::msm::DRM_IOCTL_MSM_GEM_CPU_PREP;
use crate::uapi::msm::DRM_IOCTL_MSM_GEM_INFO;
use crate::uapi::msm::DRM_IOCTL_MSM_GEM_MADVISE;
use crate::uapi::msm::DRM_IOCTL_MSM_GEM_NEW;
use crate::uapi::msm::DRM_IOCTL_MSM_GEM_SUBMIT;
use crate::uapi::msm::DRM_IOCTL_MSM_GET_PARAM;
use crate::uapi::msm::DRM_IOCTL_MSM_PERFCNTR_CONFIG;
use crate::uapi::msm::DRM_IOCTL_MSM_SET_PARAM;
use crate::uapi::msm::DRM_IOCTL_MSM_SUBMITQUEUE_CLOSE;
use crate::uapi::msm::DRM_IOCTL_MSM_SUBMITQUEUE_NEW;
use crate::uapi::msm::DRM_IOCTL_MSM_SUBMITQUEUE_QUERY;
use crate::uapi::msm::DRM_IOCTL_MSM_VM_BIND;
use crate::uapi::msm::DRM_IOCTL_MSM_WAIT_FENCE;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_CHANNEL_ALLOC;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_CHANNEL_FREE;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_EXEC;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_GEM_CPU_FINI;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_GEM_CPU_PREP;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_GEM_INFO;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_GEM_NEW;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_GEM_PUSHBUF;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_GET_ZCULL_INFO;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_GETPARAM;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_GPUOBJ_FREE;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_GROBJ_ALLOC;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_NOTIFIEROBJ_ALLOC;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_SETPARAM;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_SVM_BIND;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_SVM_INIT;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_VM_BIND;
use crate::uapi::nouveau::DRM_IOCTL_NOUVEAU_VM_INIT;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_DMABUF_SUPPORTED;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_FENCE_SUPPORTED;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GEM_ALLOC_NVKMS_MEMORY;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GEM_EXPORT_DMABUF_MEMORY;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GEM_EXPORT_NVKMS_MEMORY;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GEM_IDENTIFY_OBJECT;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GEM_IMPORT_NVKMS_MEMORY;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GEM_IMPORT_USERSPACE_MEMORY;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GEM_MAP_OFFSET;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GEM_PRIME_FENCE_ATTACH;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GET_CLIENT_CAPABILITY;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GET_CONNECTOR_ID_FOR_DPY_ID;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GET_CRTC_CRC32;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GET_CRTC_CRC32_V2;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GET_DEV_INFO;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GET_DPY_ID_FOR_CONNECTOR_ID;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GET_DRM_FILE_UNIQUE_ID;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GRANT_PERMISSIONS;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_PRIME_FENCE_CONTEXT_CREATE;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_REVOKE_PERMISSIONS;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_SEMSURF_EXPORT_TO_SYNCOBJ_POINT;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_SEMSURF_FENCE_ATTACH;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_SEMSURF_FENCE_CREATE;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_SEMSURF_FENCE_CTX_CREATE;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_SEMSURF_FENCE_WAIT;
use crate::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_SYNCOBJ_GET_SYNCFD;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_CREATE_BO;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_GET_BO_OFFSET;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_GET_PARAM;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_JM_CTX_CREATE;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_JM_CTX_DESTROY;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_MADVISE;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_MMAP_BO;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_PERFCNT_DUMP;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_PERFCNT_ENABLE;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_QUERY_BO_INFO;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_SET_LABEL_BO;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_SUBMIT;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_SYNC_BO;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_WAIT_BO;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_BO_CREATE;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_BO_MMAP_OFFSET;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_BO_QUERY_INFO;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_BO_SET_LABEL;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_BO_SYNC;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_DEV_QUERY;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_GROUP_CREATE;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_GROUP_DESTROY;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_GROUP_GET_STATE;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_GROUP_SUBMIT;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_SET_USER_MMIO_OFFSET;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_TILER_HEAP_CREATE;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_TILER_HEAP_DESTROY;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_VM_BIND;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_VM_CREATE;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_VM_DESTROY;
use crate::uapi::panthor::DRM_IOCTL_PANTHOR_VM_GET_STATE;
use crate::uapi::v3d::DRM_IOCTL_V3D_CREATE_BO;
use crate::uapi::v3d::DRM_IOCTL_V3D_GET_BO_OFFSET;
use crate::uapi::v3d::DRM_IOCTL_V3D_GET_PARAM;
use crate::uapi::v3d::DRM_IOCTL_V3D_MMAP_BO;
use crate::uapi::v3d::DRM_IOCTL_V3D_PERFMON_CREATE;
use crate::uapi::v3d::DRM_IOCTL_V3D_PERFMON_DESTROY;
use crate::uapi::v3d::DRM_IOCTL_V3D_PERFMON_GET_COUNTER;
use crate::uapi::v3d::DRM_IOCTL_V3D_PERFMON_GET_VALUES;
use crate::uapi::v3d::DRM_IOCTL_V3D_PERFMON_SET_GLOBAL;
use crate::uapi::v3d::DRM_IOCTL_V3D_SUBMIT_CL;
use crate::uapi::v3d::DRM_IOCTL_V3D_SUBMIT_CPU;
use crate::uapi::v3d::DRM_IOCTL_V3D_SUBMIT_CSD;
use crate::uapi::v3d::DRM_IOCTL_V3D_SUBMIT_TFU;
use crate::uapi::v3d::DRM_IOCTL_V3D_WAIT_BO;
use crate::uapi::vc4::DRM_IOCTL_VC4_CREATE_BO;
use crate::uapi::vc4::DRM_IOCTL_VC4_CREATE_SHADER_BO;
use crate::uapi::vc4::DRM_IOCTL_VC4_GEM_MADVISE;
use crate::uapi::vc4::DRM_IOCTL_VC4_GET_HANG_STATE;
use crate::uapi::vc4::DRM_IOCTL_VC4_GET_PARAM;
use crate::uapi::vc4::DRM_IOCTL_VC4_GET_TILING;
use crate::uapi::vc4::DRM_IOCTL_VC4_LABEL_BO;
use crate::uapi::vc4::DRM_IOCTL_VC4_MMAP_BO;
use crate::uapi::vc4::DRM_IOCTL_VC4_PERFMON_CREATE;
use crate::uapi::vc4::DRM_IOCTL_VC4_PERFMON_DESTROY;
use crate::uapi::vc4::DRM_IOCTL_VC4_PERFMON_GET_VALUES;
use crate::uapi::vc4::DRM_IOCTL_VC4_SET_TILING;
use crate::uapi::vc4::DRM_IOCTL_VC4_SUBMIT_CL;
use crate::uapi::vc4::DRM_IOCTL_VC4_WAIT_BO;
use crate::uapi::vc4::DRM_IOCTL_VC4_WAIT_SEQNO;
use crate::uapi::virtgpu::DRM_IOCTL_VIRTGPU_CONTEXT_INIT;
use crate::uapi::virtgpu::DRM_IOCTL_VIRTGPU_EXECBUFFER;
use crate::uapi::virtgpu::DRM_IOCTL_VIRTGPU_GET_CAPS;
use crate::uapi::virtgpu::DRM_IOCTL_VIRTGPU_GETPARAM;
use crate::uapi::virtgpu::DRM_IOCTL_VIRTGPU_MAP;
use crate::uapi::virtgpu::DRM_IOCTL_VIRTGPU_RESOURCE_CREATE;
use crate::uapi::virtgpu::DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB;
use crate::uapi::virtgpu::DRM_IOCTL_VIRTGPU_RESOURCE_INFO;
use crate::uapi::virtgpu::DRM_IOCTL_VIRTGPU_TRANSFER_FROM_HOST;
use crate::uapi::virtgpu::DRM_IOCTL_VIRTGPU_TRANSFER_TO_HOST;
use crate::uapi::virtgpu::DRM_IOCTL_VIRTGPU_WAIT;
use crate::uapi::xe::DRM_IOCTL_XE_DEVICE_QUERY;
use crate::uapi::xe::DRM_IOCTL_XE_EXEC;
use crate::uapi::xe::DRM_IOCTL_XE_EXEC_QUEUE_CREATE;
use crate::uapi::xe::DRM_IOCTL_XE_EXEC_QUEUE_DESTROY;
use crate::uapi::xe::DRM_IOCTL_XE_EXEC_QUEUE_GET_PROPERTY;
use crate::uapi::xe::DRM_IOCTL_XE_EXEC_QUEUE_SET_PROPERTY;
use crate::uapi::xe::DRM_IOCTL_XE_GEM_CREATE;
use crate::uapi::xe::DRM_IOCTL_XE_GEM_MMAP_OFFSET;
use crate::uapi::xe::DRM_IOCTL_XE_MADVISE;
use crate::uapi::xe::DRM_IOCTL_XE_OBSERVATION;
use crate::uapi::xe::DRM_IOCTL_XE_VM_BIND;
use crate::uapi::xe::DRM_IOCTL_XE_VM_CREATE;
use crate::uapi::xe::DRM_IOCTL_XE_VM_DESTROY;
use crate::uapi::xe::DRM_IOCTL_XE_VM_GET_PROPERTY;
use crate::uapi::xe::DRM_IOCTL_XE_VM_QUERY_MEM_RANGE_ATTRS;
use crate::uapi::xe::DRM_IOCTL_XE_WAIT_USER_FENCE;
use crate::version::KernelVersion;

pub const DRM_AUTH: u32 = 1 << 0;
pub const DRM_MASTER: u32 = 1 << 1;
pub const DRM_ROOT_ONLY: u32 = 1 << 2;
pub const DRM_RENDER_ALLOW: u32 = 1 << 5;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Func {
    DrmVersion,
    DrmGetunique,
    DrmGetmagic,
    DrmGetclient,
    DrmGetstats,
    DrmGetcap,
    DrmSetclientcap,
    DrmSetversion,
    DrmInvalidOp,
    DrmNoop,
    DrmAuthmagic,
    DrmSetmasterIoctl,
    DrmDropmasterIoctl,
    DrmWaitVblankIoctl,
    DrmGemCloseIoctl,
    DrmGemFlinkIoctl,
    DrmGemOpenIoctl,
    DrmGemChangeHandleIoctl,
    DrmModeGetresources,
    DrmPrimeHandleToFdIoctl,
    DrmPrimeFdToHandleIoctl,
    DrmSetClientName,
    DrmModeGetplaneRes,
    DrmModeGetcrtc,
    DrmModeSetcrtc,
    DrmModeGetplane,
    DrmModeSetplane,
    DrmModeCursorIoctl,
    DrmModeGammaGetIoctl,
    DrmModeGammaSetIoctl,
    DrmModeGetencoder,
    DrmModeGetconnector,
    DrmModeGetpropertyIoctl,
    DrmConnectorPropertySetIoctl,
    DrmModeGetblobIoctl,
    DrmModeGetfb,
    DrmModeGetfb2Ioctl,
    DrmModeAddfbIoctl,
    DrmModeAddfb2Ioctl,
    DrmModeRmfbIoctl,
    DrmModeClosefbIoctl,
    DrmModePageFlipIoctl,
    DrmModeDirtyfbIoctl,
    DrmModeCreateDumbIoctl,
    DrmModeMmapDumbIoctl,
    DrmModeDestroyDumbIoctl,
    DrmModeObjGetPropertiesIoctl,
    DrmModeObjSetPropertyIoctl,
    DrmModeCursor2Ioctl,
    DrmModeAtomicIoctl,
    DrmModeCreateblobIoctl,
    DrmModeDestroyblobIoctl,
    DrmSyncobjCreateIoctl,
    DrmSyncobjDestroyIoctl,
    DrmSyncobjHandleToFdIoctl,
    DrmSyncobjFdToHandleIoctl,
    DrmSyncobjTransferIoctl,
    DrmSyncobjWaitIoctl,
    DrmSyncobjTimelineWaitIoctl,
    DrmSyncobjEventfdIoctl,
    DrmSyncobjResetIoctl,
    DrmSyncobjSignalIoctl,
    DrmSyncobjTimelineSignalIoctl,
    DrmSyncobjQueryIoctl,
    DrmCrtcGetSequenceIoctl,
    DrmCrtcQueueSequenceIoctl,
    DrmModeCreateLeaseIoctl,
    DrmModeListLesseesIoctl,
    DrmModeGetLeaseIoctl,
    DrmModeRevokeLeaseIoctl,
    AmdgpuGemCreateIoctl,
    AmdgpuCtxIoctl,
    AmdgpuVmIoctl,
    AmdgpuSchedIoctl,
    AmdgpuBoListIoctl,
    AmdgpuCsFenceToHandleIoctl,
    AmdgpuGemMmapIoctl,
    AmdgpuGemWaitIdleIoctl,
    AmdgpuCsIoctl,
    AmdgpuInfoIoctl,
    AmdgpuCsWaitIoctl,
    AmdgpuCsWaitFencesIoctl,
    AmdgpuGemMetadataIoctl,
    AmdgpuGemVaIoctl,
    AmdgpuGemOpIoctl,
    AmdgpuGemUserptrIoctl,
    AmdgpuUserqIoctl,
    AmdgpuUserqSignalIoctl,
    AmdgpuUserqWaitIoctl,
    AmdgpuGemListHandlesIoctl,
    I915GetparamIoctl,
    I915GemExecbuffer2Ioctl,
    I915GemRejectPinIoctl,
    I915GemBusyIoctl,
    I915GemSetCachingIoctl,
    I915GemGetCachingIoctl,
    I915GemThrottleIoctl,
    I915GemCreateIoctl,
    I915GemCreateExtIoctl,
    I915GemPreadIoctl,
    I915GemPwriteIoctl,
    I915GemMmapIoctl,
    I915GemMmapOffsetIoctl,
    I915GemSetDomainIoctl,
    I915GemSwFinishIoctl,
    I915GemSetTilingIoctl,
    I915GemGetTilingIoctl,
    I915GemGetApertureIoctl,
    IntelCrtcGetPipeFromCrtcIdIoctl,
    I915GemMadviseIoctl,
    IntelOverlayPutImageIoctl,
    IntelOverlayAttrsIoctl,
    IntelSpriteSetColorkeyIoctl,
    I915GemWaitIoctl,
    I915GemContextCreateIoctl,
    I915GemContextDestroyIoctl,
    I915RegReadIoctl,
    I915GemContextResetStatsIoctl,
    I915GemUserptrIoctl,
    I915GemContextGetparamIoctl,
    I915GemContextSetparamIoctl,
    I915PerfOpenIoctl,
    I915PerfAddConfigIoctl,
    I915PerfRemoveConfigIoctl,
    I915QueryIoctl,
    I915GemVmCreateIoctl,
    I915GemVmDestroyIoctl,
    XeQueryIoctl,
    XeGemCreateIoctl,
    XeGemMmapOffsetIoctl,
    XeVmCreateIoctl,
    XeVmDestroyIoctl,
    XeVmBindIoctl,
    XeExecIoctl,
    XeExecQueueCreateIoctl,
    XeExecQueueDestroyIoctl,
    XeExecQueueGetPropertyIoctl,
    XeWaitUserFenceIoctl,
    XeObservationIoctl,
    XeVmMadviseIoctl,
    XeVmQueryVmasAttrsIoctl,
    XeExecQueueSetPropertyIoctl,
    XeVmGetPropertyIoctl,
    NvDrmGemImportNvkmsMemoryIoctl,
    NvDrmGemImportUserspaceMemoryIoctl,
    NvDrmGemMapOffsetIoctl,
    NvDrmGetDevInfoIoctl,
    NvDrmGetDrmFileUniqueIdIoctl,
    NvDrmFenceSupportedIoctl,
    NvDrmPrimeFenceContextCreateIoctl,
    NvDrmGemPrimeFenceAttachIoctl,
    NvDrmSemsurfFenceCtxCreateIoctl,
    NvDrmSemsurfFenceCreateIoctl,
    NvDrmSemsurfFenceWaitIoctl,
    NvDrmSemsurfFenceAttachIoctl,
    NvDrmSemsurfExportToSyncobjPointIoctl,
    NvDrmSyncobjGetSyncfdIoctl,
    NvDrmGetClientCapabilityIoctl,
    NvDrmGetCrtcCrc32Ioctl,
    NvDrmGetCrtcCrc32V2Ioctl,
    NvDrmGemExportNvkmsMemoryIoctl,
    NvDrmGemAllocNvkmsMemoryIoctl,
    NvDrmGemExportDmabufMemoryIoctl,
    NvDrmGemIdentifyObjectIoctl,
    NvDrmDmabufSupportedIoctl,
    NvDrmGetDpyIdForConnectorIdIoctl,
    NvDrmGetConnectorIdForDpyIdIoctl,
    NvDrmGrantPermissionIoctl,
    NvDrmRevokePermissionIoctl,
    NouveauAbi16IoctlGetparam,
    NouveauAbi16IoctlChannelAlloc,
    NouveauAbi16IoctlChannelFree,
    NouveauAbi16IoctlGrobjAlloc,
    NouveauAbi16IoctlNotifierobjAlloc,
    NouveauAbi16IoctlGpuobjFree,
    NouveauAbi16IoctlGetZcullInfo,
    NouveauSvmmInit,
    NouveauSvmmBind,
    NouveauGemIoctlNew,
    NouveauGemIoctlPushbuf,
    NouveauGemIoctlCpuPrep,
    NouveauGemIoctlCpuFini,
    NouveauGemIoctlInfo,
    NouveauUvmmIoctlVmInit,
    NouveauUvmmIoctlVmBind,
    NouveauExecIoctlExec,
    AsahiFileFileGetParams,
    AsahiFileFileGetTime,
    AsahiFileFileVmCreate,
    AsahiFileFileVmDestroy,
    AsahiFileFileVmBind,
    AsahiFileFileGemCreate,
    AsahiFileFileGemMmapOffset,
    AsahiFileFileGemBindObject,
    AsahiFileFileQueueCreate,
    AsahiFileFileQueueDestroy,
    AsahiFileFileSubmit,
    PanfrostIoctlSubmit,
    PanfrostIoctlWaitBo,
    PanfrostIoctlCreateBo,
    PanfrostIoctlMmapBo,
    PanfrostIoctlGetParam,
    PanfrostIoctlGetBoOffset,
    PanfrostIoctlPerfcntEnable,
    PanfrostIoctlPerfcntDump,
    PanfrostIoctlMadvise,
    PanfrostIoctlSetLabelBo,
    PanfrostIoctlJmCtxCreate,
    PanfrostIoctlJmCtxDestroy,
    PanfrostIoctlSyncBo,
    PanfrostIoctlQueryBoInfo,
    PanthorIoctlDevQuery,
    PanthorIoctlVmCreate,
    PanthorIoctlVmDestroy,
    PanthorIoctlVmBind,
    PanthorIoctlVmGetState,
    PanthorIoctlBoCreate,
    PanthorIoctlBoMmapOffset,
    PanthorIoctlGroupCreate,
    PanthorIoctlGroupDestroy,
    PanthorIoctlGroupGetState,
    PanthorIoctlTilerHeapCreate,
    PanthorIoctlTilerHeapDestroy,
    PanthorIoctlGroupSubmit,
    PanthorIoctlBoSetLabel,
    PanthorIoctlSetUserMmioOffset,
    PanthorIoctlBoSync,
    PanthorIoctlBoQueryInfo,
    V3dSubmitClIoctl,
    V3dWaitBoIoctl,
    V3dCreateBoIoctl,
    V3dMmapBoIoctl,
    V3dGetParamIoctl,
    V3dGetBoOffsetIoctl,
    V3dSubmitTfuIoctl,
    V3dSubmitCsdIoctl,
    V3dPerfmonCreateIoctl,
    V3dPerfmonDestroyIoctl,
    V3dPerfmonGetValuesIoctl,
    V3dSubmitCpuIoctl,
    V3dPerfmonGetCounterIoctl,
    V3dPerfmonSetGlobalIoctl,
    EtnavivIoctlGetParam,
    EtnavivIoctlGemNew,
    EtnavivIoctlGemInfo,
    EtnavivIoctlGemCpuPrep,
    EtnavivIoctlGemCpuFini,
    EtnavivIoctlGemSubmit,
    EtnavivIoctlWaitFence,
    EtnavivIoctlGemUserptr,
    EtnavivIoctlGemWait,
    EtnavivIoctlPmQueryDom,
    EtnavivIoctlPmQuerySig,
    LimaIoctlGetParam,
    LimaIoctlGemCreate,
    LimaIoctlGemInfo,
    LimaIoctlGemSubmit,
    LimaIoctlGemWait,
    LimaIoctlCtxCreate,
    LimaIoctlCtxFree,
    Vc4SubmitClIoctl,
    Vc4WaitSeqnoIoctl,
    Vc4WaitBoIoctl,
    Vc4CreateBoIoctl,
    Vc4MmapBoIoctl,
    Vc4CreateShaderBoIoctl,
    Vc4GetHangStateIoctl,
    Vc4GetParamIoctl,
    Vc4SetTilingIoctl,
    Vc4GetTilingIoctl,
    Vc4LabelBoIoctl,
    Vc4GemMadviseIoctl,
    Vc4PerfmonCreateIoctl,
    Vc4PerfmonDestroyIoctl,
    Vc4PerfmonGetValuesIoctl,
    VirtioGpuMapIoctl,
    VirtioGpuExecbufferIoctl,
    VirtioGpuGetparamIoctl,
    VirtioGpuResourceCreateIoctl,
    VirtioGpuResourceInfoIoctl,
    VirtioGpuTransferFromHostIoctl,
    VirtioGpuTransferToHostIoctl,
    VirtioGpuWaitIoctl,
    VirtioGpuGetCapsIoctl,
    VirtioGpuResourceCreateBlobIoctl,
    VirtioGpuContextInitIoctl,
    MsmIoctlGetParam,
    MsmIoctlSetParam,
    MsmIoctlGemNew,
    MsmIoctlGemInfo,
    MsmIoctlGemCpuPrep,
    MsmIoctlGemCpuFini,
    MsmIoctlGemSubmit,
    MsmIoctlWaitFence,
    MsmIoctlGemMadvise,
    MsmIoctlSubmitqueueNew,
    MsmIoctlSubmitqueueClose,
    MsmIoctlSubmitqueueQuery,
    MsmIoctlVmBind,
    MsmIoctlPerfcntrConfig,
}

impl Func {
    pub fn name(self) -> &'static str {
        match self {
            Func::DrmVersion => "drm_version",
            Func::DrmGetunique => "drm_getunique",
            Func::DrmGetmagic => "drm_getmagic",
            Func::DrmGetclient => "drm_getclient",
            Func::DrmGetstats => "drm_getstats",
            Func::DrmGetcap => "drm_getcap",
            Func::DrmSetclientcap => "drm_setclientcap",
            Func::DrmSetversion => "drm_setversion",
            Func::DrmInvalidOp => "drm_invalid_op",
            Func::DrmNoop => "drm_noop",
            Func::DrmAuthmagic => "drm_authmagic",
            Func::DrmSetmasterIoctl => "drm_setmaster_ioctl",
            Func::DrmDropmasterIoctl => "drm_dropmaster_ioctl",
            Func::DrmWaitVblankIoctl => "drm_wait_vblank_ioctl",
            Func::DrmGemCloseIoctl => "drm_gem_close_ioctl",
            Func::DrmGemFlinkIoctl => "drm_gem_flink_ioctl",
            Func::DrmGemOpenIoctl => "drm_gem_open_ioctl",
            Func::DrmGemChangeHandleIoctl => "drm_gem_change_handle_ioctl",
            Func::DrmModeGetresources => "drm_mode_getresources",
            Func::DrmPrimeHandleToFdIoctl => "drm_prime_handle_to_fd_ioctl",
            Func::DrmPrimeFdToHandleIoctl => "drm_prime_fd_to_handle_ioctl",
            Func::DrmSetClientName => "drm_set_client_name",
            Func::DrmModeGetplaneRes => "drm_mode_getplane_res",
            Func::DrmModeGetcrtc => "drm_mode_getcrtc",
            Func::DrmModeSetcrtc => "drm_mode_setcrtc",
            Func::DrmModeGetplane => "drm_mode_getplane",
            Func::DrmModeSetplane => "drm_mode_setplane",
            Func::DrmModeCursorIoctl => "drm_mode_cursor_ioctl",
            Func::DrmModeGammaGetIoctl => "drm_mode_gamma_get_ioctl",
            Func::DrmModeGammaSetIoctl => "drm_mode_gamma_set_ioctl",
            Func::DrmModeGetencoder => "drm_mode_getencoder",
            Func::DrmModeGetconnector => "drm_mode_getconnector",
            Func::DrmModeGetpropertyIoctl => "drm_mode_getproperty_ioctl",
            Func::DrmConnectorPropertySetIoctl => "drm_connector_property_set_ioctl",
            Func::DrmModeGetblobIoctl => "drm_mode_getblob_ioctl",
            Func::DrmModeGetfb => "drm_mode_getfb",
            Func::DrmModeGetfb2Ioctl => "drm_mode_getfb2_ioctl",
            Func::DrmModeAddfbIoctl => "drm_mode_addfb_ioctl",
            Func::DrmModeAddfb2Ioctl => "drm_mode_addfb2_ioctl",
            Func::DrmModeRmfbIoctl => "drm_mode_rmfb_ioctl",
            Func::DrmModeClosefbIoctl => "drm_mode_closefb_ioctl",
            Func::DrmModePageFlipIoctl => "drm_mode_page_flip_ioctl",
            Func::DrmModeDirtyfbIoctl => "drm_mode_dirtyfb_ioctl",
            Func::DrmModeCreateDumbIoctl => "drm_mode_create_dumb_ioctl",
            Func::DrmModeMmapDumbIoctl => "drm_mode_mmap_dumb_ioctl",
            Func::DrmModeDestroyDumbIoctl => "drm_mode_destroy_dumb_ioctl",
            Func::DrmModeObjGetPropertiesIoctl => "drm_mode_obj_get_properties_ioctl",
            Func::DrmModeObjSetPropertyIoctl => "drm_mode_obj_set_property_ioctl",
            Func::DrmModeCursor2Ioctl => "drm_mode_cursor2_ioctl",
            Func::DrmModeAtomicIoctl => "drm_mode_atomic_ioctl",
            Func::DrmModeCreateblobIoctl => "drm_mode_createblob_ioctl",
            Func::DrmModeDestroyblobIoctl => "drm_mode_destroyblob_ioctl",
            Func::DrmSyncobjCreateIoctl => "drm_syncobj_create_ioctl",
            Func::DrmSyncobjDestroyIoctl => "drm_syncobj_destroy_ioctl",
            Func::DrmSyncobjHandleToFdIoctl => "drm_syncobj_handle_to_fd_ioctl",
            Func::DrmSyncobjFdToHandleIoctl => "drm_syncobj_fd_to_handle_ioctl",
            Func::DrmSyncobjTransferIoctl => "drm_syncobj_transfer_ioctl",
            Func::DrmSyncobjWaitIoctl => "drm_syncobj_wait_ioctl",
            Func::DrmSyncobjTimelineWaitIoctl => "drm_syncobj_timeline_wait_ioctl",
            Func::DrmSyncobjEventfdIoctl => "drm_syncobj_eventfd_ioctl",
            Func::DrmSyncobjResetIoctl => "drm_syncobj_reset_ioctl",
            Func::DrmSyncobjSignalIoctl => "drm_syncobj_signal_ioctl",
            Func::DrmSyncobjTimelineSignalIoctl => "drm_syncobj_timeline_signal_ioctl",
            Func::DrmSyncobjQueryIoctl => "drm_syncobj_query_ioctl",
            Func::DrmCrtcGetSequenceIoctl => "drm_crtc_get_sequence_ioctl",
            Func::DrmCrtcQueueSequenceIoctl => "drm_crtc_queue_sequence_ioctl",
            Func::DrmModeCreateLeaseIoctl => "drm_mode_create_lease_ioctl",
            Func::DrmModeListLesseesIoctl => "drm_mode_list_lessees_ioctl",
            Func::DrmModeGetLeaseIoctl => "drm_mode_get_lease_ioctl",
            Func::DrmModeRevokeLeaseIoctl => "drm_mode_revoke_lease_ioctl",
            Func::AmdgpuGemCreateIoctl => "amdgpu_gem_create_ioctl",
            Func::AmdgpuCtxIoctl => "amdgpu_ctx_ioctl",
            Func::AmdgpuVmIoctl => "amdgpu_vm_ioctl",
            Func::AmdgpuSchedIoctl => "amdgpu_sched_ioctl",
            Func::AmdgpuBoListIoctl => "amdgpu_bo_list_ioctl",
            Func::AmdgpuCsFenceToHandleIoctl => "amdgpu_cs_fence_to_handle_ioctl",
            Func::AmdgpuGemMmapIoctl => "amdgpu_gem_mmap_ioctl",
            Func::AmdgpuGemWaitIdleIoctl => "amdgpu_gem_wait_idle_ioctl",
            Func::AmdgpuCsIoctl => "amdgpu_cs_ioctl",
            Func::AmdgpuInfoIoctl => "amdgpu_info_ioctl",
            Func::AmdgpuCsWaitIoctl => "amdgpu_cs_wait_ioctl",
            Func::AmdgpuCsWaitFencesIoctl => "amdgpu_cs_wait_fences_ioctl",
            Func::AmdgpuGemMetadataIoctl => "amdgpu_gem_metadata_ioctl",
            Func::AmdgpuGemVaIoctl => "amdgpu_gem_va_ioctl",
            Func::AmdgpuGemOpIoctl => "amdgpu_gem_op_ioctl",
            Func::AmdgpuGemUserptrIoctl => "amdgpu_gem_userptr_ioctl",
            Func::AmdgpuUserqIoctl => "amdgpu_userq_ioctl",
            Func::AmdgpuUserqSignalIoctl => "amdgpu_userq_signal_ioctl",
            Func::AmdgpuUserqWaitIoctl => "amdgpu_userq_wait_ioctl",
            Func::AmdgpuGemListHandlesIoctl => "amdgpu_gem_list_handles_ioctl",
            Func::I915GetparamIoctl => "i915_getparam_ioctl",
            Func::I915GemExecbuffer2Ioctl => "i915_gem_execbuffer2_ioctl",
            Func::I915GemRejectPinIoctl => "i915_gem_reject_pin_ioctl",
            Func::I915GemBusyIoctl => "i915_gem_busy_ioctl",
            Func::I915GemSetCachingIoctl => "i915_gem_set_caching_ioctl",
            Func::I915GemGetCachingIoctl => "i915_gem_get_caching_ioctl",
            Func::I915GemThrottleIoctl => "i915_gem_throttle_ioctl",
            Func::I915GemCreateIoctl => "i915_gem_create_ioctl",
            Func::I915GemCreateExtIoctl => "i915_gem_create_ext_ioctl",
            Func::I915GemPreadIoctl => "i915_gem_pread_ioctl",
            Func::I915GemPwriteIoctl => "i915_gem_pwrite_ioctl",
            Func::I915GemMmapIoctl => "i915_gem_mmap_ioctl",
            Func::I915GemMmapOffsetIoctl => "i915_gem_mmap_offset_ioctl",
            Func::I915GemSetDomainIoctl => "i915_gem_set_domain_ioctl",
            Func::I915GemSwFinishIoctl => "i915_gem_sw_finish_ioctl",
            Func::I915GemSetTilingIoctl => "i915_gem_set_tiling_ioctl",
            Func::I915GemGetTilingIoctl => "i915_gem_get_tiling_ioctl",
            Func::I915GemGetApertureIoctl => "i915_gem_get_aperture_ioctl",
            Func::IntelCrtcGetPipeFromCrtcIdIoctl => "intel_crtc_get_pipe_from_crtc_id_ioctl",
            Func::I915GemMadviseIoctl => "i915_gem_madvise_ioctl",
            Func::IntelOverlayPutImageIoctl => "intel_overlay_put_image_ioctl",
            Func::IntelOverlayAttrsIoctl => "intel_overlay_attrs_ioctl",
            Func::IntelSpriteSetColorkeyIoctl => "intel_sprite_set_colorkey_ioctl",
            Func::I915GemWaitIoctl => "i915_gem_wait_ioctl",
            Func::I915GemContextCreateIoctl => "i915_gem_context_create_ioctl",
            Func::I915GemContextDestroyIoctl => "i915_gem_context_destroy_ioctl",
            Func::I915RegReadIoctl => "i915_reg_read_ioctl",
            Func::I915GemContextResetStatsIoctl => "i915_gem_context_reset_stats_ioctl",
            Func::I915GemUserptrIoctl => "i915_gem_userptr_ioctl",
            Func::I915GemContextGetparamIoctl => "i915_gem_context_getparam_ioctl",
            Func::I915GemContextSetparamIoctl => "i915_gem_context_setparam_ioctl",
            Func::I915PerfOpenIoctl => "i915_perf_open_ioctl",
            Func::I915PerfAddConfigIoctl => "i915_perf_add_config_ioctl",
            Func::I915PerfRemoveConfigIoctl => "i915_perf_remove_config_ioctl",
            Func::I915QueryIoctl => "i915_query_ioctl",
            Func::I915GemVmCreateIoctl => "i915_gem_vm_create_ioctl",
            Func::I915GemVmDestroyIoctl => "i915_gem_vm_destroy_ioctl",
            Func::XeQueryIoctl => "xe_query_ioctl",
            Func::XeGemCreateIoctl => "xe_gem_create_ioctl",
            Func::XeGemMmapOffsetIoctl => "xe_gem_mmap_offset_ioctl",
            Func::XeVmCreateIoctl => "xe_vm_create_ioctl",
            Func::XeVmDestroyIoctl => "xe_vm_destroy_ioctl",
            Func::XeVmBindIoctl => "xe_vm_bind_ioctl",
            Func::XeExecIoctl => "xe_exec_ioctl",
            Func::XeExecQueueCreateIoctl => "xe_exec_queue_create_ioctl",
            Func::XeExecQueueDestroyIoctl => "xe_exec_queue_destroy_ioctl",
            Func::XeExecQueueGetPropertyIoctl => "xe_exec_queue_get_property_ioctl",
            Func::XeWaitUserFenceIoctl => "xe_wait_user_fence_ioctl",
            Func::XeObservationIoctl => "xe_observation_ioctl",
            Func::XeVmMadviseIoctl => "xe_vm_madvise_ioctl",
            Func::XeVmQueryVmasAttrsIoctl => "xe_vm_query_vmas_attrs_ioctl",
            Func::XeExecQueueSetPropertyIoctl => "xe_exec_queue_set_property_ioctl",
            Func::XeVmGetPropertyIoctl => "xe_vm_get_property_ioctl",
            Func::NvDrmGemImportNvkmsMemoryIoctl => "nv_drm_gem_import_nvkms_memory_ioctl",
            Func::NvDrmGemImportUserspaceMemoryIoctl => "nv_drm_gem_import_userspace_memory_ioctl",
            Func::NvDrmGemMapOffsetIoctl => "nv_drm_gem_map_offset_ioctl",
            Func::NvDrmGetDevInfoIoctl => "nv_drm_get_dev_info_ioctl",
            Func::NvDrmGetDrmFileUniqueIdIoctl => "nv_drm_get_drm_file_unique_id_ioctl",
            Func::NvDrmFenceSupportedIoctl => "nv_drm_fence_supported_ioctl",
            Func::NvDrmPrimeFenceContextCreateIoctl => "nv_drm_prime_fence_context_create_ioctl",
            Func::NvDrmGemPrimeFenceAttachIoctl => "nv_drm_gem_prime_fence_attach_ioctl",
            Func::NvDrmSemsurfFenceCtxCreateIoctl => "nv_drm_semsurf_fence_ctx_create_ioctl",
            Func::NvDrmSemsurfFenceCreateIoctl => "nv_drm_semsurf_fence_create_ioctl",
            Func::NvDrmSemsurfFenceWaitIoctl => "nv_drm_semsurf_fence_wait_ioctl",
            Func::NvDrmSemsurfFenceAttachIoctl => "nv_drm_semsurf_fence_attach_ioctl",
            Func::NvDrmSemsurfExportToSyncobjPointIoctl => {
                "nv_drm_semsurf_export_to_syncobj_point_ioctl"
            }
            Func::NvDrmSyncobjGetSyncfdIoctl => "nv_drm_syncobj_get_syncfd_ioctl",
            Func::NvDrmGetClientCapabilityIoctl => "nv_drm_get_client_capability_ioctl",
            Func::NvDrmGetCrtcCrc32Ioctl => "nv_drm_get_crtc_crc32_ioctl",
            Func::NvDrmGetCrtcCrc32V2Ioctl => "nv_drm_get_crtc_crc32_v2_ioctl",
            Func::NvDrmGemExportNvkmsMemoryIoctl => "nv_drm_gem_export_nvkms_memory_ioctl",
            Func::NvDrmGemAllocNvkmsMemoryIoctl => "nv_drm_gem_alloc_nvkms_memory_ioctl",
            Func::NvDrmGemExportDmabufMemoryIoctl => "nv_drm_gem_export_dmabuf_memory_ioctl",
            Func::NvDrmGemIdentifyObjectIoctl => "nv_drm_gem_identify_object_ioctl",
            Func::NvDrmDmabufSupportedIoctl => "nv_drm_dmabuf_supported_ioctl",
            Func::NvDrmGetDpyIdForConnectorIdIoctl => "nv_drm_get_dpy_id_for_connector_id_ioctl",
            Func::NvDrmGetConnectorIdForDpyIdIoctl => "nv_drm_get_connector_id_for_dpy_id_ioctl",
            Func::NvDrmGrantPermissionIoctl => "nv_drm_grant_permission_ioctl",
            Func::NvDrmRevokePermissionIoctl => "nv_drm_revoke_permission_ioctl",
            Func::NouveauAbi16IoctlGetparam => "nouveau_abi16_ioctl_getparam",
            Func::NouveauAbi16IoctlChannelAlloc => "nouveau_abi16_ioctl_channel_alloc",
            Func::NouveauAbi16IoctlChannelFree => "nouveau_abi16_ioctl_channel_free",
            Func::NouveauAbi16IoctlGrobjAlloc => "nouveau_abi16_ioctl_grobj_alloc",
            Func::NouveauAbi16IoctlNotifierobjAlloc => "nouveau_abi16_ioctl_notifierobj_alloc",
            Func::NouveauAbi16IoctlGpuobjFree => "nouveau_abi16_ioctl_gpuobj_free",
            Func::NouveauAbi16IoctlGetZcullInfo => "nouveau_abi16_ioctl_get_zcull_info",
            Func::NouveauSvmmInit => "nouveau_svmm_init",
            Func::NouveauSvmmBind => "nouveau_svmm_bind",
            Func::NouveauGemIoctlNew => "nouveau_gem_ioctl_new",
            Func::NouveauGemIoctlPushbuf => "nouveau_gem_ioctl_pushbuf",
            Func::NouveauGemIoctlCpuPrep => "nouveau_gem_ioctl_cpu_prep",
            Func::NouveauGemIoctlCpuFini => "nouveau_gem_ioctl_cpu_fini",
            Func::NouveauGemIoctlInfo => "nouveau_gem_ioctl_info",
            Func::NouveauUvmmIoctlVmInit => "nouveau_uvmm_ioctl_vm_init",
            Func::NouveauUvmmIoctlVmBind => "nouveau_uvmm_ioctl_vm_bind",
            Func::NouveauExecIoctlExec => "nouveau_exec_ioctl_exec",
            Func::AsahiFileFileGetParams => "asahi::file::File::get_params",
            Func::AsahiFileFileGetTime => "asahi::file::File::get_time",
            Func::AsahiFileFileVmCreate => "asahi::file::File::vm_create",
            Func::AsahiFileFileVmDestroy => "asahi::file::File::vm_destroy",
            Func::AsahiFileFileVmBind => "asahi::file::File::vm_bind",
            Func::AsahiFileFileGemCreate => "asahi::file::File::gem_create",
            Func::AsahiFileFileGemMmapOffset => "asahi::file::File::gem_mmap_offset",
            Func::AsahiFileFileGemBindObject => "asahi::file::File::gem_bind_object",
            Func::AsahiFileFileQueueCreate => "asahi::file::File::queue_create",
            Func::AsahiFileFileQueueDestroy => "asahi::file::File::queue_destroy",
            Func::AsahiFileFileSubmit => "asahi::file::File::submit",
            Func::PanfrostIoctlSubmit => "panfrost_ioctl_submit",
            Func::PanfrostIoctlWaitBo => "panfrost_ioctl_wait_bo",
            Func::PanfrostIoctlCreateBo => "panfrost_ioctl_create_bo",
            Func::PanfrostIoctlMmapBo => "panfrost_ioctl_mmap_bo",
            Func::PanfrostIoctlGetParam => "panfrost_ioctl_get_param",
            Func::PanfrostIoctlGetBoOffset => "panfrost_ioctl_get_bo_offset",
            Func::PanfrostIoctlPerfcntEnable => "panfrost_ioctl_perfcnt_enable",
            Func::PanfrostIoctlPerfcntDump => "panfrost_ioctl_perfcnt_dump",
            Func::PanfrostIoctlMadvise => "panfrost_ioctl_madvise",
            Func::PanfrostIoctlSetLabelBo => "panfrost_ioctl_set_label_bo",
            Func::PanfrostIoctlJmCtxCreate => "panfrost_ioctl_jm_ctx_create",
            Func::PanfrostIoctlJmCtxDestroy => "panfrost_ioctl_jm_ctx_destroy",
            Func::PanfrostIoctlSyncBo => "panfrost_ioctl_sync_bo",
            Func::PanfrostIoctlQueryBoInfo => "panfrost_ioctl_query_bo_info",
            Func::PanthorIoctlDevQuery => "panthor_ioctl_dev_query",
            Func::PanthorIoctlVmCreate => "panthor_ioctl_vm_create",
            Func::PanthorIoctlVmDestroy => "panthor_ioctl_vm_destroy",
            Func::PanthorIoctlVmBind => "panthor_ioctl_vm_bind",
            Func::PanthorIoctlVmGetState => "panthor_ioctl_vm_get_state",
            Func::PanthorIoctlBoCreate => "panthor_ioctl_bo_create",
            Func::PanthorIoctlBoMmapOffset => "panthor_ioctl_bo_mmap_offset",
            Func::PanthorIoctlGroupCreate => "panthor_ioctl_group_create",
            Func::PanthorIoctlGroupDestroy => "panthor_ioctl_group_destroy",
            Func::PanthorIoctlGroupGetState => "panthor_ioctl_group_get_state",
            Func::PanthorIoctlTilerHeapCreate => "panthor_ioctl_tiler_heap_create",
            Func::PanthorIoctlTilerHeapDestroy => "panthor_ioctl_tiler_heap_destroy",
            Func::PanthorIoctlGroupSubmit => "panthor_ioctl_group_submit",
            Func::PanthorIoctlBoSetLabel => "panthor_ioctl_bo_set_label",
            Func::PanthorIoctlSetUserMmioOffset => "panthor_ioctl_set_user_mmio_offset",
            Func::PanthorIoctlBoSync => "panthor_ioctl_bo_sync",
            Func::PanthorIoctlBoQueryInfo => "panthor_ioctl_bo_query_info",
            Func::V3dSubmitClIoctl => "v3d_submit_cl_ioctl",
            Func::V3dWaitBoIoctl => "v3d_wait_bo_ioctl",
            Func::V3dCreateBoIoctl => "v3d_create_bo_ioctl",
            Func::V3dMmapBoIoctl => "v3d_mmap_bo_ioctl",
            Func::V3dGetParamIoctl => "v3d_get_param_ioctl",
            Func::V3dGetBoOffsetIoctl => "v3d_get_bo_offset_ioctl",
            Func::V3dSubmitTfuIoctl => "v3d_submit_tfu_ioctl",
            Func::V3dSubmitCsdIoctl => "v3d_submit_csd_ioctl",
            Func::V3dPerfmonCreateIoctl => "v3d_perfmon_create_ioctl",
            Func::V3dPerfmonDestroyIoctl => "v3d_perfmon_destroy_ioctl",
            Func::V3dPerfmonGetValuesIoctl => "v3d_perfmon_get_values_ioctl",
            Func::V3dSubmitCpuIoctl => "v3d_submit_cpu_ioctl",
            Func::V3dPerfmonGetCounterIoctl => "v3d_perfmon_get_counter_ioctl",
            Func::V3dPerfmonSetGlobalIoctl => "v3d_perfmon_set_global_ioctl",
            Func::EtnavivIoctlGetParam => "etnaviv_ioctl_get_param",
            Func::EtnavivIoctlGemNew => "etnaviv_ioctl_gem_new",
            Func::EtnavivIoctlGemInfo => "etnaviv_ioctl_gem_info",
            Func::EtnavivIoctlGemCpuPrep => "etnaviv_ioctl_gem_cpu_prep",
            Func::EtnavivIoctlGemCpuFini => "etnaviv_ioctl_gem_cpu_fini",
            Func::EtnavivIoctlGemSubmit => "etnaviv_ioctl_gem_submit",
            Func::EtnavivIoctlWaitFence => "etnaviv_ioctl_wait_fence",
            Func::EtnavivIoctlGemUserptr => "etnaviv_ioctl_gem_userptr",
            Func::EtnavivIoctlGemWait => "etnaviv_ioctl_gem_wait",
            Func::EtnavivIoctlPmQueryDom => "etnaviv_ioctl_pm_query_dom",
            Func::EtnavivIoctlPmQuerySig => "etnaviv_ioctl_pm_query_sig",
            Func::LimaIoctlGetParam => "lima_ioctl_get_param",
            Func::LimaIoctlGemCreate => "lima_ioctl_gem_create",
            Func::LimaIoctlGemInfo => "lima_ioctl_gem_info",
            Func::LimaIoctlGemSubmit => "lima_ioctl_gem_submit",
            Func::LimaIoctlGemWait => "lima_ioctl_gem_wait",
            Func::LimaIoctlCtxCreate => "lima_ioctl_ctx_create",
            Func::LimaIoctlCtxFree => "lima_ioctl_ctx_free",
            Func::Vc4SubmitClIoctl => "vc4_submit_cl_ioctl",
            Func::Vc4WaitSeqnoIoctl => "vc4_wait_seqno_ioctl",
            Func::Vc4WaitBoIoctl => "vc4_wait_bo_ioctl",
            Func::Vc4CreateBoIoctl => "vc4_create_bo_ioctl",
            Func::Vc4MmapBoIoctl => "vc4_mmap_bo_ioctl",
            Func::Vc4CreateShaderBoIoctl => "vc4_create_shader_bo_ioctl",
            Func::Vc4GetHangStateIoctl => "vc4_get_hang_state_ioctl",
            Func::Vc4GetParamIoctl => "vc4_get_param_ioctl",
            Func::Vc4SetTilingIoctl => "vc4_set_tiling_ioctl",
            Func::Vc4GetTilingIoctl => "vc4_get_tiling_ioctl",
            Func::Vc4LabelBoIoctl => "vc4_label_bo_ioctl",
            Func::Vc4GemMadviseIoctl => "vc4_gem_madvise_ioctl",
            Func::Vc4PerfmonCreateIoctl => "vc4_perfmon_create_ioctl",
            Func::Vc4PerfmonDestroyIoctl => "vc4_perfmon_destroy_ioctl",
            Func::Vc4PerfmonGetValuesIoctl => "vc4_perfmon_get_values_ioctl",
            Func::VirtioGpuMapIoctl => "virtio_gpu_map_ioctl",
            Func::VirtioGpuExecbufferIoctl => "virtio_gpu_execbuffer_ioctl",
            Func::VirtioGpuGetparamIoctl => "virtio_gpu_getparam_ioctl",
            Func::VirtioGpuResourceCreateIoctl => "virtio_gpu_resource_create_ioctl",
            Func::VirtioGpuResourceInfoIoctl => "virtio_gpu_resource_info_ioctl",
            Func::VirtioGpuTransferFromHostIoctl => "virtio_gpu_transfer_from_host_ioctl",
            Func::VirtioGpuTransferToHostIoctl => "virtio_gpu_transfer_to_host_ioctl",
            Func::VirtioGpuWaitIoctl => "virtio_gpu_wait_ioctl",
            Func::VirtioGpuGetCapsIoctl => "virtio_gpu_get_caps_ioctl",
            Func::VirtioGpuResourceCreateBlobIoctl => "virtio_gpu_resource_create_blob_ioctl",
            Func::VirtioGpuContextInitIoctl => "virtio_gpu_context_init_ioctl",
            Func::MsmIoctlGetParam => "msm_ioctl_get_param",
            Func::MsmIoctlSetParam => "msm_ioctl_set_param",
            Func::MsmIoctlGemNew => "msm_ioctl_gem_new",
            Func::MsmIoctlGemInfo => "msm_ioctl_gem_info",
            Func::MsmIoctlGemCpuPrep => "msm_ioctl_gem_cpu_prep",
            Func::MsmIoctlGemCpuFini => "msm_ioctl_gem_cpu_fini",
            Func::MsmIoctlGemSubmit => "msm_ioctl_gem_submit",
            Func::MsmIoctlWaitFence => "msm_ioctl_wait_fence",
            Func::MsmIoctlGemMadvise => "msm_ioctl_gem_madvise",
            Func::MsmIoctlSubmitqueueNew => "msm_ioctl_submitqueue_new",
            Func::MsmIoctlSubmitqueueClose => "msm_ioctl_submitqueue_close",
            Func::MsmIoctlSubmitqueueQuery => "msm_ioctl_submitqueue_query",
            Func::MsmIoctlVmBind => "msm_ioctl_vm_bind",
            Func::MsmIoctlPerfcntrConfig => "msm_ioctl_perfcntr_config",
        }
    }
}

pub struct IoctlDesc {
    pub cmd: u32,
    pub func: Func,
    pub flags: u32,
    pub since: Option<KernelVersion>,
    pub until: Option<KernelVersion>,
}

impl IoctlDesc {
    pub fn exists(&self, version: KernelVersion) -> bool {
        self.since.is_none_or(|since| since <= version)
            && self.until.is_none_or(|until| version < until)
    }
}

const fn def(cmd: u32, func: Func, flags: u32) -> IoctlDesc {
    IoctlDesc {
        cmd,
        func,
        flags,
        since: None,
        until: None,
    }
}

const fn since(desc: IoctlDesc, version: KernelVersion) -> IoctlDesc {
    IoctlDesc {
        since: Some(version),
        ..desc
    }
}

const fn until(desc: IoctlDesc, version: KernelVersion) -> IoctlDesc {
    IoctlDesc {
        until: Some(version),
        ..desc
    }
}

pub static GENERIC: &[IoctlDesc] = &[
    def(DRM_IOCTL_VERSION, Func::DrmVersion, DRM_RENDER_ALLOW),
    def(DRM_IOCTL_GET_UNIQUE, Func::DrmGetunique, 0),
    def(DRM_IOCTL_GET_MAGIC, Func::DrmGetmagic, 0),
    def(DRM_IOCTL_GET_CLIENT, Func::DrmGetclient, 0),
    def(DRM_IOCTL_GET_STATS, Func::DrmGetstats, 0),
    def(DRM_IOCTL_GET_CAP, Func::DrmGetcap, DRM_RENDER_ALLOW),
    def(DRM_IOCTL_SET_CLIENT_CAP, Func::DrmSetclientcap, 0),
    def(DRM_IOCTL_SET_VERSION, Func::DrmSetversion, DRM_MASTER),
    def(
        DRM_IOCTL_SET_UNIQUE,
        Func::DrmInvalidOp,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(
        DRM_IOCTL_BLOCK,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(
        DRM_IOCTL_UNBLOCK,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(DRM_IOCTL_AUTH_MAGIC, Func::DrmAuthmagic, DRM_MASTER),
    def(DRM_IOCTL_SET_MASTER, Func::DrmSetmasterIoctl, 0),
    def(DRM_IOCTL_DROP_MASTER, Func::DrmDropmasterIoctl, 0),
    def(
        DRM_IOCTL_ADD_DRAW,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(
        DRM_IOCTL_RM_DRAW,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(DRM_IOCTL_FINISH, Func::DrmNoop, DRM_AUTH),
    def(DRM_IOCTL_WAIT_VBLANK, Func::DrmWaitVblankIoctl, 0),
    def(
        DRM_IOCTL_UPDATE_DRAW,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(
        DRM_IOCTL_GEM_CLOSE,
        Func::DrmGemCloseIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(DRM_IOCTL_GEM_FLINK, Func::DrmGemFlinkIoctl, DRM_AUTH),
    def(DRM_IOCTL_GEM_OPEN, Func::DrmGemOpenIoctl, DRM_AUTH),
    until(
        since(
            def(
                DRM_IOCTL_GEM_CHANGE_HANDLE,
                Func::DrmGemChangeHandleIoctl,
                DRM_RENDER_ALLOW,
            ),
            KernelVersion::V6_18,
        ),
        KernelVersion::V7_1,
    ),
    since(
        def(
            DRM_IOCTL_GEM_CHANGE_HANDLE,
            Func::DrmInvalidOp,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V7_1,
    ),
    def(DRM_IOCTL_MODE_GETRESOURCES, Func::DrmModeGetresources, 0),
    def(
        DRM_IOCTL_PRIME_HANDLE_TO_FD,
        Func::DrmPrimeHandleToFdIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PRIME_FD_TO_HANDLE,
        Func::DrmPrimeFdToHandleIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_SET_CLIENT_NAME,
        Func::DrmSetClientName,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_MODE_GETPLANERESOURCES,
        Func::DrmModeGetplaneRes,
        0,
    ),
    def(DRM_IOCTL_MODE_GETCRTC, Func::DrmModeGetcrtc, 0),
    def(DRM_IOCTL_MODE_SETCRTC, Func::DrmModeSetcrtc, DRM_MASTER),
    def(DRM_IOCTL_MODE_GETPLANE, Func::DrmModeGetplane, 0),
    def(DRM_IOCTL_MODE_SETPLANE, Func::DrmModeSetplane, DRM_MASTER),
    def(DRM_IOCTL_MODE_CURSOR, Func::DrmModeCursorIoctl, DRM_MASTER),
    def(DRM_IOCTL_MODE_GETGAMMA, Func::DrmModeGammaGetIoctl, 0),
    def(
        DRM_IOCTL_MODE_SETGAMMA,
        Func::DrmModeGammaSetIoctl,
        DRM_MASTER,
    ),
    def(DRM_IOCTL_MODE_GETENCODER, Func::DrmModeGetencoder, 0),
    def(DRM_IOCTL_MODE_GETCONNECTOR, Func::DrmModeGetconnector, 0),
    def(DRM_IOCTL_MODE_ATTACHMODE, Func::DrmNoop, DRM_MASTER),
    def(DRM_IOCTL_MODE_DETACHMODE, Func::DrmNoop, DRM_MASTER),
    def(DRM_IOCTL_MODE_GETPROPERTY, Func::DrmModeGetpropertyIoctl, 0),
    def(
        DRM_IOCTL_MODE_SETPROPERTY,
        Func::DrmConnectorPropertySetIoctl,
        DRM_MASTER,
    ),
    def(DRM_IOCTL_MODE_GETPROPBLOB, Func::DrmModeGetblobIoctl, 0),
    def(DRM_IOCTL_MODE_GETFB, Func::DrmModeGetfb, 0),
    def(DRM_IOCTL_MODE_GETFB2, Func::DrmModeGetfb2Ioctl, 0),
    def(DRM_IOCTL_MODE_ADDFB, Func::DrmModeAddfbIoctl, 0),
    def(DRM_IOCTL_MODE_ADDFB2, Func::DrmModeAddfb2Ioctl, 0),
    def(DRM_IOCTL_MODE_RMFB, Func::DrmModeRmfbIoctl, 0),
    def(DRM_IOCTL_MODE_CLOSEFB, Func::DrmModeClosefbIoctl, 0),
    def(
        DRM_IOCTL_MODE_PAGE_FLIP,
        Func::DrmModePageFlipIoctl,
        DRM_MASTER,
    ),
    def(
        DRM_IOCTL_MODE_DIRTYFB,
        Func::DrmModeDirtyfbIoctl,
        DRM_MASTER,
    ),
    def(DRM_IOCTL_MODE_CREATE_DUMB, Func::DrmModeCreateDumbIoctl, 0),
    def(DRM_IOCTL_MODE_MAP_DUMB, Func::DrmModeMmapDumbIoctl, 0),
    def(
        DRM_IOCTL_MODE_DESTROY_DUMB,
        Func::DrmModeDestroyDumbIoctl,
        0,
    ),
    def(
        DRM_IOCTL_MODE_OBJ_GETPROPERTIES,
        Func::DrmModeObjGetPropertiesIoctl,
        0,
    ),
    def(
        DRM_IOCTL_MODE_OBJ_SETPROPERTY,
        Func::DrmModeObjSetPropertyIoctl,
        DRM_MASTER,
    ),
    def(
        DRM_IOCTL_MODE_CURSOR2,
        Func::DrmModeCursor2Ioctl,
        DRM_MASTER,
    ),
    def(DRM_IOCTL_MODE_ATOMIC, Func::DrmModeAtomicIoctl, DRM_MASTER),
    def(
        DRM_IOCTL_MODE_CREATEPROPBLOB,
        Func::DrmModeCreateblobIoctl,
        0,
    ),
    def(
        DRM_IOCTL_MODE_DESTROYPROPBLOB,
        Func::DrmModeDestroyblobIoctl,
        0,
    ),
    def(
        DRM_IOCTL_SYNCOBJ_CREATE,
        Func::DrmSyncobjCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_SYNCOBJ_DESTROY,
        Func::DrmSyncobjDestroyIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_SYNCOBJ_HANDLE_TO_FD,
        Func::DrmSyncobjHandleToFdIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_SYNCOBJ_FD_TO_HANDLE,
        Func::DrmSyncobjFdToHandleIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_SYNCOBJ_TRANSFER,
        Func::DrmSyncobjTransferIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_SYNCOBJ_WAIT,
        Func::DrmSyncobjWaitIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_SYNCOBJ_TIMELINE_WAIT,
        Func::DrmSyncobjTimelineWaitIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_SYNCOBJ_EVENTFD,
        Func::DrmSyncobjEventfdIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_SYNCOBJ_RESET,
        Func::DrmSyncobjResetIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_SYNCOBJ_SIGNAL,
        Func::DrmSyncobjSignalIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_SYNCOBJ_TIMELINE_SIGNAL,
        Func::DrmSyncobjTimelineSignalIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_SYNCOBJ_QUERY,
        Func::DrmSyncobjQueryIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_CRTC_GET_SEQUENCE,
        Func::DrmCrtcGetSequenceIoctl,
        0,
    ),
    def(
        DRM_IOCTL_CRTC_QUEUE_SEQUENCE,
        Func::DrmCrtcQueueSequenceIoctl,
        0,
    ),
    def(
        DRM_IOCTL_MODE_CREATE_LEASE,
        Func::DrmModeCreateLeaseIoctl,
        DRM_MASTER,
    ),
    def(
        DRM_IOCTL_MODE_LIST_LESSEES,
        Func::DrmModeListLesseesIoctl,
        DRM_MASTER,
    ),
    def(
        DRM_IOCTL_MODE_GET_LEASE,
        Func::DrmModeGetLeaseIoctl,
        DRM_MASTER,
    ),
    def(
        DRM_IOCTL_MODE_REVOKE_LEASE,
        Func::DrmModeRevokeLeaseIoctl,
        DRM_MASTER,
    ),
];

pub static AMDGPU: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_AMDGPU_GEM_CREATE,
        Func::AmdgpuGemCreateIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_CTX,
        Func::AmdgpuCtxIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_VM,
        Func::AmdgpuVmIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(DRM_IOCTL_AMDGPU_SCHED, Func::AmdgpuSchedIoctl, DRM_MASTER),
    def(
        DRM_IOCTL_AMDGPU_BO_LIST,
        Func::AmdgpuBoListIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_FENCE_TO_HANDLE,
        Func::AmdgpuCsFenceToHandleIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_GEM_MMAP,
        Func::AmdgpuGemMmapIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_GEM_WAIT_IDLE,
        Func::AmdgpuGemWaitIdleIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_CS,
        Func::AmdgpuCsIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_INFO,
        Func::AmdgpuInfoIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_WAIT_CS,
        Func::AmdgpuCsWaitIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_WAIT_FENCES,
        Func::AmdgpuCsWaitFencesIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_GEM_METADATA,
        Func::AmdgpuGemMetadataIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_GEM_VA,
        Func::AmdgpuGemVaIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_GEM_OP,
        Func::AmdgpuGemOpIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_AMDGPU_GEM_USERPTR,
        Func::AmdgpuGemUserptrIoctl,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    since(
        def(
            DRM_IOCTL_AMDGPU_USERQ,
            Func::AmdgpuUserqIoctl,
            DRM_AUTH | DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_16,
    ),
    since(
        def(
            DRM_IOCTL_AMDGPU_USERQ_SIGNAL,
            Func::AmdgpuUserqSignalIoctl,
            DRM_AUTH | DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_16,
    ),
    since(
        def(
            DRM_IOCTL_AMDGPU_USERQ_WAIT,
            Func::AmdgpuUserqWaitIoctl,
            DRM_AUTH | DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_16,
    ),
    since(
        def(
            DRM_IOCTL_AMDGPU_GEM_LIST_HANDLES,
            Func::AmdgpuGemListHandlesIoctl,
            DRM_AUTH | DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_18,
    ),
];

pub static I915: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_I915_INIT,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(DRM_IOCTL_I915_FLUSH, Func::DrmNoop, DRM_AUTH),
    def(DRM_IOCTL_I915_FLIP, Func::DrmNoop, DRM_AUTH),
    def(DRM_IOCTL_I915_BATCHBUFFER, Func::DrmNoop, DRM_AUTH),
    def(DRM_IOCTL_I915_IRQ_EMIT, Func::DrmNoop, DRM_AUTH),
    def(DRM_IOCTL_I915_IRQ_WAIT, Func::DrmNoop, DRM_AUTH),
    def(
        DRM_IOCTL_I915_GETPARAM,
        Func::I915GetparamIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_SETPARAM,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(DRM_IOCTL_I915_ALLOC, Func::DrmNoop, DRM_AUTH),
    def(DRM_IOCTL_I915_FREE, Func::DrmNoop, DRM_AUTH),
    def(
        DRM_IOCTL_I915_INIT_HEAP,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(DRM_IOCTL_I915_CMDBUFFER, Func::DrmNoop, DRM_AUTH),
    def(
        DRM_IOCTL_I915_DESTROY_HEAP,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(
        DRM_IOCTL_I915_SET_VBLANK_PIPE,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(DRM_IOCTL_I915_GET_VBLANK_PIPE, Func::DrmNoop, DRM_AUTH),
    def(DRM_IOCTL_I915_VBLANK_SWAP, Func::DrmNoop, DRM_AUTH),
    def(
        DRM_IOCTL_I915_HWS_ADDR,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(
        DRM_IOCTL_I915_GEM_INIT,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(DRM_IOCTL_I915_GEM_EXECBUFFER, Func::DrmInvalidOp, DRM_AUTH),
    def(
        DRM_IOCTL_I915_GEM_EXECBUFFER2_WR,
        Func::I915GemExecbuffer2Ioctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_PIN,
        Func::I915GemRejectPinIoctl,
        DRM_AUTH | DRM_ROOT_ONLY,
    ),
    def(
        DRM_IOCTL_I915_GEM_UNPIN,
        Func::I915GemRejectPinIoctl,
        DRM_AUTH | DRM_ROOT_ONLY,
    ),
    def(
        DRM_IOCTL_I915_GEM_BUSY,
        Func::I915GemBusyIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_SET_CACHING,
        Func::I915GemSetCachingIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_GET_CACHING,
        Func::I915GemGetCachingIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_THROTTLE,
        Func::I915GemThrottleIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_ENTERVT,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(
        DRM_IOCTL_I915_GEM_LEAVEVT,
        Func::DrmNoop,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(
        DRM_IOCTL_I915_GEM_CREATE,
        Func::I915GemCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_CREATE_EXT,
        Func::I915GemCreateExtIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_PREAD,
        Func::I915GemPreadIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_PWRITE,
        Func::I915GemPwriteIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_MMAP,
        Func::I915GemMmapIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_MMAP_OFFSET,
        Func::I915GemMmapOffsetIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_SET_DOMAIN,
        Func::I915GemSetDomainIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_SW_FINISH,
        Func::I915GemSwFinishIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_SET_TILING,
        Func::I915GemSetTilingIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_GET_TILING,
        Func::I915GemGetTilingIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_GET_APERTURE,
        Func::I915GemGetApertureIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GET_PIPE_FROM_CRTC_ID,
        Func::IntelCrtcGetPipeFromCrtcIdIoctl,
        0,
    ),
    def(
        DRM_IOCTL_I915_GEM_MADVISE,
        Func::I915GemMadviseIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_OVERLAY_PUT_IMAGE,
        Func::IntelOverlayPutImageIoctl,
        DRM_MASTER,
    ),
    def(
        DRM_IOCTL_I915_OVERLAY_ATTRS,
        Func::IntelOverlayAttrsIoctl,
        DRM_MASTER,
    ),
    def(
        DRM_IOCTL_I915_SET_SPRITE_COLORKEY,
        Func::IntelSpriteSetColorkeyIoctl,
        DRM_MASTER,
    ),
    def(
        DRM_IOCTL_I915_GET_SPRITE_COLORKEY,
        Func::DrmNoop,
        DRM_MASTER,
    ),
    def(
        DRM_IOCTL_I915_GEM_WAIT,
        Func::I915GemWaitIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_CONTEXT_CREATE_EXT,
        Func::I915GemContextCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_CONTEXT_DESTROY,
        Func::I915GemContextDestroyIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_REG_READ,
        Func::I915RegReadIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GET_RESET_STATS,
        Func::I915GemContextResetStatsIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_USERPTR,
        Func::I915GemUserptrIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_CONTEXT_GETPARAM,
        Func::I915GemContextGetparamIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_CONTEXT_SETPARAM,
        Func::I915GemContextSetparamIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_PERF_OPEN,
        Func::I915PerfOpenIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_PERF_ADD_CONFIG,
        Func::I915PerfAddConfigIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_PERF_REMOVE_CONFIG,
        Func::I915PerfRemoveConfigIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(DRM_IOCTL_I915_QUERY, Func::I915QueryIoctl, DRM_RENDER_ALLOW),
    def(
        DRM_IOCTL_I915_GEM_VM_CREATE,
        Func::I915GemVmCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_I915_GEM_VM_DESTROY,
        Func::I915GemVmDestroyIoctl,
        DRM_RENDER_ALLOW,
    ),
];

pub static XE: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_XE_DEVICE_QUERY,
        Func::XeQueryIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_XE_GEM_CREATE,
        Func::XeGemCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_XE_GEM_MMAP_OFFSET,
        Func::XeGemMmapOffsetIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_XE_VM_CREATE,
        Func::XeVmCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_XE_VM_DESTROY,
        Func::XeVmDestroyIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(DRM_IOCTL_XE_VM_BIND, Func::XeVmBindIoctl, DRM_RENDER_ALLOW),
    def(DRM_IOCTL_XE_EXEC, Func::XeExecIoctl, DRM_RENDER_ALLOW),
    def(
        DRM_IOCTL_XE_EXEC_QUEUE_CREATE,
        Func::XeExecQueueCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_XE_EXEC_QUEUE_DESTROY,
        Func::XeExecQueueDestroyIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_XE_EXEC_QUEUE_GET_PROPERTY,
        Func::XeExecQueueGetPropertyIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_XE_WAIT_USER_FENCE,
        Func::XeWaitUserFenceIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_XE_OBSERVATION,
        Func::XeObservationIoctl,
        DRM_RENDER_ALLOW,
    ),
    since(
        def(
            DRM_IOCTL_XE_MADVISE,
            Func::XeVmMadviseIoctl,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_18,
    ),
    since(
        def(
            DRM_IOCTL_XE_VM_QUERY_MEM_RANGE_ATTRS,
            Func::XeVmQueryVmasAttrsIoctl,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_18,
    ),
    since(
        def(
            DRM_IOCTL_XE_EXEC_QUEUE_SET_PROPERTY,
            Func::XeExecQueueSetPropertyIoctl,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V7_0,
    ),
    since(
        def(
            DRM_IOCTL_XE_VM_GET_PROPERTY,
            Func::XeVmGetPropertyIoctl,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V7_1,
    ),
];

pub static NVIDIA_DRM: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_NVIDIA_GEM_IMPORT_NVKMS_MEMORY,
        Func::NvDrmGemImportNvkmsMemoryIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GEM_IMPORT_USERSPACE_MEMORY,
        Func::NvDrmGemImportUserspaceMemoryIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GEM_MAP_OFFSET,
        Func::NvDrmGemMapOffsetIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GET_DEV_INFO,
        Func::NvDrmGetDevInfoIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GET_DRM_FILE_UNIQUE_ID,
        Func::NvDrmGetDrmFileUniqueIdIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_FENCE_SUPPORTED,
        Func::NvDrmFenceSupportedIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_PRIME_FENCE_CONTEXT_CREATE,
        Func::NvDrmPrimeFenceContextCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GEM_PRIME_FENCE_ATTACH,
        Func::NvDrmGemPrimeFenceAttachIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_SEMSURF_FENCE_CTX_CREATE,
        Func::NvDrmSemsurfFenceCtxCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_SEMSURF_FENCE_CREATE,
        Func::NvDrmSemsurfFenceCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_SEMSURF_FENCE_WAIT,
        Func::NvDrmSemsurfFenceWaitIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_SEMSURF_FENCE_ATTACH,
        Func::NvDrmSemsurfFenceAttachIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_SEMSURF_EXPORT_TO_SYNCOBJ_POINT,
        Func::NvDrmSemsurfExportToSyncobjPointIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_SYNCOBJ_GET_SYNCFD,
        Func::NvDrmSyncobjGetSyncfdIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GET_CLIENT_CAPABILITY,
        Func::NvDrmGetClientCapabilityIoctl,
        0,
    ),
    def(
        DRM_IOCTL_NVIDIA_GET_CRTC_CRC32,
        Func::NvDrmGetCrtcCrc32Ioctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GET_CRTC_CRC32_V2,
        Func::NvDrmGetCrtcCrc32V2Ioctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GEM_EXPORT_NVKMS_MEMORY,
        Func::NvDrmGemExportNvkmsMemoryIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GEM_ALLOC_NVKMS_MEMORY,
        Func::NvDrmGemAllocNvkmsMemoryIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GEM_EXPORT_DMABUF_MEMORY,
        Func::NvDrmGemExportDmabufMemoryIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GEM_IDENTIFY_OBJECT,
        Func::NvDrmGemIdentifyObjectIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_DMABUF_SUPPORTED,
        Func::NvDrmDmabufSupportedIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GET_DPY_ID_FOR_CONNECTOR_ID,
        Func::NvDrmGetDpyIdForConnectorIdIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GET_CONNECTOR_ID_FOR_DPY_ID,
        Func::NvDrmGetConnectorIdForDpyIdIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NVIDIA_GRANT_PERMISSIONS,
        Func::NvDrmGrantPermissionIoctl,
        DRM_MASTER,
    ),
    def(
        DRM_IOCTL_NVIDIA_REVOKE_PERMISSIONS,
        Func::NvDrmRevokePermissionIoctl,
        DRM_MASTER,
    ),
];

pub static NOUVEAU: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_NOUVEAU_GETPARAM,
        Func::NouveauAbi16IoctlGetparam,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_SETPARAM,
        Func::DrmInvalidOp,
        DRM_AUTH | DRM_MASTER | DRM_ROOT_ONLY,
    ),
    def(
        DRM_IOCTL_NOUVEAU_CHANNEL_ALLOC,
        Func::NouveauAbi16IoctlChannelAlloc,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_CHANNEL_FREE,
        Func::NouveauAbi16IoctlChannelFree,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_GROBJ_ALLOC,
        Func::NouveauAbi16IoctlGrobjAlloc,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_NOTIFIEROBJ_ALLOC,
        Func::NouveauAbi16IoctlNotifierobjAlloc,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_GPUOBJ_FREE,
        Func::NouveauAbi16IoctlGpuobjFree,
        DRM_RENDER_ALLOW,
    ),
    since(
        def(
            DRM_IOCTL_NOUVEAU_GET_ZCULL_INFO,
            Func::NouveauAbi16IoctlGetZcullInfo,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V7_1,
    ),
    def(
        DRM_IOCTL_NOUVEAU_SVM_INIT,
        Func::NouveauSvmmInit,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_SVM_BIND,
        Func::NouveauSvmmBind,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_GEM_NEW,
        Func::NouveauGemIoctlNew,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_GEM_PUSHBUF,
        Func::NouveauGemIoctlPushbuf,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_GEM_CPU_PREP,
        Func::NouveauGemIoctlCpuPrep,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_GEM_CPU_FINI,
        Func::NouveauGemIoctlCpuFini,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_GEM_INFO,
        Func::NouveauGemIoctlInfo,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_VM_INIT,
        Func::NouveauUvmmIoctlVmInit,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_VM_BIND,
        Func::NouveauUvmmIoctlVmBind,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_NOUVEAU_EXEC,
        Func::NouveauExecIoctlExec,
        DRM_RENDER_ALLOW,
    ),
];

pub static MSM: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_MSM_GET_PARAM,
        Func::MsmIoctlGetParam,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_MSM_SET_PARAM,
        Func::MsmIoctlSetParam,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_MSM_GEM_NEW,
        Func::MsmIoctlGemNew,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_MSM_GEM_INFO,
        Func::MsmIoctlGemInfo,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_MSM_GEM_CPU_PREP,
        Func::MsmIoctlGemCpuPrep,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_MSM_GEM_CPU_FINI,
        Func::MsmIoctlGemCpuFini,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_MSM_GEM_SUBMIT,
        Func::MsmIoctlGemSubmit,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_MSM_WAIT_FENCE,
        Func::MsmIoctlWaitFence,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_MSM_GEM_MADVISE,
        Func::MsmIoctlGemMadvise,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_MSM_SUBMITQUEUE_NEW,
        Func::MsmIoctlSubmitqueueNew,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_MSM_SUBMITQUEUE_CLOSE,
        Func::MsmIoctlSubmitqueueClose,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_MSM_SUBMITQUEUE_QUERY,
        Func::MsmIoctlSubmitqueueQuery,
        DRM_RENDER_ALLOW,
    ),
    since(
        def(
            DRM_IOCTL_MSM_VM_BIND,
            Func::MsmIoctlVmBind,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_17,
    ),
    since(
        def(
            DRM_IOCTL_MSM_PERFCNTR_CONFIG,
            Func::MsmIoctlPerfcntrConfig,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V7_2,
    ),
];

pub static ASAHI: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_ASAHI_GET_PARAMS,
        Func::AsahiFileFileGetParams,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ASAHI_GET_TIME,
        Func::AsahiFileFileGetTime,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ASAHI_VM_CREATE,
        Func::AsahiFileFileVmCreate,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ASAHI_VM_DESTROY,
        Func::AsahiFileFileVmDestroy,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ASAHI_VM_BIND,
        Func::AsahiFileFileVmBind,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ASAHI_GEM_CREATE,
        Func::AsahiFileFileGemCreate,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ASAHI_GEM_MMAP_OFFSET,
        Func::AsahiFileFileGemMmapOffset,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ASAHI_GEM_BIND_OBJECT,
        Func::AsahiFileFileGemBindObject,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ASAHI_QUEUE_CREATE,
        Func::AsahiFileFileQueueCreate,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ASAHI_QUEUE_DESTROY,
        Func::AsahiFileFileQueueDestroy,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ASAHI_SUBMIT,
        Func::AsahiFileFileSubmit,
        DRM_AUTH | DRM_RENDER_ALLOW,
    ),
];

pub static PANFROST: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_PANFROST_SUBMIT,
        Func::PanfrostIoctlSubmit,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANFROST_WAIT_BO,
        Func::PanfrostIoctlWaitBo,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANFROST_CREATE_BO,
        Func::PanfrostIoctlCreateBo,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANFROST_MMAP_BO,
        Func::PanfrostIoctlMmapBo,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANFROST_GET_PARAM,
        Func::PanfrostIoctlGetParam,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANFROST_GET_BO_OFFSET,
        Func::PanfrostIoctlGetBoOffset,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANFROST_PERFCNT_ENABLE,
        Func::PanfrostIoctlPerfcntEnable,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANFROST_PERFCNT_DUMP,
        Func::PanfrostIoctlPerfcntDump,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANFROST_MADVISE,
        Func::PanfrostIoctlMadvise,
        DRM_RENDER_ALLOW,
    ),
    since(
        def(
            DRM_IOCTL_PANFROST_SET_LABEL_BO,
            Func::PanfrostIoctlSetLabelBo,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_17,
    ),
    since(
        def(
            DRM_IOCTL_PANFROST_JM_CTX_CREATE,
            Func::PanfrostIoctlJmCtxCreate,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_19,
    ),
    since(
        def(
            DRM_IOCTL_PANFROST_JM_CTX_DESTROY,
            Func::PanfrostIoctlJmCtxDestroy,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_19,
    ),
    since(
        def(
            DRM_IOCTL_PANFROST_SYNC_BO,
            Func::PanfrostIoctlSyncBo,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V7_0,
    ),
    since(
        def(
            DRM_IOCTL_PANFROST_QUERY_BO_INFO,
            Func::PanfrostIoctlQueryBoInfo,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V7_0,
    ),
];

pub static PANTHOR: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_PANTHOR_DEV_QUERY,
        Func::PanthorIoctlDevQuery,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANTHOR_VM_CREATE,
        Func::PanthorIoctlVmCreate,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANTHOR_VM_DESTROY,
        Func::PanthorIoctlVmDestroy,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANTHOR_VM_BIND,
        Func::PanthorIoctlVmBind,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANTHOR_VM_GET_STATE,
        Func::PanthorIoctlVmGetState,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANTHOR_BO_CREATE,
        Func::PanthorIoctlBoCreate,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANTHOR_BO_MMAP_OFFSET,
        Func::PanthorIoctlBoMmapOffset,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANTHOR_GROUP_CREATE,
        Func::PanthorIoctlGroupCreate,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANTHOR_GROUP_DESTROY,
        Func::PanthorIoctlGroupDestroy,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANTHOR_GROUP_GET_STATE,
        Func::PanthorIoctlGroupGetState,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANTHOR_TILER_HEAP_CREATE,
        Func::PanthorIoctlTilerHeapCreate,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANTHOR_TILER_HEAP_DESTROY,
        Func::PanthorIoctlTilerHeapDestroy,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_PANTHOR_GROUP_SUBMIT,
        Func::PanthorIoctlGroupSubmit,
        DRM_RENDER_ALLOW,
    ),
    since(
        def(
            DRM_IOCTL_PANTHOR_BO_SET_LABEL,
            Func::PanthorIoctlBoSetLabel,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_16,
    ),
    since(
        def(
            DRM_IOCTL_PANTHOR_SET_USER_MMIO_OFFSET,
            Func::PanthorIoctlSetUserMmioOffset,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_17,
    ),
    since(
        def(
            DRM_IOCTL_PANTHOR_BO_SYNC,
            Func::PanthorIoctlBoSync,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V7_0,
    ),
    since(
        def(
            DRM_IOCTL_PANTHOR_BO_QUERY_INFO,
            Func::PanthorIoctlBoQueryInfo,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V7_0,
    ),
];

pub static V3D: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_V3D_SUBMIT_CL,
        Func::V3dSubmitClIoctl,
        DRM_RENDER_ALLOW | DRM_AUTH,
    ),
    def(
        DRM_IOCTL_V3D_WAIT_BO,
        Func::V3dWaitBoIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_V3D_CREATE_BO,
        Func::V3dCreateBoIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_V3D_MMAP_BO,
        Func::V3dMmapBoIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_V3D_GET_PARAM,
        Func::V3dGetParamIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_V3D_GET_BO_OFFSET,
        Func::V3dGetBoOffsetIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_V3D_SUBMIT_TFU,
        Func::V3dSubmitTfuIoctl,
        DRM_RENDER_ALLOW | DRM_AUTH,
    ),
    def(
        DRM_IOCTL_V3D_SUBMIT_CSD,
        Func::V3dSubmitCsdIoctl,
        DRM_RENDER_ALLOW | DRM_AUTH,
    ),
    def(
        DRM_IOCTL_V3D_PERFMON_CREATE,
        Func::V3dPerfmonCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_V3D_PERFMON_DESTROY,
        Func::V3dPerfmonDestroyIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_V3D_PERFMON_GET_VALUES,
        Func::V3dPerfmonGetValuesIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_V3D_SUBMIT_CPU,
        Func::V3dSubmitCpuIoctl,
        DRM_RENDER_ALLOW | DRM_AUTH,
    ),
    def(
        DRM_IOCTL_V3D_PERFMON_GET_COUNTER,
        Func::V3dPerfmonGetCounterIoctl,
        DRM_RENDER_ALLOW,
    ),
    since(
        def(
            DRM_IOCTL_V3D_PERFMON_SET_GLOBAL,
            Func::V3dPerfmonSetGlobalIoctl,
            DRM_RENDER_ALLOW,
        ),
        KernelVersion::V6_14,
    ),
];

pub static VIRTGPU: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_VIRTGPU_MAP,
        Func::VirtioGpuMapIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VIRTGPU_EXECBUFFER,
        Func::VirtioGpuExecbufferIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VIRTGPU_GETPARAM,
        Func::VirtioGpuGetparamIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VIRTGPU_RESOURCE_CREATE,
        Func::VirtioGpuResourceCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VIRTGPU_RESOURCE_INFO,
        Func::VirtioGpuResourceInfoIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VIRTGPU_TRANSFER_FROM_HOST,
        Func::VirtioGpuTransferFromHostIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VIRTGPU_TRANSFER_TO_HOST,
        Func::VirtioGpuTransferToHostIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VIRTGPU_WAIT,
        Func::VirtioGpuWaitIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VIRTGPU_GET_CAPS,
        Func::VirtioGpuGetCapsIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB,
        Func::VirtioGpuResourceCreateBlobIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VIRTGPU_CONTEXT_INIT,
        Func::VirtioGpuContextInitIoctl,
        DRM_RENDER_ALLOW,
    ),
];

pub static ETNAVIV: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_ETNAVIV_GET_PARAM,
        Func::EtnavivIoctlGetParam,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ETNAVIV_GEM_NEW,
        Func::EtnavivIoctlGemNew,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ETNAVIV_GEM_INFO,
        Func::EtnavivIoctlGemInfo,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ETNAVIV_GEM_CPU_PREP,
        Func::EtnavivIoctlGemCpuPrep,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ETNAVIV_GEM_CPU_FINI,
        Func::EtnavivIoctlGemCpuFini,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ETNAVIV_GEM_SUBMIT,
        Func::EtnavivIoctlGemSubmit,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ETNAVIV_WAIT_FENCE,
        Func::EtnavivIoctlWaitFence,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ETNAVIV_GEM_USERPTR,
        Func::EtnavivIoctlGemUserptr,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ETNAVIV_GEM_WAIT,
        Func::EtnavivIoctlGemWait,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ETNAVIV_PM_QUERY_DOM,
        Func::EtnavivIoctlPmQueryDom,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_ETNAVIV_PM_QUERY_SIG,
        Func::EtnavivIoctlPmQuerySig,
        DRM_RENDER_ALLOW,
    ),
];

pub static LIMA: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_LIMA_GET_PARAM,
        Func::LimaIoctlGetParam,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_LIMA_GEM_CREATE,
        Func::LimaIoctlGemCreate,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_LIMA_GEM_INFO,
        Func::LimaIoctlGemInfo,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_LIMA_GEM_SUBMIT,
        Func::LimaIoctlGemSubmit,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_LIMA_GEM_WAIT,
        Func::LimaIoctlGemWait,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_LIMA_CTX_CREATE,
        Func::LimaIoctlCtxCreate,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_LIMA_CTX_FREE,
        Func::LimaIoctlCtxFree,
        DRM_RENDER_ALLOW,
    ),
];

pub static VC4: &[IoctlDesc] = &[
    def(
        DRM_IOCTL_VC4_SUBMIT_CL,
        Func::Vc4SubmitClIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_WAIT_SEQNO,
        Func::Vc4WaitSeqnoIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_WAIT_BO,
        Func::Vc4WaitBoIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_CREATE_BO,
        Func::Vc4CreateBoIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_MMAP_BO,
        Func::Vc4MmapBoIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_CREATE_SHADER_BO,
        Func::Vc4CreateShaderBoIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_GET_HANG_STATE,
        Func::Vc4GetHangStateIoctl,
        DRM_ROOT_ONLY,
    ),
    def(
        DRM_IOCTL_VC4_GET_PARAM,
        Func::Vc4GetParamIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_SET_TILING,
        Func::Vc4SetTilingIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_GET_TILING,
        Func::Vc4GetTilingIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_LABEL_BO,
        Func::Vc4LabelBoIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_GEM_MADVISE,
        Func::Vc4GemMadviseIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_PERFMON_CREATE,
        Func::Vc4PerfmonCreateIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_PERFMON_DESTROY,
        Func::Vc4PerfmonDestroyIoctl,
        DRM_RENDER_ALLOW,
    ),
    def(
        DRM_IOCTL_VC4_PERFMON_GET_VALUES,
        Func::Vc4PerfmonGetValuesIoctl,
        DRM_RENDER_ALLOW,
    ),
];
