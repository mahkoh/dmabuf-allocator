use manual_tests::AllocatorError;
use manual_tests::Buffer;
use manual_tests::Call;
use manual_tests::Device;
use manual_tests::DeviceBuilder;
use manual_tests::DriverName;
use manual_tests::FORMATS;
use manual_tests::Fault;
use manual_tests::Format;
use manual_tests::I915Error;
use manual_tests::KernelVersion;
use manual_tests::LINEAR;
use manual_tests::Pitch;
use manual_tests::Plane;
use manual_tests::Region;
use manual_tests::Request;
use manual_tests::VdrmError;
use manual_tests::VirtioGpuError;
use manual_tests::fourcc;
use manual_tests::intel;
use manual_tests::intel::DEVICES;
use manual_tests::intel::DeviceInfo;
use manual_tests::intel::DeviceValues;
use manual_tests::intel::IntelKernel;
use manual_tests::intel::KERNEL_DEVICES;
use manual_tests::intel::XE_VERX10;
use manual_tests::request;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_BMG_CCS;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_DG2_MC_CCS;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_DG2_RC_CCS;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_LNL_CCS;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_MTL_MC_CCS;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_MTL_RC_CCS;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_X_TILED;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_Y_TILED;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_Y_TILED_CCS;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_Yf_TILED;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_Yf_TILED_CCS;
use manual_tests::uapi::drm_fourcc::fourcc_mod_code;
use manual_tests::uapi::i915::DRM_IOCTL_I915_GEM_CREATE_EXT;
use manual_tests::uapi::i915::DRM_IOCTL_I915_GEM_SET_TILING;
use manual_tests::uapi::i915::DRM_IOCTL_I915_QUERY;
use manual_tests::uapi::i915::I915_GEM_CREATE_EXT_FLAG_NEEDS_CPU_ACCESS;
use manual_tests::uapi::i915::I915_GEM_DOMAIN_CPU;
use manual_tests::uapi::i915::I915_MEMORY_CLASS_DEVICE;
use manual_tests::uapi::i915::I915_MEMORY_CLASS_SYSTEM;
use manual_tests::uapi::i915::I915_TILING_NONE;
use manual_tests::uapi::i915::I915_TILING_X;
use manual_tests::uapi::i915::I915_TILING_Y;
use manual_tests::uapi::vdrm::I915_CCMD_GEM_CREATE;
use manual_tests::uapi::vdrm::I915_CCMD_IOCTL_SIMPLE;
use manual_tests::uapi::vdrm::I915_CCMD_QUERYPARAM;
use manual_tests::uapi::virtgpu::DRM_IOCTL_VIRTGPU_EXECBUFFER;
use manual_tests::uapi::virtgpu::DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING;
use manual_tests::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE;
use manual_tests::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_MAPPABLE;
use manual_tests::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
use manual_tests::uapi::virtgpu::VIRTGPU_BLOB_MEM_HOST3D;
use manual_tests::uapi::xe::DRM_XE_GEM_CPU_CACHING_WC;
use manual_tests::uapi::xe::DRM_XE_GEM_CREATE_FLAG_NEEDS_VISIBLE_VRAM;
use manual_tests::uapi::xe::DRM_XE_GEM_CREATE_FLAG_NO_COMPRESSION;
use manual_tests::uapi::xe::DRM_XE_GEM_CREATE_FLAG_SCANOUT;

const SKL: &str = "intel:skl_gt2:xe=0:small_bar=0";
const ICL: &str = "intel:icl_gt2:xe=0:small_bar=0";
const TGL: &str = "intel:tgl_gt2:xe=0:small_bar=0";
const ADL: &str = "intel:adl_gt2:xe=0:small_bar=0";
const SG1: &str = "intel:sg1:xe=0:small_bar=0";
const DG2: &str = "intel:dg2_g10:xe=0:small_bar=0";
const DG2_SMALL_BAR: &str = "intel:dg2_g10:xe=0:small_bar=1";
const MTL: &str = "intel:mtl_u:xe=0:small_bar=0";
const LNL_I915: &str = "intel:lnl:xe=0:small_bar=0";
const ADL_XE: &str = "intel:adl_gt2:xe=1:small_bar=0";
const DG2_XE: &str = "intel:dg2_g10:xe=1:small_bar=0";
const DG2_XE_SMALL_BAR: &str = "intel:dg2_g10:xe=1:small_bar=1";
const LNL: &str = "intel:lnl:xe=1:small_bar=0";
const BMG: &str = "intel:bmg:xe=1:small_bar=0";
const VIRTIO_SKL: &str = "virtio_gpu:host=i915_skl_gt2";
const VIRTIO_TGL: &str = "virtio_gpu:host=i915_tgl_gt2";
const VIRTIO_TGL_CROSS_DEVICE: &str = "virtio_gpu:host=i915_tgl_gt2_cross_device";
const VIRTIO_TGL_64K: &str = "virtio_gpu:host=i915_tgl_gt2_64k";
const VIRTIO_DG2: &str = "virtio_gpu:host=i915_dg2_g10";
const VIRTIO_MTL: &str = "virtio_gpu:host=i915_mtl_u";

const MAX_EXTENT: u32 = 16384;
const MAX_TILED_WIDTH_ON_GEN8_AND_SKL_WITHOUT_GT4: u32 = 16382;
const SYSTEM_REGION: Region = Region {
    memory_class: I915_MEMORY_CLASS_SYSTEM,
    memory_instance: 0,
};
const DEVICE_REGION: Region = Region {
    memory_class: I915_MEMORY_CLASS_DEVICE,
    memory_instance: 0,
};
const SYSTEM_PLACEMENT: u32 = 1 << 0;
const VRAM_PLACEMENT: u32 = 1 << 1;
const MTL_PAT_INDEX_WC: u32 = 1;
const LNL_PAT_INDEX_WC: u32 = 0;
const LNL_PAT_INDEX_SCANOUT: u32 = 6;
const LNL_PAT_INDEX_COMPRESSED_SCANOUT: u32 = 11;
const GEN9_CCS_ROWS_PER_512_IMAGE_ROWS: u64 = 32;
const BLOB_FLAGS: u32 = VIRTGPU_BLOB_FLAG_USE_MAPPABLE | VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
const UNKNOWN_PCI_ID: u16 = 0xffff;
const MIB: u64 = 1024 * 1024;

#[derive(Debug, PartialEq, Eq)]
struct Allocated {
    pitches: Vec<Pitch>,
    calls: Vec<Call>,
}

#[derive(Debug, PartialEq, Eq)]
struct Native {
    modifier: u64,
    planes: Vec<Plane>,
    calls: Vec<Call>,
}

struct PciDevice {
    info: &'static DeviceInfo,
    device: Device,
}

struct DeviceModifiers {
    device: &'static str,
    modifiers: &'static [u64],
}

struct VirtioHost {
    virtio: &'static str,
    native: &'static str,
}

struct Bucket {
    width: u32,
    height: u32,
    size: u64,
}

#[derive(PartialEq, Eq)]
struct Configuration {
    ver: u32,
    verx10: u32,
    dg2: bool,
    mtl_or_arl: bool,
    skl: bool,
    gt: u32,
    has_local_mem: bool,
    has_aux_map: bool,
    has_flat_ccs: bool,
    clear_color_state_size: u64,
}

const fn pitch(offset: u64, stride: u64) -> Pitch {
    Pitch { offset, stride }
}

fn ar24(modifiers: &[u64], width: u32, height: u32) -> Request {
    request("AR24", modifiers, width, height)
}

fn chosen(device: &str, request: &Request) -> Option<u64> {
    Device::new(device).chosen(request)
}

fn stride(device: &str, request: &Request) -> u64 {
    planes(device, request)[0].stride
}

fn planes(device: &str, request: &Request) -> Vec<Plane> {
    let layout = Device::new(device).layout(request).unwrap();
    manual_tests::planes(&layout)
}

