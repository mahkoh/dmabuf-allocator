use crate::calls::Call;
use crate::calls::Region;
use crate::calls::Traced;
use bytemuck::AnyBitPattern;
use fake_kernel::Kernel;
use fake_kernel::UserMemory;
use fake_kernel::uapi::amdgpu::AMDGPU_GEM_METADATA_OP_SET_METADATA;
use fake_kernel::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_CREATE;
use fake_kernel::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_METADATA;
use fake_kernel::uapi::amdgpu::drm_amdgpu_gem_create;
use fake_kernel::uapi::amdgpu::drm_amdgpu_gem_metadata;
use fake_kernel::uapi::asahi::DRM_IOCTL_ASAHI_GEM_CREATE;
use fake_kernel::uapi::asahi::drm_asahi_gem_create;
use fake_kernel::uapi::drm::DRM_IOCTL_MODE_CREATE_DUMB;
use fake_kernel::uapi::drm::DRM_IOCTL_PRIME_HANDLE_TO_FD;
use fake_kernel::uapi::drm::drm_mode_create_dumb;
use fake_kernel::uapi::drm::drm_prime_handle;
use fake_kernel::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GEM_NEW;
use fake_kernel::uapi::etnaviv::drm_etnaviv_gem_new;
use fake_kernel::uapi::i915::DRM_IOCTL_I915_GEM_CREATE;
use fake_kernel::uapi::i915::DRM_IOCTL_I915_GEM_CREATE_EXT;
use fake_kernel::uapi::i915::DRM_IOCTL_I915_GEM_SET_DOMAIN;
use fake_kernel::uapi::i915::DRM_IOCTL_I915_GEM_SET_TILING;
use fake_kernel::uapi::i915::I915_GEM_CREATE_EXT_MEMORY_REGIONS;
use fake_kernel::uapi::i915::I915_GEM_CREATE_EXT_SET_PAT;
use fake_kernel::uapi::i915::drm_i915_gem_create;
use fake_kernel::uapi::i915::drm_i915_gem_create_ext;
use fake_kernel::uapi::i915::drm_i915_gem_create_ext_memory_regions;
use fake_kernel::uapi::i915::drm_i915_gem_create_ext_set_pat;
use fake_kernel::uapi::i915::drm_i915_gem_memory_class_instance;
use fake_kernel::uapi::i915::drm_i915_gem_set_domain;
use fake_kernel::uapi::i915::drm_i915_gem_set_tiling;
use fake_kernel::uapi::i915::i915_user_extension;
use fake_kernel::uapi::lima::DRM_IOCTL_LIMA_GEM_CREATE;
use fake_kernel::uapi::lima::drm_lima_gem_create;
use fake_kernel::uapi::msm::DRM_IOCTL_MSM_GEM_NEW;
use fake_kernel::uapi::msm::drm_msm_gem_new;
use fake_kernel::uapi::nouveau::DRM_IOCTL_NOUVEAU_GEM_NEW;
use fake_kernel::uapi::nouveau::drm_nouveau_gem_new;
use fake_kernel::uapi::nvidia_drm::DRM_IOCTL_NVIDIA_GEM_ALLOC_NVKMS_MEMORY;
use fake_kernel::uapi::nvidia_drm::drm_nvidia_gem_alloc_nvkms_memory_params;
use fake_kernel::uapi::panfrost::DRM_IOCTL_PANFROST_CREATE_BO;
use fake_kernel::uapi::panfrost::drm_panfrost_create_bo;
use fake_kernel::uapi::panthor::DRM_IOCTL_PANTHOR_BO_CREATE;
use fake_kernel::uapi::panthor::drm_panthor_bo_create;
use fake_kernel::uapi::udmabuf::UDMABUF_CREATE;
use fake_kernel::uapi::udmabuf::udmabuf_create;
use fake_kernel::uapi::v3d::DRM_IOCTL_V3D_CREATE_BO;
use fake_kernel::uapi::v3d::drm_v3d_create_bo;
use fake_kernel::uapi::vc4::DRM_IOCTL_VC4_CREATE_BO;
use fake_kernel::uapi::vc4::DRM_IOCTL_VC4_SET_TILING;
use fake_kernel::uapi::vc4::drm_vc4_create_bo;
use fake_kernel::uapi::vc4::drm_vc4_set_tiling;
use fake_kernel::uapi::vdrm::AMDGPU_CCMD_GEM_NEW;
use fake_kernel::uapi::vdrm::AMDGPU_CCMD_SET_METADATA;
use fake_kernel::uapi::vdrm::ASAHI_CCMD_GEM_NEW;
use fake_kernel::uapi::vdrm::I915_CCMD_GEM_CREATE;
use fake_kernel::uapi::vdrm::I915_CCMD_GEM_CREATE_EXT;
use fake_kernel::uapi::vdrm::I915_CCMD_IOCTL_SIMPLE;
use fake_kernel::uapi::vdrm::PANFROST_CCMD_CREATE_BO;
use fake_kernel::uapi::vdrm::VIRTGPU_DRM_CONTEXT_AMDGPU;
use fake_kernel::uapi::vdrm::VIRTGPU_DRM_CONTEXT_ASAHI;
use fake_kernel::uapi::vdrm::VIRTGPU_DRM_CONTEXT_I915;
use fake_kernel::uapi::vdrm::VIRTGPU_DRM_CONTEXT_PANFROST;
use fake_kernel::uapi::vdrm::amdgpu_ccmd_gem_new_req;
use fake_kernel::uapi::vdrm::amdgpu_ccmd_set_metadata_req;
use fake_kernel::uapi::vdrm::asahi_ccmd_gem_new_req;
use fake_kernel::uapi::vdrm::i915_ccmd_gem_create_ext_req;
use fake_kernel::uapi::vdrm::i915_ccmd_gem_create_req;
use fake_kernel::uapi::vdrm::i915_ccmd_ioctl_simple_req;
use fake_kernel::uapi::vdrm::panfrost_ccmd_create_bo_req;
use fake_kernel::uapi::vdrm::vdrm_ccmd_req;
use fake_kernel::uapi::virtgpu::DRM_IOCTL_VIRTGPU_EXECBUFFER;
use fake_kernel::uapi::virtgpu::DRM_IOCTL_VIRTGPU_RESOURCE_CREATE;
use fake_kernel::uapi::virtgpu::DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB;
use fake_kernel::uapi::virtgpu::drm_virtgpu_execbuffer;
use fake_kernel::uapi::virtgpu::drm_virtgpu_resource_create;
use fake_kernel::uapi::virtgpu::drm_virtgpu_resource_create_blob;
use fake_kernel::uapi::xe::DRM_IOCTL_XE_GEM_CREATE;
use fake_kernel::uapi::xe::drm_xe_gem_create;

