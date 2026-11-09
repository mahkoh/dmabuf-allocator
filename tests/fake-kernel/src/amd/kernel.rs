use super::devices::DEVICES;
use super::devices::Device as DeviceValues;
use super::devices::GfxLevel;
use crate::Buffers;
use crate::DriverFeature;
use crate::DriverVersion;
use crate::Drm;
use crate::Export;
use crate::Fence;
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
use crate::id_fields_with_optional;
use crate::ioctls;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::uapi::amdgpu::AMDGPU_CHUNK_ID_BO_HANDLES;
use crate::uapi::amdgpu::AMDGPU_CHUNK_ID_FENCE;
use crate::uapi::amdgpu::AMDGPU_CHUNK_ID_IB;
use crate::uapi::amdgpu::AMDGPU_CHUNK_ID_SYNCOBJ_OUT;
use crate::uapi::amdgpu::AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_SIGNAL;
use crate::uapi::amdgpu::AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_WAIT;
use crate::uapi::amdgpu::AMDGPU_CTX_OP_ALLOC_CTX;
use crate::uapi::amdgpu::AMDGPU_CTX_OP_FREE_CTX;
use crate::uapi::amdgpu::AMDGPU_CTX_PRIORITY_HIGH;
use crate::uapi::amdgpu::AMDGPU_CTX_PRIORITY_LOW;
use crate::uapi::amdgpu::AMDGPU_CTX_PRIORITY_NORMAL;
use crate::uapi::amdgpu::AMDGPU_CTX_PRIORITY_VERY_HIGH;
use crate::uapi::amdgpu::AMDGPU_CTX_PRIORITY_VERY_LOW;
use crate::uapi::amdgpu::AMDGPU_FAMILY_AI;
use crate::uapi::amdgpu::AMDGPU_FAMILY_CZ;
use crate::uapi::amdgpu::AMDGPU_FAMILY_SI;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_COHERENT;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_CPU_GTT_USWC;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_DISCARDABLE;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_ENCRYPTED;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_EXPLICIT_SYNC;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_EXT_COHERENT;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_GFX12_DCC;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_NO_CPU_ACCESS;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_UNCACHED;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_VM_ALWAYS_VALID;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_VRAM_CLEARED;
use crate::uapi::amdgpu::AMDGPU_GEM_CREATE_VRAM_WIPE_ON_RELEASE;
use crate::uapi::amdgpu::AMDGPU_GEM_DOMAIN_CPU;
use crate::uapi::amdgpu::AMDGPU_GEM_DOMAIN_DOORBELL;
use crate::uapi::amdgpu::AMDGPU_GEM_DOMAIN_GTT;
use crate::uapi::amdgpu::AMDGPU_GEM_DOMAIN_MASK;
use crate::uapi::amdgpu::AMDGPU_GEM_DOMAIN_VRAM;
use crate::uapi::amdgpu::AMDGPU_GEM_METADATA_OP_GET_METADATA;
use crate::uapi::amdgpu::AMDGPU_GEM_METADATA_OP_SET_METADATA;
use crate::uapi::amdgpu::AMDGPU_GEM_OP_GET_GEM_CREATE_INFO;
use crate::uapi::amdgpu::AMDGPU_HW_IP_COMPUTE;
use crate::uapi::amdgpu::AMDGPU_HW_IP_GFX;
use crate::uapi::amdgpu::AMDGPU_HW_IP_INSTANCE_MAX_COUNT;
use crate::uapi::amdgpu::AMDGPU_HW_IP_NUM;
use crate::uapi::amdgpu::AMDGPU_IB_FLAG_CE;
use crate::uapi::amdgpu::AMDGPU_IB_FLAG_EMIT_MEM_SYNC;
use crate::uapi::amdgpu::AMDGPU_IB_FLAG_PREAMBLE;
use crate::uapi::amdgpu::AMDGPU_IB_FLAG_PREEMPT;
use crate::uapi::amdgpu::AMDGPU_IB_FLAG_TC_WB_NOT_INVALIDATE;
use crate::uapi::amdgpu::AMDGPU_INFO_ACCEL_WORKING;
use crate::uapi::amdgpu::AMDGPU_INFO_DEV_INFO;
use crate::uapi::amdgpu::AMDGPU_INFO_FW_GFX_ME;
use crate::uapi::amdgpu::AMDGPU_INFO_FW_GFX_MEC;
use crate::uapi::amdgpu::AMDGPU_INFO_FW_GFX_PFP;
use crate::uapi::amdgpu::AMDGPU_INFO_FW_VERSION;
use crate::uapi::amdgpu::AMDGPU_INFO_GTT_USAGE;
use crate::uapi::amdgpu::AMDGPU_INFO_HW_IP_COUNT;
use crate::uapi::amdgpu::AMDGPU_INFO_HW_IP_INFO;
use crate::uapi::amdgpu::AMDGPU_INFO_MAX_IBS;
use crate::uapi::amdgpu::AMDGPU_INFO_MEMORY;
use crate::uapi::amdgpu::AMDGPU_INFO_MMR_SE_INDEX_MASK;
use crate::uapi::amdgpu::AMDGPU_INFO_MMR_SE_INDEX_SHIFT;
use crate::uapi::amdgpu::AMDGPU_INFO_MMR_SH_INDEX_MASK;
use crate::uapi::amdgpu::AMDGPU_INFO_MMR_SH_INDEX_SHIFT;
use crate::uapi::amdgpu::AMDGPU_INFO_READ_MMR_REG;
use crate::uapi::amdgpu::AMDGPU_INFO_VIDEO_CAPS;
use crate::uapi::amdgpu::AMDGPU_INFO_VIDEO_CAPS_DECODE;
use crate::uapi::amdgpu::AMDGPU_INFO_VIDEO_CAPS_ENCODE;
use crate::uapi::amdgpu::AMDGPU_INFO_VIS_VRAM_USAGE;
use crate::uapi::amdgpu::AMDGPU_INFO_VRAM_GTT;
use crate::uapi::amdgpu::AMDGPU_INFO_VRAM_USAGE;
use crate::uapi::amdgpu::AMDGPU_TILING_TILE_SPLIT_MASK;
use crate::uapi::amdgpu::AMDGPU_TILING_TILE_SPLIT_SHIFT;
use crate::uapi::amdgpu::AMDGPU_VA_OP_MAP;
use crate::uapi::amdgpu::AMDGPU_VA_OP_UNMAP;
use crate::uapi::amdgpu::AMDGPU_VM_DELAY_UPDATE;
use crate::uapi::amdgpu::AMDGPU_VM_MTYPE_DEFAULT;
use crate::uapi::amdgpu::AMDGPU_VM_MTYPE_MASK;
use crate::uapi::amdgpu::AMDGPU_VM_MTYPE_UC;
use crate::uapi::amdgpu::AMDGPU_VM_PAGE_EXECUTABLE;
use crate::uapi::amdgpu::AMDGPU_VM_PAGE_NOALLOC;
use crate::uapi::amdgpu::AMDGPU_VM_PAGE_PRT;
use crate::uapi::amdgpu::AMDGPU_VM_PAGE_READABLE;
use crate::uapi::amdgpu::AMDGPU_VM_PAGE_WRITEABLE;
use crate::uapi::amdgpu::drm_amdgpu_bo_list_entry;
use crate::uapi::amdgpu::drm_amdgpu_bo_list_in;
use crate::uapi::amdgpu::drm_amdgpu_cs;
use crate::uapi::amdgpu::drm_amdgpu_cs_chunk;
use crate::uapi::amdgpu::drm_amdgpu_cs_chunk_fence;
use crate::uapi::amdgpu::drm_amdgpu_cs_chunk_ib;
use crate::uapi::amdgpu::drm_amdgpu_cs_chunk_sem;
use crate::uapi::amdgpu::drm_amdgpu_cs_chunk_syncobj;
use crate::uapi::amdgpu::drm_amdgpu_cs_out;
use crate::uapi::amdgpu::drm_amdgpu_ctx;
use crate::uapi::amdgpu::drm_amdgpu_gem_create;
use crate::uapi::amdgpu::drm_amdgpu_gem_create_in;
use crate::uapi::amdgpu::drm_amdgpu_gem_metadata;
use crate::uapi::amdgpu::drm_amdgpu_gem_mmap;
use crate::uapi::amdgpu::drm_amdgpu_gem_op;
use crate::uapi::amdgpu::drm_amdgpu_gem_va;
use crate::uapi::amdgpu::drm_amdgpu_heap_info;
use crate::uapi::amdgpu::drm_amdgpu_info;
use crate::uapi::amdgpu::drm_amdgpu_info__anon__read_mmr_reg;
use crate::uapi::amdgpu::drm_amdgpu_info_device;
use crate::uapi::amdgpu::drm_amdgpu_info_firmware;
use crate::uapi::amdgpu::drm_amdgpu_info_hw_ip;
use crate::uapi::amdgpu::drm_amdgpu_info_video_caps;
use crate::uapi::amdgpu::drm_amdgpu_info_vram_gtt;
use crate::uapi::amdgpu::drm_amdgpu_memory_info;
use crate::uapi::amdgpu::drm_amdgpu_query_fw;
use bytemuck::bytes_of;
use libc::EACCES;
use libc::EINVAL;
use libc::ENOENT;
use libc::ENOMEM;
use libc::EPERM;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::mem::MaybeUninit;
use std::mem::offset_of;
use std::sync::Arc;

