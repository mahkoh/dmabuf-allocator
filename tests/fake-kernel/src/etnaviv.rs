use crate::Buffers;
use crate::DriverFeature;
use crate::DriverVersion;
use crate::Drm;
use crate::Export;
use crate::Kernel;
use crate::KernelVersion;
use crate::Master;
use crate::NewHandle;
use crate::Prime;
use crate::Syncobjs;
use crate::UserMemory;
use crate::etnaviv_perfmon::DOMS_2D;
use crate::etnaviv_perfmon::DOMS_3D;
use crate::etnaviv_perfmon::DOMS_VG;
use crate::etnaviv_perfmon::PmDomain;
use crate::id_fields_with_optional;
use crate::ioctls;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::string::strscpy_pad;
use crate::uapi::etnaviv::ETNA_BO_CACHE_MASK;
use crate::uapi::etnaviv::ETNA_BO_CACHED;
use crate::uapi::etnaviv::ETNA_BO_FORCE_MMU;
use crate::uapi::etnaviv::ETNA_BO_UNCACHED;
use crate::uapi::etnaviv::ETNA_BO_WC;
use crate::uapi::etnaviv::ETNA_MAX_PIPES;
use crate::uapi::etnaviv::ETNA_PREP_NOSYNC;
use crate::uapi::etnaviv::ETNA_PREP_READ;
use crate::uapi::etnaviv::ETNA_PREP_WRITE;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_CUSTOMER_ID;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_ECO_ID;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_0;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_1;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_2;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_3;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_4;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_5;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_6;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_7;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_8;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_9;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_10;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_11;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_FEATURES_12;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_MODEL;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_NUM_VARYINGS;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_PIXEL_PIPES;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_PRODUCT_ID;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_REVISION;
use crate::uapi::etnaviv::ETNAVIV_PARAM_GPU_STREAM_COUNT;
use crate::uapi::etnaviv::ETNAVIV_PARAM_SOFTPIN_START_ADDR;
use crate::uapi::etnaviv::drm_etnaviv_gem_cpu_fini;
use crate::uapi::etnaviv::drm_etnaviv_gem_cpu_prep;
use crate::uapi::etnaviv::drm_etnaviv_gem_info;
use crate::uapi::etnaviv::drm_etnaviv_gem_new;
use crate::uapi::etnaviv::drm_etnaviv_param;
use crate::uapi::etnaviv::drm_etnaviv_pm_domain;
use crate::uapi::etnaviv::drm_etnaviv_pm_signal;
use libc::EINVAL;
use libc::ENOENT;
use libc::ENXIO;

const SZ_4K: u32 = 4096;
const ETNAVIV_SOFTPIN_START_ADDRESS: u64 = 4 << 20;
const CHIP_MINOR_FEATURES1_MMU_VERSION: u32 = 0x10000000;
const CHIP_FEATURES_PIPE_3D: u32 = 0x00000004;
const CHIP_FEATURES_PIPE_2D: u32 = 0x00000200;
const CHIP_FEATURES_PIPE_VG: u32 = 0x04000000;

macro_rules! features {
    ($($field:ident = $param:ident),* $(,)?) => {
        #[derive(Copy, Clone)]
        struct Features {
            $($field: u32),*
        }

        impl Features {
            const NAMES: &[&str] = &[$(stringify!($field)),*];

            fn param(&self, param: u32) -> Option<u32> {
                match param {
                    $($param => Some(self.$field),)*
                    _ => None,
                }
            }

            fn word(&self, name: &str) -> u32 {
                match name {
                    $(stringify!($field) => self.$field,)*
                    _ => panic!("etnaviv has no feature word {name:?}"),
                }
            }

            fn set_word(&mut self, name: &str, value: u32) {
                match name {
                    $(stringify!($field) => self.$field = value,)*
                    _ => panic!("etnaviv has no feature word {name:?}"),
                }
            }
        }
    };
}

