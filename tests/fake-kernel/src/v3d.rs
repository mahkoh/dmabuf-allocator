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
use crate::Syncobjs;
use crate::UserMemory;
use crate::drm_mm::DrmMm;
use crate::drm_mm::Request;
use crate::id_fields;
use crate::ioctls;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::string::strscpy;
use crate::uapi::v3d::DRM_V3D_PARAM_CONTEXT_RESET_COUNTER;
use crate::uapi::v3d::DRM_V3D_PARAM_GLOBAL_RESET_COUNTER;
use crate::uapi::v3d::DRM_V3D_PARAM_MAX_PERF_COUNTERS;
use crate::uapi::v3d::DRM_V3D_PARAM_SUPPORTS_CACHE_FLUSH;
use crate::uapi::v3d::DRM_V3D_PARAM_SUPPORTS_CPU_QUEUE;
use crate::uapi::v3d::DRM_V3D_PARAM_SUPPORTS_CSD;
use crate::uapi::v3d::DRM_V3D_PARAM_SUPPORTS_MULTISYNC_EXT;
use crate::uapi::v3d::DRM_V3D_PARAM_SUPPORTS_PERFMON;
use crate::uapi::v3d::DRM_V3D_PARAM_SUPPORTS_SUPER_PAGES;
use crate::uapi::v3d::DRM_V3D_PARAM_SUPPORTS_TFU;
use crate::uapi::v3d::DRM_V3D_PARAM_V3D_CORE0_IDENT0;
use crate::uapi::v3d::DRM_V3D_PARAM_V3D_CORE0_IDENT1;
use crate::uapi::v3d::DRM_V3D_PARAM_V3D_CORE0_IDENT2;
use crate::uapi::v3d::DRM_V3D_PARAM_V3D_HUB_IDENT1;
use crate::uapi::v3d::DRM_V3D_PARAM_V3D_HUB_IDENT2;
use crate::uapi::v3d::DRM_V3D_PARAM_V3D_HUB_IDENT3;
use crate::uapi::v3d::DRM_V3D_PARAM_V3D_UIFCFG;
use crate::uapi::v3d::drm_v3d_create_bo;
use crate::uapi::v3d::drm_v3d_get_bo_offset;
use crate::uapi::v3d::drm_v3d_get_param;
use crate::uapi::v3d::drm_v3d_mmap_bo;
use crate::uapi::v3d::drm_v3d_perfmon_get_counter;
use crate::v3d_counters;
use crate::v3d_counters::Counter;
use libc::EINVAL;
use libc::ENOENT;
use libc::ENOSPC;
use parking_lot::Mutex;
use std::sync::Arc;

const PT_SIZE: u64 = 4096 * 1024;
const V3D_GEN_41: u32 = 41;
const V3D_GEN_42: u32 = 42;
const V3D_GEN_71: u32 = 71;

pub struct Gpu {
    pub name: &'static str,
    ver: u32,
    uifcfg: u32,
    hub_ident1: u32,
    hub_ident2: u32,
    hub_ident3: u32,
    core0_ident0: u32,
    core0_ident1: u32,
    core0_ident2: u32,
    platform: Platform,
}

impl Gpu {
    fn counters(&self) -> &'static [Counter] {
        if self.ver >= V3D_GEN_71 {
            &v3d_counters::V71
        } else if self.ver >= V3D_GEN_42 {
            &v3d_counters::V42
        } else {
            unimplemented!("v3d {} performance counters", self.ver);
        }
    }
}

