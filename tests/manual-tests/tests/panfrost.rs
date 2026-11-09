use manual_tests::AllocatorError;
use manual_tests::Call;
use manual_tests::Device;
use manual_tests::DriverName;
use manual_tests::Fault;
use manual_tests::LINEAR;
use manual_tests::PanfrostError;
use manual_tests::Plane;
use manual_tests::Request;
use manual_tests::VirtioGpuError;
use manual_tests::request;
use manual_tests::uapi::drm_fourcc::AFBC_FORMAT_MOD_BLOCK_SIZE_16x16;
use manual_tests::uapi::drm_fourcc::AFBC_FORMAT_MOD_BLOCK_SIZE_32x8;
use manual_tests::uapi::drm_fourcc::AFBC_FORMAT_MOD_SC;
use manual_tests::uapi::drm_fourcc::AFBC_FORMAT_MOD_SPARSE;
use manual_tests::uapi::drm_fourcc::AFBC_FORMAT_MOD_SPLIT;
use manual_tests::uapi::drm_fourcc::AFBC_FORMAT_MOD_TILED;
use manual_tests::uapi::drm_fourcc::AFBC_FORMAT_MOD_YTR;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_ARM_AFBC;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_X_TILED;
use manual_tests::uapi::panfrost::PANFROST_BO_NOEXEC;
use manual_tests::uapi::vdrm::PANFROST_CCMD_GET_PARAM;
use manual_tests::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE;
use manual_tests::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_MAPPABLE;
use manual_tests::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
use manual_tests::uapi::virtgpu::VIRTGPU_BLOB_MEM_HOST3D;

const T720: &str = "panfrost:gpu=t720";
const T860: &str = "panfrost:gpu=t860";
const G52: &str = "panfrost:gpu=g52";
const G52_WITHOUT_AFBC: &str = "panfrost:gpu=g52:afbc_features=1";
const G57: &str = "panfrost:gpu=g57";
const G610: &str = "panthor:gpu=g610";
const G72: &str = "panfrost:gpu=g72";
const G720: &str = "panthor:gpu=g720";
const LATER_MIDGARD: [&str; 4] = [
    "panfrost:gpu=t760",
    "panfrost:gpu=t820",
    "panfrost:gpu=t830",
    "panfrost:gpu=t880",
];
const GPUS: [&str; 4] = ["t720", "t860", "g52", "g57"];
const VIRTIO_G52: &str = "virtio_gpu:host=panfrost_g52";
const VIRTIO_G52_CROSS_DEVICE: &str = "virtio_gpu:host=panfrost_g52_cross_device";

const MAX_EXTENT: u32 = 16384;

const AFBC_16X16_SPARSE: u64 =
    DRM_FORMAT_MOD_ARM_AFBC(AFBC_FORMAT_MOD_BLOCK_SIZE_16x16 | AFBC_FORMAT_MOD_SPARSE);
const AFBC_16X16_SPARSE_YTR: u64 = DRM_FORMAT_MOD_ARM_AFBC(
    AFBC_FORMAT_MOD_BLOCK_SIZE_16x16 | AFBC_FORMAT_MOD_SPARSE | AFBC_FORMAT_MOD_YTR,
);
const AFBC_16X16_SPARSE_SPLIT: u64 = DRM_FORMAT_MOD_ARM_AFBC(
    AFBC_FORMAT_MOD_BLOCK_SIZE_16x16 | AFBC_FORMAT_MOD_SPARSE | AFBC_FORMAT_MOD_SPLIT,
);
const AFBC_32X8_SPARSE: u64 =
    DRM_FORMAT_MOD_ARM_AFBC(AFBC_FORMAT_MOD_BLOCK_SIZE_32x8 | AFBC_FORMAT_MOD_SPARSE);
