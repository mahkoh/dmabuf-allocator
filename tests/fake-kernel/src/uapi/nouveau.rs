use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IOR;
use crate::uapi::drm::DRM_IOW;
use crate::uapi::drm::DRM_IOWR;
use core::ffi::c_char;
use core::ffi::c_int;

pub const NOUVEAU_GETPARAM_PCI_VENDOR: u64 = 3;
pub const NOUVEAU_GETPARAM_PCI_DEVICE: u64 = 4;
pub const NOUVEAU_GETPARAM_BUS_TYPE: u64 = 5;
pub const NOUVEAU_GETPARAM_FB_SIZE: u64 = 8;
pub const NOUVEAU_GETPARAM_AGP_SIZE: u64 = 9;
pub const NOUVEAU_GETPARAM_CHIPSET_ID: u64 = 11;
pub const NOUVEAU_GETPARAM_VM_VRAM_BASE: u64 = 12;
pub const NOUVEAU_GETPARAM_GRAPH_UNITS: u64 = 13;
pub const NOUVEAU_GETPARAM_PTIMER_TIME: u64 = 14;
pub const NOUVEAU_GETPARAM_HAS_BO_USAGE: u64 = 15;
pub const NOUVEAU_GETPARAM_HAS_PAGEFLIP: u64 = 16;
pub const NOUVEAU_GETPARAM_EXEC_PUSH_MAX: u64 = 17;
pub const NOUVEAU_GETPARAM_VRAM_BAR_SIZE: u64 = 18;
pub const NOUVEAU_GETPARAM_VRAM_USED: u64 = 19;
pub const NOUVEAU_GETPARAM_HAS_VMA_TILEMODE: u64 = 20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_getparam {
    pub param: u64,
    pub value: u64,
}

