use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IO;
use crate::uapi::drm::DRM_IOR;
use crate::uapi::drm::DRM_IOW;
use crate::uapi::drm::DRM_IOWR;
use crate::uapi::drm::drm_clip_rect;
use crate::uapi::drm::drm_drawable_t;
use crate::uapi::drm::drm_vblank_seq_type;
use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_uint;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct i915_user_extension {
    pub next_extension: u64,
    pub name: u32,
    pub flags: u32,
    pub rsvd: [u32; 4],
}

pub const I915_ENGINE_CLASS_RENDER: u16 = 0;

pub const I915_ENGINE_CLASS_COPY: u16 = 1;

pub const I915_ENGINE_CLASS_INVALID: i32 = -1;

pub const I915_ENGINE_CLASS_INVALID_NONE: i32 = -1;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct i915_engine_class_instance {
    pub engine_class: u16,
    pub engine_instance: u16,
}

pub type drm_i915_init_t = _drm_i915_init;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _drm_i915_init {
    pub func: c_uint,
    pub mmio_offset: c_uint,
    pub sarea_priv_offset: c_int,
    pub ring_start: c_uint,
    pub ring_end: c_uint,
    pub ring_size: c_uint,
    pub front_offset: c_uint,
    pub back_offset: c_uint,
    pub depth_offset: c_uint,
    pub w: c_uint,
    pub h: c_uint,
    pub pitch: c_uint,
    pub pitch_bits: c_uint,
    pub back_pitch: c_uint,
    pub depth_pitch: c_uint,
    pub cpp: c_uint,
    pub chipset: c_uint,
}

pub const DRM_I915_INIT: u32 = 0x00;
pub const DRM_I915_FLUSH: u32 = 0x01;
pub const DRM_I915_FLIP: u32 = 0x02;
pub const DRM_I915_BATCHBUFFER: u32 = 0x03;
pub const DRM_I915_IRQ_EMIT: u32 = 0x04;
pub const DRM_I915_IRQ_WAIT: u32 = 0x05;
pub const DRM_I915_GETPARAM: u32 = 0x06;
pub const DRM_I915_SETPARAM: u32 = 0x07;
pub const DRM_I915_ALLOC: u32 = 0x08;
pub const DRM_I915_FREE: u32 = 0x09;
pub const DRM_I915_INIT_HEAP: u32 = 0x0a;
pub const DRM_I915_CMDBUFFER: u32 = 0x0b;
pub const DRM_I915_DESTROY_HEAP: u32 = 0x0c;
pub const DRM_I915_SET_VBLANK_PIPE: u32 = 0x0d;
pub const DRM_I915_GET_VBLANK_PIPE: u32 = 0x0e;
pub const DRM_I915_VBLANK_SWAP: u32 = 0x0f;
pub const DRM_I915_HWS_ADDR: u32 = 0x11;
pub const DRM_I915_GEM_INIT: u32 = 0x13;
pub const DRM_I915_GEM_EXECBUFFER: u32 = 0x14;
pub const DRM_I915_GEM_PIN: u32 = 0x15;
pub const DRM_I915_GEM_UNPIN: u32 = 0x16;
pub const DRM_I915_GEM_BUSY: u32 = 0x17;
pub const DRM_I915_GEM_THROTTLE: u32 = 0x18;
pub const DRM_I915_GEM_ENTERVT: u32 = 0x19;
pub const DRM_I915_GEM_LEAVEVT: u32 = 0x1a;
pub const DRM_I915_GEM_CREATE: u32 = 0x1b;
pub const DRM_I915_GEM_PREAD: u32 = 0x1c;
pub const DRM_I915_GEM_PWRITE: u32 = 0x1d;
pub const DRM_I915_GEM_MMAP: u32 = 0x1e;
pub const DRM_I915_GEM_SET_DOMAIN: u32 = 0x1f;
pub const DRM_I915_GEM_SW_FINISH: u32 = 0x20;
pub const DRM_I915_GEM_SET_TILING: u32 = 0x21;
pub const DRM_I915_GEM_GET_TILING: u32 = 0x22;
pub const DRM_I915_GEM_GET_APERTURE: u32 = 0x23;
pub const DRM_I915_GEM_MMAP_GTT: u32 = 0x24;
pub const DRM_I915_GET_PIPE_FROM_CRTC_ID: u32 = 0x25;
pub const DRM_I915_GEM_MADVISE: u32 = 0x26;
pub const DRM_I915_OVERLAY_PUT_IMAGE: u32 = 0x27;
pub const DRM_I915_OVERLAY_ATTRS: u32 = 0x28;
pub const DRM_I915_GEM_EXECBUFFER2: u32 = 0x29;
pub const DRM_I915_GEM_EXECBUFFER2_WR: u32 = DRM_I915_GEM_EXECBUFFER2;
pub const DRM_I915_GET_SPRITE_COLORKEY: u32 = 0x2a;
pub const DRM_I915_SET_SPRITE_COLORKEY: u32 = 0x2b;
pub const DRM_I915_GEM_WAIT: u32 = 0x2c;
pub const DRM_I915_GEM_CONTEXT_CREATE: u32 = 0x2d;
pub const DRM_I915_GEM_CONTEXT_DESTROY: u32 = 0x2e;
pub const DRM_I915_GEM_SET_CACHING: u32 = 0x2f;
pub const DRM_I915_GEM_GET_CACHING: u32 = 0x30;
pub const DRM_I915_REG_READ: u32 = 0x31;
pub const DRM_I915_GET_RESET_STATS: u32 = 0x32;
pub const DRM_I915_GEM_USERPTR: u32 = 0x33;
pub const DRM_I915_GEM_CONTEXT_GETPARAM: u32 = 0x34;
pub const DRM_I915_GEM_CONTEXT_SETPARAM: u32 = 0x35;
pub const DRM_I915_PERF_OPEN: u32 = 0x36;
pub const DRM_I915_PERF_ADD_CONFIG: u32 = 0x37;
pub const DRM_I915_PERF_REMOVE_CONFIG: u32 = 0x38;
pub const DRM_I915_QUERY: u32 = 0x39;
pub const DRM_I915_GEM_VM_CREATE: u32 = 0x3a;
pub const DRM_I915_GEM_VM_DESTROY: u32 = 0x3b;
pub const DRM_I915_GEM_CREATE_EXT: u32 = 0x3c;

