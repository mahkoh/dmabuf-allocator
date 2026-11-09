use crate::drivers::virtio_gpu::DRM_IOCTL_VIRTGPU_GET_CAPS;
use crate::drivers::virtio_gpu::VirtioHandle;
use crate::drivers::virtio_gpu::drm_virtgpu_get_caps;
use crate::drivers::virtio_gpu::get_param;
use crate::drm;
use crate::drm::DRM_COMMAND_BASE;
use crate::drm::DRM_IOCTL_BASE;
use crate::drm::GemHandle;
use crate::errno::Errno;
use crate::fd::OwnedFd;
use crate::kernel::Kernel;
use crate::layout::align;
use crate::mutex::Mutex;
use crate::mutex::MutexGuard;
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use crate::vdrm::rsp_ring::RspRing;
use crate::vdrm::rsp_ring::RspSlot;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::iter;
use core::mem;
use core::mem::MaybeUninit;
use core::mem::offset_of;
use core::ptr;
use core::slice;
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering::Relaxed;
use run_on_drop::on_drop;
use thiserror::Error;

mod rsp_ring;

#[derive(Debug, Error)]
pub enum VdrmError {
    #[error("could not create the context")]
    ContextInit(#[source] Errno),
    #[error("could not create the shared memory")]
    ShmemBlob(#[source] Errno),
    #[error("could not get the shared memory offset")]
    Map(#[source] Errno),
    #[error("could not mmap the shared memory")]
    Mmap(#[source] Errno),
    #[error("the host response offset is invalid")]
    RspMemOffset,
    #[error("could not create the blob")]
    Blob(#[source] Errno),
    #[error("the request has an invalid size")]
    RequestSize,
    #[error("the response does not fit the shared memory")]
    ResponseSize,
    #[error("the virtio connection is broken")]
    Broken(#[source] Errno),
    #[error("could not submit the command")]
    Execbuffer(#[source] Errno),
    #[error("could not wait for the fence")]
    Fence(#[source] Errno),
}

const VIRTGPU_DRM_CAPSET_DRM: u32 = 6;
pub(crate) const VIRTGPU_DRM_CONTEXT_AMDGPU: u32 = 2;
pub(crate) const VIRTGPU_DRM_CONTEXT_I915: u32 = 3;
pub(crate) const VIRTGPU_DRM_CONTEXT_ASAHI: u32 = 4;
pub(crate) const VIRTGPU_DRM_CONTEXT_PANFROST: u32 = 5;
pub(crate) const VIRTGPU_CONTEXT_PARAM_CAPSET_ID: u64 = 0x0001;
const VIRTGPU_CONTEXT_PARAM_NUM_RINGS: u64 = 0x0002;
const NUM_RINGS: u64 = 64;
const VIRTGPU_BLOB_MEM_HOST3D: u32 = 0x0002;
pub(crate) const VIRTGPU_BLOB_FLAG_USE_MAPPABLE: u32 = 0x0001;
pub(crate) const VIRTGPU_BLOB_FLAG_USE_SHAREABLE: u32 = 0x0002;
pub(crate) const VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE: u32 = 0x0004;
pub(crate) const DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING: u32 = 0x0001;
const VIRTGPU_EXECBUF_FENCE_FD_OUT: u32 = 0x02;
const VIRTGPU_EXECBUF_RING_IDX: u32 = 0x04;
const SHMEM_SIZE: usize = 0x4000;
const VIRTGPU_PARAM_CROSS_DEVICE: u64 = 5;
const VIRTGPU_PARAM_BLOB_ALIGNMENT: u64 = 9;
const RSP_ALIGNMENT: usize = 8;
const REQUEST_ALIGNMENT: usize = 4;
const REQBUF_SIZE: usize = 0x4000;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct amdgpu_buffer_size_alignments {
    pub(crate) size_local: u64,
    pub(crate) size_remote: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct amdgpu_gpu_info {
    pub(crate) asic_id: u32,
    chip_rev: u32,
    pub(crate) chip_external_rev: u32,
    pub(crate) family_id: u32,
    pub(crate) ids_flags: u64,
    max_engine_clk: u64,
    max_memory_clk: u64,
    pub(crate) num_shader_engines: u32,
    pub(crate) num_shader_arrays_per_engine: u32,
    avail_quad_shader_pipes: u32,
    max_quad_shader_pipes: u32,
    cache_entries_per_quad_pipe: u32,
    num_hw_gfx_contexts: u32,
    pub(crate) rb_pipes: u32,
    enabled_rb_pipes_mask: u32,
    gpu_counter_freq: u32,
    backend_disable: [u32; 4],
    mc_arb_ramcfg: u32,
    pub(crate) gb_addr_cfg: u32,
    pub(crate) gb_tile_mode: [u32; 32],
    pub(crate) gb_macro_tile_mode: [u32; 16],
    pa_sc_raster_cfg: [u32; 4],
    pa_sc_raster_cfg1: [u32; 4],
    cu_active_number: u32,
    cu_ao_mask: u32,
    pub(crate) cu_bitmap: [[u32; 4]; 4],
    vram_type: u32,
    vram_bit_width: u32,
    ce_ram_size: u32,
    vce_harvest_config: u32,
    pci_rev_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CapsetDrmMsm {
    has_cached_coherent: u32,
    priorities: u32,
    va_start: u64,
    va_size: u64,
    gpu_id: u32,
    gmem_size: u32,
    gmem_base: u64,
    chip_id: u64,
    max_freq: u32,
    highest_bank_bit: u32,
    ubwc_swizzle: u64,
    macrotile_mode: u64,
    has_raytracing: u32,
    has_preemption: u32,
    uche_trap_base: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct CapsetDrmAmdgpu {
    address32_hi: u32,
    has_vm_always_valid_pad: u32,
    pub(crate) alignments: amdgpu_buffer_size_alignments,
    pub(crate) gpu_info: amdgpu_gpu_info,
    marketing_name: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct CapsetDrmIntel {
    pci_bus: u32,
    pci_dev: u32,
    pci_func: u32,
    pci_revision_id: u32,
    pci_domain: u32,
    pub(crate) pci_device_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union CapsetDrmU {
    msm: CapsetDrmMsm,
    pub(crate) amdgpu: CapsetDrmAmdgpu,
    pub(crate) intel: CapsetDrmIntel,
}

#[repr(C)]
pub(crate) struct virgl_renderer_capset_drm {
    wire_format_version: u32,
    version_major: u32,
    pub(crate) version_minor: u32,
    version_patchlevel: u32,
    pub(crate) context_type: u32,
    pad: u32,
    pub(crate) u: CapsetDrmU,
}

#[repr(C)]
pub(crate) struct drm_virtgpu_context_set_param {
    pub(crate) param: u64,
    pub(crate) value: u64,
}

#[repr(C)]
pub(crate) struct drm_virtgpu_context_init {
    pub(crate) num_params: u32,
    pub(crate) pad: u32,
    pub(crate) ctx_set_params: u64,
}

#[repr(C)]
struct drm_virtgpu_resource_create_blob {
    blob_mem: u32,
    blob_flags: u32,
    bo_handle: GemHandle,
    res_handle: VirtioHandle,
    size: u64,
    pad: u32,
    cmd_size: u32,
    cmd: u64,
    blob_id: u64,
    blob_hints: u32,
    pad2: u32,
}

#[repr(C)]
struct drm_virtgpu_map {
    offset: u64,
    handle: GemHandle,
    pad: u32,
}

#[repr(C)]
#[derive(Default)]
struct drm_virtgpu_execbuffer {
    flags: u32,
    size: u32,
    command: u64,
    bo_handles: u64,
    num_bo_handles: u32,
    fence_fd: i32,
    ring_idx: u32,
    syncobj_stride: u32,
    num_in_syncobjs: u32,
    num_out_syncobjs: u32,
    in_syncobjs: u64,
    out_syncobjs: u64,
}

#[repr(C)]
struct drm_virtgpu_execbuffer_syncobj {
    handle: u32,
    flags: u32,
    point: u64,
}

#[repr(C)]
struct vdrm_shmem {
    seqno: u32,
    rsp_mem_offset: u32,
}

#[repr(C)]
pub(crate) struct vdrm_ccmd_req {
    pub(crate) cmd: u32,
    pub(crate) len: u32,
    pub(crate) seqno: u32,
    pub(crate) rsp_off: u32,
}

impl vdrm_ccmd_req {
    pub(crate) fn new(cmd: u32) -> Self {
        Self {
            cmd,
            len: 0,
            seqno: 0,
            rsp_off: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct vdrm_ccmd_rsp {
    pub(crate) len: u32,
}

const DRM_IOCTL_VIRTGPU_MAP: Ioctl =
    _IOWR::<drm_virtgpu_map>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x01);
const DRM_IOCTL_VIRTGPU_EXECBUFFER: Ioctl =
    _IOWR::<drm_virtgpu_execbuffer>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x02);
const DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB: Ioctl =
    _IOWR::<drm_virtgpu_resource_create_blob>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x0a);
pub(crate) const DRM_IOCTL_VIRTGPU_CONTEXT_INIT: Ioctl =
    _IOWR::<drm_virtgpu_context_init>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x0b);

pub(crate) fn capset<K: Kernel>(fd: &OwnedFd<K>) -> Option<virgl_renderer_capset_drm> {
    let mut caps = unsafe { mem::zeroed::<virgl_renderer_capset_drm>() };
    let mut get = drm_virtgpu_get_caps {
        cap_set_id: VIRTGPU_DRM_CAPSET_DRM,
        cap_set_ver: 0,
        addr: ptr::from_mut(&mut caps).expose_provenance() as u64,
        size: size_of::<virgl_renderer_capset_drm>() as u32,
        pad: 0,
    };
    unsafe {
        drm::ioctl(fd, DRM_IOCTL_VIRTGPU_GET_CAPS, &mut get).ok()?;
    }
    Some(caps)
}

pub(crate) struct BlobCreate {
    pub(crate) size: u64,
    pub(crate) blob_flags: u32,
    pub(crate) blob_id: u64,
    pub(crate) blob_hints: u32,
}

pub(crate) struct Blob {
    pub(crate) bo_handle: GemHandle,
    pub(crate) res_handle: VirtioHandle,
}

struct Queue<K: Kernel> {
    data: [MaybeUninit<u8>; REQBUF_SIZE],
    len: usize,
    seqno: u32,
    slots: Vec<RspSlot>,
    fence: Option<Arc<OwnedFd<K>>>,
}

impl<K: Kernel> Queue<K> {
    fn write<T>(&mut self, req: &T, parts: &[Part<'_>], len: usize, rsp_off: u32) {
        const {
            assert!(size_of::<T>() >= size_of::<vdrm_ccmd_req>());
        }
        self.seqno = self.seqno.wrapping_add(1);
        let dst = &mut self.data[self.len..][..len];
        let mut offset = 0;
        for src in iter::once(bytes(req)).chain(parts.iter().map(|part| part.0)) {
            dst[offset..][..src.len()].copy_from_slice(src);
            offset += src.len();
        }
        let mut set = |offset: usize, value: u32| {
            dst[offset..][..size_of::<u32>()].copy_from_slice(bytes(&value));
        };
        set(offset_of!(vdrm_ccmd_req, len), len as u32);
        set(offset_of!(vdrm_ccmd_req, seqno), self.seqno);
        set(offset_of!(vdrm_ccmd_req, rsp_off), rsp_off);
    }
}

pub(crate) struct Vdrm<K: Kernel> {
    pub(crate) fd: Arc<OwnedFd<K>>,
    shmem: *mut u8,
    shmem_size: usize,
    shmem_handle: GemHandle,
    rsp_mem_offset: usize,
    rsp_ring: RspRing,
    queue: Mutex<Queue<K>>,
    next_blob_id: AtomicU64,
    blob_alignment: u64,
    cross_device: bool,
}

unsafe impl<K: Kernel + Send + Sync> Send for Vdrm<K> {}

unsafe impl<K: Kernel + Send + Sync> Sync for Vdrm<K> {}

fn create_blob<K: Kernel>(
    fd: &OwnedFd<K>,
    params: &BlobCreate,
    cmd: &[MaybeUninit<u8>],
) -> Result<Blob, Errno> {
    let mut blob = drm_virtgpu_resource_create_blob {
        blob_mem: VIRTGPU_BLOB_MEM_HOST3D,
        blob_flags: params.blob_flags,
        bo_handle: GemHandle(0),
        res_handle: VirtioHandle(0),
        size: params.size,
        pad: 0,
        cmd_size: cmd.len() as u32,
        cmd: cmd.as_ptr().expose_provenance() as u64,
        blob_id: params.blob_id,
        blob_hints: params.blob_hints,
        pad2: 0,
    };
    unsafe {
        drm::ioctl(fd, DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB, &mut blob)?;
    }
    Ok(Blob {
        bo_handle: blob.bo_handle,
        res_handle: blob.res_handle,
    })
}

impl<K: Kernel> Vdrm<K> {
    pub(crate) fn new(fd: &Arc<OwnedFd<K>>) -> Result<Self, VdrmError> {
        let mut params = [
            drm_virtgpu_context_set_param {
                param: VIRTGPU_CONTEXT_PARAM_CAPSET_ID,
                value: VIRTGPU_DRM_CAPSET_DRM as u64,
            },
            drm_virtgpu_context_set_param {
                param: VIRTGPU_CONTEXT_PARAM_NUM_RINGS,
                value: NUM_RINGS,
            },
        ];
        let mut init = drm_virtgpu_context_init {
            num_params: params.len() as u32,
            pad: 0,
            ctx_set_params: params.as_mut_ptr().expose_provenance() as u64,
        };
        unsafe {
            drm::ioctl(fd, DRM_IOCTL_VIRTGPU_CONTEXT_INIT, &mut init)
                .map_err(VdrmError::ContextInit)?;
        }
        let cross_device =
            get_param(fd, VIRTGPU_PARAM_CROSS_DEVICE).is_some_and(|value| value != 0);
        let blob_alignment = match get_param(fd, VIRTGPU_PARAM_BLOB_ALIGNMENT) {
            Some(value) if (value as u32).is_power_of_two() => value as u32 as u64,
            _ => 1,
        };
        let shmem_size = align(SHMEM_SIZE as u64, blob_alignment) as usize;
        let params = BlobCreate {
            size: shmem_size as u64,
            blob_flags: VIRTGPU_BLOB_FLAG_USE_MAPPABLE,
            blob_id: 0,
            blob_hints: 0,
        };
        let handle = create_blob(fd, &params, &[])
            .map_err(VdrmError::ShmemBlob)?
            .bo_handle;
        let close = on_drop(|| drm::gem_close(fd, handle));
        let mut map = drm_virtgpu_map {
            offset: 0,
            handle,
            pad: 0,
        };
        unsafe {
            drm::ioctl(fd, DRM_IOCTL_VIRTGPU_MAP, &mut map).map_err(VdrmError::Map)?;
        }
        let ptr = fd
            .kernel
            .mmap(fd.fd, shmem_size, map.offset)
            .map_err(VdrmError::Mmap)?;
        let unmap = on_drop(|| unsafe { fd.kernel.munmap(ptr, shmem_size) });
        let rsp_mem_offset = unsafe { ptr.cast::<vdrm_shmem>().read() }.rsp_mem_offset as usize;
        if !rsp_mem_offset.is_multiple_of(RSP_ALIGNMENT)
            || rsp_mem_offset < size_of::<vdrm_shmem>()
            || rsp_mem_offset >= SHMEM_SIZE
        {
            return Err(VdrmError::RspMemOffset);
        }
        unmap.forget();
        close.forget();
        Ok(Self {
            fd: fd.clone(),
            shmem: ptr,
            shmem_size,
            shmem_handle: handle,
            rsp_mem_offset,
            rsp_ring: RspRing::new((SHMEM_SIZE - rsp_mem_offset) / RSP_ALIGNMENT),
            queue: Mutex::new(Queue {
                data: [MaybeUninit::uninit(); REQBUF_SIZE],
                len: 0,
                seqno: 0,
                slots: Vec::new(),
                fence: None,
            }),
            next_blob_id: AtomicU64::new(1),
            blob_alignment,
            cross_device,
        })
    }

    pub(crate) fn blob_size(&self, size: u64) -> u64 {
        align(size, self.blob_alignment)
    }

    pub(crate) fn cross_device(&self) -> bool {
        self.cross_device
    }

    pub(crate) fn next_blob_id(&self) -> u64 {
        self.next_blob_id.fetch_add(1, Relaxed)
    }

    fn flush_locked(&self, queue: &mut Queue<K>) -> Result<(), VdrmError> {
        self.rsp_ring.check()?;
        if queue.len == 0 {
            return Ok(());
        }
        let mut execbuffer = drm_virtgpu_execbuffer {
            flags: VIRTGPU_EXECBUF_RING_IDX | VIRTGPU_EXECBUF_FENCE_FD_OUT,
            size: queue.len as u32,
            command: queue.data.as_ptr().expose_provenance() as u64,
            syncobj_stride: size_of::<drm_virtgpu_execbuffer_syncobj>() as u32,
            ..Default::default()
        };
        unsafe { drm::ioctl(&self.fd, DRM_IOCTL_VIRTGPU_EXECBUFFER, &mut execbuffer) }
            .inspect_err(|&errno| self.rsp_ring.fail(errno))
            .map_err(VdrmError::Execbuffer)?;
        queue.len = 0;
        queue.fence = Some(Arc::new(OwnedFd::new(&self.fd.kernel, execbuffer.fence_fd)));
        Ok(())
    }

    fn flush_queue(&self, mut queue: MutexGuard<'_, Queue<K>>) -> Result<(), VdrmError> {
        self.flush_locked(&mut queue)?;
        let Some(fence) = queue.fence.clone() else {
            return Ok(());
        };
        let slots = mem::take(&mut queue.slots);
        drop(queue);
        self.fd
            .kernel
            .wait_readable(fence.fd)
            .inspect_err(|&errno| self.rsp_ring.fail(errno))
            .map_err(VdrmError::Fence)?;
        for slot in slots {
            self.rsp_ring.release(slot);
        }
        Ok(())
    }

    fn enqueue<T, R>(
        &self,
        req: &T,
        parts: &[Part<'_>],
        rsp: Option<R>,
        payload_len: usize,
    ) -> Result<(RspSlot, MutexGuard<'_, Queue<K>>), VdrmError> {
        const {
            assert!(align_of::<R>() <= RSP_ALIGNMENT);
            assert!(size_of::<R>() >= size_of::<vdrm_ccmd_rsp>());
        }
        let len = request_len(req, parts)?;
        let size = size_of::<R>().saturating_add(payload_len);
        let slot = loop {
            if let Some(slot) = self.rsp_ring.try_acquire(size)? {
                break slot;
            }
            self.flush()?;
            let pending = || !self.queue.lock().slots.is_empty();
            if let Some(slot) = self.rsp_ring.acquire_or_wait(size, pending)? {
                break slot;
            }
        };
        let ptr = self.response(&slot);
        unsafe {
            if let Some(rsp) = rsp {
                ptr::write(ptr.cast::<R>(), rsp);
            }
            let len = slot.size() as u32;
            ptr::write(ptr.cast::<vdrm_ccmd_rsp>(), vdrm_ccmd_rsp { len });
        }
        let mut queue = self.queue.lock();
        if queue.len + len > REQBUF_SIZE {
            self.flush_locked(&mut queue)?;
        }
        queue.write(req, parts, len, slot.offset() as u32);
        queue.len += len;
        Ok((slot, queue))
    }

    fn response(&self, slot: &RspSlot) -> *mut u8 {
        self.shmem.wrapping_add(self.rsp_mem_offset + slot.offset())
    }

    pub(crate) fn send_async<T, R>(
        &self,
        req: &T,
        parts: &[Part<'_>],
        rsp_payload_len: usize,
    ) -> Result<(), VdrmError> {
        let (slot, mut queue) = self.enqueue(req, parts, None::<R>, rsp_payload_len)?;
        queue.slots.push(slot);
        drop(queue);
        self.rsp_ring.notify();
        Ok(())
    }

    pub(crate) fn flush(&self) -> Result<(), VdrmError> {
        self.flush_queue(self.queue.lock())
    }

    pub(crate) fn bo_create<T>(
        &self,
        params: &BlobCreate,
        req: &T,
        parts: &[Part<'_>],
    ) -> Result<Blob, VdrmError> {
        let len = request_len(req, parts)?;
        let mut queue = self.queue.lock();
        self.flush_locked(&mut queue)?;
        queue.write(req, parts, len, 0);
        create_blob(&self.fd, params, &queue.data[..len]).map_err(VdrmError::Blob)
    }

    pub(crate) unsafe fn send_sync<T, R>(
        &self,
        req: &T,
        parts: &[Part<'_>],
        rsp: Option<R>,
    ) -> Result<R, VdrmError> {
        unsafe { self.send_sync_payload::<T, R, [u8]>(req, parts, rsp, &mut [], |_| 0) }
    }

    pub(crate) unsafe fn send_sync_payload<T, R, P: ?Sized>(
        &self,
        req: &T,
        parts: &[Part<'_>],
        rsp: Option<R>,
        payload: &mut P,
        payload_len: impl FnOnce(&R) -> usize,
    ) -> Result<R, VdrmError> {
        let (slot, queue) = self.enqueue(req, parts, rsp, size_of_val(payload))?;
        self.flush_queue(queue)?;
        let ptr = self.response(&slot);
        let rsp = unsafe {
            let rsp = ptr::read(ptr.cast::<R>());
            let written = ptr::read(ptr.cast::<vdrm_ccmd_rsp>()).len as usize;
            let len = payload_len(&rsp)
                .min(size_of_val(payload))
                .min(written.saturating_sub(size_of::<R>()));
            ptr::copy_nonoverlapping(
                ptr.add(size_of::<R>()),
                ptr::from_mut(payload).cast::<u8>(),
                len,
            );
            rsp
        };
        self.rsp_ring.release(slot);
        Ok(rsp)
    }
}

impl<K: Kernel> Drop for Vdrm<K> {
    fn drop(&mut self) {
        unsafe {
            self.fd.kernel.munmap(self.shmem, self.shmem_size);
        }
        drm::gem_close(&self.fd, self.shmem_handle);
    }
}

fn request_len<T>(req: &T, parts: &[Part<'_>]) -> Result<usize, VdrmError> {
    let len = size_of_val(req) + parts_size(parts);
    if len > REQBUF_SIZE || !len.is_multiple_of(REQUEST_ALIGNMENT) {
        return Err(VdrmError::RequestSize);
    }
    Ok(len)
}

fn bytes<T: ?Sized>(value: &T) -> &[MaybeUninit<u8>] {
    unsafe { slice::from_raw_parts(ptr::from_ref(value).cast(), size_of_val(value)) }
}

pub(crate) struct Part<'a>(&'a [MaybeUninit<u8>]);

impl<'a> Part<'a> {
    pub(crate) fn new<T: ?Sized>(v: &'a T) -> Self {
        Self(bytes(v))
    }
}

pub(crate) fn parts_size(parts: &[Part<'_>]) -> usize {
    parts.iter().map(|part| part.0.len()).sum()
}
