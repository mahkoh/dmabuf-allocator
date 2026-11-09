mod gbm_backend_abi;

use crate::gbm_backend_abi::GBM_BACKEND_ABI_VERSION;
use crate::gbm_backend_abi::GBM_GET_BACKEND_PROC_NAME;
use crate::gbm_backend_abi::GBM_GET_BACKEND_PROC_PTR;
use crate::gbm_backend_abi::gbm_backend;
use crate::gbm_backend_abi::gbm_bo;
use crate::gbm_backend_abi::gbm_core;
use crate::gbm_backend_abi::gbm_core_v0;
use crate::gbm_backend_abi::gbm_device;
use crate::gbm_backend_abi::gbm_device_v0;
use allocator_trace::vectors;
use clap::CommandFactory;
use clap::Parser;
use clap::error::ErrorKind;
use fake_kernel::Kernel;
use fake_kernel::KernelVersion;
use fake_kernel::Node;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D;
use fake_supervisor::client;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsString;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::mem::transmute;
use std::ops::RangeInclusive;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::OwnedFd;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use std::ptr::NonNull;
use vulkan_testgen::cases::exported_calls;
use vulkan_testgen::drm;
use vulkan_testgen::formats::ALL_FORMATS;
use vulkan_testgen::formats::fourcc_code;
use vulkan_testgen::formats::largest_block;
use vulkan_testgen::gbm::GBM_BO_FORMAT_ARGB8888;
use vulkan_testgen::gbm::GBM_BO_FORMAT_XRGB8888;
use vulkan_testgen::gbm::GBM_BO_USE_RENDERING;
use vulkan_testgen::gbm::GBM_BO_USE_SCANOUT;
use vulkan_testgen::gbm::GBM_FORMAT_ARGB8888;
use vulkan_testgen::gbm::GBM_FORMAT_XRGB8888;
use vulkan_testgen::generate::FILE_SUFFIX;
use vulkan_testgen::generate::write_zst;
use vulkan_testgen::sizes::BLOCK_LINEAR_GOB_HEIGHT;
use vulkan_testgen::sizes::MAX_EXTENT;
use vulkan_testgen::sizes::Size;

#[cfg(test)]
mod tests;

const DRIVER: &str = "nvidia-drm";
const RANK_SUFFIX: &str = "_rank";

const NV_MMU_PTE_KIND_GENERIC_16BX2: u32 = 0xfe;
const NV_MMU_PTE_KIND_C32_2CRA: u32 = 0xdb;
const NV_MMU_PTE_KIND_GENERIC_MEMORY: u32 = 0x6;
const NV_MMU_PTE_KIND_GENERIC_MEMORY_COMPRESSIBLE: u32 = 0x8;

const KIND_GENERATION_FERMI_TO_VOLTA: u32 = 0;
const KIND_GENERATION_TURING: u32 = 2;
const SECTOR_LAYOUT_TEGRA_K1: u32 = 0;
const SECTOR_LAYOUT_DESKTOP: u32 = 1;
const COMPRESSION_NONE: u64 = 0;
const COMPRESSION_ROP_3D_LAYOUT_1: u64 = 1;

const BLOCK_HEIGHTS_LOG2: RangeInclusive<u64> = 0..=5;
const EDGE_WIDTH: u32 = 256;
const TALL_ROWS: u32 = 1025;
const ODD_MULTIPLE: u32 = 17;
const COMMON_SIZE: Size = Size {
    width: 1920,
    height: 1080,
};
const RANK_SIZES: [Size; 2] = [
    Size {
        width: 256,
        height: 256,
    },
    COMMON_SIZE,
];

struct DeviceConfig {
    pci_id: u16,
    kind: u32,
    compressed_kind: u32,
    generation: u32,
    sector_layout: u32,
}

const GEFORCE_GTX_1080_TI: u16 = 0x1b06;
const GEFORCE_RTX_4090: u16 = 0x2684;
const TITAN_X_PASCAL: u16 = 0x1b00;