const AFBC_32X8_SPARSE_SPLIT: u64 = DRM_FORMAT_MOD_ARM_AFBC(
    AFBC_FORMAT_MOD_BLOCK_SIZE_32x8 | AFBC_FORMAT_MOD_SPARSE | AFBC_FORMAT_MOD_SPLIT,
);
const AFBC_32X8_SPARSE_SPLIT_YTR: u64 = DRM_FORMAT_MOD_ARM_AFBC(
    AFBC_FORMAT_MOD_BLOCK_SIZE_32x8
        | AFBC_FORMAT_MOD_SPARSE
        | AFBC_FORMAT_MOD_SPLIT
        | AFBC_FORMAT_MOD_YTR,
);
const AFBC_16X16_TILED_SC_SPARSE: u64 = DRM_FORMAT_MOD_ARM_AFBC(
    AFBC_FORMAT_MOD_BLOCK_SIZE_16x16
        | AFBC_FORMAT_MOD_TILED
        | AFBC_FORMAT_MOD_SC
        | AFBC_FORMAT_MOD_SPARSE,
);
const AFBC_16X16_TILED_SC_SPARSE_YTR: u64 = DRM_FORMAT_MOD_ARM_AFBC(
    AFBC_FORMAT_MOD_BLOCK_SIZE_16x16
        | AFBC_FORMAT_MOD_TILED
        | AFBC_FORMAT_MOD_SC
        | AFBC_FORMAT_MOD_SPARSE
        | AFBC_FORMAT_MOD_YTR,
);

#[derive(Debug, PartialEq, Eq)]
struct PanfrostCreated {
    modifier: u64,
    planes: Vec<Plane>,
    size: u64,
    flags: u32,
}

fn chosen(device: &str, request: &Request) -> Option<u64> {
    Device::new(device).chosen(request)
}

fn plane(device: &str, request: &Request) -> Plane {
    let layout = Device::new(device).layout(request).unwrap();
    manual_tests::planes(&layout)[0]
}

fn create(device: &str, request: &Request) -> Call {
    let buffer = Device::new(device).allocate(request).unwrap();
    buffer.only(|call| {
        matches!(
            call,
            Call::PanfrostCreateBo { .. } | Call::PanthorBoCreate { .. }
        )
        .then(|| call.clone())
    })
}

fn created(device: &str, request: &Request) -> PanfrostCreated {
    let buffer = Device::new(device).allocate(request).unwrap();
    buffer.only(|call| match *call {
        Call::PanfrostCreateBo { size, flags } => Some(PanfrostCreated {
            modifier: buffer.modifier(),
            planes: buffer.planes(),
            size: u64::from(size),
            flags,
        }),
        Call::VirtgpuPanfrostCreateBo {
            gem_size, flags, ..
        } => Some(PanfrostCreated {
            modifier: buffer.modifier(),
            planes: buffer.planes(),
            size: gem_size,
            flags,
        }),
        _ => None,
    })
}

#[test]
fn afbc_is_preferred() {
    let request = request("AR24", &[AFBC_16X16_SPARSE, LINEAR], 64, 64);
    assert_eq!(chosen(G52, &request), Some(AFBC_16X16_SPARSE));
}

#[test]
fn nonzero_afbc_features_disable_afbc() {
    let request = request("AR24", &[AFBC_16X16_SPARSE, LINEAR], 64, 64);
    assert_eq!(chosen(G52, &request), Some(AFBC_16X16_SPARSE));
    assert_eq!(chosen(G52_WITHOUT_AFBC, &request), Some(LINEAR));
}

#[test]
fn later_midgard_has_afbc() {
    let request = request("AR24", &[AFBC_16X16_SPARSE], 64, 64);
    for device in LATER_MIDGARD {
        assert_eq!(chosen(device, &request), Some(AFBC_16X16_SPARSE));
    }
}

#[test]
fn zero_extents_have_no_layout() {
    for modifiers in [&[LINEAR][..], &[AFBC_16X16_SPARSE]] {
        assert_eq!(chosen(G52, &request("AR24", modifiers, 0, 64)), None);
        assert_eq!(chosen(G52, &request("AR24", modifiers, 64, 0)), None);
    }
}

#[test]
fn extents_up_to_16384_have_a_layout() {
    for modifiers in [&[LINEAR][..], &[AFBC_16X16_SPARSE]] {
        let wide = request("AR24", modifiers, MAX_EXTENT, 1);
        let tall = request("AR24", modifiers, 1, MAX_EXTENT);
        assert_eq!(chosen(G52, &wide), Some(modifiers[0]));
        assert_eq!(chosen(G52, &tall), Some(modifiers[0]));
    }
}

#[test]
fn extents_above_16384_have_no_layout() {
    for modifiers in [&[LINEAR][..], &[AFBC_16X16_SPARSE]] {
        let wide = request("AR24", modifiers, MAX_EXTENT + 1, 1);
        let tall = request("AR24", modifiers, 1, MAX_EXTENT + 1);
        assert_eq!(chosen(G52, &wide), None);
        assert_eq!(chosen(G52, &tall), None);
    }
}

