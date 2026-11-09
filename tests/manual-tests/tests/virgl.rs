use manual_tests::AllocatorError;
use manual_tests::Call;
use manual_tests::Device;
use manual_tests::DriverName;
use manual_tests::Fault;
use manual_tests::LINEAR;
use manual_tests::Plane;
use manual_tests::Request;
use manual_tests::VirtioGpuError;
use manual_tests::request;
use manual_tests::uapi::virtgpu::DRM_IOCTL_VIRTGPU_CONTEXT_INIT;
use manual_tests::uapi::virtgpu::DRM_IOCTL_VIRTGPU_GET_CAPS;
use manual_tests::uapi::virtgpu::DRM_IOCTL_VIRTGPU_GETPARAM;

const VIRGL: &str = "virtio_gpu:host=virgl";
const VIRGL_CONTEXT_INIT: &str = "virtio_gpu:host=virgl_context_init";
const VIRGL1_CONTEXT_INIT: &str = "virtio_gpu:host=virgl1_context_init";
const VIRGL2_CONTEXT_INIT: &str = "virtio_gpu:host=virgl2_context_init";
const VENUS: &str = "virtio_gpu:host=venus";
const VIRGL_GLES: &str = "virtio_gpu:host=virgl_gles";
const VIRGL_GLES_NO_TWEAK: &str = "virtio_gpu:host=virgl_gles_no_tweak";
const VIRGL_FORMAT_MODIFIER: &str = "virtio_gpu:host=virgl_format_modifier";
const VIRTIO_2D: &str = "virtio_gpu:host=2d";

const MAX_EXTENT: u32 = 16384;
const VIRGL_GLES_MAX_EXTENT: u32 = 8192;
const VIRGL2_CAPSET_QUERY: usize = 2;
const CAPSET_QUERY_FIX_QUERY: usize = 3;
const PIPE_TEXTURE_2D: u32 = 2;
const VIRGL_BIND_RENDER_TARGET: u32 = 1 << 1;
const VIRGL_BIND_SAMPLER_VIEW: u32 = 1 << 3;
const VIRGL_BIND_SCANOUT: u32 = 1 << 18;
const VIRGL_BIND_SHARED: u32 = 1 << 20;
const VIRGL_BIND_PREFER_EMULATED_BGRA: u32 = 1 << 21;
const VIRGL_BIND_LINEAR: u32 = 1 << 22;
const VIRGL_FORMAT_B8G8R8A8_UNORM: u32 = 1;
const SHARED_RENDERABLE: u32 =
    VIRGL_BIND_SHARED | VIRGL_BIND_LINEAR | VIRGL_BIND_RENDER_TARGET | VIRGL_BIND_SAMPLER_VIEW;

struct ResourceCreate {
    width: u32,
    height: u32,
    bind: u32,
    size: u32,
    stride: u32,
}

impl ResourceCreate {
    fn call(&self) -> Call {
        Call::VirtgpuResourceCreate {
            target: PIPE_TEXTURE_2D,
            format: VIRGL_FORMAT_B8G8R8A8_UNORM,
            bind: self.bind,
            width: self.width,
            height: self.height,
            depth: 1,
            array_size: 1,
            last_level: 0,
            nr_samples: 0,
            flags: 0,
            size: self.size,
            stride: self.stride,
        }
    }
}

fn linear(fourcc: &str, width: u32, height: u32) -> Request {
    request(fourcc, &[LINEAR], width, height)
}

fn chosen(width: u32, height: u32) -> Option<u64> {
    Device::new(VIRGL).chosen(&linear("AR24", width, height))
}

fn create(device: &Device, request: &Request) -> Call {
    let buffer = device.allocate(request).unwrap();
    buffer.only(|call| matches!(call, Call::VirtgpuResourceCreate { .. }).then(|| call.clone()))
}

fn bind(device: &Device, request: &Request) -> u32 {
    let Call::VirtgpuResourceCreate { bind, .. } = create(device, request) else {
        unreachable!();
    };
    bind
}

fn driver(id: &str) -> DriverName {
    Device::builder(id).open().unwrap().driver()
}

