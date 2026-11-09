use super::devices::DEVICES;
use super::devices::Device as DeviceValues;
use crate::Buffers;
use crate::DriverFeature;
use crate::DriverVersion;
use crate::Drm;
use crate::Export;
use crate::FileBuffers;
use crate::Kernel;
use crate::KernelVersion;
use crate::Master;
use crate::NewHandle;
use crate::PAGE_SIZE;
use crate::Pci;
use crate::Prime;
use crate::Syncobjs;
use crate::UserMemory;
use crate::errno;
use crate::id_bool;
use crate::id_fields;
use crate::ioctls;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::syncobj::Fence;
use crate::uapi::i915::__EXEC_OBJECT_UNKNOWN_FLAGS;
use crate::uapi::i915::__I915_EXEC_FENCE_UNKNOWN_FLAGS;
use crate::uapi::i915::__I915_EXEC_UNKNOWN_FLAGS;
use crate::uapi::i915::__I915_MADV_PURGED;
use crate::uapi::i915::DRM_I915_QUERY_ENGINE_INFO;
use crate::uapi::i915::DRM_I915_QUERY_GEOMETRY_SUBSLICES;
use crate::uapi::i915::DRM_I915_QUERY_GUC_SUBMISSION_VERSION;
use crate::uapi::i915::DRM_I915_QUERY_HWCONFIG_BLOB;
use crate::uapi::i915::DRM_I915_QUERY_MEMORY_REGIONS;
use crate::uapi::i915::DRM_I915_QUERY_TOPOLOGY_INFO;
use crate::uapi::i915::EXEC_OBJECT_NEEDS_GTT;
use crate::uapi::i915::EXEC_OBJECT_PAD_TO_SIZE;
use crate::uapi::i915::EXEC_OBJECT_PINNED;
use crate::uapi::i915::EXEC_OBJECT_WRITE;
use crate::uapi::i915::I915_BIT_6_SWIZZLE_NONE;
use crate::uapi::i915::I915_CACHING_CACHED;
use crate::uapi::i915::I915_CONTEXT_CREATE_EXT_SETPARAM;
use crate::uapi::i915::I915_CONTEXT_CREATE_FLAGS_SINGLE_TIMELINE;
use crate::uapi::i915::I915_CONTEXT_CREATE_FLAGS_UNKNOWN;
use crate::uapi::i915::I915_CONTEXT_CREATE_FLAGS_USE_EXTENSIONS;
use crate::uapi::i915::I915_CONTEXT_DEFAULT_PRIORITY;
use crate::uapi::i915::I915_CONTEXT_MAX_USER_PRIORITY;
use crate::uapi::i915::I915_CONTEXT_MIN_USER_PRIORITY;
use crate::uapi::i915::I915_CONTEXT_PARAM_ENGINES;
use crate::uapi::i915::I915_CONTEXT_PARAM_GTT_SIZE;
use crate::uapi::i915::I915_CONTEXT_PARAM_PRIORITY;
use crate::uapi::i915::I915_CONTEXT_PARAM_RECOVERABLE;
use crate::uapi::i915::I915_CONTEXT_PARAM_SSEU;
use crate::uapi::i915::I915_CONTEXT_PARAM_VM;
use crate::uapi::i915::I915_CONTEXT_SSEU_FLAG_ENGINE_INDEX;
use crate::uapi::i915::I915_ENGINE_CLASS_COPY;
use crate::uapi::i915::I915_ENGINE_CLASS_INVALID;
use crate::uapi::i915::I915_ENGINE_CLASS_INVALID_NONE;
use crate::uapi::i915::I915_ENGINE_CLASS_RENDER;
use crate::uapi::i915::I915_ENGINE_INFO_HAS_LOGICAL_INSTANCE;
use crate::uapi::i915::I915_EXEC_BATCH_FIRST;
use crate::uapi::i915::I915_EXEC_CONSTANTS_MASK;
use crate::uapi::i915::I915_EXEC_CONTEXT_ID_MASK;
use crate::uapi::i915::I915_EXEC_FENCE_ARRAY;
use crate::uapi::i915::I915_EXEC_FENCE_IN;
use crate::uapi::i915::I915_EXEC_FENCE_OUT;
use crate::uapi::i915::I915_EXEC_FENCE_SIGNAL;
use crate::uapi::i915::I915_EXEC_FENCE_SUBMIT;
use crate::uapi::i915::I915_EXEC_FENCE_WAIT;
use crate::uapi::i915::I915_EXEC_RESOURCE_STREAMER;
use crate::uapi::i915::I915_EXEC_RING_MASK;
use crate::uapi::i915::I915_EXEC_SECURE;
use crate::uapi::i915::I915_EXEC_USE_EXTENSIONS;
use crate::uapi::i915::I915_GEM_CREATE_EXT_FLAG_NEEDS_CPU_ACCESS;
use crate::uapi::i915::I915_GEM_CREATE_EXT_MEMORY_REGIONS;
use crate::uapi::i915::I915_GEM_CREATE_EXT_PROTECTED_CONTENT;
use crate::uapi::i915::I915_GEM_CREATE_EXT_SET_PAT;
use crate::uapi::i915::I915_GEM_DOMAIN_COMMAND;
use crate::uapi::i915::I915_GEM_DOMAIN_CPU;
use crate::uapi::i915::I915_GEM_DOMAIN_INSTRUCTION;
use crate::uapi::i915::I915_GEM_DOMAIN_RENDER;
use crate::uapi::i915::I915_GEM_DOMAIN_SAMPLER;
use crate::uapi::i915::I915_GEM_DOMAIN_VERTEX;
use crate::uapi::i915::I915_MADV_DONTNEED;
use crate::uapi::i915::I915_MADV_WILLNEED;
use crate::uapi::i915::I915_MEMORY_CLASS_DEVICE;
use crate::uapi::i915::I915_MEMORY_CLASS_SYSTEM;
use crate::uapi::i915::I915_MMAP_OFFSET_FIXED;
use crate::uapi::i915::I915_MMAP_OFFSET_GTT;
use crate::uapi::i915::I915_MMAP_OFFSET_UC;
use crate::uapi::i915::I915_PARAM_CHIPSET_ID;
use crate::uapi::i915::I915_PARAM_CS_TIMESTAMP_FREQUENCY;
use crate::uapi::i915::I915_PARAM_HAS_CONTEXT_FREQ_HINT;
use crate::uapi::i915::I915_PARAM_HAS_CONTEXT_ISOLATION;
use crate::uapi::i915::I915_PARAM_HAS_EXEC_ASYNC;
use crate::uapi::i915::I915_PARAM_HAS_EXEC_CAPTURE;
use crate::uapi::i915::I915_PARAM_HAS_EXEC_FENCE_ARRAY;
use crate::uapi::i915::I915_PARAM_HAS_EXEC_SOFTPIN;
use crate::uapi::i915::I915_PARAM_HAS_EXEC_TIMELINE_FENCES;
use crate::uapi::i915::I915_PARAM_HAS_EXECBUF2;
use crate::uapi::i915::I915_PARAM_HAS_USERPTR_PROBE;
use crate::uapi::i915::I915_PARAM_HAS_WAIT_TIMEOUT;
use crate::uapi::i915::I915_PARAM_HUC_STATUS;
use crate::uapi::i915::I915_PARAM_MMAP_GTT_VERSION;
use crate::uapi::i915::I915_PARAM_MMAP_VERSION;
use crate::uapi::i915::I915_PARAM_PERF_REVISION;
use crate::uapi::i915::I915_PARAM_PXP_STATUS;
use crate::uapi::i915::I915_PARAM_REVISION;
use crate::uapi::i915::I915_REG_READ_8B_WA;
use crate::uapi::i915::I915_TILING_LAST;
use crate::uapi::i915::I915_TILING_NONE;
use crate::uapi::i915::I915_TILING_Y;
use crate::uapi::i915::I915_USERPTR_PROBE;
use crate::uapi::i915::I915_USERPTR_READ_ONLY;
use crate::uapi::i915::I915_USERPTR_UNSYNCHRONIZED;
use crate::uapi::i915::drm_i915_engine_info;
use crate::uapi::i915::drm_i915_gem_caching;
use crate::uapi::i915::drm_i915_gem_context_create_ext;
use crate::uapi::i915::drm_i915_gem_context_create_ext_setparam;
use crate::uapi::i915::drm_i915_gem_context_destroy;
use crate::uapi::i915::drm_i915_gem_context_param;
use crate::uapi::i915::drm_i915_gem_context_param_sseu;
use crate::uapi::i915::drm_i915_gem_create;
use crate::uapi::i915::drm_i915_gem_create_ext;
use crate::uapi::i915::drm_i915_gem_create_ext_memory_regions;
use crate::uapi::i915::drm_i915_gem_create_ext_protected_content;
use crate::uapi::i915::drm_i915_gem_create_ext_set_pat;
use crate::uapi::i915::drm_i915_gem_exec_fence;
use crate::uapi::i915::drm_i915_gem_exec_object2;
use crate::uapi::i915::drm_i915_gem_execbuffer2;
use crate::uapi::i915::drm_i915_gem_get_aperture;
use crate::uapi::i915::drm_i915_gem_get_tiling;
use crate::uapi::i915::drm_i915_gem_madvise;
use crate::uapi::i915::drm_i915_gem_memory_class_instance;
use crate::uapi::i915::drm_i915_gem_mmap_offset;
use crate::uapi::i915::drm_i915_gem_set_domain;
use crate::uapi::i915::drm_i915_gem_set_tiling;
use crate::uapi::i915::drm_i915_gem_userptr;
use crate::uapi::i915::drm_i915_gem_vm_control;
use crate::uapi::i915::drm_i915_getparam;
use crate::uapi::i915::drm_i915_memory_region_info;
use crate::uapi::i915::drm_i915_memory_region_info__anon__anon;
use crate::uapi::i915::drm_i915_query;
use crate::uapi::i915::drm_i915_query_engine_info;
use crate::uapi::i915::drm_i915_query_guc_submission_version;
use crate::uapi::i915::drm_i915_query_item;
use crate::uapi::i915::drm_i915_query_memory_regions;
use crate::uapi::i915::drm_i915_query_topology_info;
use crate::uapi::i915::drm_i915_reg_read;
use crate::uapi::i915::i915_context_param_engines;
use crate::uapi::i915::i915_engine_class_instance;
use crate::uapi::i915::i915_user_extension;
use crate::uapi::xe::DRM_XE_DEVICE_QUERY_CONFIG;
use crate::uapi::xe::DRM_XE_DEVICE_QUERY_ENGINE_CYCLES;
use crate::uapi::xe::DRM_XE_DEVICE_QUERY_ENGINES;
use crate::uapi::xe::DRM_XE_DEVICE_QUERY_GT_LIST;
use crate::uapi::xe::DRM_XE_DEVICE_QUERY_GT_TOPOLOGY;
use crate::uapi::xe::DRM_XE_DEVICE_QUERY_HWCONFIG;
use crate::uapi::xe::DRM_XE_DEVICE_QUERY_MEM_REGIONS;
use crate::uapi::xe::DRM_XE_DEVICE_QUERY_PXP_STATUS;
use crate::uapi::xe::DRM_XE_DEVICE_QUERY_UC_FW_VERSION;
use crate::uapi::xe::DRM_XE_ENGINE_CLASS_COMPUTE;
use crate::uapi::xe::DRM_XE_ENGINE_CLASS_COPY;
use crate::uapi::xe::DRM_XE_ENGINE_CLASS_RENDER;
use crate::uapi::xe::DRM_XE_ENGINE_CLASS_VM_BIND;
use crate::uapi::xe::DRM_XE_EXEC_QUEUE_EXTENSION_SET_PROPERTY;
use crate::uapi::xe::DRM_XE_EXEC_QUEUE_LOW_LATENCY_HINT;
use crate::uapi::xe::DRM_XE_EXEC_QUEUE_SET_DISABLE_STATE_CACHE_PERF_FIX;
use crate::uapi::xe::DRM_XE_EXEC_QUEUE_SET_PROPERTY_MULTI_QUEUE_PRIORITY;
use crate::uapi::xe::DRM_XE_EXEC_QUEUE_SET_PROPERTY_PRIORITY;
use crate::uapi::xe::DRM_XE_EXEC_QUEUE_SET_PROPERTY_PXP_TYPE;
use crate::uapi::xe::DRM_XE_EXEC_QUEUE_SET_PROPERTY_TIMESLICE;
use crate::uapi::xe::DRM_XE_GEM_CPU_CACHING_WB;
use crate::uapi::xe::DRM_XE_GEM_CPU_CACHING_WC;
use crate::uapi::xe::DRM_XE_GEM_CREATE_FLAG_DEFER_BACKING;
use crate::uapi::xe::DRM_XE_GEM_CREATE_FLAG_NEEDS_VISIBLE_VRAM;
use crate::uapi::xe::DRM_XE_GEM_CREATE_FLAG_NO_COMPRESSION;
use crate::uapi::xe::DRM_XE_GEM_CREATE_FLAG_SCANOUT;
use crate::uapi::xe::DRM_XE_MAX_SYNCS;
use crate::uapi::xe::DRM_XE_MEM_REGION_CLASS_SYSMEM;
use crate::uapi::xe::DRM_XE_MEM_REGION_CLASS_VRAM;
use crate::uapi::xe::DRM_XE_MMAP_OFFSET_FLAG_PCI_BARRIER;
use crate::uapi::xe::DRM_XE_QUERY_CONFIG_FLAG_HAS_CPU_ADDR_MIRROR;
use crate::uapi::xe::DRM_XE_QUERY_CONFIG_FLAG_HAS_DISABLE_STATE_CACHE_PERF_FIX;
use crate::uapi::xe::DRM_XE_QUERY_CONFIG_FLAG_HAS_LOW_LATENCY;
use crate::uapi::xe::DRM_XE_QUERY_CONFIG_FLAG_HAS_NO_COMPRESSION_HINT;
use crate::uapi::xe::DRM_XE_QUERY_CONFIG_FLAG_HAS_PURGING_SUPPORT;
use crate::uapi::xe::DRM_XE_QUERY_CONFIG_FLAG_HAS_VRAM;
use crate::uapi::xe::DRM_XE_QUERY_CONFIG_FLAGS;
use crate::uapi::xe::DRM_XE_QUERY_CONFIG_MAX_EXEC_QUEUE_PRIORITY;
use crate::uapi::xe::DRM_XE_QUERY_CONFIG_MIN_ALIGNMENT;
use crate::uapi::xe::DRM_XE_QUERY_CONFIG_REV_AND_DEVICE_ID;
use crate::uapi::xe::DRM_XE_QUERY_CONFIG_VA_BITS;
use crate::uapi::xe::DRM_XE_QUERY_GT_TYPE_MAIN;
use crate::uapi::xe::DRM_XE_SYNC_FLAG_SIGNAL;
use crate::uapi::xe::DRM_XE_SYNC_TYPE_SYNCOBJ;
use crate::uapi::xe::DRM_XE_SYNC_TYPE_TIMELINE_SYNCOBJ;
use crate::uapi::xe::DRM_XE_SYNC_TYPE_USER_FENCE;
use crate::uapi::xe::DRM_XE_TOPO_DSS_COMPUTE;
use crate::uapi::xe::DRM_XE_TOPO_DSS_GEOMETRY;
use crate::uapi::xe::DRM_XE_TOPO_EU_PER_DSS;
use crate::uapi::xe::DRM_XE_TOPO_L3_BANK;
use crate::uapi::xe::DRM_XE_TOPO_SIMD16_EU_PER_DSS;
use crate::uapi::xe::DRM_XE_VM_CREATE_FLAG_FAULT_MODE;
use crate::uapi::xe::DRM_XE_VM_CREATE_FLAG_LR_MODE;
use crate::uapi::xe::DRM_XE_VM_CREATE_FLAG_NO_VM_OVERCOMMIT;
use crate::uapi::xe::DRM_XE_VM_CREATE_FLAG_SCRATCH_PAGE;
use crate::uapi::xe::DRM_XE_VMA_ATTR_PURGEABLE_STATE;
use crate::uapi::xe::DRM_XE_VMA_PURGEABLE_STATE_DONTNEED;
use crate::uapi::xe::DRM_XE_VMA_PURGEABLE_STATE_WILLNEED;
use crate::uapi::xe::XE_QUERY_UC_TYPE_GUC_SUBMISSION;
use crate::uapi::xe::XE_QUERY_UC_TYPE_HUC;
use crate::uapi::xe::drm_xe_device_query;
use crate::uapi::xe::drm_xe_engine;
use crate::uapi::xe::drm_xe_engine_class_instance;
use crate::uapi::xe::drm_xe_exec;
use crate::uapi::xe::drm_xe_exec_queue_create;
use crate::uapi::xe::drm_xe_exec_queue_destroy;
use crate::uapi::xe::drm_xe_ext_set_property;
use crate::uapi::xe::drm_xe_gem_create;
use crate::uapi::xe::drm_xe_gem_mmap_offset;
use crate::uapi::xe::drm_xe_gt;
use crate::uapi::xe::drm_xe_madvise;
use crate::uapi::xe::drm_xe_mem_region;
use crate::uapi::xe::drm_xe_query_config;
use crate::uapi::xe::drm_xe_query_engine_cycles;
use crate::uapi::xe::drm_xe_query_engines;
use crate::uapi::xe::drm_xe_query_gt_list;
use crate::uapi::xe::drm_xe_query_mem_regions;
use crate::uapi::xe::drm_xe_query_pxp_status;
use crate::uapi::xe::drm_xe_query_topology_mask;
use crate::uapi::xe::drm_xe_query_uc_fw_version;
use crate::uapi::xe::drm_xe_sync;
use crate::uapi::xe::drm_xe_user_extension;
use crate::uapi::xe::drm_xe_vm_bind;
use crate::uapi::xe::drm_xe_vm_create;
use crate::uapi::xe::drm_xe_vm_destroy;
use crate::user::put;
use bytemuck::AnyBitPattern;
use bytemuck::bytes_of;
use libc::E2BIG;
use libc::EFAULT;
use libc::EINVAL;
use libc::ENODEV;
use libc::ENOENT;
use libc::ENOSPC;
use libc::EOPNOTSUPP;
use libc::EPERM;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::mem::offset_of;
use std::time::Instant;

