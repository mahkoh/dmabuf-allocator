//! Allocator implementations for Direct Rendering Manager (DRM) devices and the udmabuf
//! driver.
//!
//! This crate provides the [`Allocator`] type that you can use to allocate dma-bufs for
//! images. This functionality is similar to the Generic Buffer Manager ([GBM]).
//!
//! Create an [`Allocator`] object from a file descriptor that refers either to a DRM
//! device or to the `/dev/udmabuf` device. The allocator detects the file descriptor type
//! automatically. If creation succeeds, then you can query the name of the underlying
//! driver with the [`Allocator::driver`] function.
//!
//! [GBM]: https://gitlab.freedesktop.org/mesa/mesa/-/blob/a54c80d04d37b5540897d00f39647a11156693af/src/gbm/main/gbm.h
//!
//! # Usage Example
//!
//! The following example allocates a buffer on a render node and prints its layout:
//!
//! ```rust
//! use dmabuf_allocator::Allocator;
//! use std::fs::OpenOptions;
//! use std::os::fd::AsRawFd;
//!
//! // In practice, you would get the list of formats and modifiers
//! // by intersecting the requirements of all users of the dma-buf.
//! let format = dmabuf_allocator::get_format(b"AR24").unwrap();
//! const DRM_FORMAT_MOD_LINEAR: u64 = 0;
//!
//! // In practice, you likely already have a file descriptor. In the case of a
//! // compositor, pass the card device so that the allocator can fall back to dumb
//! // buffers.
//! let device = OpenOptions::new()
//!     .read(true)
//!     .write(true)
//!     .open("/dev/dri/renderD128")
//!     .unwrap();
//!
//! let allocator = Allocator::new(device.as_raw_fd()).unwrap();
//! println!("driver: {:?}", allocator.driver());
//!
//! let allocation = allocator
//!     .builder()
//!     .format(format)
//!     .modifiers(&[DRM_FORMAT_MOD_LINEAR])
//!     .size(1920, 1080)
//!     .name(c"example")
//!     .allocate()
//!     .unwrap();
//!
//! println!("modifier: {:#x}", allocation.layout.modifier());
//! for plane in allocation.layout.planes() {
//!     println!("offset: {}, stride: {}", plane.offset(), plane.stride());
//! }
//! println!("dma-buf: {}", allocation.dmabuf.fd());
//! ```
//!
//! # Driver Support
//!
//! The crate currently supports the following DRM kernel drivers:
//!
//! - `amdgpu`
//! - `asahi`
//! - `etnaviv`
//! - `i915`
//! - `lima`
//! - `nouveau`
//! - `nvidia-drm`
//! - `panfrost`
//! - `panthor`
//! - `v3d`
//! - `vc4`
//! - `virtio_gpu`
//! - `xe`
//!
//! The i915 and xe drivers do not expose the required information for allocation.
//! Therefore, the crate contains a generated list of supported devices and their
//! properties, similar to Mesa. If a device does not appear in this list, then the
//! allocator allocates only linear buffers.
//!
//! # Allocating with the Invalid Modifier
//!
//! Most drivers do not support the `DRM_FORMAT_MOD_INVALID` modifier, also called the
//! invalid modifier. The only exception is the amdgpu driver on GFX6 to GFX8. For all
//! other drivers, including the invalid modifier in the slice that you pass to the
//! [`AllocationBuilder::modifiers`] function has no effect.
//!
//! # Importing with the Invalid Modifier
//!
//! In contrast, all drivers support allocating buffers that consumers _can_ import with
//! the invalid modifier. To allocate such a buffer, call the
//! [`AllocationBuilder::implicit_interop`] function. In almost all situations, such a
//! buffer performs significantly worse than a buffer that you allocate without this
//! setting. Therefore, enable this setting only if consumers of the dma-buf do not
//! support explicit modifiers.
//!
//! As outlined in the previous section, even with this setting enabled, you must still
//! _allocate_ the buffer with an explicit modifier.
//!
//! # Modifier Choice
//!
//! If the slice that you pass to the [`AllocationBuilder::modifiers`] function contains
//! more than one modifier, then the allocator chooses the most efficient one that fits
//! the requirements that you describe in the builder (for example, the size and scanout
//! capabilities).
//!
//! You must _not_ assume that the allocator filters the modifiers to ensure that the
//! device supports them. For example, if you pass an amdgpu GFX11 modifier to an
//! allocator running on an amdgpu GFX10 device, then the allocator chooses this modifier
//! over GFX10 modifiers in the same slice.

