use manual_tests::AllocatorError;
use manual_tests::AsahiError;
use manual_tests::Call;
use manual_tests::Device;
use manual_tests::DriverName;
use manual_tests::DrmError;
use manual_tests::Fault;
use manual_tests::INVALID;
use manual_tests::LINEAR;
use manual_tests::Plane;
use manual_tests::Request;
use manual_tests::VirtioGpuError;
use manual_tests::request;
use manual_tests::uapi::asahi::DRM_IOCTL_ASAHI_GEM_CREATE;
use manual_tests::uapi::drm::DRM_IOCTL_PRIME_HANDLE_TO_FD;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_APPLE_GPU_TILED;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED;
use manual_tests::uapi::drm_fourcc::I915_FORMAT_MOD_X_TILED;
use manual_tests::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_MAPPABLE;
use manual_tests::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
use manual_tests::uapi::virtgpu::VIRTGPU_BLOB_MEM_HOST3D;

const T6000: &str = "asahi:chip=t6000";
const VIRTIO_T6000: &str = "virtio_gpu:host=asahi_t6000";
const CHIPS: [&str; 5] = ["t8103", "t6000", "t6001", "t6002", "t8112"];

const MAX_EXTENT: u32 = 16384;
const ASAHI_PAGE_SIZE: u64 = 0x4000;
const ALL: [u64; 3] = [
    DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED,
    DRM_FORMAT_MOD_APPLE_GPU_TILED,
    LINEAR,
];

#[derive(Debug, PartialEq, Eq)]
struct AsahiBuffer {
    modifier: u64,
    stride: u64,
    size: u64,
    bo_size: u64,
}

