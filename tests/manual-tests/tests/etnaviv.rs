use manual_tests::AllocatorError;
use manual_tests::Call;
use manual_tests::Device;
use manual_tests::DriverName;
use manual_tests::DrmError;
use manual_tests::EtnavivError;
use manual_tests::Fault;
use manual_tests::LINEAR;
use manual_tests::PAGE_SIZE;
use manual_tests::Plane;
use manual_tests::Request;
use manual_tests::request;
use manual_tests::uapi::drm::DRM_IOCTL_PRIME_HANDLE_TO_FD;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_VIVANTE_SUPER_TILED;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_VIVANTE_TILED;
use manual_tests::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GEM_NEW;
use manual_tests::uapi::etnaviv::DRM_IOCTL_ETNAVIV_GET_PARAM;
use manual_tests::uapi::etnaviv::ETNA_BO_WC;

const GC400: &str = "etnaviv:gpu=gc400_4652";
const GC2000: &str = "etnaviv:gpu=gc2000_5108";
const GC7000: &str = "etnaviv:gpu=gc7000_6204";
const GC7000_BLT: &str = "etnaviv:gpu=gc7000_6214";

const VIVANTE_MOD_TS_64_4: u64 = 1 << 48;

const MAX_EXTENT: u32 = 8192;
const MAX_EXTENT_WITHOUT_TEXTURE_8K: u32 = 2048;
const PIXEL_PIPES: u64 = 2;
const RS_ALIGN: u64 = 16;
const BLT_ALIGN: u64 = 4;
const LINEAR_ROWS: u64 = 4;
const TILE_ROWS: u64 = 4;
const SUPER_TILE: u64 = 64;
const PE_ALIGNMENT: u64 = 64;
const BO_CACHE_MAX_SIZE: u64 = 64 << 20;
const AR24_BYTES: u64 = 4;
const AB8F_EMULATED_BYTES: u64 = 8;
const AB8F_EMULATED_LAYERS: u64 = 2;
const YUYV_BLOCK_BYTES: u64 = 4;
const YUYV_BLOCK_WIDTH: u64 = 2;
const R8_BYTES: u64 = 1;
const FEATURES: &str = "features";
const MINOR_FEATURES0: &str = "minor_features0";
const MINOR_FEATURES1: &str = "minor_features1";
const MINOR_FEATURES4: &str = "minor_features4";
const MINOR_FEATURES5: &str = "minor_features5";
const CHIP_FEATURES_FAST_CLEAR: u32 = 0x00000001;
const CHIP_MINOR_FEATURES0_FLIP_Y: u32 = 0x00000001;
const CHIP_MINOR_FEATURES1_RSUV_SWIZZLE: u32 = 0x00000001;
const CHIP_MINOR_FEATURES1_HALTI0: u32 = 0x00800000;
const CHIP_MINOR_FEATURES4_FE_ENHANCEMENTS1: u32 = 0x00000001;
const CHIP_MINOR_FEATURES5_SH_ENHANCEMENTS7: u32 = 0x00000001;
const FEATURES_OF_THE_FIRST_CORE_QUERY: usize = 2;

struct FeatureWord {
    name: &'static str,
    value: u32,
}

fn chosen(device: &str, request: &Request) -> Option<u64> {
    Device::new(device).chosen(request)
}

fn planes(device: &str, request: &Request) -> Vec<Plane> {
    let layout = Device::new(device).layout(request).unwrap();
    manual_tests::planes(&layout)
}

fn create(device: &str, request: &Request) -> Call {
    let buffer = Device::new(device).allocate(request).unwrap();
    buffer.only(|call| matches!(call, Call::EtnavivGemNew { .. }).then(|| call.clone()))
}

fn with_features(device: &str, words: &[FeatureWord]) -> Device {
    let mut id = device.to_string();
    for word in words {
        id.push_str(&format!(":{}={:#x}", word.name, word.value));
    }
    Device::new(&id)
}

fn gem_new(size: u64) -> Call {
    Call::EtnavivGemNew {
        size,
        flags: ETNA_BO_WC,
    }
}

