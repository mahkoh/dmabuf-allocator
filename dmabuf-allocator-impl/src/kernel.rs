use crate::errno::Errno;
use crate::sys::Ioctl;
use core::ffi::c_int;
use core::ffi::c_void;

pub unsafe trait Kernel: Clone {
    unsafe fn ioctl(&self, fd: c_int, request: Ioctl, arg: *mut c_void) -> Result<c_int, Errno>;

    fn dup(&self, fd: c_int) -> Result<c_int, Errno>;

    fn close(&self, fd: c_int);

    fn open_render_node(&self, fd: c_int) -> Result<c_int, Errno>;

    fn mmap(&self, fd: c_int, len: usize, offset: u64) -> Result<*mut u8, Errno>;

    unsafe fn munmap(&self, addr: *mut u8, len: usize);

    fn wait_readable(&self, fd: c_int) -> Result<(), Errno>;

    fn is_udmabuf(&self, fd: c_int) -> bool;

    fn create_memfd(&self, size: u64) -> Result<c_int, Errno>;

    fn page_size(&self) -> u64;
}