#[test]
fn panfrost_buffers_stay_below_4_gib() {
    let below = request("AB8F", &[LINEAR], MAX_EXTENT, MAX_EXTENT - 1);
    let at = request("AB8F", &[LINEAR], MAX_EXTENT, MAX_EXTENT);
    assert_eq!(chosen(G52, &below), Some(LINEAR));
    assert_eq!(chosen(G52, &at), None);
}

#[test]
fn panthor_buffers_can_reach_4_gib() {
    let request = request("AB8F", &[LINEAR], MAX_EXTENT, MAX_EXTENT);
    assert_eq!(chosen(G610, &request), Some(LINEAR));
}

#[test]
fn linear_buffers_of_4_byte_pixels_include_the_crc() {
    let full_hd = request("AR24", &[LINEAR], 1920, 1080);
    let short = request("AR24", &[LINEAR], 1920, 15);
    assert_eq!(plane(G52, &full_hd), Plane::first(7680, 8359936));
    assert_eq!(plane(G52, &short), Plane::first(7680, 118784));
}

#[test]
fn linear_buffers_above_4_byte_pixels_have_no_crc() {
    let request = request("AB4H", &[LINEAR], 65, 17);
    assert_eq!(plane(G52, &request), Plane::first(576, 12288));
}

#[test]
fn crc_header_ends_on_a_page() {
    let request = request("AR24", &[LINEAR], 1, 59);
    assert_eq!(plane(G52, &request), Plane::first(64, 4096));
}

#[test]
fn crc_regions_are_64_pixels_from_arch_12() {
    let short = request("AR24", &[LINEAR], 1, 480);
    let tall = request("AR24", &[LINEAR], 1, 780);
    assert_eq!(plane(G720, &short), Plane::first(64, 36864));
    assert_eq!(plane(G720, &tall), Plane::first(64, 57344));
}

#[test]
fn u_interleaved_strides_cover_16_pixel_blocks() {
    let u = DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED;
    assert_eq!(
        plane(G52, &request("AR24", &[u], 65, 17)),
        Plane::first(320, 12288)
    );
    assert_eq!(
        plane(G52, &request("AR24", &[u], 1920, 1080)),
        Plane::first(7680, 8421376)
    );
}

#[test]
fn u_interleaved_without_crc_fills_pages() {
    let u = DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED;
    let request = request("AB4H", &[u], 16, 32);
    assert_eq!(plane(G52, &request), Plane::first(128, 4096));
}

#[test]
fn u_interleaved_is_preferred_over_linear() {
    let u = DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED;
    assert_eq!(chosen(G52, &request("AR24", &[LINEAR, u], 64, 64)), Some(u));
}

#[test]
fn afbc_16x16_strides_cover_16_pixel_superblocks() {
    let request = |fourcc| request(fourcc, &[AFBC_16X16_SPARSE], 65, 17);
    assert_eq!(plane(G52, &request("AR24")), Plane::first(320, 12288));
    assert_eq!(plane(T860, &request("AR24")), Plane::first(320, 12288));
    assert_eq!(plane(G57, &request("RX24")), Plane::first(320, 12288));
    assert_eq!(plane(G610, &request("AB4H")), Plane::first(640, 24576));
    assert_eq!(plane(G610, &request("R16 ")), Plane::first(160, 8192));
}

#[test]
fn afbc_16x16_ytr_has_a_layout() {
    let request = request("AR24", &[AFBC_16X16_SPARSE_YTR], 64, 64);
    assert_eq!(chosen(G52, &request), Some(AFBC_16X16_SPARSE_YTR));
}

#[test]
fn afbc_16x16_split_strides_cover_16_pixel_superblocks() {
    let request = request("R8  ", &[AFBC_16X16_SPARSE_SPLIT], 65, 17);
    assert_eq!(plane(G52, &request), Plane::first(80, 4096));
}

#[test]
fn afbc_split_is_allowed_from_arch_6() {
    let request = request("R8  ", &[AFBC_16X16_SPARSE_SPLIT], 16, 16);
    assert_eq!(chosen(G72, &request), Some(AFBC_16X16_SPARSE_SPLIT));
}

