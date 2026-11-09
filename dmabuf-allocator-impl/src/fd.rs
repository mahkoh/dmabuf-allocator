use crate::allocator::Kernel;
use crate::allocator::Sys;
use core::ffi::c_int;

pub struct OwnedFd<K: Kernel = Sys> {
    pub(crate) kernel: K,
    pub(crate) fd: c_int,
    pub(crate) taken: bool,
}

impl<K: Kernel> OwnedFd<K> {
    pub(crate) fn new(k: &K, fd: c_int) -> Self {
        Self {
            kernel: k.clone(),
            fd,
            taken: false,
        }
    }

    pub fn fd(&self) -> c_int {
        self.fd
    }

    pub fn take(&mut self) -> c_int {
        self.taken = true;
        self.fd
    }
}

impl<K: Kernel> Drop for OwnedFd<K> {
    fn drop(&mut self) {
        if self.taken {
            return;
        }
        self.kernel.close(self.fd);
    }
}
