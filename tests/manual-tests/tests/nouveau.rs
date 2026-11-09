use manual_tests::Call;
use manual_tests::Device;
use manual_tests::LINEAR;
use manual_tests::Plane;
use manual_tests::Request;
use manual_tests::nvidia::COMPRESSION_CDE_VERTICAL;
use manual_tests::nvidia::COMPRESSION_NONE;
use manual_tests::nvidia::COMPRESSION_RESERVED;
use manual_tests::nvidia::COMPRESSION_ROP_3D_LAYOUT_2;
use manual_tests::nvidia::EIGHT_GOBS;
use manual_tests::nvidia::FOUR_GOBS;
use manual_tests::nvidia::GOB_KIND_VERSION_FERMI;
use manual_tests::nvidia::GOB_KIND_VERSION_G80;
use manual_tests::nvidia::GOB_KIND_VERSION_RESERVED;
use manual_tests::nvidia::GOB_KIND_VERSION_TURING;
use manual_tests::nvidia::NV_MMU_PTE_KIND_GENERIC_MEMORY;
use manual_tests::nvidia::NV_MMU_PTE_KIND_GENERIC_MEMORY_COMPRESSIBLE;
use manual_tests::nvidia::NV_MMU_PTE_KIND_Z16;
use manual_tests::nvidia::ONE_GOB;
use manual_tests::nvidia::SECTOR_LAYOUT_BLACKWELL_16BPP;
use manual_tests::nvidia::SECTOR_LAYOUT_DESKTOP;
use manual_tests::nvidia::SECTOR_LAYOUT_RESERVED;
use manual_tests::nvidia::SIXTEEN_GOBS;
use manual_tests::nvidia::SIXTY_FOUR_GOBS;
use manual_tests::nvidia::THIRTY_TWO_GOBS;
use manual_tests::nvidia::TWO_GOBS;
use manual_tests::nvidia::turing;
use manual_tests::nvidia::turing_compressed;
use manual_tests::request;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_VENDOR_INTEL;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_VENDOR_NVIDIA;
use manual_tests::uapi::drm_fourcc::fourcc_mod_code;
use manual_tests::uapi::nouveau::NOUVEAU_GEM_DOMAIN_GART;
use manual_tests::uapi::nouveau::NOUVEAU_GEM_DOMAIN_VRAM;

const AD102: &str = "nouveau:chip=AD102";
const GP100: &str = "nouveau:chip=GP100";
const PASCAL_B_AND_LATER: [&str; 7] = [
    "nouveau:chip=GP102",
    "nouveau:chip=GP104",
    "nouveau:chip=GP106",
    "nouveau:chip=GP107",
    "nouveau:chip=GP108",
    "nouveau:chip=GV100",
    AD102,
];

const MAX_EXTENT: u32 = 32768;
const MAX_EXTENT_BEFORE_PASCAL_B: u32 = 16384;
const RESERVED_LOW_BIT: u64 = 1 << 5;
const RESERVED_HIGH_BIT: u64 = 1 << 28;
const TILE_MODE_Y_LOG2_SHIFT: u32 = 4;
const TILE_FLAGS_KIND_SHIFT: u32 = 8;

const TURING_FOUR_GOBS: u64 = turing(FOUR_GOBS);
const G80_ONE_GOB: u64 = DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(
    COMPRESSION_NONE,
    SECTOR_LAYOUT_DESKTOP,
    GOB_KIND_VERSION_G80,
    NV_MMU_PTE_KIND_GENERIC_MEMORY,
    ONE_GOB,
);
const DRM_FORMAT_MOD_NVIDIA_TEGRA_TILED: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_NVIDIA, 1);

fn chosen(device: &str, request: &Request) -> Option<u64> {
    Device::new(device).chosen(request)
}

fn planes(device: &str, request: &Request) -> Vec<Plane> {
    let layout = Device::new(device).layout(request).unwrap();
    manual_tests::planes(&layout)
}

fn gem_new(device: &str, request: &Request) -> Call {
    let buffer = Device::new(device).allocate(request).unwrap();
    buffer.only(|call| matches!(call, Call::NouveauGemNew { .. }).then(|| call.clone()))
}