pub static GPUS: &[Gpu] = &[
    Gpu {
        name: "bcm2711",
        ver: 42,
        uifcfg: 0x00000045,
        hub_ident1: 0x000e1124,
        hub_ident2: 0x00000100,
        hub_ident3: 0x00000e00,
        core0_ident0: 0x04443356,
        core0_ident1: 0x81001422,
        core0_ident2: 0x40078121,
        platform: Platform {
            fullname: "/scb-bus@fc000000/gpu@7ec00000",
            compatible: &["brcm,2711-v3d"],
        },
    },
    Gpu {
        name: "bcm2712",
        ver: 71,
        uifcfg: 0x00000045,
        hub_ident1: 0x00081117,
        hub_ident2: 0x00001900,
        hub_ident3: 0x00000700,
        core0_ident0: 0x07443356,
        core0_ident1: 0x81001441,
        core0_ident2: 0xc0078101,
        platform: Platform {
            fullname: "/axi/gpu@1002000000",
            compatible: &["brcm,2712-v3d"],
        },
    },
];

struct Node {
    mm: Arc<Mutex<DrmMm>>,
    first_page: u64,
}

impl Node {
    fn insert(mm: &Arc<Mutex<DrmMm>>, pages: u64) -> Option<Self> {
        let first_page = mm.lock().insert(&Request::new(pages))?;
        Some(Self {
            mm: mm.clone(),
            first_page,
        })
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.mm.lock().remove(self.first_page);
    }
}

struct Buffer {
    node: Node,
}

pub struct Device {
    gpu: &'static Gpu,
    kernel: KernelVersion,
    buffers: Buffers<Buffer>,
    mm: Arc<Mutex<DrmMm>>,
    syncobjs: Syncobjs,
    master: Master,
}

impl Device {
    pub fn new(name: &str, kernel: KernelVersion) -> Self {
        let gpu = GPUS
            .iter()
            .find(|gpu| gpu.name == name)
            .unwrap_or_else(|| panic!("unknown v3d GPU {name:?}"));
        Self {
            gpu,
            kernel,
            buffers: Buffers::default(),
            mm: Arc::new(Mutex::new(DrmMm::new(1, PT_SIZE / 4 - 1))),
            syncobjs: Syncobjs::default(),
            master: Master::default(),
        }
    }

    pub fn from_id(fields: &str) -> Self {
        let values = id_fields(fields, &["gpu", "kernel"]);
        Self::new(
            values.get("gpu"),
            KernelVersion::parse(values.get("kernel")),
        )
    }

