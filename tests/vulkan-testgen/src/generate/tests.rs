use super::*;
use allocator_trace::vectors::Flag;
use allocator_trace::vectors::Flags;
use fake_kernel::Node;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

fn size(width: u32, height: u32) -> Size {
    Size { width, height }
}

fn device(fake: &str) -> vectors::Device {
    vectors::Device {
        driver: "i915".to_string(),
        fake: fake.to_string(),
        node: Node::Render,
    }
}

fn case(width: u32, scanout: bool, created: u64) -> vectors::Case {
    vectors::Case {
        fourcc: 0x3432_5258,
        modifier: 0x0100_0000_0000_0002,
        rejected: None,
        width,
        height: 32,
        scanout,
        flags: Flags::default(),
        layout: vec![vectors::Plane {
            offset: 0,
            stride: 4 * width as u64,
        }],
        calls: vec![
            format!("i915_gem_create size={created}").parse().unwrap(),
            "prime_handle_to_fd flags=0x80002".parse().unwrap(),
        ],
    }
}

fn record(file: &str, source: Source, fake: &str, case: vectors::Case) -> Recorded {
    Recorded {
        file: file.to_string(),
        source,
        device: device(fake),
        case,
    }
}

fn flagged(mut case: vectors::Case, flag: Flag) -> vectors::Case {
    case.flags.insert(flag);
    case
}

struct TestDirectory {
    path: std::path::PathBuf,
}

impl TestDirectory {
    fn new() -> Self {
        static NEXT: AtomicU32 = AtomicU32::new(0);
        let name = format!(
            "generate-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        );
        let path = Path::new(env!("OUT_DIR")).join(name);
        std::fs::create_dir(&path).unwrap();
        Self { path }
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path).unwrap();
    }
}

#[test]
fn a_layout_holds_the_collected_planes() {
    let planes = [
        PlaneLayout {
            offset: 0,
            stride: 64,
            size: 4096,
        },
        PlaneLayout {
            offset: 4096,
            stride: 64,
            size: 2048,
        },
    ];
    let layout: Layout = planes.into_iter().collect();
    assert_eq!(layout.planes(), planes);
    let empty: Layout = std::iter::empty().collect();
    assert!(empty.planes().is_empty());
}

#[test]
fn runs_group_equal_pairs() {
    let sizes: Vec<PairSize> = [3, 3, 0, 5, 5, 5]
        .into_iter()
        .map(|pair| PairSize {
            pair,
            size: size(1, 1),
        })
        .collect();
    let runs = pair_runs(&sizes);
    let pairs: Vec<usize> = runs.iter().map(|run| run.pair).collect();
    let ranges: Vec<Range<usize>> = runs.iter().map(|run| run.range.clone()).collect();
    assert_eq!(pairs, [3, 0, 5]);
    assert_eq!(ranges, [0..2, 2..3, 3..6]);
    assert!(pair_runs(&[]).is_empty());
}

#[test]
fn a_record_is_read_as_it_was_written() {
    for source in [Source::Gbm, Source::Vulkan] {
        let case = flagged(case(64, true, 8192), Flag::IgnoreSize);
        let written = record("intel", source, "intel:skl_gt2", case);
        assert_eq!(Recorded::parse(&written.to_text()), written);
    }
}

#[test]
fn the_source_selects_the_file() {
    let gbm = record("intel", Source::Gbm, "a", case(1, false, 1));
    let vulkan = record("intel", Source::Vulkan, "a", case(1, false, 1));
    assert_eq!(gbm.file_name(), "intel.txt.zst");
    assert_eq!(vulkan.file_name(), "intel_vulkan.txt.zst");
}

#[test]
fn a_vulkan_record_yields_to_the_gbm_record_of_the_same_request() {
    let vulkan = |width: u32| flagged(case(width, false, 4096), Flag::IgnoreSize);
    let records = vec![
        record("intel", Source::Vulkan, "a", vulkan(1)),
        record("intel", Source::Vulkan, "a", vulkan(2)),
        record("intel", Source::Vulkan, "b", vulkan(1)),
        record("amd", Source::Vulkan, "a", vulkan(1)),
        record("intel", Source::Gbm, "a", case(1, false, 4096)),
        record(
            "intel",
            Source::Gbm,
            "a",
            flagged(case(2, false, 1), Flag::Linear),
        ),
    ];
    let kept = without_gbm_duplicates(records.clone());
    let expected = [
        records[4].clone(),
        records[5].clone(),
        records[1].clone(),
        records[2].clone(),
        records[3].clone(),
    ];
    assert_eq!(kept, expected);
}

