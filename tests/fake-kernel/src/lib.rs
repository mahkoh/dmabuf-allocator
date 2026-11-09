#![allow(clippy::missing_safety_doc)]

use crate::ioctls::DRM_AUTH;
use crate::ioctls::DRM_MASTER;
use crate::ioctls::DRM_RENDER_ALLOW;
use crate::ioctls::DRM_ROOT_ONLY;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::uapi::drm::DRM_CAP_ADDFB2_MODIFIERS;
use crate::uapi::drm::DRM_CAP_PRIME;
use crate::uapi::drm::DRM_CAP_SYNCOBJ;
use crate::uapi::drm::DRM_CAP_SYNCOBJ_TIMELINE;
use crate::uapi::drm::DRM_CAP_TIMESTAMP_MONOTONIC;
use crate::uapi::drm::DRM_CLOEXEC;
use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_COMMAND_END;
use crate::uapi::drm::DRM_IOCTL_BASE;
use crate::uapi::drm::DRM_PRIME_CAP_EXPORT;
use crate::uapi::drm::DRM_PRIME_CAP_IMPORT;
use crate::uapi::drm::DRM_RDWR;
use crate::uapi::drm::drm_gem_close;
use crate::uapi::drm::drm_get_cap;
use crate::uapi::drm::drm_prime_handle;
use crate::uapi::drm::drm_version;
use crate::uapi::ioctl::_IOC_NR;
use crate::uapi::ioctl::_IOC_SIZE;
use crate::uapi::ioctl::_IOC_TYPE;
use crate::uapi::ioctl::IOC_IN;
use crate::uapi::ioctl::IOC_OUT;
use libc::EACCES;
use libc::EFAULT;
use libc::EINVAL;
use libc::ENOENT;
use libc::ENOTTY;
use libc::EOPNOTSUPP;
use parking_lot::Mutex;

pub mod amd;
mod amdgpu_ids;
pub mod asahi;
mod bitfield;
mod buffers;
mod drm_mm;
mod dumb;
pub mod etnaviv;
mod etnaviv_perfmon;
mod fault;
mod handles;
pub mod intel;
pub mod ioctls;
pub mod lima;
pub mod msm;
pub mod nouveau;
pub mod nvidia_drm;
pub mod panfrost;
pub mod panthor;
mod string;
pub mod syncobj;
mod time;
pub mod uapi;
pub mod udmabuf;
mod user;
pub mod v3d;
mod v3d_counters;
pub mod vc4;
mod vdrm_host;
pub mod version;
mod virgl_caps;
mod virtio_amdgpu;
mod virtio_asahi;
pub mod virtio_gpu;
mod virtio_i915;
mod virtio_panfrost;
pub mod vkms;

pub use buffers::Buffers;
pub use buffers::Export;
pub use buffers::FileBuffers;
pub use buffers::NewHandle;
pub use fault::Fault;
pub use syncobj::Fence;
pub use syncobj::Syncobjs;
pub use user::AlignedBytes;
pub use user::LocalMemory;
pub use user::UserMemory;
pub use version::DriverVersion;
pub use version::KernelVersion;

pub const FILE_SIZE: libc::off_t = 1 << 46;

pub struct Platform {
    pub fullname: &'static str,
    pub compatible: &'static [&'static str],
}

pub struct Memfd {
    pub size: u64,
    pub seals: i32,
}

pub struct Pci {
    pub vendor: u16,
    pub device: u16,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DriverFeature {
    Modeset,
    SyncobjTimeline,
}

#[derive(Default)]
pub struct Master {
    file: Mutex<Option<u64>>,
}

impl Master {
    fn open(&self, file: u64) {
        let mut master = self.file.lock();
        if master.is_none() {
            *master = Some(file);
        }
    }

    fn release(&self, file: u64) {
        let mut master = self.file.lock();
        if *master == Some(file) {
            *master = None;
        }
    }