#[test]
fn afbc_32x8_strides_cover_32_pixel_superblocks() {
    let r8 = request("R8  ", &[AFBC_32X8_SPARSE], 65, 17);
    let split = request("AR24", &[AFBC_32X8_SPARSE_SPLIT], 65, 17);
    let ytr = request("AR24", &[AFBC_32X8_SPARSE_SPLIT_YTR], 65, 17);
    assert_eq!(plane(G52, &r8), Plane::first(96, 4096));
    assert_eq!(plane(G52, &split), Plane::first(384, 16384));
    assert_eq!(plane(G52, &ytr), Plane::first(384, 16384));
}

#[test]
fn afbc_tiled_strides_cover_tiles_of_8_superblocks() {
    let tiled = |fourcc| request(fourcc, &[AFBC_16X16_TILED_SC_SPARSE], 65, 17);
    let ytr = request("AR24", &[AFBC_16X16_TILED_SC_SPARSE_YTR], 65, 17);
    assert_eq!(plane(G52, &tiled("AR24")), Plane::first(512, 73728));
    assert_eq!(plane(G52, &ytr), Plane::first(512, 73728));
    assert_eq!(plane(G57, &tiled("AR24")), Plane::first(512, 73728));
    assert_eq!(plane(G610, &tiled("AB4H")), Plane::first(1024, 69632));
}

#[test]
fn afbc_tiled_widest_rows_have_a_layout() {
    let rgba = request("AR24", &[AFBC_16X16_TILED_SC_SPARSE], MAX_EXTENT, 1);
    let rgba16 = request("AB4H", &[AFBC_16X16_TILED_SC_SPARSE], 4096, 1);
    assert_eq!(plane(G57, &rgba), Plane::first(65536, 8540160));
    assert_eq!(plane(G610, &rgba16), Plane::first(32768, 2113536));
}

#[test]
fn afbc_tiled_header_is_page_aligned() {
    let request = request("AR24", &[AFBC_16X16_TILED_SC_SPARSE], 1281, 2945);
    assert_eq!(plane(G57, &request), Plane::first(5632, 17702912));
}

#[test]
fn afbc_needs_arch_5_and_tiling_arch_7_and_split_arch_6() {
    assert_eq!(
        chosen(T720, &request("AR24", &[AFBC_16X16_SPARSE], 64, 64)),
        None
    );
    let tiled = request("AR24", &[AFBC_16X16_TILED_SC_SPARSE], 64, 64);
    let split = request("R8  ", &[AFBC_16X16_SPARSE_SPLIT], 64, 64);
    assert_eq!(chosen(T860, &tiled), None);
    assert_eq!(chosen(T860, &split), None);
}

#[test]
fn afbc_modes_the_format_cannot_use_have_no_layout() {
    let rgba_32x8 = request("AR24", &[AFBC_32X8_SPARSE], 64, 64);
    let r8_32x8_split = request("R8  ", &[AFBC_32X8_SPARSE_SPLIT], 64, 64);
    let r8_ytr = request("R8  ", &[AFBC_16X16_SPARSE_YTR], 64, 64);
    let rgbx_on_v7 = request("RX24", &[AFBC_16X16_SPARSE], 64, 64);
    let r16_before_v10 = request("R16 ", &[AFBC_16X16_SPARSE], 64, 64);
    assert_eq!(chosen(G52, &rgba_32x8), None);
    assert_eq!(chosen(G52, &r8_32x8_split), None);
    assert_eq!(chosen(G52, &r8_ytr), None);
    assert_eq!(chosen(G52, &rgbx_on_v7), None);
    assert_eq!(chosen(G57, &r16_before_v10), None);
}

#[test]
fn foreign_modifiers_have_no_layout() {
    let request = request("AR24", &[I915_FORMAT_MOD_X_TILED], 64, 64);
    assert_eq!(chosen(G52, &request), None);
}

#[test]
fn panfrost_buffers_are_created_without_exec() {
    let expected = Call::PanfrostCreateBo {
        size: 12288,
        flags: PANFROST_BO_NOEXEC,
    };
    assert_eq!(
        create(G52, &request("AR24", &[AFBC_16X16_SPARSE], 65, 17)),
        expected
    );
}

#[test]
fn panthor_buffers_are_created_without_flags() {
    let expected = Call::PanthorBoCreate {
        size: 24576,
        flags: 0,
    };
    assert_eq!(
        create(G610, &request("AB4H", &[AFBC_16X16_SPARSE], 65, 17)),
        expected
    );
}