fn pitches(device: &str, request: &Request) -> Vec<Pitch> {
    planes(device, request).iter().map(Plane::pitch).collect()
}

fn layout_planes(device: &Device, request: &Request) -> Option<Vec<Plane>> {
    device
        .layout(request)
        .map(|layout| manual_tests::planes(&layout))
}

fn gem_calls(buffer: &Buffer) -> Vec<Call> {
    buffer
        .calls
        .iter()
        .filter(|call| {
            matches!(
                call,
                Call::I915GemCreate { .. }
                    | Call::I915GemCreateExt { .. }
                    | Call::I915GemSetDomain { .. }
                    | Call::I915GemSetTiling { .. }
                    | Call::XeGemCreate { .. }
            )
        })
        .cloned()
        .collect()
}

fn allocated_on(device: &Device, request: &Request) -> Allocated {
    let buffer = device.allocate(request).unwrap();
    Allocated {
        pitches: buffer.pitches(),
        calls: gem_calls(&buffer),
    }
}

fn allocated(device: &str, request: &Request) -> Allocated {
    allocated_on(&Device::new(device), request)
}

fn create_size(device: &str, request: &Request) -> u64 {
    let buffer = Device::new(device).allocate(request).unwrap();
    buffer.only(|call| match *call {
        Call::I915GemCreateExt { size, .. } | Call::XeGemCreate { size, .. } => Some(size),
        _ => None,
    })
}

fn create_ext(size: u64, flags: u32, regions: &[Region], pat_index: Option<u32>) -> Call {
    Call::I915GemCreateExt {
        size,
        flags,
        regions: Some(regions.to_vec()),
        pat_index,
    }
}

fn system_create(size: u64) -> Call {
    create_ext(size, 0, &[SYSTEM_REGION], None)
}

fn set_domain() -> Call {
    Call::I915GemSetDomain {
        read_domains: I915_GEM_DOMAIN_CPU,
        write_domain: 0,
    }
}

fn set_tiling(tiling_mode: u32, stride: u32) -> Call {
    Call::I915GemSetTiling {
        tiling_mode,
        stride,
    }
}

fn xe_create(size: u64, placement: u32, flags: u32) -> Call {
    Call::XeGemCreate {
        size,
        placement,
        flags,
        vm_id: 0,
        cpu_caching: DRM_XE_GEM_CPU_CACHING_WC,
    }
}

fn intel_device(kernel: IntelKernel) -> DeviceBuilder {
    assert!(kernel.supported(), "the device has no working driver");
    Device::with_kernel(kernel)
}

fn lnl_i915_pat_index(request: &Request) -> Option<u32> {
    let device = intel_device(intel::kernel(LNL_I915).with_i915_on_xe_platforms())
        .open()
        .unwrap();
    let buffer = device.allocate(request).unwrap();
    buffer.only(|call| match *call {
        Call::I915GemCreateExt { pat_index, .. } => Some(pat_index),
        _ => None,
    })
}

fn as_native(call: &Call) -> Call {
    match call.clone() {
        Call::VirtgpuI915GemCreate { gem_size, .. } => Call::I915GemCreate { size: gem_size },
        Call::VirtgpuI915GemCreateExt {
            gem_size,
            flags,
            regions,
            pat_index,
            ..
        } => Call::I915GemCreateExt {
            size: gem_size,
            flags,
            regions,
            pat_index,
        },
        call => call,
    }
}

fn native(device: &str, request: &Request) -> Native {
    let buffer = Device::new(device).allocate(request).unwrap();
    Native {
        modifier: buffer.modifier(),
        planes: buffer.planes(),
        calls: buffer.calls.iter().map(as_native).collect(),
    }
}

fn comparison_requests() -> Vec<Request> {
    let all = [
        I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC,
        I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC,
        I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC,
        I915_FORMAT_MOD_4_TILED,
        I915_FORMAT_MOD_Y_TILED_CCS,
        I915_FORMAT_MOD_Y_TILED,
        I915_FORMAT_MOD_X_TILED,
        LINEAR,
    ];
    vec![
        ar24(&[LINEAR], 64, 64),
        request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256),
        ar24(&all, 300, 200),
        ar24(&all, 4096, 64),
        ar24(&all, 64, 64).scanout(),
        request("NV12", &all, 128, 128),
        ar24(&[I915_FORMAT_MOD_X_TILED, LINEAR], 256, 256).implicit_interop(),
        ar24(&[I915_FORMAT_MOD_Y_TILED, LINEAR], 256, 256).implicit_interop(),
    ]
}

fn virtio_calls(buffer: &Buffer) -> Vec<Call> {
    buffer
        .calls
        .iter()
        .filter(|call| !matches!(call, Call::PrimeHandleToFd { .. }))
        .cloned()
        .collect()
}

fn virtio_create(size: u64, regions: &[Region], pat_index: Option<u32>) -> Call {
    Call::VirtgpuI915GemCreateExt {
        blob_mem: VIRTGPU_BLOB_MEM_HOST3D,
        blob_flags: BLOB_FLAGS,
        blob_hints: DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING,
        size,
        gem_size: size,
        flags: 0,
        regions: Some(regions.to_vec()),
        pat_index,
    }
}

fn pci_devices(property: impl Fn(&DeviceInfo) -> bool) -> impl Iterator<Item = PciDevice> {
    DEVICES
        .iter()
        .filter(move |info| property(info))
        .map(|info| {
            let values = DeviceValues {
                name: "pci",
                pci_id: info.pci_id,
                verx10: info.verx10,
                has_local_mem: info.has_local_mem,
            };
            let xe = info.verx10 >= XE_VERX10;
            let kernel = IntelKernel::from_values(values, KernelVersion::NEWEST, xe, false);
            let kernel = match kernel.supported() {
                true => kernel,
                false => IntelKernel::from_values(values, KernelVersion::NEWEST, false, false)
                    .with_i915_on_xe_platforms(),
            };
            PciDevice {
                info,
                device: intel_device(kernel).open().unwrap(),
            }
        })
}

fn formats(property: impl Fn(Format) -> bool) -> Vec<Format> {
    FORMATS
        .iter()
        .map(Format)
        .filter(|&format| property(format))
        .collect()
}

fn unknown_modifiers() -> Vec<u64> {
    let known = [
        I915_FORMAT_MOD_X_TILED,
        I915_FORMAT_MOD_Y_TILED,
        I915_FORMAT_MOD_Y_TILED_CCS,
        I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS,
        I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS,
        I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC,
        I915_FORMAT_MOD_4_TILED,
        I915_FORMAT_MOD_4_TILED_DG2_RC_CCS,
        I915_FORMAT_MOD_4_TILED_DG2_MC_CCS,
        I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC,
        I915_FORMAT_MOD_4_TILED_MTL_RC_CCS,
        I915_FORMAT_MOD_4_TILED_MTL_MC_CCS,
        I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC,
        I915_FORMAT_MOD_4_TILED_LNL_CCS,
        I915_FORMAT_MOD_4_TILED_BMG_CCS,
    ];
    let mut modifiers = Vec::new();
    for vendor in 0..=u64::from(u8::MAX) {
        for value in 0..64 {
            modifiers.push(fourcc_mod_code(vendor, value));
        }
    }
    for bit in 0..u64::BITS {
        modifiers.push(1 << bit);
        for &modifier in &known {
            modifiers.push(modifier ^ (1 << bit));
        }
    }
    modifiers.retain(|modifier| !known.contains(modifier));
    modifiers
}

fn configuration(info: &DeviceInfo) -> Configuration {
    Configuration {
        ver: info.ver,
        verx10: info.verx10,
        dg2: info.dg2,
        mtl_or_arl: info.mtl_or_arl,
        skl: info.skl,
        gt: info.gt,
        has_local_mem: info.has_local_mem,
        has_aux_map: info.has_aux_map,
        has_flat_ccs: info.has_flat_ccs,
        clear_color_state_size: info.clear_color_state_size,
    }
}

