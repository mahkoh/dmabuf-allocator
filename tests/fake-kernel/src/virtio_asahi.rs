use crate::Drm;
use crate::Fault;
use crate::KernelVersion;
use crate::Prime;
use crate::asahi::Device as AsahiDevice;
use crate::uapi::asahi::DRM_ASAHI_GEM_VM_PRIVATE;
use crate::uapi::asahi::DRM_ASAHI_GET_TIME;
use crate::uapi::asahi::DRM_ASAHI_PRIORITY_MEDIUM;
use crate::uapi::asahi::DRM_ASAHI_QUEUE_CREATE;
use crate::uapi::asahi::DRM_ASAHI_QUEUE_DESTROY;
use crate::uapi::asahi::DRM_ASAHI_VM_CREATE;
use crate::uapi::asahi::DRM_ASAHI_VM_DESTROY;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_GEM_CREATE;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_GET_PARAMS;
use crate::uapi::asahi::DRM_IOCTL_ASAHI_VM_BIND;
use crate::uapi::asahi::drm_asahi_gem_bind_op;
use crate::uapi::asahi::drm_asahi_gem_create;
use crate::uapi::asahi::drm_asahi_queue_create;
use crate::uapi::asahi::drm_asahi_vm_bind;
use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::drm::DRM_IOCTL_GEM_CLOSE;
use crate::uapi::drm::drm_gem_close;
use crate::uapi::ioctl::_IOC_NR;
use crate::uapi::ioctl::_IOC_SIZE;
use crate::uapi::ioctl::IOC_OUT;
use crate::uapi::vdrm::ASAHI_CCMD_GEM_BIND_OBJECT;
use crate::uapi::vdrm::ASAHI_CCMD_GEM_NEW;
use crate::uapi::vdrm::ASAHI_CCMD_GET_PARAMS;
use crate::uapi::vdrm::ASAHI_CCMD_IOCTL_SIMPLE;
use crate::uapi::vdrm::ASAHI_CCMD_VM_BIND;
use crate::uapi::vdrm::VIRTGPU_DRM_CONTEXT_ASAHI;
use crate::uapi::vdrm::asahi_ccmd_gem_new_req;
use crate::uapi::vdrm::asahi_ccmd_get_params_req;
use crate::uapi::vdrm::asahi_ccmd_get_params_rsp;
use crate::uapi::vdrm::asahi_ccmd_ioctl_simple_req;
use crate::uapi::vdrm::asahi_ccmd_ioctl_simple_rsp;
use crate::uapi::vdrm::asahi_ccmd_vm_bind_req;
use crate::uapi::vdrm::asahi_shmem;
use crate::uapi::vdrm::vdrm_ccmd_req;
use crate::user::from_bytes;
use crate::user::put;
use crate::vdrm_host::Backend;
use crate::vdrm_host::Blob;
use crate::vdrm_host::Renderer;
use crate::vdrm_host::Shmem;
use crate::vdrm_host::attach_shmem;
use crate::vdrm_host::drm_capset;
use crate::vdrm_host::response;
use libc::EINVAL;
use std::collections::BTreeMap;
use std::mem::offset_of;

const BACKEND_FILE_BASE: u64 = 1 << 60;
const WIRE_FORMAT_VERSION: u32 = 2;
const HOST_PAGE_SIZE: u64 = 1 << 14;
const MAX_SIMPLE_PAYLOAD_SIZE: usize = 128;

struct Object {
    gem_handle: u32,
    size: u64,
    res_id: Option<u32>,
    exported: bool,
}

pub struct Context {
    backend_file: u64,
    shmem: Option<Shmem>,
    blobs: BTreeMap<u32, u64>,
    objects: BTreeMap<u64, Object>,
    next_object: u64,
}

pub struct Host {
    backend: Backend<AsahiDevice>,
    capset: Vec<u8>,
}

fn drm_ioctl_ret(ret: i64) -> i32 {
    match ret < 0 {
        true => -1,
        false => ret as i32,
    }
}

impl Renderer for Host {
    type Context = Context;

    const CONTEXT_TYPE: u32 = VIRTGPU_DRM_CONTEXT_ASAHI;
    const CCMD_ALIGNMENT: u32 = 4;
    const DISPATCH_SIZE: u32 = ASAHI_CCMD_GEM_BIND_OBJECT + 1;

    fn capset(&self) -> &[u8] {
        &self.capset
    }

    fn create_context(&self) -> Context {
        Context {
            backend_file: self.backend.open(),
            shmem: None,
            blobs: BTreeMap::new(),
            objects: BTreeMap::new(),
            next_object: 1,
        }
    }

    fn destroy_context(&self, context: Context) {
        self.backend.release(context.backend_file);
    }

    fn shmem(context: &Context) -> Option<&Shmem> {
        context.shmem.as_ref()
    }

