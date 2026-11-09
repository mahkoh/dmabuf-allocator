use crate::Drm;
use crate::Fault;
use crate::KernelVersion;
use crate::Prime;
use crate::panfrost::Device as PanfrostDevice;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_CREATE_BO;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_GET_BO_OFFSET;
use crate::uapi::panfrost::DRM_IOCTL_PANFROST_GET_PARAM;
use crate::uapi::panfrost::drm_panfrost_create_bo;
use crate::uapi::panfrost::drm_panfrost_get_bo_offset;
use crate::uapi::panfrost::drm_panfrost_get_param;
use crate::uapi::vdrm::PANFROST_CCMD_CREATE_BO;
use crate::uapi::vdrm::PANFROST_CCMD_GET_BO_OFFSET;
use crate::uapi::vdrm::PANFROST_CCMD_GET_PARAM;
use crate::uapi::vdrm::PANFROST_CCMD_MADVISE;
use crate::uapi::vdrm::VIRTGPU_DRM_CONTEXT_PANFROST;
use crate::uapi::vdrm::panfrost_ccmd_create_bo_req;
use crate::uapi::vdrm::panfrost_ccmd_get_bo_offset_req;
use crate::uapi::vdrm::panfrost_ccmd_get_bo_offset_rsp;
use crate::uapi::vdrm::panfrost_ccmd_get_param_req;
use crate::uapi::vdrm::panfrost_ccmd_get_param_rsp;
use crate::uapi::vdrm::vdrm_ccmd_req;
use crate::uapi::vdrm::vdrm_shmem;
use crate::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
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

const BACKEND_FILE_BASE: u64 = 1 << 59;
const WIRE_FORMAT_VERSION: u32 = 0;

struct Object {
    gem_handle: u32,
    size: u64,
    res_id: Option<u32>,
}

pub struct Context {
    backend_file: u64,
    shmem: Option<Shmem>,
    blobs: BTreeMap<u32, u64>,
    objects: BTreeMap<u64, Object>,
    next_object: u64,
}

pub struct Host {
    backend: Backend<PanfrostDevice>,
    capset: Vec<u8>,
}

impl Renderer for Host {
    type Context = Context;

    const CONTEXT_TYPE: u32 = VIRTGPU_DRM_CONTEXT_PANFROST;
    const CCMD_ALIGNMENT: u32 = 4;
    const DISPATCH_SIZE: u32 = PANFROST_CCMD_MADVISE + 1;

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
            attach_shmem(&mut context.shmem, prime, &blob, size_of::<vdrm_shmem>());
            return;
        }
        if Self::object_by_res_id(context, blob.res_id).is_some() {
            return;
        }
        let Some(key) = context.blobs.remove(&(blob.blob_id as u32)) else {
            return;
        };
        let object = context.objects.get_mut(&key).unwrap();
        if object.size != blob.size {
            return;
        }
        if blob.blob_flags & VIRTGPU_BLOB_FLAG_USE_SHAREABLE != 0 && object.gem_handle == 0 {
            return;
        }
        object.res_id = Some(blob.res_id);
    }

    fn dispatch(
        &self,
        context: &mut Context,
        _prime: &mut dyn Prime,
        hdr: &vdrm_ccmd_req,
        request: &[u8],
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        match hdr.cmd {
            PANFROST_CCMD_CREATE_BO => {
                self.create_bo(context, from_bytes(request), fault)?;
                Ok(None)
            }
            PANFROST_CCMD_GET_PARAM => self.get_param(context, from_bytes(request), fault),
            PANFROST_CCMD_GET_BO_OFFSET => self.get_bo_offset(context, from_bytes(request), fault),
            cmd => unimplemented!("panfrost native context command {cmd}"),
        }
    }
}

impl Host {
    pub fn new(gpu: &str, kernel: KernelVersion) -> Self {
        let backend = PanfrostDevice::new(gpu, kernel);
        let version = backend.driver_version();
        let capset = drm_capset(
            WIRE_FORMAT_VERSION,
            &version,
            VIRTGPU_DRM_CONTEXT_PANFROST,
            &[],
        );
        Self {
            backend: Backend::new(backend, BACKEND_FILE_BASE),
            capset,
        }
    }

    unsafe fn drm_ioctl<T>(&self, file: u64, cmd: u32, arg: &mut T, fault: Option<Fault>) -> i32 {
        let ret = unsafe { self.backend.ioctl_raw(file, cmd, arg, fault) };
        match ret < 0 {
            true => -1,
            false => ret as i32,
        }
    }

    fn object_by_res_id(context: &Context, res_id: u32) -> Option<&Object> {
        context
            .objects
            .values()
            .find(|object| object.res_id == Some(res_id))
    }

    fn create_bo(
        &self,
        context: &mut Context,
        req: panfrost_ccmd_create_bo_req,
        fault: Option<Fault>,
    ) -> Result<(), i32> {
        let mut create = drm_panfrost_create_bo {
            size: req.size,
            flags: req.flags,
            handle: 0,
            pad: 0,
            offset: 0,
        };
        if req.blob_id == 0 || context.blobs.contains_key(&req.blob_id) {
            return Err(EINVAL);
        }
        unsafe {
            self.drm_ioctl(
                context.backend_file,
                DRM_IOCTL_PANFROST_CREATE_BO,
                &mut create,
                fault,
            );
        }
        let key = context.next_object;
        context.next_object += 1;
        context.objects.insert(
            key,
            Object {
                gem_handle: create.handle,
                size: req.size as u64,
                res_id: None,
            },
        );
        context.blobs.insert(req.blob_id, key);
        Ok(())
    }

    fn get_param(
        &self,
        context: &Context,
        req: panfrost_ccmd_get_param_req,
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        let mut param = drm_panfrost_get_param {
            param: req.param,
            pad: 0,
            value: 0,
        };
        let len = size_of::<panfrost_ccmd_get_param_rsp>();
        let mut response = response(context.shmem.as_ref(), &req.hdr, len)?;
        let ret = unsafe {
            self.drm_ioctl(
                context.backend_file,
                DRM_IOCTL_PANFROST_GET_PARAM,
                &mut param,
                fault,
            )
        };
        put(
            &mut response,
            offset_of!(panfrost_ccmd_get_param_rsp, ret),
            ret,
        );
        put(
            &mut response,
            offset_of!(panfrost_ccmd_get_param_rsp, value),
            param.value,
        );
        Ok(Some(response))
    }

    fn get_bo_offset(
        &self,
        context: &Context,
        req: panfrost_ccmd_get_bo_offset_req,
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        let Some(object) = Self::object_by_res_id(context, req.res_id) else {
            return Ok(None);
        };
        let len = size_of::<panfrost_ccmd_get_bo_offset_rsp>();
        let mut response = response(context.shmem.as_ref(), &req.hdr, len)?;
        let mut offset = drm_panfrost_get_bo_offset {
            handle: object.gem_handle,
            pad: 0,
            offset: 0,
        };
        let ret = unsafe {
            self.drm_ioctl(
                context.backend_file,
                DRM_IOCTL_PANFROST_GET_BO_OFFSET,
                &mut offset,
                fault,
            )
        };
        put(
            &mut response,
            offset_of!(panfrost_ccmd_get_bo_offset_rsp, ret),
            ret,
        );
        put(
            &mut response,
            offset_of!(panfrost_ccmd_get_bo_offset_rsp, offset),
            offset.offset,
        );
        Ok(Some(response))
    }
}