pub const DRM_IOCTL_I915_INIT: u32 = DRM_IOW::<drm_i915_init_t>(DRM_COMMAND_BASE + DRM_I915_INIT);
pub const DRM_IOCTL_I915_FLUSH: u32 = DRM_IO(DRM_COMMAND_BASE + DRM_I915_FLUSH);
pub const DRM_IOCTL_I915_FLIP: u32 = DRM_IO(DRM_COMMAND_BASE + DRM_I915_FLIP);
pub const DRM_IOCTL_I915_BATCHBUFFER: u32 =
    DRM_IOW::<drm_i915_batchbuffer_t>(DRM_COMMAND_BASE + DRM_I915_BATCHBUFFER);
pub const DRM_IOCTL_I915_IRQ_EMIT: u32 =
    DRM_IOWR::<drm_i915_irq_emit_t>(DRM_COMMAND_BASE + DRM_I915_IRQ_EMIT);
pub const DRM_IOCTL_I915_IRQ_WAIT: u32 =
    DRM_IOW::<drm_i915_irq_wait_t>(DRM_COMMAND_BASE + DRM_I915_IRQ_WAIT);
pub const DRM_IOCTL_I915_GETPARAM: u32 =
    DRM_IOWR::<drm_i915_getparam_t>(DRM_COMMAND_BASE + DRM_I915_GETPARAM);
pub const DRM_IOCTL_I915_SETPARAM: u32 =
    DRM_IOW::<drm_i915_setparam_t>(DRM_COMMAND_BASE + DRM_I915_SETPARAM);
pub const DRM_IOCTL_I915_ALLOC: u32 =
    DRM_IOWR::<drm_i915_mem_alloc_t>(DRM_COMMAND_BASE + DRM_I915_ALLOC);
pub const DRM_IOCTL_I915_FREE: u32 =
    DRM_IOW::<drm_i915_mem_free_t>(DRM_COMMAND_BASE + DRM_I915_FREE);
pub const DRM_IOCTL_I915_INIT_HEAP: u32 =
    DRM_IOW::<drm_i915_mem_init_heap_t>(DRM_COMMAND_BASE + DRM_I915_INIT_HEAP);
pub const DRM_IOCTL_I915_CMDBUFFER: u32 =
    DRM_IOW::<drm_i915_cmdbuffer_t>(DRM_COMMAND_BASE + DRM_I915_CMDBUFFER);
pub const DRM_IOCTL_I915_DESTROY_HEAP: u32 =
    DRM_IOW::<drm_i915_mem_destroy_heap_t>(DRM_COMMAND_BASE + DRM_I915_DESTROY_HEAP);
pub const DRM_IOCTL_I915_SET_VBLANK_PIPE: u32 =
    DRM_IOW::<drm_i915_vblank_pipe_t>(DRM_COMMAND_BASE + DRM_I915_SET_VBLANK_PIPE);
pub const DRM_IOCTL_I915_GET_VBLANK_PIPE: u32 =
    DRM_IOR::<drm_i915_vblank_pipe_t>(DRM_COMMAND_BASE + DRM_I915_GET_VBLANK_PIPE);
pub const DRM_IOCTL_I915_VBLANK_SWAP: u32 =
    DRM_IOWR::<drm_i915_vblank_swap_t>(DRM_COMMAND_BASE + DRM_I915_VBLANK_SWAP);
pub const DRM_IOCTL_I915_HWS_ADDR: u32 =
    DRM_IOW::<drm_i915_gem_init>(DRM_COMMAND_BASE + DRM_I915_HWS_ADDR);
pub const DRM_IOCTL_I915_GEM_INIT: u32 =
    DRM_IOW::<drm_i915_gem_init>(DRM_COMMAND_BASE + DRM_I915_GEM_INIT);
