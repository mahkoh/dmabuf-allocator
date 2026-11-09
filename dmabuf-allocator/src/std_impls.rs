extern crate std;

use crate::OwnedFd;
use std::os::fd::AsFd;
use std::os::fd::AsRawFd;
use std::os::fd::BorrowedFd;
use std::os::fd::FromRawFd;
use std::os::fd::IntoRawFd;
use std::os::fd::RawFd;

impl OwnedFd {
    /// Converts this object into a `std::os::fd::OwnedFd` object.
    ///
    /// # Usage Example
    ///
    /// The following example converts the file descriptor of a memfd into a `File`
    /// object:
    ///
    /// ```rust
    /// use dmabuf_allocator::{get_format, udmabuf};
    /// use std::fs::File;
    ///
    /// let format = get_format(b"AR24").unwrap();
    /// let layout = udmabuf::layout(format, 1920, 1080).unwrap();
    /// let memfd = udmabuf::create_memfd(&layout).unwrap();
    /// let _file = File::from(memfd.fd.into_std());
    /// ```
    pub fn into_std(self) -> std::os::fd::OwnedFd {
        unsafe { std::os::fd::OwnedFd::from_raw_fd(self.unwrap()) }
    }
}

impl AsRawFd for OwnedFd {
    fn as_raw_fd(&self) -> RawFd {
        self.fd()
    }
}

impl AsFd for OwnedFd {
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.fd()) }
    }
}

impl IntoRawFd for OwnedFd {
    fn into_raw_fd(self) -> RawFd {
        self.unwrap()
    }
}

impl From<OwnedFd> for std::os::fd::OwnedFd {
    fn from(fd: OwnedFd) -> Self {
        fd.into_std()
    }
}
