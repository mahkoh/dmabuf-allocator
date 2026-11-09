use crate::Drm;
use crate::Fault;
use crate::KernelVersion;
use crate::Prime;
use crate::amd::devices::GfxLevel;
use crate::amd::kernel::Device as AmdgpuDevice;
use crate::amdgpu_ids::ASIC_IDS;
use crate::uapi::amdgpu::AMDGPU_CHUNK_ID_BO_HANDLES;
use crate::uapi::amdgpu::AMDGPU_CHUNK_ID_DEPENDENCIES;
use crate::uapi::amdgpu::AMDGPU_CHUNK_ID_FENCE;
use crate::uapi::amdgpu::AMDGPU_CHUNK_ID_IB;
use crate::uapi::amdgpu::AMDGPU_CHUNK_ID_SYNCOBJ_OUT;
use crate::uapi::amdgpu::AMDGPU_CTX_OP_ALLOC_CTX;
use crate::uapi::amdgpu::AMDGPU_CTX_OP_FREE_CTX;
use crate::uapi::amdgpu::AMDGPU_FAMILY_AI;
use crate::uapi::amdgpu::AMDGPU_GEM_METADATA_OP_SET_METADATA;
use crate::uapi::amdgpu::AMDGPU_HW_IP_NUM;
use crate::uapi::amdgpu::AMDGPU_INFO_ACCEL_WORKING;
use crate::uapi::amdgpu::AMDGPU_INFO_DEV_INFO;
use crate::uapi::amdgpu::AMDGPU_INFO_GTT_USAGE;
use crate::uapi::amdgpu::AMDGPU_INFO_HW_IP_INFO;
use crate::uapi::amdgpu::AMDGPU_INFO_MMR_SE_INDEX_SHIFT;
use crate::uapi::amdgpu::AMDGPU_INFO_MMR_SH_INDEX_MASK;
use crate::uapi::amdgpu::AMDGPU_INFO_MMR_SH_INDEX_SHIFT;
use crate::uapi::amdgpu::AMDGPU_INFO_READ_MMR_REG;
use crate::uapi::amdgpu::AMDGPU_INFO_VIS_VRAM_USAGE;
use crate::uapi::amdgpu::AMDGPU_INFO_VRAM_GTT;
use crate::uapi::amdgpu::AMDGPU_INFO_VRAM_USAGE;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_CS;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_CTX;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_CREATE;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_METADATA;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_VA;
use crate::uapi::amdgpu::DRM_IOCTL_AMDGPU_INFO;
use crate::uapi::amdgpu::drm_amdgpu_bo_list_entry;
use crate::uapi::amdgpu::drm_amdgpu_bo_list_in;
use crate::uapi::amdgpu::drm_amdgpu_cs;
use crate::uapi::amdgpu::drm_amdgpu_cs_chunk;
use crate::uapi::amdgpu::drm_amdgpu_cs_chunk_fence;
use crate::uapi::amdgpu::drm_amdgpu_cs_chunk_ib;
use crate::uapi::amdgpu::drm_amdgpu_cs_chunk_sem;
use crate::uapi::amdgpu::drm_amdgpu_cs_in;
use crate::uapi::amdgpu::drm_amdgpu_ctx;
use crate::uapi::amdgpu::drm_amdgpu_gem_create;
use crate::uapi::amdgpu::drm_amdgpu_gem_create_in;
use crate::uapi::amdgpu::drm_amdgpu_gem_metadata;
use crate::uapi::amdgpu::drm_amdgpu_gem_va;
use crate::uapi::amdgpu::drm_amdgpu_info;
use crate::uapi::amdgpu::drm_amdgpu_info__anon__query_hw_ip;
use crate::uapi::amdgpu::drm_amdgpu_info__anon__read_mmr_reg;
use crate::uapi::amdgpu::drm_amdgpu_info_device;
use crate::uapi::amdgpu::drm_amdgpu_info_hw_ip;
use crate::uapi::amdgpu::drm_amdgpu_info_vram_gtt;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_CREATE;
use crate::uapi::drm::DRM_IOCTL_SYNCOBJ_DESTROY;
use crate::uapi::drm::drm_syncobj_create;
use crate::uapi::drm::drm_syncobj_destroy;
use crate::uapi::vdrm::AMDGPU_CCMD_BO_VA_OP;
use crate::uapi::vdrm::AMDGPU_CCMD_BO_VA_OP_SPARSE_BO;
use crate::uapi::vdrm::AMDGPU_CCMD_CREATE_CTX;
use crate::uapi::vdrm::AMDGPU_CCMD_CREATE_CTX_DESTROY;
use crate::uapi::vdrm::AMDGPU_CCMD_CS_QUERY_FENCE_STATUS;
use crate::uapi::vdrm::AMDGPU_CCMD_CS_SUBMIT;
use crate::uapi::vdrm::AMDGPU_CCMD_CS_SUBMIT_MAX_NUM_CHUNKS;
use crate::uapi::vdrm::AMDGPU_CCMD_GEM_NEW;
use crate::uapi::vdrm::AMDGPU_CCMD_QUERY_INFO;
use crate::uapi::vdrm::AMDGPU_CCMD_SET_METADATA;
use crate::uapi::vdrm::VIRTGPU_DRM_CONTEXT_AMDGPU;
use crate::uapi::vdrm::amdgpu_buffer_size_alignments;
use crate::uapi::vdrm::amdgpu_ccmd_bo_va_op_req;
use crate::uapi::vdrm::amdgpu_ccmd_create_ctx_req;
use crate::uapi::vdrm::amdgpu_ccmd_create_ctx_rsp;
use crate::uapi::vdrm::amdgpu_ccmd_cs_submit_req;
use crate::uapi::vdrm::amdgpu_ccmd_gem_new_req;
use crate::uapi::vdrm::amdgpu_ccmd_query_info_req;
use crate::uapi::vdrm::amdgpu_ccmd_rsp;
use crate::uapi::vdrm::amdgpu_ccmd_set_metadata_req;
use crate::uapi::vdrm::amdgpu_gpu_info;
use crate::uapi::vdrm::amdgpu_heap_info;
use crate::uapi::vdrm::amdvgpu_shmem;
use crate::uapi::vdrm::vdrm_ccmd_req;
use crate::uapi::vdrm::virgl_renderer_capset_drm__u__amdgpu;
use crate::user::from_bytes;
use crate::vdrm_host::Backend;
use crate::vdrm_host::Blob;
use crate::vdrm_host::Renderer;
use crate::vdrm_host::Shmem;
use crate::vdrm_host::attach_shmem;
use crate::vdrm_host::drm_capset;
use crate::vdrm_host::response;
use bytemuck::Zeroable;
use bytemuck::bytes_of;
use bytemuck::bytes_of_mut;
use libc::EINVAL;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::mem::offset_of;

