use crate::drivers::amd::amdgpu;
pub use crate::drivers::amd::amdgpu::AmdgpuAllocateError;
pub use crate::drivers::amd::amdgpu::AmdgpuGfxVersionError;
pub use crate::drivers::amd::amdgpu::AmdgpuOpenError;
pub use crate::drivers::amd::amdgpu::AmdgpuQueryHwIpError;
pub use crate::drivers::amd::amdgpu::AmdgpuReadRegistersError;
pub use crate::drivers::amd::amdgpu::AmdgpuVirtioOpenError;
pub use crate::drivers::amd::virtio::AmdgpuVirtioCommandError;
use crate::drivers::asahi;
pub use crate::drivers::asahi::AsahiError;
use crate::drivers::dumb;
pub use crate::drivers::dumb::DumbError;
use crate::drivers::etnaviv;
pub use crate::drivers::etnaviv::EtnavivError;
use crate::drivers::intel::i915;
pub use crate::drivers::intel::i915::I915Error;
use crate::drivers::intel::xe;
pub use crate::drivers::intel::xe::XeError;
use crate::drivers::lima;
pub use crate::drivers::lima::LimaError;
use crate::drivers::nvidia::nouveau;
pub use crate::drivers::nvidia::nouveau::NouveauError;
use crate::drivers::nvidia::nvidia_drm;
pub use crate::drivers::nvidia::nvidia_drm::NvidiaDrmError;
use crate::drivers::panfrost;
pub use crate::drivers::panfrost::PanfrostError;
use crate::drivers::udmabuf;
pub use crate::drivers::udmabuf::UdmabufError;
use crate::drivers::v3d;
pub use crate::drivers::v3d::V3dError;
use crate::drivers::vc4;
pub use crate::drivers::vc4::Vc4Error;
use crate::drivers::virtio_gpu;
pub use crate::drivers::virtio_gpu::VirtioGpuError;
use crate::drm;
pub use crate::drm::DrmError;
use crate::errno::Errno;
use crate::fd::OwnedFd;
use crate::format::Format;
pub use crate::kernel::Kernel;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::sys::_IOW;
use crate::sys::Ioctl;
pub use crate::sys::Sys;
pub use crate::vdrm::VdrmError;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::ffi::CStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AllocatorError {
    #[error("could not duplicate the file descriptor")]
    Dup(#[source] Errno),
    #[error("could not identify the driver")]
    DriverName(#[source] DrmError),
    #[error("could not name the dma-buf")]
    SetName(#[source] Errno),
    #[error("could not compute the layout")]
    Layout(#[source] LayoutError),
    #[error("no format was set")]
    NoFormat,
    #[error("no modifiers were set")]
    NoModifiers,
    #[error("no size was set")]
    NoSize,
    #[error("could not create the udmabuf for the memfd")]
    Udmabuf(#[source] UdmabufError),
    #[error("could not create the memfd for the udmabuf")]
    UdmabufMemfd(#[source] UdmabufError),
    #[error("could not open the amdgpu device")]
    AmdgpuOpen(#[source] AmdgpuOpenError),
    #[error("could not open the i915 device")]
    I915Open(#[source] I915Error),
    #[error("could not open the xe device")]
    XeOpen(#[source] XeError),
    #[error("could not open the nouveau device")]
    NouveauOpen(#[source] NouveauError),
    #[error("could not open the asahi device")]
    AsahiOpen(#[source] AsahiError),
    #[error("could not open the panfrost device")]
    PanfrostOpen(#[source] PanfrostError),
    #[error("could not open the panthor device")]
    PanthorOpen(#[source] PanfrostError),
    #[error("could not open the etnaviv device")]
    EtnavivOpen(#[source] EtnavivError),
    #[error("could not open the virtio_gpu device")]
    VirtioGpuOpen(#[source] VirtioGpuError),
    #[error("dumb allocation failed")]
    DumbAllocate(#[source] DumbError),
    #[error("amdgpu allocation failed")]
    AmdgpuAllocate(#[source] AmdgpuAllocateError),
    #[error("i915 allocation failed")]
    I915Allocate(#[source] I915Error),
    #[error("xe allocation failed")]
    XeAllocate(#[source] XeError),
    #[error("nvidia-drm allocation failed")]
    NvidiaDrmAllocate(#[source] NvidiaDrmError),
    #[error("nouveau allocation failed")]
    NouveauAllocate(#[source] NouveauError),
    #[error("asahi allocation failed")]
    AsahiAllocate(#[source] AsahiError),
    #[error("panfrost allocation failed")]
    PanfrostAllocate(#[source] PanfrostError),
    #[error("panthor allocation failed")]
    PanthorAllocate(#[source] PanfrostError),
    #[error("v3d allocation failed")]
    V3dAllocate(#[source] V3dError),
    #[error("etnaviv allocation failed")]
    EtnavivAllocate(#[source] EtnavivError),
    #[error("lima allocation failed")]
    LimaAllocate(#[source] LimaError),
    #[error("vc4 allocation failed")]
    Vc4Allocate(#[source] Vc4Error),
    #[error("virtio_gpu allocation failed")]
    VirtioGpuAllocate(#[source] VirtioGpuError),
    #[error("udmabuf allocation failed")]
    UdmabufAllocate(#[source] UdmabufError),
}

pub fn create_udmabuf(
    udmabuf_fd: i32,
    memfd: i32,
    offset: u64,
    size: u64,
) -> Result<OwnedFd, AllocatorError> {
    udmabuf::create(&Sys, udmabuf_fd, memfd, offset, size)
        .map(|fd| OwnedFd::new(&Sys, fd))
        .map_err(AllocatorError::Udmabuf)
}

pub fn create_memfd(layout: &Layout) -> Result<(OwnedFd, u64), AllocatorError> {
    udmabuf::create_memfd(&Sys, layout).map_err(AllocatorError::UdmabufMemfd)
}

enum Driver<K: Kernel> {
    Amdgpu(amdgpu::Device<K>),
    Asahi(asahi::Device<K>),
    Dumb(Arc<OwnedFd<K>>),
    Etnaviv(etnaviv::Device<K>),
    I915(i915::Device<K>),
    Lima(lima::Device<K>),
    Nouveau(nouveau::Device<K>),
    NvidiaDrm(nvidia_drm::Device<K>),
    Panfrost(panfrost::Device<K>),
    Panthor(panfrost::Device<K>),
    Udmabuf(udmabuf::Device<K>),
    V3d(v3d::Device<K>),
    Vc4(vc4::Device<K>),
    VirtioGpu(virtio_gpu::Device<K>),
    Xe(xe::Device<K>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DriverName {
    Amdgpu,
    Asahi,
    Dumb,
    Etnaviv,
    I915,
    Lima,
    Nouveau,
    NvidiaDrm,
    Panfrost,
    Panthor,
    Udmabuf,
    V3d,
    Vc4,
    Virgl,
    VirtioGpuAmdgpu,
    VirtioGpuAsahi,
    VirtioGpuI915,
    VirtioGpuPanfrost,
    Xe,
}

pub struct Allocator<K: Kernel = Sys> {
    kernel: K,
    driver: Box<Driver<K>>,
}

pub struct Config<'a> {
    pub format: Format,
    pub modifiers: &'a [u64],
    pub width: u32,
    pub height: u32,
    pub scanout: bool,
    pub implicit_sync: bool,
    pub linear: bool,
    pub implicit_interop: bool,
    pub name: Option<&'a CStr>,
}

pub struct Allocation<K: Kernel = Sys> {
    pub layout: Layout,
    pub dmabuf: OwnedFd<K>,
}

pub(crate) struct DeviceLayout {
    pub(crate) layout: Layout,
    pub(crate) extra: Extra,
}

#[allow(clippy::large_enum_variant)]
pub(crate) enum Extra {
    None,
    Amd(amdgpu::Bo),
}

impl Allocator {
    pub fn new(fd: i32) -> Result<Allocator, AllocatorError> {
        Self::with_kernel(Sys, fd)
    }
}

const DMA_BUF_SET_NAME_B: Ioctl = _IOW::<u64>(b'b' as u32, 1);

fn name_dmabuf<K: Kernel>(fd: &OwnedFd<K>, name: &CStr) -> Result<(), AllocatorError> {
    unsafe {
        drm::ioctl(fd, DMA_BUF_SET_NAME_B, name.as_ptr().cast_mut())
            .map_err(AllocatorError::SetName)
    }
}

impl<K: Kernel> Allocator<K> {
    pub fn with_kernel(kernel: K, fd: i32) -> Result<Allocator<K>, AllocatorError> {
        let fd = kernel
            .dup(fd)
            .map(|fd| OwnedFd::new(&kernel, fd))
            .map(Arc::new)
            .map_err(AllocatorError::Dup)?;
        let mut name = [0; 32];
        let driver = match drm::driver_name(&fd, &mut name) {
            Err(DrmError::Version(Errno(libc::ENOTTY))) if kernel.is_udmabuf(fd.fd) => {
                Driver::Udmabuf(udmabuf::Device::new(&fd))
            }
            name => match name.map_err(AllocatorError::DriverName)? {
                b"amdgpu" => {
                    Driver::Amdgpu(amdgpu::Device::new(&fd).map_err(AllocatorError::AmdgpuOpen)?)
                }
                b"i915" => Driver::I915(i915::Device::new(&fd).map_err(AllocatorError::I915Open)?),
                b"xe" => Driver::Xe(xe::Device::new(&fd).map_err(AllocatorError::XeOpen)?),
                b"nvidia-drm" => Driver::NvidiaDrm(nvidia_drm::Device::new(&fd)),
                b"nouveau" => {
                    Driver::Nouveau(nouveau::Device::new(&fd).map_err(AllocatorError::NouveauOpen)?)
                }
                b"asahi" => {
                    Driver::Asahi(asahi::Device::new(&fd).map_err(AllocatorError::AsahiOpen)?)
                }
                b"panfrost" => Driver::Panfrost(
                    panfrost::Device::panfrost(&fd).map_err(AllocatorError::PanfrostOpen)?,
                ),
                b"panthor" => Driver::Panthor(
                    panfrost::Device::panthor(&fd).map_err(AllocatorError::PanthorOpen)?,
                ),
                b"v3d" => Driver::V3d(v3d::Device::new(&fd)),
                b"lima" => Driver::Lima(lima::Device::new(&fd)),
                b"vc4" => Driver::Vc4(vc4::Device::new(&fd)),
                b"etnaviv" => {
                    Driver::Etnaviv(etnaviv::Device::new(&fd).map_err(AllocatorError::EtnavivOpen)?)
                }
                // freedreno GBM implementation is broken. Nothing to test against.
                b"msm" => Driver::Dumb(fd),
                b"virtio_gpu" => {
                    match virtio_gpu::Device::probe(&fd).map_err(AllocatorError::VirtioGpuOpen)? {
                        Some(device) => Driver::VirtioGpu(device),
                        None => Driver::Dumb(fd),
                    }
                }
                _ => Driver::Dumb(fd),
            },
        };
        Ok(Allocator {
            kernel,
            driver: Box::new(driver),
        })
    }

    pub fn driver(&self) -> DriverName {
        macro_rules! map {
            ($($name:ident,)*) => {
                match &*self.driver {
                    $(Driver::$name(_) => DriverName::$name,)*
                    Driver::VirtioGpu(device) => match device {
                        virtio_gpu::Device::Virgl(_) => DriverName::Virgl,
                        virtio_gpu::Device::Amdgpu(_) => DriverName::VirtioGpuAmdgpu,
                        virtio_gpu::Device::Asahi(_) => DriverName::VirtioGpuAsahi,
                        virtio_gpu::Device::I915(_) => DriverName::VirtioGpuI915,
                        virtio_gpu::Device::Panfrost(_) => DriverName::VirtioGpuPanfrost,
                    },
                }
            };
        }
        map! {
            Amdgpu,
            Asahi,
            Dumb,
            Etnaviv,
            I915,
            Lima,
            Nouveau,
            NvidiaDrm,
            Panfrost,
            Panthor,
            Udmabuf,
            V3d,
            Vc4,
            Xe,
        }
    }

    pub fn layout(&self, config: &Config<'_>) -> Result<Layout, LayoutError> {
        self.device_layout(config).map(|layout| layout.layout)
    }

    pub fn allocate(&self, config: &Config<'_>) -> Result<Allocation<K>, AllocatorError> {
        let allocation = self.allocate_unnamed(config)?;
        if let Some(name) = config.name {
            name_dmabuf(&allocation.dmabuf, name)?;
        }
        Ok(allocation)
    }

    fn allocate_unnamed(&self, config: &Config<'_>) -> Result<Allocation<K>, AllocatorError> {
        if let Driver::Dumb(fd) = &*self.driver {
            let allocation = dumb::allocate(fd, config).map_err(AllocatorError::DumbAllocate)?;
            return Ok(Allocation {
                layout: allocation.layout,
                dmabuf: OwnedFd::new(&self.kernel, allocation.fd),
            });
        }
        let layout = self.device_layout(config).map_err(AllocatorError::Layout)?;
        let fd = match &*self.driver {
            Driver::Amdgpu(device) => device
                .allocate(&layout, config)
                .map_err(AllocatorError::AmdgpuAllocate)?,
            Driver::I915(device) => device
                .allocate(&layout, config)
                .map_err(AllocatorError::I915Allocate)?,
            Driver::Xe(device) => device
                .allocate(&layout, config)
                .map_err(AllocatorError::XeAllocate)?,
            Driver::NvidiaDrm(device) => device
                .allocate(&layout, config)
                .map_err(AllocatorError::NvidiaDrmAllocate)?,
            Driver::Nouveau(device) => device
                .allocate(&layout, config)
                .map_err(AllocatorError::NouveauAllocate)?,
            Driver::Asahi(device) => device
                .allocate(&layout)
                .map_err(AllocatorError::AsahiAllocate)?,
            Driver::Panfrost(device) => device
                .allocate(&layout)
                .map_err(AllocatorError::PanfrostAllocate)?,
            Driver::Panthor(device) => device
                .allocate(&layout)
                .map_err(AllocatorError::PanthorAllocate)?,
            Driver::V3d(device) => device
                .allocate(&layout)
                .map_err(AllocatorError::V3dAllocate)?,
            Driver::Etnaviv(device) => device
                .allocate(&layout)
                .map_err(AllocatorError::EtnavivAllocate)?,
            Driver::Lima(device) => device
                .allocate(&layout)
                .map_err(AllocatorError::LimaAllocate)?,
            Driver::Vc4(device) => device
                .allocate(&layout)
                .map_err(AllocatorError::Vc4Allocate)?,
            Driver::VirtioGpu(device) => device
                .allocate(&layout, config)
                .map_err(AllocatorError::VirtioGpuAllocate)?,
            Driver::Udmabuf(device) => device
                .allocate(&layout)
                .map_err(AllocatorError::UdmabufAllocate)?,
            Driver::Dumb(_) => return Err(AllocatorError::Layout(LayoutError::Dumb)),
        };
        Ok(Allocation {
            layout: layout.layout,
            dmabuf: OwnedFd::new(&self.kernel, fd),
        })
    }

    fn device_layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        match &*self.driver {
            Driver::Amdgpu(device) => device.layout(config),
            Driver::Asahi(device) => device.layout(config),
            Driver::Dumb(_) => Err(LayoutError::Dumb),
            Driver::Etnaviv(device) => device.layout(config),
            Driver::I915(device) => device.layout(config),
            Driver::Lima(device) => device.layout(config),
            Driver::Nouveau(device) => device.layout(config),
            Driver::NvidiaDrm(device) => device.layout(config),
            Driver::Panfrost(device) => device.layout(config),
            Driver::Panthor(device) => device.layout(config),
            Driver::Udmabuf(device) => device.layout(config),
            Driver::V3d(device) => device.layout(config),
            Driver::Vc4(device) => device.layout(config),
            Driver::VirtioGpu(device) => device.layout(config),
            Driver::Xe(device) => device.layout(config),
        }
    }
}
