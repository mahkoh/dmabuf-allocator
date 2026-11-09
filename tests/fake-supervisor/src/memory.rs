use fake_kernel::UserMemory;
use libc::EFAULT;
use libc::ENAMETOOLONG;
use libc::PATH_MAX;
use libc::iovec;
use libc::process_vm_readv;
use libc::process_vm_writev;

pub struct RemoteMemory {
    pub tid: u32,
}

impl UserMemory for RemoteMemory {
    fn read(&self, address: u64, data: &mut [u8]) -> Result<(), i32> {
        let local = iovec {
            iov_base: data.as_mut_ptr().cast(),
            iov_len: data.len(),
        };
        self.transfer(address, local, |tid, local, remote| unsafe {
            process_vm_readv(tid, local, 1, remote, 1, 0)
        })
    }

    fn write(&self, address: u64, data: &[u8]) -> Result<(), i32> {
        let local = iovec {
            iov_base: data.as_ptr().cast_mut().cast(),
            iov_len: data.len(),
        };
        self.transfer(address, local, |tid, local, remote| unsafe {
            process_vm_writev(tid, local, 1, remote, 1, 0)
        })
    }
}

impl RemoteMemory {
    fn transfer(
        &self,
        address: u64,
        local: iovec,
        copy: impl FnOnce(i32, &iovec, &iovec) -> isize,
    ) -> Result<(), i32> {
        if local.iov_len == 0 {
            return Ok(());
        }
        let remote = iovec {
            iov_base: address as usize as *mut _,
            iov_len: local.iov_len,
        };
        match copy(self.tid as i32, &local, &remote) == local.iov_len as isize {
            true => Ok(()),
            false => Err(EFAULT),
        }
    }

    pub fn path(&self, address: u64) -> Result<Vec<u8>, i32> {
        const PAGE_SIZE: usize = 4096;
        let mut page = [0u8; PAGE_SIZE];
        let mut path = Vec::new();
        let mut address = address;
        while path.len() < PATH_MAX as usize {
            let chunk = &mut page[..PAGE_SIZE - address as usize % PAGE_SIZE];
            self.read(address, chunk)?;
            if let Some(end) = chunk.iter().position(|&b| b == 0) {
                path.extend_from_slice(&chunk[..end]);
                return (path.len() < PATH_MAX as usize)
                    .then_some(path)
                    .ok_or(ENAMETOOLONG);
            }
            path.extend_from_slice(chunk);
            address += chunk.len() as u64;
        }
        Err(ENAMETOOLONG)
    }
}