const AMDGPU_GPU_PAGE_SIZE: u64 = 4096;
const AMDGPU_VA_RESERVED_BOTTOM: u64 = 1 << 16;
const AMDGPU_VA_RESERVED_SEQ64_SIZE: u64 = 2 << 20;
const AMDGPU_VA_RESERVED_CSA_SIZE: u64 = 2 << 20;
const AMDGPU_GMC_HOLE_START: u64 = 0x0000800000000000;
const AMDGPU_GMC_HOLE_END: u64 = 0xffff800000000000;
const AMDGPU_GMC_HOLE_MASK: u64 = 0x0000ffffffffffff;
const AMDGPU_VM_RESERVED_VRAM: u64 = 8 << 20;
const AMDGPU_GFX_MAX_SE: u32 = 4;
const AMDGPU_GFX_MAX_SH_PER_SE: u32 = 2;
const AMDGPU_CS_GANG_SIZE: usize = 4;
const AMDGPU_GFX_SDMA_IB_PACKET_SIZE_MAX_DW: u32 = 0xFFFFF;
const AMDGPU_BO_LIST_MAX_ENTRIES: u32 = 128 * 1024;
const AMDGPU_CTX_NUM_ENTITIES: [u32; AMDGPU_HW_IP_NUM as usize] = [1, 4, 2, 1, 1, 1, 1, 1, 1, 1];
const AMDGPU_RING_MAX_IBS: [u32; AMDGPU_HW_IP_NUM as usize] =
    [192, 125, 49, 49, 49, 49, 49, 49, 16, 49];
const HOST_RAM_GB: u64 = 32;
const MM_CC_RB_BACKEND_DISABLE: u32 = 0x263d;
const MM_GB_ADDR_CONFIG: u32 = 0x263e;
const MM_GB_TILE_MODE0: u32 = 0x2644;
const MM_GB_MACROTILE_MODE0: u32 = 0x2664;
const MM_MC_ARB_RAMCFG: u32 = 0x9d8;
const MM_PA_SC_RASTER_CONFIG: u32 = 0xa0d4;
const MM_PA_SC_RASTER_CONFIG_1: u32 = 0xa0d5;

#[derive(Clone)]
struct Buffer {
    info: drm_amdgpu_gem_create_in,
    tiling: u64,
    metadata: Vec<u8>,
    metadata_flags: u64,
}

#[derive(Copy, Clone)]
struct Mapping {
    start: u64,
    last: u64,
    object: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct EntityKey {
    hw_ip: u32,
    ring: u32,
}

#[derive(Default)]
struct Context {
    sequences: BTreeMap<EntityKey, u64>,
}

#[derive(Default)]
struct File {
    contexts: BTreeMap<u32, Context>,
    mappings: Vec<Mapping>,
}

pub struct Device {
    values: Box<DeviceValues>,
    kernel: KernelVersion,
    pte_fragment_size: u32,
    page_size: u64,
    buffers: Buffers<Buffer>,
    syncobjs: Syncobjs,
    master: Master,
    files: Mutex<BTreeMap<u64, File>>,
}

impl Device {
    pub fn new(name: &str, kernel: KernelVersion) -> Self {
        let values = DEVICES
            .iter()
            .find(|device| device.name == name)
            .unwrap_or_else(|| panic!("unknown amdgpu device {name}"));
        if let Some(first) = values.first_kernel {
            assert!(kernel >= first, "amdgpu device {name} needs kernel {first}");
        }
        Self {
            values: Box::new(values.clone()),
            kernel,
            pte_fragment_size: 2 << 20,
            page_size: PAGE_SIZE,
            buffers: Buffers::default(),
            syncobjs: Syncobjs::default(),
            master: Master::default(),
            files: Mutex::default(),
        }
    }

    pub fn from_id(id: &str) -> Self {
        let (name, fields) = id
            .split_once(':')
            .unwrap_or_else(|| panic!("invalid amdgpu id {id:?}"));
        let values =
            id_fields_with_optional(fields, &["kernel", "pte_fragment_size"], &["page_size"]);
        let pte_fragment_size = values.get("pte_fragment_size");
        let pte_fragment_size = pte_fragment_size
            .parse()
            .unwrap_or_else(|_| panic!("invalid pte_fragment_size {pte_fragment_size:?}"));
        let device = Self::new(name, KernelVersion::parse(values.get("kernel")))
            .with_pte_fragment_size(pte_fragment_size);
        match values.find("page_size") {
            Some(text) => device.with_page_size(
                text.parse()
                    .unwrap_or_else(|_| panic!("invalid page_size {text:?}")),
            ),
            None => device,
        }
    }

    pub fn with_pte_fragment_size(mut self, pte_fragment_size: u32) -> Self {
        assert!(
            pte_fragment_size as u64 >= AMDGPU_GPU_PAGE_SIZE && pte_fragment_size.is_power_of_two(),
            "invalid pte_fragment_size {pte_fragment_size}"
        );
        self.pte_fragment_size = pte_fragment_size;
        self
    }

    pub fn with_page_size(mut self, page_size: u64) -> Self {
        assert!(
            page_size >= PAGE_SIZE && page_size.is_power_of_two(),
            "invalid page_size {page_size}"
        );
        self.page_size = page_size;
        self
    }

    pub fn with_values(mut self, values: DeviceValues) -> Self {
        self.values = Box::new(values);
        self
    }

