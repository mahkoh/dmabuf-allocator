use manual_tests::Call;
use manual_tests::Device;
use manual_tests::LINEAR;
use manual_tests::Plane;
use manual_tests::Request;
use manual_tests::request;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_BROADCOM_UIF;

const BCM2711: &str = "v3d:gpu=bcm2711";

const MAX_EXTENT: u32 = 4096;

fn chosen(request: &Request) -> Option<u64> {
    Device::new(BCM2711).chosen(request)
}

fn plane(request: &Request) -> Plane {
    let layout = Device::new(BCM2711).layout(request).unwrap();
    manual_tests::planes(&layout)[0]
}

fn create(request: &Request) -> Call {
    let buffer = Device::new(BCM2711).allocate(request).unwrap();
    buffer.only(|call| matches!(call, Call::V3dCreateBo { .. }).then(|| call.clone()))
}

#[test]
fn zero_extents_have_no_layout() {
    for modifier in [LINEAR, DRM_FORMAT_MOD_BROADCOM_UIF] {
        assert_eq!(chosen(&request("AR24", &[modifier], 0, 64)), None);
        assert_eq!(chosen(&request("AR24", &[modifier], 64, 0)), None);
    }
}

#[test]
fn extents_up_to_4096_have_a_layout() {
    for modifier in [LINEAR, DRM_FORMAT_MOD_BROADCOM_UIF] {
        let wide = request("AR24", &[modifier], MAX_EXTENT, 1);
        let tall = request("AR24", &[modifier], 1, MAX_EXTENT);
        assert_eq!(chosen(&wide), Some(modifier));
        assert_eq!(chosen(&tall), Some(modifier));
    }
}

#[test]
fn extents_above_4096_have_no_layout() {
    for modifier in [LINEAR, DRM_FORMAT_MOD_BROADCOM_UIF] {
        let wide = request("AR24", &[modifier], MAX_EXTENT + 1, 1);
        let tall = request("AR24", &[modifier], 1, MAX_EXTENT + 1);
        assert_eq!(chosen(&wide), None);
        assert_eq!(chosen(&tall), None);
    }
}

#[test]
fn uif_utile_follows_the_pixel_size() {
    let uif = |fourcc| request(fourcc, &[DRM_FORMAT_MOD_BROADCOM_UIF], 65, 17);
    assert_eq!(plane(&uif("R8  ")), Plane::first(128, 4096));
    assert_eq!(plane(&uif("GR88")), Plane::first(256, 6144));
    assert_eq!(plane(&uif("AR24")), Plane::first(384, 9216));
    assert_eq!(plane(&uif("AB4H")), Plane::first(768, 15360));
    assert_eq!(plane(&uif("AB8F")), Plane::first(1280, 25600));
}

#[test]
fn uif_blocks_of_wide_pixels_are_four_rows_high() {
    let uif = |fourcc| request(fourcc, &[DRM_FORMAT_MOD_BROADCOM_UIF], 65, 1);
    assert_eq!(plane(&uif("AB4H")), Plane::first(768, 4 * 768));
    assert_eq!(plane(&uif("AB8F")), Plane::first(1280, 4 * 1280));
}

#[test]
fn uif_pads_rows_away_from_the_page_cache_boundary() {
    let uif = |height| request("AR24", &[DRM_FORMAT_MOD_BROADCOM_UIF], 65, height);
    assert_eq!(plane(&uif(207)), Plane::first(384, 79872));
    assert_eq!(plane(&uif(209)), Plane::first(384, 98304));
    assert_eq!(plane(&uif(257)), Plane::first(384, 116736));
}

#[test]
fn uif_rows_on_the_page_cache_boundary_are_not_padded() {
    let request = request("AR24", &[DRM_FORMAT_MOD_BROADCOM_UIF], 64, 256);
    assert_eq!(plane(&request), Plane::first(256, 65536));
}

#[test]
fn linear_stride_is_not_aligned() {
    let request = request("AR24", &[LINEAR], 65, 17);
    assert_eq!(plane(&request), Plane::first(260, 4420));
}

#[test]
fn uif_is_preferred() {
    let modifiers = [LINEAR, DRM_FORMAT_MOD_BROADCOM_UIF];
    let request = request("AR24", &modifiers, 64, 64);
    assert_eq!(chosen(&request), Some(DRM_FORMAT_MOD_BROADCOM_UIF));
}

#[test]
fn scanout_and_implicit_interop_are_linear() {
    let modifiers = [DRM_FORMAT_MOD_BROADCOM_UIF, LINEAR];
    let base = || request("AR24", &modifiers, 64, 64);
    assert_eq!(chosen(&base().scanout()), Some(LINEAR));
    assert_eq!(chosen(&base().implicit_interop()), Some(LINEAR));
}

#[test]
fn allocation_adds_the_tfu_readahead() {
    let request = request("AR24", &[LINEAR], 1920, 8);
    let expected = Call::V3dCreateBo {
        size: 65536,
        flags: 0,
    };
    assert_eq!(create(&request), expected);
}

#[test]
fn readahead_fills_the_last_page() {
    let request = request("AR24", &[LINEAR], 1008, 1);
    let expected = Call::V3dCreateBo {
        size: 4096,
        flags: 0,
    };
    assert_eq!(create(&request), expected);
}

#[test]
fn largest_buffer_has_a_layout() {
    let request = request("AB8F", &[LINEAR], MAX_EXTENT, MAX_EXTENT);
    assert_eq!(plane(&request), Plane::first(65536, 268435456));
}

#[test]
fn smallest_buffer_is_one_byte() {
    let request = request("R8  ", &[LINEAR], 1, 1);
    assert_eq!(plane(&request), Plane::first(1, 1));
}
