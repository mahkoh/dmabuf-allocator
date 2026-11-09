use crate::drivers::amd::amdgpu::AmdgpuAllocateError;
use crate::drivers::amd::gfx6::GFX6_METADATA_WORDS;
use crate::drivers::amd::uapi::AMDGPU_GEM_CREATE_NO_CPU_ACCESS;
use crate::drivers::amd::uapi::AMDGPU_INFO_HW_IP_INFO;
use crate::drivers::amd::uapi::drm_amdgpu_gem_create_in;
use crate::drivers::amd::uapi::drm_amdgpu_gem_metadata__data;
use crate::drivers::amd::uapi::drm_amdgpu_info;
use crate::drivers::amd::uapi::drm_amdgpu_info__anon;
use crate::drivers::amd::uapi::drm_amdgpu_info_hw_ip;
use crate::drivers::virtio_gpu::VirtioHandle;
use crate::drm;
use crate::errno::Errno;
use crate::kernel::Kernel;
use crate::vdrm::BlobCreate;
use crate::vdrm::DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING;
use crate::vdrm::Part;
use crate::vdrm::VIRTGPU_BLOB_FLAG_USE_MAPPABLE;
use crate::vdrm::VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
use crate::vdrm::Vdrm;
use crate::vdrm::VdrmError;
use crate::vdrm::vdrm_ccmd_req;
use crate::vdrm::vdrm_ccmd_rsp;
use core::ffi::c_int;
use run_on_drop::on_drop;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmdgpuVirtioCommandError {
    #[error("could not communicate with the virtio host")]
    Send(#[source] VdrmError),
    #[error("the virtio command failed")]
    Host(#[source] Errno),
}

const AMDGPU_CCMD_QUERY_INFO: u32 = 1;
const AMDGPU_CCMD_GEM_NEW: u32 = 2;
const AMDGPU_CCMD_SET_METADATA: u32 = 5;
const UMD_WORDS: usize = GFX6_METADATA_WORDS.next_multiple_of(2);

#[repr(C)]
#[derive(Copy, Clone)]
struct amdgpu_ccmd_rsp {
    base: vdrm_ccmd_rsp,
    ret: i32,
}

#[repr(C)]
struct amdgpu_ccmd_query_info_req {
    hdr: vdrm_ccmd_req,
    info: drm_amdgpu_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct amdgpu_ccmd_query_info_rsp {
    hdr: amdgpu_ccmd_rsp,
    payload: drm_amdgpu_info_hw_ip,
}

#[repr(C)]
struct AmdgpuCcmdGemNewReqR {
    alloc_size: u64,
    phys_alignment: u64,
    preferred_heap: u32,
    __pad: u32,
    flags: u64,
}

#[repr(C)]
struct amdgpu_ccmd_gem_new_req {
    hdr: vdrm_ccmd_req,
    blob_id: u64,
    r: AmdgpuCcmdGemNewReqR,
}

#[repr(C)]
struct amdgpu_ccmd_set_metadata_req {
    hdr: vdrm_ccmd_req,
    flags: u64,
    tiling_info: u64,
    res_id: VirtioHandle,
    size_metadata: u32,
}

fn unanswered() -> amdgpu_ccmd_rsp {
    amdgpu_ccmd_rsp {
        base: vdrm_ccmd_rsp { len: 0 },
        ret: -libc::EINVAL,
    }
}

pub(crate) fn allocate<K: Kernel>(
    vdrm: &Vdrm<K>,
    input: &drm_amdgpu_gem_create_in,
    metadata: &drm_amdgpu_gem_metadata__data,
) -> Result<c_int, AmdgpuAllocateError> {
    let size = vdrm.blob_size(input.bo_size);
    let blob_id = vdrm.next_blob_id();
    let req = amdgpu_ccmd_gem_new_req {
        hdr: vdrm_ccmd_req::new(AMDGPU_CCMD_GEM_NEW),
        blob_id,
        r: AmdgpuCcmdGemNewReqR {
            alloc_size: size,
            phys_alignment: input.alignment,
            preferred_heap: input.domains as u32,
            __pad: 0,
            flags: input.domain_flags,
        },
    };
    let mut blob_flags = VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
    if input.domain_flags & AMDGPU_GEM_CREATE_NO_CPU_ACCESS == 0 {
        blob_flags |= VIRTGPU_BLOB_FLAG_USE_MAPPABLE;
    }
    let params = BlobCreate {
        size,
        blob_flags,
        blob_id,
        blob_hints: DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING,
    };
    let blob = vdrm
        .bo_create(&params, &req, &[])
        .map_err(AmdgpuAllocateError::VirtioBlob)?;
    let handle = blob.bo_handle;
    let _close = on_drop(|| drm::gem_close(&vdrm.fd, handle));
    set_metadata(vdrm, blob.res_handle, metadata)
        .map_err(AmdgpuAllocateError::VirtioSetMetadata)?;
    drm::handle_to_fd(&vdrm.fd, handle).map_err(AmdgpuAllocateError::Export)
}

fn host_result(ret: i32) -> Result<(), AmdgpuVirtioCommandError> {
    match ret {
        0 => Ok(()),
        _ => Err(AmdgpuVirtioCommandError::Host(Errno(ret.wrapping_neg()))),
    }
}

fn set_metadata<K: Kernel>(
    vdrm: &Vdrm<K>,
    res_id: VirtioHandle,
    metadata: &drm_amdgpu_gem_metadata__data,
) -> Result<(), AmdgpuVirtioCommandError> {
    let size = metadata.data_size_bytes;
    let part = (size != 0).then(|| Part::new(&metadata.data[..UMD_WORDS]));
    let req = amdgpu_ccmd_set_metadata_req {
        hdr: vdrm_ccmd_req::new(AMDGPU_CCMD_SET_METADATA),
        flags: 0,
        tiling_info: metadata.tiling_info,
        res_id,
        size_metadata: size,
    };
    let rsp = unsafe {
        vdrm.send_sync(&req, part.as_slice(), Some(unanswered()))
            .map_err(AmdgpuVirtioCommandError::Send)?
    };
    host_result(rsp.ret)
}

pub(crate) fn query_hw_ip<K: Kernel>(
    vdrm: &Vdrm<K>,
    ip_type: u32,
) -> Result<drm_amdgpu_info_hw_ip, AmdgpuVirtioCommandError> {
    let req = amdgpu_ccmd_query_info_req {
        hdr: vdrm_ccmd_req::new(AMDGPU_CCMD_QUERY_INFO),
        info: drm_amdgpu_info {
            return_pointer: 0,
            return_size: size_of::<drm_amdgpu_info_hw_ip>() as u32,
            query: AMDGPU_INFO_HW_IP_INFO,
            anon: drm_amdgpu_info__anon::query_hw_ip(ip_type),
        },
    };
    let unanswered = amdgpu_ccmd_query_info_rsp {
        hdr: unanswered(),
        payload: drm_amdgpu_info_hw_ip::default(),
    };
    let rsp = unsafe {
        vdrm.send_sync(&req, &[], Some(unanswered))
            .map_err(AmdgpuVirtioCommandError::Send)?
    };
    host_result(rsp.hdr.ret)?;
    Ok(rsp.payload)
}