#![no_std]

use core::ffi::CStr;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Write;
use core::mem;
use dmabuf_allocator_impl::allocator::AllocatorError;
use dmabuf_allocator_impl::allocator::Config;
use dmabuf_allocator_impl::allocator::DriverName;

#[cfg(feature = "std")]
mod std_impls;
pub mod udmabuf;

/// An error.
///
/// # Usage Example
///
/// The following example prints the error that an invalid file descriptor causes:
///
/// ```rust
/// use dmabuf_allocator::Allocator;
///
/// let error = Allocator::new(-1).unwrap_err();
/// println!("error: {error}");
/// ```
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(AllocatorError);

/// A dma-buf allocator.
///
/// # Usage Example
///
/// The following example creates an allocator for a render node:
///
/// ```rust
/// use dmabuf_allocator::Allocator;
/// use std::fs::OpenOptions;
/// use std::os::fd::AsRawFd;
///
/// let device = OpenOptions::new()
///     .read(true)
///     .write(true)
///     .open("/dev/dri/renderD128")
///     .unwrap();
/// let allocator = Allocator::new(device.as_raw_fd()).unwrap();
/// println!("allocator: {allocator:?}");
/// ```
pub struct Allocator(dmabuf_allocator_impl::allocator::Allocator);

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Allocator>();
};

/// The name of the underlying driver.
///
/// # Usage Example
///
/// The following example checks that an allocator for the `/dev/udmabuf` device reports
/// the udmabuf driver:
///
/// ```rust
/// use dmabuf_allocator::Allocator;
/// use dmabuf_allocator::Driver;
/// use std::fs::File;
/// use std::os::fd::AsRawFd;
///
/// let device = File::open("/dev/udmabuf").unwrap();
/// let allocator = Allocator::new(device.as_raw_fd()).unwrap();
/// assert_eq!(allocator.driver(), Driver::Udmabuf);
/// ```
#[derive(Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Driver {
    /// The `amdgpu` driver.
    Amdgpu,
    /// The `asahi` driver.
    Asahi,
    /// An allocator for dumb buffers.
    Dumb,
    /// The `etnaviv` driver.
    Etnaviv,
    /// The `i915` driver.
    I915,
    /// The `lima` driver.
    Lima,
    /// The `nouveau` driver.
    Nouveau,
    /// The `nvidia-drm` driver.
    NvidiaDrm,
    /// The `panfrost` driver.
    Panfrost,
    /// The `panthor` driver.
    Panthor,
    /// The `/dev/udmabuf` device.
    Udmabuf,
    /// The `v3d` driver.
    V3d,
    /// The `vc4` driver.
    Vc4,
    /// The `virgl` driver.
    Virgl,
    /// The `amdgpu` driver over virtio.
    VirtioGpuAmdgpu,
    /// The `asahi` driver over virtio.
    VirtioGpuAsahi,
    /// The `i915` driver over virtio.
    VirtioGpuI915,
    /// The `panfrost` driver over virtio.
    VirtioGpuPanfrost,
    /// The `xe` driver.
    Xe,
}

/// A builder for an allocation or a layout.
///
/// # Usage Example
///
/// The following example configures a builder and allocates a buffer:
///
/// ```rust
/// # use dmabuf_allocator::Allocator;
/// # use std::fs::OpenOptions;
/// # use std::os::fd::AsRawFd;
/// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
/// # const DRM_FORMAT_MOD_LINEAR: u64 = 0;
/// # let device = OpenOptions::new()
/// #     .read(true)
/// #     .write(true)
/// #     .open("/dev/dri/renderD128")
/// #     .unwrap();
/// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
/// let builder = allocator
///     .builder()
///     .format(format)
///     .modifiers(&[DRM_FORMAT_MOD_LINEAR])
///     .size(1920, 1080);
/// let allocation = builder.allocate().unwrap();
/// ```
pub struct AllocationBuilder<'a> {
    allocator: &'a dmabuf_allocator_impl::allocator::Allocator,
    format: Option<Format>,
    modifiers: Option<&'a [u64]>,
    width: Option<u32>,
    height: Option<u32>,
    scanout: bool,
    implicit_sync: bool,
    linear: bool,
    implicit_interop: bool,
    name: Option<&'a CStr>,
}

