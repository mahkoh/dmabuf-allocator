use manual_tests::AllocatorError;
use manual_tests::Call;
use manual_tests::Device;
use manual_tests::DriverName;
use manual_tests::DumbError;
use manual_tests::INVALID;
use manual_tests::LINEAR;
use manual_tests::Plane;
use manual_tests::Request;
use manual_tests::request;

const VKMS: &str = "vkms";

const MAX_EXTENT: u32 = 16384;
const AR24_BYTES: u64 = 4;

#[derive(Debug, PartialEq, Eq)]
struct DumbCreate {
    width: u32,
    height: u32,
    bpp: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct DumbBuffer {
    modifier: u64,
    planes: Vec<Plane>,
    create: DumbCreate,
}

fn allocate(request: &Request) -> DumbBuffer {
    let buffer = Device::new(VKMS).allocate(request).unwrap();
    let create = buffer.only(|call| match *call {
        Call::ModeCreateDumb { width, height, bpp } => Some(DumbCreate { width, height, bpp }),
        _ => None,
    });
    DumbBuffer {
        modifier: buffer.modifier(),
        planes: buffer.planes(),
        create,
    }
}

fn error(request: &Request) -> DumbError {
    match Device::new(VKMS).allocate(request).err() {
        Some(AllocatorError::DumbAllocate(error)) => error,
        Some(error) => panic!("not a dumb allocation error: {error:?}"),
        None => panic!("the allocation succeeded"),
    }
}

#[test]
fn driver_is_dumb() {
    assert_eq!(Device::new(VKMS).driver(), DriverName::Dumb);
}

#[test]
fn extents_are_aligned_to_64_pixels() {
    let expected = DumbBuffer {
        modifier: LINEAR,
        planes: vec![Plane::first(512, 32768)],
        create: DumbCreate {
            width: 128,
            height: 64,
            bpp: 32,
        },
    };
    assert_eq!(allocate(&request("AR24", &[LINEAR], 65, 3)), expected);
}

#[test]
fn implicit_modifier_is_refused() {
    let error = error(&request("AR24", &[INVALID], 65, 3));
    assert!(matches!(error, DumbError::Modifier));
}

#[test]
fn one_byte_pixels_use_8_bpp() {
    let expected = DumbBuffer {
        modifier: LINEAR,
        planes: vec![Plane::first(128, 8192)],
        create: DumbCreate {
            width: 128,
            height: 64,
            bpp: 8,
        },
    };
    assert_eq!(allocate(&request("R8  ", &[LINEAR], 65, 3)), expected);
}

#[test]
fn extents_up_to_16384_are_allocated() {
    let wide = allocate(&request("AR24", &[LINEAR], MAX_EXTENT, 1));
    let tall = allocate(&request("AR24", &[LINEAR], 1, MAX_EXTENT));
    let wide_create = DumbCreate {
        width: MAX_EXTENT,
        height: 64,
        bpp: 32,
    };
    let tall_create = DumbCreate {
        width: 64,
        height: MAX_EXTENT,
        bpp: 32,
    };
    assert_eq!(wide.create, wide_create);
    assert_eq!(wide.planes[0].stride, u64::from(MAX_EXTENT) * AR24_BYTES);
    assert_eq!(tall.create, tall_create);
    assert_eq!(tall.planes[0].stride, 64 * AR24_BYTES);
}

#[test]
fn extents_above_16384_are_refused() {
    let wide = error(&request("AR24", &[LINEAR], MAX_EXTENT + 1, 1));
    let tall = error(&request("AR24", &[LINEAR], 1, MAX_EXTENT + 1));
    assert!(matches!(wide, DumbError::Size));
    assert!(matches!(tall, DumbError::Size));
}

#[test]
fn zero_extents_are_refused() {
    let narrow = error(&request("AR24", &[LINEAR], 0, 16));
    let flat = error(&request("AR24", &[LINEAR], 16, 0));
    assert!(matches!(narrow, DumbError::Size));
    assert!(matches!(flat, DumbError::Size));
}

#[test]
fn formats_without_dumb_support_are_refused() {
    let error = error(&request("NV12", &[LINEAR], 64, 64));
    assert!(matches!(error, DumbError::Format));
}