fn unknown_device_builder(name: &str, xe: bool) -> DeviceBuilder {
    assert!(DEVICES.iter().all(|info| info.pci_id != UNKNOWN_PCI_ID));
    let known = KERNEL_DEVICES
        .iter()
        .find(|device| device.name == name)
        .unwrap();
    let values = DeviceValues {
        name: "unknown",
        pci_id: UNKNOWN_PCI_ID,
        ..*known
    };
    let kernel = IntelKernel::from_values(values, KernelVersion::NEWEST, xe, false);
    intel_device(match xe {
        true => kernel,
        false => kernel.with_i915_on_xe_platforms(),
    })
}

fn unknown_device(name: &str, xe: bool) -> Device {
    unknown_device_builder(name, xe).open().unwrap()
}

#[test]
fn zero_extents_have_no_layout() {
    for modifiers in [
        &[LINEAR][..],
        &[I915_FORMAT_MOD_X_TILED],
        &[I915_FORMAT_MOD_Y_TILED],
    ] {
        assert_eq!(chosen(ADL, &ar24(modifiers, 0, 16)), None);
        assert_eq!(chosen(ADL, &ar24(modifiers, 16, 0)), None);
    }
}

#[test]
fn extents_up_to_16384_have_a_layout() {
    for modifiers in [&[LINEAR][..], &[I915_FORMAT_MOD_Y_TILED]] {
        assert_eq!(
            chosen(ADL, &ar24(modifiers, MAX_EXTENT, 1)),
            Some(modifiers[0])
        );
        assert_eq!(
            chosen(ADL, &ar24(modifiers, 1, MAX_EXTENT)),
            Some(modifiers[0])
        );
    }
}

#[test]
fn extents_above_16384_have_no_layout() {
    for modifiers in [&[LINEAR][..], &[I915_FORMAT_MOD_Y_TILED]] {
        assert_eq!(chosen(ADL, &ar24(modifiers, MAX_EXTENT + 1, 1)), None);
        assert_eq!(chosen(ADL, &ar24(modifiers, 1, MAX_EXTENT + 1)), None);
    }
}

#[test]
fn wide_y_ccs_pitch_is_four_tiles_on_gen9() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED_CCS], 3841, 16);
    assert_eq!(stride(SKL, &request), 15872);
}

#[test]
fn narrow_y_ccs_pitch_is_one_tile_on_gen9() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED_CCS], 3000, 16);
    assert_eq!(stride(SKL, &request), 12032);
}

#[test]
fn wide_y_ccs_pitch_is_one_tile_on_gen11() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED_CCS], 3841, 16);
    assert_eq!(stride(ICL, &request), 15488);
}

#[test]
fn linear_on_gen12_sets_no_tiling() {
    let request = request("AB24", &[LINEAR], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![
            system_create(262144),
            set_domain(),
            set_tiling(I915_TILING_NONE, 1024),
        ],
    };
    assert_eq!(allocated(ADL, &request), expected);
}

#[test]
fn x_tiled_on_gen12_sets_x_tiling() {
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![
            system_create(262144),
            set_domain(),
            set_tiling(I915_TILING_X, 1024),
        ],
    };
    assert_eq!(allocated(ADL, &request), expected);
}

#[test]
fn y_tiled_on_gen12_sets_y_tiling() {
    let request = request("AB24", &[I915_FORMAT_MOD_Y_TILED], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![
            system_create(262144),
            set_domain(),
            set_tiling(I915_TILING_Y, 1024),
        ],
    };
    assert_eq!(allocated(ADL, &request), expected);
}

#[test]
fn y_gen12_rc_ccs_has_a_ccs_plane() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024), pitch(262144, 128)],
        calls: vec![
            system_create(524288),
            set_domain(),
            set_tiling(I915_TILING_Y, 1024),
        ],
    };
    assert_eq!(allocated(ADL, &request), expected);
}

#[test]
fn y_gen12_rc_ccs_pads_pitch_and_rows() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS], 90, 96);
    let expected = Allocated {
        pitches: vec![pitch(0, 512), pitch(49152, 64)],
        calls: vec![
            system_create(65536),
            set_domain(),
            set_tiling(I915_TILING_Y, 512),
        ],
    };
    assert_eq!(allocated(ADL, &request), expected);
}

#[test]
fn media_compression_is_not_selected() {
    let devices = [
        DeviceModifiers {
            device: ADL,
            modifiers: &[I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS],
        },
        DeviceModifiers {
            device: DG2,
            modifiers: &[I915_FORMAT_MOD_4_TILED_DG2_MC_CCS],
        },
        DeviceModifiers {
            device: MTL,
            modifiers: &[I915_FORMAT_MOD_4_TILED_MTL_MC_CCS],
        },
    ];
    for DeviceModifiers { device, modifiers } in devices {
        assert_eq!(chosen(device, &ar24(modifiers, 256, 256)), None);
        assert_eq!(chosen(device, &request("NV12", modifiers, 256, 50)), None);
    }
}

#[test]
fn y_gen12_rc_ccs_cc_has_a_clear_color_plane() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024), pitch(262144, 128), pitch(266240, 64)],
        calls: vec![
            system_create(524288),
            set_domain(),
            set_tiling(I915_TILING_Y, 1024),
        ],
    };
    assert_eq!(allocated(ADL, &request), expected);
}

#[test]
fn y_ccs_has_room_for_the_clear_color_on_gen11() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED_CCS], 96, 160);
    let expected = Allocated {
        pitches: vec![pitch(0, 384), pitch(61440, 128)],
        calls: vec![
            system_create(131072),
            set_domain(),
            set_tiling(I915_TILING_Y, 384),
        ],
    };
    assert_eq!(allocated(ICL, &request), expected);
}

#[test]
fn y_ccs_has_no_clear_color_on_gen9() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED_CCS], 96, 160);
    assert_eq!(create_size(SKL, &request), 65536);
}

#[test]
fn y_ccs_plane_pitch_covers_128_blocks_per_byte() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED_CCS], 2048, 64);
    assert_eq!(pitches(ICL, &request), [pitch(0, 8192), pitch(524288, 256)]);
}

#[test]
fn y_ccs_plane_size_counts_tile_rows() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED_CCS], 96, 640);
    let expected = [Plane::at(0, 384, 245760), Plane::at(245760, 128, 8192)];
    assert_eq!(planes(ICL, &request), expected);
}

#[test]
fn yf_has_no_layout() {
    for yf in [I915_FORMAT_MOD_Yf_TILED, I915_FORMAT_MOD_Yf_TILED_CCS] {
        assert_eq!(chosen(SKL, &ar24(&[yf], 16, 16)), None);
    }
}

#[test]
fn three_byte_elements_can_be_x_tiled() {
    let request = request("RG24", &[I915_FORMAT_MOD_X_TILED], 100, 8);
    assert_eq!(planes(ADL, &request), [Plane::at(0, 512, 4096)]);
}

#[test]
fn y_tiled_two_plane_formats_have_two_pitches() {
    let request = request("NV12", &[I915_FORMAT_MOD_Y_TILED], 2, 32);
    assert_eq!(pitches(SG1, &request), [pitch(0, 128), pitch(4096, 128)]);
}

#[test]
fn tile_4_on_dg2_is_placed_in_device_and_system_memory() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![create_ext(262144, 0, &[DEVICE_REGION, SYSTEM_REGION], None)],
    };
    assert_eq!(allocated(DG2, &request), expected);
}

#[test]
fn x_tiled_on_dg2_is_placed_in_device_and_system_memory() {
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![create_ext(262144, 0, &[DEVICE_REGION, SYSTEM_REGION], None)],
    };
    assert_eq!(allocated(DG2, &request), expected);
}