/// An allocated buffer.
///
/// # Usage Example
///
/// The following example prints the modifier and the file descriptor of an allocation:
///
/// ```rust
/// # use dmabuf_allocator::Allocator;
/// # use std::fs::OpenOptions;
/// # use std::os::fd::AsRawFd;
/// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
/// # const DRM_FORMAT_MOD_LINEAR: u64 = 0;
/// # let device = OpenOptions::new()
/// #     .read(true)
/// #     .write(true)
/// #     .open("/dev/dri/renderD128")
/// #     .unwrap();
/// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
/// let allocation = allocator
///     .builder()
///     .format(format)
///     .modifiers(&[DRM_FORMAT_MOD_LINEAR])
///     .size(1920, 1080)
///     .allocate()
///     .unwrap();
/// println!("modifier: {:#x}", allocation.layout.modifier());
/// println!("dma-buf: {}", allocation.dmabuf.fd());
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub struct Allocation {
    /// The layout of the buffer.
    pub layout: Layout,
    /// The dma-buf.
    pub dmabuf: OwnedFd,
}

/// An owned file descriptor.
///
/// Dropping this object closes the file descriptor.
///
/// # Usage Example
///
/// The following example closes the file descriptor of a memfd:
///
/// ```rust
/// use dmabuf_allocator::udmabuf;
/// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
///
/// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
/// let memfd = udmabuf::create_memfd(&layout).unwrap();
/// drop(memfd.fd);
/// ```
pub struct OwnedFd(dmabuf_allocator_impl::fd::OwnedFd);

/// A known format.
///
/// Create objects of this type by calling the [`get_format`] const function.
///
/// # Usage Example
///
/// The following example stores a format in a constant:
///
/// ```rust
/// use dmabuf_allocator::Format;
///
/// const ARGB8888: Format = dmabuf_allocator::get_format(b"AR24").unwrap();
/// ```
#[derive(Copy, Clone)]
pub struct Format(dmabuf_allocator_impl::format::Format);

/// The layout of a buffer.
///
/// # Usage Example
///
/// The following example computes a linear layout and reads its properties:
///
/// ```rust
/// use dmabuf_allocator::udmabuf;
/// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
///
/// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
/// assert_eq!(layout.modifier(), 0);
/// assert_eq!(layout.planes().len(), 1);
/// ```
pub struct Layout(dmabuf_allocator_impl::layout::Layout);

/// The layout of a plane of a buffer.
///
/// # Usage Example
///
/// The following example reads the offset and the stride of the only plane of a linear
/// layout:
///
/// ```rust
/// use dmabuf_allocator::udmabuf;
/// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
///
/// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
/// let plane = &layout.planes()[0];
/// assert_eq!(plane.offset(), 0);
/// assert_eq!(plane.stride(), 7680);
/// ```
#[repr(transparent)]
pub struct PlaneLayout(dmabuf_allocator_impl::layout::PlaneLayout);

/// Returns the allocator's internal format for a given fourcc code.
///
/// # Usage Example
///
/// The following example looks up one format by the characters of its fourcc code and one
/// by the number of its fourcc code:
///
/// ```rust
/// use dmabuf_allocator::get_format;
/// use dmabuf_allocator::Format;
///
/// const AR24: Format = get_format(b"AR24").unwrap();
/// const AB24: Format = get_format(&0x34324241_u32.to_le_bytes()).unwrap();
/// ```
pub const fn get_format(fourcc: &[u8; 4]) -> Option<Format> {
    match dmabuf_allocator_impl::format::get_format(u32::from_le_bytes(*fourcc)) {
        Some(f) => Some(Format(f)),
        None => None,
    }
}

