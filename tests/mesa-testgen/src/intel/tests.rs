use super::*;

#[test]
fn x_tiles_are_512_bytes_by_8_rows() {
    let tile = tile(I915_FORMAT_MOD_X_TILED);
    assert_eq!(tile.width_bytes, 512);
    assert_eq!(tile.height, Some(8));
}

#[test]
fn y_and_4_tiles_are_128_bytes_by_32_rows() {
    for modifier in [
        I915_FORMAT_MOD_Y_TILED,
        I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC,
        I915_FORMAT_MOD_4_TILED,
        I915_FORMAT_MOD_4_TILED_BMG_CCS,
    ] {
        let tile = tile(modifier);
        assert_eq!(tile.width_bytes, 128, "{modifier:#x}");
        assert_eq!(tile.height, Some(32), "{modifier:#x}");
    }
}

#[test]
fn linear_has_no_tile_height() {
    let tile = tile(DRM_FORMAT_MOD_LINEAR);
    assert_eq!(tile.width_bytes, 256);
    assert_eq!(tile.height, None);
}

#[test]
#[should_panic(expected = "unknown Intel modifier")]
fn an_unknown_modifier_is_refused() {
    tile(fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_Yf_TILED);
}

#[test]
fn every_xe_variant_device_reaches_the_xe_minimum() {
    assert!(DEVICES.iter().any(|device| device.verx10 >= XE_MIN_VERX10));
}
