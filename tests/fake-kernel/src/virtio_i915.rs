use crate::Drm;
use crate::Fault;
use crate::KernelVersion;
use crate::Prime;
use crate::intel::devices::DEVICES;
use crate::intel::kernel::Device as IntelDevice;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_CONTEXT_GETPARAM;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_CONTEXT_SETPARAM;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_CREATE;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_CREATE_EXT;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_GET_APERTURE;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_GET_TILING;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_SET_DOMAIN;
use crate::uapi::i915::DRM_IOCTL_I915_GEM_SET_TILING;
use crate::uapi::i915::DRM_IOCTL_I915_GETPARAM;
use crate::uapi::i915::DRM_IOCTL_I915_QUERY;
use crate::uapi::i915::I915_CONTEXT_DEFAULT_PRIORITY;
use crate::uapi::i915::I915_CONTEXT_PARAM_GTT_SIZE;
use crate::uapi::i915::I915_CONTEXT_PARAM_PRIORITY;
use crate::uapi::i915::I915_CONTEXT_PARAM_RECOVERABLE;
use crate::uapi::i915::I915_CONTEXT_PARAM_VM;
use crate::uapi::i915::I915_GEM_CREATE_EXT_MEMORY_REGIONS;
use crate::uapi::i915::I915_GEM_CREATE_EXT_PROTECTED_CONTENT;
use crate::uapi::i915::I915_GEM_CREATE_EXT_SET_PAT;
use crate::uapi::i915::I915_MMAP_OFFSET_FIXED;
use crate::uapi::i915::I915_MMAP_OFFSET_GTT;
use crate::uapi::i915::I915_MMAP_OFFSET_UC;
use crate::uapi::i915::I915_MMAP_OFFSET_WB;
use crate::uapi::i915::I915_MMAP_OFFSET_WC;
use crate::uapi::i915::I915_PARAM_MMAP_GTT_VERSION;
use crate::uapi::i915::drm_i915_gem_context_param;
use crate::uapi::i915::drm_i915_gem_create;
use crate::uapi::i915::drm_i915_gem_create_ext;
use crate::uapi::i915::drm_i915_gem_create_ext_memory_regions;
use crate::uapi::i915::drm_i915_gem_create_ext_protected_content;
use crate::uapi::i915::drm_i915_gem_create_ext_set_pat;
use crate::uapi::i915::drm_i915_gem_memory_class_instance;
use crate::uapi::i915::drm_i915_getparam;
use crate::uapi::i915::drm_i915_query;
use crate::uapi::i915::drm_i915_query_item;
use crate::uapi::i915::i915_user_extension;
use crate::uapi::ioctl::_IOC_SIZE;
use crate::uapi::ioctl::IOC_OUT;
use crate::uapi::vdrm::I915_CCMD_GEM_CREATE;
use crate::uapi::vdrm::I915_CCMD_GEM_CREATE_EXT;
use crate::uapi::vdrm::I915_CCMD_GEM_EXECBUFFER2_V2;
use crate::uapi::vdrm::I915_CCMD_GEM_SET_MMAP_MODE;
use crate::uapi::vdrm::I915_CCMD_GETPARAM;
use crate::uapi::vdrm::I915_CCMD_IOCTL_SIMPLE;
use crate::uapi::vdrm::I915_CCMD_QUERYPARAM;
use crate::uapi::vdrm::VIRTGPU_DRM_CONTEXT_I915;
use crate::uapi::vdrm::i915_ccmd_gem_create_ext_req;
use crate::uapi::vdrm::i915_ccmd_gem_create_req;
use crate::uapi::vdrm::i915_ccmd_gem_set_mmap_mode_req;
use crate::uapi::vdrm::i915_ccmd_getparam_req;
use crate::uapi::vdrm::i915_ccmd_getparam_rsp;
use crate::uapi::vdrm::i915_ccmd_ioctl_simple_req;
use crate::uapi::vdrm::i915_ccmd_ioctl_simple_rsp;
use crate::uapi::vdrm::i915_ccmd_queryparam_req;
use crate::uapi::vdrm::i915_ccmd_queryparam_rsp;
use crate::uapi::vdrm::i915_shmem;
use crate::uapi::vdrm::vdrm_ccmd_req;
use crate::uapi::vdrm::virgl_renderer_capset_drm__u__intel;
use crate::user::from_bytes;
use crate::user::put;
use crate::vdrm_host::Backend;
use crate::vdrm_host::Blob;
use crate::vdrm_host::Renderer;
use crate::vdrm_host::Shmem;
use crate::vdrm_host::attach_shmem;
use crate::vdrm_host::drm_capset;
use crate::vdrm_host::response;
use bytemuck::NoUninit;
use bytemuck::bytes_of;
use libc::EINVAL;
use libc::EPERM;
use libc::c_int;
use std::collections::BTreeMap;
use std::mem::offset_of;