impl Debug for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let bytes = self.0.0.fourcc.to_le_bytes();
        f.write_char('"')?;
        for b in bytes {
            f.write_char(b as char)?;
        }
        f.write_char('"')?;
        Ok(())
    }
}

impl Debug for Driver {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Driver::Amdgpu => "amdgpu",
            Driver::Asahi => "asahi",
            Driver::Dumb => "dumb",
            Driver::Etnaviv => "etnaviv",
            Driver::I915 => "i915",
            Driver::Lima => "lima",
            Driver::Nouveau => "nouveau",
            Driver::NvidiaDrm => "nvidia-drm",
            Driver::Panfrost => "panfrost",
            Driver::Panthor => "panthor",
            Driver::Udmabuf => "udmabuf",
            Driver::V3d => "v3d",
            Driver::Vc4 => "vc4",
            Driver::Virgl => "virgl",
            Driver::VirtioGpuAmdgpu => "amdgpu/virtio",
            Driver::VirtioGpuAsahi => "asahi/virtio",
            Driver::VirtioGpuI915 => "i915/virtio",
            Driver::VirtioGpuPanfrost => "panfrost/virtio",
            Driver::Xe => "xe",
        };
        f.write_str(name)
    }
}

impl Debug for Allocator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.driver().fmt(f)
    }
}

impl Allocator {
    /// Creates an allocator.
    ///
    /// The file descriptor must refer to a DRM device or to the `/dev/udmabuf` device.
    ///
    /// You can close the file descriptor immediately after this function returns.
    ///
    /// # Errors
    ///
    /// This function returns an error in the following cases:
    ///
    /// - The file descriptor cannot be duplicated.
    /// - The driver of the device cannot be determined.
    /// - The driver-specific initialization fails.
    ///
    /// # Usage Example
    ///
    /// The following example creates an allocator and then closes the file descriptor:
    ///
    /// ```rust
    /// use dmabuf_allocator::Allocator;
    /// use std::fs::OpenOptions;
    /// use std::os::fd::AsRawFd;
    ///
    /// let device = OpenOptions::new()
    ///     .read(true)
    ///     .write(true)
    ///     .open("/dev/dri/renderD128")
    ///     .unwrap();
    /// let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    /// drop(device);
    /// println!("driver: {:?}", allocator.driver());
    /// ```
    pub fn new(fd: i32) -> Result<Allocator, Error> {
        dmabuf_allocator_impl::allocator::Allocator::new(fd)
            .map(Allocator)
            .map_err(Error)
    }

    /// Returns the underlying driver.
    ///
    /// # Usage Example
    ///
    /// The following example prints the driver of an allocator:
    ///
    /// ```rust
    /// # use dmabuf_allocator::Allocator;
    /// # use std::fs::OpenOptions;
    /// # use std::os::fd::AsRawFd;
    /// # let device = OpenOptions::new()
    /// #     .read(true)
    /// #     .write(true)
    /// #     .open("/dev/dri/renderD128")
    /// #     .unwrap();
    /// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    /// let driver = allocator.driver();
    /// println!("driver: {driver:?}");
    /// ```
    pub fn driver(&self) -> Driver {
        macro_rules! map {
            ($($name:ident,)*) => {
                match self.0.driver() {
                    $(DriverName::$name => Driver::$name,)*
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
            Virgl,
            VirtioGpuAmdgpu,
            VirtioGpuAsahi,
            VirtioGpuI915,
            VirtioGpuPanfrost,
            Xe,
        }
    }

    /// Creates a builder for an allocation.
    ///
    /// Set the format, modifiers, and size before allocating. Otherwise, the allocation
    /// fails.
    ///
    /// # Usage Example
    ///
    /// The following example shows that the allocation fails if you set none of the
    /// required properties:
    ///
    /// ```rust
    /// # use dmabuf_allocator::Allocator;
    /// # use std::fs::OpenOptions;
    /// # use std::os::fd::AsRawFd;
    /// # let device = OpenOptions::new()
    /// #     .read(true)
    /// #     .write(true)
    /// #     .open("/dev/dri/renderD128")
    /// #     .unwrap();
    /// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    /// let builder = allocator.builder();
    /// assert!(builder.allocate().is_err());
    /// ```
    pub fn builder(&self) -> AllocationBuilder<'_> {
        AllocationBuilder {
            allocator: &self.0,
            format: None,
            modifiers: None,
            width: None,
            height: None,
            scanout: false,
            implicit_sync: true,
            linear: false,
            implicit_interop: false,
            name: None,
        }
    }
}

impl<'a> AllocationBuilder<'a> {
    fn config(&self) -> Result<Config<'a>, Error> {
        Ok(Config {
            format: self.format.ok_or(Error(AllocatorError::NoFormat))?.0,
            modifiers: self.modifiers.ok_or(Error(AllocatorError::NoModifiers))?,
            width: self.width.ok_or(Error(AllocatorError::NoSize))?,
            height: self.height.ok_or(Error(AllocatorError::NoSize))?,
            scanout: self.scanout,
            implicit_sync: self.implicit_sync,
            linear: self.linear,
            implicit_interop: self.implicit_interop,
            name: self.name,
        })
    }

