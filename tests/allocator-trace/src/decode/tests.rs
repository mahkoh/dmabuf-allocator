use super::*;
use fake_kernel::DriverFeature;
use fake_kernel::DriverVersion;
use fake_kernel::Drm;
use fake_kernel::Export;
use fake_kernel::KernelVersion;
use fake_kernel::LocalMemory;
use fake_kernel::Master;
use fake_kernel::NewHandle;
use fake_kernel::Prime;
use fake_kernel::Syncobjs;
use fake_kernel::ioctls::Func;
use fake_kernel::ioctls::IoctlDesc;
use fake_kernel::uapi::i915::drm_i915_gem_create_ext_set_pat;

const FILE: u64 = 1;
const RESOURCE_HANDLE_BASE: u32 = 100;

struct Driver {
    name: &'static str,
    context: Option<u32>,
}

impl Kernel for Driver {
    fn name(&self) -> &'static str {
        self.name
    }

    fn kernel_version(&self) -> KernelVersion {
        unimplemented!()
    }

    fn id(&self) -> String {
        unimplemented!()
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Driver {
    fn context_type(&self, _file: u64) -> Option<u32> {
        self.context
    }

    fn resource_handle(&self, _file: u64, res_id: u32) -> Option<u32> {
        Some(RESOURCE_HANDLE_BASE + res_id)
    }

    fn desc(&self) -> &'static str {
        unimplemented!()
    }

    fn driver_version(&self) -> DriverVersion {
        unimplemented!()
    }

    fn uevent_driver(&self) -> &'static str {
        unimplemented!()
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        unimplemented!()
    }

    fn syncobjs(&self) -> Option<&Syncobjs> {
        unimplemented!()
    }

    fn master(&self) -> &Master {
        unimplemented!()
    }

    fn check_feature(&self, _feature: DriverFeature) -> bool {
        unimplemented!()
    }

    unsafe fn ioctl(
        &self,
        _file: u64,
        _func: Func,
        _arg: *mut u8,
        _user: &dyn UserMemory,
        _prime: &mut dyn Prime,
    ) -> Option<i64> {
        unimplemented!()
    }

    fn export(&self, _file: u64, _handle: u32) -> Result<Export, i32> {
        unimplemented!()
    }

    fn import(&self, _file: u64, _object: u32) -> Option<u32> {
        unimplemented!()
    }

    fn import_foreign(&self, _file: u64, _export: &Export) -> NewHandle {
        unimplemented!()
    }

    fn close_buffer(&self, _file: u64, _handle: u32) -> bool {
        unimplemented!()
    }

    fn object(&self, _file: u64, _handle: u32) -> Option<u32> {
        unimplemented!()
    }

    fn object_has_handles(&self, _object: u32) -> bool {
        unimplemented!()
    }

    fn release_export(&self, _object: u32) {
        unimplemented!()
    }

    fn release_file(&self, _file: u64) {
        unimplemented!()
    }
}

fn driver(name: &'static str) -> Driver {
    Driver {
        name,
        context: None,
    }
}

fn virtio(context: u32) -> Driver {
    Driver {
        name: "virtio_gpu",
        context: Some(context),
    }
}

fn address<T>(value: &mut T) -> u64 {
    std::ptr::from_mut(value) as u64
}

fn trace<T>(
    kernel: &Driver,
    request: u32,
    arg: &mut T,
    complete: impl FnOnce(&mut T),
) -> Vec<Traced> {
    let user = unsafe { LocalMemory::new() };
    let pending = decode(kernel, FILE, request, address(arg), &user);
    complete(arg);
    pending
        .into_iter()
        .map(|pending| pending.finish(kernel, FILE, &user).unwrap())
        .collect()
}

fn command<T>(buffer: &mut Vec<u64>, command: T) {
    assert_eq!(size_of::<T>() % size_of::<u64>(), 0);
    let words = size_of::<T>() / size_of::<u64>();
    let start = buffer.len();
    buffer.resize(start + words, 0);
    unsafe {
        buffer[start..].as_mut_ptr().cast::<T>().write(command);
    }
}

fn execbuffer(buffer: &mut [u64], size: usize) -> drm_virtgpu_execbuffer {
    let mut execbuffer: drm_virtgpu_execbuffer = unsafe { std::mem::zeroed() };
    execbuffer.size = size as u32;
    execbuffer.command = buffer.as_mut_ptr() as u64;
    execbuffer
}