#[test]
fn equal_records_are_written_once() {
    let records = vec![
        record("intel", Source::Gbm, "a", case(1, false, 4096)),
        record("intel", Source::Gbm, "a", case(1, false, 4096)),
        record("intel", Source::Gbm, "a", case(1, true, 4096)),
        record("intel", Source::Gbm, "b", case(1, false, 4096)),
        record("intel", Source::Vulkan, "a", case(1, false, 4096)),
        record(
            "intel",
            Source::Gbm,
            "a",
            flagged(case(1, false, 4096), Flag::Linear),
        ),
    ];
    let files = group_records(records);
    let names: Vec<&str> = files.keys().map(String::as_str).collect();
    assert_eq!(names, ["intel.txt.zst", "intel_vulkan.txt.zst"]);
    let counts: Vec<usize> = files["intel.txt.zst"]
        .values()
        .map(|device| device.cases.len())
        .collect();
    assert_eq!(counts, [3, 1]);
}

#[test]
#[should_panic(expected = "records one request with two results")]
fn conflicting_records_are_refused() {
    group_records(vec![
        record("intel", Source::Gbm, "a", case(1, false, 4096)),
        record("intel", Source::Gbm, "a", case(1, false, 8192)),
    ]);
}

#[test]
fn records_are_written_as_compressed_vectors() {
    let directory = TestDirectory::new();
    let records = vec![
        record("intel", Source::Gbm, "b", case(2, false, 4096)),
        record("intel", Source::Gbm, "a", case(1, false, 4096)),
        record("intel", Source::Gbm, "a", case(1, false, 4096)),
        record("nouveau", Source::Vulkan, "a", case(1, true, 4096)),
    ];
    for threads in [1, 2] {
        assert_eq!(write_recorded(&directory.path, records.clone(), threads), 3);
        let read = |name: &str| {
            let file = File::open(directory.path.join(name)).unwrap();
            let text = String::from_utf8(zstd::decode_all(file).unwrap()).unwrap();
            vectors::parse(&text).unwrap()
        };
        let intel = read("intel.txt.zst");
        let fakes: Vec<&str> = intel
            .iter()
            .map(|device| device.device.fake.as_str())
            .collect();
        assert_eq!(fakes, ["a", "b"]);
        assert_eq!(intel[0].cases, [case(1, false, 4096)]);
        assert_eq!(intel[1].cases, [case(2, false, 4096)]);
        assert_eq!(
            read("nouveau_vulkan.txt.zst")[0].cases,
            [case(1, true, 4096)]
        );
    }
}

struct TestBackend {
    sizes: Vec<BTreeSet<Size>>,
}

impl Backend for TestBackend {
    type ClassKey = usize;

    fn pairs(&self) -> Vec<PairInfo<'_>> {
        self.sizes
            .iter()
            .enumerate()
            .map(|(index, sizes)| PairInfo {
                index: 10 + index,
                fourcc: 1,
                modifier: 100 - index as u64,
                scanout: None,
                sizes,
            })
            .collect()
    }

    fn class_key(&self, pair: usize) -> usize {
        pair
    }
}

fn layout_of(stride: u64) -> Evaluation {
    let plane = PlaneLayout {
        offset: 0,
        stride,
        size: stride,
    };
    Evaluation::Vector(std::iter::once(plane).collect())
}

fn pair_sizes(sizes: &[PairSize]) -> Vec<String> {
    sizes
        .iter()
        .map(|pair_size| format!("pair {} width {}", pair_size.pair, pair_size.size.width))
        .collect()
}

#[test]
fn a_plan_orders_the_pairs_and_asks_for_the_neighbor_sizes() {
    let backend = TestBackend {
        sizes: vec![
            BTreeSet::from([size(1, 1), size(2, 1)]),
            BTreeSet::from([size(2, 1), size(3, 1)]),
        ],
    };
    let plan = Plan::new(&backend);
    let indices: Vec<usize> = plan.pairs.iter().map(|pair| pair.index).collect();
    assert_eq!(indices, [11, 10]);
    assert_eq!(
        pair_sizes(&plan.own_sizes),
        [
            "pair 0 width 2",
            "pair 0 width 3",
            "pair 1 width 1",
            "pair 1 width 2"
        ]
    );
    assert_eq!(plan.own_ranges, [0..2, 2..4]);
    assert_eq!(plan.neighbors, [[1], [0]]);
    let own = [
        layout_of(8),
        Evaluation::Rejected,
        layout_of(4),
        layout_of(8),
    ];
    let extra_sizes = plan.extra_sizes(&own);
    assert_eq!(pair_sizes(&extra_sizes), ["pair 0 width 1"]);
    let extra = [layout_of(4)];
    let selected = select_vectors(&plan, &own, &extra_sizes, &extra);
    assert_eq!(selected.computed, 3);
    assert_eq!(selected.rejected, 1);
    assert_eq!(selected.final_pass, selected.retained.len());
    let retained = pair_sizes(&selected.retained);
    assert!(
        retained.contains(&"pair 11 width 2".to_string()),
        "{retained:?}"
    );
    assert!(
        retained.iter().any(|kept| kept.starts_with("pair 10 ")),
        "{retained:?}"
    );
}