    unsafe fn get_param(&self, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_v3d_get_param>() };
        if args.pad != 0 {
            return -(EINVAL as i64);
        }
        let gpu = self.gpu;
        let register = match args.param {
            DRM_V3D_PARAM_V3D_UIFCFG => Some(gpu.uifcfg),
            DRM_V3D_PARAM_V3D_HUB_IDENT1 => Some(gpu.hub_ident1),
            DRM_V3D_PARAM_V3D_HUB_IDENT2 => Some(gpu.hub_ident2),
            DRM_V3D_PARAM_V3D_HUB_IDENT3 => Some(gpu.hub_ident3),
            DRM_V3D_PARAM_V3D_CORE0_IDENT0 => Some(gpu.core0_ident0),
            DRM_V3D_PARAM_V3D_CORE0_IDENT1 => Some(gpu.core0_ident1),
            DRM_V3D_PARAM_V3D_CORE0_IDENT2 => Some(gpu.core0_ident2),
            _ => None,
        };
        if let Some(register) = register {
            if args.value != 0 {
                return -(EINVAL as i64);
            }
            args.value = register as u64;
            return 0;
        }
        args.value = match args.param {
            DRM_V3D_PARAM_SUPPORTS_TFU => 1,
            DRM_V3D_PARAM_SUPPORTS_CSD => (gpu.ver >= V3D_GEN_41) as u64,
            DRM_V3D_PARAM_SUPPORTS_CACHE_FLUSH => 1,
            DRM_V3D_PARAM_SUPPORTS_PERFMON => (gpu.ver >= V3D_GEN_41) as u64,
            DRM_V3D_PARAM_SUPPORTS_MULTISYNC_EXT => 1,
            DRM_V3D_PARAM_SUPPORTS_CPU_QUEUE => 1,
            DRM_V3D_PARAM_MAX_PERF_COUNTERS => gpu.counters().len() as u64,
            DRM_V3D_PARAM_SUPPORTS_SUPER_PAGES => 0,
            DRM_V3D_PARAM_GLOBAL_RESET_COUNTER | DRM_V3D_PARAM_CONTEXT_RESET_COUNTER
                if self.kernel >= KernelVersion::V6_18 =>
            {
                0
            }
            _ => return -(EINVAL as i64),
        };
        0
    }

    unsafe fn create_bo(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_v3d_create_bo>() };
        if args.flags != 0 {
            return -(EINVAL as i64);
        }
        if args.size == 0 {
            unimplemented!("zero-sized v3d buffers");
        }
        let size = (args.size as u64).next_multiple_of(PAGE_SIZE);
        let Some(node) = Node::insert(&self.mm, size >> PAGE_SHIFT) else {
            return -(ENOSPC as i64);
        };
        args.offset = (node.first_page << PAGE_SHIFT) as u32;
        let buffers = self.buffers.file(file);
        args.handle = buffers.create_with_offset(Buffer { node }, size).handle;
        0
    }

    unsafe fn mmap_bo(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_v3d_mmap_bo>() };
        if args.flags != 0 {
            return -(EINVAL as i64);
        }
        match self.buffers.file(file).offset(args.handle) {
            Some(offset) => {
                args.offset = offset;
                0
            }
            None => -(ENOENT as i64),
        }
    }

    unsafe fn get_bo_offset(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_v3d_get_bo_offset>() };
        match self
            .buffers
            .file(file)
            .with(args.handle, |buffer| buffer.node.first_page)
        {
            Some(page) => {
                args.offset = (page << PAGE_SHIFT) as u32;
                0
            }
            None => -(ENOENT as i64),
        }
    }

    unsafe fn perfmon_get_counter(&self, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_v3d_perfmon_get_counter>() };
        if args.reserved.iter().any(|&byte| byte != 0) {
            return -(EINVAL as i64);
        }
        let Some(counter) = self.gpu.counters().get(args.counter as usize) else {
            return -(EINVAL as i64);
        };
        strscpy(&mut args.name, counter.name);
        strscpy(&mut args.category, counter.category);
        strscpy(&mut args.description, counter.description);
        0
    }
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "v3d"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        format!("v3d:gpu={}:kernel={}", self.gpu.name, self.kernel)
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn desc(&self) -> &'static str {
        "Broadcom V3D graphics"
    }

    fn driver_version(&self) -> DriverVersion {
        DriverVersion::V3D
    }

    fn uevent_driver(&self) -> &'static str {
        "v3d"
    }

    fn platform(&self) -> Option<&'static Platform> {
        Some(&self.gpu.platform)
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        ioctls::V3D
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
        _user: &dyn UserMemory,
        _prime: &mut dyn Prime,
    ) -> Option<i64> {
        let ret = unsafe {
            match func {
                Func::V3dGetParamIoctl => self.get_param(arg),
                Func::V3dCreateBoIoctl => self.create_bo(file, arg),
                Func::V3dMmapBoIoctl => self.mmap_bo(file, arg),
                Func::V3dGetBoOffsetIoctl => self.get_bo_offset(file, arg),
                Func::V3dPerfmonGetCounterIoctl => self.perfmon_get_counter(arg),
                _ => return None,
            }
        };
        Some(ret)
    }

    fn export(&self, file: u64, handle: u32) -> Result<Export, i32> {
        self.buffers.file(file).export(handle, |_| Ok(()))
    }

    fn import(&self, file: u64, object: u32) -> Option<u32> {
        self.buffers.file(file).import(object)
    }

    fn import_foreign(&self, _file: u64, _export: &Export) -> NewHandle {
        unimplemented!("importing foreign dma-bufs into v3d");
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
        self.buffers.release_file(file);
        self.syncobjs.release_file(file);
    }
}
