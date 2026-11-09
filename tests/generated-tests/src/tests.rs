use super::*;

const VECTORS: &str = "\
device driver=amdgpu fake=amdgpu:navi10:kernel=7.2:pte_fragment_size=2097152 node=render
case \"AR24\" 0x0 256x256 scanout=0 implicit_sync=0
layout 0 1024
amdgpu_gem_create size=262144 alignment=262144 domains=0x4 flags=0x84
amdgpu_gem_metadata flags=0x0 tiling_info=0x8000300000000000
prime_handle_to_fd flags=0x80002
";

struct Loaded {
    device: Device,
    case: Case,
}

fn load() -> Loaded {
    let mut devices = vectors::parse(VECTORS).unwrap();
    let cases = devices.pop().unwrap();
    Loaded {
        device: Device::new(&cases.device),
        case: cases.cases.into_iter().next().unwrap(),
    }
}

fn kinds(loaded: &Loaded) -> Vec<String> {
    match check(&loaded.device, &loaded.case) {
        Ok(()) => Vec::new(),
        Err(failure) => failure.kinds,
    }
}

fn create(size: u64) -> Call {
    Call::I915GemCreate { size }
}

#[test]
fn recorded_case_matches_the_allocator() {
    let loaded = load();
    assert_eq!(loaded.device.gfx_level, Some(GfxLevel::Gfx10));
    assert!(check(&loaded.device, &loaded.case).is_ok());
}

#[test]
fn other_layouts_are_reported() {
    let mut loaded = load();
    loaded.case.layout[0].stride = 2048;
    let failure = check(&loaded.device, &loaded.case).err().unwrap();
    assert_eq!(failure.reason, "layout: expected 0 2048, computed 0 1024");
    assert_eq!(failure.kinds, ["layout: differs"]);
}

#[test]
fn other_calls_are_categorized() {
    let mut loaded = load();
    let Call::AmdgpuGemCreate { size, .. } = &mut loaded.case.calls[0] else {
        panic!("the first call creates no buffer");
    };
    *size *= 2;
    assert_eq!(kinds(&loaded), ["amdgpu_gem_create.size: smaller"]);
}

#[test]
fn unknown_fourccs_are_reported() {
    let mut loaded = load();
    loaded.case.fourcc = u32::from_le_bytes(*b"????");
    assert_eq!(kinds(&loaded), ["unknown fourcc"]);
}

#[test]
fn calls_of_one_kind_are_paired_in_order() {
    let export = Call::PrimeHandleToFd { flags: 2 };
    let expected = [create(1), create(2), export.clone()];
    let got = [create(1), create(3), create(4)];
    let categories = categorize(&expected, &got);
    let listed = [
        "i915_gem_create.size: larger",
        "prime_handle_to_fd: missing",
        "i915_gem_create: extra",
    ];
    assert_eq!(categories, listed);
    assert!(categorize(&expected, &expected).is_empty());
}

#[test]
fn ignored_sizes_are_taken_from_the_allocator() {
    let mut expected = [create(1), Call::PrimeHandleToFd { flags: 2 }, create(2)];
    let got = [create(10), create(20)];
    ignore_size(&mut expected, &got);
    assert_eq!(
        expected,
        [create(10), Call::PrimeHandleToFd { flags: 2 }, create(20)]
    );
}

#[test]
fn cpu_access_bits_are_taken_from_the_allocator() {
    let create = |flags: u64, blob_flags: u32| Call::VirtgpuAmdgpuGemNew {
        blob_mem: 0,
        blob_flags,
        blob_hints: 0,
        size: 0,
        alloc_size: 0,
        phys_alignment: 0,
        preferred_heap: 0,
        flags,
    };
    let other = 1 << 40;
    let mut expected = [create(other, VIRTGPU_BLOB_FLAG_USE_MAPPABLE)];
    let got = [create(AMDGPU_GEM_CREATE_NO_CPU_ACCESS, 0)];
    ignore_cpu_access(&mut expected, &got);
    assert_eq!(
        expected,
        [create(other | AMDGPU_GEM_CREATE_NO_CPU_ACCESS, 0)]
    );
}

#[test]
fn only_the_last_state_is_compared() {
    let tiling = |stride: u32| Call::I915GemSetTiling {
        tiling_mode: 1,
        stride,
    };
    let export = Call::PrimeHandleToFd { flags: 2 };
    let calls = vec![tiling(1), export.clone(), tiling(2), export.clone()];
    assert_eq!(last_state_only(calls), [export, tiling(2)]);
}

#[test]
fn vector_files_are_named_txt() {
    assert!(is_vector_file(Path::new("vectors/intel.txt.zst")));
    assert!(is_vector_file(Path::new("vectors/amd_handwritten.txt")));
    assert!(!is_vector_file(Path::new("vectors/.git")));
    assert!(!is_vector_file(Path::new("vectors/README.md")));
}