#[test]
fn x_tiled_on_dg2_with_small_bar_needs_cpu_access() {
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256);
    let flags = I915_GEM_CREATE_EXT_FLAG_NEEDS_CPU_ACCESS;
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![create_ext(
            262144,
            flags,
            &[DEVICE_REGION, SYSTEM_REGION],
            None,
        )],
    };
    assert_eq!(allocated(DG2_SMALL_BAR, &request), expected);
}

#[test]
fn dg2_rc_ccs_is_in_device_memory() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED_DG2_RC_CCS], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![create_ext(524288, 0, &[DEVICE_REGION], None)],
    };
    assert_eq!(allocated(DG2, &request), expected);
    assert_eq!(allocated(DG2_SMALL_BAR, &request), expected);
}

#[test]
fn dg2_rc_ccs_cc_has_a_clear_color_plane() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024), pitch(262144, 64)],
        calls: vec![create_ext(524288, 0, &[DEVICE_REGION], None)],
    };
    assert_eq!(allocated(DG2, &request), expected);
}

#[test]
fn dg2_rc_ccs_cc_keeps_only_used_tiles_of_the_last_row() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC], 90, 96);
    let expected = Allocated {
        pitches: vec![pitch(0, 512), pitch(45056, 64)],
        calls: vec![create_ext(65536, 0, &[DEVICE_REGION], None)],
    };
    assert_eq!(allocated(DG2, &request), expected);
}

#[test]
fn x_tiled_on_mtl_sets_the_pat_index() {
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![
            create_ext(262144, 0, &[SYSTEM_REGION], Some(MTL_PAT_INDEX_WC)),
            set_domain(),
        ],
    };
    assert_eq!(allocated(MTL, &request), expected);
}

#[test]
fn mtl_rc_ccs_has_a_ccs_plane() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED_MTL_RC_CCS], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024), pitch(262144, 128)],
        calls: vec![
            create_ext(524288, 0, &[SYSTEM_REGION], Some(MTL_PAT_INDEX_WC)),
            set_domain(),
        ],
    };
    assert_eq!(allocated(MTL, &request), expected);
}

#[test]
fn mtl_rc_ccs_cc_has_a_clear_color_plane() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024), pitch(262144, 128), pitch(266240, 64)],
        calls: vec![
            create_ext(524288, 0, &[SYSTEM_REGION], Some(MTL_PAT_INDEX_WC)),
            set_domain(),
        ],
    };
    assert_eq!(allocated(MTL, &request), expected);
}

#[test]
fn lnl_ccs_is_placed_in_system_memory() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED_LNL_CCS], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![xe_create(262144, SYSTEM_PLACEMENT, 0)],
    };
    assert_eq!(allocated(LNL, &request), expected);
}

#[test]
fn bmg_ccs_is_placed_in_vram() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED_BMG_CCS], 64, 64);
    let expected = Allocated {
        pitches: vec![pitch(0, 256)],
        calls: vec![xe_create(16384, VRAM_PLACEMENT, 0)],
    };
    assert_eq!(allocated(BMG, &request), expected);
}

#[test]
fn bmg_ccs_scanout_is_64_kib_aligned() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED_BMG_CCS], 64, 64).scanout();
    let flags = DRM_XE_GEM_CREATE_FLAG_SCANOUT;
    let expected = Allocated {
        pitches: vec![pitch(0, 256)],
        calls: vec![xe_create(65536, VRAM_PLACEMENT, flags)],
    };
    assert_eq!(allocated(BMG, &request), expected);
}

#[test]
fn x_tiled_on_bmg_disables_compression() {
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256);
    let placement = SYSTEM_PLACEMENT | VRAM_PLACEMENT;
    let flags = DRM_XE_GEM_CREATE_FLAG_NO_COMPRESSION;
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![xe_create(262144, placement, flags)],
    };
    assert_eq!(allocated(BMG, &request), expected);
}

#[test]
fn x_tiled_scanout_on_bmg_is_not_64_kib_aligned() {
    let request = ar24(&[I915_FORMAT_MOD_X_TILED], 64, 64).scanout();
    let placement = SYSTEM_PLACEMENT | VRAM_PLACEMENT;
    let flags = DRM_XE_GEM_CREATE_FLAG_SCANOUT | DRM_XE_GEM_CREATE_FLAG_NO_COMPRESSION;
    let expected = Allocated {
        pitches: vec![pitch(0, 512)],
        calls: vec![xe_create(32768, placement, flags)],
    };
    assert_eq!(allocated(BMG, &request), expected);
}

#[test]
fn x_tiled_on_adl_with_xe_is_placed_in_system_memory() {
    let request = ar24(&[I915_FORMAT_MOD_X_TILED], 64, 64);
    let expected = Allocated {
        pitches: vec![pitch(0, 512)],
        calls: vec![xe_create(32768, SYSTEM_PLACEMENT, 0)],
    };
    assert_eq!(allocated(ADL_XE, &request), expected);
}

#[test]
fn x_tiled_on_dg2_with_xe_uses_the_minimum_alignment() {
    let request = ar24(&[I915_FORMAT_MOD_X_TILED], 64, 64);
    let placement = SYSTEM_PLACEMENT | VRAM_PLACEMENT;
    let expected = Allocated {
        pitches: vec![pitch(0, 512)],
        calls: vec![xe_create(65536, placement, 0)],
    };
    assert_eq!(allocated(DG2_XE, &request), expected);
}

#[test]
fn x_tiled_on_dg2_with_xe_and_small_bar_needs_visible_vram() {
    let request = ar24(&[I915_FORMAT_MOD_X_TILED], 64, 64);
    let placement = SYSTEM_PLACEMENT | VRAM_PLACEMENT;
    let flags = DRM_XE_GEM_CREATE_FLAG_NEEDS_VISIBLE_VRAM;
    let expected = Allocated {
        pitches: vec![pitch(0, 512)],
        calls: vec![xe_create(65536, placement, flags)],
    };
    assert_eq!(allocated(DG2_XE_SMALL_BAR, &request), expected);
}

#[test]
fn vram_sharing_the_system_instance_is_one_placement() {
    let device = intel_device(intel::kernel(DG2_XE).with_vram_instance(0))
        .open()
        .unwrap();
    let request = ar24(&[I915_FORMAT_MOD_X_TILED], 64, 64);
    let expected = Allocated {
        pitches: vec![pitch(0, 512)],
        calls: vec![xe_create(65536, SYSTEM_PLACEMENT, 0)],
    };
    assert_eq!(allocated_on(&device, &request), expected);
}

#[test]
fn dg2_rc_ccs_cc_with_xe_is_placed_in_vram() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024), pitch(262144, 64)],
        calls: vec![xe_create(327680, VRAM_PLACEMENT, 0)],
    };
    assert_eq!(allocated(DG2_XE, &request), expected);
}

#[test]
fn dg2_rc_ccs_cc_with_xe_and_small_bar_needs_visible_vram() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC], 256, 256);
    let flags = DRM_XE_GEM_CREATE_FLAG_NEEDS_VISIBLE_VRAM;
    let expected = Allocated {
        pitches: vec![pitch(0, 1024), pitch(262144, 64)],
        calls: vec![xe_create(327680, VRAM_PLACEMENT, flags)],
    };
    assert_eq!(allocated(DG2_XE_SMALL_BAR, &request), expected);
}

#[test]
fn dg2_rc_ccs_with_xe_and_small_bar_adds_clear_color() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED_DG2_RC_CCS], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![xe_create(327680, VRAM_PLACEMENT, 0)],
    };
    assert_eq!(allocated(DG2_XE_SMALL_BAR, &request), expected);
}