features! {
    features = ETNAVIV_PARAM_GPU_FEATURES_0,
    minor_features0 = ETNAVIV_PARAM_GPU_FEATURES_1,
    minor_features1 = ETNAVIV_PARAM_GPU_FEATURES_2,
    minor_features2 = ETNAVIV_PARAM_GPU_FEATURES_3,
    minor_features3 = ETNAVIV_PARAM_GPU_FEATURES_4,
    minor_features4 = ETNAVIV_PARAM_GPU_FEATURES_5,
    minor_features5 = ETNAVIV_PARAM_GPU_FEATURES_6,
    minor_features6 = ETNAVIV_PARAM_GPU_FEATURES_7,
    minor_features7 = ETNAVIV_PARAM_GPU_FEATURES_8,
    minor_features8 = ETNAVIV_PARAM_GPU_FEATURES_9,
    minor_features9 = ETNAVIV_PARAM_GPU_FEATURES_10,
    minor_features10 = ETNAVIV_PARAM_GPU_FEATURES_11,
    minor_features11 = ETNAVIV_PARAM_GPU_FEATURES_12,
}

#[derive(Copy, Clone)]
pub struct Gpu {
    pub name: &'static str,
    model: u32,
    revision: u32,
    product_id: u32,
    customer_id: u32,
    eco_id: u32,
    features: Features,
    pixel_pipes: u32,
}

pub static GPUS: &[Gpu] = &[
    Gpu {
        name: "gc400_4652",
        model: 0x400,
        revision: 0x4652,
        product_id: 0x70001,
        customer_id: 0x100,
        eco_id: 0x0,
        features: Features {
            features: 0xa0e9e004,
            minor_features0: 0xe1299fff,
            minor_features1: 0xbe13b219,
            minor_features2: 0xce110010,
            minor_features3: 0x08000001,
            minor_features4: 0x00020102,
            minor_features5: 0x00120000,
            minor_features6: 0,
            minor_features7: 0,
            minor_features8: 0,
            minor_features9: 0,
            minor_features10: 0,
            minor_features11: 0,
        },
        pixel_pipes: 1,
    },
    Gpu {
        name: "gc2000_5108",
        model: 0x2000,
        revision: 0x5108,
        product_id: 0x0,
        customer_id: 0x0,
        eco_id: 0x0,
        features: Features {
            features: 0x8001248d,
            minor_features0: 0x00711608,
            minor_features1: 0x20f00080,
            minor_features2: 0,
            minor_features3: 0,
            minor_features4: 0,
            minor_features5: 0,
            minor_features6: 0,
            minor_features7: 0x00000004,
            minor_features8: 0,
            minor_features9: 0,
            minor_features10: 0,
            minor_features11: 0,
        },
        pixel_pipes: 2,
    },
    Gpu {
        name: "gc7000_6204",
        model: 0x7000,
        revision: 0x6204,
        product_id: 0x70007,
        customer_id: 0x13,
        eco_id: 0x0,
        features: Features {
            features: 0xe0287c8d,
            minor_features0: 0xc1589eff,
            minor_features1: 0xfefbfad9,
            minor_features2: 0xeb9d4fbf,
            minor_features3: 0xedfffced,
            minor_features4: 0xdb0dafc7,
            minor_features5: 0x3b5ac333,
            minor_features6: 0xfcce6000,
            minor_features7: 0xfffbfa6f,
            minor_features8: 0x00e10ef3,
            minor_features9: 0x04c8003c,
            minor_features10: 0x00004060,
            minor_features11: 0x00000024,
        },
        pixel_pipes: 1,
    },
    Gpu {
        name: "gc7000_6214",
        model: 0x7000,
        revision: 0x6214,
        product_id: 0x70002,
        customer_id: 0x30,
        eco_id: 0x0,
        features: Features {
            features: 0xe0287cad,
            minor_features0: 0xc1799eff,
            minor_features1: 0xfefbfad9,
            minor_features2: 0xeb9d4fbf,
            minor_features3: 0xedfffced,
            minor_features4: 0xdb0dafc7,
            minor_features5: 0xbb5ac333,
            minor_features6: 0xfc8ee200,
            minor_features7: 0x03fbfa6f,
            minor_features8: 0x00ef0ef0,
            minor_features9: 0x0edbf03c,
            minor_features10: 0x90044250,
            minor_features11: 0x00000024,
        },
        pixel_pipes: 2,
    },
];

const MODEL: &str = "model";

fn parse_u32(key: &str, value: &str) -> u32 {
    let parsed = match value.strip_prefix("0x") {
        Some(hex) => u32::from_str_radix(hex, 16),
        None => value.parse(),
    };
    parsed.unwrap_or_else(|_| panic!("invalid etnaviv {key} {value:?}"))
}

pub struct Device {
    base: &'static Gpu,
    gpu: Gpu,
    kernel: KernelVersion,
    buffers: Buffers<()>,
    master: Master,
}