struct GemCreate {
    size: u64,
    flags: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct AsahiCreated {
    modifier: u64,
    planes: Vec<Plane>,
    gem_size: u64,
    flags: u32,
}

fn buffer(stride: u64, size: u64, bo_size: u64, modifier: u64) -> AsahiBuffer {
    AsahiBuffer {
        modifier,
        stride,
        size,
        bo_size,
    }
}

fn allocate(request: &Request) -> AsahiBuffer {
    let buffer = Device::new(T6000).allocate(request).unwrap();
    let [plane] = buffer.planes()[..] else {
        panic!("more than one plane");
    };
    let create = buffer.only(|call| match *call {
        Call::AsahiGemCreate { size, flags } => Some(GemCreate { size, flags }),
        _ => None,
    });
    assert_eq!(plane.offset, 0);
    assert_eq!(create.flags, 0);
    AsahiBuffer {
        modifier: buffer.modifier(),
        stride: plane.stride,
        size: plane.size,
        bo_size: create.size,
    }
}

fn chosen(request: &Request) -> Option<u64> {
    Device::new(T6000).chosen(request)
}

fn created(device: &str, request: &Request) -> AsahiCreated {
    let buffer = Device::new(device).allocate(request).unwrap();
    buffer.only(|call| match *call {
        Call::AsahiGemCreate { size, flags }
        | Call::VirtgpuAsahiGemNew {
            gem_size: size,
            flags,
            ..
        } => Some(AsahiCreated {
            modifier: buffer.modifier(),
            planes: buffer.planes(),
            gem_size: size,
            flags,
        }),
        _ => None,
    })
}

#[test]
fn driver_is_asahi() {
    assert_eq!(Device::new(T6000).driver(), DriverName::Asahi);
}

#[test]
fn tiled_1_byte_pixels_fill_whole_pages() {
    let large = request("R8  ", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 200, 150);
    let small = request("R8  ", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 100, 50);
    assert_eq!(
        allocate(&large),
        buffer(200, 65536, 65536, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
    assert_eq!(
        allocate(&small),
        buffer(100, 16384, 16384, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
}

#[test]
fn tiled_2_byte_pixels_fill_whole_pages() {
    let large = request("RG16", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 300, 100);
    let small = request("RG16", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 100, 100);
    assert_eq!(
        allocate(&large),
        buffer(600, 98304, 98304, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
    assert_eq!(
        allocate(&small),
        buffer(200, 32768, 32768, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
}

#[test]
fn tiled_4_byte_pixels_fill_whole_pages() {
    let large = request("AR24", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 130, 70);
    let small = request("AR24", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 33, 33);
    assert_eq!(
        allocate(&large),
        buffer(520, 98304, 98304, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
    assert_eq!(
        allocate(&small),
        buffer(132, 16384, 16384, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
}

#[test]
fn tiled_8_byte_pixels_fill_whole_pages() {
    let large = request("AB4H", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 100, 40);
    let small = request("AB4H", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 40, 40);
    assert_eq!(
        allocate(&large),
        buffer(800, 65536, 65536, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
    assert_eq!(
        allocate(&small),
        buffer(320, 32768, 32768, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
}

#[test]
fn tiled_16_byte_pixels_fill_whole_pages() {
    let large = request("AB8F", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 50, 50);
    let small = request("AB8F", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 20, 20);
    assert_eq!(
        allocate(&large),
        buffer(800, 65536, 65536, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
    assert_eq!(
        allocate(&small),
        buffer(320, 16384, 16384, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
}

#[test]
fn compressed_buffers_are_rounded_up_to_pages() {
    let r8 = request(
        "R8  ",
        &[DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED],
        200,
        150,
    );
    let ar24 = request(
        "AR24",
        &[DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED],
        100,
        60,
    );
    let ab8f = request("AB8F", &[DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED], 40, 40);
    assert_eq!(
        allocate(&r8),
        buffer(200, 67584, 81920, DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED)
    );
    assert_eq!(
        allocate(&ar24),
        buffer(400, 33024, 49152, DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED)
    );
    assert_eq!(
        allocate(&ab8f),
        buffer(640, 65664, 81920, DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED)
    );
}

#[test]
fn linear_buffers_are_rounded_up_to_a_page() {
    let request = request("AR24", &[LINEAR], 65, 3);
    assert_eq!(allocate(&request), buffer(384, 1152, 16384, LINEAR));
}

#[test]
fn implicit_modifier_has_no_layout() {
    assert_eq!(chosen(&request("AR24", &[INVALID], 65, 3)), None);
}

#[test]
fn linear_is_preferred_over_implicit() {
    let request = request("AR24", &[INVALID, LINEAR], 65, 3);
    assert_eq!(allocate(&request), buffer(384, 1152, 16384, LINEAR));
}

#[test]
fn compression_is_preferred() {
    let plain = request("AR24", &ALL, 16, 16);
    let scanout = request("AR24", &ALL, 64, 64).scanout();
    assert_eq!(
        allocate(&plain),
        buffer(64, 1152, 16384, DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED)
    );
    assert_eq!(
        allocate(&scanout),
        buffer(256, 16512, 32768, DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED)
    );
}

#[test]
fn tiled_is_preferred_over_linear() {
    let request = request("AR24", &[DRM_FORMAT_MOD_APPLE_GPU_TILED, LINEAR], 64, 64);
    assert_eq!(
        allocate(&request),
        buffer(256, 16384, 16384, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
}

#[test]
fn compression_needs_16_pixels() {
    let narrow = request("AR24", &ALL, 15, 64);
    let short = request("AR24", &ALL, 64, 15);
    assert_eq!(
        allocate(&narrow),
        buffer(60, 16384, 16384, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
    assert_eq!(
        allocate(&short),
        buffer(256, 16384, 16384, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
}

#[test]
fn extents_up_to_16384_are_allocated() {
    let wide = request("AR24", &[LINEAR], MAX_EXTENT, 1);
    let tall = request("AR24", &[LINEAR], 1, MAX_EXTENT);
    let both = request("AR24", &[LINEAR], MAX_EXTENT, MAX_EXTENT);
    assert_eq!(allocate(&wide), buffer(65536, 65536, 65536, LINEAR));
    assert_eq!(allocate(&tall), buffer(128, 2097152, 2097152, LINEAR));
    assert_eq!(
        allocate(&both),
        buffer(65536, 1073741824, 1073741824, LINEAR)
    );
}

#[test]
fn sizes_below_4_gib_are_allocated() {
    let request = request("AB8F", &[LINEAR], MAX_EXTENT - 1, MAX_EXTENT - 1);
    assert_eq!(
        allocate(&request),
        buffer(262144, 4294705152, 4294705152, LINEAR)
    );
}

#[test]
fn size_of_4_gib_has_no_layout() {
    assert_eq!(
        chosen(&request("AB8F", &[LINEAR], MAX_EXTENT, MAX_EXTENT)),
        None
    );
}

#[test]
fn extents_above_16384_have_no_layout() {
    assert_eq!(chosen(&request("AR24", &[LINEAR], MAX_EXTENT + 1, 1)), None);
    assert_eq!(chosen(&request("AR24", &[LINEAR], 1, MAX_EXTENT + 1)), None);
}

#[test]
fn zero_extents_have_no_layout() {
    for modifiers in [&[LINEAR][..], &[DRM_FORMAT_MOD_APPLE_GPU_TILED], &[INVALID]] {
        assert_eq!(chosen(&request("AR24", modifiers, 0, 16)), None);
        assert_eq!(chosen(&request("AR24", modifiers, 16, 0)), None);
    }
}

#[test]
fn unsupported_formats_have_no_layout() {
    assert_eq!(chosen(&request("C8  ", &[LINEAR], 64, 64)), None);
    assert_eq!(chosen(&request("AB12", &[LINEAR], 64, 64)), None);
}

#[test]
fn foreign_modifier_has_no_layout() {
    assert_eq!(
        chosen(&request("AR24", &[I915_FORMAT_MOD_X_TILED], 64, 64)),
        None
    );
}

#[test]
fn implicit_interop_selects_linear() {
    let request = request("AR24", &ALL, 64, 64).implicit_interop();
    assert_eq!(chosen(&request), Some(LINEAR));
}

#[test]
fn implicit_interop_without_linear_has_no_layout() {
    let request = request(
        "AR24",
        &[
            DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED,
            DRM_FORMAT_MOD_APPLE_GPU_TILED,
        ],
        64,
        64,
    )
    .implicit_interop();
    assert_eq!(chosen(&request), None);
}

#[test]
fn layout_and_allocation_agree() {
    let device = Device::new(T6000);
    let request = request(
        "AR24",
        &[DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED],
        100,
        60,
    );
    let layout = device.layout(&request).unwrap();
    let allocation = device.allocate(&request).unwrap();
    assert_eq!(manual_tests::planes(&layout), allocation.planes());
}

#[test]
fn failed_render_node_fails_the_allocator() {
    let error = Device::builder(T6000)
        .fail_render_node(libc::EACCES)
        .open()
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::AsahiOpen(AsahiError::RenderNode(errno)) if errno.0 == libc::EACCES
    ));
}

#[test]
fn failed_gem_create_fails_the_allocation() {
    let device = Device::new(T6000);
    device.fail(Fault::new(DRM_IOCTL_ASAHI_GEM_CREATE).errno(libc::ENOMEM));
    let error = device
        .allocate(&request("AR24", &[LINEAR], 64, 64))
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::AsahiAllocate(AsahiError::GemCreate(errno)) if errno.0 == libc::ENOMEM
    ));
}

#[test]
fn failed_export_closes_the_gem_handle() {
    let device = Device::new(T6000);
    device.fail(Fault::new(DRM_IOCTL_PRIME_HANDLE_TO_FD).errno(libc::EMFILE));
    let error = device
        .allocate(&request("AR24", &[LINEAR], 64, 64))
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::AsahiAllocate(AsahiError::Export(DrmError::PrimeHandleToFd(errno)))
            if errno.0 == libc::EMFILE
    ));
}

#[test]
fn virtio_host_is_the_virtio_driver() {
    assert_eq!(
        Device::new(VIRTIO_T6000).driver(),
        DriverName::VirtioGpuAsahi
    );
}

#[test]
fn virtio_hosts_allocate_like_native_devices() {
    let requests = [
        request("AB24", &ALL, 64, 64),
        request("AB24", &ALL, 300, 200),
        request("AB24", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 17, 1),
        request("R8  ", &ALL, 1920, 1080),
        request("AB24", &[LINEAR], 65, 3),
    ];
    for chip in CHIPS {
        let virtio = format!("virtio_gpu:host=asahi_{chip}");
        let native = format!("asahi:chip={chip}");
        for (index, request) in requests.iter().enumerate() {
            assert_eq!(
                created(&virtio, request),
                created(&native, request),
                "{chip} request {index}"
            );
        }
    }
}

#[test]
fn virtio_blobs_are_mappable_and_shareable() {
    let request = request("AB24", &[LINEAR], 64, 64);
    let buffer = Device::new(VIRTIO_T6000).allocate(&request).unwrap();
    let create = Call::VirtgpuAsahiGemNew {
        blob_mem: VIRTGPU_BLOB_MEM_HOST3D,
        blob_flags: VIRTGPU_BLOB_FLAG_USE_MAPPABLE | VIRTGPU_BLOB_FLAG_USE_SHAREABLE,
        blob_hints: 0,
        size: ASAHI_PAGE_SIZE,
        gem_size: ASAHI_PAGE_SIZE,
        flags: 0,
    };
    assert_eq!(buffer.calls[0], create);
}

#[test]
fn virtio_render_node_errors_fail_the_open() {
    let error = Device::builder(VIRTIO_T6000)
        .fail_render_node(libc::EACCES)
        .open()
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::VirtioGpuOpen(VirtioGpuError::Asahi(AsahiError::RenderNode(errno)))
            if errno.0 == libc::EACCES
    ));
}

#[test]
fn tiled_sizes_step_at_tile_size_boundaries() {
    let tall = request("AR24", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 64, 129);
    let wide = request("AR24", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 129, 64);
    let narrow = request("AR24", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 32, 129);
    assert_eq!(
        allocate(&tall),
        buffer(256, 49152, 49152, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
    assert_eq!(
        allocate(&wide),
        buffer(516, 49152, 49152, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
    assert_eq!(
        allocate(&narrow),
        buffer(128, 32768, 32768, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
}

#[test]
fn compression_metadata_is_padded_to_powers_of_two() {
    let narrow = request(
        "AR24",
        &[DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED],
        250,
        64,
    );
    let large = request(
        "R8  ",
        &[DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED],
        600,
        600,
    );
    assert_eq!(
        allocate(&narrow),
        buffer(
            1000,
            66048,
            81920,
            DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED
        )
    );
    assert_eq!(
        allocate(&large),
        buffer(
            600,
            442368,
            442368,
            DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED
        )
    );
}

#[test]
fn tiled_1_byte_uses_128_pixel_tiles() {
    let request = request("R8  ", &[DRM_FORMAT_MOD_APPLE_GPU_TILED], 129, 129);
    assert_eq!(
        allocate(&request),
        buffer(129, 65536, 65536, DRM_FORMAT_MOD_APPLE_GPU_TILED)
    );
}

#[test]
fn compression_metadata_rounds_up_to_16_pixel_tiles() {
    let request = request(
        "AR24",
        &[DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED],
        17,
        512,
    );
    assert_eq!(
        allocate(&request),
        buffer(68, 66048, 81920, DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED)
    );
}