const DEVICES: [DeviceConfig; 3] = [
    DeviceConfig {
        pci_id: GEFORCE_GTX_1080_TI,
        kind: NV_MMU_PTE_KIND_GENERIC_16BX2,
        compressed_kind: NV_MMU_PTE_KIND_C32_2CRA,
        generation: KIND_GENERATION_FERMI_TO_VOLTA,
        sector_layout: SECTOR_LAYOUT_DESKTOP,
    },
    DeviceConfig {
        pci_id: GEFORCE_RTX_4090,
        kind: NV_MMU_PTE_KIND_GENERIC_MEMORY,
        compressed_kind: NV_MMU_PTE_KIND_GENERIC_MEMORY_COMPRESSIBLE,
        generation: KIND_GENERATION_TURING,
        sector_layout: SECTOR_LAYOUT_DESKTOP,
    },
    DeviceConfig {
        pci_id: TITAN_X_PASCAL,
        kind: NV_MMU_PTE_KIND_GENERIC_16BX2,
        compressed_kind: NV_MMU_PTE_KIND_C32_2CRA,
        generation: KIND_GENERATION_FERMI_TO_VOLTA,
        sector_layout: SECTOR_LAYOUT_TEGRA_K1,
    },
];

impl DeviceConfig {
    fn kernel_id(&self) -> String {
        let kernel = fake_kernel::nvidia_drm::kernel::Device::new(
            self.pci_id,
            self.kind,
            self.generation,
            self.sector_layout,
            KernelVersion::NEWEST,
        );
        kernel.id()
    }

    fn block_linear_modifier(&self, compressed: bool, height_log2: u64) -> u64 {
        let (compression, kind) = match compressed {
            true => (COMPRESSION_ROP_3D_LAYOUT_1, self.compressed_kind),
            false => (COMPRESSION_NONE, self.kind),
        };
        DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(
            compression,
            self.sector_layout as u64,
            self.generation as u64,
            kind as u64,
            height_log2,
        )
    }
}

struct Format {
    fourcc: &'static str,
    compressible: bool,
}

const fn plain(fourcc: &'static str) -> Format {
    Format {
        fourcc,
        compressible: false,
    }
}

const fn compressible(fourcc: &'static str) -> Format {
    Format {
        fourcc,
        compressible: true,
    }
}

const FORMATS: [Format; 82] = [
    plain("R8  "),
    plain("GR88"),
    plain("BG16"),
    plain("RG16"),
    plain("R16 "),
    plain("GR32"),
    compressible("RG24"),
    compressible("BG24"),
    compressible("XR24"),
    compressible("AR24"),
    compressible("XB24"),
    compressible("AB24"),
    compressible("XR30"),
    compressible("AB30"),
    compressible("XB30"),
    compressible("AR30"),
    compressible("BA30"),
    compressible("RA30"),
    compressible("AB10"),
    compressible("AB12"),
    compressible("AB15"),
    compressible("AB48"),
    compressible("AR12"),
    compressible("AR15"),
    compressible("BA12"),
    compressible("BA15"),
    compressible("BG48"),
    compressible("BX12"),
    compressible("BX15"),
    compressible("RA12"),
    compressible("RA15"),
    compressible("RX12"),
    compressible("RX15"),
    compressible("XB12"),
    compressible("XB15"),
    compressible("XR12"),
    compressible("XR15"),
    compressible("XB48"),
    compressible("XB4H"),
    compressible("AB4H"),
    compressible("R  H"),
    compressible("GR H"),
    compressible("BGRH"),
    plain("R  F"),
    plain("GR F"),
    plain("BGRF"),
    plain("AB8F"),
    plain("NV12"),
    plain("NV21"),
    plain("NV16"),
    plain("NV61"),
    plain("NV24"),
    plain("NV42"),
    plain("P010"),
    plain("P012"),
    plain("P210"),
    plain("P016"),
    plain("Q410"),
    plain("Q401"),
    plain("S016"),
    plain("S216"),
    plain("S416"),
    plain("UYVY"),
    plain("VYUY"),
    plain("YUYV"),
    plain("YVYU"),
    compressible("XV30"),
    plain("Y210"),
    plain("Y212"),
    plain("Y216"),
    plain("Y410"),
    plain("GREY"),
    plain("YUV9"),
    plain("YVU9"),
    plain("YU11"),
    plain("YV11"),
    plain("YU12"),
    plain("YV12"),
    plain("YU16"),
    plain("YV16"),
    plain("YU24"),
    plain("YV24"),
];