    fn is(&self, file: u64) -> bool {
        *self.file.lock() == Some(file)
    }
}

pub trait Kernel: Send + Sync {
    fn name(&self) -> &'static str;
    fn id(&self) -> String;
    fn kernel_version(&self) -> KernelVersion;
    fn render_node(&self) -> bool {
        true
    }
    fn drm(&self) -> Option<&dyn Drm>;
    fn misc_ioctl(
        &self,
        _cmd: u32,
        _arg: u64,
        _user: &dyn UserMemory,
        _prime: &mut dyn Prime,
    ) -> Option<i64> {
        None
    }
}

pub fn drm(kernel: &dyn Kernel) -> &dyn Drm {
    match kernel.drm() {
        Some(drm) => drm,
        None => panic!("{} is not a DRM device", kernel.id()),
    }
}

pub trait Drm: Kernel {
    fn desc(&self) -> &'static str;
    fn driver_version(&self) -> DriverVersion;
    fn uevent_driver(&self) -> &'static str;
    fn pci(&self) -> Option<Pci> {
        None
    }
    fn platform(&self) -> Option<&'static Platform> {
        None
    }
    fn virtual_platform(&self) -> Option<&'static str> {
        None
    }
    fn faux(&self) -> bool {
        false
    }
    fn virtio_device(&self) -> Option<u32> {
        None
    }
    fn resource_handle(&self, _file: u64, _res_id: u32) -> Option<u32> {
        None
    }
    fn context_type(&self, _file: u64) -> Option<u32> {
        None
    }
    fn amd_gfx_level(&self) -> Option<amd::devices::GfxLevel> {
        None
    }
    fn ioctls(&self) -> &'static [IoctlDesc];
    fn syncobjs(&self) -> Option<&Syncobjs>;
    fn master(&self) -> &Master;
    fn check_feature(&self, feature: DriverFeature) -> bool;
    fn fb_modifiers_not_supported(&self) -> bool {
        unimplemented!("{} does not model framebuffer modifiers", self.id());
    }
    fn defer_commands(&self) {}
    fn wait_fence(&self, _file: u64, _prime: &mut dyn Prime) {}
    fn unlocked_ioctl(
        &self,
        _file: u64,
        _cmd: u32,
        _arg: u64,
        _user: &dyn UserMemory,
    ) -> Option<i64> {
        None
    }
    unsafe fn ioctl(
        &self,
        file: u64,
        func: Func,
        arg: *mut u8,
        user: &dyn UserMemory,
        prime: &mut dyn Prime,
    ) -> Option<i64>;
    fn export(&self, file: u64, handle: u32) -> Result<Export, i32>;
    fn import(&self, file: u64, object: u32) -> Option<u32>;
    fn import_foreign(&self, file: u64, export: &Export) -> NewHandle;
    fn close_buffer(&self, file: u64, handle: u32) -> bool;
    fn object(&self, file: u64, handle: u32) -> Option<u32>;
    fn object_has_handles(&self, object: u32) -> bool;
    fn release_export(&self, object: u32);
    fn release_file(&self, file: u64);
    fn fail_host_command(&self, _fault: Fault) {
        unimplemented!("{} has no native context host", self.id());
    }
}

pub trait Prime {
    fn sync_file(&mut self) -> Result<i32, i32>;
    fn memfd(&mut self, _fd: i32) -> Option<Memfd> {
        unimplemented!("memfds");
    }
    fn dmabuf_fd(&mut self) -> i32 {
        unimplemented!("dma-bufs without a GEM object");
    }
    fn read_file(&mut self, offset: u64, data: &mut [u8]) -> Result<(), i32>;
    fn write_file(&mut self, offset: u64, data: &[u8]) -> Result<(), i32>;
    unsafe fn handle_to_fd(
        &mut self,
        kernel: &dyn Drm,
        file: u64,
        args: &mut drm_prime_handle,
    ) -> i64;
    unsafe fn fd_to_handle(
        &mut self,
        kernel: &dyn Drm,
        file: u64,
        args: &mut drm_prime_handle,
    ) -> i64;
}

