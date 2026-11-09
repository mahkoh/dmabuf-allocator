use allocator_trace::Traced;
use dmabuf_allocator_impl::allocator::Kernel;
use dmabuf_allocator_impl::errno::Errno;
pub use fake_kernel::Fault;
use fake_kernel::LocalMemory;
use fake_kernel::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_CREATE;
use fake_kernel::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_METADATA;
use fake_kernel::uapi::amdgpu::DRM_IOCTL_AMDGPU_INFO;
use fake_kernel::uapi::asahi::DRM_IOCTL_ASAHI_GEM_CREATE;
use fake_kernel::uapi::drm::DRM_IOCTL_GEM_CLOSE;
use fake_kernel::uapi::drm::DRM_IOCTL_MODE_CREATE_DUMB;
use fake_kernel::uapi::drm::DRM_IOCTL_PRIME_HANDLE_TO_FD;
use fake_kernel::uapi::drm::DRM_IOCTL_VERSION;
use fake_kernel::uapi::drm::drm_prime_handle;
use fake_kernel::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GEM_NEW;
use fake_kernel::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GET_PARAM;
use fake_kernel::uapi::i915::DRM_IOCTL_I915_GEM_CREATE;
use fake_kernel::uapi::i915::DRM_IOCTL_I915_GEM_CREATE_EXT;
use fake_kernel::uapi::i915::DRM_IOCTL_I915_GEM_GET_TILING;
use fake_kernel::uapi::i915::DRM_IOCTL_I915_GEM_SET_DOMAIN;
use fake_kernel::uapi::i915::DRM_IOCTL_I915_GEM_SET_TILING;
use fake_kernel::uapi::i915::DRM_IOCTL_I915_GETPARAM;
use fake_kernel::uapi::i915::DRM_IOCTL_I915_QUERY;
use fake_kernel::uapi::lima::DRM_IOCTL_LIMA_GEM_CREATE;
use fake_kernel::uapi::nouveau::DRM_IOCTL_NOUVEAU_GEM_NEW;
use fake_kernel::uapi::nouveau::DRM_IOCTL_NOUVEAU_GETPARAM;
use fake_kernel::uapi::nouveau::DRM_IOCTL_NOUVEAU_VM_INIT;
use fake_kernel::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GEM_ALLOC_NVKMS_MEMORY;
use fake_kernel::uapi::panfrost::DRM_IOCTL_PANFROST_CREATE_BO;
use fake_kernel::uapi::panfrost::DRM_IOCTL_PANFROST_GET_PARAM;
use fake_kernel::uapi::panthor::DRM_IOCTL_PANTHOR_BO_CREATE;
use fake_kernel::uapi::panthor::DRM_IOCTL_PANTHOR_DEV_QUERY;
use fake_kernel::uapi::udmabuf::UDMABUF_CREATE;
use fake_kernel::uapi::v3d::DRM_IOCTL_V3D_CREATE_BO;
use fake_kernel::uapi::vc4::DRM_IOCTL_VC4_CREATE_BO;
use fake_kernel::uapi::vc4::DRM_IOCTL_VC4_GET_TILING;
use fake_kernel::uapi::vc4::DRM_IOCTL_VC4_SET_TILING;
use fake_kernel::uapi::virtgpu::DRM_IOCTL_VIRTGPU_CONTEXT_INIT;
use fake_kernel::uapi::virtgpu::DRM_IOCTL_VIRTGPU_EXECBUFFER;
use fake_kernel::uapi::virtgpu::DRM_IOCTL_VIRTGPU_GET_CAPS;
use fake_kernel::uapi::virtgpu::DRM_IOCTL_VIRTGPU_GETPARAM;
use fake_kernel::uapi::virtgpu::DRM_IOCTL_VIRTGPU_MAP;
use fake_kernel::uapi::virtgpu::DRM_IOCTL_VIRTGPU_RESOURCE_CREATE;
use fake_kernel::uapi::virtgpu::DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB;
use fake_kernel::uapi::xe::DRM_IOCTL_XE_DEVICE_QUERY;
use fake_kernel::uapi::xe::DRM_IOCTL_XE_GEM_CREATE;
use std::alloc;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ffi::c_int;
use std::ffi::c_void;
use std::ptr;
use std::rc::Rc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;

