use super::*;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD;

fn field(value: u64, shift: u32) -> u64 {
    value << shift
}

fn gfx9(tile: u64, version: u64) -> u64 {
    AMD_FMT_MOD
        | field(version, AMD_FMT_MOD_TILE_VERSION_SHIFT)
        | field(tile, AMD_FMT_MOD_TILE_SHIFT)
}

fn gfx6(tile: u64, microtile: u64) -> u64 {
    AMD_FMT_MOD
        | field(AMD_FMT_MOD_TILE_VER_GFX6, AMD_FMT_MOD_TILE_VERSION_SHIFT)
        | field(tile, AMD_FMT_MOD_TILE_SHIFT)
        | field(microtile, AMD_FMT_MOD_MICROTILE_SHIFT)
}

#[test]
fn linear_is_its_own_class() {
    assert_eq!(classify(DRM_FORMAT_MOD_LINEAR), Class::Linear);
}

#[test]
fn gfx6_modifiers_are_classified_by_their_array_mode() {
    let display = gfx6(
        AMD_FMT_MOD_TILE_GFX6_1D_TILED_THIN1,
        AMD_FMT_MOD_MICROTILE_DISPLAY,
    );
    let thin = gfx6(
        AMD_FMT_MOD_TILE_GFX6_1D_TILED_THIN1,
        AMD_FMT_MOD_MICROTILE_THIN,
    );
    let two_d = gfx6(
        AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
        AMD_FMT_MOD_MICROTILE_THIN,
    );
    assert_eq!(
        classify(display),
        Class::Gfx6OneD {
            microtile: Microtile::Display
        }
    );
    assert_eq!(
        classify(thin),
        Class::Gfx6OneD {
            microtile: Microtile::Thin
        }
    );
    assert_eq!(classify(two_d), Class::Gfx6TwoD { dcc: false });
    let with_dcc = two_d | field(1, AMD_FMT_MOD_DCC_SHIFT);
    assert_eq!(classify(with_dcc), Class::Gfx6TwoD { dcc: true });
    assert!(dcc(with_dcc));
}

#[test]
fn gfx9_to_gfx12_modifiers_carry_their_block_size() {
    let versions = [
        AMD_FMT_MOD_TILE_VER_GFX9,
        AMD_FMT_MOD_TILE_VER_GFX10,
        AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS,
        AMD_FMT_MOD_TILE_VER_GFX11,
    ];
    for version in versions {
        let class = |tile: u64| classify(gfx9(tile, version));
        assert_eq!(
            class(AMD_FMT_MOD_TILE_GFX9_4K_D_X),
            Class::Gfx9To11 { block_log2: 12 }
        );
        assert_eq!(
            class(AMD_FMT_MOD_TILE_GFX9_64K_R_X),
            Class::Gfx9To11 { block_log2: 16 }
        );
        assert_eq!(
            class(AMD_FMT_MOD_TILE_GFX11_256K_R_X),
            Class::Gfx9To11 { block_log2: 18 }
        );
    }
    let class = |tile: u64| classify(gfx9(tile, AMD_FMT_MOD_TILE_VER_GFX12));
    assert_eq!(
        class(AMD_FMT_MOD_TILE_GFX12_256B_2D),
        Class::Gfx12 { block_log2: 8 }
    );
    assert_eq!(
        class(AMD_FMT_MOD_TILE_GFX12_256K_2D),
        Class::Gfx12 { block_log2: 18 }
    );
}

#[test]
#[should_panic(expected = "unknown AMD tile version 6")]
fn an_unknown_tile_version_is_refused() {
    classify(gfx9(AMD_FMT_MOD_TILE_GFX9_64K_S, 6));
}

#[test]
fn the_tile_version_names_the_generation() {
    let version = |version: u64| tile_version(gfx9(AMD_FMT_MOD_TILE_GFX9_64K_S, version));
    assert_eq!(version(AMD_FMT_MOD_TILE_VER_GFX9), TileVersion::Gfx9);
    assert_eq!(
        version(AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS),
        TileVersion::Gfx10Rbplus
    );
    assert_eq!(version(AMD_FMT_MOD_TILE_VER_GFX12), TileVersion::Gfx12);
}

#[test]
fn the_pipe_config_gives_the_pipe_count() {
    let with_config = |config: u64| pipes(field(config, AMD_FMT_MOD_PIPE_CONFIG_SHIFT));
    assert_eq!(with_config(AMD_FMT_MOD_PIPE_CONFIG_P2), 2);
    assert_eq!(with_config(AMD_FMT_MOD_PIPE_CONFIG_P4_32x32), 4);
    assert_eq!(with_config(AMD_FMT_MOD_PIPE_CONFIG_P8_16x16_8x16), 8);
    assert_eq!(with_config(AMD_FMT_MOD_PIPE_CONFIG_P8_32x64_32x32), 8);
    assert_eq!(with_config(AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16), 16);
}

#[test]
#[should_panic(expected = "unknown AMD pipe config 15")]
fn an_unnamed_pipe_config_is_refused() {
    pipes(field(0xf, AMD_FMT_MOD_PIPE_CONFIG_SHIFT));
}

#[test]
fn the_macro_tile_fields_are_powers_of_two() {
    let modifier = field(2, AMD_FMT_MOD_BANK_WIDTH_SHIFT)
        | field(1, AMD_FMT_MOD_BANK_HEIGHT_SHIFT)
        | field(3, AMD_FMT_MOD_MACRO_TILE_ASPECT_SHIFT)
        | field(2, AMD_FMT_MOD_NUM_BANKS_SHIFT);
    assert_eq!(bank_width(modifier), 4);
    assert_eq!(bank_height(modifier), 2);
    assert_eq!(aspect(modifier), 8);
    assert_eq!(banks(modifier), 8);
    let modifier = field(5, AMD_FMT_MOD_PIPE_XOR_BITS_SHIFT) | field(3, AMD_FMT_MOD_RB_SHIFT);
    assert_eq!(pipe_xor_bits(modifier), 5);
    assert_eq!(rb(modifier), 3);
    assert!(!dcc_retile(modifier));
    assert!(dcc_retile(field(1, AMD_FMT_MOD_DCC_RETILE_SHIFT)));
    assert!(dcc_pipe_align(field(1, AMD_FMT_MOD_DCC_PIPE_ALIGN_SHIFT)));
}

#[test]
fn a_block_is_as_square_as_its_size_allows() {
    let dims = |width: u32, height: u32| BlockDims { width, height };
    assert_eq!(block_dims(16, 2), dims(128, 128));
    assert_eq!(block_dims(16, 3), dims(128, 64));
    assert_eq!(block_dims(8, 0), dims(16, 16));
    assert_eq!(block_dims(12, 4), dims(16, 16));
}

#[test]
#[should_panic(expected = "exceeds the block")]
fn an_element_larger_than_the_block_is_refused() {
    block_dims(3, 4);
}
