use crate::Buffers;
use crate::DriverFeature;
use crate::DriverVersion;
use crate::Drm;
use crate::Export;
use crate::Fault;
use crate::Fence;
use crate::Kernel;
use crate::KernelVersion;
use crate::Master;
use crate::NewHandle;
use crate::PAGE_SIZE;
use crate::Pci;
use crate::Prime;
use crate::Syncobjs;
use crate::UserMemory;
use crate::amd::devices::GfxLevel;
use crate::dumb::create_dumb;
use crate::id_fields;
use crate::ioctls;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::uapi::drm::drm_mode_create_dumb;
use crate::uapi::drm::drm_mode_destroy_dumb;
use crate::uapi::venus::virgl_renderer_capset_venus;
use crate::uapi::virtgpu::DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING;
use crate::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE;
use crate::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_MAPPABLE;
use crate::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
use crate::uapi::virtgpu::VIRTGPU_BLOB_MEM_GUEST;
use crate::uapi::virtgpu::VIRTGPU_BLOB_MEM_HOST3D;
use crate::uapi::virtgpu::VIRTGPU_BLOB_MEM_HOST3D_GUEST;
use crate::uapi::virtgpu::VIRTGPU_CONTEXT_PARAM_CAPSET_ID;
use crate::uapi::virtgpu::VIRTGPU_CONTEXT_PARAM_DEBUG_NAME;
use crate::uapi::virtgpu::VIRTGPU_CONTEXT_PARAM_NUM_RINGS;
use crate::uapi::virtgpu::VIRTGPU_CONTEXT_PARAM_POLL_RINGS_MASK;
use crate::uapi::virtgpu::VIRTGPU_DRM_CAPSET_DRM;
use crate::uapi::virtgpu::VIRTGPU_DRM_CAPSET_VENUS;
use crate::uapi::virtgpu::VIRTGPU_DRM_CAPSET_VIRGL;
use crate::uapi::virtgpu::VIRTGPU_DRM_CAPSET_VIRGL2;
use crate::uapi::virtgpu::VIRTGPU_EXECBUF_FENCE_FD_IN;
use crate::uapi::virtgpu::VIRTGPU_EXECBUF_FENCE_FD_OUT;
use crate::uapi::virtgpu::VIRTGPU_EXECBUF_FLAGS;
use crate::uapi::virtgpu::VIRTGPU_EXECBUF_RING_IDX;
use crate::uapi::virtgpu::VIRTGPU_EXECBUF_SYNCOBJ_FLAGS;
use crate::uapi::virtgpu::VIRTGPU_EXECBUF_SYNCOBJ_RESET;
use crate::uapi::virtgpu::VIRTGPU_PARAM_3D_FEATURES;
use crate::uapi::virtgpu::VIRTGPU_PARAM_BLOB_ALIGNMENT;
use crate::uapi::virtgpu::VIRTGPU_PARAM_CAPSET_QUERY_FIX;
use crate::uapi::virtgpu::VIRTGPU_PARAM_CONTEXT_INIT;
use crate::uapi::virtgpu::VIRTGPU_PARAM_CROSS_DEVICE;
use crate::uapi::virtgpu::VIRTGPU_PARAM_EXPLICIT_DEBUG_NAME;
use crate::uapi::virtgpu::VIRTGPU_PARAM_HOST_VISIBLE;
use crate::uapi::virtgpu::VIRTGPU_PARAM_RESOURCE_BLOB;
use crate::uapi::virtgpu::VIRTGPU_PARAM_SUPPORTED_CAPSET_IDs;
use crate::uapi::virtgpu::drm_virtgpu_3d_wait;
use crate::uapi::virtgpu::drm_virtgpu_context_init;
use crate::uapi::virtgpu::drm_virtgpu_context_set_param;
use crate::uapi::virtgpu::drm_virtgpu_execbuffer;
use crate::uapi::virtgpu::drm_virtgpu_execbuffer_syncobj;
use crate::uapi::virtgpu::drm_virtgpu_get_caps;
use crate::uapi::virtgpu::drm_virtgpu_getparam;
use crate::uapi::virtgpu::drm_virtgpu_map;
use crate::uapi::virtgpu::drm_virtgpu_resource_create;
use crate::uapi::virtgpu::drm_virtgpu_resource_create_blob;
use crate::uapi::virtgpu::drm_virtgpu_resource_info;
use crate::user::from_bytes;
use crate::vdrm_host::Blob;
use crate::vdrm_host::Host as NativeHost;
use crate::vdrm_host::NativeHost as Native;
use crate::virgl_caps;
use crate::virtio_amdgpu;
use crate::virtio_asahi;
use crate::virtio_i915;
use crate::virtio_panfrost;
use libc::EEXIST;
use libc::EFAULT;
use libc::EINVAL;
use libc::ENOENT;
use libc::ENOSYS;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::mem;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

