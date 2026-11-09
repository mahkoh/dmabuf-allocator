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
use crate::dumb::create_dumb;
use crate::dumb::shmem_dumb_size;
use crate::id_fields;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::uapi::drm::drm_mode_create_dumb;
use crate::uapi::drm::drm_mode_destroy_dumb;
use libc::EINVAL;

pub struct Device {
    kernel: KernelVersion,
    buffers: Buffers<()>,
    master: Master,
}

impl Device {
    pub fn new(kernel: KernelVersion) -> Self {
        Self {
            kernel,
            buffers: Buffers::default(),
            master: Master::default(),
        }
    }

    pub fn from_id(fields: &str) -> Self {
        let values = id_fields(fields, &["kernel"]);
        Self::new(KernelVersion::parse(values.get("kernel")))
    }

    unsafe fn create_dumb(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_mode_create_dumb>() };
        create_dumb(self.kernel, args, |args| {
            shmem_dumb_size(self.kernel, args)?;
            let buffers = self.buffers.file(file);
            args.handle = buffers.create_with_offset((), args.size).handle;
            Ok(())
        })
    }

    unsafe fn destroy_dumb(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &*arg.cast::<drm_mode_destroy_dumb>() };
        match self.buffers.file(file).close(args.handle) {
            true => 0,
            false => -(EINVAL as i64),
        }
    }
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "vkms"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn render_node(&self) -> bool {
        false
    }

    fn id(&self) -> String {
        format!("vkms:kernel={}", self.kernel)
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn desc(&self) -> &'static str {
        "Virtual Kernel Mode Setting"
    }

    fn driver_version(&self) -> DriverVersion {
        DriverVersion::VKMS
    }

    fn uevent_driver(&self) -> &'static str {
        match self.faux() {
            true => "faux_driver",
            false => unimplemented!("vkms platform devices have no driver"),
        }
    }

    fn virtual_platform(&self) -> Option<&'static str> {
        match self.faux() {
            true => None,
            false => Some("vkms"),
        }
    }

    fn faux(&self) -> bool {
        self.kernel >= KernelVersion::V6_17
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        &[]
    }

    fn syncobjs(&self) -> Option<&Syncobjs> {
        None
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
                Func::DrmModeCreateDumbIoctl => self.create_dumb(file, arg),
                Func::DrmModeDestroyDumbIoctl => self.destroy_dumb(file, arg),
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
        unimplemented!("importing dma-bufs into vkms");
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