    fn rings(&self, hw_ip: u32) -> u32 {
        let rings = match hw_ip {
            AMDGPU_HW_IP_GFX => self.values.gfx_rings,
            AMDGPU_HW_IP_COMPUTE => self.values.compute_rings,
            _ => return 0,
        };
        rings.min(AMDGPU_CTX_NUM_ENTITIES[hw_ip as usize])
    }

    fn max_pfn(&self) -> u64 {
        let gb = match self.values.gfx_level {
            GfxLevel::Gfx6 | GfxLevel::Gfx7 | GfxLevel::Gfx8 => (HOST_RAM_GB * 3)
                .clamp(64, 1 << (40 - 30))
                .next_power_of_two(),
            _ => 1 << (48 - 30),
        };
        gb << 18
    }

    fn vm_size(&self) -> u64 {
        self.max_pfn() * AMDGPU_GPU_PAGE_SIZE - self.va_reserved_top()
    }

    fn va_reserved_top(&self) -> u64 {
        let trap = match self.kernel >= KernelVersion::V7_0 {
            true => 1 << 16,
            false => 2 << 12,
        };
        trap + AMDGPU_VA_RESERVED_SEQ64_SIZE + AMDGPU_VA_RESERVED_CSA_SIZE
    }
}

struct Chunk {
    chunk_id: u32,
    data: Vec<u8>,
}

impl Chunk {
    unsafe fn read<T: Copy>(&self) -> T {
        let mut value = MaybeUninit::<T>::zeroed();
        let len = self.data.len().min(size_of::<T>());
        unsafe {
            value
                .as_mut_ptr()
                .cast::<u8>()
                .copy_from_nonoverlapping(self.data.as_ptr(), len);
            value.assume_init()
        }
    }