const BACKEND_FILE_BASE: u64 = 1 << 62;
const MAX_SIZE: u32 = 1 << 20;
const AMDGPU_FAMILY_CI: u32 = 120;
const MM_CC_RB_BACKEND_DISABLE: u32 = 0x263d;
const MM_GB_ADDR_CONFIG: u32 = 0x263e;
const MM_GB_TILE_MODE0: u32 = 0x2644;
const MM_GB_MACROTILE_MODE0: u32 = 0x2664;
const MM_MC_ARB_RAMCFG: u32 = 0x9d8;
const MM_PA_SC_RASTER_CONFIG: u32 = 0xa0d4;
const MM_PA_SC_RASTER_CONFIG_1: u32 = 0xa0d5;
const AMDGPU_INFO_MMR_BROADCAST: u32 = 0xffff_ffff;
const DEFAULT_MARKETING_NAME: &str = "AMD Radeon Graphics";

struct Object {
    gem_handle: u32,
    res_id: Option<u32>,
    exported: bool,
}

pub struct Context {
    backend_file: u64,
    shmem: Option<Shmem>,
    blobs: BTreeMap<u32, u64>,
    objects: BTreeMap<u64, Object>,
    next_object: u64,
    timeline_count: u32,
    ctx_ids: BTreeSet<u32>,
}

pub struct Host {
    backend: Backend<AmdgpuDevice>,
    capset: Vec<u8>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
struct Desc {
    chunk_id: u16,
    length_dw: u16,
    offset: u32,
}

impl Host {
    pub fn new(chip: &str, kernel: KernelVersion) -> Self {
        let mut host = Self {
            backend: Backend::new(AmdgpuDevice::new(chip, kernel), BACKEND_FILE_BASE),
            capset: Vec::new(),
        };
        host.capset = host.probe();
        host
    }

    unsafe fn backend_ioctl<T>(
        &self,
        file: u64,
        cmd: u32,
        arg: &mut T,
        fault: Option<Fault>,
    ) -> i64 {
        unsafe { self.backend.ioctl_raw(file, cmd, arg, fault) }
    }

    fn query(
        &self,
        file: u64,
        query: u32,
        read_mmr_reg: drm_amdgpu_info__anon__read_mmr_reg,
        out: &mut [u8],
    ) -> i64 {
        let mut info: drm_amdgpu_info = Zeroable::zeroed();
        info.return_pointer = out.as_mut_ptr() as u64;
        info.return_size = out.len() as u32;
        info.query = query;
        info.anon.read_mmr_reg = read_mmr_reg;
        unsafe { self.backend_ioctl(file, DRM_IOCTL_AMDGPU_INFO, &mut info, None) }
    }