    fn get_blob(&self, context: &mut Context, prime: &mut dyn Prime, blob: Blob) {
        if blob.blob_id >> 32 != 0 {
            return;
        }
        if blob.blob_id == 0 {
            attach_shmem(&mut context.shmem, prime, &blob, size_of::<asahi_shmem>());
            return;
        }
        if context
            .objects
            .values()
            .any(|object| object.res_id == Some(blob.res_id))
        {
            return;
        }
        let Some(key) = context.blobs.remove(&(blob.blob_id as u32)) else {
            return;
        };
        let object = context.objects.get_mut(&key).unwrap();
        if object.exported {
            return;
        }
        if object.size.next_multiple_of(HOST_PAGE_SIZE) != blob.size {
            return;
        }
        object.res_id = Some(blob.res_id);
        object.exported = true;
    }

    fn dispatch(
        &self,
        context: &mut Context,
        prime: &mut dyn Prime,
        hdr: &vdrm_ccmd_req,
        request: &[u8],
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        match hdr.cmd {
            ASAHI_CCMD_IOCTL_SIMPLE => self.ioctl_simple(context, hdr, request, fault),
            ASAHI_CCMD_VM_BIND => {
                self.vm_bind(context, prime, hdr, request, fault)?;
                Ok(None)
            }
            ASAHI_CCMD_GET_PARAMS => self.get_params(context, hdr, request, fault),
            ASAHI_CCMD_GEM_NEW => {
                self.gem_new(context, prime, from_bytes(request), fault);
                Ok(None)
            }
            cmd => unimplemented!("asahi native context command {cmd}"),
        }
    }
}

impl Host {
    pub fn new(chip: &str, kernel: KernelVersion) -> Self {
        let backend = AsahiDevice::new(chip, kernel);
        let version = backend.driver_version();
        let capset = drm_capset(
            WIRE_FORMAT_VERSION,
            &version,
            VIRTGPU_DRM_CONTEXT_ASAHI,
            &[],
        );
        Self {
            backend: Backend::new(backend, BACKEND_FILE_BASE),
            capset,
        }
    }

    fn ioctl_simple(
        &self,
        context: &Context,
        hdr: &vdrm_ccmd_req,
        request: &[u8],
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        let req: asahi_ccmd_ioctl_simple_req = from_bytes(request);
        let header_len = size_of::<asahi_ccmd_ioctl_simple_req>();
        let payload_len = _IOC_SIZE(req.cmd) as usize;
        if hdr.len as usize != header_len + payload_len {
            return Err(EINVAL);
        }
        if payload_len > MAX_SIMPLE_PAYLOAD_SIZE {
            return Err(EINVAL);
        }
        let iocnr = _IOC_NR(req.cmd).wrapping_sub(DRM_COMMAND_BASE);
        match iocnr {
            DRM_ASAHI_VM_CREATE
            | DRM_ASAHI_VM_DESTROY
            | DRM_ASAHI_QUEUE_CREATE
            | DRM_ASAHI_QUEUE_DESTROY
            | DRM_ASAHI_GET_TIME => {}
            _ => return Err(EINVAL),
        }
        let mut payload = request[header_len..].to_vec();
        if iocnr == DRM_ASAHI_QUEUE_CREATE {
            let args: drm_asahi_queue_create = from_bytes(&payload);
            if args.priority > DRM_ASAHI_PRIORITY_MEDIUM {
                return Err(EINVAL);
            }
        }
        let rsp_header_len = size_of::<asahi_ccmd_ioctl_simple_rsp>();
        let mut rsp_len = rsp_header_len;
        if req.cmd & IOC_OUT != 0 {
            rsp_len += payload_len;
        }
        let mut response = response(context.shmem.as_ref(), hdr, rsp_len)?;
        let file = context.backend_file;
        let ret = unsafe { self.backend.ioctl_bytes(file, req.cmd, &mut payload, fault) };
        let ret_offset = offset_of!(asahi_ccmd_ioctl_simple_rsp, ret);
        put(&mut response, ret_offset, drm_ioctl_ret(ret));
        if req.cmd & IOC_OUT != 0 {
            response[rsp_header_len..].copy_from_slice(&payload);
        }
        Ok(Some(response))
    }