const TIMESTAMP_FREQUENCY: u32 = 12_000_000;
const REGION_SIZE: u64 = 1 << 36;
const VA_BITS: u64 = 48;
const GGTT_SIZE: u64 = 1 << 32;
const SZ_64K: u64 = 64 * 1024;
const SMALL_BAR_SIZE: u64 = 256 << 20;
const TASK_SIZE_MAX: u64 = (1 << 47) - PAGE_SIZE;
const XE_EXEC_QUEUE_PRIORITY_NORMAL: u64 = 1;
const INTEL_ADLP_IDS: [u16; 18] = [
    0x46A0, 0x46A1, 0x46A2, 0x46A3, 0x46A6, 0x46A8, 0x46AA, 0x462A, 0x4626, 0x4628, 0x46B0, 0x46B1,
    0x46B2, 0x46B3, 0x46C0, 0x46C1, 0x46C2, 0x46C3,
];
const INTEL_RPLU_IDS: [u16; 5] = [0xA721, 0xA7A1, 0xA7A9, 0xA7AC, 0xA7AD];
const INTEL_RPLP_IDS: [u16; 5] = [0xA720, 0xA7A0, 0xA7A8, 0xA7AA, 0xA7AB];
const INTEL_TGL_IDS: [u16; 11] = [
    0x9A60, 0x9A68, 0x9A70, 0x9A40, 0x9A49, 0x9A59, 0x9A78, 0x9AC0, 0x9AC9, 0x9AD9, 0x9AF8,
];
const INTEL_RKL_IDS: [u16; 6] = [0x4C80, 0x4C8A, 0x4C8B, 0x4C8C, 0x4C90, 0x4C9A];
const INTEL_ADLS_IDS: [u16; 8] = [
    0x4680, 0x4682, 0x4688, 0x468A, 0x468B, 0x4690, 0x4692, 0x4693,
];
const ENABLE_GUC_SUBMISSION: u32 = 1 << 0;
const ENABLE_GUC_LOAD_HUC: u32 = 1 << 1;
const I915_GEM_GPU_DOMAINS: u32 = I915_GEM_DOMAIN_RENDER
    | I915_GEM_DOMAIN_SAMPLER
    | I915_GEM_DOMAIN_COMMAND
    | I915_GEM_DOMAIN_INSTRUCTION
    | I915_GEM_DOMAIN_VERTEX;
pub const XE_VERX10: u32 = 200;
const SSEU_SLICES: u16 = 1;
const SSEU_SUBSLICES: u16 = 8;
const SSEU_EUS: u16 = 8;
const HWCONFIG_UNDEFINED_KEY: u32 = u32::MAX;
const I915_ENGINE_CLASSES: [u16; 2] = [I915_ENGINE_CLASS_RENDER, I915_ENGINE_CLASS_COPY];
const I915_USER_EXTENSION_DEPTH: u32 = 512;
const MTL_MAX_PAT_INDEX: u32 = 4;
const XE2_MAX_PAT_INDEX: u32 = 31;
const XE_HW_ENGINE_MAX_INSTANCE: usize = 9;
const XE_EXEC_QUEUE_PRIORITY_HIGH: u64 = 2;
const XE_MAX_USER_EXTENSIONS: usize = 16;

struct UcVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

struct GmdId {
    arch: u16,
    release: u16,
}

#[derive(Copy, Clone)]
struct I915Buffer {
    pat_set_by_user: bool,
    tiling: u32,
    madv: u32,
    has_pages: bool,
    truncatable: bool,
}

#[derive(Copy, Clone)]
struct XeBuffer {
    private: bool,
}

#[derive(Copy, Clone)]
enum DriverBuffer {
    I915(I915Buffer),
    Xe(XeBuffer),
}

#[derive(Copy, Clone)]
struct Buffer {
    size: u64,
    driver: DriverBuffer,
}

struct I915Object {
    pat_set_by_user: bool,
    truncatable: bool,
}

impl Buffer {
    fn i915(size: u64, object: I915Object) -> Self {
        Self {
            size,
            driver: DriverBuffer::I915(I915Buffer {
                pat_set_by_user: object.pat_set_by_user,
                tiling: I915_TILING_NONE,
                madv: I915_MADV_WILLNEED,
                has_pages: false,
                truncatable: object.truncatable,
            }),
        }
    }

    fn xe(size: u64, private: bool) -> Self {
        Self {
            size,
            driver: DriverBuffer::Xe(XeBuffer { private }),
        }
    }

    fn i915_state(&mut self) -> &mut I915Buffer {
        match &mut self.driver {
            DriverBuffer::I915(state) => state,
            DriverBuffer::Xe(_) => unreachable!("an xe object in an i915 ioctl"),
        }
    }

    fn private(&self) -> bool {
        match self.driver {
            DriverBuffer::I915(_) => false,
            DriverBuffer::Xe(state) => state.private,
        }
    }
}

struct CreateExt {
    page_size: u64,
    shmem: bool,
    pat_set_by_user: bool,
}

enum XeSignal {
    Syncobj { object: u64 },
    Timeline { object: u64, point: u64 },
    UserFence { address: u64, value: u64 },
}

enum QueryItem<T> {
    RequiredLength(i32),
    Data(T),
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct HwconfigKlv {
    key: u32,
    length: u32,
    value: u32,
}

pub struct Device {
    values: DeviceValues,
    kernel: KernelVersion,
    xe: bool,
    small_bar: bool,
    vram_instance: u16,
    gt0_topology: Option<Gt0Topology>,
    i915_on_xe_platforms: bool,
    created: Instant,
    buffers: Buffers<Buffer>,
    syncobjs: Syncobjs,
    master: Master,
    vms: Mutex<BTreeMap<u64, BTreeSet<u32>>>,
    contexts: Mutex<BTreeMap<u64, BTreeMap<u32, Context>>>,
    exec_queues: Mutex<BTreeMap<u64, BTreeMap<u32, ExecQueue>>>,
}

struct Gt0Topology {
    geometry: Vec<u8>,
    eus: Vec<Vec<u8>>,
}

#[derive(Copy, Clone)]
struct ExecQueue {
    vm: u32,
    bind: bool,
}

struct Context {
    engines: Option<Vec<Option<i915_engine_class_instance>>>,
}

impl Device {
    pub fn new(index: usize, kernel: KernelVersion) -> Self {
        Self::variant(index, kernel, DEVICES[index].verx10 >= XE_VERX10, false)
    }

    pub fn variant(index: usize, kernel: KernelVersion, xe: bool, small_bar: bool) -> Self {
        Self::from_values(DEVICES[index], kernel, xe, small_bar)
    }

    pub fn from_values(
        values: DeviceValues,
        kernel: KernelVersion,
        xe: bool,
        small_bar: bool,
    ) -> Self {
        Self {
            values,
            kernel,
            xe,
            small_bar,
            vram_instance: 1,
            gt0_topology: None,
            i915_on_xe_platforms: false,
            created: Instant::now(),
            buffers: Buffers::default(),
            syncobjs: Syncobjs::default(),
            master: Master::default(),
            vms: Mutex::default(),
            contexts: Mutex::default(),
            exec_queues: Mutex::default(),
        }
    }

    pub fn from_id(id: &str) -> Self {
        let device = Self::from_id_unchecked(id);
        assert!(device.supported(), "{id} has no working driver");
        device
    }

    pub fn from_id_unchecked(id: &str) -> Self {
        let (name, fields) = id
            .split_once(':')
            .unwrap_or_else(|| panic!("invalid intel id {id:?}"));
        let index = DEVICES
            .iter()
            .position(|device| device.name == name)
            .unwrap_or_else(|| panic!("unknown intel device {name}"));
        let values = id_fields(fields, &["kernel", "xe", "small_bar"]);
        Self::variant(
            index,
            KernelVersion::parse(values.get("kernel")),
            id_bool(values.get("xe")),
            id_bool(values.get("small_bar")),
        )
    }

    pub fn with_vram_instance(mut self, instance: u16) -> Self {
        self.vram_instance = instance;
        self
    }

    pub fn with_gt0_topology(mut self, geometry: Vec<u8>, eus: Vec<Vec<u8>>) -> Self {
        self.gt0_topology = Some(Gt0Topology { geometry, eus });
        self
    }

