use crate::Buffers;
use crate::DriverFeature;
use crate::DriverVersion;
use crate::Drm;
use crate::Export;
use crate::Kernel;
use crate::KernelVersion;
use crate::Master;
use crate::NewHandle;
use crate::PAGE_SHIFT;
use crate::PAGE_SIZE;
use crate::Platform;
use crate::Prime;
use crate::SZ_4G;
use crate::Syncobjs;
use crate::UserMemory;
use crate::drm_mm::DrmMm;
use crate::drm_mm::Request;
use crate::id_fields;
use crate::ioctls;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::string::strndup_user;
use crate::uapi::panfrost::DRM_PANFROST_GPU_COHERENCY_NONE;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_AFBC_FEATURES;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_ALLOWED_JM_CTX_PRIORITIES;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_GPU_PROD_ID;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_GPU_REVISION;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_L2_FEATURES;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_MAX_THREADS;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_MEM_FEATURES;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_MMU_FEATURES;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_SELECTED_COHERENCY;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_SHADER_PRESENT;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_SYSTEM_TIMESTAMP;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_SYSTEM_TIMESTAMP_FREQUENCY;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_TEXTURE_FEATURES0;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_TEXTURE_FEATURES3;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_THREAD_FEATURES;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_THREAD_MAX_WORKGROUP_SZ;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_THREAD_TLS_ALLOC;
use crate::uapi::panfrost::DRM_PANFROST_PARAM_TILER_FEATURES;
use crate::uapi::panfrost::PANFROST_BO_HEAP;
use crate::uapi::panfrost::PANFROST_BO_NOEXEC;
use crate::uapi::panfrost::PANFROST_BO_WB_MMAP;
use crate::uapi::panfrost::PANFROST_JM_CTX_PRIORITY_LOW;
use crate::uapi::panfrost::PANFROST_JM_CTX_PRIORITY_MEDIUM;
use crate::uapi::panfrost::PANFROST_MADV_DONTNEED;
use crate::uapi::panfrost::drm_panfrost_create_bo;
use crate::uapi::panfrost::drm_panfrost_get_bo_offset;
use crate::uapi::panfrost::drm_panfrost_get_param;
use crate::uapi::panfrost::drm_panfrost_madvise;
use crate::uapi::panfrost::drm_panfrost_mmap_bo;
use crate::uapi::panfrost::drm_panfrost_set_label_bo;
use libc::E2BIG;
use libc::EINVAL;
use libc::ENOENT;
use libc::ENOSPC;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::ops::Range;
use std::sync::Arc;

const SZ_2M: u64 = 2 << 20;
const SZ_16M: u64 = 16 << 20;
const SZ_32M: u64 = 32 << 20;
const PFN_4G: u64 = SZ_4G >> PAGE_SHIFT;
const PFN_4G_MASK: u64 = PFN_4G - 1;
const PFN_16M: u64 = SZ_16M >> PAGE_SHIFT;
const PANFROST_BO_LABEL_MAXLEN: u64 = 4096;

struct Features {
    revision: u32,
    shader_present: u64,
    l2_features: u32,
    tiler_features: u32,
    mem_features: u32,
    mmu_features: u32,
    thread_features: u32,
    max_threads: u32,
    thread_max_workgroup_sz: u32,
    texture_features: [u32; 4],
    thread_tls_alloc: u32,
}

const MESA_DRM_SHIM: Features = Features {
    revision: 0,
    shader_present: 0xf,
    l2_features: 0x7120306,
    tiler_features: 0x809,
    mem_features: 0,
    mmu_features: 0x00280030,
    thread_features: 0,
    max_threads: 256,
    thread_max_workgroup_sz: 256,
    texture_features: [0xffff_ffff; 4],
    thread_tls_alloc: 0,
};

pub struct Gpu {
    pub name: &'static str,
    prod_id: u64,
    features: Features,
    platform: Option<Platform>,
}

pub static GPUS: &[Gpu] = &[
    Gpu {
        name: "t720",
        prod_id: 0x0720,
        features: MESA_DRM_SHIM,
        platform: Some(Platform {
            fullname: "/soc/gpu@1800000",
            compatible: &["allwinner,sun50i-h6-mali", "arm,mali-t720"],
        }),
    },
    Gpu {
        name: "t860",
        prod_id: 0x0860,
        features: MESA_DRM_SHIM,
        platform: Some(Platform {
            fullname: "/gpu@ff9a0000",
            compatible: &["rockchip,rk3399-mali", "arm,mali-t860"],
        }),
    },
    Gpu {
        name: "g52",
        prod_id: 0x7212,
        features: MESA_DRM_SHIM,
        platform: Some(Platform {
            fullname: "/gpu@fde60000",
            compatible: &["rockchip,rk3568-mali", "arm,mali-bifrost"],
        }),
    },
    Gpu {
        name: "g57",
        prod_id: 0x9001,
        features: MESA_DRM_SHIM,
        platform: Some(Platform {
            fullname: "/soc/gpu@13000000",
            compatible: &[
                "mediatek,mt8195-mali",
                "mediatek,mt8192-mali",
                "arm,mali-valhall-jm",
            ],
        }),
    },
];

