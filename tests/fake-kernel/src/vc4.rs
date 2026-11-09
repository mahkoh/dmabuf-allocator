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
use crate::id_fields;
use crate::ioctls;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::uapi::drm_fourcc::DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED;
use crate::uapi::drm_fourcc::DRM_FORMAT_MOD_LINEAR;
use crate::uapi::vc4::DRM_VC4_PARAM_SUPPORTS_BRANCHES;
use crate::uapi::vc4::DRM_VC4_PARAM_SUPPORTS_ETC1;
use crate::uapi::vc4::DRM_VC4_PARAM_SUPPORTS_FIXED_RCL_ORDER;
use crate::uapi::vc4::DRM_VC4_PARAM_SUPPORTS_MADVISE;
use crate::uapi::vc4::DRM_VC4_PARAM_SUPPORTS_PERFMON;
use crate::uapi::vc4::DRM_VC4_PARAM_SUPPORTS_THREADED_FS;
use crate::uapi::vc4::DRM_VC4_PARAM_V3D_IDENT0;
use crate::uapi::vc4::DRM_VC4_PARAM_V3D_IDENT1;
use crate::uapi::vc4::DRM_VC4_PARAM_V3D_IDENT2;
use crate::uapi::vc4::drm_vc4_create_bo;
use crate::uapi::vc4::drm_vc4_get_param;
use crate::uapi::vc4::drm_vc4_get_tiling;
use crate::uapi::vc4::drm_vc4_set_tiling;
use libc::EINVAL;
use libc::ENOENT;

const V3D_EXPECTED_IDENT0: u32 =
    (2 << 24) | (b'V' as u32) | ((b'3' as u32) << 8) | ((b'D' as u32) << 16);
const V3D_IDENT1_VPM_SIZE_SHIFT: u32 = 28;
const V3D_IDENT1_NSEM_SHIFT: u32 = 16;
const V3D_IDENT1_TUPS_SHIFT: u32 = 12;
const V3D_IDENT1_QUPS_SHIFT: u32 = 8;
const V3D_IDENT1_NSLC_SHIFT: u32 = 4;
const V3D_IDENT1_REV_SHIFT: u32 = 0;

struct Ident1 {
    vpm_size: u32,
    nsem: u32,
    tups: u32,
    qups: u32,
    nslc: u32,
    rev: u32,
}

impl Ident1 {
    fn register(&self) -> u32 {
        self.vpm_size << V3D_IDENT1_VPM_SIZE_SHIFT
            | self.nsem << V3D_IDENT1_NSEM_SHIFT
            | self.tups << V3D_IDENT1_TUPS_SHIFT
            | self.qups << V3D_IDENT1_QUPS_SHIFT
            | self.nslc << V3D_IDENT1_NSLC_SHIFT
            | self.rev << V3D_IDENT1_REV_SHIFT
    }
}

pub struct Gpu {
    pub name: &'static str,
    ident1: Ident1,
    platform: Platform,
}

pub static GPUS: &[Gpu] = &[Gpu {
    name: "bcm2837",
    ident1: Ident1 {
        vpm_size: 0,
        nsem: 0,
        tups: 0,
        qups: 0,
        nslc: 0,
        rev: 1,
    },
    platform: Platform {
        fullname: "/gpu",
        compatible: &["brcm,bcm2835-vc4"],
    },
}];

struct Buffer {
    t_format: bool,
}

pub struct Device {
    gpu: &'static Gpu,
    kernel: KernelVersion,
    buffers: Buffers<Buffer>,
    syncobjs: Syncobjs,
    master: Master,
}

impl Device {
    pub fn new(name: &str, kernel: KernelVersion) -> Self {
        let gpu = GPUS
            .iter()
            .find(|gpu| gpu.name == name)
            .unwrap_or_else(|| panic!("unknown vc4 GPU {name:?}"));
        Self {
            gpu,
            kernel,
            buffers: Buffers::default(),
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
        let args = unsafe { &mut *arg.cast::<drm_vc4_get_param>() };
        if args.pad != 0 {
            return -(EINVAL as i64);
        }
        args.value = match args.param {
            DRM_VC4_PARAM_V3D_IDENT0 => V3D_EXPECTED_IDENT0 as u64,
            DRM_VC4_PARAM_V3D_IDENT1 => self.gpu.ident1.register() as u64,
            DRM_VC4_PARAM_V3D_IDENT2 => unimplemented!("vc4 V3D_IDENT2"),
            DRM_VC4_PARAM_SUPPORTS_BRANCHES
            | DRM_VC4_PARAM_SUPPORTS_ETC1
            | DRM_VC4_PARAM_SUPPORTS_THREADED_FS
            | DRM_VC4_PARAM_SUPPORTS_FIXED_RCL_ORDER
            | DRM_VC4_PARAM_SUPPORTS_MADVISE
            | DRM_VC4_PARAM_SUPPORTS_PERFMON => 1,
            _ => return -(EINVAL as i64),
        };
        0
    }

    unsafe fn create_bo(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_vc4_create_bo>() };
        let size = (args.size as u64).next_multiple_of(PAGE_SIZE);
        if size == 0 {
            return -(EINVAL as i64);
        }
        let buffer = Buffer { t_format: false };
        let buffers = self.buffers.file(file);
        args.handle = buffers.create_with_offset(buffer, size).handle;
        0
    }

    unsafe fn set_tiling(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_vc4_set_tiling>() };
        if args.flags != 0 {
            return -(EINVAL as i64);
        }
        let t_format = match args.modifier {
            DRM_FORMAT_MOD_LINEAR => false,
            DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED => true,
            _ => return -(EINVAL as i64),
        };
        let updated = self
            .buffers
            .file(file)
            .with(args.handle, |buffer| buffer.t_format = t_format);
        match updated {
            Some(()) => 0,
            None => -(ENOENT as i64),
        }
    }

    unsafe fn get_tiling(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_vc4_get_tiling>() };
        if args.flags != 0 || args.modifier != 0 {
            return -(EINVAL as i64);
        }
        let t_format = self
            .buffers
            .file(file)
            .with(args.handle, |buffer| buffer.t_format);
        match t_format {
            Some(true) => args.modifier = DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED,
            Some(false) => args.modifier = DRM_FORMAT_MOD_LINEAR,
            None => return -(ENOENT as i64),
        }
        0
    }
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "vc4"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        format!("vc4:gpu={}:kernel={}", self.gpu.name, self.kernel)
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn desc(&self) -> &'static str {
        "Broadcom VC4 graphics"
    }

    fn driver_version(&self) -> DriverVersion {
        DriverVersion::VC4
    }

    fn uevent_driver(&self) -> &'static str {
        "vc4-drm"
    }

    fn platform(&self) -> Option<&'static Platform> {
        Some(&self.gpu.platform)
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        ioctls::VC4
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
                Func::Vc4GetParamIoctl => self.get_param(arg),
                Func::Vc4CreateBoIoctl => self.create_bo(file, arg),
                Func::Vc4SetTilingIoctl => self.set_tiling(file, arg),
                Func::Vc4GetTilingIoctl => self.get_tiling(file, arg),
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
        unimplemented!("importing foreign dma-bufs into vc4");
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