#[cfg(test)]
mod tests;

const MAX_EXTENSIONS: usize = 512;

type ReadHandle = Box<dyn Fn(&dyn UserMemory) -> Option<u32>>;

enum Handle {
    None,
    Output(ReadHandle),
    Resource(u32),
}

pub struct Pending {
    call: Call,
    handle: Handle,
}

impl Pending {
    pub fn finish(self, kernel: &dyn Kernel, file: u64, user: &dyn UserMemory) -> Option<Traced> {
        let handle = match self.handle {
            Handle::None => None,
            Handle::Output(read) => Some(read(user)?),
            Handle::Resource(res_id) => Some(kernel.drm()?.resource_handle(file, res_id)?),
        };
        Some(Traced {
            handle,
            call: self.call,
        })
    }
}

unsafe fn load<T: AnyBitPattern>(user: &dyn UserMemory, address: u64) -> Option<T> {
    user.read_value(address).ok()
}

unsafe fn created<T: AnyBitPattern + 'static>(
    user: &dyn UserMemory,
    address: u64,
    call: impl FnOnce(&T) -> Option<Call>,
    handle: fn(&T) -> u32,
) -> Option<Pending> {
    let args: T = unsafe { load(user, address)? };
    let read = move |user: &dyn UserMemory| {
        let args: T = unsafe { load(user, address)? };
        Some(handle(&args))
    };
    Some(Pending {
        call: call(&args)?,
        handle: Handle::Output(Box::new(read)),
    })
}