#[test]
fn afbc_header_is_128_byte_aligned_from_arch_6() {
    let request = request("AR24", &[AFBC_32X8_SPARSE_SPLIT], 129, 209);
    assert_eq!(plane(G52, &request), Plane::first(640, 151552));
}

#[test]
fn afbc_header_alignment_follows_the_arch() {
    let bifrost = request("AR24", &[AFBC_16X16_SPARSE], 81, 993);
    let small = request("AR24", &[AFBC_16X16_SPARSE], 1, 161);
    assert_eq!(plane(G52, &bifrost), Plane::first(384, 401408));
    assert_eq!(plane(T860, &small), Plane::first(64, 12288));
    assert_eq!(plane(G72, &small), Plane::first(64, 16384));
}

#[test]
fn afbc_header_covers_every_superblock() {
    let request = request("AR24", &[AFBC_16X16_SPARSE], 1920, 1080);
    assert_eq!(plane(G52, &request), Plane::first(7680, 8552448));
}

#[test]
fn afbc_header_of_sixteen_superblocks_takes_one_page() {
    let request = request("AR24", &[AFBC_16X16_SPARSE], 256, 16);
    assert_eq!(plane(G52, &request), Plane::first(1024, 20480));
}

#[test]
fn virtio_host_is_the_virtio_driver() {
    assert_eq!(
        Device::new(VIRTIO_G52).driver(),
        DriverName::VirtioGpuPanfrost
    );
}

#[test]
fn virtio_hosts_allocate_like_native_devices() {
    let modifiers = [
        AFBC_16X16_TILED_SC_SPARSE_YTR,
        AFBC_32X8_SPARSE_SPLIT_YTR,
        AFBC_16X16_SPARSE_YTR,
        AFBC_16X16_SPARSE,
        DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED,
        LINEAR,
    ];
    let requests = [
        request("AB24", &modifiers, 64, 64),
        request("AB24", &modifiers, 300, 200),
        request("AB24", &modifiers, 1920, 1080).scanout(),
        request("R8  ", &modifiers, 17, 3),
        request("AB24", &[LINEAR], 65, 3),
    ];
    for gpu in GPUS {
        let virtio = format!("virtio_gpu:host=panfrost_{gpu}");
        let native = format!("panfrost:gpu={gpu}");
        for (index, request) in requests.iter().enumerate() {
            assert_eq!(
                created(&virtio, request),
                created(&native, request),
                "{gpu} request {index}"
            );
        }
    }
}

#[test]
fn virtio_blobs_are_mappable_and_shareable() {
    let request = request("AB24", &[LINEAR], 64, 64);
    let native = created(G52, &request);
    let buffer = Device::new(VIRTIO_G52).allocate(&request).unwrap();
    let create = Call::VirtgpuPanfrostCreateBo {
        blob_mem: VIRTGPU_BLOB_MEM_HOST3D,
        blob_flags: VIRTGPU_BLOB_FLAG_USE_MAPPABLE | VIRTGPU_BLOB_FLAG_USE_SHAREABLE,
        blob_hints: 0,
        size: native.size,
        gem_size: native.size,
        flags: PANFROST_BO_NOEXEC,
    };
    assert_eq!(buffer.calls[0], create);
}

#[test]
fn virtio_blobs_are_cross_device_when_supported() {
    let request = request("AB24", &[LINEAR], 64, 64);
    let native = created(G52, &request);
    let buffer = Device::new(VIRTIO_G52_CROSS_DEVICE)
        .allocate(&request)
        .unwrap();
    let create = Call::VirtgpuPanfrostCreateBo {
        blob_mem: VIRTGPU_BLOB_MEM_HOST3D,
        blob_flags: VIRTGPU_BLOB_FLAG_USE_MAPPABLE
            | VIRTGPU_BLOB_FLAG_USE_SHAREABLE
            | VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE,
        blob_hints: 0,
        size: native.size,
        gem_size: native.size,
        flags: PANFROST_BO_NOEXEC,
    };
    assert_eq!(buffer.calls[0], create);
}

#[test]
fn virtio_gpu_query_errors_fail_the_open() {
    let error = Device::builder(VIRTIO_G52)
        .fail_host(Fault::new(PANFROST_CCMD_GET_PARAM))
        .open()
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::VirtioGpuOpen(VirtioGpuError::Panfrost(PanfrostError::VirtioGetParam(_)))
    ));
}
