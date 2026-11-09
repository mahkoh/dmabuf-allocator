use crate::Drm;
use crate::Fault;
use crate::LocalMemory;
use crate::Node;
use crate::Prime;
use crate::amd::devices::GfxLevel;
use crate::uapi::drm::drm_prime_handle;
use crate::uapi::ioctl::_IOC_SIZE;
use crate::uapi::vdrm::vdrm_ccmd_req;
use crate::uapi::vdrm::vdrm_shmem;
use crate::uapi::vdrm::virgl_renderer_capset_drm;
use crate::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_MAPPABLE;
use crate::user::from_bytes;
use crate::user::put;
use crate::version::DriverVersion;
use libc::EFAULT;
use libc::EINVAL;
use libc::ENOMEM;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::mem::offset_of;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;

pub(crate) struct Blob {
    pub(crate) res_id: u32,
    pub(crate) blob_id: u64,
    pub(crate) size: u64,
    pub(crate) blob_flags: u32,
    pub(crate) mmap_offset: u64,
}

struct BackendPrime;

impl Prime for BackendPrime {
    fn sync_file(&mut self) -> Result<i32, i32> {
        unimplemented!("sync files on the native context host");
    }

    fn read_file(&mut self, _offset: u64, _data: &mut [u8]) -> Result<(), i32> {
        unimplemented!("mapped memory on the native context host");
    }

    fn write_file(&mut self, _offset: u64, _data: &[u8]) -> Result<(), i32> {
        unimplemented!("mapped memory on the native context host");
    }

    unsafe fn handle_to_fd(
        &mut self,
        _kernel: &dyn Drm,
        _file: u64,
        _args: &mut drm_prime_handle,
    ) -> i64 {
        unimplemented!("PRIME export on the native context host");
    }

    unsafe fn fd_to_handle(
        &mut self,
        _kernel: &dyn Drm,
        _file: u64,
        _args: &mut drm_prime_handle,
    ) -> i64 {
        unimplemented!("PRIME import on the native context host");
    }
}

pub(crate) struct Backend<K> {
    pub(crate) kernel: K,
    next_file: AtomicU64,
}

impl<K: Drm> Backend<K> {
    pub(crate) fn new(kernel: K, first_file: u64) -> Self {
        Self {
            kernel,
            next_file: AtomicU64::new(first_file),
        }
    }

    pub(crate) fn open(&self) -> u64 {
        self.next_file.fetch_add(1, Relaxed)
    }

    pub(crate) fn release(&self, file: u64) {
        self.kernel.release_file(file);
    }

    pub(crate) unsafe fn ioctl_raw<T>(
        &self,
        file: u64,
        cmd: u32,
        arg: *mut T,
        fault: Option<Fault>,
    ) -> i64 {
        assert_eq!(_IOC_SIZE(cmd) as usize, size_of::<T>());
        unsafe { self.ioctl_address(file, cmd, arg as u64, fault) }
    }

    pub(crate) unsafe fn ioctl_bytes(
        &self,
        file: u64,
        cmd: u32,
        payload: &mut [u8],
        fault: Option<Fault>,
    ) -> i64 {
        assert_eq!(_IOC_SIZE(cmd) as usize, payload.len());
        unsafe { self.ioctl_address(file, cmd, payload.as_mut_ptr() as u64, fault) }
    }

    unsafe fn ioctl_address(&self, file: u64, cmd: u32, arg: u64, fault: Option<Fault>) -> i64 {
        if let Some(fault) = fault {
            return -(fault.errno as i64);
        }
        let user = unsafe { LocalMemory::new() };
        crate::ioctl(
            &self.kernel,
            file,
            Node::Render,
            cmd,
            arg,
            &user,
            &mut BackendPrime,
        )
    }
}

pub(crate) struct Shmem {
    offset: u64,
    size: u64,
    header_size: u64,
}

impl Shmem {
    pub(crate) fn field(&self, offset: usize) -> u64 {
        self.offset + offset as u64
    }

    fn rsp_mem(&self) -> u64 {
        self.offset + self.header_size
    }

    fn rsp_mem_size(&self) -> u64 {
        self.size - self.header_size
    }
}