    fn read_registers(&self, file: u64, register: u32, instance: u32, out: &mut [u32]) {
        let read = drm_amdgpu_info__anon__read_mmr_reg {
            dword_offset: register,
            count: out.len() as u32,
            instance,
            flags: 0,
        };
        let mut bytes = vec![0u8; size_of_val(out)];
        assert_eq!(
            self.query(file, AMDGPU_INFO_READ_MMR_REG, read, &mut bytes),
            0
        );
        for (register, bytes) in out.iter_mut().zip(bytes.as_chunks::<4>().0) {
            *register = u32::from_ne_bytes(*bytes);
        }
    }

    fn probe(&self) -> Vec<u8> {
        let file = self.backend.open();
        let zero_mmr = drm_amdgpu_info__anon__read_mmr_reg {
            dword_offset: 0,
            count: 0,
            instance: 0,
            flags: 0,
        };
        let mut accel_working = [0u8; 4];
        assert_eq!(
            self.query(
                file,
                AMDGPU_INFO_ACCEL_WORKING,
                zero_mmr,
                &mut accel_working
            ),
            0
        );
        assert_ne!(u32::from_ne_bytes(accel_working), 0);
        let mut dev_info_bytes = vec![0u8; size_of::<drm_amdgpu_info_device>()];
        assert_eq!(
            self.query(file, AMDGPU_INFO_DEV_INFO, zero_mmr, &mut dev_info_bytes),
            0
        );
        let dev_info: drm_amdgpu_info_device = from_bytes(&dev_info_bytes);
        let pre_soc15 = dev_info.family < AMDGPU_FAMILY_AI;
        let mut gpu_info: amdgpu_gpu_info = Zeroable::zeroed();
        if pre_soc15 {
            let engines =
                (dev_info.num_shader_engines as usize).min(gpu_info.backend_disable.len());
            for engine in 0..engines {
                let instance = (engine as u32) << AMDGPU_INFO_MMR_SE_INDEX_SHIFT
                    | AMDGPU_INFO_MMR_SH_INDEX_MASK << AMDGPU_INFO_MMR_SH_INDEX_SHIFT;
                let mut backend_disable = [0];
                self.read_registers(
                    file,
                    MM_CC_RB_BACKEND_DISABLE,
                    instance,
                    &mut backend_disable,
                );
                gpu_info.backend_disable[engine] = (backend_disable[0] >> 16) & 0xff;
                self.read_registers(
                    file,
                    MM_PA_SC_RASTER_CONFIG,
                    instance,
                    &mut gpu_info.pa_sc_raster_cfg[engine..=engine],
                );
                if dev_info.family >= AMDGPU_FAMILY_CI {
                    self.read_registers(
                        file,
                        MM_PA_SC_RASTER_CONFIG_1,
                        instance,
                        &mut gpu_info.pa_sc_raster_cfg1[engine..=engine],
                    );
                }
            }
        }
        self.read_registers(
            file,
            MM_GB_ADDR_CONFIG,
            AMDGPU_INFO_MMR_BROADCAST,
            std::slice::from_mut(&mut gpu_info.gb_addr_cfg),
        );
        if pre_soc15 {
            self.read_registers(
                file,
                MM_GB_TILE_MODE0,
                AMDGPU_INFO_MMR_BROADCAST,
                &mut gpu_info.gb_tile_mode,
            );
            if dev_info.family >= AMDGPU_FAMILY_CI {
                self.read_registers(
                    file,
                    MM_GB_MACROTILE_MODE0,
                    AMDGPU_INFO_MMR_BROADCAST,
                    &mut gpu_info.gb_macro_tile_mode,
                );
            }
            self.read_registers(
                file,
                MM_MC_ARB_RAMCFG,
                AMDGPU_INFO_MMR_BROADCAST,
                std::slice::from_mut(&mut gpu_info.mc_arb_ramcfg),
            );
        }
        self.backend.release(file);
        gpu_info.asic_id = dev_info.device_id;
        gpu_info.chip_rev = dev_info.chip_rev;
        gpu_info.chip_external_rev = dev_info.external_rev;
        gpu_info.family_id = dev_info.family;
        gpu_info.max_engine_clk = dev_info.max_engine_clock;
        gpu_info.max_memory_clk = dev_info.max_memory_clock;
        gpu_info.gpu_counter_freq = dev_info.gpu_counter_freq;
        gpu_info.enabled_rb_pipes_mask = dev_info.enabled_rb_pipes_mask;
        gpu_info.rb_pipes = dev_info.num_rb_pipes;
        gpu_info.ids_flags = dev_info.ids_flags;
        gpu_info.num_hw_gfx_contexts = dev_info.num_hw_gfx_contexts;
        gpu_info.num_shader_engines = dev_info.num_shader_engines;
        gpu_info.num_shader_arrays_per_engine = dev_info.num_shader_arrays_per_engine;
        gpu_info.vram_type = dev_info.vram_type;
        gpu_info.vram_bit_width = dev_info.vram_bit_width;
        gpu_info.ce_ram_size = dev_info.ce_ram_size;
        gpu_info.vce_harvest_config = dev_info.vce_harvest_config;
        gpu_info.pci_rev_id = dev_info.pci_rev;
        gpu_info.cu_active_number = dev_info.cu_active_number;
        gpu_info.cu_ao_mask = dev_info.cu_ao_mask;
        gpu_info.cu_bitmap = dev_info.cu_bitmap;
        let low_32_max = dev_info.virtual_address_max.min(1 << 32);
        let high_32_max = dev_info
            .high_va_max
            .min((dev_info.high_va_offset & !0xffff_ffff) + (1 << 32));
        let address32_hi = match high_32_max {
            0 => (low_32_max - 1) >> 32,
            max => (max - 1) >> 32,
        };
        let name = ASIC_IDS
            .iter()
            .find(|id| {
                u32::from(id.device_id) == gpu_info.asic_id
                    && u32::from(id.revision_id) == gpu_info.pci_rev_id
            })
            .map(|id| id.marketing_name)
            .unwrap_or(DEFAULT_MARKETING_NAME);
        let mut marketing_name = [0u8; 128];
        let len = name.len().min(marketing_name.len() - 1);
        marketing_name[..len].copy_from_slice(&name.as_bytes()[..len]);
        let version = self.backend.kernel.driver_version();
        let mut amdgpu = virgl_renderer_capset_drm__u__amdgpu {
            address32_hi: address32_hi as u32,
            _bitfield_1: 0,
            alignments: amdgpu_buffer_size_alignments {
                size_local: dev_info.pte_fragment_size as u64,
                size_remote: dev_info.gart_page_size as u64,
            },
            gpu_info,
            marketing_name: bytemuck::cast(marketing_name),
        };
        amdgpu.set_has_vm_always_valid(1);
        drm_capset(0, &version, VIRTGPU_DRM_CONTEXT_AMDGPU, bytes_of(&amdgpu))
    }