#[test]
fn etnaviv_is_its_own_driver() {
    assert_eq!(Device::new(GC400).driver(), DriverName::Etnaviv);
}

#[test]
fn open_fails_without_a_gpu() {
    let error = Device::builder(GC400)
        .fail(Fault::new(DRM_IOCTL_ETNAVIV_GET_PARAM))
        .open()
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::EtnavivOpen(EtnavivError::NoGpu)
    ));
}

#[test]
fn open_fails_without_a_3d_pipe() {
    let error = Device::builder(GC400)
        .fail(Fault::new(DRM_IOCTL_ETNAVIV_GET_PARAM).nth(FEATURES_OF_THE_FIRST_CORE_QUERY))
        .open()
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::EtnavivOpen(EtnavivError::NoGpu)
    ));
}

#[test]
fn open_fails_when_the_model_is_zero() {
    let error = Device::builder(&format!("{GC400}:model=0"))
        .open()
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::EtnavivOpen(EtnavivError::NoGpu)
    ));
}

#[test]
fn the_3d_pipe_is_bit_2_of_the_features() {
    let error = Device::builder(&format!("{GC400}:{FEATURES}={CHIP_FEATURES_FAST_CLEAR:#x}"))
        .open()
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::EtnavivOpen(EtnavivError::NoGpu)
    ));
}

#[test]
fn extents_are_limited_by_the_texture_size() {
    assert_eq!(
        chosen(GC400, &request("AR24", &[LINEAR], MAX_EXTENT, 1)),
        Some(LINEAR)
    );
    assert_eq!(
        chosen(GC400, &request("AR24", &[LINEAR], 1, MAX_EXTENT)),
        Some(LINEAR)
    );
    assert_eq!(
        chosen(GC400, &request("AR24", &[LINEAR], MAX_EXTENT + 1, 1)),
        None
    );
    assert_eq!(
        chosen(GC400, &request("AR24", &[LINEAR], 1, MAX_EXTENT + 1)),
        None
    );
    assert_eq!(chosen(GC400, &request("AR24", &[LINEAR], 0, 1)), None);
    assert_eq!(chosen(GC400, &request("AR24", &[LINEAR], 1, 0)), None);
}

#[test]
fn without_texture_8k_extents_are_limited_to_2048() {
    let device = with_features(
        GC400,
        &[FeatureWord {
            name: MINOR_FEATURES0,
            value: CHIP_MINOR_FEATURES0_FLIP_Y,
        }],
    );
    let at_limit = request("AR24", &[LINEAR], MAX_EXTENT_WITHOUT_TEXTURE_8K, 1);
    let above_limit = request("AR24", &[LINEAR], MAX_EXTENT_WITHOUT_TEXTURE_8K + 1, 1);
    assert!(device.layout(&at_limit).is_some());
    assert!(device.layout(&above_limit).is_none());
}

#[test]
fn single_pipe_prefers_super_tiled_then_tiled_then_linear() {
    let all = [
        LINEAR,
        DRM_FORMAT_MOD_VIVANTE_TILED,
        DRM_FORMAT_MOD_VIVANTE_SUPER_TILED,
        DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED,
        DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED,
    ];
    assert_eq!(
        chosen(GC7000, &request("AR24", &all, 64, 64)),
        Some(DRM_FORMAT_MOD_VIVANTE_SUPER_TILED)
    );
    let no_super = [
        LINEAR,
        DRM_FORMAT_MOD_VIVANTE_TILED,
        DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED,
        DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED,
    ];
    assert_eq!(
        chosen(GC7000, &request("AR24", &no_super, 64, 64)),
        Some(DRM_FORMAT_MOD_VIVANTE_TILED)
    );
    let split = [
        LINEAR,
        DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED,
        DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED,
    ];
    assert_eq!(
        chosen(GC7000, &request("AR24", &split, 64, 64)),
        Some(LINEAR)
    );
}

