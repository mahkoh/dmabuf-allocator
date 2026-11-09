use allocator_trace::Call;
use allocator_trace::Traced;
use allocator_trace::vectors;
use allocator_trace::vectors::Flag;
use allocator_trace::vectors::Flags;
use fake_kernel::Node;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_INVALID;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::fd::AsFd;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::OwnedFd;
use std::ptr::NonNull;
use std::sync::OnceLock;
use vulkan_testgen::cases;
use vulkan_testgen::drm;
use vulkan_testgen::gbm::GBM_BO_USE_LINEAR;
use vulkan_testgen::gbm::GBM_BO_USE_SCANOUT;
use vulkan_testgen::generate::Layout;
use vulkan_testgen::generate::PlaneLayout;
use vulkan_testgen::sizes::Size;

const GBM_LIBRARY: &str = env!("GBM_LIBRARY");
const NOUVEAU_DRIVER: &str = "nouveau";

#[repr(C)]
struct RawDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
struct RawBo {
    _opaque: [u8; 0],
}

type CreateDevice = unsafe extern "C" fn(c_int) -> *mut RawDevice;
type DeviceDestroy = unsafe extern "C" fn(*mut RawDevice);
type DeviceGetFormatModifierPlaneCount = unsafe extern "C" fn(*mut RawDevice, u32, u64) -> c_int;
type BoCreate = unsafe extern "C" fn(*mut RawDevice, u32, u32, u32, u32) -> *mut RawBo;
type BoCreateWithModifiers2 =
    unsafe extern "C" fn(*mut RawDevice, u32, u32, u32, *const u64, c_uint, u32) -> *mut RawBo;
type BoDestroy = unsafe extern "C" fn(*mut RawBo);
type BoGetFd = unsafe extern "C" fn(*mut RawBo) -> c_int;
type BoGetModifier = unsafe extern "C" fn(*mut RawBo) -> u64;
type BoGetPlaneCount = unsafe extern "C" fn(*mut RawBo) -> c_int;
type BoGetOffset = unsafe extern "C" fn(*mut RawBo, c_int) -> u32;
type BoGetStrideForPlane = unsafe extern "C" fn(*mut RawBo, c_int) -> u32;

struct Library {
    create_device: CreateDevice,
    device_destroy: DeviceDestroy,
    device_get_format_modifier_plane_count: DeviceGetFormatModifierPlaneCount,
    bo_create: BoCreate,
    bo_create_with_modifiers2: BoCreateWithModifiers2,
    bo_destroy: BoDestroy,
    bo_get_fd: BoGetFd,
    bo_get_modifier: BoGetModifier,
    bo_get_plane_count: BoGetPlaneCount,
    bo_get_offset: BoGetOffset,
    bo_get_stride_for_plane: BoGetStrideForPlane,
}

fn symbol(library: *mut c_void, name: &CStr) -> *mut c_void {
    let symbol = unsafe { libc::dlsym(library, name.as_ptr()) };
    assert!(!symbol.is_null(), "{GBM_LIBRARY} has no {name:?}");
    symbol
}

macro_rules! function {
    ($library:expr, $name:literal, $type:ty) => {
        unsafe { transmute::<*mut c_void, $type>(symbol($library, $name)) }
    };
}

fn load_library() -> Library {
    let path = CString::new(GBM_LIBRARY).expect("the GBM library path contains a NUL byte");
    let library = unsafe { libc::dlopen(path.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL) };
    assert!(!library.is_null(), "cannot load {GBM_LIBRARY}");
    Library {
        create_device: function!(library, c"gbm_create_device", CreateDevice),
        device_destroy: function!(library, c"gbm_device_destroy", DeviceDestroy),
        device_get_format_modifier_plane_count: function!(
            library,
            c"gbm_device_get_format_modifier_plane_count",
            DeviceGetFormatModifierPlaneCount
        ),
        bo_create: function!(library, c"gbm_bo_create", BoCreate),
        bo_create_with_modifiers2: function!(
            library,
            c"gbm_bo_create_with_modifiers2",
            BoCreateWithModifiers2
        ),
        bo_destroy: function!(library, c"gbm_bo_destroy", BoDestroy),
        bo_get_fd: function!(library, c"gbm_bo_get_fd", BoGetFd),
        bo_get_modifier: function!(library, c"gbm_bo_get_modifier", BoGetModifier),
        bo_get_plane_count: function!(library, c"gbm_bo_get_plane_count", BoGetPlaneCount),
        bo_get_offset: function!(library, c"gbm_bo_get_offset", BoGetOffset),
        bo_get_stride_for_plane: function!(
            library,
            c"gbm_bo_get_stride_for_plane",
            BoGetStrideForPlane
        ),
    }
}