    fn heap_info(&self, file: u64, vram: bool, cpu_access: bool) -> amdgpu_heap_info {
        let zero_mmr = drm_amdgpu_info__anon__read_mmr_reg {
            dword_offset: 0,
            count: 0,
            instance: 0,
            flags: 0,
        };
        let mut vram_gtt: drm_amdgpu_info_vram_gtt = Zeroable::zeroed();
        let vram_gtt_bytes = bytes_of_mut(&mut vram_gtt);
        assert_eq!(
            self.query(file, AMDGPU_INFO_VRAM_GTT, zero_mmr, vram_gtt_bytes),
            0
        );
        let mut usage = [0u8; 8];
        let (heap_size, usage_query) = match (vram, cpu_access) {
            (true, true) => (
                vram_gtt.vram_cpu_accessible_size,
                AMDGPU_INFO_VIS_VRAM_USAGE,
            ),
            (true, false) => (vram_gtt.vram_size, AMDGPU_INFO_VRAM_USAGE),
            (false, _) => (vram_gtt.gtt_size, AMDGPU_INFO_GTT_USAGE),
        };
        assert_eq!(self.query(file, usage_query, zero_mmr, &mut usage), 0);
        amdgpu_heap_info {
            heap_size,
            heap_usage: u64::from_ne_bytes(usage),
            max_allocation: vram_gtt.vram_cpu_accessible_size,
        }
    }

    fn update_heap_info(&self, context: &Context, prime: &mut dyn Prime) {
        let Some(shmem) = &context.shmem else {
            return;
        };
        let file = context.backend_file;
        let heaps = [
            (
                offset_of!(amdvgpu_shmem, vis_vram),
                self.heap_info(file, true, true),
            ),
            (
                offset_of!(amdvgpu_shmem, vram),
                self.heap_info(file, true, false),
            ),
            (
                offset_of!(amdvgpu_shmem, gtt),
                self.heap_info(file, false, false),
            ),
        ];
        for (offset, heap) in heaps {
            prime
                .write_file(shmem.field(offset), bytes_of(&heap))
                .unwrap();
        }
    }

    fn async_error(&self, context: &Context, prime: &mut dyn Prime) {
        let Some(shmem) = &context.shmem else {
            return;
        };
        let offset = shmem.field(offset_of!(amdvgpu_shmem, async_error));
        let mut value = [0u8; 4];
        prime.read_file(offset, &mut value).unwrap();
        let value = u32::from_ne_bytes(value).wrapping_add(1);
        prime.write_file(offset, &value.to_ne_bytes()).unwrap();
    }

    fn object_by_res_id(context: &Context, res_id: u32) -> Option<&Object> {
        context
            .objects
            .values()
            .find(|object| object.res_id == Some(res_id))
    }