    pub fn with_i915_on_xe_platforms(mut self) -> Self {
        self.i915_on_xe_platforms = true;
        self
    }

    fn vram_mask(&self) -> u32 {
        1 << self.vram_instance
    }

    fn values(&self) -> &DeviceValues {
        &self.values
    }

    fn has_64k_pages(&self) -> bool {
        self.values().verx10 == 125 && self.values().has_local_mem
    }

    fn min_alignment(&self) -> u64 {
        match self.has_64k_pages() {
            true => SZ_64K,
            false => PAGE_SIZE,
        }
    }

    fn cpu_visible_size(&self) -> u64 {
        match self.small_bar {
            true => SMALL_BAR_SIZE,
            false => REGION_SIZE,
        }
    }

    fn graphics_gmdid(&self) -> Option<GmdId> {
        let values = self.values();
        let gmdid = |arch, release| Some(GmdId { arch, release });
        match values.verx10 {
            125 if !values.has_local_mem => gmdid(12, 70),
            200 if values.has_local_mem => gmdid(20, 1),
            200 => gmdid(20, 4),
            300 => gmdid(30, 0),
            350 => gmdid(35, 10),
            _ => None,
        }
    }

    fn timestamp(&self) -> u64 {
        let elapsed = self.created.elapsed();
        (elapsed.as_nanos() * TIMESTAMP_FREQUENCY as u128 / 1_000_000_000) as u64
    }

    fn ip_12_70_or_later(&self) -> bool {
        let device = self.values();
        device.verx10 > 125 || (device.verx10 == 125 && !device.has_local_mem)
    }

    fn has_fences(&self) -> bool {
        !self.values().has_local_mem && !self.ip_12_70_or_later()
    }

    fn has_hwconfig_table(&self) -> bool {
        let id = self.values().pci_id;
        let alderlake_p = INTEL_ADLP_IDS.contains(&id)
            || INTEL_RPLU_IDS.contains(&id)
            || INTEL_RPLP_IDS.contains(&id);
        alderlake_p || self.values().verx10 >= 125
    }

    fn enable_guc(&self) -> u32 {
        let id = self.values().pci_id;
        let tigerlake_or_rocketlake = INTEL_TGL_IDS.contains(&id) || INTEL_RKL_IDS.contains(&id);
        if self.values().verx10 < 120 || tigerlake_or_rocketlake {
            0
        } else if INTEL_ADLS_IDS.contains(&id) {
            ENABLE_GUC_LOAD_HUC
        } else {
            ENABLE_GUC_LOAD_HUC | ENABLE_GUC_SUBMISSION
        }
    }

    fn uses_guc(&self) -> bool {
        self.enable_guc() & (ENABLE_GUC_SUBMISSION | ENABLE_GUC_LOAD_HUC) != 0
    }

    fn guc_submission(&self) -> bool {
        self.enable_guc() & ENABLE_GUC_SUBMISSION != 0
    }

    fn huc_status(&self) -> Result<i32, i32> {
        if self.values().verx10 < 90 {
            Err(ENODEV)
        } else if self.enable_guc() & ENABLE_GUC_LOAD_HUC == 0 {
            Err(EOPNOTSUPP)
        } else {
            Ok(1)
        }
    }

    fn guc_submission_version(&self) -> Option<UcVersion> {
        let version = |major, minor, patch| {
            Some(UcVersion {
                major,
                minor,
                patch,
            })
        };
        match self.values().verx10 {
            300 | 200 => version(1, 38, 1),
            125 => version(1, 26, 0),
            120 => version(1, 24, 4),
            _ => None,
        }
    }

    fn xe_engine_classes(&self) -> &'static [u16] {
        match self.values().verx10 > 120 {
            true => &[
                DRM_XE_ENGINE_CLASS_RENDER,
                DRM_XE_ENGINE_CLASS_COPY,
                DRM_XE_ENGINE_CLASS_COMPUTE,
            ],
            false => &[DRM_XE_ENGINE_CLASS_RENDER, DRM_XE_ENGINE_CLASS_COPY],
        }
    }

    fn xe_hw_engine_lookup(&self, eci: &drm_xe_engine_class_instance) -> bool {
        eci.gt_id == 0
            && eci.engine_instance == 0
            && self.xe_engine_classes().contains(&eci.engine_class)
    }

    fn pxp_full_feature(&self) -> bool {
        self.values().verx10 >= 120 && !self.values().has_local_mem && !self.ip_12_70_or_later()
    }

    fn xe_guc_submission_version(&self) -> Option<UcVersion> {
        let probes = match self.values().name {
            "ptl" => self.kernel >= KernelVersion::V6_17,
            "wcl" => self.kernel >= KernelVersion::V6_17,
            "nvl_s_hx_ul" | "nvl_u_h" => self.kernel >= KernelVersion::V7_2,
            _ => true,
        };
        match probes {
            true => self.guc_submission_version(),
            false => None,
        }
    }

    pub fn supported(&self) -> bool {
        match self.xe {
            true => self.xe_guc_submission_version().is_some(),
            false => self.i915_on_xe_platforms || self.values().verx10 < XE_VERX10,
        }
    }

    fn context_exists(&self, file: u64, id: u32) -> bool {
        id == 0
            || self
                .contexts
                .lock()
                .get(&file)
                .is_some_and(|contexts| contexts.contains_key(&id))
    }

    fn create_vm(&self, file: u64) -> u32 {
        let mut vms = self.vms.lock();
        let vms = vms.entry(file).or_default();
        let vm = (1..).find(|vm| !vms.contains(vm)).unwrap();
        vms.insert(vm);
        vm
    }

    fn destroy_vm(&self, file: u64, vm: u32) -> bool {
        self.vms.lock().entry(file).or_default().remove(&vm)
    }

    fn vm_exists(&self, file: u64, vm: u32) -> bool {
        self.vms.lock().entry(file).or_default().contains(&vm)
    }

    unsafe fn create_ext(
        &self,
        create: &drm_i915_gem_create_ext,
        user: &dyn UserMemory,
    ) -> Result<CreateExt, i32> {
        const INTEL_REGION_UNKNOWN: u32 = 7;
        let max_pat_index = match self.values().verx10 >= XE_VERX10 {
            true => XE2_MAX_PAT_INDEX,
            false => MTL_MAX_PAT_INDEX,
        };
        if create.flags & !I915_GEM_CREATE_EXT_FLAG_NEEDS_CPU_ACCESS != 0 {
            return Err(EINVAL);
        }
        let mut placements: Option<Vec<u16>> = None;
        let mut pat_set_by_user = false;
        i915_user_extensions(user, create.extensions, |name, address| match name {
            I915_GEM_CREATE_EXT_MEMORY_REGIONS => {
                let regions: drm_i915_gem_create_ext_memory_regions = user.read_value(address)?;
                if regions.pad != 0
                    || regions.num_regions == 0
                    || regions.num_regions > INTEL_REGION_UNKNOWN
                {
                    return Err(EINVAL);
                }
                let mut classes = Vec::new();
                for i in 0..regions.num_regions as u64 {
                    let address = regions
                        .regions
                        .wrapping_add(i * size_of::<drm_i915_gem_memory_class_instance>() as u64);
                    let region: drm_i915_gem_memory_class_instance = user.read_value(address)?;
                    let exists = region.memory_instance == 0
                        && (region.memory_class == I915_MEMORY_CLASS_SYSTEM
                            || region.memory_class == I915_MEMORY_CLASS_DEVICE
                                && self.values().has_local_mem);
                    if !exists || classes.contains(&region.memory_class) {
                        return Err(EINVAL);
                    }
                    classes.push(region.memory_class);
                }
                if placements.is_some() {
                    return Err(EINVAL);
                }
                placements = Some(classes);
                Ok(())
            }
            I915_GEM_CREATE_EXT_PROTECTED_CONTENT => {
                let protected: drm_i915_gem_create_ext_protected_content =
                    user.read_value(address)?;
                if protected.flags != 0 {
                    return Err(EINVAL);
                }
                unimplemented!("protected objects");
            }
            I915_GEM_CREATE_EXT_SET_PAT => {
                if !self.ip_12_70_or_later() {
                    return Err(ENODEV);
                }
                let pat: drm_i915_gem_create_ext_set_pat = user.read_value(address)?;
                if pat.pat_index > max_pat_index {
                    return Err(EINVAL);
                }
                pat_set_by_user = true;
                Ok(())
            }
            _ => Err(EINVAL),
        })?;
        let placements = placements.unwrap_or(vec![I915_MEMORY_CLASS_SYSTEM]);
        if create.flags & I915_GEM_CREATE_EXT_FLAG_NEEDS_CPU_ACCESS != 0
            && (placements.len() == 1 || !placements.contains(&I915_MEMORY_CLASS_SYSTEM))
        {
            return Err(EINVAL);
        }
        let page_size = match placements.contains(&I915_MEMORY_CLASS_DEVICE) {
            true => self.min_alignment(),
            false => PAGE_SIZE,
        };
        Ok(CreateExt {
            page_size,
            shmem: placements[0] == I915_MEMORY_CLASS_SYSTEM && !self.values().has_local_mem,
            pat_set_by_user,
        })
    }
}

fn i915_user_extensions(
    user: &dyn UserMemory,
    mut next: u64,
    mut extension: impl FnMut(u32, u64) -> Result<(), i32>,
) -> Result<(), i32> {
    let mut depth = I915_USER_EXTENSION_DEPTH;
    while next != 0 {
        if depth == 0 {
            return Err(E2BIG);
        }
        depth -= 1;
        let field = |offset: usize| -> Result<u32, i32> {
            user.read_value::<u32>(next.wrapping_add(offset as u64))
        };
        if field(offset_of!(i915_user_extension, flags))? != 0 {
            return Err(EINVAL);
        }
        for i in 0..4 {
            if field(offset_of!(i915_user_extension, rsvd) + 4 * i)? != 0 {
                return Err(EINVAL);
            }
        }
        extension(field(offset_of!(i915_user_extension, name))?, next)?;
        let next_address =
            next.wrapping_add(offset_of!(i915_user_extension, next_extension) as u64);
        next = user.read_value::<u64>(next_address)?;
    }
    Ok(())
}

fn tiling_ok(tiling: u32, stride: u32) -> bool {
    const GEN7_FENCE_MAX_PITCH_VAL: u32 = 0x0800;
    if tiling == I915_TILING_NONE {
        return true;
    }
    if tiling > I915_TILING_LAST || stride / 128 > GEN7_FENCE_MAX_PITCH_VAL {
        return false;
    }
    let tile_width = match tiling {
        I915_TILING_Y => 128,
        _ => 512,
    };
    stride != 0 && stride.is_multiple_of(tile_width)
}

fn i915_object_size(size: u64, page_size: u64, shmem: bool) -> Result<u64, i32> {
    const DRM_FILE_PAGE_OFFSET_SIZE: u64 = (0xFFFFFFFF >> 12) * 256;
    let size = size
        .checked_next_multiple_of(page_size)
        .filter(|&size| size != 0)
        .ok_or(EINVAL)?;
    let pages = size / PAGE_SIZE;
    match shmem {
        true if size > i64::MAX as u64 => Err(E2BIG),
        false if pages > i32::MAX as u64 => Err(E2BIG),
        false if pages > DRM_FILE_PAGE_OFFSET_SIZE => Err(libc::ENXIO),
        _ => Ok(size),
    }
}

fn sseu_stride(max: u16) -> u16 {
    max.div_ceil(8)
}

fn topology() -> Vec<u8> {
    let ss_stride = sseu_stride(SSEU_SUBSLICES);
    let eu_stride = sseu_stride(SSEU_EUS);
    let slice_length = size_of::<u8>() as u16;
    let subslice_length = SSEU_SLICES * ss_stride;
    let eu_length = SSEU_SLICES * SSEU_SUBSLICES * eu_stride;
    let header = drm_i915_query_topology_info {
        flags: 0,
        max_slices: SSEU_SLICES,
        max_subslices: SSEU_SUBSLICES,
        max_eus_per_subslice: SSEU_EUS,
        subslice_offset: slice_length,
        subslice_stride: ss_stride,
        eu_offset: slice_length + subslice_length,
        eu_stride,
        data: [],
    };
    let mut out = bytes_of(&header).to_vec();
    out.push((1u16 << SSEU_SLICES).wrapping_sub(1) as u8);
    out.extend(vec![0xff; subslice_length as usize]);
    out.extend(vec![0xff; eu_length as usize]);
    out
}

unsafe fn copy_query_item<T: AnyBitPattern>(
    item: &drm_i915_query_item,
    total_length: usize,
    user: &dyn UserMemory,
) -> Result<QueryItem<T>, i32> {
    if item.length == 0 {
        return Ok(QueryItem::RequiredLength(total_length as i32));
    }
    if (item.length as u32) < total_length as u32 {
        return Err(EINVAL);
    }
    user.read_value(item.data_ptr).map(QueryItem::Data)
}

unsafe fn i915_query_topology(item: &drm_i915_query_item, user: &dyn UserMemory) -> i32 {
    let bytes = topology();
    let header =
        unsafe { copy_query_item::<drm_i915_query_topology_info>(item, bytes.len(), user) };
    match header {
        Ok(QueryItem::Data(_)) => {}
        Ok(QueryItem::RequiredLength(length)) => return length,
        Err(e) => return -e,
    }
    match user.write(item.data_ptr, &bytes) {
        Ok(()) => bytes.len() as i32,
        Err(e) => -e,
    }
}