#[test]
fn linear_bo_sizes_on_i915_are_rounded_to_cache_buckets() {
    let bucket = |width, height, size| Bucket {
        width,
        height,
        size,
    };
    let buckets = [
        bucket(64, 64, 16384),
        bucket(1280, 1024, 6 * MIB),
        bucket(1920, 1080, 8 * MIB),
        bucket(1536, 1536, 10 * MIB),
        bucket(4096, 3700, 64 * MIB),
        bucket(4096, 4250, 68 * MIB),
        bucket(1024, 512, 2 * MIB),
        bucket(1024, 1792, 8 * MIB),
        bucket(1024, 2049, 10 * MIB),
        bucket(1024, 2561, 12 * MIB),
        bucket(1024, 3073, 14 * MIB),
        bucket(1024, 4097, 20 * MIB),
        bucket(1024, 5121, 24 * MIB),
        bucket(1024, 6145, 28 * MIB),
        bucket(1024, 8193, 40 * MIB),
        bucket(1024, 10241, 48 * MIB),
        bucket(1024, 12289, 56 * MIB),
        bucket(1024, 14337, 64 * MIB),
        bucket(2048, 8193, 66 * MIB),
    ];
    for bucket in buckets {
        let size = create_size(ADL, &ar24(&[LINEAR], bucket.width, bucket.height));
        assert_eq!(size, bucket.size, "{}x{}", bucket.width, bucket.height);
    }
}

#[test]
fn bo_sizes_on_xe_are_aligned_to_2_mib_from_1_mib() {
    let request = ar24(&[LINEAR], 1280, 1024);
    assert_eq!(create_size(BMG, &request), 6 * MIB);
}

#[test]
fn modifiers_are_ranked() {
    let devices = [
        DeviceModifiers {
            device: SKL,
            modifiers: &[
                LINEAR,
                I915_FORMAT_MOD_X_TILED,
                I915_FORMAT_MOD_Y_TILED,
                I915_FORMAT_MOD_Y_TILED_CCS,
            ],
        },
        DeviceModifiers {
            device: ADL,
            modifiers: &[
                LINEAR,
                I915_FORMAT_MOD_X_TILED,
                I915_FORMAT_MOD_Y_TILED,
                I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS,
                I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC,
            ],
        },
        DeviceModifiers {
            device: DG2,
            modifiers: &[
                LINEAR,
                I915_FORMAT_MOD_X_TILED,
                I915_FORMAT_MOD_4_TILED,
                I915_FORMAT_MOD_4_TILED_DG2_RC_CCS,
                I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC,
            ],
        },
        DeviceModifiers {
            device: MTL,
            modifiers: &[
                LINEAR,
                I915_FORMAT_MOD_X_TILED,
                I915_FORMAT_MOD_4_TILED,
                I915_FORMAT_MOD_4_TILED_MTL_RC_CCS,
                I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC,
            ],
        },
        DeviceModifiers {
            device: LNL,
            modifiers: &[
                LINEAR,
                I915_FORMAT_MOD_X_TILED,
                I915_FORMAT_MOD_4_TILED,
                I915_FORMAT_MOD_4_TILED_LNL_CCS,
            ],
        },
        DeviceModifiers {
            device: BMG,
            modifiers: &[
                LINEAR,
                I915_FORMAT_MOD_X_TILED,
                I915_FORMAT_MOD_4_TILED,
                I915_FORMAT_MOD_4_TILED_BMG_CCS,
            ],
        },
    ];
    for DeviceModifiers { device, modifiers } in devices {
        for pair in modifiers.windows(2) {
            assert_eq!(chosen(device, &ar24(pair, 256, 256)), Some(pair[1]));
        }
    }
}

#[test]
fn modifiers_of_other_devices_are_not_selected() {
    let devices = [
        DeviceModifiers {
            device: SKL,
            modifiers: &[
                I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS,
                I915_FORMAT_MOD_4_TILED,
                I915_FORMAT_MOD_4_TILED_DG2_RC_CCS,
            ],
        },
        DeviceModifiers {
            device: ADL,
            modifiers: &[
                I915_FORMAT_MOD_Y_TILED_CCS,
                I915_FORMAT_MOD_4_TILED,
                I915_FORMAT_MOD_4_TILED_MTL_RC_CCS,
            ],
        },
        DeviceModifiers {
            device: DG2,
            modifiers: &[
                I915_FORMAT_MOD_Y_TILED,
                I915_FORMAT_MOD_4_TILED_MTL_RC_CCS,
                I915_FORMAT_MOD_4_TILED_BMG_CCS,
            ],
        },
        DeviceModifiers {
            device: MTL,
            modifiers: &[
                I915_FORMAT_MOD_Y_TILED,
                I915_FORMAT_MOD_4_TILED_DG2_RC_CCS,
                I915_FORMAT_MOD_4_TILED_LNL_CCS,
            ],
        },
        DeviceModifiers {
            device: LNL,
            modifiers: &[
                I915_FORMAT_MOD_4_TILED_MTL_RC_CCS,
                I915_FORMAT_MOD_4_TILED_BMG_CCS,
            ],
        },
        DeviceModifiers {
            device: BMG,
            modifiers: &[
                I915_FORMAT_MOD_4_TILED_DG2_RC_CCS,
                I915_FORMAT_MOD_4_TILED_LNL_CCS,
            ],
        },
    ];
    for DeviceModifiers { device, modifiers } in devices {
        for &unsupported in modifiers {
            assert_eq!(chosen(device, &ar24(&[unsupported], 256, 256)), None);
        }
    }
}

#[test]
fn discrete_devices_without_memory_regions_fail_to_open() {
    let error = Device::builder(DG2)
        .fail(Fault::new(DRM_IOCTL_I915_QUERY))
        .open()
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::I915Open(I915Error::MemoryRegions(_))
    ));
}

#[test]
fn uncompressed_buffers_use_the_writecombining_pat_index() {
    let request = ar24(&[LINEAR], 64, 64);
    assert_eq!(lnl_i915_pat_index(&request), Some(LNL_PAT_INDEX_WC));
}

#[test]
fn uncompressed_scanout_uses_the_scanout_pat_index() {
    let request = ar24(&[LINEAR], 64, 64).scanout();
    assert_eq!(lnl_i915_pat_index(&request), Some(LNL_PAT_INDEX_SCANOUT));
}

#[test]
fn compressed_scanout_uses_the_compressed_scanout_pat_index() {
    let request = ar24(&[I915_FORMAT_MOD_4_TILED_LNL_CCS], 64, 64).scanout();
    assert_eq!(
        lnl_i915_pat_index(&request),
        Some(LNL_PAT_INDEX_COMPRESSED_SCANOUT)
    );
}

#[test]
fn implicit_interop_keeps_x_and_y_with_fences() {
    let device = Device::new(ADL);
    let x = ar24(&[I915_FORMAT_MOD_X_TILED], 64, 64).implicit_interop();
    let y = ar24(&[I915_FORMAT_MOD_Y_TILED], 64, 64).implicit_interop();
    assert_eq!(device.chosen(&x), Some(I915_FORMAT_MOD_X_TILED));
    assert_eq!(device.chosen(&y), Some(I915_FORMAT_MOD_Y_TILED));
}

#[test]
fn implicit_interop_scanout_excludes_y() {
    let modifiers = [I915_FORMAT_MOD_Y_TILED, LINEAR];
    let request = ar24(&modifiers, 64, 64).implicit_interop().scanout();
    assert_eq!(chosen(ADL, &request), Some(LINEAR));
}

#[test]
fn implicit_interop_excludes_compression() {
    let modifiers = [I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS, LINEAR];
    let request = ar24(&modifiers, 64, 64).implicit_interop();
    assert_eq!(chosen(ADL, &request), Some(LINEAR));
}

#[test]
fn implicit_interop_without_fences_excludes_tiling() {
    for tiled in [I915_FORMAT_MOD_X_TILED, I915_FORMAT_MOD_Y_TILED] {
        let request = ar24(&[tiled, LINEAR], 64, 64).implicit_interop();
        assert_eq!(chosen(DG2, &request), Some(LINEAR));
    }
}

#[test]
fn implicit_interop_sets_the_tiling() {
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256).implicit_interop();
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![
            system_create(262144),
            set_domain(),
            set_tiling(I915_TILING_X, 1024),
        ],
    };
    assert_eq!(allocated(ADL, &request), expected);
}