pub(crate) fn attach_shmem(
    shmem: &mut Option<Shmem>,
    prime: &mut dyn Prime,
    blob: &Blob,
    header_size: usize,
) -> bool {
    let header_size = header_size as u64;
    if blob.blob_flags != VIRTGPU_BLOB_FLAG_USE_MAPPABLE
        || shmem.is_some()
        || blob.size < header_size
        || blob.size > u32::MAX as u64
    {
        return false;
    }
    *shmem = Some(Shmem {
        offset: blob.mmap_offset,
        size: blob.size,
        header_size,
    });
    let rsp_mem_offset = (header_size as u32).to_ne_bytes();
    prime
        .write_file(
            blob.mmap_offset + offset_of!(vdrm_shmem, rsp_mem_offset) as u64,
            &rsp_mem_offset,
        )
        .unwrap();
    true
}

fn check_shm_bounds(shmem: Option<&Shmem>, hdr: &vdrm_ccmd_req, len: usize) -> bool {
    let rsp_mem_size = shmem.map_or(0, Shmem::rsp_mem_size);
    let off = hdr.rsp_off as u64;
    off <= rsp_mem_size && len as u64 <= rsp_mem_size - off
}

pub(crate) fn response(
    shmem: Option<&Shmem>,
    hdr: &vdrm_ccmd_req,
    len: usize,
) -> Result<Vec<u8>, i32> {
    if !check_shm_bounds(shmem, hdr, len) {
        return Err(ENOMEM);
    }
    let mut response = vec![0u8; len];
    response[..4].copy_from_slice(&(len as u32).to_ne_bytes());
    Ok(response)
}

pub(crate) fn drm_capset(
    wire_format_version: u32,
    version: &DriverVersion,
    context_type: u32,
    u: &[u8],
) -> Vec<u8> {
    let mut capset = vec![0u8; size_of::<virgl_renderer_capset_drm>()];
    put(
        &mut capset,
        offset_of!(virgl_renderer_capset_drm, wire_format_version),
        wire_format_version,
    );
    put(
        &mut capset,
        offset_of!(virgl_renderer_capset_drm, version_major),
        version.major as u32,
    );
    put(
        &mut capset,
        offset_of!(virgl_renderer_capset_drm, version_minor),
        version.minor as u32,
    );
    put(
        &mut capset,
        offset_of!(virgl_renderer_capset_drm, version_patchlevel),
        version.patchlevel as u32,
    );
    put(
        &mut capset,
        offset_of!(virgl_renderer_capset_drm, context_type),
        context_type,
    );
    capset[offset_of!(virgl_renderer_capset_drm, u)..][..u.len()].copy_from_slice(u);
    capset
}

struct CommandFault {
    fault: Fault,
    seen: usize,
}

pub(crate) trait Renderer: Send + Sync {
    type Context: Send;
    const CONTEXT_TYPE: u32;
    const CCMD_ALIGNMENT: u32;
    const DISPATCH_SIZE: u32;

    fn capset(&self) -> &[u8];
    fn gfx_level(&self) -> Option<GfxLevel> {
        None
    }
    fn create_context(&self) -> Self::Context;
    fn destroy_context(&self, context: Self::Context);
    fn shmem(context: &Self::Context) -> Option<&Shmem>;
    fn get_blob(&self, context: &mut Self::Context, prime: &mut dyn Prime, blob: Blob);
    fn dispatch(
        &self,
        context: &mut Self::Context,
        prime: &mut dyn Prime,
        hdr: &vdrm_ccmd_req,
        request: &[u8],
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32>;
}

pub(crate) trait Host: Send + Sync {
    fn capset(&self) -> &[u8];
    fn context_type(&self) -> u32;
    fn gfx_level(&self) -> Option<GfxLevel>;
    fn create_context(&self, file: u64);
    fn destroy_context(&self, file: u64);
    fn submit(&self, file: u64, prime: &mut dyn Prime, command: &[u8]);
    fn get_blob(&self, file: u64, prime: &mut dyn Prime, blob: Blob);
    fn fail_command(&self, fault: Fault);
}

pub(crate) struct NativeHost<R: Renderer> {
    renderer: R,
    contexts: Mutex<BTreeMap<u64, R::Context>>,
    faults: Mutex<Vec<CommandFault>>,
}

impl<R: Renderer> NativeHost<R> {
    pub(crate) fn new(renderer: R) -> Self {
        Self {
            renderer,
            contexts: Mutex::new(BTreeMap::new()),
            faults: Mutex::new(Vec::new()),
        }
    }