struct Region {
    class: u16,
    cpu_visible: u64,
}

unsafe fn i915_query_memory_regions(
    device: &Device,
    item: &drm_i915_query_item,
    user: &dyn UserMemory,
) -> i32 {
    if item.flags != 0 {
        return -EINVAL;
    }
    let mut regions = vec![Region {
        class: I915_MEMORY_CLASS_SYSTEM,
        cpu_visible: REGION_SIZE,
    }];
    if device.values().has_local_mem {
        regions.push(Region {
            class: I915_MEMORY_CLASS_DEVICE,
            cpu_visible: device.cpu_visible_size(),
        });
    }
    let total_length = size_of::<drm_i915_query_memory_regions>()
        + regions.len() * size_of::<drm_i915_memory_region_info>();
    let mut header: drm_i915_query_memory_regions =
        match unsafe { copy_query_item(item, total_length, user) } {
            Ok(QueryItem::Data(header)) => header,
            Ok(QueryItem::RequiredLength(length)) => return length,
            Err(e) => return -e,
        };
    if header.num_regions != 0 || header.rsvd != [0; 3] {
        return -EINVAL;
    }
    let info_base = item
        .data_ptr
        .wrapping_add(size_of::<drm_i915_query_memory_regions>() as u64);
    for region in &regions {
        let mut info = vec![0u8; size_of::<drm_i915_memory_region_info>()];
        put(
            &mut info,
            offset_of!(drm_i915_memory_region_info, region),
            drm_i915_gem_memory_class_instance {
                memory_class: region.class,
                memory_instance: 0,
            },
        );
        put(
            &mut info,
            offset_of!(drm_i915_memory_region_info, probed_size),
            REGION_SIZE,
        );
        put(
            &mut info,
            offset_of!(drm_i915_memory_region_info, unallocated_size),
            REGION_SIZE,
        );
        put(
            &mut info,
            offset_of!(drm_i915_memory_region_info, anon),
            drm_i915_memory_region_info__anon__anon {
                probed_cpu_visible_size: region.cpu_visible,
                unallocated_cpu_visible_size: region.cpu_visible,
            },
        );
        let address = info_base.wrapping_add(
            header.num_regions as u64 * size_of::<drm_i915_memory_region_info>() as u64,
        );
        if let Err(e) = user.write(address, &info) {
            return -e;
        }
        header.num_regions += 1;
    }
    let mut bytes = vec![0u8; size_of::<drm_i915_query_memory_regions>()];
    put(
        &mut bytes,
        offset_of!(drm_i915_query_memory_regions, num_regions),
        header.num_regions,
    );
    put(
        &mut bytes,
        offset_of!(drm_i915_query_memory_regions, rsvd),
        header.rsvd,
    );
    match user.write(item.data_ptr, &bytes) {
        Ok(()) => total_length as i32,
        Err(e) => -e,
    }
}

fn i915_engine_lookup_user(class: u16, instance: u16) -> bool {
    instance == 0 && I915_ENGINE_CLASSES.contains(&class)
}

fn i915_context_engines(
    proto: &mut Context,
    param: &drm_i915_gem_context_param,
    user: &dyn UserMemory,
) -> Result<(), i32> {
    let header = size_of::<i915_context_param_engines>();
    let entry = size_of::<i915_engine_class_instance>();
    if proto.engines.is_some() {
        return Err(EINVAL);
    }
    let size = param.size as usize;
    if size < header || !(size - header).is_multiple_of(entry) {
        return Err(EINVAL);
    }
    let count = (size - header) / entry;
    if count as u64 > I915_EXEC_RING_MASK + 1 {
        return Err(EINVAL);
    }
    let mut engines = Vec::new();
    for n in 0..count {
        let address = param.value.wrapping_add((header + n * entry) as u64);
        let ci: i915_engine_class_instance = user.read_value(address)?;
        if ci.engine_class == I915_ENGINE_CLASS_INVALID as u16
            && ci.engine_instance == I915_ENGINE_CLASS_INVALID_NONE as u16
        {
            engines.push(None);
            continue;
        }
        if !i915_engine_lookup_user(ci.engine_class, ci.engine_instance) {
            return Err(ENOENT);
        }
        engines.push(Some(ci));
    }
    let extensions: u64 = user.read_value(param.value)?;
    if extensions != 0 {
        unimplemented!("context engine extensions");
    }
    proto.engines = Some(engines);
    Ok(())
}

fn i915_context_create_setparam(
    device: &Device,
    file: u64,
    proto: &mut Context,
    address: u64,
    user: &dyn UserMemory,
) -> Result<(), i32> {
    let local: drm_i915_gem_context_create_ext_setparam = user.read_value(address)?;
    let param = &local.param;
    if param.ctx_id != 0 {
        return Err(EINVAL);
    }
    match param.param {
        I915_CONTEXT_PARAM_RECOVERABLE => match param.size {
            0 => Ok(()),
            _ => Err(EINVAL),
        },
        I915_CONTEXT_PARAM_ENGINES => i915_context_engines(proto, param, user),
        I915_CONTEXT_PARAM_VM => {
            if param.size != 0 {
                return Err(EINVAL);
            }
            if param.value >> 32 != 0 {
                return Err(ENOENT);
            }
            match device.vm_exists(file, param.value as u32) {
                true => Ok(()),
                false => Err(ENOENT),
            }
        }
        param => unimplemented!("context create parameter {param:#x}"),
    }
}

fn xe_exec_queue_extensions(
    device: &Device,
    class: u16,
    mut next: u64,
    user: &dyn UserMemory,
) -> Result<(), i32> {
    let kernel = device.kernel;
    let last_property = if kernel >= KernelVersion::V7_1 {
        DRM_XE_EXEC_QUEUE_SET_DISABLE_STATE_CACHE_PERF_FIX
    } else if kernel >= KernelVersion::V7_0 {
        DRM_XE_EXEC_QUEUE_SET_PROPERTY_MULTI_QUEUE_PRIORITY
    } else if kernel >= KernelVersion::V6_15 {
        DRM_XE_EXEC_QUEUE_SET_PROPERTY_PXP_TYPE
    } else {
        DRM_XE_EXEC_QUEUE_SET_PROPERTY_TIMESLICE
    };
    let mut number = 0;
    while next != 0 {
        if number >= XE_MAX_USER_EXTENSIONS {
            return Err(E2BIG);
        }
        let ext: drm_xe_user_extension = user.read_value(next)?;
        if ext.pad != 0 || ext.name != DRM_XE_EXEC_QUEUE_EXTENSION_SET_PROPERTY {
            return Err(EINVAL);
        }
        let property: drm_xe_ext_set_property = user.read_value(next)?;
        if property.property > last_property || property.pad != 0 {
            return Err(EINVAL);
        }
        match property.property {
            DRM_XE_EXEC_QUEUE_SET_PROPERTY_PRIORITY => {
                let value = unsafe { property.anon.value };
                if value > XE_EXEC_QUEUE_PRIORITY_HIGH {
                    return Err(EINVAL);
                }
                if value > XE_EXEC_QUEUE_PRIORITY_NORMAL {
                    return Err(EPERM);
                }
            }
            DRM_XE_EXEC_QUEUE_SET_DISABLE_STATE_CACHE_PERF_FIX => {
                if class != DRM_XE_ENGINE_CLASS_RENDER {
                    return Err(EOPNOTSUPP);
                }
            }
            other => unimplemented!("exec queue property {other}"),
        }
        next = ext.next_extension;
        number += 1;
    }
    Ok(())
}

fn xe_exec_queue_create(
    device: &Device,
    file: u64,
    args: &mut drm_xe_exec_queue_create,
    user: &dyn UserMemory,
) -> Result<(), i32> {
    if args.flags & !DRM_XE_EXEC_QUEUE_LOW_LATENCY_HINT != 0 || args.reserved != [0; 2] {
        return Err(EINVAL);
    }
    let len = args.width as usize * args.num_placements as usize;
    if len == 0 || len > XE_HW_ENGINE_MAX_INSTANCE {
        return Err(EINVAL);
    }
    let mut eci = Vec::with_capacity(len);
    for i in 0..len {
        let address = args
            .instances
            .wrapping_add((i * size_of::<drm_xe_engine_class_instance>()) as u64);
        eci.push(user.read_value::<drm_xe_engine_class_instance>(address)?);
    }
    if eci[0].gt_id != 0 {
        return Err(EINVAL);
    }
    if args.flags & DRM_XE_EXEC_QUEUE_LOW_LATENCY_HINT != 0 {
        unimplemented!("low-latency exec queues");
    }
    if eci[0].engine_class == DRM_XE_ENGINE_CLASS_VM_BIND {
        if args.width != 1 || args.num_placements != 1 || eci[0].engine_instance != 0 {
            return Err(EINVAL);
        }
        if device.kernel >= KernelVersion::V6_19 && !device.vm_exists(file, args.vm_id) {
            return Err(ENOENT);
        }
        if args.extensions != 0 {
            unimplemented!("extensions on a VM_BIND exec queue");
        }
    } else {
        if args.width != 1 {
            unimplemented!("exec queues wider than one engine");
        }
        let class = eci[0].engine_class;
        for engine in &eci {
            if !device.xe_hw_engine_lookup(engine) || engine.engine_class != class {
                return Err(EINVAL);
            }
        }
        if !device.vm_exists(file, args.vm_id) {
            return Err(ENOENT);
        }
        xe_exec_queue_extensions(device, class, args.extensions, user)?;
    }
    let mut queues = device.exec_queues.lock();
    let queues = queues.entry(file).or_default();
    let id = (1..).find(|id| !queues.contains_key(id)).unwrap();
    let bind = eci[0].engine_class == DRM_XE_ENGINE_CLASS_VM_BIND;
    queues.insert(
        id,
        ExecQueue {
            vm: args.vm_id,
            bind,
        },
    );
    args.exec_queue_id = id;
    Ok(())
}

fn xe_exec(
    device: &Device,
    file: u64,
    args: &drm_xe_exec,
    user: &dyn UserMemory,
) -> Result<(), i32> {
    if args.extensions != 0 || args.pad != [0; 3] || args.reserved != [0; 2] {
        return Err(EINVAL);
    }
    if device.kernel >= KernelVersion::V6_19 && args.num_syncs > DRM_XE_MAX_SYNCS {
        return Err(EINVAL);
    }
    let bind = device
        .exec_queues
        .lock()
        .get(&file)
        .and_then(|queues| queues.get(&args.exec_queue_id))
        .map(|queue| queue.bind);
    match bind {
        None => return Err(ENOENT),
        Some(true) => return Err(EINVAL),
        Some(false) => {}
    }
    if args.num_batch_buffer != 0 && args.num_batch_buffer != 1 {
        return Err(EINVAL);
    }
    let signals = unsafe { xe_syncs(device, file, args.syncs, args.num_syncs, true, user)? };
    let user_fences = signals
        .iter()
        .filter(|signal| matches!(signal, XeSignal::UserFence { .. }))
        .count();
    if user_fences > 1 {
        return Err(EINVAL);
    }
    signal_fences(device, signals, user)?;
    Ok(())
}

fn gen8_canonical_addr(address: u64) -> u64 {
    ((address << 16) as i64 >> 16) as u64
}