const PCI_VENDOR_ID_REDHAT_QUMRANET: u16 = 0x1af4;
const PCI_DEVICE_ID_VIRTIO_GPU: u16 = 0x1040 + VIRTIO_ID_GPU as u16;
const VIRTIO_ID_GPU: u32 = 16;
const FIRST_RESOURCE_ID: u32 = 2;
const NUM_SCANOUTS: u32 = 1;
const MAX_CAPSET_ID: u64 = 63;
const MAX_RINGS: u64 = 64;
const MAX_CONTEXT_PARAMS: u32 = 4;
const DEBUG_NAME_MAX_LEN: u64 = 65;
const VIRTGPU_BLOB_FLAG_USE_MASK: u32 = VIRTGPU_BLOB_FLAG_USE_MAPPABLE
    | VIRTGPU_BLOB_FLAG_USE_SHAREABLE
    | VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE;

pub struct Capset {
    id: u32,
    max_version: u32,
    data: fn() -> &'static [u8],
}

pub struct Host {
    pub name: &'static str,
    virgl_3d: bool,
    resource_blob: bool,
    host_visible: bool,
    context_init: bool,
    capsets: &'static [Capset],
    native: Option<NativeDevice>,
    cross_device: bool,
    blob_alignment: Option<u32>,
}

fn virgl_capset() -> &'static [u8] {
    bytemuck::bytes_of(&virgl_caps::VIRGL)
}

fn virgl2_capset() -> &'static [u8] {
    bytemuck::bytes_of(&virgl_caps::VIRGL2)
}

fn virgl2_gles_capset() -> &'static [u8] {
    bytemuck::bytes_of(&virgl_caps::VIRGL2_GLES)
}

fn virgl2_format_modifier_capset() -> &'static [u8] {
    bytemuck::bytes_of(&virgl_caps::VIRGL2_FORMAT_MODIFIER)
}

fn virgl2_gles_no_tweak_capset() -> &'static [u8] {
    bytemuck::bytes_of(&virgl_caps::VIRGL2_GLES_NO_TWEAK)
}

static VENUS: virgl_renderer_capset_venus = virgl_renderer_capset_venus {
    wire_format_version: 0,
    vk_xml_version: 0,
    vk_ext_command_serialization_spec_version: 0,
    vk_mesa_venus_protocol_spec_version: 0,
    supports_blob_id_0: 0,
    vk_extension_mask1: [0; 32],
    allow_vk_wait_syncs: 0,
    supports_multiple_timelines: 0,
    use_guest_vram: 0,
};

fn venus_capset() -> &'static [u8] {
    bytemuck::bytes_of(&VENUS)
}

