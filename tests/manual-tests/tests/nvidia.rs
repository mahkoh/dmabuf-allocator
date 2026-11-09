use manual_tests::Call;
use manual_tests::Device;
use manual_tests::LINEAR;
use manual_tests::Pitch;
use manual_tests::Request;
use manual_tests::nvidia::ONE_GOB;
use manual_tests::nvidia::turing;
use manual_tests::nvidia::turing_compressed;
use manual_tests::request;
use manual_tests::uapi::nvidia_drm::NV_GEM_ALLOC_NO_SCANOUT;

const RTX_4090: &str = "nvidia-drm:pci_id=0x2684:page_kind=0x6:generation=2:sector_layout=1";

const TURING_ONE_GOB: u64 = turing(ONE_GOB);
const TURING_ONE_GOB_COMPRESSED: u64 = turing_compressed(ONE_GOB);

fn chosen(request: &Request) -> Option<u64> {
    Device::new(RTX_4090).chosen(request)
}

fn alloc(request: &Request) -> Call {
    let buffer = Device::new(RTX_4090).allocate(request).unwrap();
    buffer.only(|call| matches!(call, Call::NvidiaGemAllocNvkmsMemory { .. }).then(|| call.clone()))
}

#[test]
fn supported_format_has_a_layout() {
    let request = request("AR24", &[TURING_ONE_GOB], 64, 64);
    assert_eq!(chosen(&request), Some(TURING_ONE_GOB));
}

#[test]
fn unsupported_format_has_no_layout() {
    for modifier in [LINEAR, TURING_ONE_GOB] {
        assert_eq!(chosen(&request("C8  ", &[modifier], 64, 64)), None);
    }
}

#[test]
fn zero_extents_have_no_layout() {
    for modifier in [LINEAR, TURING_ONE_GOB] {
        assert_eq!(chosen(&request("AR24", &[modifier], 0, 64)), None);
        assert_eq!(chosen(&request("AR24", &[modifier], 64, 0)), None);
    }
}

#[test]
fn implicit_interop_is_linear() {
    let both = [TURING_ONE_GOB, LINEAR];
    assert_eq!(
        chosen(&request("AR24", &both, 64, 64)),
        Some(TURING_ONE_GOB)
    );
    assert_eq!(
        chosen(&request("AR24", &both, 64, 64).implicit_interop()),
        Some(LINEAR)
    );
    assert_eq!(
        chosen(&request("AR24", &[TURING_ONE_GOB], 64, 64).implicit_interop()),
        None
    );
}

#[test]
fn block_linear_modifiers_allocate_block_linear_memory() {
    let expected = Call::NvidiaGemAllocNvkmsMemory {
        block_linear: 1,
        compressible: 0,
        memory_size: 65536,
        flags: NV_GEM_ALLOC_NO_SCANOUT,
    };
    assert_eq!(alloc(&request("AR24", &[TURING_ONE_GOB], 1, 1)), expected);
}

#[test]
fn compressed_modifiers_allocate_compressible_memory() {
    let expected = Call::NvidiaGemAllocNvkmsMemory {
        block_linear: 1,
        compressible: 1,
        memory_size: 65536,
        flags: NV_GEM_ALLOC_NO_SCANOUT,
    };
    let request = request("AR24", &[TURING_ONE_GOB_COMPRESSED], 1, 1);
    assert_eq!(alloc(&request), expected);
}

#[test]
fn scanout_allocations_drop_the_no_scanout_flag() {
    let expected = Call::NvidiaGemAllocNvkmsMemory {
        block_linear: 1,
        compressible: 0,
        memory_size: 65536,
        flags: 0,
    };
    let request = request("AR24", &[TURING_ONE_GOB], 1, 1).scanout();
    assert_eq!(alloc(&request), expected);
}

#[test]
fn subsampled_formats_have_one_pitch_per_plane() {
    let request = request("NV12", &[TURING_ONE_GOB], 2, 2);
    let buffer = Device::new(RTX_4090).allocate(&request).unwrap();
    let expected = [
        Pitch {
            offset: 0,
            stride: 64,
        },
        Pitch {
            offset: 65536,
            stride: 64,
        },
    ];
    assert_eq!(buffer.pitches(), expected);
}