    fn command_fault(&self, cmd: u32) -> Option<Fault> {
        let mut found = None;
        for command in self.faults.lock().iter_mut() {
            if command.fault.request != cmd {
                continue;
            }
            command.seen += 1;
            if command.seen == command.fault.occurrence && found.is_none() {
                found = Some(command.fault);
            }
        }
        found
    }

    fn dispatch(
        &self,
        context: &mut R::Context,
        prime: &mut dyn Prime,
        hdr: &vdrm_ccmd_req,
        request: &[u8],
    ) -> Result<(), i32> {
        if hdr.cmd >= R::DISPATCH_SIZE || hdr.cmd == 0 {
            return Err(EINVAL);
        }
        let fault = self.command_fault(hdr.cmd);
        let unanswered = fault.is_some_and(|fault| !fault.response);
        let backend_fault = fault.filter(|fault| fault.response);
        let response = self
            .renderer
            .dispatch(context, prime, hdr, request, backend_fault)?;
        let Some(shmem) = R::shmem(context) else {
            return Ok(());
        };
        if let Some(mut response) = response.filter(|_| !unanswered) {
            if !check_shm_bounds(Some(shmem), hdr, response.len()) {
                return Err(EFAULT);
            }
            let address = shmem.rsp_mem() + hdr.rsp_off as u64;
            let mut untrusted = [0u8; 4];
            prime.read_file(address, &mut untrusted).unwrap();
            let len = response.len().min(u32::from_ne_bytes(untrusted) as usize);
            response.truncate(len);
            prime.write_file(address, &response).unwrap();
            prime
                .write_file(address, &(len as u32).to_ne_bytes())
                .unwrap();
        }
        let seqno = shmem.field(offset_of!(vdrm_shmem, seqno));
        prime.write_file(seqno, &hdr.seqno.to_ne_bytes()).unwrap();
        Ok(())
    }
}

impl<R: Renderer> Host for NativeHost<R> {
    fn capset(&self) -> &[u8] {
        self.renderer.capset()
    }

    fn context_type(&self) -> u32 {
        R::CONTEXT_TYPE
    }

    fn gfx_level(&self) -> Option<GfxLevel> {
        self.renderer.gfx_level()
    }

    fn create_context(&self, file: u64) {
        let context = self.renderer.create_context();
        let previous = self.contexts.lock().insert(file, context);
        assert!(previous.is_none(), "file {file} already has a context");
    }

    fn destroy_context(&self, file: u64) {
        if let Some(context) = self.contexts.lock().remove(&file) {
            self.renderer.destroy_context(context);
        }
    }

    fn submit(&self, file: u64, prime: &mut dyn Prime, mut buffer: &[u8]) {
        let mut contexts = self.contexts.lock();
        let context = contexts
            .get_mut(&file)
            .unwrap_or_else(|| panic!("file {file} has no native context"));
        if buffer.len() > u32::MAX as usize {
            return;
        }
        while buffer.len() >= size_of::<vdrm_ccmd_req>() {
            let hdr: vdrm_ccmd_req = from_bytes(buffer);
            let len = hdr.len as usize;
            if len > buffer.len()
                || len < size_of::<vdrm_ccmd_req>()
                || hdr.len & (R::CCMD_ALIGNMENT - 1) != 0
            {
                return;
            }
            if hdr.rsp_off & (R::CCMD_ALIGNMENT - 1) != 0 {
                return;
            }
            if self.dispatch(context, prime, &hdr, &buffer[..len]).is_err() {
                return;
            }
            buffer = &buffer[len..];
        }
    }

    fn get_blob(&self, file: u64, prime: &mut dyn Prime, blob: Blob) {
        let mut contexts = self.contexts.lock();
        let context = contexts
            .get_mut(&file)
            .unwrap_or_else(|| panic!("file {file} has no native context"));
        self.renderer.get_blob(context, prime, blob);
    }

    fn fail_command(&self, fault: Fault) {
        self.faults.lock().push(CommandFault { fault, seen: 0 });
    }
}