fn size_multiple(format: &Format) -> Size {
    let known = ALL_FORMATS
        .iter()
        .find(|known| known.fourcc == format.fourcc);
    let known = known.unwrap_or_else(|| panic!("formats.toml has no {:?}", format.fourcc));
    largest_block(known.planes)
}

fn sizes(multiple: Size, height_log2: u64) -> [Size; 8] {
    let block = BLOCK_LINEAR_GOB_HEIGHT << height_log2;
    let past_edge = (EDGE_WIDTH + 1).next_multiple_of(multiple.width);
    let size = |width: u32, height: u32| Size { width, height };
    [
        multiple,
        size(EDGE_WIDTH, block * multiple.height),
        size(past_edge, block * multiple.height),
        size(EDGE_WIDTH, (block + 1) * multiple.height),
        size(past_edge, TALL_ROWS * multiple.height),
        size(
            ODD_MULTIPLE * multiple.width,
            ODD_MULTIPLE * multiple.height,
        ),
        COMMON_SIZE,
        size(MAX_EXTENT, MAX_EXTENT),
    ]
}

unsafe extern "C" fn format_canonicalize(gbm_format: u32) -> u32 {
    match gbm_format {
        GBM_BO_FORMAT_XRGB8888 => GBM_FORMAT_XRGB8888,
        GBM_BO_FORMAT_ARGB8888 => GBM_FORMAT_ARGB8888,
        _ => gbm_format,
    }
}

static GBM_CORE: gbm_core = gbm_core {
    v0: gbm_core_v0 {
        core_version: GBM_BACKEND_ABI_VERSION,
        format_canonicalize: Some(format_canonicalize),
    },
};

unsafe fn load_backend(path: &CStr) -> &'static gbm_backend {
    let library = unsafe { libc::dlopen(path.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL) };
    if library.is_null() {
        let error = unsafe { libc::dlerror() };
        assert!(!error.is_null(), "cannot load {path:?}");
        let error = unsafe { CStr::from_ptr(error) };
        panic!("cannot load {path:?}: {error:?}");
    }
    let symbol = unsafe { libc::dlsym(library, GBM_GET_BACKEND_PROC_NAME.as_ptr()) };
    assert!(
        !symbol.is_null(),
        "{path:?} has no {GBM_GET_BACKEND_PROC_NAME:?}"
    );
    let get_backend = unsafe { transmute::<*mut c_void, GBM_GET_BACKEND_PROC_PTR>(symbol) };
    let backend = unsafe { get_backend(&GBM_CORE) };
    assert!(!backend.is_null(), "{path:?} returned no backend");
    unsafe { &*backend }
}

struct Device {
    device: NonNull<gbm_device>,
    _fd: OwnedFd,
}

struct Bo<'a> {
    device: &'a Device,
    bo: NonNull<gbm_bo>,
}

impl Drop for Bo<'_> {
    fn drop(&mut self) {
        let destroy = self.device.v0().bo_destroy.expect("no bo_destroy");
        unsafe {
            destroy(self.bo.as_ptr());
        }
    }
}

impl Device {
    unsafe fn create(backend: &gbm_backend, node: u32) -> Self {
        let fd = drm::open_node(node, Node::Render);
        let version = GBM_BACKEND_ABI_VERSION.min(backend.v0.backend_version);
        let create_device = backend.v0.create_device.expect("no create_device");
        let device = unsafe { create_device(fd.as_raw_fd(), version) };
        let device = NonNull::new(device)
            .unwrap_or_else(|| panic!("the backend created no device for node {node}"));
        let device = Self { device, _fd: fd };
        assert_eq!(
            device.v0().backend_version,
            version,
            "the device does not use the negotiated backend version"
        );
        device
    }