#[cfg(test)]
mod tests;

pub const DEVICE_FD: c_int = 3;

const PAGE_SIZE: usize = 4096;

static NEXT_FILE: AtomicU64 = AtomicU64::new(1);

const DRM_IOCTLS: [u32; 3] = [
    DRM_IOCTL_VERSION,
    DRM_IOCTL_GEM_CLOSE,
    DRM_IOCTL_PRIME_HANDLE_TO_FD,
];

fn driver_ioctls(driver: &str) -> &'static [u32] {
    match driver {
        "amdgpu" => &[
            DRM_IOCTL_AMDGPU_INFO,
            DRM_IOCTL_AMDGPU_GEM_CREATE,
            DRM_IOCTL_AMDGPU_GEM_METADATA,
        ],
        "i915" => &[
            DRM_IOCTL_I915_GETPARAM,
            DRM_IOCTL_I915_QUERY,
            DRM_IOCTL_I915_GEM_CREATE,
            DRM_IOCTL_I915_GEM_CREATE_EXT,
            DRM_IOCTL_I915_GEM_GET_TILING,
            DRM_IOCTL_I915_GEM_SET_TILING,
            DRM_IOCTL_I915_GEM_SET_DOMAIN,
        ],
        "xe" => &[DRM_IOCTL_XE_DEVICE_QUERY, DRM_IOCTL_XE_GEM_CREATE],
        "nvidia-drm" => &[DRM_IOCTL_NVIDIA_GEM_ALLOC_NVKMS_MEMORY],
        "nouveau" => &[
            DRM_IOCTL_NOUVEAU_VM_INIT,
            DRM_IOCTL_NOUVEAU_GETPARAM,
            DRM_IOCTL_NOUVEAU_GEM_NEW,
        ],
        "asahi" => &[DRM_IOCTL_ASAHI_GEM_CREATE],
        "panfrost" => &[DRM_IOCTL_PANFROST_GET_PARAM, DRM_IOCTL_PANFROST_CREATE_BO],
        "panthor" => &[DRM_IOCTL_PANTHOR_DEV_QUERY, DRM_IOCTL_PANTHOR_BO_CREATE],
        "v3d" => &[DRM_IOCTL_V3D_CREATE_BO],
        "lima" => &[DRM_IOCTL_LIMA_GEM_CREATE],
        "vc4" => &[
            DRM_IOCTL_VC4_CREATE_BO,
            DRM_IOCTL_VC4_SET_TILING,
            DRM_IOCTL_VC4_GET_TILING,
        ],
        "etnaviv" => &[DRM_IOCTL_ETNAVIV_GET_PARAM, DRM_IOCTL_ETNAVIV_GEM_NEW],
        "msm" | "vkms" => &[DRM_IOCTL_MODE_CREATE_DUMB],
        "virtio_gpu" => &[
            DRM_IOCTL_VIRTGPU_GETPARAM,
            DRM_IOCTL_VIRTGPU_CONTEXT_INIT,
            DRM_IOCTL_VIRTGPU_GET_CAPS,
            DRM_IOCTL_VIRTGPU_RESOURCE_CREATE,
            DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB,
            DRM_IOCTL_VIRTGPU_MAP,
            DRM_IOCTL_VIRTGPU_EXECBUFFER,
            DRM_IOCTL_MODE_CREATE_DUMB,
        ],
        "udmabuf" => &[UDMABUF_CREATE],
        _ => panic!("the ioctls of the allocator on {driver} are not listed"),
    }
}

fn expected(driver: &str, request: u32) -> bool {
    let generic = match driver {
        "udmabuf" => request == DRM_IOCTL_VERSION,
        _ => DRM_IOCTLS.contains(&request),
    };
    generic || driver_ioctls(driver).contains(&request)
}

