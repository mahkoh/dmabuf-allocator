use crate::Buffers;
use crate::DriverFeature;
use crate::DriverVersion;
use crate::Drm;
use crate::Export;
use crate::Kernel;
use crate::KernelVersion;
use crate::Master;
use crate::NewHandle;
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
use crate::uapi::lima::DRM_LIMA_PARAM_GP_VERSION;
use crate::uapi::lima::DRM_LIMA_PARAM_GPU_ID;
use crate::uapi::lima::DRM_LIMA_PARAM_GPU_ID_MALI400;
use crate::uapi::lima::DRM_LIMA_PARAM_GPU_ID_MALI450;
use crate::uapi::lima::DRM_LIMA_PARAM_NUM_PP;
use crate::uapi::lima::DRM_LIMA_PARAM_PP_VERSION;
use crate::uapi::lima::LIMA_BO_FLAG_HEAP;
use crate::uapi::lima::drm_lima_gem_create;
use crate::uapi::lima::drm_lima_gem_info;
use crate::uapi::lima::drm_lima_get_param;
use libc::EINVAL;
use libc::ENOENT;
use libc::ENOSPC;
use parking_lot::Mutex;
use std::collections::BTreeMap;

const LIMA_VA_RESERVE_START: u64 = 0x0FFF00000;
const LIMA_VA_RESERVE_END: u64 = 0x100000000;

pub struct Gpu {
    pub name: &'static str,
    gpu_id: u32,
    num_pp: u32,
    platform: Platform,
}

pub static GPUS: &[Gpu] = &[
    Gpu {
        name: "a64",
        gpu_id: DRM_LIMA_PARAM_GPU_ID_MALI400,
        num_pp: 2,
        platform: Platform {
            fullname: "/soc/gpu@1c40000",
            compatible: &["allwinner,sun50i-a64-mali", "arm,mali-400"],
        },
    },
    Gpu {
        name: "rk3328",
        gpu_id: DRM_LIMA_PARAM_GPU_ID_MALI450,
        num_pp: 2,
        platform: Platform {
            fullname: "/gpu@ff300000",
            compatible: &["rockchip,rk3328-mali", "arm,mali-450"],
        },
    },
];

struct Vm {
    mm: DrmMm,
    objects: BTreeMap<u32, u64>,
}

impl Vm {
    fn new(end: u64) -> Self {
        Self {
            mm: DrmMm::new(0, end),
            objects: BTreeMap::new(),
        }
    }

    fn insert(&mut self, object: u32, size: u64) -> Option<u64> {
        let start = self.mm.insert(&Request::new(size))?;
        self.objects.insert(object, start);
        Some(start)
    }

    fn remove(&mut self, object: u32) {
        if let Some(start) = self.objects.remove(&object) {
            self.mm.remove(start);
        }
    }
}

pub struct Device {
    gpu: &'static Gpu,
    kernel: KernelVersion,
    buffers: Buffers<()>,
    vms: Mutex<BTreeMap<u64, Vm>>,
    syncobjs: Syncobjs,
    master: Master,
}

impl Device {
    pub fn new(name: &str, kernel: KernelVersion) -> Self {
        let gpu = GPUS
            .iter()
            .find(|gpu| gpu.name == name)
            .unwrap_or_else(|| panic!("unknown lima GPU {name:?}"));
        Self {
            gpu,
            kernel,
            buffers: Buffers::default(),
            vms: Mutex::new(BTreeMap::new()),
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

    fn va_end(&self) -> u64 {
        match self.gpu.gpu_id {
            DRM_LIMA_PARAM_GPU_ID_MALI450 => LIMA_VA_RESERVE_START,
            _ => LIMA_VA_RESERVE_END,
        }
    }

    unsafe fn get_param(&self, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_lima_get_param>() };
        if args.pad != 0 {
            return -(EINVAL as i64);
        }
        args.value = match args.param {
            DRM_LIMA_PARAM_GPU_ID => self.gpu.gpu_id as u64,
            DRM_LIMA_PARAM_NUM_PP => self.gpu.num_pp as u64,
            DRM_LIMA_PARAM_GP_VERSION | DRM_LIMA_PARAM_PP_VERSION => {
                unimplemented!("lima param {}", args.param);
            }
            _ => return -(EINVAL as i64),
        };
        0
    }

    unsafe fn gem_create(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_lima_gem_create>() };
        if args.pad != 0 {
            return -(EINVAL as i64);
        }
        if args.flags & !LIMA_BO_FLAG_HEAP != 0 {
            return -(EINVAL as i64);
        }
        if args.size == 0 {
            return -(EINVAL as i64);
        }
        if args.flags & LIMA_BO_FLAG_HEAP != 0 {
            unimplemented!("lima heap buffers");
        }
        let size = (args.size as u64).next_multiple_of(PAGE_SIZE);
        let mut vms = self.vms.lock();
        let vm = vms.entry(file).or_insert_with(|| Vm::new(self.va_end()));
        let buffers = self.buffers.file(file);
        let created = buffers.create_with_offset((), size);
        if vm.insert(created.object, size).is_none() {
            buffers.close(created.handle);
            return -(ENOSPC as i64);
        }
        args.handle = created.handle;
        0
    }

    unsafe fn gem_info(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_lima_gem_info>() };
        let vms = self.vms.lock();
        let buffers = self.buffers.file(file);
        let Some(object) = buffers.object(args.handle) else {
            return -(ENOENT as i64);
        };
        let Some(va) = vms
            .get(&file)
            .and_then(|vm| vm.objects.get(&object).copied())
        else {
            unimplemented!("lima virtual addresses of imported buffers");
        };
        args.va = va as u32;
        args.offset = buffers.offset(args.handle).unwrap();
        0
    }
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "lima"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        format!("lima:gpu={}:kernel={}", self.gpu.name, self.kernel)
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn desc(&self) -> &'static str {
        "lima DRM"
    }

    fn driver_version(&self) -> DriverVersion {
        DriverVersion::LIMA
    }

    fn uevent_driver(&self) -> &'static str {
        "lima"
    }

    fn platform(&self) -> Option<&'static Platform> {
        Some(&self.gpu.platform)
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        ioctls::LIMA
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
                Func::LimaIoctlGetParam => self.get_param(arg),
                Func::LimaIoctlGemCreate => self.gem_create(file, arg),
                Func::LimaIoctlGemInfo => self.gem_info(file, arg),
                _ => return None,
            }
        };
        Some(ret)
    }

    fn export(&self, file: u64, handle: u32) -> Result<Export, i32> {
        self.buffers.file(file).export(handle, |()| Ok(()))
    }

    fn import(&self, file: u64, object: u32) -> Option<u32> {
        self.buffers.file(file).import(object)
    }

    fn import_foreign(&self, _file: u64, _export: &Export) -> NewHandle {
        unimplemented!("importing foreign dma-bufs into lima");
    }

    fn close_buffer(&self, file: u64, handle: u32) -> bool {
        let mut vms = self.vms.lock();
        let buffers = self.buffers.file(file);
        let Some(object) = buffers.object(handle) else {
            return false;
        };
        buffers.close(handle);
        if let Some(vm) = vms.get_mut(&file) {
            vm.remove(object);
        }
        true
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
        self.buffers.release_file(file);
        self.syncobjs.release_file(file);
    }
}