#[test]
fn multi_pipe_without_single_buffer_needs_split_tiling() {
    let unsplit = [
        DRM_FORMAT_MOD_VIVANTE_SUPER_TILED,
        DRM_FORMAT_MOD_VIVANTE_TILED,
    ];
    assert_eq!(chosen(GC2000, &request("AR24", &unsplit, 64, 64)), None);
    let split = [
        LINEAR,
        DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED,
        DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED,
    ];
    assert_eq!(
        chosen(GC2000, &request("AR24", &split, 64, 64)),
        Some(DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED)
    );
    let split_tiled = [LINEAR, DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED];
    assert_eq!(
        chosen(GC2000, &request("AR24", &split_tiled, 64, 64)),
        Some(DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED)
    );
}

#[test]
fn multi_pipe_without_single_buffer_refuses_tiled() {
    let device = with_features(
        GC7000_BLT,
        &[FeatureWord {
            name: MINOR_FEATURES4,
            value: CHIP_MINOR_FEATURES4_FE_ENHANCEMENTS1,
        }],
    );
    let layout = device.layout(&request("AR24", &[DRM_FORMAT_MOD_VIVANTE_TILED], 64, 64));
    assert!(layout.is_none());
}

#[test]
fn multi_pipe_with_single_buffer_prefers_unsplit_tiling() {
    let modifiers = [
        DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED,
        DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED,
        DRM_FORMAT_MOD_VIVANTE_TILED,
    ];
    assert_eq!(
        chosen(GC7000_BLT, &request("AR24", &modifiers, 64, 64)),
        Some(DRM_FORMAT_MOD_VIVANTE_TILED)
    );
    let split = [
        DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED,
        DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED,
    ];
    assert_eq!(
        chosen(GC7000_BLT, &request("AR24", &split, 64, 64)),
        Some(DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED)
    );
}

#[test]
fn single_pipe_without_single_buffer_allows_unsplit_tiling() {
    assert_eq!(
        chosen(
            GC400,
            &request("AR24", &[DRM_FORMAT_MOD_VIVANTE_TILED], 64, 64)
        ),
        Some(DRM_FORMAT_MOD_VIVANTE_TILED)
    );
    assert_eq!(
        chosen(
            GC400,
            &request("AR24", &[DRM_FORMAT_MOD_VIVANTE_SUPER_TILED], 64, 64)
        ),
        Some(DRM_FORMAT_MOD_VIVANTE_SUPER_TILED)
    );
}

#[test]
fn super_tiling_needs_16_or_32_bit_pixels_without_blt() {
    let super_tiled = [DRM_FORMAT_MOD_VIVANTE_SUPER_TILED];
    assert_eq!(chosen(GC7000, &request("R8  ", &super_tiled, 64, 64)), None);
    assert_eq!(chosen(GC7000, &request("AB4H", &super_tiled, 64, 64)), None);
    assert_eq!(
        chosen(GC7000, &request("AR15", &super_tiled, 64, 64)),
        Some(DRM_FORMAT_MOD_VIVANTE_SUPER_TILED)
    );
    assert_eq!(
        chosen(GC7000, &request("AR24", &super_tiled, 64, 64)),
        Some(DRM_FORMAT_MOD_VIVANTE_SUPER_TILED)
    );
    assert_eq!(
        chosen(
            GC7000,
            &request(
                "R8  ",
                &[
                    DRM_FORMAT_MOD_VIVANTE_SUPER_TILED,
                    DRM_FORMAT_MOD_VIVANTE_TILED
                ],
                64,
                64
            )
        ),
        Some(DRM_FORMAT_MOD_VIVANTE_TILED)
    );
    assert_eq!(
        chosen(GC7000_BLT, &request("R8  ", &super_tiled, 64, 64)),
        Some(DRM_FORMAT_MOD_VIVANTE_SUPER_TILED)
    );
    let split_super = [
        DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED,
        DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED,
    ];
    assert_eq!(
        chosen(GC2000, &request("AB24", &split_super, 64, 64)),
        Some(DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED)
    );
    assert_eq!(
        chosen(GC2000, &request("R8  ", &split_super, 64, 64)),
        Some(DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED)
    );
}

