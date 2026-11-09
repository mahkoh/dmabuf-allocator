use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::allocator::Extra;
use crate::errno::Errno;
use crate::fd::OwnedFd;
use crate::kernel::Kernel;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::best_layout;
use crate::layout::end;
use crate::sys::_IOW;
use crate::sys::Ioctl;
use alloc::sync::Arc;
use core::ffi::c_int;
use core::ptr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UdmabufError {
    #[error("could not create the memfd")]
    Memfd(#[source] Errno),
    #[error("could not create the buffer")]
    Create(#[source] Errno),
    #[error("the buffer is too large")]
    Size,
}

const UDMABUF_FLAGS_CLOEXEC: u32 = 0x01;

#[repr(C)]
struct udmabuf_create {
    memfd: u32,
    flags: u32,
    offset: u64,
    size: u64,
}

const UDMABUF_CREATE: Ioctl = _IOW::<udmabuf_create>(b'u' as u32, 0x42);

pub(crate) struct Device<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
}

impl<K: Kernel> Device<K> {
    pub(crate) fn new(fd: &Arc<OwnedFd<K>>) -> Self {
        Device { fd: fd.clone() }
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        let extents = 1..=1 << 16;
        if !extents.contains(&config.width) || !extents.contains(&config.height) {
            return Err(LayoutError::Size);
        }
        let layout = best_layout(config, |layout| layout, |_| None, |_| None)?;
        Ok(DeviceLayout {
            layout,
            extra: Extra::None,
        })
    }

    pub(crate) fn allocate(&self, device_layout: &DeviceLayout) -> Result<c_int, UdmabufError> {
        let k = &self.fd.kernel;
        let layout = &device_layout.layout;
        let (memfd, size) = create_memfd(k, layout)?;
        create(k, self.fd.fd, memfd.fd, 0, size)
    }
}

fn memfd_size<K: Kernel>(kernel: &K, layout: &Layout) -> Option<u64> {
    end(&layout.planes).checked_next_multiple_of(kernel.page_size())
}

pub(crate) fn create_memfd<K: Kernel>(
    kernel: &K,
    layout: &Layout,
) -> Result<(OwnedFd<K>, u64), UdmabufError> {
    let size = memfd_size(kernel, layout).ok_or(UdmabufError::Size)?;
    let fd = kernel
        .create_memfd(size)
        .map(|v| OwnedFd::new(kernel, v))
        .map_err(UdmabufError::Memfd)?;
    Ok((fd, size))
}

pub(crate) fn create<K: Kernel>(
    kernel: &K,
    fd: c_int,
    memfd: c_int,
    offset: u64,
    size: u64,
) -> Result<c_int, UdmabufError> {
    let mut create = udmabuf_create {
        memfd: memfd as u32,
        flags: UDMABUF_FLAGS_CLOEXEC,
        offset,
        size,
    };
    unsafe { kernel.ioctl(fd, UDMABUF_CREATE, ptr::from_mut(&mut create).cast()) }
        .map_err(UdmabufError::Create)
}