fn blob(
    user: &dyn UserMemory,
    address: u64,
    call: impl FnOnce(&drm_virtgpu_resource_create_blob) -> Option<Call>,
) -> Option<Pending> {
    unsafe { created(user, address, call, |create| create.bo_handle) }
}

unsafe fn command<T: AnyBitPattern>(
    user: &dyn UserMemory,
    create: &drm_virtgpu_resource_create_blob,
) -> Option<T> {
    if (create.cmd_size as usize) < size_of::<T>() {
        return None;
    }
    unsafe { load(user, create.cmd) }
}

struct Extensions {
    regions: Option<Vec<Region>>,
    pat_index: Option<u32>,
}

fn regions(array: &[drm_i915_gem_memory_class_instance]) -> Vec<Region> {
    array
        .iter()
        .map(|region| Region {
            memory_class: region.memory_class,
            memory_instance: region.memory_instance,
        })
        .collect()
}

fn i915_inline_extensions(user: &dyn UserMemory, start: u64, size: u64) -> Option<Extensions> {
    let fits = |offset: u64, len: usize| size - offset >= len as u64;
    let mut extensions = Extensions {
        regions: None,
        pat_index: None,
    };
    let mut offset = 0;
    while offset < size {
        if !fits(offset, size_of::<i915_user_extension>()) {
            return None;
        }
        let address = start.checked_add(offset)?;
        let base: i915_user_extension = unsafe { load(user, address)? };
        match base.name {
            I915_GEM_CREATE_EXT_MEMORY_REGIONS => {
                let header = size_of::<drm_i915_gem_create_ext_memory_regions>();
                if !fits(offset, header) {
                    return None;
                }
                let ext: drm_i915_gem_create_ext_memory_regions = unsafe { load(user, address)? };
                offset += header as u64;
                let count = ext.num_regions as usize;
                let len = count * size_of::<drm_i915_gem_memory_class_instance>();
                if !fits(offset, len) {
                    return None;
                }
                let array: Vec<drm_i915_gem_memory_class_instance> =
                    user.read_array(start.checked_add(offset)?, count).ok()?;
                offset += len as u64;
                extensions.regions = Some(regions(&array));
            }
            I915_GEM_CREATE_EXT_SET_PAT => {
                let len = size_of::<drm_i915_gem_create_ext_set_pat>();
                if !fits(offset, len) {
                    return None;
                }
                let ext: drm_i915_gem_create_ext_set_pat = unsafe { load(user, address)? };
                offset += len as u64;
                extensions.pat_index = Some(ext.pat_index);
            }
            _ => return None,
        }
    }
    Some(extensions)
}

fn i915_extensions(user: &dyn UserMemory, mut next: u64) -> Option<Extensions> {
    let mut extensions = Extensions {
        regions: None,
        pat_index: None,
    };
    for _ in 0..MAX_EXTENSIONS {
        if next == 0 {
            return Some(extensions);
        }
        let base: i915_user_extension = unsafe { load(user, next)? };
        match base.name {
            I915_GEM_CREATE_EXT_MEMORY_REGIONS => {
                let ext: drm_i915_gem_create_ext_memory_regions = unsafe { load(user, next)? };
                let array: Vec<drm_i915_gem_memory_class_instance> = user
                    .read_array(ext.regions, ext.num_regions as usize)
                    .ok()?;
                extensions.regions = Some(regions(&array));
            }
            I915_GEM_CREATE_EXT_SET_PAT => {
                let ext: drm_i915_gem_create_ext_set_pat = unsafe { load(user, next)? };
                extensions.pat_index = Some(ext.pat_index);
            }
            _ => return None,
        }
        next = base.next_extension;
    }
    match next {
        0 => Some(extensions),
        _ => None,
    }
}

struct Command {
    cmd: u32,
    address: u64,
    len: u64,
}