pub static HOSTS: &[Host] = &[
    Host {
        name: "2d",
        virgl_3d: false,
        resource_blob: false,
        host_visible: false,
        context_init: false,
        capsets: &[],
        native: None,
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "virgl",
        virgl_3d: true,
        resource_blob: false,
        host_visible: false,
        context_init: false,
        capsets: &[
            Capset {
                id: VIRTGPU_DRM_CAPSET_VIRGL,
                max_version: virgl_caps::VIRGL_MAX_VERSION,
                data: virgl_capset,
            },
            Capset {
                id: VIRTGPU_DRM_CAPSET_VIRGL2,
                max_version: virgl_caps::VIRGL2_MAX_VERSION,
                data: virgl2_capset,
            },
        ],
        native: None,
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "virgl_context_init",
        virgl_3d: true,
        resource_blob: false,
        host_visible: false,
        context_init: true,
        capsets: &[
            Capset {
                id: VIRTGPU_DRM_CAPSET_VIRGL,
                max_version: virgl_caps::VIRGL_MAX_VERSION,
                data: virgl_capset,
            },
            Capset {
                id: VIRTGPU_DRM_CAPSET_VIRGL2,
                max_version: virgl_caps::VIRGL2_MAX_VERSION,
                data: virgl2_capset,
            },
        ],
        native: None,
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "virgl1_context_init",
        virgl_3d: true,
        resource_blob: false,
        host_visible: false,
        context_init: true,
        capsets: &[Capset {
            id: VIRTGPU_DRM_CAPSET_VIRGL,
            max_version: virgl_caps::VIRGL_MAX_VERSION,
            data: virgl_capset,
        }],
        native: None,
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "virgl2_context_init",
        virgl_3d: true,
        resource_blob: false,
        host_visible: false,
        context_init: true,
        capsets: &[Capset {
            id: VIRTGPU_DRM_CAPSET_VIRGL2,
            max_version: virgl_caps::VIRGL2_MAX_VERSION,
            data: virgl2_capset,
        }],
        native: None,
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "venus",
        virgl_3d: true,
        resource_blob: false,
        host_visible: false,
        context_init: true,
        capsets: &[Capset {
            id: VIRTGPU_DRM_CAPSET_VENUS,
            max_version: 0,
            data: venus_capset,
        }],
        native: None,
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "virgl_gles",
        virgl_3d: true,
        resource_blob: false,
        host_visible: false,
        context_init: false,
        capsets: &[
            Capset {
                id: VIRTGPU_DRM_CAPSET_VIRGL,
                max_version: virgl_caps::VIRGL_MAX_VERSION,
                data: virgl_capset,
            },
            Capset {
                id: VIRTGPU_DRM_CAPSET_VIRGL2,
                max_version: virgl_caps::VIRGL2_MAX_VERSION,
                data: virgl2_gles_capset,
            },
        ],
        native: None,
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "virgl_gles_no_tweak",
        virgl_3d: true,
        resource_blob: false,
        host_visible: false,
        context_init: false,
        capsets: &[
            Capset {
                id: VIRTGPU_DRM_CAPSET_VIRGL,
                max_version: virgl_caps::VIRGL_MAX_VERSION,
                data: virgl_capset,
            },
            Capset {
                id: VIRTGPU_DRM_CAPSET_VIRGL2,
                max_version: virgl_caps::VIRGL2_MAX_VERSION,
                data: virgl2_gles_no_tweak_capset,
            },
        ],
        native: None,
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "virgl_format_modifier",
        virgl_3d: true,
        resource_blob: false,
        host_visible: false,
        context_init: false,
        capsets: &[
            Capset {
                id: VIRTGPU_DRM_CAPSET_VIRGL,
                max_version: virgl_caps::VIRGL_MAX_VERSION,
                data: virgl_capset,
            },
            Capset {
                id: VIRTGPU_DRM_CAPSET_VIRGL2,
                max_version: virgl_caps::VIRGL2_MAX_VERSION,
                data: virgl2_format_modifier_capset,
            },
        ],
        native: None,
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "amdgpu_navi21",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Amdgpu,
            device: "navi21",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "amdgpu_tahiti",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Amdgpu,
            device: "tahiti",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "amdgpu_fiji",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Amdgpu,
            device: "fiji",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "amdgpu_vega10",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Amdgpu,
            device: "vega10",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "i915_skl_gt2",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::I915,
            device: "skl_gt2",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "i915_tgl_gt2",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::I915,
            device: "tgl_gt2",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "i915_dg2_g10",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::I915,
            device: "dg2_g10",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "i915_mtl_u",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::I915,
            device: "mtl_u",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "i915_tgl_gt2_cross_device",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::I915,
            device: "tgl_gt2",
        }),
        cross_device: true,
        blob_alignment: None,
    },
    Host {
        name: "i915_tgl_gt2_64k",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::I915,
            device: "tgl_gt2",
        }),
        cross_device: false,
        blob_alignment: Some(65536),
    },
    Host {
        name: "amdgpu_navi21_64k",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Amdgpu,
            device: "navi21",
        }),
        cross_device: false,
        blob_alignment: Some(65536),
    },
    Host {
        name: "asahi_t8103",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Asahi,
            device: "t8103",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "asahi_t6000",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Asahi,
            device: "t6000",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "asahi_t6001",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Asahi,
            device: "t6001",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "asahi_t6002",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Asahi,
            device: "t6002",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "asahi_t8112",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Asahi,
            device: "t8112",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "panfrost_t720",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Panfrost,
            device: "t720",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "panfrost_t860",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Panfrost,
            device: "t860",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "panfrost_g52",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Panfrost,
            device: "g52",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "panfrost_g57",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Panfrost,
            device: "g57",
        }),
        cross_device: false,
        blob_alignment: None,
    },
    Host {
        name: "panfrost_g52_cross_device",
        virgl_3d: true,
        resource_blob: true,
        host_visible: true,
        context_init: true,
        capsets: &[],
        native: Some(NativeDevice {
            driver: NativeDriver::Panfrost,
            device: "g52",
        }),
        cross_device: true,
        blob_alignment: None,
    },
];

#[derive(Copy, Clone)]
enum NativeDriver {
    Amdgpu,
    I915,
    Asahi,
    Panfrost,
}

#[derive(Copy, Clone)]
struct NativeDevice {
    driver: NativeDriver,
    device: &'static str,
}