    fn v0(&self) -> &gbm_device_v0 {
        unsafe { &(*self.device.as_ptr()).v0 }
    }

    fn create_bo(&self, fourcc: u32, modifiers: &[u64], usage: u32, size: Size) -> Option<Bo<'_>> {
        let bo_create = self.v0().bo_create.expect("no bo_create");
        let bo = unsafe {
            bo_create(
                self.device.as_ptr(),
                size.width,
                size.height,
                fourcc,
                usage,
                modifiers.as_ptr(),
                modifiers.len() as c_uint,
            )
        };
        Some(Bo {
            device: self,
            bo: NonNull::new(bo)?,
        })
    }

    fn export(&self, bo: &Bo<'_>) -> OwnedFd {
        let bo_get_fd = self.v0().bo_get_fd.expect("no bo_get_fd");
        let fd = unsafe { bo_get_fd(bo.bo.as_ptr()) };
        assert!(fd >= 0, "bo_get_fd failed");
        unsafe { OwnedFd::from_raw_fd(fd) }
    }

    fn planes(&self, bo: &Bo<'_>) -> Vec<vectors::Plane> {
        let v0 = self.v0();
        let bo_get_planes = v0.bo_get_planes.expect("no bo_get_planes");
        let bo_get_offset = v0.bo_get_offset.expect("no bo_get_offset");
        let bo_get_stride = v0.bo_get_stride.expect("no bo_get_stride");
        let bo = bo.bo.as_ptr();
        let count = unsafe { bo_get_planes(bo) };
        (0..count)
            .map(|plane| vectors::Plane {
                offset: unsafe { bo_get_offset(bo, plane) } as u64,
                stride: unsafe { bo_get_stride(bo, plane) } as u64,
            })
            .collect()
    }

    fn bo_modifier(&self, bo: &Bo<'_>) -> u64 {
        let bo_get_modifier = self.v0().bo_get_modifier.expect("no bo_get_modifier");
        unsafe { bo_get_modifier(bo.bo.as_ptr()) }
    }

    fn record_offered(
        &self,
        fourcc: u32,
        modifiers: &[u64],
        size: Size,
        scanout: bool,
    ) -> Option<vectors::Case> {
        let usage = match scanout {
            true => GBM_BO_USE_RENDERING | GBM_BO_USE_SCANOUT,
            false => GBM_BO_USE_RENDERING,
        };
        client::start_recording();
        let Some(bo) = self.create_bo(fourcc, modifiers, usage, size) else {
            client::stop_recording();
            return None;
        };
        let fd = self.export(&bo);
        let traced = client::stop_recording();
        drop(fd);
        Some(vectors::Case {
            fourcc,
            modifier: self.bo_modifier(&bo),
            rejected: None,
            width: size.width,
            height: size.height,
            scanout,
            flags: vectors::Flags::default(),
            layout: self.planes(&bo),
            calls: exported_calls(traced),
        })
    }

    fn record(
        &self,
        fourcc: u32,
        modifier: u64,
        size: Size,
        scanout: bool,
    ) -> Option<vectors::Case> {
        let case = self.record_offered(fourcc, &[modifier], size, scanout)?;
        assert_eq!(case.modifier, modifier, "the buffer has another modifier");
        Some(case)
    }

    fn record_pair(
        &self,
        fourcc: u32,
        modifiers: [u64; 2],
        size: Size,
        scanout: bool,
    ) -> Option<vectors::Case> {
        let mut case = self.record_offered(fourcc, &modifiers, size, scanout)?;
        let rejected = match modifiers {
            [chosen, rejected] | [rejected, chosen] if chosen == case.modifier => rejected,
            _ => panic!(
                "the backend chose {:#x} from {modifiers:#x?}",
                case.modifier
            ),
        };
        case.rejected = Some(rejected);
        Some(case)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        let destroy = self.v0().destroy.expect("no destroy");
        unsafe {
            destroy(self.device.as_ptr());
        }
    }
}