pub const DRM_IOCTL_I915_GEM_EXECBUFFER: u32 =
    DRM_IOW::<drm_i915_gem_execbuffer>(DRM_COMMAND_BASE + DRM_I915_GEM_EXECBUFFER);
pub const DRM_IOCTL_I915_GEM_EXECBUFFER2: u32 =
    DRM_IOW::<drm_i915_gem_execbuffer2>(DRM_COMMAND_BASE + DRM_I915_GEM_EXECBUFFER2);
pub const DRM_IOCTL_I915_GEM_EXECBUFFER2_WR: u32 =
    DRM_IOWR::<drm_i915_gem_execbuffer2>(DRM_COMMAND_BASE + DRM_I915_GEM_EXECBUFFER2_WR);
pub const DRM_IOCTL_I915_GEM_PIN: u32 =
    DRM_IOWR::<drm_i915_gem_pin>(DRM_COMMAND_BASE + DRM_I915_GEM_PIN);
pub const DRM_IOCTL_I915_GEM_UNPIN: u32 =
    DRM_IOW::<drm_i915_gem_unpin>(DRM_COMMAND_BASE + DRM_I915_GEM_UNPIN);
pub const DRM_IOCTL_I915_GEM_BUSY: u32 =
    DRM_IOWR::<drm_i915_gem_busy>(DRM_COMMAND_BASE + DRM_I915_GEM_BUSY);
pub const DRM_IOCTL_I915_GEM_SET_CACHING: u32 =
    DRM_IOW::<drm_i915_gem_caching>(DRM_COMMAND_BASE + DRM_I915_GEM_SET_CACHING);
pub const DRM_IOCTL_I915_GEM_GET_CACHING: u32 =
    DRM_IOWR::<drm_i915_gem_caching>(DRM_COMMAND_BASE + DRM_I915_GEM_GET_CACHING);
pub const DRM_IOCTL_I915_GEM_THROTTLE: u32 = DRM_IO(DRM_COMMAND_BASE + DRM_I915_GEM_THROTTLE);
pub const DRM_IOCTL_I915_GEM_ENTERVT: u32 = DRM_IO(DRM_COMMAND_BASE + DRM_I915_GEM_ENTERVT);
pub const DRM_IOCTL_I915_GEM_LEAVEVT: u32 = DRM_IO(DRM_COMMAND_BASE + DRM_I915_GEM_LEAVEVT);
pub const DRM_IOCTL_I915_GEM_CREATE: u32 =
    DRM_IOWR::<drm_i915_gem_create>(DRM_COMMAND_BASE + DRM_I915_GEM_CREATE);
pub const DRM_IOCTL_I915_GEM_CREATE_EXT: u32 =
    DRM_IOWR::<drm_i915_gem_create_ext>(DRM_COMMAND_BASE + DRM_I915_GEM_CREATE_EXT);
pub const DRM_IOCTL_I915_GEM_PREAD: u32 =
    DRM_IOW::<drm_i915_gem_pread>(DRM_COMMAND_BASE + DRM_I915_GEM_PREAD);
pub const DRM_IOCTL_I915_GEM_PWRITE: u32 =
    DRM_IOW::<drm_i915_gem_pwrite>(DRM_COMMAND_BASE + DRM_I915_GEM_PWRITE);
pub const DRM_IOCTL_I915_GEM_MMAP: u32 =
    DRM_IOWR::<drm_i915_gem_mmap>(DRM_COMMAND_BASE + DRM_I915_GEM_MMAP);
pub const DRM_IOCTL_I915_GEM_MMAP_GTT: u32 =
    DRM_IOWR::<drm_i915_gem_mmap_gtt>(DRM_COMMAND_BASE + DRM_I915_GEM_MMAP_GTT);
pub const DRM_IOCTL_I915_GEM_MMAP_OFFSET: u32 =
    DRM_IOWR::<drm_i915_gem_mmap_offset>(DRM_COMMAND_BASE + DRM_I915_GEM_MMAP_GTT);
pub const DRM_IOCTL_I915_GEM_SET_DOMAIN: u32 =
    DRM_IOW::<drm_i915_gem_set_domain>(DRM_COMMAND_BASE + DRM_I915_GEM_SET_DOMAIN);
pub const DRM_IOCTL_I915_GEM_SW_FINISH: u32 =
    DRM_IOW::<drm_i915_gem_sw_finish>(DRM_COMMAND_BASE + DRM_I915_GEM_SW_FINISH);
pub const DRM_IOCTL_I915_GEM_SET_TILING: u32 =
    DRM_IOWR::<drm_i915_gem_set_tiling>(DRM_COMMAND_BASE + DRM_I915_GEM_SET_TILING);
pub const DRM_IOCTL_I915_GEM_GET_TILING: u32 =
    DRM_IOWR::<drm_i915_gem_get_tiling>(DRM_COMMAND_BASE + DRM_I915_GEM_GET_TILING);
pub const DRM_IOCTL_I915_GEM_GET_APERTURE: u32 =
    DRM_IOR::<drm_i915_gem_get_aperture>(DRM_COMMAND_BASE + DRM_I915_GEM_GET_APERTURE);