const BACKEND_FILE_BASE: u64 = 1 << 61;
const WIRE_FORMAT_VERSION: u32 = 2;
const MAX_SIMPLE_PAYLOAD: u32 = 128;

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
    backend: Backend<IntelDevice>,
    capset: Vec<u8>,
}

fn write_struct<T: NoUninit>(bytes: &mut [u8], value: &T) {
    bytes[..size_of::<T>()].copy_from_slice(bytes_of(value));
}

fn intel_ioctl_ret(ret: i64) -> i32 {
    -(ret.min(0) as i32)
}

impl Renderer for Host {
    type Context = Context;

    const CONTEXT_TYPE: u32 = VIRTGPU_DRM_CONTEXT_I915;
    const CCMD_ALIGNMENT: u32 = 4;
    const DISPATCH_SIZE: u32 = I915_CCMD_GEM_EXECBUFFER2_V2 + 1;

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
            attach_shmem(&mut context.shmem, prime, &blob, size_of::<i915_shmem>());
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
            I915_CCMD_IOCTL_SIMPLE => self.ioctl_simple(context, hdr, request, fault),
            I915_CCMD_GETPARAM => self.getparam(context, from_bytes(request), fault),
            I915_CCMD_QUERYPARAM => self.queryparam(context, from_bytes(request), fault),
            I915_CCMD_GEM_CREATE => {
                self.gem_create(context, from_bytes(request), fault)?;
                Ok(None)
            }
            I915_CCMD_GEM_CREATE_EXT => {
                self.gem_create_ext(context, hdr, request, fault)?;
                Ok(None)
            }
            I915_CCMD_GEM_SET_MMAP_MODE => {
                Self::gem_set_mmap_mode(context, from_bytes(request))?;
                Ok(None)
            }
            cmd => unimplemented!("i915 native context command {cmd}"),
        }
    }
}

impl Host {
    pub fn new(chip: &str, kernel: KernelVersion) -> Self {
        let index = DEVICES
            .iter()
            .position(|device| device.name == chip)
            .unwrap_or_else(|| panic!("unknown intel device {chip}"));
        let backend = IntelDevice::variant(index, kernel, false, false);
        let version = backend.driver_version();
        let intel = virgl_renderer_capset_drm__u__intel {
            pci_bus: 0,
            pci_dev: 0,
            pci_func: 0,
            pci_revision_id: 0,
            pci_domain: 0,
            pci_device_id: DEVICES[index].pci_id as u32,
        };
        let capset = drm_capset(
            WIRE_FORMAT_VERSION,
            &version,
            VIRTGPU_DRM_CONTEXT_I915,
            bytes_of(&intel),
        );
        Self {
            backend: Backend::new(backend, BACKEND_FILE_BASE),
            capset,
        }
    }

    fn object_by_res_id(context: &Context, res_id: u32) -> Option<&Object> {
        context
            .objects
            .values()
            .find(|object| object.res_id == Some(res_id))
    }

    fn blob_id_valid(context: &Context, blob_id: u32) -> bool {
        blob_id != 0 && !context.blobs.contains_key(&blob_id)
    }

    fn add_object(context: &mut Context, gem_handle: u32, size: u64, blob_id: u32) {
        let key = context.next_object;
        context.next_object += 1;
        context.objects.insert(
            key,
            Object {
                gem_handle,
                size,
                res_id: None,
            },
        );
        context.blobs.insert(blob_id, key);
    }