    fn set_ret(response: &mut [u8], ret: i32) {
        let offset = offset_of!(amdgpu_ccmd_rsp, ret);
        response[offset..offset + 4].copy_from_slice(&ret.to_ne_bytes());
    }

    fn query_info(
        &self,
        context: &Context,
        req: amdgpu_ccmd_query_info_req,
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        let rsp_size = size_of::<amdgpu_ccmd_rsp>() as u32;
        let return_size = req.info.return_size;
        if return_size > MAX_SIZE
            || rsp_size
                .checked_add(return_size)
                .is_none_or(|len| len > MAX_SIZE)
        {
            return Err(EINVAL);
        }
        let mut response = response(
            context.shmem.as_ref(),
            &req.hdr,
            (rsp_size + return_size) as usize,
        )?;
        let mut value = vec![0u8; return_size as usize];
        let mut request = req.info;
        request.return_pointer = value.as_mut_ptr() as u64;
        let file = context.backend_file;
        let ret = unsafe { self.backend_ioctl(file, DRM_IOCTL_AMDGPU_INFO, &mut request, fault) };
        Self::set_ret(&mut response, ret as i32);
        response[rsp_size as usize..].copy_from_slice(&value);
        Ok(Some(response))
    }

    fn gem_new(
        &self,
        context: &mut Context,
        prime: &mut dyn Prime,
        req: amdgpu_ccmd_gem_new_req,
        fault: Option<Fault>,
    ) -> Result<(), i32> {
        let blob_id = req.blob_id as u32;
        if req.r.__pad != 0 || blob_id == 0 || context.blobs.contains_key(&blob_id) {
            self.async_error(context, prime);
            return Err(EINVAL);
        }
        let mut create: drm_amdgpu_gem_create = Zeroable::zeroed();
        create.in_ = drm_amdgpu_gem_create_in {
            bo_size: req.r.alloc_size,
            alignment: req.r.phys_alignment,
            domains: req.r.preferred_heap as u64,
            domain_flags: req.r.flags,
        };
        let file = context.backend_file;
        let ret =
            unsafe { self.backend_ioctl(file, DRM_IOCTL_AMDGPU_GEM_CREATE, &mut create, fault) };
        if ret != 0 {
            self.async_error(context, prime);
            return Err(-ret as i32);
        }
        let key = context.next_object;
        context.next_object += 1;
        context.objects.insert(
            key,
            Object {
                gem_handle: unsafe { create.out.handle },
                res_id: None,
                exported: false,
            },
        );
        context.blobs.insert(blob_id, key);
        Ok(())
    }

    fn bo_va_op(
        &self,
        context: &Context,
        prime: &mut dyn Prime,
        req: amdgpu_ccmd_bo_va_op_req,
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        let mut response = response(
            context.shmem.as_ref(),
            &req.hdr,
            size_of::<amdgpu_ccmd_rsp>(),
        )?;
        if req.flags2 & !AMDGPU_CCMD_BO_VA_OP_SPARSE_BO != 0 {
            return Err(EINVAL);
        }
        let handle = match req.flags2 & AMDGPU_CCMD_BO_VA_OP_SPARSE_BO != 0 {
            true => 0,
            false => match Self::object_by_res_id(context, req.res_id) {
                Some(object) => object.gem_handle,
                None => return Err(EINVAL),
            },
        };
        let mut va: drm_amdgpu_gem_va = Zeroable::zeroed();
        va.handle = handle;
        va.operation = req.op;
        va.flags = req.flags as u32;
        va.va_address = req.va;
        va.offset_in_bo = req.offset;
        va.map_size = req.vm_map_size;
        let file = context.backend_file;
        let ret = unsafe { self.backend_ioctl(file, DRM_IOCTL_AMDGPU_GEM_VA, &mut va, fault) };
        Self::set_ret(&mut response, ret as i32);
        if ret != 0 {
            self.async_error(context, prime);
        }
        Ok(Some(response))
    }

    fn set_metadata(
        &self,
        context: &Context,
        request: &[u8],
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        let req: amdgpu_ccmd_set_metadata_req = from_bytes(request);
        let mut response = response(
            context.shmem.as_ref(),
            &req.hdr,
            size_of::<amdgpu_ccmd_rsp>(),
        )?;
        let Some(object) = Self::object_by_res_id(context, req.res_id) else {
            return Err(EINVAL);
        };
        let mut metadata: drm_amdgpu_gem_metadata = Zeroable::zeroed();
        metadata.handle = object.gem_handle;
        metadata.op = AMDGPU_GEM_METADATA_OP_SET_METADATA;
        metadata.data.flags = req.flags;
        metadata.data.tiling_info = req.tiling_info;
        metadata.data.data_size_bytes = req.size_metadata;
        if req.size_metadata != 0 {
            let umd = size_of::<amdgpu_ccmd_set_metadata_req>();
            if req.size_metadata as usize > size_of_val(&metadata.data.data) {
                return Err(EINVAL);
            }
            if umd + req.size_metadata as usize > req.hdr.len as usize {
                return Err(EINVAL);
            }
            let source = &request[umd..umd + req.size_metadata as usize];
            bytes_of_mut(&mut metadata.data.data)[..source.len()].copy_from_slice(source);
        }
        let file = context.backend_file;
        let ret = unsafe {
            self.backend_ioctl(file, DRM_IOCTL_AMDGPU_GEM_METADATA, &mut metadata, fault)
        };
        Self::set_ret(&mut response, ret as i32);
        Ok(Some(response))
    }