const fn test_gpu(name: &'static str, prod_id: u64) -> Gpu {
    Gpu {
        name,
        prod_id,
        features: MESA_DRM_SHIM,
        platform: None,
    }
}

static TEST_GPUS: &[Gpu] = &[
    test_gpu("t760", 0x0750),
    test_gpu("t820", 0x0820),
    test_gpu("t830", 0x0830),
    test_gpu("t880", 0x0880),
    test_gpu("g72", 0x6201),
];

fn color_adjust(color: u64, hole: Range<u64>) -> Range<u64> {
    if color & PANFROST_BO_NOEXEC as u64 != 0 {
        return hole;
    }
    let Range { mut start, mut end } = hole;
    if start & PFN_4G_MASK == 0 {
        start += 1;
    }
    if end & PFN_4G_MASK == 0 {
        end -= 1;
    }
    let next_seg = start.next_multiple_of(PFN_4G);
    if next_seg - start <= PFN_16M {
        start = next_seg + 1;
    }
    start..end.min(start.next_multiple_of(PFN_4G) - 1)
}

struct Mapping {
    file: u64,
    mmu: Arc<Mutex<DrmMm>>,
    first_page: u64,
}

impl Drop for Mapping {
    fn drop(&mut self) {
        self.mmu.lock().remove(self.first_page);
    }
}

struct Buffer {
    heap: bool,
    noexec: bool,
    mappings: Vec<Mapping>,
}

pub struct Device {
    gpu: &'static Gpu,
    kernel: KernelVersion,
    afbc_features: u64,
    buffers: Buffers<Buffer>,
    mmus: Mutex<BTreeMap<u64, Arc<Mutex<DrmMm>>>>,
    syncobjs: Syncobjs,
    master: Master,
}

impl Device {
    pub fn new(name: &str, kernel: KernelVersion) -> Self {
        let gpu = GPUS
            .iter()
            .chain(TEST_GPUS)
            .find(|gpu| gpu.name == name)
            .unwrap_or_else(|| panic!("unknown panfrost GPU {name:?}"));
        Self {
            gpu,
            kernel,
            afbc_features: 0,
            buffers: Buffers::default(),
            mmus: Mutex::new(BTreeMap::new()),
            syncobjs: Syncobjs::default(),
            master: Master::default(),
        }
    }

    pub fn from_id(fields: &str) -> Self {
        let values = id_fields(fields, &["gpu", "kernel", "afbc_features"]);
        let mut device = Self::new(
            values.get("gpu"),
            KernelVersion::parse(values.get("kernel")),
        );
        device.afbc_features = values
            .get("afbc_features")
            .parse()
            .unwrap_or_else(|_| panic!("invalid afbc_features {:?}", values.get("afbc_features")));
        device
    }

    fn param(&self, param: u32) -> Option<u64> {
        let features = &self.gpu.features;
        let value = match param {
            DRM_PANFROST_PARAM_GPU_PROD_ID => self.gpu.prod_id,
            DRM_PANFROST_PARAM_GPU_REVISION => features.revision as u64,
            DRM_PANFROST_PARAM_SHADER_PRESENT => features.shader_present,
            DRM_PANFROST_PARAM_L2_FEATURES => features.l2_features as u64,
            DRM_PANFROST_PARAM_TILER_FEATURES => features.tiler_features as u64,
            DRM_PANFROST_PARAM_MEM_FEATURES => features.mem_features as u64,
            DRM_PANFROST_PARAM_MMU_FEATURES => features.mmu_features as u64,
            DRM_PANFROST_PARAM_THREAD_FEATURES => features.thread_features as u64,
            DRM_PANFROST_PARAM_MAX_THREADS => features.max_threads as u64,
            DRM_PANFROST_PARAM_THREAD_MAX_WORKGROUP_SZ => features.thread_max_workgroup_sz as u64,
            DRM_PANFROST_PARAM_TEXTURE_FEATURES0..=DRM_PANFROST_PARAM_TEXTURE_FEATURES3 => {
                let index = param - DRM_PANFROST_PARAM_TEXTURE_FEATURES0;
                features.texture_features[index as usize] as u64
            }
            DRM_PANFROST_PARAM_THREAD_TLS_ALLOC => features.thread_tls_alloc as u64,
            DRM_PANFROST_PARAM_AFBC_FEATURES => self.afbc_features,
            DRM_PANFROST_PARAM_ALLOWED_JM_CTX_PRIORITIES if self.kernel >= KernelVersion::V6_19 => {
                1 << PANFROST_JM_CTX_PRIORITY_LOW | 1 << PANFROST_JM_CTX_PRIORITY_MEDIUM
            }
            DRM_PANFROST_PARAM_SELECTED_COHERENCY if self.kernel >= KernelVersion::V7_0 => {
                DRM_PANFROST_GPU_COHERENCY_NONE
            }
            DRM_PANFROST_PARAM_SYSTEM_TIMESTAMP_FREQUENCY => 0,
            DRM_PANFROST_PARAM_SYSTEM_TIMESTAMP => unimplemented!("panfrost timestamps"),
            _ if param < DRM_PANFROST_PARAM_SYSTEM_TIMESTAMP => {
                unimplemented!("panfrost param {param}");
            }
            _ => return None,
        };
        Some(value)
    }