fn header<T>(cmd: u32) -> vdrm_ccmd_req {
    vdrm_ccmd_req {
        cmd,
        len: size_of::<T>() as u32,
        seqno: 0,
        rsp_off: 0,
    }
}

fn set_metadata(res_id: u32, tiling_info: u64) -> amdgpu_ccmd_set_metadata_req {
    amdgpu_ccmd_set_metadata_req {
        hdr: header::<amdgpu_ccmd_set_metadata_req>(AMDGPU_CCMD_SET_METADATA),
        flags: 0,
        tiling_info,
        res_id,
        size_metadata: 0,
        umd_metadata: [],
    }
}

#[repr(C)]
struct SetTiling {
    req: i915_ccmd_ioctl_simple_req,
    tiling: drm_i915_gem_set_tiling,
}

fn set_tiling(res_id: u32, stride: u32) -> SetTiling {
    SetTiling {
        req: i915_ccmd_ioctl_simple_req {
            hdr: header::<SetTiling>(I915_CCMD_IOCTL_SIMPLE),
            cmd: DRM_IOCTL_I915_GEM_SET_TILING,
            pad: 0,
            payload: [],
        },
        tiling: drm_i915_gem_set_tiling {
            handle: res_id,
            tiling_mode: 1,
            stride,
            swizzle_mode: 0,
        },
    }
}

#[test]
fn created_handles_are_read_after_the_ioctl() {
    let mut create = drm_v3d_create_bo {
        size: 4096,
        flags: 2,
        handle: 0,
        offset: 0,
    };
    let request = DRM_IOCTL_V3D_CREATE_BO;
    let traced = trace(&driver("v3d"), request, &mut create, |create| {
        create.handle = 7;
    });
    let expected = Traced {
        handle: Some(7),
        call: Call::V3dCreateBo {
            size: 4096,
            flags: 2,
        },
    };
    assert_eq!(traced, [expected]);
}

#[test]
fn requests_of_other_drivers_are_not_decoded() {
    let mut create = drm_v3d_create_bo {
        size: 4096,
        flags: 0,
        handle: 0,
        offset: 0,
    };
    let request = DRM_IOCTL_V3D_CREATE_BO;
    assert_eq!(trace(&driver("vc4"), request, &mut create, |_| ()), []);
}

#[test]
fn udmabufs_have_no_handle() {
    let mut create = udmabuf_create {
        memfd: 5,
        flags: 1,
        offset: 0,
        size: 8192,
    };
    let traced = trace(&driver("udmabuf"), UDMABUF_CREATE, &mut create, |_| ());
    let expected = Traced {
        handle: None,
        call: Call::UdmabufCreate {
            flags: 1,
            offset: 0,
            size: 8192,
        },
    };
    assert_eq!(traced, [expected]);
}

#[test]
fn metadata_words_are_limited_to_the_array() {
    let mut metadata: drm_amdgpu_gem_metadata = unsafe { std::mem::zeroed() };
    metadata.handle = 3;
    metadata.op = AMDGPU_GEM_METADATA_OP_SET_METADATA;
    metadata.data.data_size_bytes = u32::MAX;
    metadata.data.data[0] = 9;
    let request = DRM_IOCTL_AMDGPU_GEM_METADATA;
    let traced = trace(&driver("amdgpu"), request, &mut metadata, |_| ());
    let [traced] = traced.as_slice() else {
        panic!("{} calls were decoded", traced.len());
    };
    let Call::AmdgpuGemMetadata { data, .. } = &traced.call else {
        panic!("{} was decoded", traced.call);
    };
    assert_eq!(data.len(), metadata.data.data.len());
    assert_eq!(data[0], 9);
}

fn set_pat(next_extension: u64, name: u32) -> drm_i915_gem_create_ext_set_pat {
    let mut pat: drm_i915_gem_create_ext_set_pat = unsafe { std::mem::zeroed() };
    pat.base.next_extension = next_extension;
    pat.base.name = name;
    pat.pat_index = 3;
    pat
}

fn create_ext(extensions: u64) -> drm_i915_gem_create_ext {
    drm_i915_gem_create_ext {
        size: 4096,
        handle: 0,
        flags: 0,
        extensions,
    }
}

