use std::io;
use std::os::fd::AsRawFd;
use std::os::fd::BorrowedFd;
use std::os::fd::FromRawFd;
use std::os::fd::OwnedFd;

#[cfg(test)]
mod tests;

#[repr(C)]
struct dma_buf_export_sync_file {
    flags: u32,
    fd: i32,
}

#[repr(C)]
struct dma_buf_import_sync_file {
    flags: u32,
    fd: i32,
}

const DMA_BUF_SYNC_WRITE: u32 = 2;

const DMA_BUF_BASE: u32 = b'b' as u32;
const DMA_BUF_IOCTL_EXPORT_SYNC_FILE: libc::Ioctl =
    libc::_IOWR::<dma_buf_export_sync_file>(DMA_BUF_BASE, 2);
const DMA_BUF_IOCTL_IMPORT_SYNC_FILE: libc::Ioctl =
    libc::_IOW::<dma_buf_import_sync_file>(DMA_BUF_BASE, 3);

pub fn export_write_fence(dmabuf: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    let mut arg = dma_buf_export_sync_file {
        flags: DMA_BUF_SYNC_WRITE,
        fd: -1,
    };
    let fd = dmabuf.as_raw_fd();
    if unsafe { libc::ioctl(fd, DMA_BUF_IOCTL_EXPORT_SYNC_FILE, &mut arg) } == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(unsafe { OwnedFd::from_raw_fd(arg.fd) })
}

pub fn import_write_fence(dmabuf: BorrowedFd<'_>, sync_file: BorrowedFd<'_>) -> io::Result<()> {
    let arg = dma_buf_import_sync_file {
        flags: DMA_BUF_SYNC_WRITE,
        fd: sync_file.as_raw_fd(),
    };
    let fd = dmabuf.as_raw_fd();
    if unsafe { libc::ioctl(fd, DMA_BUF_IOCTL_IMPORT_SYNC_FILE, &arg) } == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}