pub const DRM_IOCTL_I915_GET_PIPE_FROM_CRTC_ID: u32 =
    DRM_IOWR::<drm_i915_get_pipe_from_crtc_id>(DRM_COMMAND_BASE + DRM_I915_GET_PIPE_FROM_CRTC_ID);
pub const DRM_IOCTL_I915_GEM_MADVISE: u32 =
    DRM_IOWR::<drm_i915_gem_madvise>(DRM_COMMAND_BASE + DRM_I915_GEM_MADVISE);
pub const DRM_IOCTL_I915_OVERLAY_PUT_IMAGE: u32 =
    DRM_IOW::<drm_intel_overlay_put_image>(DRM_COMMAND_BASE + DRM_I915_OVERLAY_PUT_IMAGE);
pub const DRM_IOCTL_I915_OVERLAY_ATTRS: u32 =
    DRM_IOWR::<drm_intel_overlay_attrs>(DRM_COMMAND_BASE + DRM_I915_OVERLAY_ATTRS);
pub const DRM_IOCTL_I915_SET_SPRITE_COLORKEY: u32 =
    DRM_IOWR::<drm_intel_sprite_colorkey>(DRM_COMMAND_BASE + DRM_I915_SET_SPRITE_COLORKEY);
pub const DRM_IOCTL_I915_GET_SPRITE_COLORKEY: u32 =
    DRM_IOWR::<drm_intel_sprite_colorkey>(DRM_COMMAND_BASE + DRM_I915_GET_SPRITE_COLORKEY);
pub const DRM_IOCTL_I915_GEM_WAIT: u32 =
    DRM_IOWR::<drm_i915_gem_wait>(DRM_COMMAND_BASE + DRM_I915_GEM_WAIT);
pub const DRM_IOCTL_I915_GEM_CONTEXT_CREATE: u32 =
    DRM_IOWR::<drm_i915_gem_context_create>(DRM_COMMAND_BASE + DRM_I915_GEM_CONTEXT_CREATE);
pub const DRM_IOCTL_I915_GEM_CONTEXT_CREATE_EXT: u32 =
    DRM_IOWR::<drm_i915_gem_context_create_ext>(DRM_COMMAND_BASE + DRM_I915_GEM_CONTEXT_CREATE);
pub const DRM_IOCTL_I915_GEM_CONTEXT_DESTROY: u32 =
    DRM_IOW::<drm_i915_gem_context_destroy>(DRM_COMMAND_BASE + DRM_I915_GEM_CONTEXT_DESTROY);
pub const DRM_IOCTL_I915_REG_READ: u32 =
    DRM_IOWR::<drm_i915_reg_read>(DRM_COMMAND_BASE + DRM_I915_REG_READ);
pub const DRM_IOCTL_I915_GET_RESET_STATS: u32 =
    DRM_IOWR::<drm_i915_reset_stats>(DRM_COMMAND_BASE + DRM_I915_GET_RESET_STATS);
pub const DRM_IOCTL_I915_GEM_USERPTR: u32 =
    DRM_IOWR::<drm_i915_gem_userptr>(DRM_COMMAND_BASE + DRM_I915_GEM_USERPTR);
pub const DRM_IOCTL_I915_GEM_CONTEXT_GETPARAM: u32 =
    DRM_IOWR::<drm_i915_gem_context_param>(DRM_COMMAND_BASE + DRM_I915_GEM_CONTEXT_GETPARAM);
pub const DRM_IOCTL_I915_GEM_CONTEXT_SETPARAM: u32 =
    DRM_IOWR::<drm_i915_gem_context_param>(DRM_COMMAND_BASE + DRM_I915_GEM_CONTEXT_SETPARAM);
pub const DRM_IOCTL_I915_PERF_OPEN: u32 =
    DRM_IOW::<drm_i915_perf_open_param>(DRM_COMMAND_BASE + DRM_I915_PERF_OPEN);
pub const DRM_IOCTL_I915_PERF_ADD_CONFIG: u32 =
    DRM_IOW::<drm_i915_perf_oa_config>(DRM_COMMAND_BASE + DRM_I915_PERF_ADD_CONFIG);
pub const DRM_IOCTL_I915_PERF_REMOVE_CONFIG: u32 =
    DRM_IOW::<u64>(DRM_COMMAND_BASE + DRM_I915_PERF_REMOVE_CONFIG);
pub const DRM_IOCTL_I915_QUERY: u32 = DRM_IOWR::<drm_i915_query>(DRM_COMMAND_BASE + DRM_I915_QUERY);
pub const DRM_IOCTL_I915_GEM_VM_CREATE: u32 =
    DRM_IOWR::<drm_i915_gem_vm_control>(DRM_COMMAND_BASE + DRM_I915_GEM_VM_CREATE);
pub const DRM_IOCTL_I915_GEM_VM_DESTROY: u32 =
    DRM_IOW::<drm_i915_gem_vm_control>(DRM_COMMAND_BASE + DRM_I915_GEM_VM_DESTROY);

pub type drm_i915_batchbuffer_t = drm_i915_batchbuffer;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_batchbuffer {
    pub start: c_int,
    pub used: c_int,
    pub DR1: c_int,
    pub DR4: c_int,
    pub num_cliprects: c_int,
    pub cliprects: *mut drm_clip_rect,
}