    fn cs_submit(
        &self,
        context: &mut Context,
        prime: &mut dyn Prime,
        request: &[u8],
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        let req: amdgpu_ccmd_cs_submit_req = from_bytes(request);
        let mut response = response(
            context.shmem.as_ref(),
            &req.hdr,
            size_of::<amdgpu_ccmd_rsp>(),
        )?;
        if req.num_chunks > AMDGPU_CCMD_CS_SUBMIT_MAX_NUM_CHUNKS
            || req.ring_idx == 0
            || context.timeline_count < req.ring_idx
        {
            return Err(EINVAL);
        }
        if !context.ctx_ids.contains(&req.ctx_id) {
            unimplemented!("CS_SUBMIT with unknown context {}", req.ctx_id);
        }
        let ret = self.cs_submit_chunks(context, &req, request, fault)?;
        if ret != 0 {
            self.async_error(context, prime);
            return Err(-ret);
        }
        Self::set_ret(&mut response, 0);
        Ok(Some(response))
    }

    fn cs_submit_chunks(
        &self,
        context: &Context,
        req: &amdgpu_ccmd_cs_submit_req,
        request: &[u8],
        fault: Option<Fault>,
    ) -> Result<i32, i32> {
        let descriptor_size = size_of::<Desc>();
        let descriptors_len =
            size_of::<amdgpu_ccmd_cs_submit_req>() + req.num_chunks as usize * descriptor_size;
        if descriptors_len > req.hdr.len as usize {
            return Err(EINVAL);
        }
        let mut bo_list: Option<Vec<drm_amdgpu_bo_list_entry>> = None;
        let mut bo_list_in: drm_amdgpu_bo_list_in = Zeroable::zeroed();
        let mut user_fence: drm_amdgpu_cs_chunk_fence = Zeroable::zeroed();
        let mut chunks: Vec<drm_amdgpu_cs_chunk> = Vec::new();
        let mut chunk_data: Vec<Vec<u8>> = Vec::new();
        enum Data {
            BoList,
            Fence,
            Copied(usize),
        }
        let mut sources = Vec::new();
        for i in 0..req.num_chunks as usize {
            let base = size_of::<amdgpu_ccmd_cs_submit_req>() + i * descriptor_size;
            let desc: Desc = from_bytes(&request[base..base + descriptor_size]);
            let chunk_id = desc.chunk_id as u32;
            let length_dw = desc.length_dw;
            let offset = descriptors_len + desc.offset as usize;
            let len = length_dw as usize * 4;
            let end = offset + len;
            if end > req.hdr.len as usize {
                return Err(EINVAL);
            }
            let input = &request[offset..end];
            match chunk_id {
                AMDGPU_CHUNK_ID_BO_HANDLES => {
                    let entry_size = size_of::<drm_amdgpu_bo_list_entry>();
                    let count = len / entry_size;
                    if !offset.is_multiple_of(align_of::<drm_amdgpu_bo_list_entry>())
                        || bo_list.is_some()
                    {
                        return Err(EINVAL);
                    }
                    let mut list = Vec::new();
                    for j in 0..count {
                        let entry: drm_amdgpu_bo_list_entry =
                            from_bytes(&input[j * entry_size..(j + 1) * entry_size]);
                        let Some(object) = Self::object_by_res_id(context, entry.bo_handle) else {
                            return Err(EINVAL);
                        };
                        list.push(drm_amdgpu_bo_list_entry {
                            bo_handle: object.gem_handle,
                            bo_priority: entry.bo_priority,
                        });
                    }
                    bo_list = Some(list);
                    chunks.push(drm_amdgpu_cs_chunk {
                        chunk_id,
                        length_dw: (size_of::<drm_amdgpu_bo_list_in>() / 4) as u32,
                        chunk_data: 0,
                    });
                    sources.push(Data::BoList);
                }
                AMDGPU_CHUNK_ID_FENCE => {
                    if !offset.is_multiple_of(align_of::<drm_amdgpu_cs_chunk_fence>())
                        || len < size_of::<drm_amdgpu_cs_chunk_fence>()
                    {
                        return Err(EINVAL);
                    }
                    let fence: drm_amdgpu_cs_chunk_fence = from_bytes(input);
                    if !fence.offset.is_multiple_of(8) {
                        return Err(EINVAL);
                    }
                    let Some(object) = Self::object_by_res_id(context, fence.handle) else {
                        return Err(EINVAL);
                    };
                    user_fence = drm_amdgpu_cs_chunk_fence {
                        handle: object.gem_handle,
                        offset: fence.offset / 8 * 8,
                    };
                    chunks.push(drm_amdgpu_cs_chunk {
                        chunk_id,
                        length_dw: (size_of::<drm_amdgpu_cs_chunk_fence>() / 4) as u32,
                        chunk_data: 0,
                    });
                    sources.push(Data::Fence);
                }
                AMDGPU_CHUNK_ID_DEPENDENCIES | AMDGPU_CHUNK_ID_IB => {
                    if chunk_id == AMDGPU_CHUNK_ID_IB
                        && length_dw as usize != size_of::<drm_amdgpu_cs_chunk_ib>() / 4
                    {
                        return Err(EINVAL);
                    }
                    chunks.push(drm_amdgpu_cs_chunk {
                        chunk_id,
                        length_dw: length_dw as u32,
                        chunk_data: 0,
                    });
                    chunk_data.push(input.to_vec());
                    sources.push(Data::Copied(chunk_data.len() - 1));
                }
                _ => return Err(EINVAL),
            }
        }
        let file = context.backend_file;
        let mut create: drm_syncobj_create = Zeroable::zeroed();
        let ret = unsafe { self.backend_ioctl(file, DRM_IOCTL_SYNCOBJ_CREATE, &mut create, fault) };
        if ret != 0 {
            return Err(-ret as i32);
        }
        let mut syncobj_out = drm_amdgpu_cs_chunk_sem {
            handle: create.handle,
        };
        chunks.push(drm_amdgpu_cs_chunk {
            chunk_id: AMDGPU_CHUNK_ID_SYNCOBJ_OUT,
            length_dw: (size_of::<drm_amdgpu_cs_chunk_sem>() / 4) as u32,
            chunk_data: &mut syncobj_out as *mut drm_amdgpu_cs_chunk_sem as u64,
        });
        if let Some(list) = &mut bo_list {
            bo_list_in = drm_amdgpu_bo_list_in {
                operation: !0,
                list_handle: !0,
                bo_number: list.len() as u32,
                bo_info_size: size_of::<drm_amdgpu_bo_list_entry>() as u32,
                bo_info_ptr: list.as_mut_ptr() as u64,
            };
        }
        for (chunk, source) in chunks.iter_mut().zip(&sources) {
            chunk.chunk_data = match source {
                Data::BoList => &mut bo_list_in as *mut drm_amdgpu_bo_list_in as u64,
                Data::Fence => &mut user_fence as *mut drm_amdgpu_cs_chunk_fence as u64,
                Data::Copied(index) => chunk_data[*index].as_ptr() as u64,
            };
        }
        let chunk_pointers: Vec<u64> = chunks
            .iter()
            .map(|chunk| chunk as *const drm_amdgpu_cs_chunk as u64)
            .collect();
        let mut cs: drm_amdgpu_cs = Zeroable::zeroed();
        cs.in_ = drm_amdgpu_cs_in {
            ctx_id: req.ctx_id,
            bo_list_handle: 0,
            num_chunks: chunks.len() as u32,
            flags: 0,
            chunks: chunk_pointers.as_ptr() as u64,
        };
        let ret = unsafe { self.backend_ioctl(file, DRM_IOCTL_AMDGPU_CS, &mut cs, fault) };
        let mut destroy = drm_syncobj_destroy {
            handle: create.handle,
            pad: 0,
        };
        let destroyed =
            unsafe { self.backend_ioctl(file, DRM_IOCTL_SYNCOBJ_DESTROY, &mut destroy, None) };
        assert_eq!(destroyed, 0);
        Ok(ret as i32)
    }