#[test]
fn implicit_interop_ignores_tiling_errors() {
    let device = Device::new(ADL);
    device.fail(Fault::new(DRM_IOCTL_I915_GEM_SET_TILING));
    let request = ar24(&[I915_FORMAT_MOD_X_TILED], 64, 64).implicit_interop();
    assert!(device.allocate(&request).is_ok());
}

#[test]
fn implicit_interop_without_fences_allocates_linear() {
    let request = request("AB24", &[LINEAR], 256, 256).implicit_interop();
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![create_ext(262144, 0, &[DEVICE_REGION, SYSTEM_REGION], None)],
    };
    assert_eq!(allocated(DG2, &request), expected);
}

#[test]
fn failed_memory_region_query_uses_gem_create() {
    let device = Device::builder(ADL)
        .fail(Fault::new(DRM_IOCTL_I915_QUERY))
        .open()
        .unwrap();
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256);
    let expected = Allocated {
        pitches: vec![pitch(0, 1024)],
        calls: vec![
            Call::I915GemCreate { size: 262144 },
            set_tiling(I915_TILING_X, 1024),
        ],
    };
    assert_eq!(allocated_on(&device, &request), expected);
}

#[test]
fn pat_index_rejections_fail_the_allocation() {
    let device = Device::new(MTL);
    device.fail(Fault::new(DRM_IOCTL_I915_GEM_CREATE_EXT).errno(libc::EINVAL));
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256);
    let error = device.allocate(&request).err().unwrap();
    assert!(matches!(
        error,
        AllocatorError::I915Allocate(I915Error::GemCreateExt(errno)) if errno.0 == libc::EINVAL
    ));
}

#[test]
fn other_gem_create_errors_fail_the_allocation() {
    let device = Device::new(MTL);
    device.fail(Fault::new(DRM_IOCTL_I915_GEM_CREATE_EXT).errno(libc::ENOMEM));
    let request = ar24(&[I915_FORMAT_MOD_X_TILED], 64, 64);
    let error = device.allocate(&request).err().unwrap();
    assert!(matches!(
        error,
        AllocatorError::I915Allocate(I915Error::GemCreateExt(errno)) if errno.0 == libc::ENOMEM
    ));
}

#[test]
fn virtio_hosts_with_fences_allocate_like_native_devices() {
    let hosts = [
        VirtioHost {
            virtio: VIRTIO_SKL,
            native: SKL,
        },
        VirtioHost {
            virtio: VIRTIO_TGL,
            native: TGL,
        },
    ];
    for host in hosts {
        for (index, request) in comparison_requests().iter().enumerate() {
            assert_eq!(
                native(host.virtio, request),
                native(host.native, request),
                "{} request {index}",
                host.virtio
            );
        }
    }
}

#[test]
fn virtio_hosts_without_fences_allocate_like_native_devices() {
    let hosts = [
        VirtioHost {
            virtio: VIRTIO_DG2,
            native: DG2,
        },
        VirtioHost {
            virtio: VIRTIO_MTL,
            native: MTL,
        },
    ];
    for host in hosts {
        for (index, request) in comparison_requests().iter().enumerate() {
            let sent = native(host.virtio, request);
            let tiling = sent
                .calls
                .iter()
                .filter(|call| matches!(call, Call::I915GemSetTiling { .. }))
                .count();
            assert_eq!(tiling, 0, "{} request {index}", host.virtio);
            assert_eq!(
                sent,
                native(host.native, request),
                "{} request {index}",
                host.virtio
            );
        }
    }
}

#[test]
fn virtio_domain_requests_are_sent_with_their_buffer() {
    let device = Device::new(VIRTIO_MTL);
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256);
    let create = virtio_create(262144, &[SYSTEM_REGION], Some(MTL_PAT_INDEX_WC));
    for _ in 0..2 {
        let buffer = device.allocate(&request).unwrap();
        assert_eq!(virtio_calls(&buffer), [create.clone(), set_domain()]);
    }
}

#[test]
fn failed_virtio_flushes_break_the_allocator() {
    let device = Device::new(VIRTIO_MTL);
    let flush = 4;
    device.fail(
        Fault::new(DRM_IOCTL_VIRTGPU_EXECBUFFER)
            .nth(flush)
            .errno(libc::EIO),
    );
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256);
    let error = device.allocate(&request).err().unwrap();
    assert!(matches!(
        error,
        AllocatorError::VirtioGpuAllocate(VirtioGpuError::I915(I915Error::VirtioFlush(
            VdrmError::Execbuffer(errno)
        ))) if errno.0 == libc::EIO
    ));
    let error = device.allocate(&request).err().unwrap();
    assert!(matches!(
        error,
        AllocatorError::VirtioGpuAllocate(VirtioGpuError::I915(I915Error::VirtioBlob(
            VdrmError::Broken(errno)
        ))) if errno.0 == libc::EIO
    ));
}

#[test]
fn virtio_i915_host_is_the_virtio_driver() {
    assert_eq!(Device::new(VIRTIO_SKL).driver(), DriverName::VirtioGpuI915);
}

#[test]
fn virtio_blobs_are_shareable_host_memory_with_deferred_mapping() {
    let request = request("AB24", &[LINEAR], 256, 256);
    let buffer = Device::new(VIRTIO_DG2).allocate(&request).unwrap();
    let create = virtio_create(262144, &[DEVICE_REGION, SYSTEM_REGION], None);
    assert_eq!(virtio_calls(&buffer)[0], create);
}

#[test]
fn virtio_pat_index_is_sent_inline() {
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256);
    let buffer = Device::new(VIRTIO_MTL).allocate(&request).unwrap();
    let create = virtio_create(262144, &[SYSTEM_REGION], Some(MTL_PAT_INDEX_WC));
    assert_eq!(virtio_calls(&buffer)[0], create);
}

#[test]
fn failed_virtio_memory_region_query_uses_gem_create() {
    let device = Device::builder(VIRTIO_SKL)
        .fail_host(Fault::new(I915_CCMD_QUERYPARAM))
        .open()
        .unwrap();
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256).implicit_interop();
    let buffer = device.allocate(&request).unwrap();
    let create = Call::VirtgpuI915GemCreate {
        blob_mem: VIRTGPU_BLOB_MEM_HOST3D,
        blob_flags: BLOB_FLAGS,
        blob_hints: DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING,
        size: 262144,
        gem_size: 262144,
    };
    let expected = [create, set_tiling(I915_TILING_X, 1024)];
    assert_eq!(virtio_calls(&buffer), expected);
}

#[test]
fn virtio_probe_without_a_host_object_has_no_tiling_ioctls() {
    let device = Device::builder(VIRTIO_SKL)
        .fail_host(Fault::new(I915_CCMD_GEM_CREATE).errno(libc::ENOMEM))
        .open()
        .unwrap();
    let request = ar24(&[I915_FORMAT_MOD_X_TILED, LINEAR], 64, 64).implicit_interop();
    assert_eq!(device.chosen(&request), Some(LINEAR));
}

#[test]
fn virtio_fence_probe_errors_mean_no_fences() {
    let fault = Fault::new(I915_CCMD_IOCTL_SIMPLE).errno(libc::EIO);
    let device = Device::builder(VIRTIO_SKL).fail_host(fault).open().unwrap();
    let request = ar24(&[I915_FORMAT_MOD_X_TILED, LINEAR], 64, 64).implicit_interop();
    assert_eq!(device.chosen(&request), Some(LINEAR));
}

#[test]
fn virtio_fence_probe_transport_errors_mean_no_fences() {
    let probe_get_tiling = 3;
    let fault = Fault::new(DRM_IOCTL_VIRTGPU_EXECBUFFER)
        .nth(probe_get_tiling)
        .errno(libc::EIO);
    let device = Device::builder(VIRTIO_SKL).fail(fault).open().unwrap();
    let request = ar24(&[I915_FORMAT_MOD_X_TILED, LINEAR], 64, 64).implicit_interop();
    assert_eq!(device.chosen(&request), Some(LINEAR));
}