#[test]
fn super_tiling_needs_the_super_tiled_feature() {
    let device = with_features(
        GC400,
        &[FeatureWord {
            name: MINOR_FEATURES0,
            value: CHIP_MINOR_FEATURES0_FLIP_Y,
        }],
    );
    let layout = device.layout(&request(
        "AR24",
        &[DRM_FORMAT_MOD_VIVANTE_SUPER_TILED],
        64,
        64,
    ));
    assert!(layout.is_none());
}

#[test]
fn modifiers_with_tile_status_are_not_chosen() {
    let modifiers = [DRM_FORMAT_MOD_VIVANTE_TILED | VIVANTE_MOD_TS_64_4];
    assert_eq!(chosen(GC7000, &request("AR24", &modifiers, 64, 64)), None);
}

#[test]
fn implicit_interop_is_linear() {
    let modifiers = [DRM_FORMAT_MOD_VIVANTE_SUPER_TILED, LINEAR];
    let base = || request("AR24", &modifiers, 64, 64);
    assert_eq!(chosen(GC7000, &base().implicit_interop()), Some(LINEAR));
    assert_eq!(
        chosen(GC7000, &base().scanout()),
        Some(DRM_FORMAT_MOD_VIVANTE_SUPER_TILED)
    );
    assert_eq!(
        chosen(GC7000, &base()),
        Some(DRM_FORMAT_MOD_VIVANTE_SUPER_TILED)
    );
}

#[test]
fn formats_follow_the_gpu_features() {
    assert_eq!(chosen(GC400, &request("R8  ", &[LINEAR], 64, 64)), None);
    assert_eq!(chosen(GC400, &request("AB30", &[LINEAR], 64, 64)), None);
    assert_eq!(chosen(GC400, &request("AR12", &[LINEAR], 64, 64)), None);
    assert_eq!(
        chosen(GC2000, &request("R8  ", &[LINEAR], 64, 64)),
        Some(LINEAR)
    );
    assert_eq!(
        chosen(GC2000, &request("AB30", &[LINEAR], 64, 64)),
        Some(LINEAR)
    );
    assert_eq!(chosen(GC2000, &request("AB4H", &[LINEAR], 64, 64)), None);
    assert_eq!(chosen(GC2000, &request("AB8F", &[LINEAR], 64, 64)), None);
    assert_eq!(chosen(GC400, &request("AB8F", &[LINEAR], 64, 64)), None);
    assert_eq!(
        chosen(GC7000, &request("AB4H", &[LINEAR], 64, 64)),
        Some(LINEAR)
    );
    assert_eq!(
        chosen(GC7000, &request("AB8F", &[LINEAR], 64, 64)),
        Some(LINEAR)
    );
}

#[test]
fn ab4h_needs_halti2() {
    let device = with_features(
        GC7000,
        &[FeatureWord {
            name: MINOR_FEATURES4,
            value: CHIP_MINOR_FEATURES4_FE_ENHANCEMENTS1,
        }],
    );
    let layout = device.layout(&request("AB4H", &[LINEAR], 64, 64));
    assert!(layout.is_none());
}

#[test]
fn r8_without_halti5_is_sampler_only() {
    let device = with_features(
        GC7000,
        &[
            FeatureWord {
                name: MINOR_FEATURES1,
                value: CHIP_MINOR_FEATURES1_HALTI0 | CHIP_MINOR_FEATURES1_RSUV_SWIZZLE,
            },
            FeatureWord {
                name: MINOR_FEATURES5,
                value: CHIP_MINOR_FEATURES5_SH_ENHANCEMENTS7,
            },
        ],
    );
    let layout = device.layout(&request("R8  ", &[LINEAR], 1, 1)).unwrap();
    let stride = BLT_ALIGN * R8_BYTES;
    assert_eq!(
        manual_tests::planes(&layout),
        [Plane::first(stride, PE_ALIGNMENT)]
    );
}

#[test]
fn linear_padding_follows_the_rs_and_blt_alignment() {
    let linear = request("AR24", &[LINEAR], 1, 1);
    let stride = RS_ALIGN * AR24_BYTES;
    assert_eq!(
        planes(GC7000, &linear),
        [Plane::first(stride, stride * LINEAR_ROWS)]
    );
    let stride = BLT_ALIGN * AR24_BYTES;
    assert_eq!(
        planes(GC7000_BLT, &linear),
        [Plane::first(stride, PE_ALIGNMENT)]
    );
    let stride = 16 * AR24_BYTES;
    let linear = request("AR24", &[LINEAR], 16, 1);
    assert_eq!(planes(GC7000_BLT, &linear), [Plane::first(stride, stride)]);
}