fn library() -> &'static Library {
    static LIBRARY: OnceLock<Library> = OnceLock::new();
    LIBRARY.get_or_init(load_library)
}

pub fn load() {
    library();
}

struct Bo<'a> {
    bo: NonNull<RawBo>,
    _device: PhantomData<&'a GbmDevice>,
}

impl Bo<'_> {
    fn export(&self) -> OwnedFd {
        let fd = unsafe { (library().bo_get_fd)(self.bo.as_ptr()) };
        assert!(fd >= 0, "gbm_bo_get_fd failed");
        unsafe { OwnedFd::from_raw_fd(fd) }
    }

    fn modifier(&self) -> u64 {
        unsafe { (library().bo_get_modifier)(self.bo.as_ptr()) }
    }

    fn planes(&self) -> Vec<vectors::Plane> {
        let library = library();
        let bo = self.bo.as_ptr();
        let count = unsafe { (library.bo_get_plane_count)(bo) };
        (0..count)
            .map(|plane| vectors::Plane {
                offset: unsafe { (library.bo_get_offset)(bo, plane) } as u64,
                stride: unsafe { (library.bo_get_stride_for_plane)(bo, plane) } as u64,
            })
            .collect()
    }
}

impl Drop for Bo<'_> {
    fn drop(&mut self) {
        unsafe {
            (library().bo_destroy)(self.bo.as_ptr());
        }
    }
}

#[derive(Copy, Clone)]
pub struct Implicit {
    pub scanout: bool,
    pub linear: bool,
}

#[derive(Copy, Clone)]
enum Request {
    Modifier { modifier: u64, scanout: bool },
    Pair { modifiers: [u64; 2], scanout: bool },
    Implicit(Implicit),
}

struct Recording {
    modifier: u64,
    planes: Vec<vectors::Plane>,
    calls: Vec<Call>,
}

struct Measured {
    planes: Vec<vectors::Plane>,
    total: u64,
}

pub struct GbmDevice {
    device: NonNull<RawDevice>,
    _fd: OwnedFd,
    description: vectors::Device,
}

impl Drop for GbmDevice {
    fn drop(&mut self) {
        unsafe {
            (library().device_destroy)(self.device.as_ptr());
        }
    }
}

fn created_size(call: &Call) -> Option<u64> {
    match *call {
        Call::I915GemCreate { size }
        | Call::I915GemCreateExt { size, .. }
        | Call::XeGemCreate { size, .. }
        | Call::NouveauGemNew { size, .. } => Some(size),
        _ => None,
    }
}

fn scanout_usage(scanout: bool) -> u32 {
    match scanout {
        true => GBM_BO_USE_SCANOUT,
        false => 0,
    }
}

impl GbmDevice {
    pub fn open(node: u32, kind: Node) -> Self {
        let fd = drm::open_node(node, kind);
        let description = cases::device(fd.as_fd(), node, kind);
        let index = node as usize;
        let visible = match description.driver.as_str() {
            NOUVEAU_DRIVER => None,
            _ => Some(index..index + 1),
        };
        drm::set_visible(visible);
        let device = unsafe { (library().create_device)(fd.as_raw_fd()) };
        drm::set_visible(None);
        let device = NonNull::new(device)
            .unwrap_or_else(|| panic!("gbm_create_device failed on node {node}"));
        Self {
            device,
            _fd: fd,
            description,
        }
    }

    pub fn description(&self) -> &vectors::Device {
        &self.description
    }

    pub fn modifier_supported(&self, fourcc: u32, modifier: u64) -> bool {
        let count = unsafe {
            (library().device_get_format_modifier_plane_count)(
                self.device.as_ptr(),
                fourcc,
                modifier,
            )
        };
        count > 0
    }