pub fn from_id(id: &str) -> Box<dyn Kernel> {
    let Some((driver, rest)) = id.split_once(':') else {
        panic!("fake kernel id {id:?} has no driver");
    };
    match driver {
        "amdgpu" => Box::new(amd::kernel::Device::from_id(rest)),
        "intel" => Box::new(intel::kernel::Device::from_id(rest)),
        "nvidia-drm" => Box::new(nvidia_drm::kernel::Device::from_id(rest)),
        "nouveau" => Box::new(nouveau::kernel::Device::from_id(rest)),
        "msm" => Box::new(msm::kernel::Device::from_id(rest)),
        "vkms" => Box::new(vkms::Device::from_id(rest)),
        "asahi" => Box::new(asahi::Device::from_id(rest)),
        "panfrost" => Box::new(panfrost::Device::from_id(rest)),
        "panthor" => Box::new(panthor::Device::from_id(rest)),
        "v3d" => Box::new(v3d::Device::from_id(rest)),
        "etnaviv" => Box::new(etnaviv::Device::from_id(rest)),
        "lima" => Box::new(lima::Device::from_id(rest)),
        "vc4" => Box::new(vc4::Device::from_id(rest)),
        "virtio_gpu" => Box::new(virtio_gpu::Device::from_id(rest)),
        "udmabuf" => Box::new(udmabuf::Device::from_id(rest)),
        _ => panic!("unknown fake kernel {id}"),
    }
}

struct IdField<'a> {
    key: &'a str,
    value: &'a str,
}

pub struct IdFields<'a> {
    id: &'a str,
    fields: Vec<IdField<'a>>,
}

impl<'a> IdFields<'a> {
    pub fn get(&self, key: &str) -> &'a str {
        match self.find(key) {
            Some(value) => value,
            None => panic!("field {key:?} of {:?} was not requested", self.id),
        }
    }

    pub fn find(&self, key: &str) -> Option<&'a str> {
        self.fields
            .iter()
            .find(|field| field.key == key)
            .map(|field| field.value)
    }
}

pub(crate) const PAGE_SHIFT: u32 = 12;
pub(crate) const PAGE_SIZE: u64 = 1 << PAGE_SHIFT;
pub(crate) const SZ_4G: u64 = 4 << 30;
pub(crate) const PCI_VENDOR_ID_NVIDIA: u16 = 0x10de;

pub(crate) fn errno(errno: i32) -> i64 {
    -(errno as i64)
}

pub fn id_fields<'a>(id: &'a str, keys: &[&str]) -> IdFields<'a> {
    id_fields_with_optional(id, keys, &[])
}

pub fn id_fields_with_optional<'a>(
    id: &'a str,
    required: &[&str],
    optional: &[&str],
) -> IdFields<'a> {
    let mut fields: Vec<IdField<'a>> = Vec::new();
    for field in id.split(':') {
        let Some((key, value)) = field.split_once('=') else {
            panic!("field {field:?} of {id:?} has no key");
        };
        assert!(
            required.contains(&key) || optional.contains(&key),
            "unknown field {key:?} in {id:?}"
        );
        let duplicate = fields.iter().any(|field| field.key == key);
        assert!(!duplicate, "duplicate field {key:?} in {id:?}");
        fields.push(IdField { key, value });
    }
    for key in required {
        let present = fields.iter().any(|field| field.key == *key);
        assert!(present, "missing field {key:?} in {id:?}");
    }
    IdFields { id, fields }
}

pub fn id_bool(value: &str) -> bool {
    match value {
        "0" => false,
        "1" => true,
        _ => panic!("{value:?} is not 0 or 1"),
    }
}