    unsafe fn array<T: Copy>(&self) -> Vec<T> {
        (0..self.data.len() / size_of::<T>())
            .map(|i| unsafe {
                self.data
                    .as_ptr()
                    .add(i * size_of::<T>())
                    .cast::<T>()
                    .read_unaligned()
            })
            .collect()
    }
}

fn read_mmr_reg(values: &DeviceValues, register: u32) -> u32 {
    let pre_soc15 = values.family < AMDGPU_FAMILY_AI;
    let si = values.family == AMDGPU_FAMILY_SI;
    let tile_mode = register
        .checked_sub(MM_GB_TILE_MODE0)
        .and_then(|index| values.gb_tile_mode.get(index as usize));
    let macrotile_mode = register
        .checked_sub(MM_GB_MACROTILE_MODE0)
        .and_then(|index| values.gb_macrotile_mode.get(index as usize));
    match register {
        MM_GB_ADDR_CONFIG => values.gb_addr_config,
        MM_MC_ARB_RAMCFG if pre_soc15 => values.mc_arb_ramcfg,
        MM_CC_RB_BACKEND_DISABLE if pre_soc15 => values.cc_rb_backend_disable,
        MM_PA_SC_RASTER_CONFIG if pre_soc15 => values.pa_sc_raster_config,
        MM_PA_SC_RASTER_CONFIG_1 if pre_soc15 && !si => values.pa_sc_raster_config_1,
        _ => match (tile_mode, macrotile_mode) {
            (Some(&mode), _) if pre_soc15 => mode,
            (_, Some(&mode)) if pre_soc15 && !si => mode,
            _ => unimplemented!("reading register {register:#x}"),
        },
    }
}

fn hw_ip(values: &DeviceValues, rings: u32) -> drm_amdgpu_info_hw_ip {
    let mut ip: drm_amdgpu_info_hw_ip = unsafe { std::mem::zeroed() };
    ip.hw_ip_version_major = values.gfx_ip_version.major;
    ip.hw_ip_version_minor = values.gfx_ip_version.minor;
    ip.ib_start_alignment = 32;
    ip.ib_size_alignment = 32;
    ip.available_rings = (1 << rings) - 1;
    ip.ip_discovery_version = values.ip_discovery_version;
    ip
}

fn firmware() -> drm_amdgpu_info_firmware {
    drm_amdgpu_info_firmware {
        ver: 0xffff,
        feature: 0xffff,
    }
}

fn dev_info(device: &Device) -> drm_amdgpu_info_device {
    let values = &device.values;
    let mut info: drm_amdgpu_info_device = unsafe { std::mem::zeroed() };
    info.device_id = values.device_id as u32;
    info.chip_rev = values.chip_rev;
    info.external_rev = values.external_rev;
    info.pci_rev = values.pci_rev as u32;
    info.family = values.family;
    info.num_shader_engines = values.num_shader_engines;
    info.num_shader_arrays_per_engine = values.num_shader_arrays_per_engine;
    info.gpu_counter_freq = values.gpu_counter_freq;
    info.max_engine_clock = values.max_engine_clock;
    info.max_memory_clock = values.max_memory_clock;
    info.min_engine_clock = values.min_engine_clock;
    info.min_memory_clock = values.min_memory_clock;
    info.enabled_rb_pipes_mask = values.enabled_rb_pipes_mask;
    info.num_rb_pipes = values.num_rb_pipes;
    info.num_hw_gfx_contexts = values.num_hw_gfx_contexts;
    info.ids_flags = values.ids_flags;
    let vm_size = device.vm_size();
    info.virtual_address_offset = AMDGPU_VA_RESERVED_BOTTOM;
    info.virtual_address_max = vm_size.min(AMDGPU_GMC_HOLE_START);
    if vm_size > AMDGPU_GMC_HOLE_START {
        info.high_va_offset = AMDGPU_GMC_HOLE_END;
        info.high_va_max = AMDGPU_GMC_HOLE_END | vm_size;
    }
    let page_size = device.page_size.max(AMDGPU_GPU_PAGE_SIZE) as u32;
    info.virtual_address_alignment = page_size;
    info.pte_fragment_size = device.pte_fragment_size;
    info.gart_page_size = page_size;
    info.cu_active_number = values
        .cu_bitmap
        .iter()
        .flatten()
        .map(|cus| cus.count_ones())
        .sum();
    info.cu_ao_mask = values.cu_ao_mask;
    info.ce_ram_size = values.ce_ram_size;
    info.cu_ao_bitmap = values.cu_ao_bitmap;
    info.cu_bitmap = values.cu_bitmap;
    info.vram_type = values.vram_type;
    info.vram_bit_width = values.vram_bit_width;
    info.vce_harvest_config = values.vce_harvest_config;
    info.gc_double_offchip_lds_buf = values.gc_double_offchip_lds_buf;
    info.wave_front_size = values.wave_front_size;
    info.num_shader_visible_vgprs = values.num_shader_visible_vgprs;
    info.num_cu_per_sh = values.num_cu_per_sh;
    info.num_tcc_blocks = values.num_tcc_blocks;
    info.gs_vgt_table_depth = values.gs_vgt_table_depth;
    info.gs_prim_buffer_depth = values.gs_prim_buffer_depth;
    info.max_gs_waves_per_vgt = values.max_gs_waves_per_vgt;
    info.pa_sc_tile_steering_override = values.pa_sc_tile_steering_override;
    info.pcie_gen = values.pcie_gen;
    info.pcie_num_lanes = values.pcie_num_lanes;
    info.tcp_cache_size = values.tcp_cache_size;
    info.num_sqc_per_wgp = values.num_sqc_per_wgp;
    info.sqc_data_cache_size = values.sqc_data_cache_size;
    info.sqc_inst_cache_size = values.sqc_inst_cache_size;
    info.gl1c_cache_size = values.gl1c_cache_size;
    info.gl2c_cache_size = values.gl2c_cache_size;
    info.mall_size = values.mall_size;
    info
}

const VRAM_SIZE: u64 = 8 << 30;
const GTT_SIZE: u64 = 16 << 30;

fn memory() -> drm_amdgpu_memory_info {
    let heap = |total: u64, usable: u64| drm_amdgpu_heap_info {
        total_heap_size: total,
        usable_heap_size: usable,
        heap_usage: 0,
        max_allocation: usable * 3 / 4,
    };
    let vram = VRAM_SIZE - AMDGPU_VM_RESERVED_VRAM;
    drm_amdgpu_memory_info {
        vram: heap(VRAM_SIZE, vram),
        cpu_accessible_vram: heap(VRAM_SIZE, VRAM_SIZE.min(vram)),
        gtt: heap(GTT_SIZE, GTT_SIZE),
    }
}

fn reply_size<T>(kernel: KernelVersion, added: Option<KernelVersion>, end: usize) -> usize {
    match added.is_some_and(|added| kernel < added) {
        true => end.next_multiple_of(align_of::<T>()),
        false => size_of::<T>(),
    }
}

fn info_firmware(query: &drm_amdgpu_query_fw) -> Result<drm_amdgpu_info_firmware, i32> {
    if query.ip_instance != 0 {
        return Err(EINVAL);
    }
    match query.fw_type {
        AMDGPU_INFO_FW_GFX_ME | AMDGPU_INFO_FW_GFX_PFP => Ok(firmware()),
        AMDGPU_INFO_FW_GFX_MEC => match query.index {
            0 => Ok(firmware()),
            index => unimplemented!("MEC firmware index {index}"),
        },
        fw_type => unimplemented!("firmware type {fw_type:#x}"),
    }
}

fn info_read_mmr_reg(
    values: &DeviceValues,
    read: &drm_amdgpu_info__anon__read_mmr_reg,
) -> Result<Vec<u8>, i32> {
    let se = (read.instance >> AMDGPU_INFO_MMR_SE_INDEX_SHIFT) & AMDGPU_INFO_MMR_SE_INDEX_MASK;
    let sh = (read.instance >> AMDGPU_INFO_MMR_SH_INDEX_SHIFT) & AMDGPU_INFO_MMR_SH_INDEX_MASK;
    if se != AMDGPU_INFO_MMR_SE_INDEX_MASK && se >= AMDGPU_GFX_MAX_SE {
        return Err(EINVAL);
    }
    if sh != AMDGPU_INFO_MMR_SH_INDEX_MASK && sh >= AMDGPU_GFX_MAX_SH_PER_SE {
        return Err(EINVAL);
    }
    if read.count > 128 {
        return Err(EINVAL);
    }
    let mut bytes = Vec::new();
    for i in 0..read.count {
        bytes.extend(read_mmr_reg(values, read.dword_offset.wrapping_add(i)).to_ne_bytes());
    }
    Ok(bytes)
}

unsafe fn info(device: &Device, arg: *mut u8, user: &dyn UserMemory) -> i64 {
    let values = &device.values;
    let request = unsafe { &*arg.cast::<drm_amdgpu_info>() };
    let args = &request.anon;
    if request.return_size == 0 || request.return_pointer == 0 {
        return errno(EINVAL);
    }
    let mut bytes = match request.query {
        AMDGPU_INFO_ACCEL_WORKING => 1u32.to_ne_bytes().to_vec(),
        AMDGPU_INFO_HW_IP_INFO => {
            let query = unsafe { args.query_hw_ip };
            if query.ip_instance >= AMDGPU_HW_IP_INSTANCE_MAX_COUNT
                || query.type_ >= AMDGPU_HW_IP_NUM
            {
                unimplemented!(
                    "HW IP info for type {} instance {}",
                    query.type_,
                    query.ip_instance
                );
            }
            let ip = match query.type_ {
                AMDGPU_HW_IP_GFX | AMDGPU_HW_IP_COMPUTE => hw_ip(values, device.rings(query.type_)),
                _ => unsafe { std::mem::zeroed() },
            };
            let size = reply_size::<drm_amdgpu_info_hw_ip>(
                device.kernel,
                Some(KernelVersion::V6_17),
                offset_of!(drm_amdgpu_info_hw_ip, userq_num_slots),
            );
            bytes_of(&ip)[..size].to_vec()
        }
        AMDGPU_INFO_HW_IP_COUNT => match unsafe { args.query_hw_ip.type_ } {
            AMDGPU_HW_IP_GFX | AMDGPU_HW_IP_COMPUTE => 1u32.to_ne_bytes().to_vec(),
            AMDGPU_HW_IP_NUM.. => {
                unimplemented!("HW IP type {}", unsafe { args.query_hw_ip.type_ });
            }
            _ => return errno(EINVAL),
        },
        AMDGPU_INFO_FW_VERSION => match info_firmware(unsafe { &args.query_fw }) {
            Ok(fw) => bytes_of(&fw).to_vec(),
            Err(e) => return errno(e),
        },
        AMDGPU_INFO_READ_MMR_REG => {
            match info_read_mmr_reg(values, unsafe { &args.read_mmr_reg }) {
                Ok(bytes) => bytes,
                Err(e) => return errno(e),
            }
        }
        AMDGPU_INFO_DEV_INFO => {
            let size = reply_size::<drm_amdgpu_info_device>(
                device.kernel,
                Some(KernelVersion::V6_16),
                offset_of!(drm_amdgpu_info_device, userq_ip_mask),
            );
            bytes_of(&dev_info(device))[..size].to_vec()
        }
        AMDGPU_INFO_MEMORY => bytes_of(&memory()).to_vec(),
        AMDGPU_INFO_VRAM_GTT => {
            let memory = memory();
            let vram_gtt = drm_amdgpu_info_vram_gtt {
                vram_size: memory.vram.usable_heap_size,
                vram_cpu_accessible_size: memory.cpu_accessible_vram.usable_heap_size,
                gtt_size: memory.gtt.usable_heap_size,
            };
            bytes_of(&vram_gtt).to_vec()
        }
        AMDGPU_INFO_VRAM_USAGE => memory().vram.heap_usage.to_ne_bytes().to_vec(),
        AMDGPU_INFO_VIS_VRAM_USAGE => memory()
            .cpu_accessible_vram
            .heap_usage
            .to_ne_bytes()
            .to_vec(),
        AMDGPU_INFO_GTT_USAGE => memory().gtt.heap_usage.to_ne_bytes().to_vec(),
        AMDGPU_INFO_VIDEO_CAPS => match unsafe { args.video_cap.type_ } {
            AMDGPU_INFO_VIDEO_CAPS_DECODE | AMDGPU_INFO_VIDEO_CAPS_ENCODE => {
                vec![0; size_of::<drm_amdgpu_info_video_caps>()]
            }
            video_cap => unimplemented!("video caps type {video_cap}"),
        },
        AMDGPU_INFO_MAX_IBS => AMDGPU_RING_MAX_IBS
            .iter()
            .flat_map(|max| max.to_ne_bytes())
            .collect(),
        query => unimplemented!("AMDGPU_INFO query {query:#x}"),
    };
    bytes.truncate(request.return_size as usize);
    match user.write(request.return_pointer, &bytes) {
        Ok(()) => 0,
        Err(e) => errno(e),
    }
}

fn bo_info(
    page_size: u64,
    size: u64,
    alignment: u64,
    domain: u64,
    flags: u64,
) -> drm_amdgpu_gem_create_in {
    let byte_align = alignment as u32 as i32;
    let mask = page_size as i32 - 1;
    let shift = page_size.trailing_zeros();
    let page_align = (byte_align.wrapping_add(mask) & !mask) as i64 as u64 >> shift;
    let page_alignment = page_align as u32;
    drm_amdgpu_gem_create_in {
        bo_size: size.wrapping_add(page_size - 1) & !(page_size - 1),
        alignment: (page_alignment << shift) as u64,
        domains: domain,
        domain_flags: (flags | AMDGPU_GEM_CREATE_VRAM_WIPE_ON_RELEASE)
            & !AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED,
    }
}

fn domains_valid(domains: u64) -> bool {
    let normal = AMDGPU_GEM_DOMAIN_CPU | AMDGPU_GEM_DOMAIN_GTT | AMDGPU_GEM_DOMAIN_VRAM;
    let special = (AMDGPU_GEM_DOMAIN_MASK & !normal) as u32;
    let domains = domains as u32;
    let normal_mask = domains & normal as u32;
    let special_mask = domains & special;
    if special_mask == 0 {
        return true;
    }
    if normal_mask != 0 {
        return false;
    }
    special_mask & (special_mask - 1) == 0
}

fn create_info(
    kernel: KernelVersion,
    page_size: u64,
    create: &drm_amdgpu_gem_create_in,
) -> Result<drm_amdgpu_gem_create_in, i32> {
    const AMDGPU_GEM_DOMAIN_MMIO_REMAP: u64 = 0x80;
    let mut flags = create.domain_flags;
    if kernel < KernelVersion::V6_16 && create.domains & AMDGPU_GEM_DOMAIN_DOORBELL != 0 {
        return Err(EINVAL);
    }
    let mut settable = AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED
        | AMDGPU_GEM_CREATE_NO_CPU_ACCESS
        | AMDGPU_GEM_CREATE_CPU_GTT_USWC
        | AMDGPU_GEM_CREATE_VRAM_CLEARED
        | AMDGPU_GEM_CREATE_VM_ALWAYS_VALID
        | AMDGPU_GEM_CREATE_EXPLICIT_SYNC
        | AMDGPU_GEM_CREATE_ENCRYPTED
        | AMDGPU_GEM_CREATE_GFX12_DCC
        | AMDGPU_GEM_CREATE_DISCARDABLE;
    if kernel >= KernelVersion::V6_18 {
        settable |= AMDGPU_GEM_CREATE_VRAM_WIPE_ON_RELEASE
            | AMDGPU_GEM_CREATE_COHERENT
            | AMDGPU_GEM_CREATE_UNCACHED
            | AMDGPU_GEM_CREATE_EXT_COHERENT;
    }
    if flags & !settable != 0 {
        return Err(EINVAL);
    }
    let mmio_remap = kernel >= KernelVersion::V6_18 && kernel < KernelVersion::V7_0;
    let mut domain_mask = AMDGPU_GEM_DOMAIN_MASK;
    if mmio_remap {
        domain_mask |= AMDGPU_GEM_DOMAIN_MMIO_REMAP;
    }
    if create.domains & !domain_mask != 0 {
        return Err(EINVAL);
    }
    if kernel >= KernelVersion::V7_2 && !domains_valid(create.domains) {
        return Err(EINVAL);
    }
    if mmio_remap && create.domains & AMDGPU_GEM_DOMAIN_MMIO_REMAP != 0 {
        return Err(EINVAL);
    }
    let unused_flags = AMDGPU_GEM_CREATE_VM_ALWAYS_VALID
        | AMDGPU_GEM_CREATE_ENCRYPTED
        | AMDGPU_GEM_CREATE_VRAM_WIPE_ON_RELEASE
        | AMDGPU_GEM_CREATE_COHERENT
        | AMDGPU_GEM_CREATE_UNCACHED
        | AMDGPU_GEM_CREATE_EXT_COHERENT;
    if flags & unused_flags != 0 {
        unimplemented!("GEM create flags {:#x}", flags & unused_flags);
    }
    let unused_domains = !(AMDGPU_GEM_DOMAIN_GTT | AMDGPU_GEM_DOMAIN_VRAM);
    if create.domains & unused_domains != 0 {
        unimplemented!("GEM create domains {:#x}", create.domains & unused_domains);
    }
    flags |= AMDGPU_GEM_CREATE_VRAM_CLEARED;
    Ok(bo_info(
        page_size,
        create.bo_size,
        create.alignment,
        create.domains,
        flags,
    ))
}

fn create_buffer(buffers: &FileBuffers<Buffer>, info: drm_amdgpu_gem_create_in) -> NewHandle {
    let size = info.bo_size;
    let buffer = Buffer {
        info,
        tiling: 0,
        metadata: Vec::new(),
        metadata_flags: 0,
    };
    buffers.create(buffer, size)
}

fn fits(size: u64, domains: u64) -> bool {
    if domains & AMDGPU_GEM_DOMAIN_GTT != 0 {
        size < GTT_SIZE
    } else if domains & AMDGPU_GEM_DOMAIN_VRAM != 0 {
        size < VRAM_SIZE
    } else {
        true
    }
}

fn place(info: &mut drm_amdgpu_gem_create_in) -> Result<(), i32> {
    if info.bo_size == 0 {
        unimplemented!("BOs whose aligned size is 0");
    }
    if fits(info.bo_size, info.domains) {
        return Ok(());
    }
    if info.domains == AMDGPU_GEM_DOMAIN_VRAM {
        info.domains |= AMDGPU_GEM_DOMAIN_GTT;
        if fits(info.bo_size, info.domains) {
            return Ok(());
        }
    }
    Err(ENOMEM)
}

fn va_error(device: &Device, va: &drm_amdgpu_gem_va) -> Option<i32> {
    let valid_flags = AMDGPU_VM_DELAY_UPDATE
        | AMDGPU_VM_PAGE_READABLE
        | AMDGPU_VM_PAGE_WRITEABLE
        | AMDGPU_VM_PAGE_EXECUTABLE
        | AMDGPU_VM_MTYPE_MASK
        | AMDGPU_VM_PAGE_NOALLOC;
    let prt_flags = AMDGPU_VM_DELAY_UPDATE | AMDGPU_VM_PAGE_PRT;
    if va.va_address < AMDGPU_VA_RESERVED_BOTTOM {
        return Some(EINVAL);
    }
    if (AMDGPU_GMC_HOLE_START..AMDGPU_GMC_HOLE_END).contains(&va.va_address) {
        return Some(EINVAL);
    }
    let address = va.va_address & AMDGPU_GMC_HOLE_MASK;
    let vm_size = device.vm_size();
    let outside = match device.kernel >= KernelVersion::V7_1 {
        true => address
            .checked_add(va.map_size)
            .is_none_or(|end| end > vm_size),
        false => address.wrapping_add(va.map_size) > vm_size,
    };
    if outside {
        return Some(EINVAL);
    }
    if va.flags & !valid_flags != 0 && va.flags & !prt_flags != 0 {
        return Some(EINVAL);
    }
    if device.kernel >= KernelVersion::V7_1
        && va.flags & AMDGPU_VM_DELAY_UPDATE != 0
        && va.vm_timeline_syncobj_out != 0
    {
        return Some(EINVAL);
    }
    None
}

fn verify_parameters(
    device: &Device,
    bo_size: u64,
    saddr: u64,
    offset: u64,
    size: u64,
) -> Result<(), i32> {
    let mask = AMDGPU_GPU_PAGE_SIZE - 1;
    if saddr & mask != 0 || offset & mask != 0 || size & mask != 0 {
        return Err(EINVAL);
    }
    if saddr.checked_add(size).is_none() || offset.checked_add(size).is_none() || size == 0 {
        return Err(EINVAL);
    }
    if offset + size > bo_size {
        return Err(EINVAL);
    }
    if (saddr + size - 1) / AMDGPU_GPU_PAGE_SIZE >= device.max_pfn() {
        return Err(EINVAL);
    }
    Ok(())
}

fn gem_va(device: &Device, file_id: u64, va: &mut drm_amdgpu_gem_va) -> i64 {
    let buffers = device.buffers.file(file_id);
    if let Some(e) = va_error(device, va) {
        return errno(e);
    }
    va.va_address &= AMDGPU_GMC_HOLE_MASK;
    let mtype = va.flags & AMDGPU_VM_MTYPE_MASK;
    let unused_flags = AMDGPU_VM_DELAY_UPDATE | AMDGPU_VM_PAGE_PRT | AMDGPU_VM_PAGE_NOALLOC;
    if va.flags & unused_flags != 0
        || mtype != AMDGPU_VM_MTYPE_DEFAULT && mtype != AMDGPU_VM_MTYPE_UC
    {
        unimplemented!("GEM VA flags {:#x}", va.flags);
    }
    if va.operation != AMDGPU_VA_OP_MAP && va.operation != AMDGPU_VA_OP_UNMAP {
        unimplemented!("GEM VA operation {}", va.operation);
    }
    let timeline = device.kernel >= KernelVersion::V6_16;
    if timeline && (va.num_syncobj_handles != 0 || va.vm_timeline_syncobj_out != 0) {
        unimplemented!("GEM VA syncobjs");
    }
    let mut files = device.files.lock();
    let Some(object) = buffers.object(va.handle) else {
        return errno(ENOENT);
    };
    let Some(bo_size) = buffers.with(va.handle, |buffer| buffer.info.bo_size) else {
        return errno(ENOENT);
    };
    let file = files.entry(file_id).or_default();
    let start = va.va_address / AMDGPU_GPU_PAGE_SIZE;
    let result = match va.operation {
        AMDGPU_VA_OP_MAP => {
            verify_parameters(device, bo_size, va.va_address, va.offset_in_bo, va.map_size)
                .and_then(|()| {
                    let last = start + (va.map_size - 1) / AMDGPU_GPU_PAGE_SIZE;
                    if file
                        .mappings
                        .iter()
                        .any(|mapping| mapping.start <= last && mapping.last >= start)
                    {
                        return Err(EINVAL);
                    }
                    file.mappings.push(Mapping {
                        start,
                        last,
                        object,
                    });
                    Ok(())
                })
        }
        _ => match file
            .mappings
            .iter()
            .position(|mapping| mapping.object == object && mapping.start == start)
        {
            Some(i) => {
                file.mappings.remove(i);
                Ok(())
            }
            None => Err(ENOENT),
        },
    };
    match result {
        Ok(()) => 0,
        Err(e) => errno(e),
    }
}

unsafe fn gem_metadata(device: &Device, buffers: &FileBuffers<Buffer>, arg: *mut u8) -> i64 {
    let metadata = unsafe { &mut *arg.cast::<drm_amdgpu_gem_metadata>() };
    match metadata.op {
        AMDGPU_GEM_METADATA_OP_GET_METADATA => {
            let data = &mut metadata.data;
            let found = buffers.with(metadata.handle, |buffer| {
                data.tiling_info = buffer.tiling;
                data.data_size_bytes = buffer.metadata.len() as u32;
                let bytes = data.data.as_mut_ptr().cast::<u8>();
                unsafe {
                    bytes.copy_from_nonoverlapping(buffer.metadata.as_ptr(), buffer.metadata.len());
                }
                data.flags = buffer.metadata_flags;
            });
            match found {
                Some(()) => 0,
                None => errno(ENOENT),
            }
        }
        AMDGPU_GEM_METADATA_OP_SET_METADATA => {
            if !buffers.contains(metadata.handle) {
                return errno(ENOENT);
            }
            let size = metadata.data.data_size_bytes as usize;
            if size > size_of_val(&metadata.data.data) {
                return errno(EINVAL);
            }
            let tiling = metadata.data.tiling_info;
            let tile_split =
                (tiling >> AMDGPU_TILING_TILE_SPLIT_SHIFT) & AMDGPU_TILING_TILE_SPLIT_MASK;
            if device.values.family <= AMDGPU_FAMILY_CZ && tile_split > 6 {
                return errno(EINVAL);
            }
            let data = bytes_of(&metadata.data.data)[..size].to_vec();
            let flags = metadata.data.flags;
            let found = buffers.with(metadata.handle, |buffer| {
                buffer.tiling = tiling;
                if size == 0 {
                    buffer.metadata.clear();
                } else {
                    buffer.metadata = data;
                    buffer.metadata_flags = flags;
                }
            });
            match found {
                Some(()) => 0,
                None => errno(ENOENT),
            }
        }
        op => {
            if !buffers.contains(metadata.handle) {
                return errno(ENOENT);
            }
            unimplemented!("GEM metadata op {op}");
        }
    }
}

fn entity(
    device: &Device,
    context: &mut Context,
    hw_ip: u32,
    instance: u32,
    ring: u32,
) -> Result<EntityKey, i32> {
    if hw_ip >= AMDGPU_HW_IP_NUM || instance != 0 || ring >= AMDGPU_CTX_NUM_ENTITIES[hw_ip as usize]
    {
        return Err(EINVAL);
    }
    let key = EntityKey { hw_ip, ring };
    if let Entry::Vacant(entry) = context.sequences.entry(key) {
        if device.rings(hw_ip) == 0 {
            return Err(EINVAL);
        }
        entry.insert(1);
    }
    Ok(key)
}

struct PostDep {
    object: u64,
    point: u64,
}

unsafe fn cs(
    device: &Device,
    file_id: u64,
    buffers: &FileBuffers<Buffer>,
    arg: *mut u8,
    user: &dyn UserMemory,
) -> i64 {
    let cs = unsafe { &mut *arg.cast::<drm_amdgpu_cs>() };
    let input = unsafe { cs.in_ };
    if input.num_chunks == 0 {
        return errno(EINVAL);
    }
    let mut files = device.files.lock();
    let file = files.entry(file_id).or_default();
    let Some(context) = file.contexts.get_mut(&input.ctx_id) else {
        return errno(EINVAL);
    };
    let chunk_pointers: Vec<u64> = match user.read_array(input.chunks, input.num_chunks as usize) {
        Ok(pointers) => pointers,
        Err(e) => return errno(e),
    };
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut gang: Vec<EntityKey> = Vec::new();
    let mut num_ibs = [0u32; AMDGPU_CS_GANG_SIZE];
    let mut leader = 0;
    let mut user_fence = false;
    let mut bo_list = false;
    let mut job_idx = |context: &mut Context, ib: &drm_amdgpu_cs_chunk_ib| -> Result<usize, i32> {
        let key = entity(device, context, ib.ip_type, ib.ip_instance, ib.ring)?;
        if let Some(i) = gang.iter().position(|&k| k == key) {
            return Ok(i);
        }
        if gang.len() == AMDGPU_CS_GANG_SIZE {
            return Err(EINVAL);
        }
        gang.push(key);
        Ok(gang.len() - 1)
    };
    for &pointer in &chunk_pointers {
        let header: drm_amdgpu_cs_chunk = match user.read_value(pointer) {
            Ok(header) => header,
            Err(e) => return errno(e),
        };
        let size = header.length_dw as usize * 4;
        let mut data = vec![0u8; size];
        if let Err(e) = user.read(header.chunk_data, &mut data) {
            return errno(e);
        }
        chunks.push(Chunk {
            chunk_id: header.chunk_id,
            data,
        });
        let chunk = chunks.last().unwrap();
        let ret = match chunk.chunk_id {
            AMDGPU_CHUNK_ID_IB => {
                if size < size_of::<drm_amdgpu_cs_chunk_ib>() {
                    return errno(EINVAL);
                }
                let ib: drm_amdgpu_cs_chunk_ib = unsafe { chunk.read() };
                job_idx(context, &ib).and_then(|i| {
                    if num_ibs[i] >= AMDGPU_RING_MAX_IBS[ib.ip_type as usize] {
                        return Err(EINVAL);
                    }
                    num_ibs[i] += 1;
                    leader = i;
                    Ok(())
                })
            }
            AMDGPU_CHUNK_ID_FENCE => {
                if size < size_of::<drm_amdgpu_cs_chunk_fence>() {
                    return errno(EINVAL);
                }
                if user_fence && device.kernel >= KernelVersion::V7_2 {
                    return errno(EINVAL);
                }
                user_fence = true;
                let fence: drm_amdgpu_cs_chunk_fence = unsafe { chunk.read() };
                match buffers.with(fence.handle, |buffer| buffer.info.bo_size) {
                    Some(size) if size == device.page_size && fence.offset as u64 <= size - 8 => {
                        Ok(())
                    }
                    _ => Err(EINVAL),
                }
            }
            AMDGPU_CHUNK_ID_BO_HANDLES => {
                if size < size_of::<drm_amdgpu_bo_list_in>() || bo_list {
                    return errno(EINVAL);
                }
                bo_list = true;
                let list: drm_amdgpu_bo_list_in = unsafe { chunk.read() };
                unsafe { bo_list_entries(device, buffers, &list, user) }
            }
            AMDGPU_CHUNK_ID_SYNCOBJ_OUT
            | AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_WAIT
            | AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_SIGNAL => Ok(()),
            chunk_id => unimplemented!("CS chunk {chunk_id}"),
        };
        if let Err(e) = ret {
            return errno(e);
        }
    }
    if gang.is_empty() {
        return errno(EINVAL);
    }
    let mut post_deps: Option<Vec<PostDep>> = None;
    let mut preempt = 0;
    for chunk in &chunks {
        let ret = match chunk.chunk_id {
            AMDGPU_CHUNK_ID_IB => {
                let ib: drm_amdgpu_cs_chunk_ib = unsafe { chunk.read() };
                let used = AMDGPU_IB_FLAG_PREAMBLE
                    | AMDGPU_IB_FLAG_PREEMPT
                    | AMDGPU_IB_FLAG_TC_WB_NOT_INVALIDATE
                    | AMDGPU_IB_FLAG_EMIT_MEM_SYNC;
                if ib.flags & AMDGPU_IB_FLAG_CE != 0 && device.kernel >= KernelVersion::V6_18 {
                    return errno(EINVAL);
                }
                if ib.flags & !used != 0 {
                    unimplemented!("IB flags {:#x}", ib.flags & !used);
                }
                if ib.ip_type == AMDGPU_HW_IP_GFX && ib.flags & AMDGPU_IB_FLAG_PREEMPT != 0 {
                    preempt += 1;
                }
                let too_large = device.kernel >= KernelVersion::V7_2
                    && ib.ib_bytes / 4 > AMDGPU_GFX_SDMA_IB_PACKET_SIZE_MAX_DW;
                match preempt > 1 || too_large {
                    true => Err(EINVAL),
                    false => Ok(()),
                }
            }
            AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_WAIT => {
                let deps: Vec<drm_amdgpu_cs_chunk_syncobj> = unsafe { chunk.array() };
                deps.iter().try_for_each(|dep| {
                    device
                        .syncobjs
                        .find_fence(file_id, dep.handle, dep.point, dep.flags as u64)
                        .map(|_| ())
                })
            }
            AMDGPU_CHUNK_ID_SYNCOBJ_OUT => {
                if post_deps.is_some() {
                    return errno(EINVAL);
                }
                let deps: Vec<drm_amdgpu_cs_chunk_sem> = unsafe { chunk.array() };
                let deps: Option<Vec<PostDep>> = deps
                    .iter()
                    .map(|dep| {
                        let object = device.syncobjs.object(file_id, dep.handle)?;
                        Some(PostDep { object, point: 0 })
                    })
                    .collect();
                match deps {
                    Some(deps) => {
                        post_deps = Some(deps);
                        Ok(())
                    }
                    None => Err(EINVAL),
                }
            }
            AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_SIGNAL => {
                if post_deps.is_some() {
                    return errno(EINVAL);
                }
                let deps: Vec<drm_amdgpu_cs_chunk_syncobj> = unsafe { chunk.array() };
                let deps: Option<Vec<PostDep>> = deps
                    .iter()
                    .map(|dep| {
                        let object = device.syncobjs.object(file_id, dep.handle)?;
                        Some(PostDep {
                            object,
                            point: dep.point,
                        })
                    })
                    .collect();
                match deps {
                    Some(deps) => {
                        post_deps = Some(deps);
                        Ok(())
                    }
                    None => Err(EINVAL),
                }
            }
            _ => Ok(()),
        };
        if let Err(e) = ret {
            return errno(e);
        }
    }
    if input.bo_list_handle != 0 {
        if bo_list {
            return errno(EINVAL);
        }
        unimplemented!("BO list handles");
    }
    let sequence = context.sequences.get_mut(&gang[leader]).unwrap();
    let seq = *sequence;
    *sequence += 1;
    drop(files);
    for dep in post_deps.into_iter().flatten() {
        match dep.point {
            0 => device.syncobjs.replace(dep.object, Some(Fence::Plain)),
            point => device.syncobjs.add_point(dep.object, point),
        }
    }
    cs.out = drm_amdgpu_cs_out { handle: seq };
    0
}

unsafe fn bo_list_entries(
    device: &Device,
    buffers: &FileBuffers<Buffer>,
    list: &drm_amdgpu_bo_list_in,
    user: &dyn UserMemory,
) -> Result<(), i32> {
    if device.kernel >= KernelVersion::V7_0 && list.bo_number > AMDGPU_BO_LIST_MAX_ENTRIES {
        return Err(EINVAL);
    }
    let copied = (list.bo_info_size as usize).min(size_of::<drm_amdgpu_bo_list_entry>());
    let mut handles = Vec::new();
    for i in 0..list.bo_number as u64 {
        let mut entry = [0u8; size_of::<drm_amdgpu_bo_list_entry>()];
        let source = list.bo_info_ptr.wrapping_add(i * list.bo_info_size as u64);
        user.read(source, &mut entry[..copied])?;
        handles.push(u32::from_ne_bytes(entry[..4].try_into().unwrap()));
    }
    match handles.iter().all(|&handle| buffers.contains(handle)) {
        true => Ok(()),
        false => Err(ENOENT),
    }
}

unsafe fn ctx(device: &Device, file_id: u64, arg: *mut u8) -> i64 {
    let args = unsafe { &mut *arg.cast::<drm_amdgpu_ctx>() };
    let input = unsafe { args.in_ };
    let priority = match input.priority {
        AMDGPU_CTX_PRIORITY_VERY_LOW
        | AMDGPU_CTX_PRIORITY_LOW
        | AMDGPU_CTX_PRIORITY_NORMAL
        | AMDGPU_CTX_PRIORITY_HIGH
        | AMDGPU_CTX_PRIORITY_VERY_HIGH => input.priority,
        _ => AMDGPU_CTX_PRIORITY_NORMAL,
    };
    let mut files = device.files.lock();
    let file = files.entry(file_id).or_default();
    match input.op {
        AMDGPU_CTX_OP_ALLOC_CTX => {
            if input.flags != 0 {
                return errno(EINVAL);
            }
            if priority > AMDGPU_CTX_PRIORITY_NORMAL {
                args.out.alloc.ctx_id = 0;
                return errno(EACCES);
            }
            let id = (1..).find(|id| !file.contexts.contains_key(id)).unwrap();
            file.contexts.insert(id, Context::default());
            args.out.alloc.ctx_id = id;
            0
        }
        AMDGPU_CTX_OP_FREE_CTX => {
            if input.flags != 0 {
                return errno(EINVAL);
            }
            match file.contexts.remove(&input.ctx_id) {
                Some(_) => 0,
                None => errno(EINVAL),
            }
        }
        op => unimplemented!("AMDGPU_CTX op {op}"),
    }
}

unsafe fn ioctl(
    device: &Device,
    file: u64,
    func: Func,
    arg: *mut u8,
    user: &dyn UserMemory,
) -> Option<i64> {
    let buffers = &device.buffers.file(file);
    let ret = unsafe {
        match func {
            Func::AmdgpuInfoIoctl => info(device, arg, user),
            Func::AmdgpuGemCreateIoctl => {
                let create = &mut *arg.cast::<drm_amdgpu_gem_create>();
                match create_info(device.kernel, device.page_size, &create.in_).and_then(
                    |mut info| {
                        place(&mut info)?;
                        Ok(info)
                    },
                ) {
                    Ok(info) => {
                        let handle = create_buffer(buffers, info).handle;
                        *create = std::mem::zeroed();
                        create.out.handle = handle;
                        0
                    }
                    Err(e) => errno(e),
                }
            }
            Func::AmdgpuGemMmapIoctl => {
                let mmap = &mut *arg.cast::<drm_amdgpu_gem_mmap>();
                let handle = mmap.in_.handle;
                *mmap = std::mem::zeroed();
                match buffers.with(handle, |buffer| buffer.info.domain_flags) {
                    None => errno(ENOENT),
                    Some(flags) if flags & AMDGPU_GEM_CREATE_NO_CPU_ACCESS != 0 => errno(EPERM),
                    Some(_) => match buffers.offset(handle) {
                        Some(offset) => {
                            mmap.out.addr_ptr = offset;
                            0
                        }
                        None => errno(ENOENT),
                    },
                }
            }
            Func::AmdgpuGemMetadataIoctl => gem_metadata(device, buffers, arg),
            Func::AmdgpuGemVaIoctl => gem_va(device, file, &mut *arg.cast::<drm_amdgpu_gem_va>()),
            Func::AmdgpuGemOpIoctl => {
                let op = &*arg.cast::<drm_amdgpu_gem_op>();
                if device.kernel >= KernelVersion::V6_18 && op.padding != 0 {
                    return Some(errno(EINVAL));
                }
                let Some(info) = buffers.with(op.handle, |buffer| buffer.info) else {
                    return Some(errno(ENOENT));
                };
                match op.op {
                    AMDGPU_GEM_OP_GET_GEM_CREATE_INFO => match user.write_value(op.value, &info) {
                        Ok(()) => 0,
                        Err(e) => errno(e),
                    },
                    op => unimplemented!("AMDGPU_GEM_OP op {op}"),
                }
            }
            Func::AmdgpuCtxIoctl => ctx(device, file, arg),
            Func::AmdgpuCsIoctl => cs(device, file, buffers, arg, user),
            _ => return None,
        }
    };
    Some(ret)
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "amdgpu"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        let mut id = format!(
            "amdgpu:{}:kernel={}:pte_fragment_size={}",
            self.values.name, self.kernel, self.pte_fragment_size
        );
        if self.page_size != PAGE_SIZE {
            id.push_str(&format!(":page_size={}", self.page_size));
        }
        id
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn amd_gfx_level(&self) -> Option<GfxLevel> {
        Some(self.values.gfx_level)
    }

    fn desc(&self) -> &'static str {
        "AMD GPU"
    }

