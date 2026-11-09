use manual_tests::Device;
use manual_tests::Fault;
use manual_tests::KernelVersion;
use manual_tests::LINEAR;
use manual_tests::request;
use manual_tests::uapi::drm::DRM_IOCTL_GEM_CLOSE;

const NAVI21: &str = "amdgpu:navi21";
const G52: &str = "panfrost:gpu=g52";

#[test]
fn missing_id_fields_get_defaults() {
    let kernel = KernelVersion::NEWEST;
    assert_eq!(
        Device::new(NAVI21).id(),
        format!("{NAVI21}:kernel={kernel}:pte_fragment_size=2097152")
    );
    assert_eq!(
        Device::new(G52).id(),
        format!("{G52}:kernel={kernel}:afbc_features=0")
    );
}

#[test]
#[should_panic(expected = "GEM handle")]
fn leaked_gem_handle_fails_the_test() {
    let device = Device::new(NAVI21);
    device.fail(Fault::new(DRM_IOCTL_GEM_CLOSE));
    let _ = device.allocate(&request("AR24", &[LINEAR], 64, 64));
}

#[test]
#[should_panic(expected = "unknown format")]
fn unknown_format_fails_the_test() {
    let device = Device::new(NAVI21);
    device.layout(&request("????", &[LINEAR], 64, 64));
}