fn lookup(kernel: &dyn Drm, nr: u32) -> Option<&'static IoctlDesc> {
    let table = match (DRM_COMMAND_BASE..DRM_COMMAND_END).contains(&nr) {
        true => kernel.ioctls(),
        false => ioctls::GENERIC,
    };
    let version = kernel.kernel_version();
    table
        .iter()
        .find(|desc| _IOC_NR(desc.cmd) == nr && desc.exists(version))
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Node {
    Primary,
    Render,
}

const CAPABLE_SYS_ADMIN: bool = false;

pub fn open(kernel: &dyn Kernel, file: u64, node: Node) {
    if let Some(drm) = kernel.drm()
        && node == Node::Primary
    {
        drm.master().open(file);
    }
}

pub fn release(kernel: &dyn Kernel, file: u64) {
    if let Some(drm) = kernel.drm() {
        drm.master().release(file);
        drm.release_file(file);
    }
}

fn permit(kernel: &dyn Drm, flags: u32, file: u64, node: Node) -> bool {
    let render_client = node == Node::Render;
    let master = kernel.master().is(file);
    let authenticated = CAPABLE_SYS_ADMIN || master;
    !(flags & DRM_ROOT_ONLY != 0 && !CAPABLE_SYS_ADMIN
        || flags & DRM_AUTH != 0 && !render_client && !authenticated
        || flags & DRM_MASTER != 0 && !master
        || flags & DRM_RENDER_ALLOW == 0 && render_client)
}

pub fn ioctl(
    kernel: &dyn Kernel,
    file: u64,
    node: Node,
    cmd: u32,
    arg: u64,
    user: &dyn UserMemory,
    prime: &mut dyn Prime,
) -> i64 {
    if let Some(ret) = kernel.misc_ioctl(cmd, arg, user, prime) {
        return ret;
    }
    let Some(kernel) = kernel.drm() else {
        return -(ENOTTY as i64);
    };
    if let Some(ret) = kernel.unlocked_ioctl(file, cmd, arg, user) {
        return ret;
    }
    if _IOC_TYPE(cmd) != DRM_IOCTL_BASE {
        return -(ENOTTY as i64);
    }
    let Some(desc) = lookup(kernel, _IOC_NR(cmd)) else {
        return -(EINVAL as i64);
    };
    let drv_size = _IOC_SIZE(desc.cmd) as usize;
    let mut in_size = _IOC_SIZE(cmd) as usize;
    let mut out_size = in_size;
    if cmd & desc.cmd & IOC_IN == 0 {
        in_size = 0;
    }
    if cmd & desc.cmd & IOC_OUT == 0 {
        out_size = 0;
    }
    let ksize = in_size.max(out_size).max(drv_size);
    let mut kdata = vec![0u64; ksize.div_ceil(8)];
    let bytes = unsafe { std::slice::from_raw_parts_mut(kdata.as_mut_ptr().cast::<u8>(), ksize) };
    if user.read(arg, &mut bytes[..in_size]).is_err() {
        return -(EFAULT as i64);
    }
    let ret = match permit(kernel, desc.flags, file, node) {
        true => unsafe { dispatch(kernel, file, desc.func, bytes.as_mut_ptr(), user, prime) },
        false => -(EACCES as i64),
    };
    match user.write(arg, &bytes[..out_size]) {
        Ok(()) => ret,
        Err(_) => -(EFAULT as i64),
    }
}

fn copy_field(
    user: &dyn UserMemory,
    buf: *mut libc::c_char,
    buf_len: &mut usize,
    value: &str,
) -> Result<(), i32> {
    let len = value.len().min(*buf_len);
    *buf_len = value.len();
    let copy = len != 0 && !buf.is_null();
    match copy && user.write(buf as u64, &value.as_bytes()[..len]).is_err() {
        true => Err(EFAULT),
        false => Ok(()),
    }
}