impl Device {
    pub fn new(name: &str, kernel: KernelVersion) -> Self {
        let base = GPUS
            .iter()
            .find(|gpu| gpu.name == name)
            .unwrap_or_else(|| panic!("unknown etnaviv GPU {name:?}"));
        Self {
            base,
            gpu: *base,
            kernel,
            buffers: Buffers::default(),
            master: Master::default(),
        }
    }

    pub fn from_id(fields: &str) -> Self {
        let mut overrides = vec![MODEL];
        overrides.extend(Features::NAMES);
        let values = id_fields_with_optional(fields, &["gpu", "kernel"], &overrides);
        let mut device = Self::new(
            values.get("gpu"),
            KernelVersion::parse(values.get("kernel")),
        );
        if let Some(model) = values.find(MODEL) {
            device.gpu.model = parse_u32(MODEL, model);
        }
        for name in Features::NAMES {
            if let Some(value) = values.find(name) {
                device.gpu.features.set_word(name, parse_u32(name, value));
            }
        }
        device
    }

    unsafe fn get_param(&self, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_etnaviv_param>() };
        if args.pipe >= ETNA_MAX_PIPES {
            return -(EINVAL as i64);
        }
        if args.pipe != 0 {
            return -(ENXIO as i64);
        }
        let gpu = &self.gpu;
        if let Some(word) = gpu.features.param(args.param) {
            args.value = word as u64;
            return 0;
        }
        args.value = match args.param {
            ETNAVIV_PARAM_GPU_MODEL => gpu.model as u64,
            ETNAVIV_PARAM_GPU_REVISION => gpu.revision as u64,
            ETNAVIV_PARAM_GPU_PIXEL_PIPES => gpu.pixel_pipes as u64,
            ETNAVIV_PARAM_GPU_PRODUCT_ID => gpu.product_id as u64,
            ETNAVIV_PARAM_GPU_CUSTOMER_ID => gpu.customer_id as u64,
            ETNAVIV_PARAM_GPU_ECO_ID => gpu.eco_id as u64,
            ETNAVIV_PARAM_SOFTPIN_START_ADDR => {
                match gpu.features.minor_features1 & CHIP_MINOR_FEATURES1_MMU_VERSION {
                    0 => !0,
                    _ => ETNAVIV_SOFTPIN_START_ADDRESS,
                }
            }
            ETNAVIV_PARAM_GPU_STREAM_COUNT..=ETNAVIV_PARAM_GPU_NUM_VARYINGS => {
                unimplemented!("etnaviv param {}", args.param);
            }
            _ => return -(EINVAL as i64),
        };
        0
    }

    unsafe fn gem_new(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_etnaviv_gem_new>() };
        if args.flags & !(ETNA_BO_CACHED | ETNA_BO_WC | ETNA_BO_UNCACHED | ETNA_BO_FORCE_MMU) != 0 {
            return -(EINVAL as i64);
        }
        let size = args.size as u32;
        match args.flags & ETNA_BO_CACHE_MASK {
            ETNA_BO_UNCACHED | ETNA_BO_CACHED | ETNA_BO_WC => {}
            _ => return -(EINVAL as i64),
        }
        let size = size.wrapping_add(SZ_4K - 1) & !(SZ_4K - 1);
        if size == 0 {
            unimplemented!("zero-sized etnaviv buffers");
        }
        args.handle = self.buffers.file(file).create((), size as u64).handle;
        0
    }

    unsafe fn gem_info(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_etnaviv_gem_info>() };
        if args.pad != 0 {
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

    unsafe fn gem_cpu_prep(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_etnaviv_gem_cpu_prep>() };
        if args.op & !(ETNA_PREP_READ | ETNA_PREP_WRITE | ETNA_PREP_NOSYNC) != 0 {
            return -(EINVAL as i64);
        }
        match self.buffers.file(file).with(args.handle, |_| ()) {
            Some(()) => 0,
            None => -(ENOENT as i64),
        }
    }

    unsafe fn gem_cpu_fini(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_etnaviv_gem_cpu_fini>() };
        if args.flags != 0 {
            return -(EINVAL as i64);
        }
        match self.buffers.file(file).with(args.handle, |_| ()) {
            Some(()) => 0,
            None => -(ENOENT as i64),
        }
    }

    fn num_pm_domains(&self) -> u32 {
        DOMS_META
            .iter()
            .filter(|meta| self.gpu.features.features & meta.feature != 0)
            .map(|meta| meta.domains.len() as u32)
            .sum()
    }

    fn pm_domain(&self, index: u32) -> Option<&'static PmDomain> {
        let mut domain = None;
        let mut offset = 0u32;
        for meta in &DOMS_META {
            if self.gpu.features.features & meta.feature == 0 {
                continue;
            }
            let nr_domains = meta.domains.len() as u32;
            if index.wrapping_sub(offset) >= nr_domains {
                offset += nr_domains;
                continue;
            }
            domain = Some(&meta.domains[index.wrapping_sub(offset) as usize]);
        }
        domain
    }

    unsafe fn pm_query_dom(&self, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_etnaviv_pm_domain>() };
        if args.pipe >= ETNA_MAX_PIPES {
            return -(EINVAL as i64);
        }
        if args.pipe != 0 {
            return -(ENXIO as i64);
        }
        let nr_domains = self.num_pm_domains();
        if args.iter as u32 >= nr_domains {
            return -(EINVAL as i64);
        }
        let Some(dom) = self.pm_domain(args.iter as u32) else {
            return -(EINVAL as i64);
        };
        args.id = args.iter;
        args.nr_signals = dom.signals.len() as u16;
        strscpy_pad(&mut args.name, dom.name);
        args.iter += 1;
        if args.iter as u32 == nr_domains {
            args.iter = 0xff;
        }
        0
    }

    unsafe fn pm_query_sig(&self, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_etnaviv_pm_signal>() };
        if args.pipe >= ETNA_MAX_PIPES {
            return -(EINVAL as i64);
        }
        if args.pipe != 0 {
            return -(ENXIO as i64);
        }
        let nr_domains = self.num_pm_domains();
        if args.domain as u32 >= nr_domains {
            return -(EINVAL as i64);
        }
        let Some(dom) = self.pm_domain(args.domain as u32) else {
            return -(EINVAL as i64);
        };
        if args.iter as usize >= dom.signals.len() {
            return -(EINVAL as i64);
        }
        let sig = dom.signals[args.iter as usize];
        args.id = args.iter;
        strscpy_pad(&mut args.name, sig);
        args.iter += 1;
        if args.iter as usize == dom.signals.len() {
            args.iter = 0xffff;
        }
        0
    }
}