    fn ioctl_simple(
        &self,
        context: &Context,
        hdr: &vdrm_ccmd_req,
        request: &[u8],
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        let req: i915_ccmd_ioctl_simple_req = from_bytes(request);
        let header_len = size_of::<i915_ccmd_ioctl_simple_req>();
        let payload_len = _IOC_SIZE(req.cmd);
        if hdr.len as usize != header_len + payload_len as usize {
            return Err(EINVAL);
        }
        if payload_len > MAX_SIMPLE_PAYLOAD {
            return Err(EINVAL);
        }
        let rsp_header_len = size_of::<i915_ccmd_ioctl_simple_rsp>();
        let mut rsp_len = rsp_header_len;
        if req.cmd & IOC_OUT != 0 {
            rsp_len += payload_len as usize;
        }
        let mut response = response(context.shmem.as_ref(), hdr, rsp_len)?;
        let mut payload = request[header_len..].to_vec();
        let ret_offset = offset_of!(i915_ccmd_ioctl_simple_rsp, ret);
        match req.cmd {
            DRM_IOCTL_I915_GEM_SET_TILING
            | DRM_IOCTL_I915_GEM_GET_TILING
            | DRM_IOCTL_I915_GEM_SET_DOMAIN => {
                let res_id: u32 = from_bytes(&payload);
                let Some(object) = Self::object_by_res_id(context, res_id) else {
                    put(&mut response, ret_offset, EINVAL);
                    return Ok(Some(response));
                };
                put(&mut payload, 0, object.gem_handle);
            }
            DRM_IOCTL_I915_GEM_GET_APERTURE => {}
            DRM_IOCTL_I915_GEM_CONTEXT_GETPARAM | DRM_IOCTL_I915_GEM_CONTEXT_SETPARAM => {
                let param: drm_i915_gem_context_param = from_bytes(&payload);
                match param.param {
                    I915_CONTEXT_PARAM_PRIORITY => {
                        if param.value > I915_CONTEXT_DEFAULT_PRIORITY as u64 {
                            put(&mut response, ret_offset, EPERM);
                            return Ok(Some(response));
                        }
                    }
                    I915_CONTEXT_PARAM_RECOVERABLE
                    | I915_CONTEXT_PARAM_GTT_SIZE
                    | I915_CONTEXT_PARAM_VM => {}
                    _ => return Err(EINVAL),
                }
            }
            cmd => unimplemented!("i915 native context simple ioctl {cmd:#x}"),
        }
        let file = context.backend_file;
        let ret = unsafe { self.backend.ioctl_bytes(file, req.cmd, &mut payload, fault) };
        put(&mut response, ret_offset, intel_ioctl_ret(ret));
        if req.cmd & IOC_OUT != 0 {
            response[rsp_header_len..].copy_from_slice(&payload);
        }
        Ok(Some(response))
    }

    fn getparam(
        &self,
        context: &Context,
        req: i915_ccmd_getparam_req,
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        let len = size_of::<i915_ccmd_getparam_rsp>();
        let mut response = response(context.shmem.as_ref(), &req.hdr, len)?;
        let mut value: c_int = -1;
        let mut getparam = drm_i915_getparam {
            param: req.param as i32,
            value: &mut value,
        };
        let file = context.backend_file;
        let ret = unsafe {
            self.backend
                .ioctl_raw(file, DRM_IOCTL_I915_GETPARAM, &mut getparam, fault)
        };
        let ret = intel_ioctl_ret(ret);
        if getparam.param == I915_PARAM_MMAP_GTT_VERSION && value >= 5 {
            value = 4;
        }
        let ret_offset = offset_of!(i915_ccmd_getparam_rsp, ret);
        let value_offset = offset_of!(i915_ccmd_getparam_rsp, value);
        put(&mut response, ret_offset, ret);
        put(&mut response, value_offset, value);
        Ok(Some(response))
    }

    fn queryparam(
        &self,
        context: &Context,
        req: i915_ccmd_queryparam_req,
        fault: Option<Fault>,
    ) -> Result<Option<Vec<u8>>, i32> {
        let rsp_header_len = size_of::<i915_ccmd_queryparam_rsp>();
        let len = rsp_header_len + req.length as usize;
        let mut response = response(context.shmem.as_ref(), &req.hdr, len)?;
        let mut buffer = vec![0u8; req.length as usize];
        let mut item = drm_i915_query_item {
            query_id: req.query_id as u64,
            length: req.length as i32,
            flags: req.flags,
            data_ptr: match req.length {
                0 => 0,
                _ => buffer.as_mut_ptr() as u64,
            },
        };
        let mut query = drm_i915_query {
            num_items: 1,
            flags: 0,
            items_ptr: &mut item as *mut drm_i915_query_item as u64,
        };
        let file = context.backend_file;
        let ret = unsafe {
            self.backend
                .ioctl_raw(file, DRM_IOCTL_I915_QUERY, &mut query, fault)
        };
        let ret = intel_ioctl_ret(ret);
        let ret_offset = offset_of!(i915_ccmd_queryparam_rsp, ret);
        let length_offset = offset_of!(i915_ccmd_queryparam_rsp, length);
        put(&mut response, ret_offset, ret);
        put(&mut response, length_offset, item.length);
        response[rsp_header_len..].copy_from_slice(&buffer);
        Ok(Some(response))
    }