fn block_linear_gem_new(size: u64, align: u32, log2_gobs: u64) -> Call {
    let log2_gobs = u32::try_from(log2_gobs).unwrap();
    let kind = u32::try_from(NV_MMU_PTE_KIND_GENERIC_MEMORY).unwrap();
    Call::NouveauGemNew {
        domain: NOUVEAU_GEM_DOMAIN_VRAM | NOUVEAU_GEM_DOMAIN_GART,
        size,
        align,
        tile_mode: log2_gobs << TILE_MODE_Y_LOG2_SHIFT,
        tile_flags: kind << TILE_FLAGS_KIND_SHIFT,
    }
}

#[test]
fn zero_extents_have_no_layout() {
    for modifier in [LINEAR, TURING_FOUR_GOBS] {
        assert_eq!(chosen(AD102, &request("AR24", &[modifier], 0, 16)), None);
        assert_eq!(chosen(AD102, &request("AR24", &[modifier], 16, 0)), None);
    }
}

#[test]
fn extents_up_to_32768_have_a_layout_since_pascal_b() {
    for device in PASCAL_B_AND_LATER {
        for modifier in [LINEAR, TURING_FOUR_GOBS] {
            let wide = request("AR24", &[modifier], MAX_EXTENT, 1);
            let tall = request("AR24", &[modifier], 1, MAX_EXTENT);
            assert_eq!(chosen(device, &wide), Some(modifier));
            assert_eq!(chosen(device, &tall), Some(modifier));
        }
    }
}

#[test]
fn extents_before_pascal_b_are_limited_to_16384() {
    let max = MAX_EXTENT_BEFORE_PASCAL_B;
    for modifier in [LINEAR, TURING_FOUR_GOBS] {
        let largest = request("AR24", &[modifier], max, max);
        let wide = request("AR24", &[modifier], max + 1, 1);
        let tall = request("AR24", &[modifier], 1, max + 1);
        assert_eq!(chosen(GP100, &largest), Some(modifier));
        assert_eq!(chosen(GP100, &wide), None);
        assert_eq!(chosen(GP100, &tall), None);
    }
}

#[test]
fn extents_above_32768_have_no_layout() {
    for modifier in [LINEAR, TURING_FOUR_GOBS] {
        let wide = request("AR24", &[modifier], MAX_EXTENT + 1, 1);
        let tall = request("AR24", &[modifier], 1, MAX_EXTENT + 1);
        assert_eq!(chosen(AD102, &wide), None);
        assert_eq!(chosen(AD102, &tall), None);
    }
}

#[test]
fn block_linear_needs_a_render_format() {
    for fourcc in ["NV12", "YUYV", "C8  "] {
        let fallback = request(fourcc, &[TURING_FOUR_GOBS, LINEAR], 64, 64);
        let block_linear = request(fourcc, &[TURING_FOUR_GOBS], 64, 64);
        assert_eq!(chosen(AD102, &fallback), Some(LINEAR));
        assert_eq!(chosen(AD102, &block_linear), None);
    }
}

#[test]
fn render_formats_take_block_linear() {
    for fourcc in ["AR24", "R8  ", "AB8F", "Y410"] {
        let request = request(fourcc, &[TURING_FOUR_GOBS, LINEAR], 64, 64);
        assert_eq!(chosen(AD102, &request), Some(TURING_FOUR_GOBS));
    }
}

#[test]
fn thirty_two_gob_blocks_are_passed_to_gem_new() {
    let request = request("AR24", &[turing(THIRTY_TWO_GOBS)], 1920, 1080);
    let expected = block_linear_gem_new(9830400, 16384, THIRTY_TWO_GOBS);
    assert_eq!(gem_new(AD102, &request), expected);
}

#[test]
fn block_height_is_clamped_to_the_image() {
    let request = request("AR24", &[turing(THIRTY_TWO_GOBS)], 64, 31);
    let expected = block_linear_gem_new(8192, 4096, FOUR_GOBS);
    assert_eq!(gem_new(AD102, &request), expected);
}