fn i915_execbuffer2(
    device: &Device,
    file: u64,
    args: &mut drm_i915_gem_execbuffer2,
    user: &dyn UserMemory,
) -> Result<(), i32> {
    let entry = size_of::<drm_i915_gem_exec_object2>();
    let count = args.buffer_count as usize;
    if count < 1 || count > i32::MAX as usize {
        return Err(EINVAL);
    }
    let illegal =
        __I915_EXEC_UNKNOWN_FLAGS | I915_EXEC_CONSTANTS_MASK | I915_EXEC_RESOURCE_STREAMER;
    if args.flags & illegal != 0 {
        return Err(EINVAL);
    }
    if args.flags & (I915_EXEC_FENCE_ARRAY | I915_EXEC_USE_EXTENSIONS) == 0
        && (args.num_cliprects != 0 || args.cliprects_ptr != 0)
    {
        return Err(EINVAL);
    }
    if args.DR4 == 0xffffffff {
        args.DR4 = 0;
    }
    if args.DR1 != 0 || args.DR4 != 0 {
        return Err(EINVAL);
    }
    if (args.batch_start_offset | args.batch_len) & 0x7 != 0 {
        return Err(EINVAL);
    }
    let mut objects = Vec::with_capacity(count);
    for i in 0..count {
        let address = args.buffers_ptr.wrapping_add((i * entry) as u64);
        objects.push(user.read_value::<drm_i915_gem_exec_object2>(address)?);
    }
    if args.flags & I915_EXEC_SECURE != 0 {
        return Err(match device.values().verx10 >= 110 {
            true => libc::ENODEV,
            false => EPERM,
        });
    }
    if args.flags & I915_EXEC_USE_EXTENSIONS != 0 {
        unimplemented!("execbuffer extensions");
    }
    let mut signals = Vec::new();
    if args.flags & I915_EXEC_FENCE_ARRAY != 0 {
        for i in 0..args.num_cliprects as u64 {
            let address = args
                .cliprects_ptr
                .wrapping_add(i * size_of::<drm_i915_gem_exec_fence>() as u64);
            let fence: drm_i915_gem_exec_fence = user.read_value(address)?;
            if fence.flags & __I915_EXEC_FENCE_UNKNOWN_FLAGS != 0 {
                return Err(EINVAL);
            }
            let Some(object) = device.syncobjs.object(file, fence.handle) else {
                return Err(ENOENT);
            };
            if fence.flags & I915_EXEC_FENCE_WAIT != 0
                && device.syncobjs.current(file, fence.handle)?.is_none()
            {
                return Err(EINVAL);
            }
            if fence.flags & I915_EXEC_FENCE_SIGNAL != 0 {
                signals.push(object);
            }
        }
    }
    if args.flags & (I915_EXEC_FENCE_IN | I915_EXEC_FENCE_SUBMIT | I915_EXEC_FENCE_OUT) != 0 {
        unimplemented!("execbuffer sync file fences {:#x}", args.flags);
    }
    let context_id = (args.rsvd1 & I915_EXEC_CONTEXT_ID_MASK) as u32;
    let engines = {
        let contexts = device.contexts.lock();
        match context_id {
            0 => None,
            id => match contexts.get(&file).and_then(|contexts| contexts.get(&id)) {
                Some(context) => context.engines.clone(),
                None => return Err(ENOENT),
            },
        }
    };
    let Some(engines) = engines else {
        unimplemented!("execbuffer on a context without an engine map");
    };
    let index = (args.flags & I915_EXEC_RING_MASK) as usize;
    if engines.get(index).copied().flatten().is_none() {
        return Err(EINVAL);
    }
    let invalid_flags = __EXEC_OBJECT_UNKNOWN_FLAGS | EXEC_OBJECT_NEEDS_GTT;
    let batch = match args.flags & I915_EXEC_BATCH_FIRST {
        0 => count - 1,
        _ => 0,
    };
    let buffers = device.buffers.file(file);
    let mut handles = BTreeSet::new();
    for (i, object) in objects.iter().enumerate() {
        let Some(size) = buffers.with(object.handle, |buffer| buffer.size) else {
            return Err(ENOENT);
        };
        if !handles.insert(object.handle) {
            unimplemented!("an object listed twice in one execbuffer");
        }
        if object.relocation_count != 0 {
            unimplemented!("relocations");
        }
        if object.flags & invalid_flags != 0 {
            return Err(EINVAL);
        }
        if object.alignment != 0 && !object.alignment.is_power_of_two() {
            return Err(EINVAL);
        }
        if object.flags & EXEC_OBJECT_PINNED != 0
            && object.offset != gen8_canonical_addr(object.offset & !(PAGE_SIZE - 1))
        {
            return Err(EINVAL);
        }
        let pad_to_size = unsafe { object.anon.pad_to_size };
        if object.flags & EXEC_OBJECT_PAD_TO_SIZE != 0 && pad_to_size % PAGE_SIZE != 0 {
            return Err(EINVAL);
        }
        if i == batch {
            if object.flags & EXEC_OBJECT_WRITE != 0 {
                return Err(EINVAL);
            }
            let start = args.batch_start_offset as u64;
            let length = args.batch_len as u64;
            if start > size || length > size - start {
                return Err(EINVAL);
            }
            if length == 0 && size == start {
                return Err(EINVAL);
            }
        }
    }
    for object in signals {
        device.syncobjs.replace(object, Some(Fence::Plain));
    }
    Ok(())
}

fn i915_context_create(
    device: &Device,
    file: u64,
    args: &mut drm_i915_gem_context_create_ext,
    user: &dyn UserMemory,
) -> Result<(), i32> {
    if args.flags & I915_CONTEXT_CREATE_FLAGS_UNKNOWN != 0 {
        return Err(EINVAL);
    }
    if args.flags & I915_CONTEXT_CREATE_FLAGS_SINGLE_TIMELINE != 0 {
        unimplemented!("single-timeline contexts");
    }
    let mut proto = Context { engines: None };
    if args.flags & I915_CONTEXT_CREATE_FLAGS_USE_EXTENSIONS != 0 {
        i915_user_extensions(user, args.extensions, |name, address| match name {
            I915_CONTEXT_CREATE_EXT_SETPARAM => {
                i915_context_create_setparam(device, file, &mut proto, address, user)
            }
            _ => Err(EINVAL),
        })?;
    }
    let mut contexts = device.contexts.lock();
    let contexts = contexts.entry(file).or_default();
    let id = (1..).find(|id| !contexts.contains_key(id)).unwrap();
    contexts.insert(id, proto);
    args.ctx_id = id;
    Ok(())
}

unsafe fn i915_query_engine_info(item: &drm_i915_query_item, user: &dyn UserMemory) -> i32 {
    if item.flags != 0 {
        return -EINVAL;
    }
    let classes = I915_ENGINE_CLASSES;
    let len =
        size_of::<drm_i915_query_engine_info>() + classes.len() * size_of::<drm_i915_engine_info>();
    let mut query: drm_i915_query_engine_info = match unsafe { copy_query_item(item, len, user) } {
        Ok(QueryItem::Data(query)) => query,
        Ok(QueryItem::RequiredLength(length)) => return length,
        Err(e) => return -e,
    };
    if query.num_engines != 0 || query.rsvd != [0; 3] {
        return -EINVAL;
    }
    let info_base = item
        .data_ptr
        .wrapping_add(size_of::<drm_i915_query_engine_info>() as u64);
    for class in classes {
        let mut info: drm_i915_engine_info = unsafe { std::mem::zeroed() };
        info.engine.engine_class = class;
        info.engine.engine_instance = 0;
        info.flags = I915_ENGINE_INFO_HAS_LOGICAL_INSTANCE;
        info.logical_instance = 0;
        let address = info_base
            .wrapping_add(query.num_engines as u64 * size_of::<drm_i915_engine_info>() as u64);
        if let Err(e) = user.write_value(address, &info) {
            return -e;
        }
        query.num_engines += 1;
    }
    match user.write_value(item.data_ptr, &query) {
        Ok(()) => len as i32,
        Err(e) => -e,
    }
}

unsafe fn i915_query_guc_submission_version(
    device: &Device,
    item: &drm_i915_query_item,
    user: &dyn UserMemory,
) -> i32 {
    let Some(version) = device.guc_submission_version() else {
        unimplemented!("the GuC submission version of {}", device.values().name);
    };
    let size = size_of::<drm_i915_query_guc_submission_version>();
    let query: drm_i915_query_guc_submission_version =
        match unsafe { copy_query_item(item, size, user) } {
            Ok(QueryItem::Data(query)) => query,
            Ok(QueryItem::RequiredLength(length)) => return length,
            Err(e) => return -e,
        };
    if query.branch != 0 || query.major != 0 || query.minor != 0 || query.patch != 0 {
        return -EINVAL;
    }
    let version = drm_i915_query_guc_submission_version {
        branch: 0,
        major: version.major,
        minor: version.minor,
        patch: version.patch,
    };
    match user.write_value(item.data_ptr, &version) {
        Ok(()) => 0,
        Err(e) => -e,
    }
}

unsafe fn i915_query(device: &Device, arg: *mut u8, user: &dyn UserMemory) -> i64 {
    unsafe {
        let query = &*arg.cast::<drm_i915_query>();
        if query.flags != 0 {
            return errno(EINVAL);
        }
        for index in 0..query.num_items as u64 {
            let item_address = query
                .items_ptr
                .wrapping_add(index * size_of::<drm_i915_query_item>() as u64);
            let item: drm_i915_query_item = match user.read_value(item_address) {
                Ok(item) => item,
                Err(e) => return errno(e),
            };
            let ret = match item.query_id {
                DRM_I915_QUERY_TOPOLOGY_INFO if item.flags != 0 => -EINVAL,
                DRM_I915_QUERY_TOPOLOGY_INFO => i915_query_topology(&item, user),
                DRM_I915_QUERY_MEMORY_REGIONS => i915_query_memory_regions(device, &item, user),
                DRM_I915_QUERY_GEOMETRY_SUBSLICES => {
                    let engine_class = item.flags & 0xff;
                    let engine_instance = (item.flags >> 16) & 0xff;
                    if device.values().verx10 < 125 {
                        -ENODEV
                    } else if engine_class != I915_ENGINE_CLASS_RENDER as u32
                        || engine_instance != 0
                    {
                        -EINVAL
                    } else {
                        i915_query_topology(&item, user)
                    }
                }
                DRM_I915_QUERY_HWCONFIG_BLOB
                    if !device.uses_guc() || !device.has_hwconfig_table() =>
                {
                    -ENODEV
                }
                DRM_I915_QUERY_HWCONFIG_BLOB => {
                    let bytes = hwconfig();
                    if item.length == 0 {
                        bytes.len() as i32
                    } else if (item.length as usize) < bytes.len() {
                        -EINVAL
                    } else {
                        match user.write(item.data_ptr, &bytes) {
                            Ok(()) => bytes.len() as i32,
                            Err(e) => -e,
                        }
                    }
                }
                DRM_I915_QUERY_ENGINE_INFO => i915_query_engine_info(&item, user),
                DRM_I915_QUERY_GUC_SUBMISSION_VERSION if !device.guc_submission() => -ENODEV,
                DRM_I915_QUERY_GUC_SUBMISSION_VERSION => {
                    i915_query_guc_submission_version(device, &item, user)
                }
                query_id => unimplemented!("i915 query {query_id}"),
            };
            let length_address =
                item_address.wrapping_add(offset_of!(drm_i915_query_item, length) as u64);
            if ret != item.length
                && let Err(e) = user.write_value(length_address, &ret)
            {
                return errno(e);
            }
        }
    }
    0
}

unsafe fn i915_getparam(device: &Device, arg: *mut u8, user: &dyn UserMemory) -> i64 {
    unsafe {
        let param = &*arg.cast::<drm_i915_getparam>();
        let value = match param.param {
            I915_PARAM_CS_TIMESTAMP_FREQUENCY => TIMESTAMP_FREQUENCY as i32,
            I915_PARAM_MMAP_GTT_VERSION => match device.kernel >= KernelVersion::V6_14 {
                true => 5,
                false => 4,
            },
            I915_PARAM_MMAP_VERSION => 1,
            I915_PARAM_CHIPSET_ID => device.values().pci_id as i32,
            I915_PARAM_REVISION => 0,
            I915_PARAM_PERF_REVISION => 7,
            I915_PARAM_HUC_STATUS => match device.huc_status() {
                Ok(status) => status,
                Err(e) => return errno(e),
            },
            I915_PARAM_PXP_STATUS => match device.pxp_full_feature() {
                true => 1,
                false => return errno(ENODEV),
            },
            I915_PARAM_HAS_CONTEXT_FREQ_HINT => match device.guc_submission() {
                true => 1,
                false => -EINVAL,
            },
            I915_PARAM_HAS_CONTEXT_ISOLATION => I915_ENGINE_CLASSES
                .iter()
                .fold(0, |classes, &class| classes | 1 << class),
            I915_PARAM_HAS_EXECBUF2
            | I915_PARAM_HAS_WAIT_TIMEOUT
            | I915_PARAM_HAS_EXEC_SOFTPIN
            | I915_PARAM_HAS_EXEC_ASYNC
            | I915_PARAM_HAS_EXEC_CAPTURE
            | I915_PARAM_HAS_EXEC_FENCE_ARRAY
            | I915_PARAM_HAS_EXEC_TIMELINE_FENCES
            | I915_PARAM_HAS_USERPTR_PROBE => 1,
            param => unimplemented!("i915 param {param}"),
        };
        match user.write_value(param.value as u64, &value) {
            Ok(()) => 0,
            Err(e) => errno(e),
        }
    }
}

fn signal_fences(
    device: &Device,
    signals: Vec<XeSignal>,
    user: &dyn UserMemory,
) -> Result<(), i32> {
    for signal in signals {
        match signal {
            XeSignal::Syncobj { object } => device.syncobjs.replace(object, Some(Fence::Plain)),
            XeSignal::Timeline { object, point } => device.syncobjs.add_point(object, point),
            XeSignal::UserFence { address, value } => user.write_value(address, &value)?,
        }
    }
    Ok(())
}

fn i915_validate_priority(param: &drm_i915_gem_context_param) -> Result<(), i32> {
    let priority = param.value as i64;
    if param.size != 0 {
        return Err(EINVAL);
    }
    if !(I915_CONTEXT_MIN_USER_PRIORITY..=I915_CONTEXT_MAX_USER_PRIORITY).contains(&priority) {
        return Err(EINVAL);
    }
    if priority > I915_CONTEXT_DEFAULT_PRIORITY {
        return Err(EPERM);
    }
    Ok(())
}