    fn vm_bind(
        &self,
        context: &Context,
        prime: &mut dyn Prime,
        hdr: &vdrm_ccmd_req,
        request: &[u8],
        fault: Option<Fault>,
    ) -> Result<(), i32> {
        let req: asahi_ccmd_vm_bind_req = from_bytes(request);
        if (req.stride as usize) < offset_of!(drm_asahi_gem_bind_op, offset) {
            return Err(EINVAL);
        }
        let header_len = size_of::<asahi_ccmd_vm_bind_req>();
        if hdr.len as usize != header_len + req.stride as usize * req.count as usize {
            return Err(EINVAL);
        }
        let op_size = size_of::<drm_asahi_gem_bind_op>();
        let copied = (req.stride as usize).min(op_size);
        let mut ops: Vec<drm_asahi_gem_bind_op> = (0..req.count as usize)
            .map(|index| {
                let start = header_len + index * req.stride as usize;
                let mut op: drm_asahi_gem_bind_op = from_bytes(&request[start..start + copied]);
                op.handle = context
                    .objects
                    .values()
                    .find(|object| object.res_id == Some(op.handle))
                    .map_or(0, |object| object.gem_handle);
                op
            })
            .collect();
        let mut bind = drm_asahi_vm_bind {
            vm_id: req.vm_id,
            num_binds: req.count,
            stride: op_size as u32,
            pad: 0,
            userptr: ops.as_mut_ptr() as u64,
        };
        let file = context.backend_file;
        let ret = unsafe {
            self.backend
                .ioctl_raw(file, DRM_IOCTL_ASAHI_VM_BIND, &mut bind, fault)
        };
        if ret < 0 {
            Self::async_error(context, prime);
        }
        Ok(())
    }

    fn get_params(
        &self,
        context: &Context,
        hdr: &vdrm_ccmd_req,
        request: &[u8],
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        if hdr.len as usize != size_of::<asahi_ccmd_get_params_req>() {
            return Err(EINVAL);
        }
        let mut req: asahi_ccmd_get_params_req = from_bytes(request);
        let rsp_header_len = size_of::<asahi_ccmd_get_params_rsp>();
        let len = rsp_header_len + req.params.size as usize;
        let mut response = response(context.shmem.as_ref(), hdr, len)?;
        let ret_offset = offset_of!(asahi_ccmd_get_params_rsp, ret);
        if req.params.param_group != 0 {
            put(&mut response, ret_offset, -EINVAL);
            return Ok(Some(response));
        }
        let mut payload = vec![0u8; req.params.size as usize];
        req.params.pointer = payload.as_mut_ptr() as u64;
        let file = context.backend_file;
        let ret = unsafe {
            self.backend
                .ioctl_raw(file, DRM_IOCTL_ASAHI_GET_PARAMS, &mut req.params, fault)
        };
        put(&mut response, ret_offset, drm_ioctl_ret(ret));
        response[rsp_header_len..].copy_from_slice(&payload);
        Ok(Some(response))
    }

    fn async_error(context: &Context, prime: &mut dyn Prime) {
        let Some(shmem) = &context.shmem else {
            return;
        };
        let offset = shmem.field(offset_of!(asahi_shmem, async_error));
        let mut value = [0u8; 4];
        prime.read_file(offset, &mut value).unwrap();
        let value = u32::from_ne_bytes(value).wrapping_add(1);
        prime.write_file(offset, &value.to_ne_bytes()).unwrap();
    }

    fn gem_new(
        &self,
        context: &mut Context,
        prime: &mut dyn Prime,
        req: asahi_ccmd_gem_new_req,
        fault: Option<Fault>,
    ) {
        if req.blob_id == 0 || context.blobs.contains_key(&req.blob_id) {
            Self::async_error(context, prime);
            return;
        }
        let create_vm_id = match req.flags & DRM_ASAHI_GEM_VM_PRIVATE {
            0 => 0,
            _ => req.vm_id,
        };
        let mut create = drm_asahi_gem_create {
            size: req.size,
            flags: req.flags,
            vm_id: create_vm_id,
            handle: 0,
            pad: 0,
        };
        let file = context.backend_file;
        let created = unsafe {
            self.backend
                .ioctl_raw(file, DRM_IOCTL_ASAHI_GEM_CREATE, &mut create, fault)
        };
        if created < 0 {
            Self::async_error(context, prime);
            return;
        }
        if req.addr != 0 {
            let mut op = drm_asahi_gem_bind_op {
                flags: req.bind_flags,
                handle: create.handle,
                offset: 0,
                range: req.size,
                addr: req.addr,
            };
            let mut bind = drm_asahi_vm_bind {
                vm_id: req.vm_id,
                num_binds: 1,
                stride: size_of::<drm_asahi_gem_bind_op>() as u32,
                pad: 0,
                userptr: (&mut op as *mut drm_asahi_gem_bind_op) as u64,
            };
            let bound = unsafe {
                self.backend
                    .ioctl_raw(file, DRM_IOCTL_ASAHI_VM_BIND, &mut bind, None)
            };
            if bound < 0 {
                let mut close = drm_gem_close {
                    handle: create.handle,
                    pad: 0,
                };
                unsafe {
                    self.backend
                        .ioctl_raw(file, DRM_IOCTL_GEM_CLOSE, &mut close, None);
                }
                Self::async_error(context, prime);
                return;
            }
        }
        let key = context.next_object;
        context.next_object += 1;
        context.objects.insert(
            key,
            Object {
                gem_handle: create.handle,
                size: req.size,
                res_id: None,
                exported: false,
            },
        );
        context.blobs.insert(req.blob_id, key);
    }
}