    /// Sets the format of the buffer.
    ///
    /// # Usage Example
    ///
    /// The following example sets the format to ARGB8888:
    ///
    /// ```rust
    /// # use dmabuf_allocator::Allocator;
    /// # use std::fs::OpenOptions;
    /// # use std::os::fd::AsRawFd;
    /// # let device = OpenOptions::new()
    /// #     .read(true)
    /// #     .write(true)
    /// #     .open("/dev/dri/renderD128")
    /// #     .unwrap();
    /// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    ///
    /// let format = dmabuf_allocator::get_format(b"AR24").unwrap();
    /// let builder = allocator.builder().format(format);
    /// ```
    #[must_use]
    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    /// Sets the modifiers.
    ///
    /// # Usage Example
    ///
    /// The following example requests a buffer with the linear modifier:
    ///
    /// ```rust
    /// # use dmabuf_allocator::Allocator;
    /// # use std::fs::OpenOptions;
    /// # use std::os::fd::AsRawFd;
    /// # let device = OpenOptions::new()
    /// #     .read(true)
    /// #     .write(true)
    /// #     .open("/dev/dri/renderD128")
    /// #     .unwrap();
    /// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    /// const DRM_FORMAT_MOD_LINEAR: u64 = 0;
    ///
    /// let builder = allocator.builder().modifiers(&[DRM_FORMAT_MOD_LINEAR]);
    /// ```
    #[must_use]
    pub fn modifiers(mut self, modifiers: &'a [u64]) -> Self {
        self.modifiers = Some(modifiers);
        self
    }

    /// Sets the size of the buffer.
    ///
    /// # Usage Example
    ///
    /// The following example sets the size to 1920 by 1080 pixels:
    ///
    /// ```rust
    /// # use dmabuf_allocator::Allocator;
    /// # use std::fs::OpenOptions;
    /// # use std::os::fd::AsRawFd;
    /// # let device = OpenOptions::new()
    /// #     .read(true)
    /// #     .write(true)
    /// #     .open("/dev/dri/renderD128")
    /// #     .unwrap();
    /// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    /// let builder = allocator.builder().size(1920, 1080);
    /// ```
    #[must_use]
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Sets whether the buffer supports scanout.
    ///
    /// By default, this setting is disabled.
    ///
    /// # Usage Example
    ///
    /// The following example computes the layout of a buffer that supports scanout:
    ///
    /// ```rust
    /// # use dmabuf_allocator::Allocator;
    /// # use std::fs::OpenOptions;
    /// # use std::os::fd::AsRawFd;
    /// # const DRM_FORMAT_MOD_LINEAR: u64 = 0;
    /// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
    /// # let device = OpenOptions::new()
    /// #     .read(true)
    /// #     .write(true)
    /// #     .open("/dev/dri/renderD128")
    /// #     .unwrap();
    /// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    /// let layout = allocator
    ///     .builder()
    ///     .format(format)
    ///     .modifiers(&[DRM_FORMAT_MOD_LINEAR])
    ///     .size(1920, 1080)
    ///     .scanout(true)
    ///     .layout();
    /// ```
    #[must_use]
    pub fn scanout(mut self, v: bool) -> Self {
        self.scanout = v;
        self
    }