unsafe fn i915_ioctl(
    device: &Device,
    file: u64,
    func: Func,
    arg: *mut u8,
    user: &dyn UserMemory,
) -> Option<i64> {
    let buffers = &device.buffers.file(file);
    let ret = unsafe {
        match func {
            Func::I915GetparamIoctl => i915_getparam(device, arg, user),
            Func::I915QueryIoctl => i915_query(device, arg, user),
            Func::I915GemExecbuffer2Ioctl => {
                let args = &mut *arg.cast::<drm_i915_gem_execbuffer2>();
                match i915_execbuffer2(device, file, args, user) {
                    Ok(()) => 0,
                    Err(e) => errno(e),
                }
            }
            Func::I915GemContextDestroyIoctl => {
                let args = &*arg.cast::<drm_i915_gem_context_destroy>();
                if args.pad != 0 {
                    return Some(errno(EINVAL));
                }
                if args.ctx_id == 0 {
                    return Some(errno(ENOENT));
                }
                let removed = device
                    .contexts
                    .lock()
                    .get_mut(&file)
                    .and_then(|contexts| contexts.remove(&args.ctx_id));
                match removed {
                    Some(_) => 0,
                    None => errno(ENOENT),
                }
            }
            Func::I915GemContextCreateIoctl => {
                let args = &mut *arg.cast::<drm_i915_gem_context_create_ext>();
                match i915_context_create(device, file, args, user) {
                    Ok(()) => 0,
                    Err(e) => errno(e),
                }
            }
            Func::I915GemContextGetparamIoctl => {
                let param = &mut *arg.cast::<drm_i915_gem_context_param>();
                if param.ctx_id != 0 {
                    unimplemented!("contexts other than the default context");
                }
                match param.param {
                    I915_CONTEXT_PARAM_GTT_SIZE => {
                        param.size = 0;
                        param.value = 1 << VA_BITS;
                    }
                    I915_CONTEXT_PARAM_SSEU if param.size == 0 => {
                        param.size = size_of::<drm_i915_gem_context_param_sseu>() as u32;
                    }
                    I915_CONTEXT_PARAM_SSEU => {
                        if (param.size as usize) < size_of::<drm_i915_gem_context_param_sseu>() {
                            return Some(errno(EINVAL));
                        }
                        let mut sseu: drm_i915_gem_context_param_sseu =
                            match user.read_value(param.value) {
                                Ok(sseu) => sseu,
                                Err(e) => return Some(errno(e)),
                            };
                        if sseu.rsvd != 0 || sseu.flags & !I915_CONTEXT_SSEU_FLAG_ENGINE_INDEX != 0
                        {
                            return Some(errno(EINVAL));
                        }
                        if sseu.flags & I915_CONTEXT_SSEU_FLAG_ENGINE_INDEX != 0 {
                            return Some(errno(EINVAL));
                        }
                        let engine = sseu.engine;
                        if !i915_engine_lookup_user(engine.engine_class, engine.engine_instance) {
                            return Some(errno(EINVAL));
                        }
                        sseu.slice_mask = (1 << SSEU_SLICES) - 1;
                        sseu.subslice_mask = (1 << SSEU_SUBSLICES) - 1;
                        sseu.min_eus_per_subslice = SSEU_EUS;
                        sseu.max_eus_per_subslice = SSEU_EUS;
                        if let Err(e) = user.write_value(param.value, &sseu) {
                            return Some(errno(e));
                        }
                        param.size = size_of::<drm_i915_gem_context_param_sseu>() as u32;
                    }
                    I915_CONTEXT_PARAM_VM => {
                        param.value = device.create_vm(file) as u64;
                        param.size = 0;
                    }
                    _ => unimplemented!("i915 context param {}", param.param),
                }
                0
            }
            Func::I915GemCreateExtIoctl => {
                let create = &mut *arg.cast::<drm_i915_gem_create_ext>();
                let ext = match device.create_ext(create, user) {
                    Ok(ext) => ext,
                    Err(e) => return Some(errno(e)),
                };
                let size = match i915_object_size(create.size, ext.page_size, ext.shmem) {
                    Ok(size) => size,
                    Err(e) => return Some(errno(e)),
                };
                create.size = size;
                let object = I915Object {
                    pat_set_by_user: ext.pat_set_by_user,
                    truncatable: true,
                };
                create.handle = buffers.create(Buffer::i915(size, object), size).handle;
                0
            }
            Func::I915GemCreateIoctl => {
                let create = &mut *arg.cast::<drm_i915_gem_create>();
                let size = match i915_object_size(
                    create.size,
                    PAGE_SIZE,
                    !device.values().has_local_mem,
                ) {
                    Ok(size) => size,
                    Err(e) => return Some(errno(e)),
                };
                create.size = size;
                let object = I915Object {
                    pat_set_by_user: false,
                    truncatable: true,
                };
                create.handle = buffers.create(Buffer::i915(size, object), size).handle;
                0
            }
            Func::I915GemMmapOffsetIoctl => {
                let mmap = &mut *arg.cast::<drm_i915_gem_mmap_offset>();
                if mmap.extensions != 0 {
                    return Some(errno(EINVAL));
                }
                let local = device.values().has_local_mem;
                match mmap.flags {
                    I915_MMAP_OFFSET_GTT if local => errno(ENODEV),
                    I915_MMAP_OFFSET_GTT..=I915_MMAP_OFFSET_FIXED
                        if !buffers.contains(mmap.handle) =>
                    {
                        errno(ENOENT)
                    }
                    flags @ I915_MMAP_OFFSET_GTT..=I915_MMAP_OFFSET_FIXED
                        if (flags == I915_MMAP_OFFSET_FIXED) != local =>
                    {
                        errno(ENODEV)
                    }
                    I915_MMAP_OFFSET_GTT | I915_MMAP_OFFSET_UC => {
                        unimplemented!("mmap offset flags {}", mmap.flags);
                    }
                    I915_MMAP_OFFSET_GTT..=I915_MMAP_OFFSET_FIXED => {
                        buffers.with(mmap.handle, |buffer| buffer.i915_state().has_pages = true);
                        match buffers.offset(mmap.handle) {
                            Some(offset) => {
                                mmap.offset = offset;
                                0
                            }
                            None => errno(ENOENT),
                        }
                    }
                    flags => unimplemented!("mmap offset flags {flags}"),
                }
            }
            Func::I915GemSetDomainIoctl => {
                let domain = &*arg.cast::<drm_i915_gem_set_domain>();
                let read_domains = domain.read_domains;
                let write_domain = domain.write_domain;
                if device.values().has_local_mem {
                    errno(ENODEV)
                } else if (write_domain | read_domains) & I915_GEM_GPU_DOMAINS != 0
                    || (write_domain != 0 && read_domains != write_domain)
                {
                    errno(EINVAL)
                } else if read_domains != I915_GEM_DOMAIN_CPU || write_domain != 0 {
                    unimplemented!("set domain {read_domains:#x} {write_domain:#x}");
                } else {
                    let pinned = buffers.with(domain.handle, |buffer| {
                        let state = buffer.i915_state();
                        if !state.has_pages && state.madv != I915_MADV_WILLNEED {
                            return false;
                        }
                        state.has_pages = true;
                        true
                    });
                    match pinned {
                        Some(true) => 0,
                        Some(false) => errno(EFAULT),
                        None => errno(ENOENT),
                    }
                }
            }
            Func::I915GemSetTilingIoctl => {
                let tiling = &mut *arg.cast::<drm_i915_gem_set_tiling>();
                if !device.has_fences() {
                    return Some(errno(EOPNOTSUPP));
                }
                if !buffers.contains(tiling.handle) {
                    return Some(errno(ENOENT));
                }
                if !tiling_ok(tiling.tiling_mode, tiling.stride) {
                    return Some(errno(EINVAL));
                }
                tiling.swizzle_mode = I915_BIT_6_SWIZZLE_NONE;
                if tiling.tiling_mode == I915_TILING_NONE {
                    tiling.stride = 0;
                }
                let mode = tiling.tiling_mode;
                match buffers.with(tiling.handle, |buffer| buffer.i915_state().tiling = mode) {
                    Some(()) => 0,
                    None => errno(ENOENT),
                }
            }
            Func::I915GemGetTilingIoctl => {
                let tiling = &mut *arg.cast::<drm_i915_gem_get_tiling>();
                if !device.has_fences() {
                    return Some(errno(EOPNOTSUPP));
                }
                let Some(mode) = buffers.with(tiling.handle, |buffer| buffer.i915_state().tiling)
                else {
                    return Some(errno(ENOENT));
                };
                tiling.tiling_mode = mode;
                tiling.swizzle_mode = I915_BIT_6_SWIZZLE_NONE;
                tiling.phys_swizzle_mode = I915_BIT_6_SWIZZLE_NONE;
                0
            }
            Func::I915GemUserptrIoctl => {
                let userptr = &*arg.cast::<drm_i915_gem_userptr>();
                let flags =
                    I915_USERPTR_READ_ONLY | I915_USERPTR_UNSYNCHRONIZED | I915_USERPTR_PROBE;
                let end = userptr.user_ptr.checked_add(userptr.user_size);
                if userptr.flags & !flags != 0
                    || userptr.user_size == 0
                    || (userptr.user_ptr | userptr.user_size) % PAGE_SIZE != 0
                {
                    errno(EINVAL)
                } else if end.is_none_or(|end| end > TASK_SIZE_MAX) {
                    errno(EFAULT)
                } else if userptr.flags != 0 {
                    unimplemented!("userptr flags {:#x}", userptr.flags);
                } else {
                    unimplemented!("creating userptr objects");
                }
            }
            Func::I915RegReadIoctl => {
                const RING_TIMESTAMP_RENDER: u64 = 0x02000 + 0x358;
                let reg = &mut *arg.cast::<drm_i915_reg_read>();
                let graphics_ver = device.values().verx10 / 10;
                let size = 8;
                if !(4..=12).contains(&graphics_ver)
                    || reg.offset & !(size - 1) != RING_TIMESTAMP_RENDER
                {
                    return Some(errno(EINVAL));
                }
                match reg.offset & (size - 1) {
                    0 => unimplemented!("register read without I915_REG_READ_8B_WA"),
                    I915_REG_READ_8B_WA => {
                        reg.val = device.timestamp();
                        0
                    }
                    flags => unimplemented!("register read flags {flags:#x}"),
                }
            }
            Func::I915GemSetCachingIoctl => {
                let caching = &*arg.cast::<drm_i915_gem_caching>();
                if device.values().has_local_mem {
                    return Some(errno(ENODEV));
                }
                if device.ip_12_70_or_later() {
                    return Some(errno(EOPNOTSUPP));
                }
                if caching.caching != I915_CACHING_CACHED {
                    unimplemented!("caching mode {}", caching.caching);
                }
                let ret = buffers.with(caching.handle, |buffer| {
                    match buffer.i915_state().pat_set_by_user {
                        true => errno(EOPNOTSUPP),
                        false => 0,
                    }
                });
                ret.unwrap_or(errno(ENOENT))
            }
            Func::I915GemMadviseIoctl => {
                let madvise = &mut *arg.cast::<drm_i915_gem_madvise>();
                if madvise.madv != I915_MADV_WILLNEED && madvise.madv != I915_MADV_DONTNEED {
                    return Some(errno(EINVAL));
                }
                let madv = madvise.madv;
                let retained = buffers.with(madvise.handle, |buffer| {
                    let state = buffer.i915_state();
                    if state.madv != __I915_MADV_PURGED {
                        state.madv = madv;
                    }
                    if state.madv == I915_MADV_DONTNEED && !state.has_pages && state.truncatable {
                        state.madv = __I915_MADV_PURGED;
                    }
                    state.madv != __I915_MADV_PURGED
                });
                match retained {
                    Some(retained) => {
                        madvise.retained = retained as u32;
                        0
                    }
                    None => errno(ENOENT),
                }
            }
            Func::I915GemGetApertureIoctl => {
                let aperture = &mut *arg.cast::<drm_i915_gem_get_aperture>();
                aperture.aper_size = GGTT_SIZE;
                aperture.aper_available_size = GGTT_SIZE;
                0
            }
            Func::I915GemVmCreateIoctl => {
                let control = &mut *arg.cast::<drm_i915_gem_vm_control>();
                if control.flags != 0 {
                    return Some(errno(EINVAL));
                }
                let extensions = i915_user_extensions(user, control.extensions, |_, _| Err(EINVAL));
                if let Err(e) = extensions {
                    return Some(errno(e));
                }
                control.vm_id = device.create_vm(file);
                0
            }
            Func::I915GemVmDestroyIoctl => {
                let control = &*arg.cast::<drm_i915_gem_vm_control>();
                if control.flags != 0 || control.extensions != 0 {
                    errno(EINVAL)
                } else if device.destroy_vm(file, control.vm_id) {
                    0
                } else {
                    errno(ENOENT)
                }
            }
            Func::I915GemContextSetparamIoctl => {
                let param = &*arg.cast::<drm_i915_gem_context_param>();
                if !device.context_exists(file, param.ctx_id) {
                    return Some(errno(ENOENT));
                }
                let result = match param.param {
                    I915_CONTEXT_PARAM_PRIORITY => i915_validate_priority(param),
                    I915_CONTEXT_PARAM_RECOVERABLE if param.size != 0 => Err(EINVAL),
                    I915_CONTEXT_PARAM_RECOVERABLE => Ok(()),
                    param => unimplemented!("context param {param}"),
                };
                match result {
                    Ok(()) => 0,
                    Err(e) => errno(e),
                }
            }
            _ => return None,
        }
    };
    Some(ret)
}

