use super::*;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_LINEAR;

fn block(bits: u32) -> Block {
    Block {
        bits,
        width: 1,
        height: 1,
    }
}

fn size(width: u32, height: u32) -> Size {
    Size { width, height }
}

#[test]
fn the_element_is_the_block_or_a_third_of_twelve_bytes() {
    for (bytes, log2) in [1, 2, 4, 8, 16].into_iter().zip(0..) {
        assert_eq!(element_log2(bytes), log2);
        assert_eq!(element_log2(bytes), bytes.trailing_zeros());
    }
    assert_eq!(element_log2(EXPANDED_BYTES), 2);
    assert_eq!(
        element_log2(EXPANDED_BYTES),
        EXPANDED_BYTES.trailing_zeros()
    );
}

#[test]
#[should_panic(expected = "has no power-of-two element")]
fn three_byte_blocks_have_no_element() {
    element_log2(3);
}

#[test]
fn the_expanded_linear_widths_surround_the_pitch_in_pixels() {
    let mut expected = fixed_edges();
    for multiple in 1..=9i64 {
        let width = (64 * multiple + 2) / 3;
        expected.extend([width - 1, width, width + 1]);
    }
    assert_eq!(linear_expanded_widths(), clamp(expected));
    assert!(linear_expanded_widths().contains(&22));
}

#[test]
fn linear_sizes_follow_the_pitch_alignment() {
    let sizes = size_set(&[block(32)], GfxLevel::Gfx10, DRM_FORMAT_MOD_LINEAR);
    for expected in [
        size(64, 1),
        size(63, 1),
        size(65, 1),
        size(192, 1),
        size(1, 1),
    ] {
        assert!(sizes.contains(&expected), "{expected:?}");
    }
    assert!(
        sizes
            .iter()
            .all(|size| size.height <= 2 || size.height >= MAX_EXTENT - 1)
    );
    let expanded = size_set(&[block(96)], GfxLevel::Gfx10, DRM_FORMAT_MOD_LINEAR);
    assert!(expanded.contains(&size(22, 1)));
    assert!(expanded.contains(&size(43, 1)));
    assert!(!sizes.contains(&size(22, 1)));
}

#[test]
fn all_sizes_stay_inside_the_maximal_extent() {
    let sizes = size_set(
        &[block(8), block(128)],
        GfxLevel::Gfx10,
        DRM_FORMAT_MOD_LINEAR,
    );
    assert!(!sizes.is_empty());
    for size in sizes {
        assert!((1..=MAX_EXTENT).contains(&size.width));
        assert!((1..=MAX_EXTENT).contains(&size.height));
    }
}

#[test]
fn a_double_row_block_has_eight_bytes_for_two_pixels() {
    let double_row = Block {
        bits: 64,
        width: 2,
        height: 1,
    };
    assert!(is_double_row_block(&double_row));
    assert!(!is_double_row_block(&block(64)));
}