struct PmDomainMeta {
    feature: u32,
    domains: &'static [PmDomain],
}

static DOMS_META: [PmDomainMeta; 3] = [
    PmDomainMeta {
        feature: CHIP_FEATURES_PIPE_3D,
        domains: DOMS_3D,
    },
    PmDomainMeta {
        feature: CHIP_FEATURES_PIPE_2D,
        domains: DOMS_2D,
    },
    PmDomainMeta {
        feature: CHIP_FEATURES_PIPE_VG,
        domains: DOMS_VG,
    },
];

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "etnaviv"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        let mut id = format!("etnaviv:gpu={}:kernel={}", self.gpu.name, self.kernel);
        if self.gpu.model != self.base.model {
            id.push_str(&format!(":{MODEL}={:#x}", self.gpu.model));
        }
        for name in Features::NAMES {
            let word = self.gpu.features.word(name);
            if word != self.base.features.word(name) {
                id.push_str(&format!(":{name}={word:#x}"));
            }
        }
        id
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn desc(&self) -> &'static str {
        "etnaviv DRM"
    }

    fn driver_version(&self) -> DriverVersion {
        DriverVersion::ETNAVIV
    }

    fn uevent_driver(&self) -> &'static str {
        "etnaviv"
    }

    fn virtual_platform(&self) -> Option<&'static str> {
        Some("etnaviv")
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        ioctls::ETNAVIV
    }

    fn syncobjs(&self) -> Option<&Syncobjs> {
        None
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
                Func::EtnavivIoctlGetParam => self.get_param(arg),
                Func::EtnavivIoctlGemNew => self.gem_new(file, arg),
                Func::EtnavivIoctlGemInfo => self.gem_info(file, arg),
                Func::EtnavivIoctlGemCpuPrep => self.gem_cpu_prep(file, arg),
                Func::EtnavivIoctlGemCpuFini => self.gem_cpu_fini(file, arg),
                Func::EtnavivIoctlPmQueryDom => self.pm_query_dom(arg),
                Func::EtnavivIoctlPmQuerySig => self.pm_query_sig(arg),
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
        unimplemented!("importing foreign dma-bufs into etnaviv");
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
    }
}