    fn create(&self, fourcc: u32, size: Size, request: Request) -> Option<Bo<'_>> {
        let library = library();
        let device = self.device.as_ptr();
        let bo = match request {
            Request::Modifier { modifier, scanout } => unsafe {
                (library.bo_create_with_modifiers2)(
                    device,
                    size.width,
                    size.height,
                    fourcc,
                    &modifier,
                    1,
                    scanout_usage(scanout),
                )
            },
            Request::Pair { modifiers, scanout } => unsafe {
                (library.bo_create_with_modifiers2)(
                    device,
                    size.width,
                    size.height,
                    fourcc,
                    modifiers.as_ptr(),
                    modifiers.len() as c_uint,
                    scanout_usage(scanout),
                )
            },
            Request::Implicit(implicit) => {
                let mut usage = scanout_usage(implicit.scanout);
                if implicit.linear {
                    usage |= GBM_BO_USE_LINEAR;
                }
                unsafe { (library.bo_create)(device, size.width, size.height, fourcc, usage) }
            }
        };
        Some(Bo {
            bo: NonNull::new(bo)?,
            _device: PhantomData,
        })
    }

    fn record_request(&self, fourcc: u32, size: Size, request: Request) -> Option<Recording> {
        drm::start_recording();
        let Some(bo) = self.create(fourcc, size, request) else {
            drm::stop_recording();
            return None;
        };
        let fd = bo.export();
        let traced: Vec<Traced> = drm::stop_recording();
        drop(fd);
        let planes = bo.planes();
        let modifier = bo.modifier();
        drop(bo);
        Some(Recording {
            modifier,
            planes,
            calls: cases::exported_calls(traced),
        })
    }

    pub fn record(
        &self,
        fourcc: u32,
        modifier: u64,
        size: Size,
        scanout: bool,
        flags: Flags,
    ) -> Option<vectors::Case> {
        let request = Request::Modifier { modifier, scanout };
        let recording = self.record_request(fourcc, size, request)?;
        Some(vectors::Case {
            fourcc,
            modifier,
            rejected: None,
            width: size.width,
            height: size.height,
            scanout,
            flags,
            layout: recording.planes,
            calls: recording.calls,
        })
    }

    pub fn record_pair(
        &self,
        fourcc: u32,
        modifiers: [u64; 2],
        size: Size,
        scanout: bool,
        flags: Flags,
    ) -> Option<vectors::Case> {
        let request = Request::Pair { modifiers, scanout };
        let recording = self.record_request(fourcc, size, request)?;
        let rejected = match modifiers {
            [chosen, rejected] | [rejected, chosen] if chosen == recording.modifier => rejected,
            _ => panic!(
                "GBM chose {:#x} from {modifiers:#x?} on {}",
                recording.modifier, self.description.fake
            ),
        };
        Some(vectors::Case {
            fourcc,
            modifier: recording.modifier,
            rejected: Some(rejected),
            width: size.width,
            height: size.height,
            scanout,
            flags,
            layout: recording.planes,
            calls: recording.calls,
        })
    }

    pub fn record_implicit(
        &self,
        fourcc: u32,
        size: Size,
        implicit: Implicit,
    ) -> Option<vectors::Case> {
        let recording = self.record_request(fourcc, size, Request::Implicit(implicit))?;
        let flags = match implicit.linear {
            true => cases::flags(&[Flag::Linear]),
            false => Flags::default(),
        };
        Some(vectors::Case {
            fourcc,
            modifier: DRM_FORMAT_MOD_INVALID,
            rejected: None,
            width: size.width,
            height: size.height,
            scanout: implicit.scanout,
            flags,
            layout: recording.planes,
            calls: recording.calls,
        })
    }

    fn measure_export(&self, fourcc: u32, size: Size, request: Request) -> Option<Measured> {
        let bo = self.create(fourcc, size, request)?;
        let mut file = File::from(bo.export());
        let end = file.seek(SeekFrom::End(0));
        let total = end.unwrap_or_else(|error| panic!("cannot measure the dma-buf: {error}"));
        drop(file);
        Some(Measured {
            planes: bo.planes(),
            total,
        })
    }

    fn measure_creation(&self, fourcc: u32, size: Size, request: Request) -> Option<Measured> {
        let recording = self.record_request(fourcc, size, request)?;
        let total = recording
            .calls
            .iter()
            .find_map(created_size)
            .expect("no buffer creation recorded");
        Some(Measured {
            planes: recording.planes,
            total,
        })
    }

    pub fn layout(
        &self,
        fourcc: u32,
        modifier: u64,
        size: Size,
        scanout: bool,
        exact_export_size: bool,
    ) -> Option<Layout> {
        let request = Request::Modifier { modifier, scanout };
        let Measured { planes, total } = match exact_export_size {
            true => self.measure_export(fourcc, size, request)?,
            false => self.measure_creation(fourcc, size, request)?,
        };
        let layout = planes
            .iter()
            .map(|plane| {
                let end = planes
                    .iter()
                    .map(|other| other.offset)
                    .filter(|&other| other > plane.offset)
                    .min()
                    .unwrap_or(total);
                PlaneLayout {
                    offset: plane.offset,
                    stride: plane.stride,
                    size: end - plane.offset,
                }
            })
            .collect();
        Some(layout)
    }
}
