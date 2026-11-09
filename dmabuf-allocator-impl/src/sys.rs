use crate::errno::Errno;
use crate::errno::errno;
use crate::kernel::Kernel;
use alloc::ffi::CString;
use alloc::format;
use core::ffi::CStr;
use core::ffi::c_int;
use core::ffi::c_long;
use core::ffi::c_void;
use core::mem;
use core::ptr;
use core::sync::atomic::AtomicU32;
pub(crate) use libc::_IOW;
pub(crate) use libc::_IOWR;
pub(crate) use libc::Ioctl;
use run_on_drop::on_drop;

unsafe extern "C" {
    pub(crate) safe fn getpagesize() -> c_int;
}

const UDMABUF_DEV: &CStr = c"/sys/devices/virtual/misc/udmabuf/dev";

pub(crate) fn futex_wait(futex: &AtomicU32, expected: u32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            futex.as_ptr() as c_long,
            (libc::FUTEX_WAIT | libc::FUTEX_PRIVATE_FLAG) as c_long,
            expected as c_long,
            0 as c_long,
        );
    }
}

pub(crate) fn futex_wake(futex: &AtomicU32, count: u32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            futex.as_ptr() as c_long,
            (libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG) as c_long,
            count as c_long,
        );
    }
}

#[derive(Copy, Clone, Default)]
pub struct Sys;

unsafe impl Kernel for Sys {
    unsafe fn ioctl(&self, fd: c_int, request: Ioctl, arg: *mut c_void) -> Result<c_int, Errno> {
        loop {
            let ret = unsafe { libc::ioctl(fd, request, arg) };
            if ret != -1 {
                return Ok(ret);
            }
            let errno = errno();
            if errno.0 != libc::EINTR && errno.0 != libc::EAGAIN {
                return Err(errno);
            }
        }
    }

    fn dup(&self, fd: c_int) -> Result<c_int, Errno> {
        let ret = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) };
        if ret == -1 {
            return Err(errno());
        }
        Ok(ret)
    }

    fn close(&self, fd: c_int) {
        unsafe {
            libc::close(fd);
        }
    }

    fn open_render_node(&self, fd: c_int) -> Result<c_int, Errno> {
        let mut stat: libc::stat = unsafe { mem::zeroed() };
        if unsafe { libc::fstat(fd, &mut stat) } == -1 {
            return Err(errno());
        }
        let (major, minor) = (libc::major(stat.st_rdev), libc::minor(stat.st_rdev));
        let Ok(dir) = CString::new(format!("/sys/dev/char/{major}:{minor}/device/drm")) else {
            return Err(Errno(libc::EINVAL));
        };
        let handle = unsafe { libc::opendir(dir.as_ptr()) };
        if handle.is_null() {
            return Err(errno());
        }
        let mut path = None;
        loop {
            let entry = unsafe { libc::readdir(handle) };
            if entry.is_null() {
                break;
            }
            let name = unsafe { CStr::from_ptr((&raw const (*entry).d_name).cast()).to_bytes() };
            if name.starts_with(b"renderD") {
                path = CString::new([b"/dev/dri/", name].concat()).ok();
                break;
            }
        }
        unsafe {
            libc::closedir(handle);
        }
        let path = path.ok_or(Errno(libc::ENOENT))?;
        let flags = libc::O_RDWR | libc::O_CLOEXEC;
        let ret = unsafe { libc::openat(libc::AT_FDCWD, path.as_ptr(), flags) };
        if ret == -1 {
            return Err(errno());
        }
        Ok(ret)
    }

    fn mmap(&self, fd: c_int, len: usize, offset: u64) -> Result<*mut u8, Errno> {
        let offset = libc::off64_t::try_from(offset).map_err(|_| Errno(libc::EOVERFLOW))?;
        let ret = unsafe {
            libc::mmap64(
                ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                offset,
            )
        };
        if ret == libc::MAP_FAILED {
            return Err(errno());
        }
        Ok(ret.cast())
    }

    unsafe fn munmap(&self, addr: *mut u8, len: usize) {
        unsafe {
            libc::munmap(addr.cast(), len);
        }
    }

    fn wait_readable(&self, fd: c_int) -> Result<(), Errno> {
        let mut pollfd = libc::pollfd {
            fd,
            events: libc::POLLIN,
            revents: 0,
        };
        loop {
            let ret = unsafe { libc::ppoll(&mut pollfd, 1, ptr::null(), ptr::null()) };
            if ret > 0 {
                if pollfd.revents & (libc::POLLERR | libc::POLLNVAL) != 0 {
                    return Err(Errno(libc::EINVAL));
                }
                return Ok(());
            }
            if ret == -1 {
                let errno = errno();
                if errno.0 != libc::EINTR && errno.0 != libc::EAGAIN {
                    return Err(errno);
                }
            }
        }
    }

    fn is_udmabuf(&self, fd: c_int) -> bool {
        let mut stat: libc::stat = unsafe { mem::zeroed() };
        if unsafe { libc::fstat(fd, &mut stat) } == -1 {
            return false;
        }
        if stat.st_mode & libc::S_IFMT != libc::S_IFCHR {
            return false;
        }
        let (major, minor) = (libc::major(stat.st_rdev), libc::minor(stat.st_rdev));
        let expected = format!("{major}:{minor}\n");
        let flags = libc::O_RDONLY | libc::O_CLOEXEC;
        let dev = unsafe { libc::openat(libc::AT_FDCWD, UDMABUF_DEV.as_ptr(), flags) };
        if dev == -1 {
            return false;
        }
        let _close = on_drop(|| self.close(dev));
        let mut actual = [0u8; 32];
        let len = unsafe { libc::read(dev, actual.as_mut_ptr().cast(), actual.len()) };
        len >= 0 && actual[..len as usize] == *expected.as_bytes()
    }

    fn create_memfd(&self, size: u64) -> Result<c_int, Errno> {
        let flags = libc::MFD_CLOEXEC | libc::MFD_ALLOW_SEALING;
        let fd = unsafe { libc::memfd_create(c"dmabuf-allocator".as_ptr(), flags) };
        if fd == -1 {
            return Err(errno());
        }
        let close = on_drop(|| self.close(fd));
        let size = libc::off64_t::try_from(size).map_err(|_| Errno(libc::EOVERFLOW))?;
        if unsafe { libc::ftruncate64(fd, size) } == -1 {
            return Err(errno());
        }
        if unsafe { libc::fcntl(fd, libc::F_ADD_SEALS, libc::F_SEAL_SHRINK) } == -1 {
            return Err(errno());
        }
        close.forget();
        Ok(fd)
    }

    fn page_size(&self) -> u64 {
        getpagesize() as u64
    }
}
