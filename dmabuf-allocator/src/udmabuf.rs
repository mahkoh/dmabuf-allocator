//! Utilities for creating udmabufs.
//!
//! # Usage Example
//!
//! The following example creates a dma-buf from a new memfd:
//!
//! ```rust
//! use dmabuf_allocator::{get_format, udmabuf};
//! use std::fs::File;
//! use std::os::fd::AsRawFd;
//!
//! let format = get_format(b"AR24").unwrap();
//! let layout = udmabuf::layout(format, 1920, 1080).unwrap();
//! let memfd = udmabuf::create_memfd(&layout).unwrap();
//! let device = File::open("/dev/udmabuf").unwrap();
//! let _dmabuf = udmabuf::create_dmabuf(device.as_raw_fd(), memfd.fd.fd(), 0, memfd.size).unwrap();
//! ```

use crate::Error;
use crate::Format;
use crate::Layout;
use crate::OwnedFd;
use dmabuf_allocator_impl::allocator::AllocatorError;

/// A memfd.
///
/// # Usage Example
///
/// ```rust
/// use dmabuf_allocator::udmabuf;
///
/// let format = dmabuf_allocator::get_format(b"AR24").unwrap();
/// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
/// let _memfd = udmabuf::create_memfd(&layout).unwrap();
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub struct Memfd {
    /// The file descriptor.
    pub fd: OwnedFd,
    /// The size of the file descriptor.
    pub size: u64,
}

/// Computes the linear layout for a given format and size.
///
/// # Errors
///
/// This function returns an error if the layout is too large.
///
/// # Usage Example
///
/// The following example computes the linear layout of an ARGB8888 buffer:
///
/// ```rust
/// use dmabuf_allocator::udmabuf;
///
/// let format = dmabuf_allocator::get_format(b"AR24").unwrap();
/// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
/// assert_eq!(layout.planes().len(), 1);
/// ```
pub fn layout(format: Format, width: u32, height: u32) -> Result<Layout, Error> {
    let layout = dmabuf_allocator_impl::layout::linear::compute_layout(format.0.0, width, height);
    layout
        .map(Layout)
        .map_err(AllocatorError::Layout)
        .map_err(Error)
}

/// Creates a memfd with a given layout.
///
/// You can pass the created memfd to the [`create_dmabuf`] function.
///
/// # Errors
///
/// This function returns an error if the memfd cannot be created, resized, or sealed.
///
/// # Usage Example
///
/// The following example creates a memfd that is large enough for an ARGB8888 buffer:
///
/// ```rust
/// use dmabuf_allocator::{get_format, udmabuf};
///
/// let format = get_format(b"AR24").unwrap();
/// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
/// let _memfd = udmabuf::create_memfd(&layout).unwrap();
/// ```
pub fn create_memfd(layout: &Layout) -> Result<Memfd, Error> {
    let (fd, size) = dmabuf_allocator_impl::allocator::create_memfd(&layout.0).map_err(Error)?;
    Ok(Memfd {
        fd: OwnedFd(fd),
        size,
    })
}

/// Creates a dma-buf from a memfd.
///
/// The `udmabuf_fd` argument must refer to the `/dev/udmabuf` device.
///
/// # Errors
///
/// This function returns an error if the kernel fails to create the dma-buf.
///
/// # Usage Example
///
/// The following example creates a dma-buf that covers a whole memfd:
///
/// ```rust
/// use dmabuf_allocator::{get_format, udmabuf};
/// use std::fs::File;
/// use std::os::fd::AsRawFd;
///
/// let format = get_format(b"AR24").unwrap();
/// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
/// let memfd = udmabuf::create_memfd(&layout).unwrap();
/// let device = File::open("/dev/udmabuf").unwrap();
/// let fd = memfd.fd.fd();
/// let size = memfd.size;
/// let _dmabuf = udmabuf::create_dmabuf(device.as_raw_fd(), fd, 0, size).unwrap();
/// ```
pub fn create_dmabuf(
    udmabuf_fd: i32,
    memfd: i32,
    offset: u64,
    size: u64,
) -> Result<OwnedFd, Error> {
    dmabuf_allocator_impl::allocator::create_udmabuf(udmabuf_fd, memfd, offset, size)
        .map(OwnedFd)
        .map_err(Error)
}