    fn create_ctx(
        &self,
        context: &mut Context,
        req: amdgpu_ccmd_create_ctx_req,
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        let mut response = response(
            context.shmem.as_ref(),
            &req.hdr,
            size_of::<amdgpu_ccmd_create_ctx_rsp>(),
        )?;
        if req.flags & !AMDGPU_CCMD_CREATE_CTX_DESTROY != 0 {
            return Err(EINVAL);
        }
        if req.flags & AMDGPU_CCMD_CREATE_CTX_DESTROY != 0 {
            let id = unsafe { req.anon.id };
            if context.ctx_ids.remove(&id) {
                let mut ctx: drm_amdgpu_ctx = Zeroable::zeroed();
                ctx.in_.op = AMDGPU_CTX_OP_FREE_CTX;
                ctx.in_.ctx_id = id;
                let file = context.backend_file;
                unsafe {
                    self.backend_ioctl(file, DRM_IOCTL_AMDGPU_CTX, &mut ctx, fault);
                }
                Self::set_ret(&mut response, 0);
            }
            return Ok(Some(response));
        }
        let mut ctx: drm_amdgpu_ctx = Zeroable::zeroed();
        ctx.in_.op = AMDGPU_CTX_OP_ALLOC_CTX;
        ctx.in_.priority = unsafe { req.anon.priority };
        let file = context.backend_file;
        let ret = unsafe { self.backend_ioctl(file, DRM_IOCTL_AMDGPU_CTX, &mut ctx, fault) };
        Self::set_ret(&mut response, ret as i32);
        if ret == 0 {
            let ctx_id = unsafe { ctx.out.alloc.ctx_id };
            context.ctx_ids.insert(ctx_id);
            let offset = offset_of!(amdgpu_ccmd_create_ctx_rsp, ctx_id);
            response[offset..offset + 4].copy_from_slice(&ctx_id.to_ne_bytes());
        }
        Ok(Some(response))
    }
}

impl Renderer for Host {
    type Context = Context;