struct State {
    fake: Box<dyn fake_kernel::Kernel>,
    node: fake_kernel::Node,
    file: u64,
    fds: RefCell<BTreeMap<c_int, Fd>>,
    mappings: RefCell<Vec<Mapping>>,
    contents: RefCell<Vec<Content>>,
    calls: RefCell<Vec<Traced>>,
    handles: RefCell<BTreeSet<Handle>>,
    leaked: RefCell<Vec<String>>,
    unexpected: RefCell<Vec<String>>,
    faults: RefCell<Vec<Fault>>,
    occurrences: RefCell<BTreeMap<u32, usize>>,
    render_node_errno: RefCell<Option<i32>>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Handle {
    file: u64,
    handle: u32,
}

struct Overlap {
    target: usize,
    source: usize,
    len: usize,
}

fn overlap(
    target_offset: u64,
    target_len: usize,
    source_offset: u64,
    source_len: usize,
) -> Option<Overlap> {
    let start = target_offset.max(source_offset);
    let stop = (target_offset + target_len as u64).min(source_offset + source_len as u64);
    if start >= stop {
        return None;
    }
    Some(Overlap {
        target: (start - target_offset) as usize,
        source: (start - source_offset) as usize,
        len: (stop - start) as usize,
    })
}

struct Mapping {
    file: u64,
    offset: u64,
    len: usize,
    ptr: *mut u8,
}

impl Mapping {
    fn layout(len: usize) -> Option<alloc::Layout> {
        alloc::Layout::from_size_align(len.max(1), PAGE_SIZE).ok()
    }

    fn read(&self, offset: u64, data: &mut [u8]) {
        let Some(overlap) = overlap(offset, data.len(), self.offset, self.len) else {
            return;
        };
        unsafe {
            ptr::copy(
                self.ptr.add(overlap.source),
                data.as_mut_ptr().add(overlap.target),
                overlap.len,
            );
        }
    }

    fn write(&self, offset: u64, data: &[u8]) {
        let Some(overlap) = overlap(self.offset, self.len, offset, data.len()) else {
            return;
        };
        unsafe {
            ptr::copy(
                data.as_ptr().add(overlap.source),
                self.ptr.add(overlap.target),
                overlap.len,
            );
        }
    }

    fn contents(&self) -> Vec<u8> {
        let mut data = vec![0; self.len];
        self.read(self.offset, &mut data);
        data
    }
}

struct Content {
    file: u64,
    offset: u64,
    data: Vec<u8>,
}

impl Content {
    fn end(&self) -> u64 {
        self.offset + self.data.len() as u64
    }

    fn read(&self, offset: u64, data: &mut [u8]) {
        if let Some(overlap) = overlap(offset, data.len(), self.offset, self.data.len()) {
            data[overlap.target..][..overlap.len]
                .copy_from_slice(&self.data[overlap.source..][..overlap.len]);
        }
    }
}

#[derive(Copy, Clone)]
enum Fd {
    Device,
    RenderNode { file: u64 },
    DmaBuf { object: Option<u32> },
    SyncFile { file: u64 },
    Memfd { size: u64, seals: c_int },
}

struct DrmFile {
    file: u64,
    node: fake_kernel::Node,
}

impl Fd {
    fn name(&self) -> &'static str {
        match self {
            Fd::Device => "device",
            Fd::RenderNode { .. } => "render node",
            Fd::DmaBuf { .. } => "dma-buf",
            Fd::SyncFile { .. } => "sync file",
            Fd::Memfd { .. } => "memfd",
        }
    }
}

#[derive(Clone)]
pub struct FakeDevice(Rc<State>);

impl FakeDevice {
    pub fn new(fake: Box<dyn fake_kernel::Kernel>, node: fake_kernel::Node) -> Self {
        if let Some(drm) = fake.drm() {
            drm.defer_commands();
        }
        let file = NEXT_FILE.fetch_add(1, Relaxed);
        fake_kernel::open(fake.as_ref(), file, node);
        Self(Rc::new(State {
            fake,
            node,
            file,
            fds: RefCell::new(BTreeMap::from([(DEVICE_FD, Fd::Device)])),
            mappings: RefCell::new(Vec::new()),
            contents: RefCell::new(Vec::new()),
            calls: RefCell::new(Vec::new()),
            handles: RefCell::new(BTreeSet::new()),
            leaked: RefCell::new(Vec::new()),
            unexpected: RefCell::new(Vec::new()),
            faults: RefCell::new(Vec::new()),
            occurrences: RefCell::new(BTreeMap::new()),
            render_node_errno: RefCell::new(None),
        }))
    }