    /// Sets whether the buffer supports implicit sync.
    ///
    /// By default, this setting is enabled.
    ///
    /// This setting affects only the amdgpu driver.
    ///
    /// # Usage Example
    ///
    /// The following example allocates a buffer without implicit sync:
    ///
    /// ```rust
    /// # use dmabuf_allocator::Allocator;
    /// # use std::fs::OpenOptions;
    /// # use std::os::fd::AsRawFd;
    /// # const DRM_FORMAT_MOD_LINEAR: u64 = 0;
    /// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
    /// # let device = OpenOptions::new()
    /// #     .read(true)
    /// #     .write(true)
    /// #     .open("/dev/dri/renderD128")
    /// #     .unwrap();
    /// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    /// let allocation = allocator
    ///     .builder()
    ///     .format(format)
    ///     .modifiers(&[DRM_FORMAT_MOD_LINEAR])
    ///     .size(1920, 1080)
    ///     .implicit_sync(false)
    ///     .allocate()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn implicit_sync(mut self, v: bool) -> Self {
        self.implicit_sync = v;
        self
    }

    /// Sets the name of the file descriptor.
    ///
    /// By default, the file descriptor has no name.
    ///
    /// This name is visible in the `/proc` file system.
    ///
    /// The name must not be longer than 31 bytes. Otherwise, the allocation fails.
    ///
    /// # Usage Example
    ///
    /// The following example allocates a buffer with the name `example`:
    ///
    /// ```rust
    /// # use dmabuf_allocator::Allocator;
    /// # use std::fs::OpenOptions;
    /// # use std::os::fd::AsRawFd;
    /// # const DRM_FORMAT_MOD_LINEAR: u64 = 0;
    /// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
    /// # let device = OpenOptions::new()
    /// #     .read(true)
    /// #     .write(true)
    /// #     .open("/dev/dri/renderD128")
    /// #     .unwrap();
    /// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    /// let allocation = allocator
    ///     .builder()
    ///     .format(format)
    ///     .modifiers(&[DRM_FORMAT_MOD_LINEAR])
    ///     .size(1920, 1080)
    ///     .name(c"example")
    ///     .allocate()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn name(mut self, name: &'a CStr) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets whether the buffer must be importable with the invalid modifier.
    ///
    /// By default, this setting is disabled.
    ///
    /// # Usage Example
    ///
    /// The following example computes the layout of a buffer that consumers can import
    /// with the invalid modifier:
    ///
    /// ```rust
    /// # use dmabuf_allocator::Allocator;
    /// # use std::fs::OpenOptions;
    /// # use std::os::fd::AsRawFd;
    /// # const DRM_FORMAT_MOD_LINEAR: u64 = 0;
    /// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
    /// # let device = OpenOptions::new()
    /// #     .read(true)
    /// #     .write(true)
    /// #     .open("/dev/dri/renderD128")
    /// #     .unwrap();
    /// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    /// let layout = allocator
    ///     .builder()
    ///     .format(format)
    ///     .modifiers(&[DRM_FORMAT_MOD_LINEAR])
    ///     .size(1920, 1080)
    ///     .implicit_interop(true)
    ///     .layout();
    /// ```
    #[must_use]
    pub fn implicit_interop(mut self, v: bool) -> Self {
        self.implicit_interop = v;
        self
    }

    /// Sets the linear compatibility setting for allocations with the invalid modifier.
    ///
    /// By default, this setting is disabled.
    ///
    /// This setting has no effect unless you allocate the buffer with the invalid
    /// modifier, which only the amdgpu driver on GFX6 to GFX8 supports.
    ///
    /// # Usage Example
    ///
    /// The following example computes a layout with the invalid modifier and the linear
    /// compatibility setting:
    ///
    /// ```rust
    /// # use dmabuf_allocator::Allocator;
    /// # use std::fs::OpenOptions;
    /// # use std::os::fd::AsRawFd;
    /// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
    /// # let device = OpenOptions::new()
    /// #     .read(true)
    /// #     .write(true)
    /// #     .open("/dev/dri/renderD128")
    /// #     .unwrap();
    /// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    /// const DRM_FORMAT_MOD_INVALID: u64 = 0x00ff_ffff_ffff_ffff;
    ///
    /// let layout = allocator
    ///     .builder()
    ///     .format(format)
    ///     .modifiers(&[DRM_FORMAT_MOD_INVALID])
    ///     .size(1920, 1080)
    ///     .implicit_linear(true)
    ///     .layout();
    /// ```
    #[must_use]
    pub fn implicit_linear(mut self, v: bool) -> Self {
        self.linear = v;
        self
    }

    /// Returns the computed layout.
    ///
    /// The layout is the same layout that the [`AllocationBuilder::allocate`] function
    /// returns.
    ///
    /// This function always returns an error for the dumb driver because the kernel
    /// calculates the layout of dumb buffers.
    ///
    /// # Errors
    ///
    /// This function returns an error in the following cases:
    ///
    /// - The format, the modifiers, or the size are not set.
    /// - The device does not support the format or the size.
    /// - The device can use none of the modifiers for this format and size.
    /// - The buffer is too large.
    /// - The allocator uses the dumb driver.
    ///
    /// # Usage Example
    ///
    /// The following example computes a layout without allocating a buffer:
    ///
    /// ```rust
    /// # use dmabuf_allocator::Allocator;
    /// # use std::fs::OpenOptions;
    /// # use std::os::fd::AsRawFd;
    /// # const DRM_FORMAT_MOD_LINEAR: u64 = 0;
    /// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
    /// # let device = OpenOptions::new()
    /// #     .read(true)
    /// #     .write(true)
    /// #     .open("/dev/dri/renderD128")
    /// #     .unwrap();
    /// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    /// let layout = allocator
    ///     .builder()
    ///     .format(format)
    ///     .modifiers(&[DRM_FORMAT_MOD_LINEAR])
    ///     .size(1920, 1080)
    ///     .layout();
    /// if let Ok(layout) = layout {
    ///     println!("modifier: {:#x}", layout.modifier());
    /// }
    /// ```
    pub fn layout(&self) -> Result<Layout, Error> {
        let config = self.config()?;
        let layout = self.allocator.layout(&config);
        layout
            .map(Layout)
            .map_err(AllocatorError::Layout)
            .map_err(Error)
    }

    /// Allocates the buffer.
    ///
    /// # Errors
    ///
    /// This function returns an error in the following cases:
    ///
    /// - The format, the modifiers, or the size are not set.
    /// - The layout cannot be computed.
    /// - The kernel fails to create or export the buffer.
    /// - The name of the dma-buf cannot be set.
    ///
    /// # Usage Example
    ///
    /// The following example allocates a buffer and prints its file descriptor:
    ///
    /// ```rust
    /// # use dmabuf_allocator::Allocator;
    /// # use std::fs::OpenOptions;
    /// # use std::os::fd::AsRawFd;
    /// # const DRM_FORMAT_MOD_LINEAR: u64 = 0;
    /// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
    /// # let device = OpenOptions::new()
    /// #     .read(true)
    /// #     .write(true)
    /// #     .open("/dev/dri/renderD128")
    /// #     .unwrap();
    /// # let allocator = Allocator::new(device.as_raw_fd()).unwrap();
    /// let allocation = allocator
    ///     .builder()
    ///     .format(format)
    ///     .modifiers(&[DRM_FORMAT_MOD_LINEAR])
    ///     .size(1920, 1080)
    ///     .allocate()
    ///     .unwrap();
    /// println!("dma-buf: {}", allocation.dmabuf.fd());
    /// ```
    pub fn allocate(&self) -> Result<Allocation, Error> {
        let config = self.config()?;
        let allocation = self.allocator.allocate(&config).map_err(Error)?;
        Ok(Allocation {
            layout: Layout(allocation.layout),
            dmabuf: OwnedFd(allocation.dmabuf),
        })
    }
}