    fn gem_set_mmap_mode(
        context: &Context,
        req: i915_ccmd_gem_set_mmap_mode_req,
    ) -> Result<(), i32> {
        if Self::object_by_res_id(context, req.res_id).is_none() {
            return Ok(());
        }
        match req.flags as u64 {
            I915_MMAP_OFFSET_GTT
            | I915_MMAP_OFFSET_WC
            | I915_MMAP_OFFSET_FIXED
            | I915_MMAP_OFFSET_WB
            | I915_MMAP_OFFSET_UC => Ok(()),
            _ => Err(EINVAL),
        }
    }

    fn gem_create(
        &self,
        context: &mut Context,
        req: i915_ccmd_gem_create_req,
        fault: Option<Fault>,
    ) -> Result<(), i32> {
        if !Self::blob_id_valid(context, req.blob_id) {
            return Err(EINVAL);
        }
        let mut create = drm_i915_gem_create {
            size: req.size,
            handle: 0,
            pad: 0,
        };
        let file = context.backend_file;
        let ret = unsafe {
            self.backend
                .ioctl_raw(file, DRM_IOCTL_I915_GEM_CREATE, &mut create, fault)
        };
        if ret != 0 {
            return Err(intel_ioctl_ret(ret));
        }
        Self::add_object(context, create.handle, req.size, req.blob_id);
        Ok(())
    }

    fn gem_create_ext(
        &self,
        context: &mut Context,
        hdr: &vdrm_ccmd_req,
        request: &[u8],
        fault: Option<Fault>,
    ) -> Result<(), i32> {
        let req: i915_ccmd_gem_create_ext_req = from_bytes(request);
        let header_len = size_of::<i915_ccmd_gem_create_ext_req>();
        if hdr.len as usize != header_len + req.ext_size as usize {
            return Err(EINVAL);
        }
        if !Self::blob_id_valid(context, req.blob_id) {
            return Err(EINVAL);
        }
        let mut payload = request[header_len..].to_vec();
        let base = payload.as_mut_ptr() as u64;
        let mut ext_size = req.ext_size as i64;
        let mut offset = 0usize;
        let mut extension = 0usize;
        while ext_size > 0 {
            if ext_size < size_of::<i915_user_extension>() as i64 {
                return Err(EINVAL);
            }
            let header: i915_user_extension = from_bytes(&payload[extension..]);
            match header.name {
                I915_GEM_CREATE_EXT_MEMORY_REGIONS => {
                    let mut regions: drm_i915_gem_create_ext_memory_regions =
                        from_bytes(&payload[extension..]);
                    offset += size_of::<drm_i915_gem_create_ext_memory_regions>();
                    ext_size -= size_of::<drm_i915_gem_create_ext_memory_regions>() as i64;
                    if ext_size >= 0 {
                        regions.regions = base + offset as u64;
                        write_struct(&mut payload[extension..], &regions);
                        let instances = size_of::<drm_i915_gem_memory_class_instance>()
                            * regions.num_regions as usize;
                        offset += instances;
                        ext_size -= instances as i64;
                    }
                }
                I915_GEM_CREATE_EXT_PROTECTED_CONTENT => {
                    offset += size_of::<drm_i915_gem_create_ext_protected_content>();
                    ext_size -= size_of::<drm_i915_gem_create_ext_protected_content>() as i64;
                }
                I915_GEM_CREATE_EXT_SET_PAT => {
                    offset += size_of::<drm_i915_gem_create_ext_set_pat>();
                    ext_size -= size_of::<drm_i915_gem_create_ext_set_pat>() as i64;
                }
                _ => return Err(EINVAL),
            }
            let next = match ext_size > 0 {
                true => base + offset as u64,
                false => 0,
            };
            let next_offset = extension + offset_of!(i915_user_extension, next_extension);
            payload[next_offset..next_offset + 8].copy_from_slice(&next.to_ne_bytes());
            extension = offset;
        }
        if ext_size != 0 {
            return Err(EINVAL);
        }
        let mut create = drm_i915_gem_create_ext {
            size: req.size,
            handle: 0,
            flags: req.gem_flags,
            extensions: match req.ext_size {
                0 => 0,
                _ => base,
            },
        };
        let file = context.backend_file;
        let ret = unsafe {
            self.backend
                .ioctl_raw(file, DRM_IOCTL_I915_GEM_CREATE_EXT, &mut create, fault)
        };
        if ret != 0 {
            return Err(intel_ioctl_ret(ret));
        }
        Self::add_object(context, create.handle, req.size, req.blob_id);
        Ok(())
    }
}