fn driver_with_failed_context_init(id: &str, errno: i32) -> DriverName {
    Device::builder(id)
        .fail(Fault::new(DRM_IOCTL_VIRTGPU_CONTEXT_INIT).errno(errno))
        .open()
        .unwrap()
        .driver()
}

#[test]
fn zero_extents_have_no_layout() {
    assert_eq!(chosen(0, 64), None);
    assert_eq!(chosen(64, 0), None);
}

#[test]
fn extents_up_to_16384_have_a_layout() {
    assert_eq!(chosen(MAX_EXTENT, 1), Some(LINEAR));
    assert_eq!(chosen(1, MAX_EXTENT), Some(LINEAR));
}

#[test]
fn extents_above_16384_have_no_layout() {
    assert_eq!(chosen(MAX_EXTENT + 1, 1), None);
    assert_eq!(chosen(1, MAX_EXTENT + 1), None);
}

#[test]
fn extents_are_limited_by_the_host() {
    let device = Device::new(VIRGL_GLES);
    let at_limit = linear("AR24", VIRGL_GLES_MAX_EXTENT, 1);
    let above_limit = linear("AR24", VIRGL_GLES_MAX_EXTENT + 1, 1);
    assert!(device.layout(&at_limit).is_some());
    assert!(device.layout(&above_limit).is_none());
}

#[test]
fn v1_caps_allow_16384() {
    let device = Device::builder(VIRGL)
        .fail(Fault::new(DRM_IOCTL_VIRTGPU_GETPARAM).nth(CAPSET_QUERY_FIX_QUERY))
        .open()
        .unwrap();
    assert!(device.layout(&linear("AR24", MAX_EXTENT, 1)).is_some());
    assert!(device.layout(&linear("AR24", MAX_EXTENT + 1, 1)).is_none());
}

#[test]
fn resources_stay_below_4_gib() {
    let device = Device::new(VIRGL);
    assert!(device.layout(&linear("AB8F", MAX_EXTENT, 16320)).is_some());
    assert!(device.layout(&linear("AB8F", MAX_EXTENT, 16321)).is_none());
}

#[test]
fn layout_is_tightly_packed() {
    let layout = Device::new(VIRGL).layout(&linear("AR24", 1920, 1)).unwrap();
    assert_eq!(manual_tests::planes(&layout), [Plane::first(7680, 7680)]);
}

#[test]
fn layout_size_covers_every_row() {
    let layout = Device::new(VIRGL).layout(&linear("AR24", 1, 65)).unwrap();
    assert_eq!(manual_tests::planes(&layout), [Plane::first(4, 260)]);
}

#[test]
fn resource_size_aligns_the_stride_to_1024_and_the_height_to_64() {
    let device = Device::new(VIRGL);
    let wide = ResourceCreate {
        width: 1920,
        height: 1,
        bind: SHARED_RENDERABLE,
        size: 524288,
        stride: 7680,
    };
    let tall = ResourceCreate {
        width: 1,
        height: 65,
        bind: SHARED_RENDERABLE,
        size: 131072,
        stride: 4,
    };
    assert_eq!(create(&device, &linear("AR24", 1920, 1)), wide.call());
    assert_eq!(create(&device, &linear("AR24", 1, 65)), tall.call());
}

#[test]
fn scanout_adds_the_scanout_bind() {
    let device = Device::new(VIRGL);
    let scanout = ResourceCreate {
        width: 1,
        height: 1,
        bind: SHARED_RENDERABLE | VIRGL_BIND_SCANOUT,
        size: 65536,
        stride: 4,
    };
    assert_eq!(
        create(&device, &linear("AR24", 1, 1).scanout()),
        scanout.call()
    );
}

#[test]
fn formats_the_host_cannot_sample_have_no_layout() {
    let device = Device::new(VIRGL);
    assert!(device.layout(&linear("RG88", 64, 64)).is_none());
}

#[test]
fn formats_with_wide_blocks_have_no_layout() {
    let device = Device::new(VIRGL_GLES);
    assert!(device.layout(&linear("YUYV", 64, 64)).is_none());
}

#[test]
fn three_channel_float_formats_have_no_layout() {
    let device = Device::new(VIRGL);
    assert!(device.layout(&linear("BGRF", 64, 64)).is_none());
}