fn xe_config(device: &Device) -> Vec<u8> {
    let kernel = device.kernel;
    let values = device.values();
    let mut info = [0u64; DRM_XE_QUERY_CONFIG_MAX_EXEC_QUEUE_PRIORITY + 1];
    info[DRM_XE_QUERY_CONFIG_REV_AND_DEVICE_ID] = values.pci_id as u64;
    let flags = &mut info[DRM_XE_QUERY_CONFIG_FLAGS];
    if values.has_local_mem {
        *flags |= DRM_XE_QUERY_CONFIG_FLAG_HAS_VRAM;
    }
    if kernel >= KernelVersion::V6_15 {
        if values.verx10 >= XE_VERX10 {
            *flags |= DRM_XE_QUERY_CONFIG_FLAG_HAS_CPU_ADDR_MIRROR;
        }
        *flags |= DRM_XE_QUERY_CONFIG_FLAG_HAS_LOW_LATENCY;
    }
    if kernel >= KernelVersion::V7_0 && values.verx10 >= XE_VERX10 {
        *flags |= DRM_XE_QUERY_CONFIG_FLAG_HAS_NO_COMPRESSION_HINT;
    }
    if kernel >= KernelVersion::V7_1 {
        *flags |= DRM_XE_QUERY_CONFIG_FLAG_HAS_DISABLE_STATE_CACHE_PERF_FIX;
        *flags |= DRM_XE_QUERY_CONFIG_FLAG_HAS_PURGING_SUPPORT;
    }
    info[DRM_XE_QUERY_CONFIG_MIN_ALIGNMENT] = device.min_alignment();
    info[DRM_XE_QUERY_CONFIG_VA_BITS] = VA_BITS;
    info[DRM_XE_QUERY_CONFIG_MAX_EXEC_QUEUE_PRIORITY] = XE_EXEC_QUEUE_PRIORITY_NORMAL;
    let header = drm_xe_query_config {
        num_params: info.len() as u32,
        pad: 0,
        info: [],
    };
    let mut out = bytes_of(&header).to_vec();
    for value in info {
        out.extend(value.to_ne_bytes());
    }
    out
}

fn xe_mem_regions(device: &Device) -> Vec<u8> {
    let mut regions = Vec::new();
    let mut system: drm_xe_mem_region = unsafe { std::mem::zeroed() };
    system.mem_class = DRM_XE_MEM_REGION_CLASS_SYSMEM;
    system.instance = 0;
    system.min_page_size = PAGE_SIZE as u32;
    system.total_size = REGION_SIZE;
    regions.push(system);
    if device.values().has_local_mem {
        let mut vram: drm_xe_mem_region = unsafe { std::mem::zeroed() };
        vram.mem_class = DRM_XE_MEM_REGION_CLASS_VRAM;
        vram.instance = device.vram_instance;
        vram.min_page_size = device.min_alignment() as u32;
        vram.total_size = REGION_SIZE;
        vram.cpu_visible_size = device.cpu_visible_size();
        regions.push(vram);
    }
    let header = drm_xe_query_mem_regions {
        num_mem_regions: regions.len() as u32,
        pad: 0,
        mem_regions: [],
    };
    let mut out = bytes_of(&header).to_vec();
    for region in &regions {
        out.extend(bytes_of(region));
    }
    out
}

fn xe_gt_list(device: &Device) -> Vec<u8> {
    let mut gt: drm_xe_gt = unsafe { std::mem::zeroed() };
    gt.type_ = DRM_XE_QUERY_GT_TYPE_MAIN;
    gt.reference_clock = TIMESTAMP_FREQUENCY;
    if let Some(gmdid) = device.graphics_gmdid() {
        gt.ip_ver_major = gmdid.arch;
        gt.ip_ver_minor = gmdid.release;
    }
    let mem_region_mask: u64 = match device.values().has_local_mem {
        true => 0b01 | device.vram_mask() as u64,
        false => 0b01,
    };
    gt.near_mem_regions = match device.values().has_local_mem {
        true => device.vram_mask() as u64,
        false => 0b01,
    };
    gt.far_mem_regions = mem_region_mask ^ gt.near_mem_regions;
    let header = drm_xe_query_gt_list {
        num_gt: 1,
        pad: 0,
        gt_list: [],
    };
    let mut out = bytes_of(&header).to_vec();
    out.extend(bytes_of(&gt));
    out
}

struct TopologyMask {
    kind: u16,
    bits: Vec<u8>,
}

fn xe_topology(device: &Device) -> Vec<u8> {
    let simd16 = device.values().verx10 >= XE_VERX10;
    let eu_type = match simd16 {
        true => DRM_XE_TOPO_SIMD16_EU_PER_DSS,
        false => DRM_XE_TOPO_EU_PER_DSS,
    };
    let eu_mask: u64 = match simd16 {
        true => 0xff,
        false => 0xffff,
    };
    let dss = [[0xff; 8], [0; 8]].concat();
    let geometry = match &device.gt0_topology {
        Some(topology) => topology.geometry.clone(),
        None => dss.clone(),
    };
    let eus = match &device.gt0_topology {
        Some(topology) => topology.eus.clone(),
        None => vec![eu_mask.to_ne_bytes().to_vec()],
    };
    let mut masks = vec![
        TopologyMask {
            kind: DRM_XE_TOPO_DSS_GEOMETRY,
            bits: geometry,
        },
        TopologyMask {
            kind: DRM_XE_TOPO_DSS_COMPUTE,
            bits: dss,
        },
        TopologyMask {
            kind: DRM_XE_TOPO_L3_BANK,
            bits: vec![0xff; 8],
        },
    ];
    masks.extend(eus.into_iter().map(|bits| TopologyMask {
        kind: eu_type,
        bits,
    }));
    let mut out = Vec::new();
    for mask in masks {
        let header = drm_xe_query_topology_mask {
            gt_id: 0,
            type_: mask.kind,
            num_bytes: mask.bits.len() as u32,
            mask: [],
        };
        out.extend(bytes_of(&header));
        out.extend(mask.bits);
    }
    out
}

fn xe_engines(device: &Device) -> Vec<u8> {
    let classes = device.xe_engine_classes();
    let header = drm_xe_query_engines {
        num_engines: classes.len() as u32,
        pad: 0,
        engines: [],
    };
    let mut out = bytes_of(&header).to_vec();
    for &class in classes {
        let mut engine: drm_xe_engine = unsafe { std::mem::zeroed() };
        engine.instance.engine_class = class;
        out.extend(bytes_of(&engine));
    }
    out
}

fn hwconfig() -> Vec<u8> {
    let klv = HwconfigKlv {
        key: HWCONFIG_UNDEFINED_KEY,
        length: 1,
        value: 0,
    };
    bytes_of(&klv).to_vec()
}

unsafe fn xe_query(device: &Device, arg: *mut u8, user: &dyn UserMemory) -> i64 {
    unsafe {
        let query = &mut *arg.cast::<drm_xe_device_query>();
        if query.extensions != 0 || query.reserved != [0; 2] {
            return errno(EINVAL);
        }
        let bytes = match query.query {
            DRM_XE_DEVICE_QUERY_CONFIG => xe_config(device),
            DRM_XE_DEVICE_QUERY_MEM_REGIONS => xe_mem_regions(device),
            DRM_XE_DEVICE_QUERY_GT_LIST => xe_gt_list(device),
            DRM_XE_DEVICE_QUERY_GT_TOPOLOGY => xe_topology(device),
            DRM_XE_DEVICE_QUERY_ENGINES => xe_engines(device),
            DRM_XE_DEVICE_QUERY_HWCONFIG => match device.has_hwconfig_table() {
                true => hwconfig(),
                false => Vec::new(),
            },
            DRM_XE_DEVICE_QUERY_PXP_STATUS if device.kernel >= KernelVersion::V6_15 => {
                if query.size == 0 {
                    query.size = size_of::<drm_xe_query_pxp_status>() as u32;
                    return 0;
                }
                if query.size as usize != size_of::<drm_xe_query_pxp_status>() {
                    return errno(EINVAL);
                }
                return errno(ENODEV);
            }
            DRM_XE_DEVICE_QUERY_ENGINE_CYCLES => {
                let size = size_of::<drm_xe_query_engine_cycles>();
                if query.size == 0 {
                    query.size = size as u32;
                    return 0;
                }
                if query.size as usize != size {
                    return errno(EINVAL);
                }
                let mut response: drm_xe_query_engine_cycles = match user.read_value(query.data) {
                    Ok(response) => response,
                    Err(e) => return errno(e),
                };
                let clocks = [
                    libc::CLOCK_MONOTONIC,
                    libc::CLOCK_MONOTONIC_RAW,
                    libc::CLOCK_REALTIME,
                    libc::CLOCK_BOOTTIME,
                    libc::CLOCK_TAI,
                ];
                if !clocks.contains(&response.clockid) {
                    return errno(EINVAL);
                }
                if !device.xe_hw_engine_lookup(&response.eci) {
                    return errno(EINVAL);
                }
                let mut time: libc::timespec = std::mem::zeroed();
                libc::clock_gettime(response.clockid, &mut time);
                response.cpu_timestamp = time.tv_sec as u64 * 1_000_000_000 + time.tv_nsec as u64;
                response.cpu_delta = 0;
                response.engine_cycles = device.timestamp();
                response.width = match device.values().verx10 >= XE_VERX10 {
                    true => 64,
                    false => 36,
                };
                let field = |offset: usize| query.data.wrapping_add(offset as u64);
                let written = user
                    .write_value(
                        field(offset_of!(drm_xe_query_engine_cycles, cpu_timestamp)),
                        &response.cpu_timestamp,
                    )
                    .and_then(|()| {
                        user.write_value(
                            field(offset_of!(drm_xe_query_engine_cycles, cpu_delta)),
                            &response.cpu_delta,
                        )
                    })
                    .and_then(|()| {
                        user.write_value(
                            field(offset_of!(drm_xe_query_engine_cycles, engine_cycles)),
                            &response.engine_cycles,
                        )
                    })
                    .and_then(|()| {
                        user.write_value(
                            field(offset_of!(drm_xe_query_engine_cycles, width)),
                            &response.width,
                        )
                    });
                return match written {
                    Ok(()) => 0,
                    Err(e) => errno(e),
                };
            }
            DRM_XE_DEVICE_QUERY_UC_FW_VERSION => {
                let size = size_of::<drm_xe_query_uc_fw_version>();
                if query.size == 0 {
                    query.size = size as u32;
                    return 0;
                }
                if query.size as usize != size {
                    return errno(EINVAL);
                }
                let mut response: drm_xe_query_uc_fw_version = match user.read_value(query.data) {
                    Ok(response) => response,
                    Err(e) => return errno(e),
                };
                if response.pad != 0 || response.pad2 != 0 || response.reserved != 0 {
                    return errno(EINVAL);
                }
                let version = match response.uc_type {
                    XE_QUERY_UC_TYPE_GUC_SUBMISSION => device.xe_guc_submission_version().unwrap(),
                    XE_QUERY_UC_TYPE_HUC => return errno(ENODEV),
                    uc_type => unimplemented!("UC firmware type {uc_type}"),
                };
                response.branch_ver = 0;
                response.major_ver = version.major;
                response.minor_ver = version.minor;
                response.patch_ver = version.patch;
                return match user.write_value(query.data, &response) {
                    Ok(()) => 0,
                    Err(e) => errno(e),
                };
            }
            query_id => unimplemented!("xe query {query_id}"),
        };
        if query.size == 0 {
            query.size = bytes.len() as u32;
            return 0;
        }
        if query.size as usize != bytes.len() {
            return errno(EINVAL);
        }
        if user.write(query.data, &bytes).is_err() {
            return match query.query {
                DRM_XE_DEVICE_QUERY_MEM_REGIONS => errno(ENOSPC),
                _ => errno(EFAULT),
            };
        }
    }
    0
}

unsafe fn xe_gem_create(
    device: &Device,
    file: u64,
    buffers: &FileBuffers<Buffer>,
    arg: *mut u8,
) -> i64 {
    let create = unsafe { &mut *arg.cast::<drm_xe_gem_create>() };
    let local = device.values().has_local_mem;
    let vram_mask = device.vram_mask();
    let region_mask: u32 = if local { 0b01 | vram_mask } else { 0b01 };
    if create.pad != [0; 3] || create.reserved != [0; 2] {
        return errno(EINVAL);
    }
    if create.placement & !region_mask != 0 || create.placement == 0 {
        return errno(EINVAL);
    }
    let mut flags = DRM_XE_GEM_CREATE_FLAG_DEFER_BACKING
        | DRM_XE_GEM_CREATE_FLAG_SCANOUT
        | DRM_XE_GEM_CREATE_FLAG_NEEDS_VISIBLE_VRAM;
    if device.kernel >= KernelVersion::V7_0 {
        flags |= DRM_XE_GEM_CREATE_FLAG_NO_COMPRESSION;
    }
    if create.flags & !flags != 0 {
        return errno(EINVAL);
    }
    if create.handle != 0 || create.size == 0 || create.size % PAGE_SIZE != 0 {
        return errno(EINVAL);
    }
    if create.flags & DRM_XE_GEM_CREATE_FLAG_DEFER_BACKING != 0 {
        unimplemented!("DRM_XE_GEM_CREATE_FLAG_DEFER_BACKING");
    }
    if create.flags & DRM_XE_GEM_CREATE_FLAG_NO_COMPRESSION != 0
        && device.values().verx10 < XE_VERX10
    {
        return errno(EOPNOTSUPP);
    }
    let vram = create.placement & vram_mask != 0;
    if create.flags & DRM_XE_GEM_CREATE_FLAG_NEEDS_VISIBLE_VRAM != 0 && !vram {
        return errno(EINVAL);
    }
    if create.cpu_caching == 0 || create.cpu_caching > DRM_XE_GEM_CPU_CACHING_WC {
        return errno(EINVAL);
    }
    if vram && create.cpu_caching != DRM_XE_GEM_CPU_CACHING_WC {
        return errno(EINVAL);
    }
    if create.flags & DRM_XE_GEM_CREATE_FLAG_SCANOUT != 0
        && create.cpu_caching == DRM_XE_GEM_CPU_CACHING_WB
    {
        return errno(EINVAL);
    }
    if create.vm_id != 0 && !device.vm_exists(file, create.vm_id) {
        return errno(ENOENT);
    }
    if vram && device.has_64k_pages() && create.size % SZ_64K != 0 {
        return errno(EINVAL);
    }
    if create.extensions != 0 {
        unimplemented!("GEM create extensions");
    }
    create.handle = buffers
        .create(Buffer::xe(create.size, create.vm_id != 0), create.size)
        .handle;
    0
}