#[test]
fn block_height_is_clamped_to_one_and_two_gobs() {
    let one = request("AR24", &[turing(FOUR_GOBS)], 64, 8);
    let two = request("AR24", &[turing(FOUR_GOBS)], 64, 16);
    assert_eq!(
        gem_new(AD102, &one),
        block_linear_gem_new(4096, 4096, ONE_GOB)
    );
    assert_eq!(
        gem_new(AD102, &two),
        block_linear_gem_new(4096, 4096, TWO_GOBS)
    );
}

#[test]
fn stride_is_aligned_to_a_gob() {
    let request = request("AR24", &[turing(FOUR_GOBS)], 33, 1);
    assert_eq!(planes(AD102, &request)[0].stride, 192);
}

#[test]
fn g80_gobs_are_four_rows_high() {
    let request = request("AR24", &[G80_ONE_GOB], 64, 4);
    assert_eq!(planes(AD102, &request), [Plane::first(256, 1024)]);
}

#[test]
fn g80_page_kind_is_passed_to_gem_new() {
    let request = request("AR24", &[G80_ONE_GOB], 64, 4);
    let expected = block_linear_gem_new(4096, 4096, ONE_GOB);
    assert_eq!(gem_new(AD102, &request), expected);
}

#[test]
fn invalid_modifiers_have_no_layout() {
    let invalid = [
        DRM_FORMAT_MOD_NVIDIA_TEGRA_TILED,
        turing(FOUR_GOBS) | RESERVED_LOW_BIT,
        turing(FOUR_GOBS) | RESERVED_HIGH_BIT,
        DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(
            COMPRESSION_RESERVED,
            SECTOR_LAYOUT_DESKTOP,
            GOB_KIND_VERSION_TURING,
            NV_MMU_PTE_KIND_GENERIC_MEMORY,
            FOUR_GOBS,
        ),
        DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(
            COMPRESSION_NONE,
            SECTOR_LAYOUT_RESERVED,
            GOB_KIND_VERSION_TURING,
            NV_MMU_PTE_KIND_GENERIC_MEMORY,
            FOUR_GOBS,
        ),
        DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(
            COMPRESSION_NONE,
            SECTOR_LAYOUT_DESKTOP,
            GOB_KIND_VERSION_RESERVED,
            NV_MMU_PTE_KIND_GENERIC_MEMORY,
            FOUR_GOBS,
        ),
    ];
    for invalid in invalid {
        assert_eq!(chosen(AD102, &request("AR24", &[invalid], 64, 64)), None);
    }
}

#[test]
fn largest_valid_fields_have_a_layout() {
    let valid = [
        turing(THIRTY_TWO_GOBS),
        DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(
            COMPRESSION_CDE_VERTICAL,
            SECTOR_LAYOUT_DESKTOP,
            GOB_KIND_VERSION_TURING,
            NV_MMU_PTE_KIND_GENERIC_MEMORY,
            FOUR_GOBS,
        ),
        DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(
            COMPRESSION_NONE,
            SECTOR_LAYOUT_BLACKWELL_16BPP,
            GOB_KIND_VERSION_TURING,
            NV_MMU_PTE_KIND_GENERIC_MEMORY,
            FOUR_GOBS,
        ),
    ];
    for valid in valid {
        let request = request("AR24", &[valid], 64, 64);
        assert_eq!(chosen(AD102, &request), Some(valid));
    }
}

#[test]
fn block_heights_above_thirty_two_gobs_have_no_layout() {
    let request = request("AR24", &[turing(SIXTY_FOUR_GOBS)], 64, 64);
    assert_eq!(chosen(AD102, &request), None);
}

#[test]
fn other_vendors_have_no_layout() {
    let intel = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, TURING_FOUR_GOBS);
    assert_eq!(chosen(AD102, &request("AR24", &[intel], 64, 64)), None);
}

#[test]
fn fermi_gobs_have_a_layout() {
    let fermi = DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(
        COMPRESSION_NONE,
        SECTOR_LAYOUT_DESKTOP,
        GOB_KIND_VERSION_FERMI,
        NV_MMU_PTE_KIND_GENERIC_MEMORY,
        FOUR_GOBS,
    );
    assert_eq!(
        chosen(AD102, &request("AR24", &[fermi], 64, 64)),
        Some(fermi)
    );
}