    pub fn fake(&self) -> &dyn fake_kernel::Kernel {
        self.0.fake.as_ref()
    }

    fn drm(&self) -> &dyn fake_kernel::Drm {
        fake_kernel::drm(self.0.fake.as_ref())
    }

    pub fn take_calls(&self) -> Vec<Traced> {
        self.0.calls.take()
    }

    pub fn fail_render_node(&self, errno: i32) {
        *self.0.render_node_errno.borrow_mut() = Some(errno);
    }

    pub fn fail_host(&self, fault: Fault) {
        self.drm().fail_host_command(fault);
    }

    pub fn fail(&self, fault: Fault) {
        self.0.faults.borrow_mut().push(fault);
    }

    pub fn leak_error(&self) -> Option<String> {
        let unexpected = self.0.unexpected.borrow();
        if !unexpected.is_empty() {
            return Some(format!("unexpected: {}", unexpected.join(", ")));
        }
        if let Some(mapping) = self.0.mappings.borrow().first() {
            return Some(format!(
                "mapping of offset {:#x} left mapped",
                mapping.offset
            ));
        }
        let fds = self.0.fds.borrow();
        if !fds.contains_key(&DEVICE_FD) {
            return Some(format!("device file descriptor {DEVICE_FD} closed"));
        }
        if let Some((fd, kind)) = fds.iter().find(|(fd, _)| **fd != DEVICE_FD) {
            return Some(format!("{} {fd} left open", kind.name()));
        }
        None
    }

    pub fn handle_error(&self) -> Option<String> {
        if let Some(leaked) = self.0.leaked.borrow().first() {
            return Some(leaked.clone());
        }
        let handles = self.0.handles.borrow();
        let open = handles.iter().find(|handle| self.is_open(handle))?;
        Some(format!("GEM handle {} left open", open.handle))
    }

    fn is_open(&self, handle: &Handle) -> bool {
        self.drm().object(handle.file, handle.handle).is_some()
    }

    fn unexpected(&self, event: String) {
        self.0.unexpected.borrow_mut().push(event);
    }

    fn open(&self, kind: Fd) -> c_int {
        let mut fds = self.0.fds.borrow_mut();
        let fd = (DEVICE_FD..).find(|fd| !fds.contains_key(fd)).unwrap();
        fds.insert(fd, kind);
        fd
    }

    fn drm_file(&self, fd: c_int) -> Option<DrmFile> {
        match *self.0.fds.borrow().get(&fd)? {
            Fd::Device => Some(DrmFile {
                file: self.0.file,
                node: self.0.node,
            }),
            Fd::RenderNode { file } => Some(DrmFile {
                file,
                node: fake_kernel::Node::Render,
            }),
            Fd::DmaBuf { .. } | Fd::SyncFile { .. } | Fd::Memfd { .. } => None,
        }
    }

    fn is_file_open(&self, file: u64) -> bool {
        let fds = self.0.fds.borrow();
        file == self.0.file
            || fds
                .values()
                .any(|fd| matches!(*fd, Fd::RenderNode { file: open } if open == file))
    }

    fn release_file(&self, file: u64) {
        let mut handles = self.0.handles.borrow_mut();
        for handle in handles.iter().filter(|handle| handle.file == file) {
            if self.is_open(handle) {
                let leaked = format!("GEM handle {} left open", handle.handle);
                self.0.leaked.borrow_mut().push(leaked);
            }
        }
        handles.retain(|handle| handle.file != file);
        self.0
            .contents
            .borrow_mut()
            .retain(|content| content.file != file);
        fake_kernel::release(self.0.fake.as_ref(), file);
    }

    fn store(&self, content: Content) {
        let mut contents = self.0.contents.borrow_mut();
        contents.retain(|old| {
            old.file != content.file || old.offset < content.offset || old.end() > content.end()
        });
        contents.push(content);
    }

