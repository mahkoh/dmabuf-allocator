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
use crate::PCI_VENDOR_ID_NVIDIA;
use crate::Pci;
use crate::Prime;
use crate::Syncobjs;
use crate::UserMemory;
use crate::errno;
use crate::id_fields;
use crate::ioctls;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::uapi::nvidia_drm::drm_nvidia_gem_alloc_nvkms_memory_params;
use crate::uapi::nvidia_drm::drm_nvidia_get_dev_info_params;
use crate::uapi::nvidia_drm::drm_nvidia_get_drm_file_unique_id_params;
use libc::EINVAL;

#[derive(Copy, Clone)]
struct Buffer {
    size: u64,
}

pub struct Device {
    pci_id: u16,
    generic_page_kind: u32,
    page_kind_generation: u32,
    sector_layout: u32,
    kernel: KernelVersion,
    buffers: Buffers<Buffer>,
    syncobjs: Syncobjs,
    master: Master,
}

fn parse_number(text: &str) -> u32 {
    let parsed = match text.strip_prefix("0x") {
        Some(hex) => u32::from_str_radix(hex, 16),
        None => text.parse(),
    };
    parsed.unwrap_or_else(|_| panic!("invalid number {text:?}"))
}

impl Device {
    pub fn new(
        pci_id: u16,
        generic_page_kind: u32,
        page_kind_generation: u32,
        sector_layout: u32,
        kernel: KernelVersion,
    ) -> Self {
        Self {
            pci_id,
            generic_page_kind,
            page_kind_generation,
            sector_layout,
            kernel,
            buffers: Buffers::default(),
            syncobjs: Syncobjs::default(),
            master: Master::default(),
        }
    }

    pub fn from_id(fields: &str) -> Self {
        let values = id_fields(
            fields,
            &[
                "pci_id",
                "page_kind",
                "generation",
                "sector_layout",
                "kernel",
            ],
        );
        let pci_id = parse_number(values.get("pci_id"));
        Self::new(
            u16::try_from(pci_id).unwrap_or_else(|_| panic!("invalid PCI id {pci_id:#x}")),
            parse_number(values.get("page_kind")),
            parse_number(values.get("generation")),
            parse_number(values.get("sector_layout")),
            KernelVersion::parse(values.get("kernel")),
        )
    }

    unsafe fn get_dev_info(&self, arg: *mut u8) -> i64 {
        let params = unsafe { &mut *arg.cast::<drm_nvidia_get_dev_info_params>() };
        params.gpu_id = 0;
        params.mig_device = 0;
        params.primary_index = 0;
        params.supports_alloc = 1;
        params.generic_page_kind = self.generic_page_kind;
        params.page_kind_generation = self.page_kind_generation;
        params.sector_layout = self.sector_layout;
        params.supports_sync_fd = 0;
        params.supports_semsurf = 0;
        0
    }

    unsafe fn get_drm_file_unique_id(&self, file: u64, arg: *mut u8) -> i64 {
        let params = unsafe { &mut *arg.cast::<drm_nvidia_get_drm_file_unique_id_params>() };
        params.id = file;
        0
    }

    unsafe fn gem_alloc_nvkms_memory(&self, file: u64, arg: *mut u8) -> i64 {
        let params = unsafe { &mut *arg.cast::<drm_nvidia_gem_alloc_nvkms_memory_params>() };
        if params.__pad0 != 0 || params.__pad1 != 0 {
            return errno(EINVAL);
        }
        if params.memory_size == 0 {
            unimplemented!("NVKMS allocations of size 0");
        }
        params.compressible = (params.block_linear != 0 && params.compressible != 0) as u8;
        if !params.memory_size.is_multiple_of(PAGE_SIZE) {
            return errno(EINVAL);
        }
        let buffer = Buffer {
            size: params.memory_size,
        };
        params.handle = self.buffers.file(file).create(buffer, buffer.size).handle;
        0
    }
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "nvidia-drm"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        format!(
            "nvidia-drm:pci_id={:#06x}:page_kind={:#x}:generation={}:sector_layout={}:kernel={}",
            self.pci_id,
            self.generic_page_kind,
            self.page_kind_generation,
            self.sector_layout,
            self.kernel
        )
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn desc(&self) -> &'static str {
        "NVIDIA DRM driver"
    }

    fn driver_version(&self) -> DriverVersion {
        DriverVersion::NVIDIA_DRM
    }

    fn uevent_driver(&self) -> &'static str {
        "nvidia"
    }

    fn pci(&self) -> Option<Pci> {
        Some(Pci {
            vendor: PCI_VENDOR_ID_NVIDIA,
            device: self.pci_id,
        })
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        ioctls::NVIDIA_DRM
    }

    fn syncobjs(&self) -> Option<&Syncobjs> {
        Some(&self.syncobjs)
    }

    fn master(&self) -> &Master {
        &self.master
    }

    fn check_feature(&self, feature: DriverFeature) -> bool {
        match feature {
            DriverFeature::Modeset => unimplemented!("the nvidia-drm modeset parameter"),
            DriverFeature::SyncobjTimeline => true,
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
                Func::NvDrmGetDevInfoIoctl => self.get_dev_info(arg),
                Func::NvDrmGemAllocNvkmsMemoryIoctl => self.gem_alloc_nvkms_memory(file, arg),
                Func::NvDrmGetDrmFileUniqueIdIoctl => self.get_drm_file_unique_id(file, arg),
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
        unimplemented!("importing dma-bufs into nvidia-drm");
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