    unsafe fn get_param(&self, arg: *mut u8) -> i64 {
        let data = unsafe { &mut *arg.cast::<drm_panfrost_get_param>() };
        if data.pad != 0 {
            return -(EINVAL as i64);
        }
        match self.param(data.param) {
            Some(value) => {
                data.value = value;
                0
            }
            None => -(EINVAL as i64),
        }
    }

    fn mmu(&self, file: u64) -> Arc<Mutex<DrmMm>> {
        let mut mmus = self.mmus.lock();
        let mmu = mmus.entry(file).or_insert_with(|| {
            let mm = DrmMm::new(SZ_32M >> PAGE_SHIFT, (SZ_4G - SZ_32M) >> PAGE_SHIFT);
            Arc::new(Mutex::new(mm.with_color_adjust(color_adjust)))
        });
        mmu.clone()
    }

    fn open(&self, file: u64, handle: u32) -> Result<u64, i32> {
        let size = self.buffers.file(file).size(handle).ok_or(EINVAL)?;
        let mmu = self.mmu(file);
        let mapped = self.buffers.file(file).with(handle, |buffer| {
            if let Some(mapping) = buffer.mappings.iter().find(|m| m.file == file) {
                return Ok(mapping.first_page);
            }
            let pages = size >> PAGE_SHIFT;
            let request = Request {
                size: pages,
                alignment: match buffer.noexec {
                    false => pages,
                    true if size >= SZ_2M => SZ_2M >> PAGE_SHIFT,
                    true => 0,
                },
                color: match buffer.noexec {
                    true => PANFROST_BO_NOEXEC as u64,
                    false => 0,
                },
                range: 0..u64::MAX,
            };
            let first_page = mmu.lock().insert(&request).ok_or(ENOSPC)?;
            buffer.mappings.push(Mapping {
                file,
                mmu: mmu.clone(),
                first_page,
            });
            Ok(first_page)
        });
        mapped.ok_or(EINVAL)?.map(|page| page << PAGE_SHIFT)
    }

    fn close(&self, file: u64, handle: u32) -> bool {
        let buffers = self.buffers.file(file);
        buffers.with(handle, |buffer| buffer.mappings.retain(|m| m.file != file));
        buffers.close(handle)
    }

    unsafe fn create_bo(&self, file: u64, arg: *mut u8) -> i64 {
        let data = unsafe { &mut *arg.cast::<drm_panfrost_create_bo>() };
        let all = match self.kernel >= KernelVersion::V7_0 {
            true => PANFROST_BO_NOEXEC | PANFROST_BO_HEAP | PANFROST_BO_WB_MMAP,
            false => PANFROST_BO_NOEXEC | PANFROST_BO_HEAP,
        };
        if data.size == 0 || data.pad != 0 || data.flags & !all != 0 {
            return -(EINVAL as i64);
        }
        let heap = data.flags & PANFROST_BO_HEAP != 0;
        if heap && data.flags & PANFROST_BO_NOEXEC == 0 {
            return -(EINVAL as i64);
        }
        if heap && data.flags & PANFROST_BO_WB_MMAP != 0 {
            return -(EINVAL as i64);
        }
        let mut size = data.size as u64;
        if heap {
            size = size.next_multiple_of(SZ_2M);
        }
        let size = size.next_multiple_of(PAGE_SIZE);
        let buffer = Buffer {
            heap,
            noexec: data.flags & PANFROST_BO_NOEXEC != 0,
            mappings: Vec::new(),
        };
        let buffers = self.buffers.file(file);
        let handle = buffers.create_with_offset(buffer, size).handle;
        match self.open(file, handle) {
            Ok(offset) => {
                data.handle = handle;
                data.offset = offset;
                0
            }
            Err(errno) => {
                buffers.close(handle);
                -(errno as i64)
            }
        }
    }