    fn fault(&self, request: u32) -> Option<i32> {
        let mut occurrences = self.0.occurrences.borrow_mut();
        let occurrence = occurrences.entry(request).or_insert(0);
        *occurrence += 1;
        self.0
            .faults
            .borrow()
            .iter()
            .find(|fault| fault.request == request && fault.occurrence == *occurrence)
            .map(|fault| fault.errno)
    }
}

struct DevicePrime<'a> {
    device: &'a FakeDevice,
    file: u64,
}

impl fake_kernel::Prime for DevicePrime<'_> {
    fn sync_file(&mut self) -> Result<i32, i32> {
        Ok(self.device.open(Fd::SyncFile { file: self.file }))
    }

    fn memfd(&mut self, fd: i32) -> Option<fake_kernel::Memfd> {
        match self.device.0.fds.borrow().get(&fd) {
            Some(&Fd::Memfd { size, seals }) => Some(fake_kernel::Memfd { size, seals }),
            _ => None,
        }
    }

    fn dmabuf_fd(&mut self) -> i32 {
        self.device.open(Fd::DmaBuf { object: None })
    }

    fn read_file(&mut self, offset: u64, data: &mut [u8]) -> Result<(), i32> {
        data.fill(0);
        for content in self.device.0.contents.borrow().iter() {
            if content.file == self.file {
                content.read(offset, data);
            }
        }
        for mapping in self.device.0.mappings.borrow().iter() {
            if mapping.file == self.file {
                mapping.read(offset, data);
            }
        }
        Ok(())
    }

    fn write_file(&mut self, offset: u64, data: &[u8]) -> Result<(), i32> {
        self.device.store(Content {
            file: self.file,
            offset,
            data: data.to_vec(),
        });
        for mapping in self.device.0.mappings.borrow().iter() {
            if mapping.file == self.file {
                mapping.write(offset, data);
            }
        }
        Ok(())
    }

    unsafe fn handle_to_fd(
        &mut self,
        kernel: &dyn fake_kernel::Drm,
        file: u64,
        args: &mut drm_prime_handle,
    ) -> i64 {
        match kernel.export(file, args.handle) {
            Ok(export) => {
                args.fd = self.device.open(Fd::DmaBuf {
                    object: Some(export.object),
                });
                0
            }
            Err(errno) => -(errno as i64),
        }
    }

    unsafe fn fd_to_handle(
        &mut self,
        _kernel: &dyn fake_kernel::Drm,
        _file: u64,
        _args: &mut drm_prime_handle,
    ) -> i64 {
        panic!("the allocator does not import dma-bufs");
    }
}

unsafe impl Kernel for FakeDevice {
    unsafe fn ioctl(
        &self,
        fd: c_int,
        request: libc::Ioctl,
        arg: *mut c_void,
    ) -> Result<c_int, Errno> {
        let request = request as u32;
        let arg = arg as u64;
        let user = unsafe { LocalMemory::new() };
        let fake = self.0.fake.as_ref();
        if !expected(fake.name(), request) {
            self.unexpected(format!("ioctl {request:#x}"));
            return Err(Errno(libc::ENOTTY));
        }
        let Some(DrmFile { file, node }) = self.drm_file(fd) else {
            self.unexpected(format!("ioctl {request:#x} on file descriptor {fd}"));
            return Err(Errno(libc::EBADF));
        };
        if let Some(errno) = self.fault(request) {
            return Err(Errno(errno));
        }
        let pending = allocator_trace::decode(fake, file, request, arg, &user);
        let mut prime = DevicePrime { device: self, file };
        let ret = fake_kernel::ioctl(fake, file, node, request, arg, &user, &mut prime);
        if ret < 0 {
            return Err(Errno(-ret as i32));
        }
        for pending in pending {
            let traced = pending
                .finish(fake, file, &user)
                .expect("the ioctl arguments became unreadable");
            if let Some(handle) = traced.handle {
                self.0.handles.borrow_mut().insert(Handle { file, handle });
            }
            self.0.calls.borrow_mut().push(traced);
        }
        Ok(ret as c_int)
    }