    const CONTEXT_TYPE: u32 = VIRTGPU_DRM_CONTEXT_AMDGPU;
    const CCMD_ALIGNMENT: u32 = 8;
    const DISPATCH_SIZE: u32 = AMDGPU_CCMD_CS_QUERY_FENCE_STATUS + 1;

    fn capset(&self) -> &[u8] {
        &self.capset
    }

    fn gfx_level(&self) -> Option<GfxLevel> {
        self.backend.kernel.amd_gfx_level()
    }

    fn create_context(&self) -> Context {
        let backend_file = self.backend.open();
        let mut timeline_count = 0;
        for ip_type in 0..AMDGPU_HW_IP_NUM {
            let mut ip_info = vec![0u8; size_of::<drm_amdgpu_info_hw_ip>()];
            let mut info: drm_amdgpu_info = Zeroable::zeroed();
            info.return_pointer = ip_info.as_mut_ptr() as u64;
            info.return_size = ip_info.len() as u32;
            info.query = AMDGPU_INFO_HW_IP_INFO;
            info.anon.query_hw_ip = drm_amdgpu_info__anon__query_hw_ip {
                type_: ip_type,
                ip_instance: 0,
            };
            let ret =
                unsafe { self.backend_ioctl(backend_file, DRM_IOCTL_AMDGPU_INFO, &mut info, None) };
            if ret < 0 {
                continue;
            }
            let ip_info: drm_amdgpu_info_hw_ip = from_bytes(&ip_info);
            timeline_count += ip_info.available_rings.count_ones();
        }
        assert_ne!(timeline_count, 0);
        Context {
            backend_file,
            shmem: None,
            blobs: BTreeMap::new(),
            objects: BTreeMap::new(),
            next_object: 1,
            timeline_count,
            ctx_ids: BTreeSet::new(),
        }
    }

    fn destroy_context(&self, context: Context) {
        self.backend.release(context.backend_file);
    }

    fn shmem(context: &Context) -> Option<&Shmem> {
        context.shmem.as_ref()
    }

    fn get_blob(&self, context: &mut Context, prime: &mut dyn Prime, blob: Blob) {
        if blob.blob_id >> 32 != 0 {
            return;
        }
        if blob.blob_id == 0 {
            let size = size_of::<amdvgpu_shmem>();
            if attach_shmem(&mut context.shmem, prime, &blob, size) {
                self.update_heap_info(context, prime);
            }
            return;
        }
        if Self::object_by_res_id(context, blob.res_id).is_some() {
            return;
        }
        let Some(key) = context.blobs.remove(&(blob.blob_id as u32)) else {
            return;
        };
        let object = context.objects.get_mut(&key).unwrap();
        if object.exported {
            return;
        }
        object.res_id = Some(blob.res_id);
        object.exported = true;
        self.update_heap_info(context, prime);
    }

    fn dispatch(
        &self,
        context: &mut Context,
        prime: &mut dyn Prime,
        hdr: &vdrm_ccmd_req,
        request: &[u8],
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        match hdr.cmd {
            AMDGPU_CCMD_QUERY_INFO => self.query_info(context, from_bytes(request), fault),
            AMDGPU_CCMD_GEM_NEW => {
                self.gem_new(context, prime, from_bytes(request), fault)?;
                Ok(None)
            }
            AMDGPU_CCMD_BO_VA_OP => self.bo_va_op(context, prime, from_bytes(request), fault),
            AMDGPU_CCMD_SET_METADATA => self.set_metadata(context, request, fault),
            AMDGPU_CCMD_CREATE_CTX => self.create_ctx(context, from_bytes(request), fault),
            AMDGPU_CCMD_CS_SUBMIT => self.cs_submit(context, prime, request, fault),
            cmd => unimplemented!("amdgpu native context command {cmd}"),
        }
    }
}