fn device_cases(device: &Device, config: &DeviceConfig) -> Vec<vectors::Case> {
    let mut cases = Vec::new();
    for format in &FORMATS {
        let fourcc = fourcc_code(format.fourcc);
        let multiple = size_multiple(format);
        for height_log2 in BLOCK_HEIGHTS_LOG2 {
            let compressions: &[bool] = match format.compressible {
                true => &[false, true],
                false => &[false],
            };
            for &compressed in compressions {
                let modifier = config.block_linear_modifier(compressed, height_log2);
                for size in sizes(multiple, height_log2) {
                    for scanout in [false, true] {
                        cases.extend(device.record(fourcc, modifier, size, scanout));
                    }
                }
            }
        }
    }
    cases
}

fn format_modifiers(config: &DeviceConfig, format: &Format) -> Vec<u64> {
    let compressions: &[bool] = match format.compressible {
        true => &[false, true],
        false => &[false],
    };
    let mut modifiers = Vec::new();
    for height_log2 in BLOCK_HEIGHTS_LOG2 {
        for &compressed in compressions {
            modifiers.push(config.block_linear_modifier(compressed, height_log2));
        }
    }
    modifiers.sort_unstable();
    modifiers
}

fn device_rank_cases(device: &Device, config: &DeviceConfig) -> Vec<vectors::Case> {
    let mut cases = Vec::new();
    for format in &FORMATS {
        let fourcc = fourcc_code(format.fourcc);
        let modifiers = format_modifiers(config, format);
        for (index, &low) in modifiers.iter().enumerate() {
            for &high in &modifiers[index + 1..] {
                for size in RANK_SIZES {
                    for scanout in [false, true] {
                        cases.extend(device.record_pair(fourcc, [low, high], size, scanout));
                    }
                }
            }
        }
    }
    cases
}

fn rank_path(output: &Path) -> Option<PathBuf> {
    let name = output.file_name()?.to_str()?;
    let stem = name.strip_suffix(FILE_SUFFIX)?;
    Some(output.with_file_name(format!("{stem}{RANK_SUFFIX}{FILE_SUFFIX}")))
}

fn generate(library: &CStr, output: &Path, rank_output: &Path) {
    let ids: Vec<String> = DEVICES.iter().map(DeviceConfig::kernel_id).collect();
    let nodes = drm::register(ids.clone());
    let backend = unsafe { load_backend(library) };
    let mut text = String::new();
    let mut rank_text = String::new();
    let mut written = 0;
    let mut rank_written = 0;
    for ((config, id), node) in DEVICES.iter().zip(ids).zip(nodes) {
        let device = unsafe { Device::create(backend, node as u32) };
        let cases = device_cases(&device, config);
        let rank_cases = device_rank_cases(&device, config);
        let description = vectors::Device {
            driver: DRIVER.to_string(),
            fake: id,
            node: Node::Render,
        };
        vectors::write_compact_device(&mut text, &description, &cases);
        vectors::write_compact_device(&mut rank_text, &description, &rank_cases);
        written += cases.len();
        rank_written += rank_cases.len();
    }
    write_zst(output, &text, 1);
    write_zst(rank_output, &rank_text, 1);
    println!("{written} allocator cases, {rank_written} rank cases");
}

#[derive(Parser)]
struct Args {
    library: OsString,
    output: PathBuf,
}

fn invalid(message: String) -> ! {
    Args::command()
        .error(ErrorKind::ValueValidation, message)
        .exit()
}

fn main() -> ExitCode {
    let args = Args::parse();
    let Some(rank_output) = rank_path(&args.output) else {
        invalid(format!(
            "the output file name does not end in {FILE_SUFFIX}"
        ));
    };
    if !client::is_worker() {
        let program = std::env::current_exe().expect("cannot find the nvidia-testgen binary");
        let status = fake_supervisor::supervise(&program, &[args.library, args.output.into()]);
        std::process::exit(status.code().unwrap_or(1));
    }
    let Ok(library) = CString::new(args.library.as_bytes()) else {
        invalid("the library path contains a NUL byte".to_string());
    };
    generate(&library, &args.output, &rank_output);
    ExitCode::SUCCESS
}