fn commands(user: &dyn UserMemory, arg: u64) -> Option<Vec<Command>> {
    let execbuffer: drm_virtgpu_execbuffer = unsafe { load(user, arg)? };
    let header_size = size_of::<vdrm_ccmd_req>() as u64;
    let size = execbuffer.size as u64;
    let mut commands = Vec::new();
    let mut offset = 0;
    while size - offset >= header_size {
        let address = execbuffer.command.checked_add(offset)?;
        let hdr: vdrm_ccmd_req = unsafe { load(user, address)? };
        let len = hdr.len as u64;
        if len < header_size || size - offset < len {
            break;
        }
        commands.push(Command {
            cmd: hdr.cmd,
            address,
            len,
        });
        offset += len;
    }
    Some(commands)
}

fn amdgpu_command(
    user: &dyn UserMemory,
    command: &Command,
    pending: &mut Vec<Pending>,
) -> Option<()> {
    let traced = command.cmd == AMDGPU_CCMD_SET_METADATA
        && command.len >= size_of::<amdgpu_ccmd_set_metadata_req>() as u64;
    if !traced {
        return Some(());
    }
    let req: amdgpu_ccmd_set_metadata_req = unsafe { load(user, command.address)? };
    pending.push(Pending {
        call: Call::VirtgpuAmdgpuSetMetadata {
            flags: req.flags,
            tiling_info: req.tiling_info,
        },
        handle: Handle::Resource(req.res_id),
    });
    Some(())
}

fn i915_command(
    user: &dyn UserMemory,
    command: &Command,
    pending: &mut Vec<Pending>,
) -> Option<()> {
    let header = size_of::<i915_ccmd_ioctl_simple_req>() as u64;
    if command.cmd != I915_CCMD_IOCTL_SIMPLE || command.len < header {
        return Some(());
    }
    let req: i915_ccmd_ioctl_simple_req = unsafe { load(user, command.address)? };
    let payload = command.address.checked_add(header)?;
    let fits = |len: usize| command.len - header >= len as u64;
    match req.cmd {
        DRM_IOCTL_I915_GEM_SET_TILING if fits(size_of::<drm_i915_gem_set_tiling>()) => {
            let tiling: drm_i915_gem_set_tiling = unsafe { load(user, payload)? };
            pending.push(Pending {
                call: Call::I915GemSetTiling {
                    tiling_mode: tiling.tiling_mode,
                    stride: tiling.stride,
                },
                handle: Handle::Resource(tiling.handle),
            });
        }
        DRM_IOCTL_I915_GEM_SET_DOMAIN if fits(size_of::<drm_i915_gem_set_domain>()) => {
            let domain: drm_i915_gem_set_domain = unsafe { load(user, payload)? };
            pending.push(Pending {
                call: Call::I915GemSetDomain {
                    read_domains: domain.read_domains,
                    write_domain: domain.write_domain,
                },
                handle: Handle::Resource(domain.handle),
            });
        }
        _ => {}
    }
    Some(())
}

fn execbuffer(
    user: &dyn UserMemory,
    arg: u64,
    decode: fn(&dyn UserMemory, &Command, &mut Vec<Pending>) -> Option<()>,
) -> Option<Vec<Pending>> {
    let mut pending = Vec::new();
    for command in commands(user, arg)? {
        decode(user, &command, &mut pending)?;
    }
    Some(pending)
}

pub fn decode(
    kernel: &dyn Kernel,
    file: u64,
    request: u32,
    arg: u64,
    user: &dyn UserMemory,
) -> Vec<Pending> {
    let context = kernel.drm().and_then(|drm| drm.context_type(file));
    if request == DRM_IOCTL_VIRTGPU_EXECBUFFER {
        match context {
            Some(VIRTGPU_DRM_CONTEXT_AMDGPU) => {
                return execbuffer(user, arg, amdgpu_command).unwrap_or_default();
            }
            Some(VIRTGPU_DRM_CONTEXT_I915) => {
                return execbuffer(user, arg, i915_command).unwrap_or_default();
            }
            _ => {}
        }
    }
    decode_one(kernel.name(), context, request, arg, user)
        .into_iter()
        .collect()
}