pub type drm_i915_cmdbuffer_t = _drm_i915_cmdbuffer;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _drm_i915_cmdbuffer {
    pub buf: *mut c_char,
    pub sz: c_int,
    pub DR1: c_int,
    pub DR4: c_int,
    pub num_cliprects: c_int,
    pub cliprects: *mut drm_clip_rect,
}

pub type drm_i915_irq_emit_t = drm_i915_irq_emit;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_irq_emit {
    pub irq_seq: *mut c_int,
}

pub type drm_i915_irq_wait_t = drm_i915_irq_wait;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_irq_wait {
    pub irq_seq: c_int,
}

pub const I915_PARAM_CHIPSET_ID: i32 = 4;
pub const I915_PARAM_HAS_GEM: i32 = 5;
pub const I915_PARAM_HAS_PAGEFLIPPING: i32 = 8;
pub const I915_PARAM_HAS_EXECBUF2: i32 = 9;
pub const I915_PARAM_HAS_RELAXED_FENCING: i32 = 12;
pub const I915_PARAM_HAS_COHERENT_RINGS: i32 = 13;
pub const I915_PARAM_HAS_RELAXED_DELTA: i32 = 15;
pub const I915_PARAM_HAS_GEN7_SOL_RESET: i32 = 16;
pub const I915_PARAM_HAS_WAIT_TIMEOUT: i32 = 19;
pub const I915_PARAM_HAS_PRIME_VMAP_FLUSH: i32 = 21;
pub const I915_PARAM_HAS_PINNED_BATCHES: i32 = 24;
pub const I915_PARAM_HAS_EXEC_NO_RELOC: i32 = 25;
pub const I915_PARAM_HAS_EXEC_HANDLE_LUT: i32 = 26;
pub const I915_PARAM_HAS_COHERENT_PHYS_GTT: i32 = 29;
pub const I915_PARAM_MMAP_VERSION: i32 = 30;
pub const I915_PARAM_REVISION: i32 = 32;
pub const I915_PARAM_EU_TOTAL: i32 = 34;
pub const I915_PARAM_HAS_EXEC_SOFTPIN: i32 = 37;
pub const I915_PARAM_MMAP_GTT_VERSION: i32 = 40;

pub const I915_PARAM_HUC_STATUS: i32 = 42;

pub const I915_PARAM_HAS_EXEC_ASYNC: i32 = 43;

pub const I915_PARAM_HAS_EXEC_FENCE: i32 = 44;

pub const I915_PARAM_HAS_EXEC_CAPTURE: i32 = 45;

pub const I915_PARAM_SLICE_MASK: i32 = 46;

pub const I915_PARAM_SUBSLICE_MASK: i32 = 47;

pub const I915_PARAM_HAS_EXEC_BATCH_FIRST: i32 = 48;

pub const I915_PARAM_HAS_EXEC_FENCE_ARRAY: i32 = 49;

pub const I915_PARAM_HAS_CONTEXT_ISOLATION: i32 = 50;

pub const I915_PARAM_CS_TIMESTAMP_FREQUENCY: i32 = 51;

pub const I915_PARAM_HAS_EXEC_SUBMIT_FENCE: i32 = 53;

pub const I915_PARAM_PERF_REVISION: i32 = 54;

pub const I915_PARAM_HAS_EXEC_TIMELINE_FENCES: i32 = 55;

pub const I915_PARAM_HAS_USERPTR_PROBE: i32 = 56;

pub const I915_PARAM_PXP_STATUS: i32 = 58;

pub const I915_PARAM_HAS_CONTEXT_FREQ_HINT: i32 = 59;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_getparam {
    pub param: i32,
    pub value: *mut c_int,
}

pub type drm_i915_getparam_t = drm_i915_getparam;

pub type drm_i915_setparam_t = drm_i915_setparam;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_setparam {
    pub param: c_int,
    pub value: c_int,
}

pub type drm_i915_mem_alloc_t = drm_i915_mem_alloc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_mem_alloc {
    pub region: c_int,
    pub alignment: c_int,
    pub size: c_int,
    pub region_offset: *mut c_int,
}

pub type drm_i915_mem_free_t = drm_i915_mem_free;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_mem_free {
    pub region: c_int,
    pub region_offset: c_int,
}

pub type drm_i915_mem_init_heap_t = drm_i915_mem_init_heap;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_mem_init_heap {
    pub region: c_int,
    pub size: c_int,
    pub start: c_int,
}

pub type drm_i915_mem_destroy_heap_t = drm_i915_mem_destroy_heap;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_mem_destroy_heap {
    pub region: c_int,
}

pub type drm_i915_vblank_pipe_t = drm_i915_vblank_pipe;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_vblank_pipe {
    pub pipe: c_int,
}