impl OwnedFd {
    /// Returns the file descriptor.
    ///
    /// # Usage Example
    ///
    /// The following example prints the file descriptor of a memfd:
    ///
    /// ```rust
    /// use dmabuf_allocator::udmabuf;
    /// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
    ///
    /// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
    /// let memfd = udmabuf::create_memfd(&layout).unwrap();
    /// println!("memfd: {}", memfd.fd.fd());
    /// ```
    pub fn fd(&self) -> i32 {
        self.0.fd()
    }

    /// Consumes this object without closing the file descriptor.
    ///
    /// # Usage Example
    ///
    /// The following example transfers the ownership of the file descriptor of a memfd to
    /// a `File` object:
    ///
    /// ```rust
    /// use dmabuf_allocator::udmabuf;
    /// use std::fs::File;
    /// use std::os::fd::FromRawFd;
    /// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
    ///
    /// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
    /// let memfd = udmabuf::create_memfd(&layout).unwrap();
    /// let fd = memfd.fd.unwrap();
    /// let file = unsafe { File::from_raw_fd(fd) };
    /// ```
    #[must_use]
    pub fn unwrap(mut self) -> i32 {
        self.0.take()
    }
}

impl Layout {
    /// The chosen modifier.
    ///
    /// This modifier is one of the modifiers that you pass to the
    /// [`AllocationBuilder::modifiers`] function.
    ///
    /// # Usage Example
    ///
    /// The following example reads the modifier of a linear layout:
    ///
    /// ```rust
    /// use dmabuf_allocator::udmabuf;
    /// # let format = dmabuf_allocator::get_format(b"AR24").unwrap();
    ///
    /// const DRM_FORMAT_MOD_LINEAR: u64 = 0;
    ///
    /// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
    /// assert_eq!(layout.modifier(), DRM_FORMAT_MOD_LINEAR);
    /// ```
    pub fn modifier(&self) -> u64 {
        self.0.modifier
    }