fn decode_one(
    driver: &str,
    context: Option<u32>,
    request: u32,
    arg: u64,
    user: &dyn UserMemory,
) -> Option<Pending> {
    unsafe {
        match request {
            DRM_IOCTL_PRIME_HANDLE_TO_FD => created(
                user,
                arg,
                |prime: &drm_prime_handle| Some(Call::PrimeHandleToFd { flags: prime.flags }),
                |prime| prime.handle,
            ),
            DRM_IOCTL_AMDGPU_GEM_CREATE if driver == "amdgpu" => created(
                user,
                arg,
                |create: &drm_amdgpu_gem_create| {
                    Some(Call::AmdgpuGemCreate {
                        size: create.in_.bo_size,
                        alignment: create.in_.alignment,
                        domains: create.in_.domains,
                        flags: create.in_.domain_flags,
                    })
                },
                |create| create.out.handle,
            ),
            DRM_IOCTL_AMDGPU_GEM_METADATA if driver == "amdgpu" => created(
                user,
                arg,
                |metadata: &drm_amdgpu_gem_metadata| {
                    if metadata.op != AMDGPU_GEM_METADATA_OP_SET_METADATA {
                        return None;
                    }
                    let data = &metadata.data.data;
                    let words =
                        (metadata.data.data_size_bytes as usize / size_of::<u32>()).min(data.len());
                    Some(Call::AmdgpuGemMetadata {
                        flags: metadata.data.flags,
                        tiling_info: metadata.data.tiling_info,
                        data: data[..words].to_vec(),
                    })
                },
                |metadata| metadata.handle,
            ),
            DRM_IOCTL_I915_GEM_CREATE if driver == "i915" => created(
                user,
                arg,
                |create: &drm_i915_gem_create| Some(Call::I915GemCreate { size: create.size }),
                |create| create.handle,
            ),
            DRM_IOCTL_ASAHI_GEM_CREATE if driver == "asahi" => created(
                user,
                arg,
                |create: &drm_asahi_gem_create| {
                    Some(Call::AsahiGemCreate {
                        size: create.size,
                        flags: create.flags,
                    })
                },
                |create| create.handle,
            ),
            DRM_IOCTL_PANFROST_CREATE_BO if driver == "panfrost" => created(
                user,
                arg,
                |create: &drm_panfrost_create_bo| {
                    Some(Call::PanfrostCreateBo {
                        size: create.size,
                        flags: create.flags,
                    })
                },
                |create| create.handle,
            ),
            DRM_IOCTL_PANTHOR_BO_CREATE if driver == "panthor" => created(
                user,
                arg,
                |create: &drm_panthor_bo_create| {
                    Some(Call::PanthorBoCreate {
                        size: create.size,
                        flags: create.flags,
                    })
                },
                |create| create.handle,
            ),
            DRM_IOCTL_V3D_CREATE_BO if driver == "v3d" => created(
                user,
                arg,
                |create: &drm_v3d_create_bo| {
                    Some(Call::V3dCreateBo {
                        size: create.size,
                        flags: create.flags,
                    })
                },
                |create| create.handle,
            ),
            DRM_IOCTL_VC4_CREATE_BO if driver == "vc4" => created(
                user,
                arg,
                |create: &drm_vc4_create_bo| {
                    Some(Call::Vc4CreateBo {
                        size: create.size,
                        flags: create.flags,
                    })
                },
                |create| create.handle,
            ),
            DRM_IOCTL_VC4_SET_TILING if driver == "vc4" => created(
                user,
                arg,
                |tiling: &drm_vc4_set_tiling| {
                    Some(Call::Vc4SetTiling {
                        flags: tiling.flags,
                        modifier: tiling.modifier,
                    })
                },
                |tiling| tiling.handle,
            ),
            DRM_IOCTL_LIMA_GEM_CREATE if driver == "lima" => created(
                user,
                arg,
                |create: &drm_lima_gem_create| {
                    Some(Call::LimaGemCreate {
                        size: create.size,
                        flags: create.flags,
                    })
                },
                |create| create.handle,
            ),
            DRM_IOCTL_ETNAVIV_GEM_NEW if driver == "etnaviv" => created(
                user,
                arg,
                |create: &drm_etnaviv_gem_new| {
                    Some(Call::EtnavivGemNew {
                        size: create.size,
                        flags: create.flags,
                    })
                },
                |create| create.handle,
            ),
            DRM_IOCTL_VIRTGPU_RESOURCE_CREATE if driver == "virtio_gpu" => created(
                user,
                arg,
                |create: &drm_virtgpu_resource_create| {
                    Some(Call::VirtgpuResourceCreate {
                        target: create.target,
                        format: create.format,
                        bind: create.bind,
                        width: create.width,
                        height: create.height,
                        depth: create.depth,
                        array_size: create.array_size,
                        last_level: create.last_level,
                        nr_samples: create.nr_samples,
                        flags: create.flags,
                        size: create.size,
                        stride: create.stride,
                    })
                },
                |create| create.bo_handle,
            ),
            DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB
                if context == Some(VIRTGPU_DRM_CONTEXT_AMDGPU) =>
            {
                blob(user, arg, |create| {
                    let req: amdgpu_ccmd_gem_new_req = command(user, create)?;
                    if req.hdr.cmd != AMDGPU_CCMD_GEM_NEW {
                        return None;
                    }
                    Some(Call::VirtgpuAmdgpuGemNew {
                        blob_mem: create.blob_mem,
                        blob_flags: create.blob_flags,
                        blob_hints: create.blob_hints,
                        size: create.size,
                        alloc_size: req.r.alloc_size,
                        phys_alignment: req.r.phys_alignment,
                        preferred_heap: req.r.preferred_heap,
                        flags: req.r.flags,
                    })
                })
            }
            DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB
                if context == Some(VIRTGPU_DRM_CONTEXT_PANFROST) =>
            {
                blob(user, arg, |create| {
                    let req: panfrost_ccmd_create_bo_req = command(user, create)?;
                    if req.hdr.cmd != PANFROST_CCMD_CREATE_BO {
                        return None;
                    }
                    Some(Call::VirtgpuPanfrostCreateBo {
                        blob_mem: create.blob_mem,
                        blob_flags: create.blob_flags,
                        blob_hints: create.blob_hints,
                        size: create.size,
                        gem_size: req.size as u64,
                        flags: req.flags,
                    })
                })
            }
            DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB
                if context == Some(VIRTGPU_DRM_CONTEXT_ASAHI) =>
            {
                blob(user, arg, |create| {
                    let req: asahi_ccmd_gem_new_req = command(user, create)?;
                    if req.hdr.cmd != ASAHI_CCMD_GEM_NEW {
                        return None;
                    }
                    Some(Call::VirtgpuAsahiGemNew {
                        blob_mem: create.blob_mem,
                        blob_flags: create.blob_flags,
                        blob_hints: create.blob_hints,
                        size: create.size,
                        gem_size: req.size,
                        flags: req.flags,
                    })
                })
            }
            DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB if context == Some(VIRTGPU_DRM_CONTEXT_I915) => {
                blob(user, arg, |create| {
                    let hdr: vdrm_ccmd_req = command(user, create)?;
                    match hdr.cmd {
                        I915_CCMD_GEM_CREATE => {
                            let req: i915_ccmd_gem_create_req = command(user, create)?;
                            Some(Call::VirtgpuI915GemCreate {
                                blob_mem: create.blob_mem,
                                blob_flags: create.blob_flags,
                                blob_hints: create.blob_hints,
                                size: create.size,
                                gem_size: req.size,
                            })
                        }
                        I915_CCMD_GEM_CREATE_EXT => {
                            let req: i915_ccmd_gem_create_ext_req = command(user, create)?;
                            let header = size_of::<i915_ccmd_gem_create_ext_req>() as u64;
                            let size = req.ext_size as u64;
                            if create.cmd_size as u64 - header < size {
                                return None;
                            }
                            let start = create.cmd.checked_add(header)?;
                            let extensions = i915_inline_extensions(user, start, size)?;
                            Some(Call::VirtgpuI915GemCreateExt {
                                blob_mem: create.blob_mem,
                                blob_flags: create.blob_flags,
                                blob_hints: create.blob_hints,
                                size: create.size,
                                gem_size: req.size,
                                flags: req.gem_flags,
                                regions: extensions.regions,
                                pat_index: extensions.pat_index,
                            })
                        }
                        _ => None,
                    }
                })
            }
            DRM_IOCTL_MODE_CREATE_DUMB => created(
                user,
                arg,
                |create: &drm_mode_create_dumb| {
                    Some(Call::ModeCreateDumb {
                        width: create.width,
                        height: create.height,
                        bpp: create.bpp,
                    })
                },
                |create| create.handle,
            ),
            DRM_IOCTL_I915_GEM_CREATE_EXT if driver == "i915" => created(
                user,
                arg,
                |create: &drm_i915_gem_create_ext| {
                    let extensions = i915_extensions(user, create.extensions)?;
                    Some(Call::I915GemCreateExt {
                        size: create.size,
                        flags: create.flags,
                        regions: extensions.regions,
                        pat_index: extensions.pat_index,
                    })
                },
                |create| create.handle,
            ),
            DRM_IOCTL_I915_GEM_SET_TILING if driver == "i915" => created(
                user,
                arg,
                |tiling: &drm_i915_gem_set_tiling| {
                    Some(Call::I915GemSetTiling {
                        tiling_mode: tiling.tiling_mode,
                        stride: tiling.stride,
                    })
                },
                |tiling| tiling.handle,
            ),
            DRM_IOCTL_I915_GEM_SET_DOMAIN if driver == "i915" => created(
                user,
                arg,
                |domain: &drm_i915_gem_set_domain| {
                    Some(Call::I915GemSetDomain {
                        read_domains: domain.read_domains,
                        write_domain: domain.write_domain,
                    })
                },
                |domain| domain.handle,
            ),
            DRM_IOCTL_XE_GEM_CREATE if driver == "xe" => created(
                user,
                arg,
                |create: &drm_xe_gem_create| {
                    Some(Call::XeGemCreate {
                        size: create.size,
                        placement: create.placement,
                        flags: create.flags,
                        vm_id: create.vm_id,
                        cpu_caching: create.cpu_caching,
                    })
                },
                |create| create.handle,
            ),
            DRM_IOCTL_NVIDIA_GEM_ALLOC_NVKMS_MEMORY if driver == "nvidia-drm" => created(
                user,
                arg,
                |alloc: &drm_nvidia_gem_alloc_nvkms_memory_params| {
                    Some(Call::NvidiaGemAllocNvkmsMemory {
                        block_linear: alloc.block_linear,
                        compressible: alloc.compressible,
                        memory_size: alloc.memory_size,
                        flags: alloc.flags,
                    })
                },
                |alloc| alloc.handle,
            ),
            DRM_IOCTL_NOUVEAU_GEM_NEW if driver == "nouveau" => created(
                user,
                arg,
                |new: &drm_nouveau_gem_new| {
                    Some(Call::NouveauGemNew {
                        domain: new.info.domain,
                        size: new.info.size,
                        align: new.align,
                        tile_mode: new.info.tile_mode,
                        tile_flags: new.info.tile_flags,
                    })
                },
                |new| new.info.handle,
            ),
            DRM_IOCTL_MSM_GEM_NEW if driver == "msm" => created(
                user,
                arg,
                |new: &drm_msm_gem_new| {
                    Some(Call::MsmGemNew {
                        size: new.size,
                        flags: new.flags,
                    })
                },
                |new| new.handle,
            ),
            UDMABUF_CREATE if driver == "udmabuf" => {
                let create: udmabuf_create = load(user, arg)?;
                Some(Pending {
                    call: Call::UdmabufCreate {
                        flags: create.flags,
                        offset: create.offset,
                        size: create.size,
                    },
                    handle: Handle::None,
                })
            }
            _ => None,
        }
    }
}