    unsafe fn mmap_bo(&self, file: u64, arg: *mut u8) -> i64 {
        let data = unsafe { &mut *arg.cast::<drm_panfrost_mmap_bo>() };
        if data.flags != 0 {
            return -(EINVAL as i64);
        }
        let buffers = self.buffers.file(file);
        match buffers.with(data.handle, |buffer| buffer.heap) {
            None => return -(ENOENT as i64),
            Some(true) => return -(EINVAL as i64),
            Some(false) => {}
        }
        data.offset = buffers.offset(data.handle).unwrap();
        0
    }

    unsafe fn set_label_bo(&self, file: u64, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let data = unsafe { &*arg.cast::<drm_panfrost_set_label_bo>() };
        if data.pad != 0 {
            return -(EINVAL as i64);
        }
        if !self.buffers.file(file).contains(data.handle) {
            return -(ENOENT as i64);
        }
        if data.label == 0 {
            return 0;
        }
        match strndup_user(user, data.label, PANFROST_BO_LABEL_MAXLEN) {
            Ok(_) => 0,
            Err(EINVAL) => -(E2BIG as i64),
            Err(errno) => -(errno as i64),
        }
    }

    unsafe fn madvise(&self, file: u64, arg: *mut u8) -> i64 {
        let data = unsafe { &mut *arg.cast::<drm_panfrost_madvise>() };
        let buffers = self.buffers.file(file);
        let Some(mappings) = buffers.with(data.handle, |buffer| buffer.mappings.len()) else {
            return -(ENOENT as i64);
        };
        if data.madv == PANFROST_MADV_DONTNEED && mappings != 1 {
            return -(EINVAL as i64);
        }
        data.retained = 1;
        0
    }

    unsafe fn get_bo_offset(&self, file: u64, arg: *mut u8) -> i64 {
        let data = unsafe { &mut *arg.cast::<drm_panfrost_get_bo_offset>() };
        let buffers = self.buffers.file(file);
        let mapping = buffers.with(data.handle, |buffer| {
            let mapping = buffer.mappings.iter().find(|m| m.file == file);
            mapping.map(|mapping| mapping.first_page)
        });
        match mapping {
            None => -(ENOENT as i64),
            Some(None) => -(EINVAL as i64),
            Some(Some(page)) => {
                data.offset = page << PAGE_SHIFT;
                0
            }
        }
    }
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "panfrost"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        format!(
            "panfrost:gpu={}:kernel={}:afbc_features={}",
            self.gpu.name, self.kernel, self.afbc_features
        )
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn desc(&self) -> &'static str {
        "panfrost DRM"
    }

    fn driver_version(&self) -> DriverVersion {
        self.kernel.panfrost()
    }

    fn uevent_driver(&self) -> &'static str {
        "panfrost"
    }

    fn platform(&self) -> Option<&'static Platform> {
        self.gpu.platform.as_ref()
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        ioctls::PANFROST
    }

    fn syncobjs(&self) -> Option<&Syncobjs> {
        Some(&self.syncobjs)
    }

    fn master(&self) -> &Master {
        &self.master
    }

    fn check_feature(&self, feature: DriverFeature) -> bool {
        match feature {
            DriverFeature::Modeset => false,
            DriverFeature::SyncobjTimeline => false,
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
        let ret = unsafe {
            match func {
                Func::PanfrostIoctlSetLabelBo => self.set_label_bo(file, arg, user),
                Func::PanfrostIoctlMadvise => self.madvise(file, arg),
                Func::PanfrostIoctlGetParam => self.get_param(arg),
                Func::PanfrostIoctlCreateBo => self.create_bo(file, arg),
                Func::PanfrostIoctlMmapBo => self.mmap_bo(file, arg),
                Func::PanfrostIoctlGetBoOffset => self.get_bo_offset(file, arg),
                _ => return None,
            }
        };
        Some(ret)
    }

    fn export(&self, file: u64, handle: u32) -> Result<Export, i32> {
        self.buffers.file(file).export(handle, |_| Ok(()))
    }

    fn import(&self, file: u64, object: u32) -> Option<u32> {
        let handle = self.buffers.file(file).import(object)?;
        if self.open(file, handle).is_err() {
            unimplemented!("panfrost imports into a full address space");
        }
        Some(handle)
    }

    fn import_foreign(&self, _file: u64, _export: &Export) -> NewHandle {
        unimplemented!("importing foreign dma-bufs into panfrost");
    }

    fn close_buffer(&self, file: u64, handle: u32) -> bool {
        self.close(file, handle)
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
        self.buffers
            .for_each(|buffer| buffer.mappings.retain(|m| m.file != file));
        self.buffers.release_file(file);
        self.mmus.lock().remove(&file);
        self.syncobjs.release_file(file);
    }
}