#[test]
fn odd_page_kinds_have_a_layout() {
    let z16 = DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(
        COMPRESSION_NONE,
        SECTOR_LAYOUT_DESKTOP,
        GOB_KIND_VERSION_TURING,
        NV_MMU_PTE_KIND_Z16,
        FOUR_GOBS,
    );
    assert_eq!(chosen(AD102, &request("AR24", &[z16], 64, 64)), Some(z16));
}

#[test]
fn the_largest_block_height_is_chosen_for_every_image_height() {
    let all = [
        turing(ONE_GOB),
        turing(TWO_GOBS),
        turing(FOUR_GOBS),
        turing(THIRTY_TWO_GOBS),
        turing(SIXTEEN_GOBS),
        turing(EIGHT_GOBS),
    ];
    for height in [100, 86, 50, 43, 30, 22, 15, 11, 5] {
        let request = request("AR24", &all, 64, height);
        assert_eq!(chosen(AD102, &request), Some(turing(THIRTY_TWO_GOBS)));
    }
}

#[test]
fn the_larger_block_height_is_preferred() {
    let low_first = request("AR24", &[turing(ONE_GOB), turing(EIGHT_GOBS)], 64, 100);
    let high_first = request(
        "AR24",
        &[turing(THIRTY_TWO_GOBS), turing(TWO_GOBS)],
        64,
        100,
    );
    assert_eq!(chosen(AD102, &low_first), Some(turing(EIGHT_GOBS)));
    assert_eq!(chosen(AD102, &high_first), Some(turing(THIRTY_TWO_GOBS)));
}

#[test]
fn compression_is_preferred_at_the_best_block_height() {
    let modifiers = [turing(SIXTEEN_GOBS), turing_compressed(SIXTEEN_GOBS)];
    let request = request("AR24", &modifiers, 64, 100);
    assert_eq!(
        chosen(AD102, &request),
        Some(turing_compressed(SIXTEEN_GOBS))
    );
}

#[test]
fn compression_beats_a_larger_block_height() {
    let modifiers = [turing(SIXTEEN_GOBS), turing_compressed(EIGHT_GOBS)];
    let request = request("AR24", &modifiers, 64, 100);
    assert_eq!(chosen(AD102, &request), Some(turing_compressed(EIGHT_GOBS)));
}

#[test]
fn compression_beats_a_closer_block_height() {
    let near = [turing(FOUR_GOBS), turing_compressed(EIGHT_GOBS)];
    let far = [turing(EIGHT_GOBS), turing_compressed(THIRTY_TWO_GOBS)];
    let near = request("AR24", &near, 64, 100);
    let far = request("AR24", &far, 64, 100);
    assert_eq!(chosen(AD102, &near), Some(turing_compressed(EIGHT_GOBS)));
    assert_eq!(
        chosen(AD102, &far),
        Some(turing_compressed(THIRTY_TWO_GOBS))
    );
}

#[test]
fn every_compression_beats_uncompressed_at_the_same_block_height() {
    let other = DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(
        COMPRESSION_ROP_3D_LAYOUT_2,
        SECTOR_LAYOUT_DESKTOP,
        GOB_KIND_VERSION_TURING,
        NV_MMU_PTE_KIND_GENERIC_MEMORY_COMPRESSIBLE,
        SIXTEEN_GOBS,
    );
    let request = request("AR24", &[turing(SIXTEEN_GOBS), other], 64, 100);
    assert_eq!(chosen(AD102, &request), Some(other));
}

#[test]
fn linear_gem_new_has_no_tiling() {
    let request = request("AR24", &[LINEAR], 64, 64);
    let Call::NouveauGemNew {
        tile_mode,
        tile_flags,
        ..
    } = gem_new(AD102, &request)
    else {
        unreachable!();
    };
    assert_eq!(tile_mode, 0);
    assert_eq!(tile_flags, 0);
}