pub type drm_i915_vblank_swap_t = drm_i915_vblank_swap;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_vblank_swap {
    pub drawable: drm_drawable_t,
    pub seqtype: drm_vblank_seq_type,
    pub sequence: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_init {
    pub gtt_start: u64,
    pub gtt_end: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_i915_gem_create {
    pub size: u64,
    pub handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_pread {
    pub handle: u32,
    pub pad: u32,
    pub offset: u64,
    pub size: u64,
    pub data_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_pwrite {
    pub handle: u32,
    pub pad: u32,
    pub offset: u64,
    pub size: u64,
    pub data_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_mmap {
    pub handle: u32,
    pub pad: u32,
    pub offset: u64,
    pub size: u64,
    pub addr_ptr: u64,
    pub flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_mmap_gtt {
    pub handle: u32,
    pub pad: u32,
    pub offset: u64,
}

pub const I915_MMAP_OFFSET_GTT: u64 = 0;
pub const I915_MMAP_OFFSET_WC: u64 = 1;
pub const I915_MMAP_OFFSET_WB: u64 = 2;
pub const I915_MMAP_OFFSET_UC: u64 = 3;
pub const I915_MMAP_OFFSET_FIXED: u64 = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_mmap_offset {
    pub handle: u32,
    pub pad: u32,
    pub offset: u64,
    pub flags: u64,
    pub extensions: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_i915_gem_set_domain {
    pub handle: u32,
    pub read_domains: u32,
    pub write_domain: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_sw_finish {
    pub handle: u32,
}

pub const I915_GEM_DOMAIN_CPU: u32 = 0x00000001;
pub const I915_GEM_DOMAIN_RENDER: u32 = 0x00000002;
pub const I915_GEM_DOMAIN_SAMPLER: u32 = 0x00000004;
pub const I915_GEM_DOMAIN_COMMAND: u32 = 0x00000008;
pub const I915_GEM_DOMAIN_INSTRUCTION: u32 = 0x00000010;
pub const I915_GEM_DOMAIN_VERTEX: u32 = 0x00000020;
pub const I915_GEM_DOMAIN_GTT: u32 = 0x00000040;
pub const I915_GEM_DOMAIN_WC: u32 = 0x00000080;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_execbuffer {
    pub buffers_ptr: u64,
    pub buffer_count: u32,
    pub batch_start_offset: u32,
    pub batch_len: u32,
    pub DR1: u32,
    pub DR4: u32,
    pub num_cliprects: u32,
    pub cliprects_ptr: u64,
}

pub const EXEC_OBJECT_NEEDS_GTT: u64 = 1 << 1;
pub const EXEC_OBJECT_WRITE: u64 = 1 << 2;
pub const EXEC_OBJECT_PINNED: u64 = 1 << 4;
pub const EXEC_OBJECT_PAD_TO_SIZE: u64 = 1 << 5;
pub const EXEC_OBJECT_CAPTURE: u64 = 1 << 7;
pub const __EXEC_OBJECT_UNKNOWN_FLAGS: u64 = (EXEC_OBJECT_CAPTURE << 1).wrapping_neg();

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub union drm_i915_gem_exec_object2__anon {
    pub rsvd1: u64,
    pub pad_to_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_i915_gem_exec_object2 {
    pub handle: u32,
    pub relocation_count: u32,
    pub relocs_ptr: u64,
    pub alignment: u64,
    pub offset: u64,
    pub flags: u64,
    pub anon: drm_i915_gem_exec_object2__anon,
    pub rsvd2: u64,
}

pub const I915_EXEC_FENCE_WAIT: u32 = 1 << 0;
pub const I915_EXEC_FENCE_SIGNAL: u32 = 1 << 1;
pub const __I915_EXEC_FENCE_UNKNOWN_FLAGS: u32 = (I915_EXEC_FENCE_SIGNAL << 1).wrapping_neg();

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_i915_gem_exec_fence {
    pub handle: u32,
    pub flags: u32,
}

pub const I915_EXEC_RING_MASK: u64 = 0x3f;

pub const I915_EXEC_CONSTANTS_MASK: u64 = 3 << 6;

pub const I915_EXEC_SECURE: u64 = 1 << 9;

pub const I915_EXEC_NO_RELOC: u64 = 1 << 11;

pub const I915_EXEC_HANDLE_LUT: u64 = 1 << 12;

pub const I915_EXEC_RESOURCE_STREAMER: u64 = 1 << 15;

pub const I915_EXEC_FENCE_IN: u64 = 1 << 16;

pub const I915_EXEC_FENCE_OUT: u64 = 1 << 17;

pub const I915_EXEC_BATCH_FIRST: u64 = 1 << 18;

pub const I915_EXEC_FENCE_ARRAY: u64 = 1 << 19;

pub const I915_EXEC_FENCE_SUBMIT: u64 = 1 << 20;

pub const I915_EXEC_USE_EXTENSIONS: u64 = 1 << 21;
pub const __I915_EXEC_UNKNOWN_FLAGS: u64 = (I915_EXEC_USE_EXTENSIONS << 1).wrapping_neg();

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_execbuffer2 {
    pub buffers_ptr: u64,
    pub buffer_count: u32,
    pub batch_start_offset: u32,
    pub batch_len: u32,
    pub DR1: u32,
    pub DR4: u32,
    pub num_cliprects: u32,
    pub cliprects_ptr: u64,
    pub flags: u64,
    pub rsvd1: u64,
    pub rsvd2: u64,
}

pub const I915_EXEC_CONTEXT_ID_MASK: u64 = 0xffffffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_pin {
    pub handle: u32,
    pub pad: u32,
    pub alignment: u64,
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_unpin {
    pub handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_busy {
    pub handle: u32,
    pub busy: u32,
}

pub const I915_CACHING_NONE: u32 = 0;
pub const I915_CACHING_CACHED: u32 = 1;
pub const I915_CACHING_DISPLAY: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_caching {
    pub handle: u32,
    pub caching: u32,
}

pub const I915_TILING_NONE: u32 = 0;
pub const I915_TILING_X: u32 = 1;
pub const I915_TILING_Y: u32 = 2;
pub const I915_TILING_LAST: u32 = I915_TILING_Y;

pub const I915_BIT_6_SWIZZLE_NONE: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_i915_gem_set_tiling {
    pub handle: u32,
    pub tiling_mode: u32,
    pub stride: u32,
    pub swizzle_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_get_tiling {
    pub handle: u32,
    pub tiling_mode: u32,
    pub swizzle_mode: u32,
    pub phys_swizzle_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_get_aperture {
    pub aper_size: u64,
    pub aper_available_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_get_pipe_from_crtc_id {
    pub crtc_id: u32,
    pub pipe: u32,
}

pub const I915_MADV_WILLNEED: u32 = 0;
pub const I915_MADV_DONTNEED: u32 = 1;
pub const __I915_MADV_PURGED: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_madvise {
    pub handle: u32,
    pub madv: u32,
    pub retained: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_intel_overlay_put_image {
    pub flags: u32,
    pub bo_handle: u32,
    pub stride_Y: u16,
    pub stride_UV: u16,
    pub offset_Y: u32,
    pub offset_U: u32,
    pub offset_V: u32,
    pub src_width: u16,
    pub src_height: u16,
    pub src_scan_width: u16,
    pub src_scan_height: u16,
    pub crtc_id: u32,
    pub dst_x: u16,
    pub dst_y: u16,
    pub dst_width: u16,
    pub dst_height: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_intel_overlay_attrs {
    pub flags: u32,
    pub color_key: u32,
    pub brightness: i32,
    pub contrast: u32,
    pub saturation: u32,
    pub gamma0: u32,
    pub gamma1: u32,
    pub gamma2: u32,
    pub gamma3: u32,
    pub gamma4: u32,
    pub gamma5: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_intel_sprite_colorkey {
    pub plane_id: u32,
    pub min_value: u32,
    pub channel_mask: u32,
    pub max_value: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_wait {
    pub bo_handle: u32,
    pub flags: u32,
    pub timeout_ns: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_context_create {
    pub ctx_id: u32,
    pub pad: u32,
}

pub const I915_CONTEXT_CREATE_FLAGS_USE_EXTENSIONS: u32 = 1 << 0;
pub const I915_CONTEXT_CREATE_FLAGS_SINGLE_TIMELINE: u32 = 1 << 1;
pub const I915_CONTEXT_CREATE_FLAGS_UNKNOWN: u32 =
    (I915_CONTEXT_CREATE_FLAGS_SINGLE_TIMELINE << 1).wrapping_neg();

pub const I915_CONTEXT_CREATE_EXT_SETPARAM: u32 = 0;
pub const I915_CONTEXT_CREATE_EXT_CLONE: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_context_create_ext {
    pub ctx_id: u32,
    pub flags: u32,
    pub extensions: u64,
}

pub const I915_CONTEXT_PARAM_GTT_SIZE: u64 = 0x3;
pub const I915_CONTEXT_PARAM_PRIORITY: u64 = 0x6;
pub const I915_CONTEXT_MAX_USER_PRIORITY: i64 = 1023;
pub const I915_CONTEXT_DEFAULT_PRIORITY: i64 = 0;
pub const I915_CONTEXT_MIN_USER_PRIORITY: i64 = -1023;
pub const I915_CONTEXT_PARAM_SSEU: u64 = 0x7;

pub const I915_CONTEXT_PARAM_RECOVERABLE: u64 = 0x8;

pub const I915_CONTEXT_PARAM_VM: u64 = 0x9;

pub const I915_CONTEXT_PARAM_ENGINES: u64 = 0xa;

pub const I915_CONTEXT_PARAM_PROTECTED_CONTENT: u64 = 0xd;

pub const I915_CONTEXT_PARAM_LOW_LATENCY: u64 = 0xe;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_i915_gem_context_param {
    pub ctx_id: u32,
    pub size: u32,
    pub param: u64,
    pub value: u64,
}

pub const I915_CONTEXT_SSEU_FLAG_ENGINE_INDEX: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_i915_gem_context_param_sseu {
    pub engine: i915_engine_class_instance,
    pub flags: u32,
    pub slice_mask: u64,
    pub subslice_mask: u64,
    pub min_eus_per_subslice: u16,
    pub max_eus_per_subslice: u16,
    pub rsvd: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct i915_context_param_engines {
    pub extensions: u64,
    pub engines: [i915_engine_class_instance; 0],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_i915_gem_context_create_ext_setparam {
    pub base: i915_user_extension,
    pub param: drm_i915_gem_context_param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_context_destroy {
    pub ctx_id: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_vm_control {
    pub extensions: u64,
    pub flags: u32,
    pub vm_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_i915_query_guc_submission_version {
    pub branch: u32,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

pub const I915_REG_READ_8B_WA: u64 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_reg_read {
    pub offset: u64,
    pub val: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_reset_stats {
    pub ctx_id: u32,
    pub flags: u32,
    pub reset_count: u32,
    pub batch_active: u32,
    pub batch_pending: u32,
    pub pad: u32,
}

pub const I915_USERPTR_READ_ONLY: u32 = 0x1;
pub const I915_USERPTR_PROBE: u32 = 0x2;
pub const I915_USERPTR_UNSYNCHRONIZED: u32 = 0x80000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_userptr {
    pub user_ptr: u64,
    pub user_size: u64,
    pub flags: u32,
    pub handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_perf_open_param {
    pub flags: u32,
    pub num_properties: u32,
    pub properties_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_perf_oa_config {
    pub uuid: [c_char; 36],
    pub n_mux_regs: u32,
    pub n_boolean_regs: u32,
    pub n_flex_regs: u32,
    pub mux_regs_ptr: u64,
    pub boolean_regs_ptr: u64,
    pub flex_regs_ptr: u64,
}

pub const DRM_I915_QUERY_TOPOLOGY_INFO: u64 = 1;
pub const DRM_I915_QUERY_ENGINE_INFO: u64 = 2;
pub const DRM_I915_QUERY_MEMORY_REGIONS: u64 = 4;
pub const DRM_I915_QUERY_HWCONFIG_BLOB: u64 = 5;
pub const DRM_I915_QUERY_GEOMETRY_SUBSLICES: u64 = 6;
pub const DRM_I915_QUERY_GUC_SUBMISSION_VERSION: u64 = 7;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_i915_query_item {
    pub query_id: u64,
    pub length: i32,
    pub flags: u32,
    pub data_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_query {
    pub num_items: u32,
    pub flags: u32,
    pub items_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_i915_query_topology_info {
    pub flags: u16,
    pub max_slices: u16,
    pub max_subslices: u16,
    pub max_eus_per_subslice: u16,
    pub subslice_offset: u16,
    pub subslice_stride: u16,
    pub eu_offset: u16,
    pub eu_stride: u16,
    pub data: [u8; 0],
}

pub const I915_ENGINE_INFO_HAS_LOGICAL_INSTANCE: u64 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_i915_engine_info {
    pub engine: i915_engine_class_instance,
    pub rsvd0: u32,
    pub flags: u64,
    pub capabilities: u64,
    pub logical_instance: u16,
    pub rsvd1: [u16; 3],
    pub rsvd2: [u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_i915_query_engine_info {
    pub num_engines: u32,
    pub rsvd: [u32; 3],
    pub engines: [drm_i915_engine_info; 0],
}

pub const I915_MEMORY_CLASS_SYSTEM: u16 = 0;
pub const I915_MEMORY_CLASS_DEVICE: u16 = 1;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_i915_gem_memory_class_instance {
    pub memory_class: u16,
    pub memory_instance: u16,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_i915_memory_region_info__anon__anon {
    pub probed_cpu_visible_size: u64,
    pub unallocated_cpu_visible_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub union drm_i915_memory_region_info__anon {
    pub rsvd1: [u64; 8],
    pub anon: drm_i915_memory_region_info__anon__anon,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_i915_memory_region_info {
    pub region: drm_i915_gem_memory_class_instance,
    pub rsvd0: u32,
    pub probed_size: u64,
    pub unallocated_size: u64,
    pub anon: drm_i915_memory_region_info__anon,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_query_memory_regions {
    pub num_regions: u32,
    pub rsvd: [u32; 3],
    pub regions: [drm_i915_memory_region_info; 0],
}

unsafe impl bytemuck::Zeroable for drm_i915_query_memory_regions {}

unsafe impl bytemuck::AnyBitPattern for drm_i915_query_memory_regions {}

pub const I915_GEM_CREATE_EXT_FLAG_NEEDS_CPU_ACCESS: u32 = 1 << 0;

pub const I915_GEM_CREATE_EXT_MEMORY_REGIONS: u32 = 0;
pub const I915_GEM_CREATE_EXT_PROTECTED_CONTENT: u32 = 1;
pub const I915_GEM_CREATE_EXT_SET_PAT: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_i915_gem_create_ext {
    pub size: u64,
    pub handle: u32,
    pub flags: u32,
    pub extensions: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct drm_i915_gem_create_ext_memory_regions {
    pub base: i915_user_extension,
    pub pad: u32,
    pub num_regions: u32,
    pub regions: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_i915_gem_create_ext_protected_content {
    pub base: i915_user_extension,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_i915_gem_create_ext_set_pat {
    pub base: i915_user_extension,
    pub pat_index: u32,
    pub rsvd: u32,
}