impl NativeDevice {
    fn host(self, kernel: KernelVersion) -> Box<dyn NativeHost> {
        let device = self.device;
        match self.driver {
            NativeDriver::Amdgpu => Box::new(Native::new(virtio_amdgpu::Host::new(device, kernel))),
            NativeDriver::I915 => Box::new(Native::new(virtio_i915::Host::new(device, kernel))),
            NativeDriver::Asahi => Box::new(Native::new(virtio_asahi::Host::new(device, kernel))),
            NativeDriver::Panfrost => {
                Box::new(Native::new(virtio_panfrost::Host::new(device, kernel)))
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Buffer {
    res_handle: u32,
    blob_mem: u32,
}

#[derive(Default)]
struct FileState {
    capset_id: u64,
    base_fence_ctx: bool,
    num_rings: u64,
    ring_idx_mask: u64,
    explicit_debug_name: bool,
    context_created: bool,
    native: bool,
    pending: Vec<Vec<u8>>,
}

struct PostDep {
    object: u64,
    point: u64,
}

pub struct Device {
    host: &'static Host,
    kernel: KernelVersion,
    buffers: Buffers<Buffer>,
    next_resource_id: AtomicU32,
    native: Option<Box<dyn NativeHost>>,
    files: Mutex<BTreeMap<u64, FileState>>,
    syncobjs: Syncobjs,
    master: Master,
    deferred: AtomicBool,
}

impl Device {
    pub fn new(name: &str, kernel: KernelVersion) -> Self {
        let host = HOSTS
            .iter()
            .find(|host| host.name == name)
            .unwrap_or_else(|| panic!("unknown virtio_gpu host {name:?}"));
        Self {
            host,
            kernel,
            buffers: Buffers::default(),
            next_resource_id: AtomicU32::new(FIRST_RESOURCE_ID),
            native: host.native.map(|native| native.host(kernel)),
            files: Mutex::new(BTreeMap::new()),
            syncobjs: Syncobjs::default(),
            master: Master::default(),
            deferred: AtomicBool::new(false),
        }
    }

    pub fn from_id(fields: &str) -> Self {
        let values = id_fields(fields, &["host", "kernel"]);
        Self::new(
            values.get("host"),
            KernelVersion::parse(values.get("kernel")),
        )
    }

    fn capset(&self, id: u32, version: u32) -> Option<&[u8]> {
        if let Some(capset) = self
            .host
            .capsets
            .iter()
            .find(|capset| capset.id == id && capset.max_version >= version)
        {
            return Some((capset.data)());
        }
        match (&self.native, id, version) {
            (Some(native), VIRTGPU_DRM_CAPSET_DRM, 0) => Some(native.capset()),
            _ => None,
        }
    }

    fn capset_id_mask(&self) -> u64 {
        let mut mask = self
            .host
            .capsets
            .iter()
            .fold(0, |mask, capset| mask | 1 << capset.id);
        if self.native.is_some() {
            mask |= 1 << VIRTGPU_DRM_CAPSET_DRM;
        }
        mask
    }

    fn has_capsets(&self) -> bool {
        self.capset_id_mask() != 0
    }

    unsafe fn getparam(&self, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let args = unsafe { &*arg.cast::<drm_virtgpu_getparam>() };
        let host = self.host;
        let value: i32 = match args.param {
            VIRTGPU_PARAM_3D_FEATURES => host.virgl_3d as i32,
            VIRTGPU_PARAM_CAPSET_QUERY_FIX => 1,
            VIRTGPU_PARAM_RESOURCE_BLOB => host.resource_blob as i32,
            VIRTGPU_PARAM_HOST_VISIBLE => host.host_visible as i32,
            VIRTGPU_PARAM_CROSS_DEVICE => host.cross_device as i32,
            VIRTGPU_PARAM_CONTEXT_INIT => host.context_init as i32,
            param if param == VIRTGPU_PARAM_SUPPORTED_CAPSET_IDs => self.capset_id_mask() as i32,
            VIRTGPU_PARAM_EXPLICIT_DEBUG_NAME => host.context_init as i32,
            VIRTGPU_PARAM_BLOB_ALIGNMENT if self.kernel >= KernelVersion::V7_2 => {
                match host.blob_alignment {
                    Some(alignment) => alignment as i32,
                    None => return -(ENOENT as i64),
                }
            }
            _ => return -(EINVAL as i64),
        };
        match user.write(args.value, &value.to_ne_bytes()) {
            Ok(()) => 0,
            Err(_) => -(EFAULT as i64),
        }
    }

    fn new_buffer(&self, file: u64, size: u64, res_handle: u32, blob_mem: u32) -> u32 {
        let buffer = Buffer {
            res_handle,
            blob_mem,
        };
        let buffers = self.buffers.file(file);
        buffers.create_with_offset(buffer, size).handle
    }

    unsafe fn create_dumb(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_mode_create_dumb>() };
        create_dumb(self.kernel, args, |args| {
            if args.bpp != 32 {
                return Err(EINVAL);
            }
            if self.host.resource_blob && !self.host.virgl_3d {
                unimplemented!("guest blob dumb buffers");
            }
            let pitch = args.width.wrapping_mul(4);
            let size = (pitch.wrapping_mul(args.height) as u64).next_multiple_of(PAGE_SIZE);
            let res_handle = self.next_resource_id.fetch_add(1, Relaxed);
            args.size = size;
            args.handle = self.new_buffer(file, size, res_handle, 0);
            args.pitch = pitch;
            Ok(())
        })
    }

    unsafe fn destroy_dumb(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &*arg.cast::<drm_mode_destroy_dumb>() };
        match self.buffers.file(file).close(args.handle) {
            true => 0,
            false => -(EINVAL as i64),
        }
    }

    unsafe fn get_caps(&self, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let args = unsafe { &*arg.cast::<drm_virtgpu_get_caps>() };
        if !self.has_capsets() {
            return -(ENOSYS as i64);
        }
        if args.size == 0 {
            return -(EINVAL as i64);
        }
        let Some(data) = self.capset(args.cap_set_id, args.cap_set_ver) else {
            return -(EINVAL as i64);
        };
        let size = (args.size as usize).min(data.len());
        match user.write(args.addr, &data[..size]) {
            Ok(()) => 0,
            Err(_) => -(EFAULT as i64),
        }
    }

    fn create_context(&self, file: u64, state: &mut FileState) {
        if state.context_created {
            return;
        }
        state.context_created = true;
        match (state.capset_id as u32, &self.native) {
            (VIRTGPU_DRM_CAPSET_DRM, Some(native)) => {
                native.create_context(file);
                state.native = true;
            }
            (_, Some(_)) => unimplemented!("virgl contexts on native context hosts"),
            (_, None) => {}
        }
    }

    unsafe fn resource_create(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_virtgpu_resource_create>() };
        if self.host.virgl_3d {
            let mut files = self.files.lock();
            self.create_context(file, files.entry(file).or_default());
        } else if args.depth > 1
            || args.nr_samples > 1
            || args.last_level > 1
            || args.target != 2
            || args.array_size > 1
        {
            return -(EINVAL as i64);
        }
        let size = match args.size {
            0 => PAGE_SIZE,
            size => (size as u64).next_multiple_of(PAGE_SIZE),
        };
        let res_handle = self.next_resource_id.fetch_add(1, Relaxed);
        args.res_handle = res_handle;
        args.bo_handle = self.new_buffer(file, size, res_handle, 0);
        0
    }

    unsafe fn context_init(&self, file: u64, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let args = unsafe { &*arg.cast::<drm_virtgpu_context_init>() };
        if !self.host.context_init || !self.host.virgl_3d {
            return -(EINVAL as i64);
        }
        if args.num_params > MAX_CONTEXT_PARAMS {
            return -(EINVAL as i64);
        }
        let params: Vec<drm_virtgpu_context_set_param> =
            match user.read_array(args.ctx_set_params, args.num_params as usize) {
                Ok(params) => params,
                Err(_) => return -(EFAULT as i64),
            };
        let mut files = self.files.lock();
        let state = files.entry(file).or_default();
        if state.context_created {
            return -(EEXIST as i64);
        }
        for param in params {
            match param.param {
                VIRTGPU_CONTEXT_PARAM_CAPSET_ID => {
                    if param.value > MAX_CAPSET_ID
                        || self.capset_id_mask() & (1 << param.value) == 0
                        || state.capset_id != 0
                    {
                        return -(EINVAL as i64);
                    }
                    state.capset_id = param.value;
                }
                VIRTGPU_CONTEXT_PARAM_NUM_RINGS => {
                    if state.base_fence_ctx || param.value > MAX_RINGS {
                        return -(EINVAL as i64);
                    }
                    state.base_fence_ctx = true;
                    state.num_rings = param.value;
                }
                VIRTGPU_CONTEXT_PARAM_POLL_RINGS_MASK => {
                    if state.ring_idx_mask != 0 {
                        return -(EINVAL as i64);
                    }
                    state.ring_idx_mask = param.value;
                }
                VIRTGPU_CONTEXT_PARAM_DEBUG_NAME => {
                    if state.explicit_debug_name {
                        return -(EINVAL as i64);
                    }
                    for index in 0..DEBUG_NAME_MAX_LEN - 1 {
                        let mut byte = [0u8];
                        if user.read(param.value + index, &mut byte).is_err() {
                            return -(EFAULT as i64);
                        }
                        if byte[0] == 0 {
                            break;
                        }
                    }
                    state.explicit_debug_name = true;
                }
                _ => return -(EINVAL as i64),
            }
        }
        if state.ring_idx_mask != 0 {
            let valid = match state.num_rings {
                64 => u64::MAX,
                rings => (1 << rings) - 1,
            };
            if !valid & state.ring_idx_mask != 0 {
                return -(EINVAL as i64);
            }
        }
        self.create_context(file, state);
        0
    }

    unsafe fn resource_create_blob(
        &self,
        file: u64,
        arg: *mut u8,
        user: &dyn UserMemory,
        prime: &mut dyn Prime,
    ) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_virtgpu_resource_create_blob>() };
        if !self.host.resource_blob
            || args.blob_flags & !VIRTGPU_BLOB_FLAG_USE_MASK != 0
            || (args.blob_flags & VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE != 0
                && !self.host.cross_device)
        {
            return -(EINVAL as i64);
        }
        let (guest_blob, host3d_blob) = match args.blob_mem {
            VIRTGPU_BLOB_MEM_GUEST => (true, false),
            VIRTGPU_BLOB_MEM_HOST3D_GUEST => (true, true),
            VIRTGPU_BLOB_MEM_HOST3D => (false, true),
            _ => return -(EINVAL as i64),
        };
        if host3d_blob {
            if !self.host.virgl_3d || args.cmd_size % 4 != 0 {
                return -(EINVAL as i64);
            }
        } else if args.blob_id != 0 || args.cmd_size != 0 {
            return -(EINVAL as i64);
        }
        if let Some(alignment) = self.host.blob_alignment
            && self.kernel >= KernelVersion::V7_2
            && !args.size.is_multiple_of(alignment as u64)
        {
            return -(EINVAL as i64);
        }
        if guest_blob {
            unimplemented!("guest blob resources");
        }
        let mut files = self.files.lock();
        let state = files.entry(file).or_default();
        if self.host.virgl_3d {
            self.create_context(file, state);
        }
        if args.cmd_size != 0 {
            let mut cmd = vec![0u8; args.cmd_size as usize];
            if user.read(args.cmd, &mut cmd).is_err() {
                return -(EFAULT as i64);
            }
            self.run_pending(file, state, prime);
            if let Some(native) = self.native.as_ref().filter(|_| state.native) {
                native.submit(file, prime, &cmd);
            }
        }
        let size = args.size.next_multiple_of(PAGE_SIZE);
        let res_handle = self.next_resource_id.fetch_add(1, Relaxed);
        let handle = self.new_buffer(file, size, res_handle, args.blob_mem);
        let mmap_offset = self.buffers.file(file).offset(handle).unwrap();
        if let Some(native) = self.native.as_ref().filter(|_| state.native) {
            let blob = Blob {
                res_id: res_handle,
                blob_id: args.blob_id,
                size,
                blob_flags: args.blob_flags,
                mmap_offset,
            };
            native.get_blob(file, prime, blob);
        }
        let defer_mapping = self.kernel >= KernelVersion::V7_2
            && args.blob_hints & DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING != 0;
        let map_now = !defer_mapping && args.blob_flags & VIRTGPU_BLOB_FLAG_USE_MAPPABLE != 0;
        if map_now && !self.host.host_visible {
            self.buffers.file(file).close(handle);
            return -(EINVAL as i64);
        }
        args.res_handle = res_handle;
        args.bo_handle = handle;
        0
    }

    unsafe fn map(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_virtgpu_map>() };
        match self.buffers.file(file).offset(args.handle) {
            Some(offset) => {
                args.offset = offset;
                0
            }
            None => -(ENOENT as i64),
        }
    }

    unsafe fn resource_info(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_virtgpu_resource_info>() };
        let buffers = self.buffers.file(file);
        match buffers.with(args.bo_handle, |buffer| *buffer) {
            Some(buffer) => {
                args.size = buffers.size(args.bo_handle).unwrap() as u32;
                args.res_handle = buffer.res_handle;
                if buffer.blob_mem != 0 {
                    args.blob_mem = buffer.blob_mem;
                }
                0
            }
            None => -(ENOENT as i64),
        }
    }

    unsafe fn wait(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &*arg.cast::<drm_virtgpu_3d_wait>() };
        if !self.buffers.file(file).contains(args.handle) {
            return -(ENOENT as i64);
        }
        0
    }

    fn run_pending(&self, file: u64, state: &mut FileState, prime: &mut dyn Prime) {
        let pending = mem::take(&mut state.pending);
        if let Some(native) = self.native.as_ref().filter(|_| state.native) {
            for command in pending {
                native.submit(file, prime, &command);
            }
        }
    }

    unsafe fn execbuffer(
        &self,
        file: u64,
        arg: *mut u8,
        user: &dyn UserMemory,
        prime: &mut dyn Prime,
    ) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_virtgpu_execbuffer>() };
        if !self.host.virgl_3d {
            return -(ENOSYS as i64);
        }
        if args.flags & !VIRTGPU_EXECBUF_FLAGS != 0 {
            return -(EINVAL as i64);
        }
        let mut files = self.files.lock();
        let state = files.entry(file).or_default();
        if args.flags & VIRTGPU_EXECBUF_RING_IDX != 0 && args.ring_idx as u64 >= state.num_rings {
            return -(EINVAL as i64);
        }
        self.create_context(file, state);
        if args.num_bo_handles != 0 {
            let handles: Vec<u32> =
                match user.read_array(args.bo_handles, args.num_bo_handles as usize) {
                    Ok(handles) => handles,
                    Err(_) => return -(EFAULT as i64),
                };
            let buffers = self.buffers.file(file);
            if handles.iter().any(|&handle| !buffers.contains(handle)) {
                return -(ENOENT as i64);
            }
        }
        let mut command = vec![0u8; args.size as usize];
        if user.read(args.command, &mut command).is_err() {
            return -(EFAULT as i64);
        }
        let mut post_deps = Vec::new();
        for i in 0..args.num_out_syncobjs as u64 {
            let desc = match Self::syncobj_desc(args, args.out_syncobjs, i, user) {
                Ok(desc) => desc,
                Err(errno) => return -(errno as i64),
            };
            if desc.flags != 0 {
                return -(EINVAL as i64);
            }
            match self.syncobjs.object(file, desc.handle) {
                Some(object) => post_deps.push(PostDep {
                    object,
                    point: desc.point,
                }),
                None => return -(EINVAL as i64),
            }
        }
        let mut resets = Vec::new();
        for i in 0..args.num_in_syncobjs as u64 {
            let desc = match Self::syncobj_desc(args, args.in_syncobjs, i, user) {
                Ok(desc) => desc,
                Err(errno) => return -(errno as i64),
            };
            if desc.flags & !VIRTGPU_EXECBUF_SYNCOBJ_FLAGS != 0 {
                return -(EINVAL as i64);
            }
            if let Err(errno) = self.syncobjs.find_fence(file, desc.handle, desc.point, 0) {
                return -(errno as i64);
            }
            if desc.flags & VIRTGPU_EXECBUF_SYNCOBJ_RESET != 0 {
                match self.syncobjs.object(file, desc.handle) {
                    Some(object) => resets.push(object),
                    None => return -(EINVAL as i64),
                }
            }
        }
        if args.flags & VIRTGPU_EXECBUF_FENCE_FD_IN != 0 {
            unimplemented!("virtio_gpu in-fences");
        }
        if self.native.is_none() || !state.native {
            unimplemented!("virgl command streams");
        }
        state.pending.push(command);
        if !self.deferred.load(Relaxed) {
            self.run_pending(file, state, prime);
        }
        if args.flags & VIRTGPU_EXECBUF_FENCE_FD_OUT != 0 {
            match prime.sync_file() {
                Ok(fd) => args.fence_fd = fd,
                Err(errno) => return -(errno as i64),
            }
        }
        for dep in post_deps {
            match dep.point {
                0 => self.syncobjs.replace(dep.object, Some(Fence::Plain)),
                point => self.syncobjs.add_point(dep.object, point),
            }
        }
        for object in resets {
            self.syncobjs.replace(object, None);
        }
        0
    }

    fn syncobj_desc(
        args: &drm_virtgpu_execbuffer,
        array: u64,
        index: u64,
        user: &dyn UserMemory,
    ) -> Result<drm_virtgpu_execbuffer_syncobj, i32> {
        let len = (args.syncobj_stride as usize).min(size_of::<drm_virtgpu_execbuffer_syncobj>());
        let mut bytes = vec![0u8; len];
        user.read(array + index * args.syncobj_stride as u64, &mut bytes)?;
        let desc = from_bytes(&bytes);
        Ok(desc)
    }
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "virtio_gpu"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        format!("virtio_gpu:host={}:kernel={}", self.host.name, self.kernel)
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn defer_commands(&self) {
        self.deferred.store(true, Relaxed);
    }

    fn wait_fence(&self, file: u64, prime: &mut dyn Prime) {
        let mut files = self.files.lock();
        if let Some(state) = files.get_mut(&file) {
            self.run_pending(file, state, prime);
        }
    }

    fn desc(&self) -> &'static str {
        "virtio GPU"
    }

    fn driver_version(&self) -> DriverVersion {
        DriverVersion::VIRTIO_GPU
    }

    fn uevent_driver(&self) -> &'static str {
        "virtio_gpu"
    }

    fn pci(&self) -> Option<Pci> {
        Some(Pci {
            vendor: PCI_VENDOR_ID_REDHAT_QUMRANET,
            device: PCI_DEVICE_ID_VIRTIO_GPU,
        })
    }

    fn virtio_device(&self) -> Option<u32> {
        Some(VIRTIO_ID_GPU)
    }

    fn amd_gfx_level(&self) -> Option<GfxLevel> {
        self.native.as_ref()?.gfx_level()
    }

    fn context_type(&self, file: u64) -> Option<u32> {
        let files = self.files.lock();
        let state = files.get(&file)?;
        match (state.capset_id as u32, &self.native, state.native) {
            (VIRTGPU_DRM_CAPSET_DRM, Some(native), true) => Some(native.context_type()),
            _ => None,
        }
    }

    fn resource_handle(&self, file: u64, res_id: u32) -> Option<u32> {
        self.buffers
            .file(file)
            .find(|buffer| buffer.res_handle == res_id)
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        ioctls::VIRTGPU
    }

    fn syncobjs(&self) -> Option<&Syncobjs> {
        Some(&self.syncobjs)
    }

    fn master(&self) -> &Master {
        &self.master
    }

    fn check_feature(&self, feature: DriverFeature) -> bool {
        match feature {
            DriverFeature::Modeset => NUM_SCANOUTS != 0,
            DriverFeature::SyncobjTimeline => true,
        }
    }

    fn fb_modifiers_not_supported(&self) -> bool {
        false
    }

    unsafe fn ioctl(
        &self,
        file: u64,
        func: Func,
        arg: *mut u8,
        user: &dyn UserMemory,
        prime: &mut dyn Prime,
    ) -> Option<i64> {
        let ret = unsafe {
            match func {
                Func::VirtioGpuGetparamIoctl => self.getparam(arg, user),
                Func::VirtioGpuGetCapsIoctl => self.get_caps(arg, user),
                Func::VirtioGpuResourceCreateIoctl => self.resource_create(file, arg),
                Func::VirtioGpuContextInitIoctl => self.context_init(file, arg, user),
                Func::VirtioGpuResourceCreateBlobIoctl => {
                    self.resource_create_blob(file, arg, user, prime)
                }
                Func::VirtioGpuMapIoctl => self.map(file, arg),
                Func::VirtioGpuResourceInfoIoctl => self.resource_info(file, arg),
                Func::VirtioGpuWaitIoctl => self.wait(file, arg),
                Func::VirtioGpuExecbufferIoctl => self.execbuffer(file, arg, user, prime),
                Func::DrmModeCreateDumbIoctl => self.create_dumb(file, arg),
                Func::DrmModeDestroyDumbIoctl => self.destroy_dumb(file, arg),
                _ => return None,
            }
        };
        Some(ret)
    }

    fn export(&self, file: u64, handle: u32) -> Result<Export, i32> {
        self.buffers.file(file).export(handle, |_| Ok(()))
    }

    fn import(&self, file: u64, object: u32) -> Option<u32> {
        self.buffers.file(file).import(object)
    }

    fn import_foreign(&self, _file: u64, _export: &Export) -> NewHandle {
        unimplemented!("importing foreign dma-bufs into virtio_gpu");
    }

    fn close_buffer(&self, file: u64, handle: u32) -> bool {
        self.buffers.file(file).close(handle)
    }

    fn object(&self, file: u64, handle: u32) -> Option<u32> {
        self.buffers.file(file).object(handle)
    }

    fn object_has_handles(&self, object: u32) -> bool {
        self.buffers.has_handles(object)
    }

    fn release_export(&self, object: u32) {
        self.buffers.release_export(object);
    }

    fn fail_host_command(&self, fault: Fault) {
        match &self.native {
            Some(native) => native.fail_command(fault),
            None => unimplemented!("{} has no native context host", self.id()),
        }
    }

    fn release_file(&self, file: u64) {
        self.buffers.release_file(file);
        let state = self.files.lock().remove(&file);
        if let Some(native) = &self.native
            && state.is_some_and(|state| state.native)
        {
            native.destroy_context(file);
        }
        self.syncobjs.release_file(file);
    }
}