#[test]
fn tiled_padding_follows_the_rs_and_blt_alignment() {
    let tiled = request("AR24", &[DRM_FORMAT_MOD_VIVANTE_TILED], 1, 1);
    let stride = RS_ALIGN * AR24_BYTES;
    assert_eq!(
        planes(GC7000, &tiled),
        [Plane::first(stride, stride * TILE_ROWS)]
    );
    let stride = BLT_ALIGN * AR24_BYTES;
    assert_eq!(
        planes(GC7000_BLT, &tiled),
        [Plane::first(stride, stride * TILE_ROWS)]
    );
}

#[test]
fn without_texture_halign_only_render_targets_use_rs_alignment() {
    let device = with_features(
        GC400,
        &[FeatureWord {
            name: MINOR_FEATURES1,
            value: CHIP_MINOR_FEATURES1_RSUV_SWIZZLE,
        }],
    );
    let layout = device.layout(&request("AR24", &[LINEAR], 1, 1)).unwrap();
    let stride = RS_ALIGN * AR24_BYTES;
    assert_eq!(
        manual_tests::planes(&layout),
        [Plane::first(stride, stride * LINEAR_ROWS)]
    );
}

#[test]
fn super_tiled_buffers_are_padded_to_a_super_tile() {
    let stride = SUPER_TILE * AR24_BYTES;
    let super_tiled = request("AR24", &[DRM_FORMAT_MOD_VIVANTE_SUPER_TILED], 1, 1);
    assert_eq!(
        planes(GC7000, &super_tiled),
        [Plane::first(stride, stride * SUPER_TILE)]
    );
}

#[test]
fn super_tiled_padding_ignores_the_pixel_pipes() {
    let stride = SUPER_TILE * AR24_BYTES;
    let super_tiled = request("AR24", &[DRM_FORMAT_MOD_VIVANTE_SUPER_TILED], 1, 1);
    assert_eq!(
        planes(GC7000_BLT, &super_tiled),
        [Plane::first(stride, stride * SUPER_TILE)]
    );
}

#[test]
fn split_padding_covers_every_pixel_pipe() {
    let stride = RS_ALIGN * AR24_BYTES;
    let split = request("AR24", &[DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED], 1, 1);
    let rows = TILE_ROWS * PIXEL_PIPES;
    assert_eq!(
        planes(GC2000, &split),
        [Plane::first(stride, stride * rows)]
    );
    let stride = SUPER_TILE * AR24_BYTES;
    let split_super = request("AR24", &[DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED], 1, 1);
    let rows = SUPER_TILE * PIXEL_PIPES;
    assert_eq!(
        planes(GC2000, &split_super),
        [Plane::first(stride, stride * rows)]
    );
}

#[test]
fn sizes_are_aligned_for_the_pixel_engine() {
    let linear = request("R8  ", &[LINEAR], 1, 1);
    assert_eq!(
        planes(GC7000_BLT, &linear),
        [Plane::first(BLT_ALIGN, PE_ALIGNMENT)]
    );
}

#[test]
fn ab8f_is_laid_out_as_two_64_bit_layers() {
    let linear = request("AB8F", &[LINEAR], 1, 1);
    let stride = RS_ALIGN * AB8F_EMULATED_BYTES;
    let size = stride * LINEAR_ROWS * AB8F_EMULATED_LAYERS;
    assert_eq!(planes(GC7000, &linear), [Plane::first(stride, size)]);
}

#[test]
fn yuyv_uses_2x1_blocks() {
    let linear = request("YUYV", &[LINEAR], 1, 1);
    let stride = RS_ALIGN / YUYV_BLOCK_WIDTH * YUYV_BLOCK_BYTES;
    assert_eq!(
        planes(GC7000, &linear),
        [Plane::first(stride, stride * LINEAR_ROWS)]
    );
}