    fn driver_version(&self) -> DriverVersion {
        self.kernel.amdgpu()
    }

    fn uevent_driver(&self) -> &'static str {
        "amdgpu"
    }

    fn pci(&self) -> Option<Pci> {
        Some(Pci {
            vendor: 0x1002,
            device: self.values.device_id,
        })
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        ioctls::AMDGPU
    }

    fn syncobjs(&self) -> Option<&Syncobjs> {
        Some(&self.syncobjs)
    }

    fn master(&self) -> &Master {
        &self.master
    }

    fn check_feature(&self, feature: DriverFeature) -> bool {
        match feature {
            DriverFeature::Modeset => true,
            DriverFeature::SyncobjTimeline => true,
        }
    }

    fn fb_modifiers_not_supported(&self) -> bool {
        false
    }

    unsafe fn ioctl(
        &self,
        file: u64,
        func: Func,
        arg: *mut u8,
        user: &dyn UserMemory,
        _prime: &mut dyn Prime,
    ) -> Option<i64> {
        unsafe { ioctl(self, file, func, arg, user) }
    }

    fn export(&self, file: u64, handle: u32) -> Result<Export, i32> {
        let buffers = self.buffers.file(file);
        let info = buffers.with(handle, |buffer| buffer.info).ok_or(ENOENT)?;
        let mut export = buffers.export(handle, |_| Ok(()))?;
        export.exporter_data = Some(Arc::new(info));
        Ok(export)
    }

    fn import(&self, file: u64, object: u32) -> Option<u32> {
        self.buffers.file(file).import(object)
    }

    fn import_foreign(&self, file: u64, export: &Export) -> NewHandle {
        let inherited = AMDGPU_GEM_CREATE_CPU_GTT_USWC
            | AMDGPU_GEM_CREATE_COHERENT
            | AMDGPU_GEM_CREATE_EXT_COHERENT
            | AMDGPU_GEM_CREATE_UNCACHED;
        let exporter = export.exporter_data.as_ref();
        let exporter = exporter.and_then(|data| data.downcast_ref::<drm_amdgpu_gem_create_in>());
        let flags = exporter.map_or(0, |info| info.domain_flags) & inherited;
        let info = bo_info(
            self.page_size,
            export.size,
            self.page_size,
            AMDGPU_GEM_DOMAIN_GTT,
            flags,
        );
        create_buffer(&self.buffers.file(file), info)
    }

    fn close_buffer(&self, file: u64, handle: u32) -> bool {
        let mut files = self.files.lock();
        let buffers = self.buffers.file(file);
        let Some(object) = buffers.object(handle) else {
            return false;
        };
        if let Some(file) = files.get_mut(&file) {
            file.mappings.retain(|mapping| mapping.object != object);
        }
        buffers.close(handle)
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
        let mut files = self.files.lock();
        files.remove(&file);
        self.buffers.release_file(file);
        self.syncobjs.release_file(file);
    }
}