#[test]
fn virtio_host_tiling_errors_are_ignored() {
    let set_tiling = 2;
    let device = Device::new(VIRTIO_SKL);
    device.fail_host(
        Fault::new(I915_CCMD_IOCTL_SIMPLE)
            .nth(set_tiling)
            .errno(libc::EIO),
    );
    let request = ar24(&[I915_FORMAT_MOD_X_TILED], 64, 64).implicit_interop();
    assert!(device.allocate(&request).is_ok());
}

#[test]
fn virtio_host_domain_errors_are_ignored() {
    let device = Device::new(VIRTIO_SKL);
    device.fail_host(Fault::new(I915_CCMD_IOCTL_SIMPLE).errno(libc::EIO));
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256).implicit_interop();
    let buffer = device.allocate(&request).unwrap();
    let expected = [
        virtio_create(262144, &[SYSTEM_REGION], None),
        set_domain(),
        set_tiling(I915_TILING_X, 1024),
    ];
    assert_eq!(virtio_calls(&buffer), expected);
}

#[test]
fn virtio_cross_device_hosts_get_cross_device_blobs() {
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 256, 256);
    let buffer = Device::new(VIRTIO_TGL_CROSS_DEVICE)
        .allocate(&request)
        .unwrap();
    let create = Call::VirtgpuI915GemCreateExt {
        blob_mem: VIRTGPU_BLOB_MEM_HOST3D,
        blob_flags: BLOB_FLAGS | VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE,
        blob_hints: DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING,
        size: 262144,
        gem_size: 262144,
        flags: 0,
        regions: Some(vec![SYSTEM_REGION]),
        pat_index: None,
    };
    assert_eq!(virtio_calls(&buffer)[0], create);
}

#[test]
fn virtio_blob_sizes_are_not_aligned() {
    let request = request("AB24", &[I915_FORMAT_MOD_X_TILED], 64, 1);
    let buffer = Device::new(VIRTIO_TGL).allocate(&request).unwrap();
    assert_eq!(
        virtio_calls(&buffer)[0],
        virtio_create(4096, &[SYSTEM_REGION], None)
    );
    let error = Device::builder(VIRTIO_TGL_64K).open().err().unwrap();
    assert!(matches!(
        error,
        AllocatorError::VirtioGpuOpen(VirtioGpuError::I915(I915Error::VirtioBlob(
            VdrmError::Blob(errno)
        ))) if errno.0 == libc::EINVAL
    ));
}

#[test]
fn tiling_errors_are_ignored_without_implicit_interop() {
    let device = Device::new(ADL);
    device.fail(Fault::new(DRM_IOCTL_I915_GEM_SET_TILING));
    let request = ar24(&[I915_FORMAT_MOD_X_TILED], 64, 64);
    assert!(device.allocate(&request).is_ok());
}

#[test]
fn tiled_render_targets_wider_than_16382_are_rejected_on_gen8_and_skl_without_gt4() {
    let max = MAX_TILED_WIDTH_ON_GEN8_AND_SKL_WITHOUT_GT4;
    for PciDevice { info, device } in
        pci_devices(|info| info.ver == 8 || (info.skl && info.gt != 4))
    {
        for tiled in [I915_FORMAT_MOD_X_TILED, I915_FORMAT_MOD_Y_TILED] {
            let at_limit = device.chosen(&ar24(&[tiled], max, 1));
            let above_limit = device.chosen(&ar24(&[tiled], max + 1, 1));
            assert_eq!(at_limit, Some(tiled), "{:#x}", info.pci_id);
            assert_eq!(above_limit, None, "{:#x}", info.pci_id);
        }
    }
}

#[test]
fn skl_gt4_tiles_render_targets_wider_than_16382() {
    for PciDevice { info, device } in pci_devices(|info| info.skl && info.gt == 4) {
        let modifier = device.chosen(&ar24(&[I915_FORMAT_MOD_Y_TILED], MAX_EXTENT, 1));
        assert_eq!(
            modifier,
            Some(I915_FORMAT_MOD_Y_TILED),
            "{:#x}",
            info.pci_id
        );
    }
}

#[test]
fn gen8_surfaces_are_at_most_2_gib() {
    let width = MAX_TILED_WIDTH_ON_GEN8_AND_SKL_WITHOUT_GT4;
    let at_limit = request("AB48", &[I915_FORMAT_MOD_X_TILED], width, MAX_EXTENT);
    let above_limit = request("AB8F", &[I915_FORMAT_MOD_X_TILED], width, MAX_EXTENT);
    for PciDevice { info, device } in pci_devices(|info| info.ver == 8) {
        let expected = vec![Plane::at(0, 131072, 1 << 31)];
        assert_eq!(
            layout_planes(&device, &at_limit),
            Some(expected),
            "{:#x}",
            info.pci_id
        );
        assert_eq!(
            layout_planes(&device, &above_limit),
            None,
            "{:#x}",
            info.pci_id
        );
    }
}

#[test]
fn three_byte_elements_span_three_tiles() {
    let request = request("RG24", &[I915_FORMAT_MOD_Y_TILED], 200, 40);
    for PciDevice { info, device } in pci_devices(|info| info.ver >= 8 && info.verx10 < 125) {
        let expected = vec![Plane::at(0, 640, 36864)];
        assert_eq!(
            layout_planes(&device, &request),
            Some(expected),
            "{:#x}",
            info.pci_id
        );
    }
}

#[test]
fn y_ccs_planes_of_64_and_128_bit_formats_are_one_page() {
    let request64 = request("XB4H", &[I915_FORMAT_MOD_Y_TILED_CCS], 256, 256);
    let request128 = request("AB8F", &[I915_FORMAT_MOD_Y_TILED_CCS], 256, 256);
    let expected64 = vec![Plane::at(0, 2048, 524288), Plane::at(524288, 128, 4096)];
    let expected128 = vec![Plane::at(0, 4096, 1048576), Plane::at(1048576, 128, 4096)];
    for PciDevice { info, device } in pci_devices(|info| (9..=11).contains(&info.ver)) {
        assert_eq!(
            layout_planes(&device, &request64),
            Some(expected64.clone()),
            "{:#x}",
            info.pci_id
        );
        assert_eq!(
            layout_planes(&device, &request128),
            Some(expected128.clone()),
            "{:#x}",
            info.pci_id
        );
    }
}

#[test]
fn gen12_clear_color_plane_holds_the_clear_color_state() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC], 64, 64);
    for PciDevice { info, device } in pci_devices(|info| info.verx10 == 120 && info.has_aux_map) {
        let expected = vec![
            Plane::at(0, 512, 32768),
            Plane::at(32768, 64, 128),
            Plane::at(36864, 64, info.clear_color_state_size),
        ];
        assert_eq!(
            layout_planes(&device, &request),
            Some(expected),
            "{:#x}",
            info.pci_id
        );
    }
}

#[test]
fn tiled_layouts_need_power_of_two_or_three_times_power_of_two_elements() {
    let tileable = |bits: u32| {
        (bits >= 8 && bits.is_power_of_two())
            || (bits.is_multiple_of(3) && bits / 3 >= 8 && (bits / 3).is_power_of_two())
    };
    let formats = formats(|format| format.0.planes.iter().any(|plane| !tileable(plane.bits)));
    for PciDevice { info, device } in pci_devices(|info| info.ver >= 8) {
        for &format in &formats {
            let request = Request::for_format(format, 64, 64).modifiers(&[I915_FORMAT_MOD_X_TILED]);
            assert!(
                device.layout(&request).is_none(),
                "{:#x} {}",
                info.pci_id,
                fourcc(format)
            );
        }
    }
}