    fn dup(&self, fd: c_int) -> Result<c_int, Errno> {
        let kind = self.0.fds.borrow().get(&fd).copied();
        let Some(kind) = kind else {
            self.unexpected(format!("dup of file descriptor {fd}"));
            return Err(Errno(libc::EBADF));
        };
        Ok(self.open(kind))
    }

    fn close(&self, fd: c_int) {
        let closed = self.0.fds.borrow_mut().remove(&fd);
        match closed {
            None => self.unexpected(format!("close of closed file descriptor {fd}")),
            Some(Fd::RenderNode { file }) if !self.is_file_open(file) => self.release_file(file),
            Some(Fd::DmaBuf {
                object: Some(object),
            }) => {
                let shared =
                    self.0.fds.borrow().values().any(
                        |fd| matches!(*fd, Fd::DmaBuf { object: Some(open) } if open == object),
                    );
                if !shared {
                    self.drm().release_export(object);
                }
            }
            Some(_) => {}
        }
    }

    fn open_render_node(&self, fd: c_int) -> Result<c_int, Errno> {
        if self.drm_file(fd).is_none() {
            self.unexpected(format!("render node of file descriptor {fd}"));
            return Err(Errno(libc::EBADF));
        }
        if let Some(errno) = *self.0.render_node_errno.borrow() {
            return Err(Errno(errno));
        }
        if !self.0.fake.render_node() {
            return Err(Errno(libc::ENOENT));
        }
        let file = NEXT_FILE.fetch_add(1, Relaxed);
        fake_kernel::open(self.0.fake.as_ref(), file, fake_kernel::Node::Render);
        Ok(self.open(Fd::RenderNode { file }))
    }

    fn mmap(&self, fd: c_int, len: usize, offset: u64) -> Result<*mut u8, Errno> {
        let Some(DrmFile { file, .. }) = self.drm_file(fd) else {
            self.unexpected(format!("mmap of file descriptor {fd}"));
            return Err(Errno(libc::EBADF));
        };
        let layout = Mapping::layout(len).ok_or(Errno(libc::EINVAL))?;
        let ptr = unsafe { alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err(Errno(libc::ENOMEM));
        }
        let mapping = Mapping {
            file,
            offset,
            len,
            ptr,
        };
        for content in self.0.contents.borrow().iter() {
            if content.file == file {
                mapping.write(content.offset, &content.data);
            }
        }
        self.0.mappings.borrow_mut().push(mapping);
        Ok(ptr)
    }

    unsafe fn munmap(&self, addr: *mut u8, len: usize) {
        let mut mappings = self.0.mappings.borrow_mut();
        let Some(index) = mappings
            .iter()
            .position(|mapping| mapping.ptr == addr && mapping.len == len)
        else {
            self.unexpected(format!("munmap of {len} bytes that are not mapped"));
            return;
        };
        let mapping = mappings.remove(index);
        if self.is_file_open(mapping.file) {
            self.store(Content {
                file: mapping.file,
                offset: mapping.offset,
                data: mapping.contents(),
            });
        }
        let layout = Mapping::layout(mapping.len).unwrap();
        unsafe {
            alloc::dealloc(mapping.ptr, layout);
        }
    }

    fn wait_readable(&self, fd: c_int) -> Result<(), Errno> {
        let Some(&Fd::SyncFile { file }) = self.0.fds.borrow().get(&fd) else {
            self.unexpected(format!("wait on file descriptor {fd}"));
            return Err(Errno(libc::EBADF));
        };
        let mut prime = DevicePrime { device: self, file };
        self.drm().wait_fence(file, &mut prime);
        Ok(())
    }

    fn is_udmabuf(&self, fd: c_int) -> bool {
        let device = matches!(self.0.fds.borrow().get(&fd), Some(Fd::Device));
        device && self.0.fake.name() == "udmabuf"
    }

    fn create_memfd(&self, size: u64) -> Result<c_int, Errno> {
        Ok(self.open(Fd::Memfd {
            size,
            seals: libc::F_SEAL_SHRINK,
        }))
    }

    fn page_size(&self) -> u64 {
        PAGE_SIZE as u64
    }
}