#[test]
fn hosts_that_choose_modifiers_fall_back_to_dumb() {
    assert_eq!(driver(VIRGL_FORMAT_MODIFIER), DriverName::Dumb);
}

#[test]
fn caps_fall_back_to_virgl_v1() {
    let device = Device::builder(VIRGL)
        .fail(
            Fault::new(DRM_IOCTL_VIRTGPU_GET_CAPS)
                .nth(VIRGL2_CAPSET_QUERY)
                .errno(libc::EINVAL),
        )
        .open()
        .unwrap();
    assert_eq!(device.driver(), DriverName::Virgl);
}

#[test]
fn failed_caps_query_uses_zeroed_caps() {
    let device = Device::builder(VIRGL)
        .fail(
            Fault::new(DRM_IOCTL_VIRTGPU_GET_CAPS)
                .nth(VIRGL2_CAPSET_QUERY)
                .errno(libc::ENOMEM),
        )
        .open()
        .unwrap();
    assert_eq!(device.driver(), DriverName::Virgl);
    assert!(device.layout(&linear("AR24", 64, 64)).is_none());
}

#[test]
fn failed_3d_query_falls_back_to_dumb() {
    let device = Device::builder(VIRGL)
        .fail(Fault::new(DRM_IOCTL_VIRTGPU_GETPARAM))
        .open()
        .unwrap();
    assert_eq!(device.driver(), DriverName::Dumb);
}

#[test]
fn hosts_without_3d_fall_back_to_dumb() {
    assert_eq!(driver(VIRTIO_2D), DriverName::Dumb);
}

#[test]
fn context_init_uses_any_virgl_capset() {
    assert_eq!(driver(VIRGL_CONTEXT_INIT), DriverName::Virgl);
    assert_eq!(driver(VIRGL1_CONTEXT_INIT), DriverName::Virgl);
    assert_eq!(driver(VIRGL2_CONTEXT_INIT), DriverName::Virgl);
}

#[test]
fn render_node_errors_fail_the_open() {
    let error = Device::builder(VIRGL)
        .fail_render_node(libc::EACCES)
        .open()
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::VirtioGpuOpen(VirtioGpuError::RenderNode(errno)) if errno.0 == libc::EACCES
    ));
}

#[test]
fn context_init_without_virgl_capsets_falls_back_to_dumb() {
    assert_eq!(driver(VENUS), DriverName::Dumb);
    assert_eq!(
        driver_with_failed_context_init(VENUS, libc::EEXIST),
        DriverName::Dumb
    );
}

#[test]
fn existing_context_is_used() {
    assert_eq!(
        driver_with_failed_context_init(VIRGL_CONTEXT_INIT, libc::EEXIST),
        DriverName::Virgl
    );
}

#[test]
fn failed_context_init_falls_back_to_dumb() {
    assert_eq!(
        driver_with_failed_context_init(VIRGL_CONTEXT_INIT, libc::EINVAL),
        DriverName::Dumb
    );
}

#[test]
fn emulated_bgra_is_preferred_when_the_host_cannot_render_srgb_bgra() {
    let device = Device::new(VIRGL_GLES);
    for fourcc in ["AR24", "XR24"] {
        let bind = bind(&device, &linear(fourcc, 64, 64));
        assert_ne!(bind & VIRGL_BIND_PREFER_EMULATED_BGRA, 0, "{fourcc}");
    }
}

#[test]
fn emulated_bgra_requires_tweak_support() {
    let device = Device::new(VIRGL_GLES_NO_TWEAK);
    let bind = bind(&device, &linear("AR24", 64, 64));
    assert_eq!(bind & VIRGL_BIND_PREFER_EMULATED_BGRA, 0);
}

#[test]
fn bgra_emulation_does_not_add_formats() {
    let device = Device::new(VIRGL_GLES);
    assert!(device.layout(&linear("RG88", 64, 64)).is_none());
}

#[test]
fn formats_the_host_cannot_render_are_not_render_targets() {
    let device = Device::new(VIRGL_GLES);
    let bind = bind(&device, &linear("AB24", 64, 64));
    assert_eq!(bind & VIRGL_BIND_RENDER_TARGET, 0);
    assert_ne!(bind & VIRGL_BIND_SAMPLER_VIEW, 0);
}