pub const NOUVEAU_FIFO_ENGINE_GR: u32 = 0x01;
pub const NOUVEAU_FIFO_ENGINE_VP: u32 = 0x02;
pub const NOUVEAU_FIFO_ENGINE_PPP: u32 = 0x04;
pub const NOUVEAU_FIFO_ENGINE_BSP: u32 = 0x08;
pub const NOUVEAU_FIFO_ENGINE_CE: u32 = 0x30;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_channel_alloc__subchan {
    pub handle: u32,
    pub grclass: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_channel_alloc {
    pub fb_ctxdma_handle: u32,
    pub tt_ctxdma_handle: u32,
    pub channel: i32,
    pub pushbuf_domains: u32,
    pub notifier_handle: u32,
    pub subchan: [drm_nouveau_channel_alloc__subchan; 8],
    pub nr_subchan: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_channel_free {
    pub channel: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_notifierobj_alloc {
    pub channel: u32,
    pub handle: u32,
    pub size: u32,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gpuobj_free {
    pub channel: i32,
    pub handle: u32,
}

pub const NOUVEAU_GEM_DOMAIN_CPU: u32 = 1 << 0;
pub const NOUVEAU_GEM_DOMAIN_VRAM: u32 = 1 << 1;
pub const NOUVEAU_GEM_DOMAIN_GART: u32 = 1 << 2;
pub const NOUVEAU_GEM_DOMAIN_MAPPABLE: u32 = 1 << 3;
pub const NOUVEAU_GEM_DOMAIN_COHERENT: u32 = 1 << 4;
pub const NOUVEAU_GEM_DOMAIN_NO_SHARE: u32 = 1 << 5;

pub const NOUVEAU_GEM_TILE_COMP: u32 = 0x00030000;
pub const NOUVEAU_GEM_TILE_LAYOUT_MASK: u32 = 0x0000ff00;
pub const NOUVEAU_GEM_TILE_16BPP: u32 = 0x00000001;
pub const NOUVEAU_GEM_TILE_32BPP: u32 = 0x00000002;
pub const NOUVEAU_GEM_TILE_ZETA: u32 = 0x00000004;
pub const NOUVEAU_GEM_TILE_NONCONTIG: u32 = 0x00000008;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_nouveau_gem_info {
    pub handle: u32,
    pub domain: u32,
    pub size: u64,
    pub offset: u64,
    pub map_handle: u64,
    pub tile_mode: u32,
    pub tile_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_nouveau_gem_new {
    pub info: drm_nouveau_gem_info,
    pub channel_hint: u32,
    pub align: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gem_pushbuf {
    pub channel: u32,
    pub nr_buffers: u32,
    pub buffers: u64,
    pub nr_relocs: u32,
    pub nr_push: u32,
    pub relocs: u64,
    pub push: u64,
    pub suffix0: u32,
    pub suffix1: u32,
    pub vram_available: u64,
    pub gart_available: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gem_cpu_prep {
    pub handle: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gem_cpu_fini {
    pub handle: u32,
}

pub const DRM_NOUVEAU_SYNC_SYNCOBJ: u32 = 0x0;
pub const DRM_NOUVEAU_SYNC_TIMELINE_SYNCOBJ: u32 = 0x1;
pub const DRM_NOUVEAU_SYNC_TYPE_MASK: u32 = 0xf;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_nouveau_sync {
    pub flags: u32,
    pub handle: u32,
    pub timeline_value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_vm_init {
    pub kernel_managed_addr: u64,
    pub kernel_managed_size: u64,
}

pub const DRM_NOUVEAU_VM_BIND_OP_MAP: u32 = 0x0;
pub const DRM_NOUVEAU_VM_BIND_OP_UNMAP: u32 = 0x1;
pub const DRM_NOUVEAU_VM_BIND_SPARSE: u32 = 1 << 8;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_nouveau_vm_bind_op {
    pub op: u32,
    pub flags: u32,
    pub handle: u32,
    pub pad: u32,
    pub addr: u64,
    pub bo_offset: u64,
    pub range: u64,
}

pub const DRM_NOUVEAU_VM_BIND_RUN_ASYNC: u32 = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_vm_bind {
    pub op_count: u32,
    pub flags: u32,
    pub wait_count: u32,
    pub sig_count: u32,
    pub wait_ptr: u64,
    pub sig_ptr: u64,
    pub op_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_nouveau_exec_push {
    pub va: u64,
    pub va_len: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_exec {
    pub channel: u32,
    pub push_count: u32,
    pub wait_count: u32,
    pub sig_count: u32,
    pub wait_ptr: u64,
    pub sig_ptr: u64,
    pub push_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_get_zcull_info {
    pub width_align_pixels: u32,
    pub height_align_pixels: u32,
    pub pixel_squares_by_aliquots: u32,
    pub aliquot_total: u32,
    pub zcull_region_byte_multiplier: u32,
    pub zcull_region_header_size: u32,
    pub zcull_subregion_header_size: u32,
    pub subregion_count: u32,
    pub subregion_width_align_pixels: u32,
    pub subregion_height_align_pixels: u32,
    pub ctxsw_size: u32,
    pub ctxsw_align: u32,
}

pub const DRM_NOUVEAU_GETPARAM: u32 = 0x00;
pub const DRM_NOUVEAU_SETPARAM: u32 = 0x01;
pub const DRM_NOUVEAU_CHANNEL_ALLOC: u32 = 0x02;
pub const DRM_NOUVEAU_CHANNEL_FREE: u32 = 0x03;
pub const DRM_NOUVEAU_GROBJ_ALLOC: u32 = 0x04;
pub const DRM_NOUVEAU_NOTIFIEROBJ_ALLOC: u32 = 0x05;
pub const DRM_NOUVEAU_GPUOBJ_FREE: u32 = 0x06;
pub const DRM_NOUVEAU_NVIF: u32 = 0x07;
pub const DRM_NOUVEAU_SVM_INIT: u32 = 0x08;
pub const DRM_NOUVEAU_SVM_BIND: u32 = 0x09;
pub const DRM_NOUVEAU_VM_INIT: u32 = 0x10;
pub const DRM_NOUVEAU_VM_BIND: u32 = 0x11;
pub const DRM_NOUVEAU_EXEC: u32 = 0x12;
pub const DRM_NOUVEAU_GET_ZCULL_INFO: u32 = 0x13;
pub const DRM_NOUVEAU_GEM_NEW: u32 = 0x40;
pub const DRM_NOUVEAU_GEM_PUSHBUF: u32 = 0x41;
pub const DRM_NOUVEAU_GEM_CPU_PREP: u32 = 0x42;
pub const DRM_NOUVEAU_GEM_CPU_FINI: u32 = 0x43;
pub const DRM_NOUVEAU_GEM_INFO: u32 = 0x44;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_svm_init {
    pub unmanaged_addr: u64,
    pub unmanaged_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_svm_bind {
    pub header: u64,
    pub va_start: u64,
    pub va_end: u64,
    pub npages: u64,
    pub stride: u64,
    pub result: u64,
    pub reserved0: u64,
    pub reserved1: u64,
}

pub const DRM_IOCTL_NOUVEAU_GETPARAM: u32 =
    DRM_IOWR::<drm_nouveau_getparam>(DRM_COMMAND_BASE + DRM_NOUVEAU_GETPARAM);
pub const DRM_IOCTL_NOUVEAU_CHANNEL_ALLOC: u32 =
    DRM_IOWR::<drm_nouveau_channel_alloc>(DRM_COMMAND_BASE + DRM_NOUVEAU_CHANNEL_ALLOC);
pub const DRM_IOCTL_NOUVEAU_CHANNEL_FREE: u32 =
    DRM_IOW::<drm_nouveau_channel_free>(DRM_COMMAND_BASE + DRM_NOUVEAU_CHANNEL_FREE);

pub const DRM_IOCTL_NOUVEAU_SVM_INIT: u32 =
    DRM_IOWR::<drm_nouveau_svm_init>(DRM_COMMAND_BASE + DRM_NOUVEAU_SVM_INIT);
pub const DRM_IOCTL_NOUVEAU_SVM_BIND: u32 =
    DRM_IOWR::<drm_nouveau_svm_bind>(DRM_COMMAND_BASE + DRM_NOUVEAU_SVM_BIND);
pub const DRM_IOCTL_NOUVEAU_GEM_NEW: u32 =
    DRM_IOWR::<drm_nouveau_gem_new>(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_NEW);
pub const DRM_IOCTL_NOUVEAU_GEM_PUSHBUF: u32 =
    DRM_IOWR::<drm_nouveau_gem_pushbuf>(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_PUSHBUF);
pub const DRM_IOCTL_NOUVEAU_GEM_CPU_PREP: u32 =
    DRM_IOW::<drm_nouveau_gem_cpu_prep>(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_CPU_PREP);
pub const DRM_IOCTL_NOUVEAU_GEM_CPU_FINI: u32 =
    DRM_IOW::<drm_nouveau_gem_cpu_fini>(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_CPU_FINI);
pub const DRM_IOCTL_NOUVEAU_GEM_INFO: u32 =
    DRM_IOWR::<drm_nouveau_gem_info>(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_INFO);
pub const DRM_IOCTL_NOUVEAU_VM_INIT: u32 =
    DRM_IOWR::<drm_nouveau_vm_init>(DRM_COMMAND_BASE + DRM_NOUVEAU_VM_INIT);
pub const DRM_IOCTL_NOUVEAU_VM_BIND: u32 =
    DRM_IOWR::<drm_nouveau_vm_bind>(DRM_COMMAND_BASE + DRM_NOUVEAU_VM_BIND);
pub const DRM_IOCTL_NOUVEAU_EXEC: u32 =
    DRM_IOWR::<drm_nouveau_exec>(DRM_COMMAND_BASE + DRM_NOUVEAU_EXEC);
pub const DRM_IOCTL_NOUVEAU_GET_ZCULL_INFO: u32 =
    DRM_IOR::<drm_nouveau_get_zcull_info>(DRM_COMMAND_BASE + DRM_NOUVEAU_GET_ZCULL_INFO);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_grobj_alloc {
    pub channel: c_int,
    pub handle: u32,
    pub class: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_setparam {
    pub param: u64,
    pub value: u64,
}

pub const DRM_IOCTL_NOUVEAU_SETPARAM: u32 =
    DRM_IOWR::<drm_nouveau_setparam>(DRM_COMMAND_BASE + DRM_NOUVEAU_SETPARAM);
pub const DRM_IOCTL_NOUVEAU_GROBJ_ALLOC: u32 =
    DRM_IOW::<drm_nouveau_grobj_alloc>(DRM_COMMAND_BASE + DRM_NOUVEAU_GROBJ_ALLOC);
pub const DRM_IOCTL_NOUVEAU_NOTIFIEROBJ_ALLOC: u32 =
    DRM_IOWR::<drm_nouveau_notifierobj_alloc>(DRM_COMMAND_BASE + DRM_NOUVEAU_NOTIFIEROBJ_ALLOC);
pub const DRM_IOCTL_NOUVEAU_GPUOBJ_FREE: u32 =
    DRM_IOW::<drm_nouveau_gpuobj_free>(DRM_COMMAND_BASE + DRM_NOUVEAU_GPUOBJ_FREE);

pub const NVIF_IOCTL_V0_SCLASS: u8 = 0x01;
pub const NVIF_IOCTL_V0_NEW: u8 = 0x02;
pub const NVIF_IOCTL_V0_DEL: u8 = 0x03;
pub const NVIF_IOCTL_V0_MTHD: u8 = 0x04;
pub const NVIF_IOCTL_V0_MAP: u8 = 0x07;
pub const NVIF_IOCTL_V0_UNMAP: u8 = 0x08;
pub const NVIF_IOCTL_V0_OWNER_NVIF: u8 = 0x00;
pub const NVIF_IOCTL_V0_OWNER_ANY: u8 = 0xff;
pub const NVIF_IOCTL_V0_ROUTE_NVIF: u8 = 0x00;
pub const NVIF_IOCTL_V0_ROUTE_HIDDEN: u8 = 0xff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_ioctl_v0 {
    pub version: u8,
    pub type_: u8,
    pub pad02: [u8; 4],
    pub owner: u8,
    pub route: u8,
    pub token: u64,
    pub object: u64,
    pub data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_ioctl_sclass_oclass_v0 {
    pub oclass: i32,
    pub minver: i16,
    pub maxver: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_ioctl_sclass_v0 {
    pub version: u8,
    pub count: u8,
    pub pad02: [u8; 6],
    pub oclass: [nvif_ioctl_sclass_oclass_v0; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_ioctl_new_v0 {
    pub version: u8,
    pub pad01: [u8; 6],
    pub route: u8,
    pub token: u64,
    pub object: u64,
    pub handle: u32,
    pub oclass: i32,
    pub data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_ioctl_mthd_v0 {
    pub version: u8,
    pub method: u8,
    pub pad02: [u8; 6],
    pub data: [u8; 0],
}

pub const NV_DEVICE: i32 = 0x00000080;

pub const NV_DEVICE_V0_INFO: u8 = 0x00;

pub const NV_DEVICE_INFO_V0_IGP: u8 = 0x00;
pub const NV_DEVICE_INFO_V0_PCI: u8 = 0x01;
pub const NV_DEVICE_INFO_V0_AGP: u8 = 0x02;
pub const NV_DEVICE_INFO_V0_PCIE: u8 = 0x03;
pub const NV_DEVICE_INFO_V0_SOC: u8 = 0x04;
pub const NV_DEVICE_INFO_V0_TESLA: u8 = 0x06;
pub const NV_DEVICE_INFO_V0_FERMI: u8 = 0x07;
pub const NV_DEVICE_INFO_V0_KEPLER: u8 = 0x08;
pub const NV_DEVICE_INFO_V0_MAXWELL: u8 = 0x09;
pub const NV_DEVICE_INFO_V0_PASCAL: u8 = 0x0a;
pub const NV_DEVICE_INFO_V0_VOLTA: u8 = 0x0b;
pub const NV_DEVICE_INFO_V0_TURING: u8 = 0x0c;
pub const NV_DEVICE_INFO_V0_AMPERE: u8 = 0x0d;
pub const NV_DEVICE_INFO_V0_ADA: u8 = 0x0e;
pub const NV_DEVICE_INFO_V0_HOPPER: u8 = 0x0f;
pub const NV_DEVICE_INFO_V0_BLACKWELL: u8 = 0x10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_device_info_v0 {
    pub version: u8,
    pub platform: u8,
    pub chipset: u16,
    pub revision: u8,
    pub family: u8,
    pub pad06: [u8; 2],
    pub ram_size: u64,
    pub ram_user: u64,
    pub chip: [c_char; 16],
    pub name: [c_char; 64],
}