#[test]
fn buffers_are_rounded_to_cache_buckets() {
    let width = 1080;
    let stride = 1088 * AR24_BYTES;
    let size = stride * LINEAR_ROWS;
    let linear = request("AR24", &[LINEAR], width, 1);
    assert_eq!(planes(GC2000, &linear), [Plane::first(stride, size)]);
    let bucket = 4 * PAGE_SIZE + PAGE_SIZE;
    assert!(size > 4 * PAGE_SIZE && size <= bucket);
    assert_eq!(create(GC2000, &linear), gem_new(bucket));
}

#[test]
fn sizes_above_five_quarters_use_the_six_quarters_bucket() {
    let linear = request("AR24", &[LINEAR], 1440, 1);
    let size = 1440 * AR24_BYTES * LINEAR_ROWS;
    let bucket = 4 * PAGE_SIZE + 4 * PAGE_SIZE * 2 / 4;
    assert!(size > 4 * PAGE_SIZE + PAGE_SIZE && size <= bucket);
    assert_eq!(create(GC2000, &linear), gem_new(bucket));
}

#[test]
fn small_buffers_use_page_buckets() {
    let linear = request("AR24", &[LINEAR], 640, 1);
    let size = 640 * AR24_BYTES * LINEAR_ROWS;
    assert!(size > 2 * PAGE_SIZE && size <= 3 * PAGE_SIZE);
    assert_eq!(create(GC2000, &linear), gem_new(3 * PAGE_SIZE));
    let tiny = request("R8  ", &[LINEAR], 1, 1);
    assert_eq!(create(GC2000, &tiny), gem_new(PAGE_SIZE));
}

#[test]
fn sizes_above_one_page_use_the_two_page_bucket() {
    let linear = request("AR24", &[LINEAR], 320, 1);
    let size = 320 * AR24_BYTES * LINEAR_ROWS;
    assert!(size > PAGE_SIZE && size <= 2 * PAGE_SIZE);
    assert_eq!(create(GC2000, &linear), gem_new(2 * PAGE_SIZE));
}

#[test]
fn the_largest_bucket_is_seven_quarters_of_the_cache_limit() {
    let linear = request("AB8F", &[LINEAR], 2560, 2560);
    let size = 2560 * AB8F_EMULATED_BYTES * 2560 * AB8F_EMULATED_LAYERS;
    let largest = BO_CACHE_MAX_SIZE + BO_CACHE_MAX_SIZE * 3 / 4;
    assert!(size > BO_CACHE_MAX_SIZE + BO_CACHE_MAX_SIZE / 2 && size <= largest);
    assert_eq!(create(GC7000, &linear), gem_new(largest));
}

#[test]
fn buffers_above_the_cache_limit_are_page_aligned() {
    let linear = request("AB8F", &[LINEAR], MAX_EXTENT, MAX_EXTENT);
    let extent = u64::from(MAX_EXTENT);
    let size = extent * AB8F_EMULATED_BYTES * extent * AB8F_EMULATED_LAYERS;
    assert!(size > BO_CACHE_MAX_SIZE + BO_CACHE_MAX_SIZE * 3 / 4);
    assert_eq!(create(GC7000, &linear), gem_new(size));
}

#[test]
fn failed_gem_new_fails_the_allocation() {
    let device = Device::new(GC7000);
    device.fail(Fault::new(DRM_IOCTL_ETNAVIV_GEM_NEW).errno(libc::ENOMEM));
    let error = device
        .allocate(&request("AR24", &[LINEAR], 64, 64))
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::EtnavivAllocate(EtnavivError::GemNew(errno)) if errno.0 == libc::ENOMEM
    ));
}

#[test]
fn failed_export_closes_the_gem_handle() {
    let device = Device::new(GC7000);
    device.fail(Fault::new(DRM_IOCTL_PRIME_HANDLE_TO_FD).errno(libc::EMFILE));
    let error = device
        .allocate(&request("AR24", &[LINEAR], 64, 64))
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::EtnavivAllocate(EtnavivError::Export(DrmError::PrimeHandleToFd(errno)))
            if errno.0 == libc::EMFILE
    ));
}