unsafe fn xe_syncs(
    device: &Device,
    file: u64,
    syncs: u64,
    count: u32,
    exec: bool,
    user: &dyn UserMemory,
) -> Result<Vec<XeSignal>, i32> {
    let mut signals = Vec::new();
    for i in 0..count as u64 {
        let address = syncs.wrapping_add(i * size_of::<drm_xe_sync>() as u64);
        let sync: drm_xe_sync = user.read_value(address)?;
        if sync.flags & !DRM_XE_SYNC_FLAG_SIGNAL != 0 || sync.reserved != [0; 2] {
            return Err(EINVAL);
        }
        let signal = sync.flags & DRM_XE_SYNC_FLAG_SIGNAL != 0;
        let addr = unsafe { sync.anon.addr };
        let handle = addr as u32;
        match sync.type_ {
            DRM_XE_SYNC_TYPE_SYNCOBJ => {
                if addr >> 32 != 0 {
                    return Err(EINVAL);
                }
                let object = device.syncobjs.object(file, handle).ok_or(ENOENT)?;
                match signal {
                    true => signals.push(XeSignal::Syncobj { object }),
                    false => {
                        if device.syncobjs.current(file, handle)?.is_none() {
                            return Err(EINVAL);
                        }
                    }
                }
            }
            DRM_XE_SYNC_TYPE_TIMELINE_SYNCOBJ => {
                if addr >> 32 != 0 || sync.timeline_value == 0 {
                    return Err(EINVAL);
                }
                let object = device.syncobjs.object(file, handle).ok_or(ENOENT)?;
                match signal {
                    true => signals.push(XeSignal::Timeline {
                        object,
                        point: sync.timeline_value,
                    }),
                    false => {
                        device
                            .syncobjs
                            .find_fence(file, handle, sync.timeline_value, 0)?;
                    }
                }
            }
            DRM_XE_SYNC_TYPE_USER_FENCE => {
                if !exec {
                    unimplemented!("user fences outside of exec");
                }
                if !signal {
                    return Err(libc::EOPNOTSUPP);
                }
                if addr & 0x7 != 0 {
                    return Err(EINVAL);
                }
                signals.push(XeSignal::UserFence {
                    address: addr,
                    value: sync.timeline_value,
                });
            }
            _ => return Err(EINVAL),
        }
    }
    Ok(signals)
}

unsafe fn xe_madvise(device: &Device, file: u64, arg: *mut u8, user: &dyn UserMemory) -> i64 {
    let madvise = unsafe { &*arg.cast::<drm_xe_madvise>() };
    if !device.vm_exists(file, madvise.vm_id) {
        return errno(EINVAL);
    }
    if madvise.start % PAGE_SIZE != 0 || madvise.range % PAGE_SIZE != 0 || madvise.range < PAGE_SIZE
    {
        return errno(EINVAL);
    }
    if madvise.type_ != DRM_XE_VMA_ATTR_PURGEABLE_STATE || device.kernel < KernelVersion::V7_1 {
        unimplemented!("xe madvise type {}", madvise.type_);
    }
    let purge = unsafe { madvise.anon.purge_state_val };
    let valid = purge.val == DRM_XE_VMA_PURGEABLE_STATE_WILLNEED
        || purge.val == DRM_XE_VMA_PURGEABLE_STATE_DONTNEED;
    if !valid || purge.pad != 0 || madvise.reserved != [0; 2] {
        return errno(EINVAL);
    }
    if purge.retained_ptr != 0 {
        match user.read_value::<u32>(purge.retained_ptr) {
            Ok(0) => {}
            Ok(_) => return errno(EINVAL),
            Err(e) => return errno(e),
        }
        if let Err(e) = user.write_value(purge.retained_ptr, &1u32) {
            return errno(e);
        }
    }
    0
}

unsafe fn xe_vm_bind(device: &Device, file: u64, arg: *mut u8, user: &dyn UserMemory) -> i64 {
    let bind = unsafe { &*arg.cast::<drm_xe_vm_bind>() };
    let user_vm = device.kernel >= KernelVersion::V6_19;
    if user_vm && bind.num_syncs > DRM_XE_MAX_SYNCS {
        return errno(EINVAL);
    }
    if !device.vm_exists(file, bind.vm_id) {
        return errno(EINVAL);
    }
    if bind.exec_queue_id != 0 {
        let queue = device
            .exec_queues
            .lock()
            .get(&file)
            .and_then(|queues| queues.get(&bind.exec_queue_id))
            .copied();
        match queue {
            None => return errno(ENOENT),
            Some(queue) if !queue.bind => return errno(EINVAL),
            Some(queue) if user_vm && queue.vm != bind.vm_id => return errno(EINVAL),
            Some(_) => {}
        }
    }
    let signals = unsafe { xe_syncs(device, file, bind.syncs, bind.num_syncs, false, user) };
    match signals.and_then(|signals| signal_fences(device, signals, user)) {
        Ok(()) => 0,
        Err(e) => errno(e),
    }
}

unsafe fn xe_ioctl(
    device: &Device,
    file: u64,
    func: Func,
    arg: *mut u8,
    user: &dyn UserMemory,
) -> Option<i64> {
    let buffers = &device.buffers.file(file);
    let ret = unsafe {
        match func {
            Func::XeQueryIoctl => xe_query(device, arg, user),
            Func::XeGemCreateIoctl => xe_gem_create(device, file, buffers, arg),
            Func::XeGemMmapOffsetIoctl => {
                let args = &mut *arg.cast::<drm_xe_gem_mmap_offset>();
                let barrier = device.kernel >= KernelVersion::V6_15;
                let allowed = match barrier {
                    true => DRM_XE_MMAP_OFFSET_FLAG_PCI_BARRIER,
                    false => 0,
                };
                if args.extensions != 0 || args.reserved != [0; 2] || args.flags & !allowed != 0 {
                    return Some(errno(EINVAL));
                }
                if args.flags & DRM_XE_MMAP_OFFSET_FLAG_PCI_BARRIER != 0 {
                    unimplemented!("DRM_XE_MMAP_OFFSET_FLAG_PCI_BARRIER");
                }
                match buffers.offset(args.handle) {
                    Some(offset) => {
                        args.offset = offset;
                        0
                    }
                    None => errno(ENOENT),
                }
            }
            Func::XeExecIoctl => {
                let args = &*arg.cast::<drm_xe_exec>();
                match xe_exec(device, file, args, user) {
                    Ok(()) => 0,
                    Err(e) => errno(e),
                }
            }
            Func::XeExecQueueDestroyIoctl => {
                let args = &*arg.cast::<drm_xe_exec_queue_destroy>();
                if args.pad != 0 || args.reserved != [0; 2] {
                    return Some(errno(EINVAL));
                }
                let removed = device
                    .exec_queues
                    .lock()
                    .get_mut(&file)
                    .and_then(|queues| queues.remove(&args.exec_queue_id));
                match removed {
                    Some(_) => 0,
                    None => errno(ENOENT),
                }
            }
            Func::XeExecQueueCreateIoctl => {
                let args = &mut *arg.cast::<drm_xe_exec_queue_create>();
                match xe_exec_queue_create(device, file, args, user) {
                    Ok(()) => 0,
                    Err(e) => errno(e),
                }
            }
            Func::XeVmCreateIoctl => {
                let create = &mut *arg.cast::<drm_xe_vm_create>();
                let flags = DRM_XE_VM_CREATE_FLAG_SCRATCH_PAGE
                    | DRM_XE_VM_CREATE_FLAG_LR_MODE
                    | DRM_XE_VM_CREATE_FLAG_FAULT_MODE
                    | DRM_XE_VM_CREATE_FLAG_NO_VM_OVERCOMMIT;
                if create.extensions != 0 {
                    return Some(errno(EINVAL));
                }
                if create.flags & DRM_XE_VM_CREATE_FLAG_FAULT_MODE != 0 {
                    unimplemented!("DRM_XE_VM_CREATE_FLAG_FAULT_MODE");
                }
                if create.reserved != [0; 2] || create.flags & !flags != 0 {
                    return Some(errno(EINVAL));
                }
                if create.flags != DRM_XE_VM_CREATE_FLAG_SCRATCH_PAGE {
                    unimplemented!("VM create flags {:#x}", create.flags);
                }
                create.vm_id = device.create_vm(file);
                0
            }
            Func::XeVmDestroyIoctl => {
                let destroy = &*arg.cast::<drm_xe_vm_destroy>();
                if destroy.pad != 0 || destroy.reserved != [0; 2] {
                    errno(EINVAL)
                } else if device.destroy_vm(file, destroy.vm_id) {
                    0
                } else {
                    errno(ENOENT)
                }
            }
            Func::XeVmBindIoctl => xe_vm_bind(device, file, arg, user),
            Func::XeVmMadviseIoctl => xe_madvise(device, file, arg, user),
            Func::XeWaitUserFenceIoctl => unimplemented!("DRM_IOCTL_XE_WAIT_USER_FENCE"),
            _ => return None,
        }
    };
    Some(ret)
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        match self.xe {
            true => "xe",
            false => "i915",
        }
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        format!(
            "intel:{}:kernel={}:xe={}:small_bar={}",
            self.values().name,
            self.kernel,
            self.xe as u8,
            self.small_bar as u8
        )
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn desc(&self) -> &'static str {
        match self.xe {
            true if self.kernel >= KernelVersion::V6_16 => "Intel Xe2 Graphics",
            true => "Intel Xe Graphics",
            false => "Intel Graphics",
        }
    }

    fn driver_version(&self) -> DriverVersion {
        match self.xe {
            true => DriverVersion::XE,
            false => DriverVersion::I915,
        }
    }

    fn uevent_driver(&self) -> &'static str {
        self.name()
    }

    fn pci(&self) -> Option<Pci> {
        Some(Pci {
            vendor: 0x8086,
            device: self.values().pci_id,
        })
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        match self.xe {
            true => ioctls::XE,
            false => ioctls::I915,
        }
    }

    fn syncobjs(&self) -> Option<&Syncobjs> {
        Some(&self.syncobjs)
    }

    fn master(&self) -> &Master {
        &self.master
    }

    fn check_feature(&self, feature: DriverFeature) -> bool {
        match feature {
            DriverFeature::Modeset => unimplemented!("intel displays"),
            DriverFeature::SyncobjTimeline => true,
        }
    }

    unsafe fn ioctl(
        &self,
        file: u64,
        func: Func,
        arg: *mut u8,
        user: &dyn UserMemory,
        _prime: &mut dyn Prime,
    ) -> Option<i64> {
        unsafe {
            match self.xe {
                true => xe_ioctl(self, file, func, arg, user),
                false => i915_ioctl(self, file, func, arg, user),
            }
        }
    }

    fn export(&self, file: u64, handle: u32) -> Result<Export, i32> {
        self.buffers
            .file(file)
            .export(handle, |buffer| match buffer.private() {
                true => Err(EPERM),
                false => Ok(()),
            })
    }

    fn import(&self, file: u64, object: u32) -> Option<u32> {
        self.buffers.file(file).import(object)
    }

    fn import_foreign(&self, file: u64, export: &Export) -> NewHandle {
        let object = I915Object {
            pat_set_by_user: false,
            truncatable: false,
        };
        let buffer = match self.xe {
            true => Buffer::xe(export.size, false),
            false => Buffer::i915(export.size, object),
        };
        self.buffers.file(file).create(buffer, export.size)
    }

    fn close_buffer(&self, file: u64, handle: u32) -> bool {
        self.buffers.file(file).close(handle)
    }

    fn object(&self, file: u64, handle: u32) -> Option<u32> {
        self.buffers.file(file).object(handle)
    }

    fn object_has_handles(&self, object: u32) -> bool {
        self.buffers.has_handles(object)
    }

    fn release_export(&self, object: u32) {
        self.buffers.release_export(object);
    }

    fn release_file(&self, file: u64) {
        self.vms.lock().remove(&file);
        self.contexts.lock().remove(&file);
        self.exec_queues.lock().remove(&file);
        self.buffers.release_file(file);
        self.syncobjs.release_file(file);
    }
}