#[test]
fn multi_plane_formats_have_no_ccs_layout() {
    let modifiers = [
        I915_FORMAT_MOD_Y_TILED_CCS,
        I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS,
        I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC,
        I915_FORMAT_MOD_4_TILED_DG2_RC_CCS,
        I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC,
        I915_FORMAT_MOD_4_TILED_MTL_RC_CCS,
        I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC,
        I915_FORMAT_MOD_4_TILED_LNL_CCS,
        I915_FORMAT_MOD_4_TILED_BMG_CCS,
    ];
    let formats = formats(|format| format.0.planes.len() > 1);
    for PciDevice { info, device } in pci_devices(|info| info.ver >= 8) {
        for modifier in modifiers {
            for &format in &formats {
                let request = Request::for_format(format, 64, 64).modifiers(&[modifier]);
                assert!(
                    device.layout(&request).is_none(),
                    "{:#x} {modifier:#x} {}",
                    info.pci_id,
                    fourcc(format)
                );
            }
        }
    }
}

#[test]
fn surfaces_above_2_gib_are_allowed_from_gen9() {
    let width = MAX_TILED_WIDTH_ON_GEN8_AND_SKL_WITHOUT_GT4;
    let request = request("AB8F", &[I915_FORMAT_MOD_X_TILED], width, MAX_EXTENT);
    for PciDevice { info, device } in pci_devices(|info| info.ver >= 9) {
        let expected = vec![Plane::at(0, 262144, 1 << 32)];
        assert_eq!(
            layout_planes(&device, &request),
            Some(expected),
            "{:#x}",
            info.pci_id
        );
    }
}

#[test]
fn y_ccs_needs_32_64_or_128_bit_elements() {
    let formats = formats(|format| {
        let bits = format.0.planes[0].bits;
        !matches!(bits, 32 | 64 | 128)
    });
    for PciDevice { info, device } in pci_devices(|info| (9..=11).contains(&info.ver)) {
        for &format in &formats {
            let request =
                Request::for_format(format, 64, 64).modifiers(&[I915_FORMAT_MOD_Y_TILED_CCS]);
            assert!(
                device.layout(&request).is_none(),
                "{:#x} {}",
                info.pci_id,
                fourcc(format)
            );
        }
    }
}

#[test]
fn only_known_modifiers_have_a_layout() {
    let modifiers = unknown_modifiers();
    let mut configurations: Vec<&DeviceInfo> = Vec::new();
    for info in DEVICES.iter().filter(|info| info.ver >= 8) {
        let known = configurations
            .iter()
            .any(|other| configuration(other) == configuration(info));
        if !known {
            configurations.push(info);
        }
    }
    for fourcc in ["AR24", "R8  ", "AB48", "NV12"] {
        let request = Request::new(fourcc, 64, 64);
        for info in &configurations {
            for &modifier in &modifiers {
                assert!(
                    intel::layout(&request, modifier, info).is_none(),
                    "{:#x} {fourcc} {modifier:#x}",
                    info.pci_id
                );
            }
        }
    }
}

#[test]
fn y_tiled_scanout_is_rejected_on_gen8() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED], 64, 64).scanout();
    for PciDevice { info, device } in pci_devices(|info| info.ver == 8) {
        assert!(device.layout(&request).is_none(), "{:#x}", info.pci_id);
    }
}

#[test]
fn y_tiled_scanout_is_allowed_from_gen9() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED], 64, 64).scanout();
    for PciDevice { info, device } in pci_devices(|info| info.ver >= 9 && info.verx10 < 125) {
        let modifier = device.chosen(&request);
        assert_eq!(
            modifier,
            Some(I915_FORMAT_MOD_Y_TILED),
            "{:#x}",
            info.pci_id
        );
    }
}

#[test]
fn y_ccs_is_rejected_on_gen8() {
    let request = ar24(&[I915_FORMAT_MOD_Y_TILED_CCS], 64, 64);
    for PciDevice { info, device } in pci_devices(|info| info.ver == 8) {
        assert!(device.layout(&request).is_none(), "{:#x}", info.pci_id);
    }
}

#[test]
fn gen9_aux_height_is_rounded_to_512_rows() {
    let ccs = planes(SKL, &ar24(&[I915_FORMAT_MOD_Y_TILED_CCS], 256, 513))[1];
    assert_eq!(ccs.stride, 128);
    assert_eq!(ccs.size, 2 * GEN9_CCS_ROWS_PER_512_IMAGE_ROWS * ccs.stride);
}

#[test]
fn xe_implicit_interop_refuses_x_tiling() {
    let request = ar24(&[I915_FORMAT_MOD_X_TILED, LINEAR], 256, 256).implicit_interop();
    assert_eq!(chosen(ADL_XE, &request), Some(LINEAR));
}

#[test]
fn unknown_devices_only_support_linear() {
    let with_linear = ar24(
        &[I915_FORMAT_MOD_4_TILED, I915_FORMAT_MOD_X_TILED, LINEAR],
        64,
        64,
    );
    let without_linear = ar24(&[I915_FORMAT_MOD_4_TILED, I915_FORMAT_MOD_X_TILED], 64, 64);
    for name in ["lnl", "bmg"] {
        let device = unknown_device(name, true);
        assert_eq!(device.driver(), DriverName::Xe, "{name}");
        let modifier = device.chosen(&with_linear);
        assert_eq!(modifier, Some(LINEAR), "{name}");
        assert!(device.layout(&without_linear).is_none(), "{name}");
    }
    for name in ["adl_gt2", "dg2_g10", "mtl_u"] {
        let device = unknown_device(name, false);
        assert_eq!(device.driver(), DriverName::I915, "{name}");
        let modifier = device.chosen(&with_linear);
        assert_eq!(modifier, Some(LINEAR), "{name}");
        assert!(device.layout(&without_linear).is_none(), "{name}");
    }
}

#[test]
fn unknown_xe_devices_allocate_linear_like_known_devices() {
    for request in [ar24(&[LINEAR], 64, 64), ar24(&[LINEAR], 64, 64).scanout()] {
        let lnl = allocated_on(&unknown_device("lnl", true), &request);
        assert_eq!(lnl, allocated(LNL, &request));
        let bmg = allocated_on(&unknown_device("bmg", true), &request);
        assert_eq!(bmg, allocated(BMG, &request));
    }
}

#[test]
fn unknown_i915_devices_allocate_linear_like_devices_without_pat_indices() {
    let request = ar24(&[LINEAR], 64, 64);
    let adl = allocated_on(&unknown_device("adl_gt2", false), &request);
    assert_eq!(adl, allocated(ADL, &request));
    let dg2 = allocated_on(&unknown_device("dg2_g10", false), &request);
    assert_eq!(dg2, allocated(DG2, &request));
}

#[test]
fn unknown_i915_devices_set_no_pat_index() {
    let request = ar24(&[LINEAR], 64, 64);
    let known = allocated(MTL, &request);
    let unknown = allocated_on(&unknown_device("mtl_u", false), &request);
    assert_eq!(
        known.calls[0],
        create_ext(16384, 0, &[SYSTEM_REGION], Some(MTL_PAT_INDEX_WC))
    );
    assert_eq!(unknown.calls[0], system_create(16384));
    assert_eq!(unknown.calls[1..], known.calls[1..]);
}

#[test]
fn unknown_discrete_devices_open_without_memory_regions() {
    let device = unknown_device_builder("dg2_g10", false)
        .fail(Fault::new(DRM_IOCTL_I915_QUERY))
        .open()
        .unwrap();
    assert!(device.layout(&ar24(&[LINEAR], 64, 64)).is_some());
}

#[test]
fn devices_older_than_gen8_fail_to_open() {
    let values = DeviceValues {
        name: "ilk",
        pci_id: 0x0042,
        verx10: 50,
        has_local_mem: false,
    };
    let kernel = IntelKernel::from_values(values, KernelVersion::NEWEST, false, false)
        .with_i915_on_xe_platforms();
    let error = intel_device(kernel).open().err().unwrap();
    assert!(matches!(error, AllocatorError::I915Open(I915Error::Device)));
}