#[test]
fn extension_chains_are_followed() {
    let mut pat = set_pat(0, I915_GEM_CREATE_EXT_SET_PAT);
    let mut create = create_ext(address(&mut pat));
    let request = DRM_IOCTL_I915_GEM_CREATE_EXT;
    let traced = trace(&driver("i915"), request, &mut create, |create| {
        create.handle = 2;
    });
    let expected = Traced {
        handle: Some(2),
        call: Call::I915GemCreateExt {
            size: 4096,
            flags: 0,
            regions: None,
            pat_index: Some(3),
        },
    };
    assert_eq!(traced, [expected]);
}

#[test]
fn cyclic_extension_chains_are_not_decoded() {
    let mut pat = set_pat(0, I915_GEM_CREATE_EXT_SET_PAT);
    pat.base.next_extension = address(&mut pat);
    let mut create = create_ext(address(&mut pat));
    let request = DRM_IOCTL_I915_GEM_CREATE_EXT;
    assert_eq!(trace(&driver("i915"), request, &mut create, |_| ()), []);
}

#[test]
fn unknown_extensions_are_not_decoded() {
    let mut pat = set_pat(0, u32::MAX);
    let mut create = create_ext(address(&mut pat));
    let request = DRM_IOCTL_I915_GEM_CREATE_EXT;
    assert_eq!(trace(&driver("i915"), request, &mut create, |_| ()), []);
}

#[test]
fn every_set_metadata_of_an_execbuffer_is_decoded() {
    let mut buffer = Vec::new();
    command(&mut buffer, set_metadata(1, 0x10));
    command(&mut buffer, set_metadata(2, 0x20));
    let size = buffer.len() * size_of::<u64>();
    let mut execbuffer = execbuffer(&mut buffer, size);
    let kernel = virtio(VIRTGPU_DRM_CONTEXT_AMDGPU);
    let request = DRM_IOCTL_VIRTGPU_EXECBUFFER;
    let traced = trace(&kernel, request, &mut execbuffer, |_| ());
    let expected = [1u32, 2].map(|res_id| Traced {
        handle: Some(RESOURCE_HANDLE_BASE + res_id),
        call: Call::VirtgpuAmdgpuSetMetadata {
            flags: 0,
            tiling_info: res_id as u64 * 0x10,
        },
    });
    assert_eq!(traced, expected);
}

#[test]
fn commands_beyond_the_execbuffer_are_not_decoded() {
    let mut buffer = Vec::new();
    command(&mut buffer, set_metadata(1, 0x10));
    command(&mut buffer, set_metadata(2, 0x20));
    let size = buffer.len() * size_of::<u64>() - 1;
    let mut execbuffer = execbuffer(&mut buffer, size);
    let kernel = virtio(VIRTGPU_DRM_CONTEXT_AMDGPU);
    let request = DRM_IOCTL_VIRTGPU_EXECBUFFER;
    let traced = trace(&kernel, request, &mut execbuffer, |_| ());
    assert_eq!(traced.len(), 1);
    assert_eq!(traced[0].handle, Some(RESOURCE_HANDLE_BASE + 1));
}

#[test]
fn simple_i915_commands_name_their_resource() {
    let mut buffer = Vec::new();
    command(&mut buffer, set_tiling(4, 512));
    let size = buffer.len() * size_of::<u64>();
    let mut execbuffer = execbuffer(&mut buffer, size);
    let kernel = virtio(VIRTGPU_DRM_CONTEXT_I915);
    let request = DRM_IOCTL_VIRTGPU_EXECBUFFER;
    let traced = trace(&kernel, request, &mut execbuffer, |_| ());
    let expected = Traced {
        handle: Some(RESOURCE_HANDLE_BASE + 4),
        call: Call::I915GemSetTiling {
            tiling_mode: 1,
            stride: 512,
        },
    };
    assert_eq!(traced, [expected]);
}

#[test]
fn truncated_simple_i915_commands_are_not_decoded() {
    let mut buffer = Vec::new();
    let mut tiling = set_tiling(4, 512);
    tiling.req.hdr.len = size_of::<i915_ccmd_ioctl_simple_req>() as u32;
    command(&mut buffer, tiling);
    let size = buffer.len() * size_of::<u64>();
    let mut execbuffer = execbuffer(&mut buffer, size);
    let kernel = virtio(VIRTGPU_DRM_CONTEXT_I915);
    let request = DRM_IOCTL_VIRTGPU_EXECBUFFER;
    assert_eq!(trace(&kernel, request, &mut execbuffer, |_| ()), []);
}