unsafe fn dispatch(
    kernel: &dyn Drm,
    file: u64,
    func: Func,
    arg: *mut u8,
    user: &dyn UserMemory,
    prime: &mut dyn Prime,
) -> i64 {
    unsafe {
        match func {
            Func::DrmNoop => 0,
            Func::DrmInvalidOp => -(EINVAL as i64),
            Func::DrmVersion => {
                let args = &mut *arg.cast::<drm_version>();
                let version = kernel.driver_version();
                args.version_major = version.major;
                args.version_minor = version.minor;
                args.version_patchlevel = version.patchlevel;
                let copied = copy_field(user, args.name, &mut args.name_len, kernel.name())
                    .and_then(|()| copy_field(user, args.date, &mut args.date_len, "0"))
                    .and_then(|()| copy_field(user, args.desc, &mut args.desc_len, kernel.desc()));
                match copied {
                    Ok(()) => 0,
                    Err(errno) => -(errno as i64),
                }
            }
            Func::DrmGetcap => getcap(kernel, &mut *arg.cast::<drm_get_cap>()),
            Func::DrmGemCloseIoctl => {
                let args = &*arg.cast::<drm_gem_close>();
                match kernel.close_buffer(file, args.handle) {
                    true => 0,
                    false => -(EINVAL as i64),
                }
            }
            Func::DrmPrimeHandleToFdIoctl => {
                let args = &mut *arg.cast::<drm_prime_handle>();
                if args.flags & !(DRM_CLOEXEC | DRM_RDWR) != 0 {
                    return -(EINVAL as i64);
                }
                if args.flags != DRM_CLOEXEC | DRM_RDWR {
                    if kernel.object(file, args.handle).is_none() {
                        return -(ENOENT as i64);
                    }
                    unimplemented!("PRIME export flags {:#x}", args.flags);
                }
                prime.handle_to_fd(kernel, file, args)
            }
            Func::DrmPrimeFdToHandleIoctl => {
                prime.fd_to_handle(kernel, file, &mut *arg.cast::<drm_prime_handle>())
            }
            func => match syncobj::dispatch(kernel, file, func, arg, user) {
                Some(ret) => ret,
                None => match kernel.ioctl(file, func, arg, user, prime) {
                    Some(ret) => ret,
                    None => panic!("{} does not model {}", kernel.id(), func.name()),
                },
            },
        }
    }
}

fn getcap(kernel: &dyn Drm, args: &mut drm_get_cap) -> i64 {
    args.value = 0;
    match args.capability {
        DRM_CAP_TIMESTAMP_MONOTONIC => unimplemented!("DRM_CAP_TIMESTAMP_MONOTONIC"),
        DRM_CAP_PRIME => {
            args.value = DRM_PRIME_CAP_IMPORT | DRM_PRIME_CAP_EXPORT;
            return 0;
        }
        DRM_CAP_SYNCOBJ => {
            args.value = kernel.syncobjs().is_some() as u64;
            return 0;
        }
        DRM_CAP_SYNCOBJ_TIMELINE => {
            args.value = kernel.check_feature(DriverFeature::SyncobjTimeline) as u64;
            return 0;
        }
        _ => {}
    }
    if !kernel.check_feature(DriverFeature::Modeset) {
        return -(EOPNOTSUPP as i64);
    }
    match args.capability {
        DRM_CAP_ADDFB2_MODIFIERS => args.value = !kernel.fb_modifiers_not_supported() as u64,
        capability => unimplemented!("capability {capability:#x}"),
    }
    0
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    #[test]
    fn device_names_are_unique() {
        let amd: Vec<_> = crate::amd::devices::DEVICES
            .iter()
            .map(|d| d.name)
            .collect();
        let intel: Vec<_> = crate::intel::devices::DEVICES
            .iter()
            .map(|d| d.name)
            .collect();
        assert_eq!(amd.iter().collect::<BTreeSet<_>>().len(), amd.len());
        assert_eq!(intel.iter().collect::<BTreeSet<_>>().len(), intel.len());
    }
}
