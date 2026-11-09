use crate::drivers::intel::i915::Bo;
use crate::drivers::intel::i915::DRM_IOCTL_I915_GEM_GET_TILING;
use crate::drivers::intel::i915::I915Error;
use crate::drivers::intel::i915::I915QueryError;
use crate::drivers::intel::i915::IoctlHandle;
use crate::drivers::intel::i915::drm_i915_gem_get_tiling;
use crate::drivers::intel::i915::drm_i915_gem_memory_class_instance;
use crate::drivers::intel::i915::regions_ext;
use crate::drivers::intel::i915::set_pat_ext;
use crate::errno::Errno;
use crate::kernel::Kernel;
use crate::sys::Ioctl;
use crate::vdrm::BlobCreate;
use crate::vdrm::DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING;
use crate::vdrm::Part;
use crate::vdrm::VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE;
use crate::vdrm::VIRTGPU_BLOB_FLAG_USE_MAPPABLE;
use crate::vdrm::VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
use crate::vdrm::Vdrm;
use crate::vdrm::parts_size;
use crate::vdrm::vdrm_ccmd_req;
use crate::vdrm::vdrm_ccmd_rsp;
use arrayvec::ArrayVec;

const I915_CCMD_IOCTL_SIMPLE: u32 = 1;
const I915_CCMD_QUERYPARAM: u32 = 3;
const I915_CCMD_GEM_CREATE: u32 = 4;
const I915_CCMD_GEM_CREATE_EXT: u32 = 5;

#[repr(C)]
struct i915_ccmd_ioctl_simple_req {
    hdr: vdrm_ccmd_req,
    cmd: u32,
    pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct i915_ccmd_ioctl_simple_rsp {
    hdr: vdrm_ccmd_rsp,
    ret: i32,
}

#[repr(C)]
struct i915_ccmd_queryparam_req {
    hdr: vdrm_ccmd_req,
    query_id: u32,
    length: u32,
    flags: u32,
    pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct i915_ccmd_queryparam_rsp {
    hdr: vdrm_ccmd_rsp,
    pad: u32,
    ret: i32,
    length: i32,
}

#[repr(C)]
struct i915_ccmd_gem_create_req {
    hdr: vdrm_ccmd_req,
    size: u64,
    blob_id: u32,
    pad: u32,
}

#[repr(C)]
struct i915_ccmd_gem_create_ext_req {
    hdr: vdrm_ccmd_req,
    size: u64,
    blob_id: u32,
    gem_flags: u32,
    ext_size: u32,
    pad: u32,
}

fn simple_req(cmd: Ioctl) -> i915_ccmd_ioctl_simple_req {
    i915_ccmd_ioctl_simple_req {
        hdr: vdrm_ccmd_req::new(I915_CCMD_IOCTL_SIMPLE),
        cmd: cmd as u32,
        pad: 0,
    }
}

pub(super) fn query<K: Kernel>(
    vdrm: &Vdrm<K>,
    query_id: u64,
    data: &mut [u8],
) -> Result<i32, I915QueryError> {
    let req = i915_ccmd_queryparam_req {
        hdr: vdrm_ccmd_req::new(I915_CCMD_QUERYPARAM),
        query_id: query_id as u32,
        length: data.len() as u32,
        flags: 0,
        pad: 0,
    };
    let rsp = unsafe {
        vdrm.send_sync_payload(&req, &[], None, data, |rsp: &i915_ccmd_queryparam_rsp| {
            rsp.length.max(0) as usize
        })
        .map_err(I915QueryError::VirtioSend)?
    };
    match rsp.ret {
        0 => Ok(rsp.length),
        ret => Err(I915QueryError::VirtioHost(Errno(ret.wrapping_neg()))),
    }
}

fn bo_create<K: Kernel, T>(
    vdrm: &Vdrm<K>,
    size: u64,
    blob_id: u64,
    req: &T,
    parts: &[Part<'_>],
) -> Result<Bo, I915Error> {
    let mut blob_flags = VIRTGPU_BLOB_FLAG_USE_MAPPABLE | VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
    if vdrm.cross_device() {
        blob_flags |= VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE;
    }
    let params = BlobCreate {
        size,
        blob_flags,
        blob_id,
        blob_hints: DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING,
    };
    let blob = vdrm
        .bo_create(&params, req, parts)
        .map_err(I915Error::VirtioBlob)?;
    Ok(Bo {
        gem_handle: blob.bo_handle,
        ioctl_handle: IoctlHandle {
            virtio: blob.res_handle,
        },
    })
}

pub(super) fn gem_create<K: Kernel>(vdrm: &Vdrm<K>, size: u64) -> Result<Bo, I915Error> {
    let blob_id = vdrm.next_blob_id();
    let req = i915_ccmd_gem_create_req {
        hdr: vdrm_ccmd_req::new(I915_CCMD_GEM_CREATE),
        size,
        blob_id: blob_id as u32,
        pad: 0,
    };
    bo_create(vdrm, size, blob_id, &req, &[])
}

pub(super) fn gem_create_ext<K: Kernel>(
    vdrm: &Vdrm<K>,
    size: u64,
    gem_flags: u32,
    instances: &[drm_i915_gem_memory_class_instance],
    pat_index: Option<u32>,
) -> Result<Bo, I915Error> {
    let regions = regions_ext(instances.len() as u32, 0, 0);
    let set_pat = pat_index.map(set_pat_ext);
    let mut parts = ArrayVec::<_, 3>::new();
    parts.push(Part::new(&regions));
    parts.push(Part::new(instances));
    if let Some(set_pat) = &set_pat {
        parts.push(Part::new(set_pat));
    }
    let blob_id = vdrm.next_blob_id();
    let req = i915_ccmd_gem_create_ext_req {
        hdr: vdrm_ccmd_req::new(I915_CCMD_GEM_CREATE_EXT),
        size,
        blob_id: blob_id as u32,
        gem_flags,
        ext_size: parts_size(&parts) as u32,
        pad: 0,
    };
    bo_create(vdrm, size, blob_id, &req, &parts)
}

pub(super) fn simple_ioctl<K: Kernel, T>(
    vdrm: &Vdrm<K>,
    cmd: Ioctl,
    arg: &T,
    rsp_payload_len: usize,
) {
    let _ = vdrm.send_async::<_, i915_ccmd_ioctl_simple_rsp>(
        &simple_req(cmd),
        &[Part::new(arg)],
        rsp_payload_len,
    );
}

pub(super) fn get_tiling<K: Kernel>(
    vdrm: &Vdrm<K>,
    get_tiling: &mut drm_i915_gem_get_tiling,
) -> bool {
    let arg = *get_tiling;
    let rsp = unsafe {
        vdrm.send_sync_payload(
            &simple_req(DRM_IOCTL_I915_GEM_GET_TILING),
            &[Part::new(&arg)],
            None,
            get_tiling,
            |_: &i915_ccmd_ioctl_simple_rsp| size_of::<drm_i915_gem_get_tiling>(),
        )
    };
    rsp.is_ok_and(|rsp| rsp.ret == 0)
}