    /// The planes.
    ///
    /// # Usage Example
    ///
    /// The following example counts the planes of a linear NV12 layout:
    ///
    /// ```rust
    /// use dmabuf_allocator::udmabuf;
    ///
    /// let format = dmabuf_allocator::get_format(b"NV12").unwrap();
    /// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
    /// assert_eq!(layout.planes().len(), 2);
    /// ```
    pub fn planes(&self) -> &[PlaneLayout] {
        unsafe {
            mem::transmute::<&[dmabuf_allocator_impl::layout::PlaneLayout], &[PlaneLayout]>(
                &self.0.planes,
            )
        }
    }
}

impl PlaneLayout {
    /// The offset of the plane.
    ///
    /// # Usage Example
    ///
    /// The following example reads the offsets of the two planes of a linear NV12 layout:
    ///
    /// ```rust
    /// use dmabuf_allocator::{get_format, udmabuf};
    ///
    /// let format = get_format(b"NV12").unwrap();
    /// let layout = udmabuf::layout(format, 64, 64).unwrap();
    /// let planes = layout.planes();
    /// assert_eq!(planes[0].offset(), 0);
    /// assert_eq!(planes[1].offset(), 16384);
    /// ```
    pub fn offset(&self) -> u64 {
        self.0.offset
    }

    /// The stride of the plane.
    ///
    /// # Usage Example
    ///
    /// The following example reads the stride of a linear ARGB8888 layout that is 1920
    /// pixels wide:
    ///
    /// ```rust
    /// use dmabuf_allocator::{get_format, udmabuf};
    ///
    /// let format = get_format(b"AR24").unwrap();
    /// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
    /// assert_eq!(layout.planes()[0].stride(), 1920 * 4);
    /// ```
    pub fn stride(&self) -> u64 {
        self.0.stride
    }
}

impl Debug for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Layout")
            .field(
                "modifier",
                &fmt::from_fn(|f| write!(f, "{:#x}", self.modifier())),
            )
            .field("planes", &self.planes())
            .finish()
    }
}

impl Debug for PlaneLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlaneLayout")
            .field("offset", &self.offset())
            .field("stride", &self.stride())
            .finish()
    }
}

impl Debug for OwnedFd {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OwnedFd").field(&self.fd()).finish()
    }
}

impl Debug for AllocationBuilder<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AllocationBuilder")
            .field("format", &self.format)
            .field("modifiers", &self.modifiers)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("scanout", &self.scanout)
            .field("implicit_sync", &self.implicit_sync)
            .field("linear", &self.linear)
            .field("implicit_interop", &self.implicit_interop)
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}
