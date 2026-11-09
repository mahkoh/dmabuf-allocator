use super::*;
use std::collections::BTreeSet;
use vulkan_testgen::distribute::Job as _;
use vulkan_testgen::generate::Layout;
use vulkan_testgen::generate::PairInfo;
use vulkan_testgen::sizes::Size;

fn pair_size(pair: usize, width: u32) -> PairSize {
    PairSize {
        pair,
        size: Size { width, height: 1 },
    }
}

fn described(text: &str) -> Described {
    vulkan::merge(vulkan::parse_descriptions(text))
}

fn rejected_or_empty(index: usize) -> Evaluation {
    match index % 2 {
        0 => Evaluation::Rejected,
        _ => Evaluation::Vector(std::iter::empty().collect::<Layout>()),
    }
}

#[test]
fn lists_are_interleaved_item_by_item() {
    let lists = [vec![1, 2, 3], vec![], vec![4], vec![5, 6]];
    assert_eq!(interleave(&lists), [1, 4, 5, 2, 6, 3]);
    assert!(interleave::<u32>(&[]).is_empty());
}

#[test]
fn the_pair_order_alternates_between_the_pairs() {
    let sizes = [
        pair_size(0, 1),
        pair_size(0, 2),
        pair_size(0, 3),
        pair_size(1, 1),
        pair_size(2, 1),
        pair_size(2, 2),
    ];
    assert_eq!(pair_order(&sizes), [0, 3, 4, 1, 5, 2]);
}

#[test]
fn evaluations_return_to_the_position_of_their_size() {
    let sizes = [BTreeSet::new(), BTreeSet::new()];
    let pair = |index: usize| PairInfo {
        index,
        fourcc: 0,
        modifier: 0,
        scanout: None,
        sizes: &sizes[0],
    };
    let plan = |backend: Backend, first: usize| BackendPlan {
        backend,
        plan: Plan {
            pairs: vec![pair(first), pair(first + 1)],
            own_sizes: Vec::new(),
            own_ranges: Vec::new(),
            classes: Vec::new(),
            neighbors: Vec::new(),
        },
    };
    let plans = [plan(Backend::ALL[0], 10), plan(Backend::ALL[3], 20)];
    let first = [pair_size(0, 1), pair_size(0, 2), pair_size(1, 3)];
    let second = [pair_size(1, 4)];
    let sizes: [&[PairSize]; 2] = [&first, &second];
    let EvaluationJobs { jobs, origins } = evaluation_jobs(&plans, &sizes);
    let expected = [
        Job::Evaluate {
            backend: Backend::ALL[0],
            pair: 10,
            size: first[0].size,
        },
        Job::Evaluate {
            backend: Backend::ALL[3],
            pair: 21,
            size: second[0].size,
        },
        Job::Evaluate {
            backend: Backend::ALL[0],
            pair: 11,
            size: first[2].size,
        },
        Job::Evaluate {
            backend: Backend::ALL[0],
            pair: 10,
            size: first[1].size,
        },
    ];
    assert_eq!(jobs, expected);
    let evaluations: Vec<Evaluation> = (0..jobs.len()).map(rejected_or_empty).collect();
    let scattered = scatter(evaluations, &origins, &sizes);
    assert_eq!(
        scattered,
        [
            vec![
                rejected_or_empty(0),
                rejected_or_empty(3),
                rejected_or_empty(2)
            ],
            vec![rejected_or_empty(1)],
        ]
    );
}

#[test]
fn the_registry_maps_names_to_node_ranges() {
    let registry = Registry::parse("amd 0 70\ndumb 70 71\nvc4 71 71\n");
    assert_eq!(registry.nodes("amd"), 0..70);
    assert_eq!(registry.nodes("dumb"), 70..71);
    assert!(registry.nodes("vc4").is_empty());
}

#[test]
#[should_panic(expected = "the registry has no intel nodes")]
fn a_missing_registry_entry_is_refused() {
    Registry::parse("amd 0 70\n").nodes("intel");
}

#[test]
#[should_panic(expected = "invalid registry line")]
fn a_malformed_registry_line_is_refused() {
    Registry::parse("amd 0\n");
}

#[test]
fn the_work_directory_is_private_and_removed_with_its_content() {
    let parent = Path::new(env!("OUT_DIR")).join(format!("pool-test-{}", std::process::id()));
    let first = WorkDirectory::create_in(&parent);
    let second = WorkDirectory::create_in(&parent);
    assert!(first.path != second.path);
    let mode = std::fs::metadata(&first.path).unwrap().permissions();
    assert_eq!(
        std::os::unix::fs::PermissionsExt::mode(&mode) & 0o777,
        WORK_DIRECTORY_MODE
    );
    std::fs::write(first.path.join("file"), "content").unwrap();
    let path = first.path.clone();
    drop(first);
    assert!(!path.exists());
    assert!(second.path.exists());
    drop(second);
    std::fs::remove_dir(&parent).unwrap();
}

#[test]
fn coverage_jobs_pair_each_node_with_its_pairs() {
    let groups = [
        DescribedGroup {
            group: Group::Intel,
            nodes: vec![5, 6],
            described: described(
                "device 5 0\nunit 0 0x1 1 64 64 1\nunit 1 0x1 1 64 64 1\n\
                 device 6 1\nunit 1 0x1 1 64 64 1\n",
            ),
        },
        DescribedGroup {
            group: Group::Amd,
            nodes: vec![2],
            described: described("device 2 0\nunit 3 0x0 1 64 64 1\n"),
        },
    ];
    let coverage = |group: Group, pair: u32, node: u32| Job::Coverage { group, pair, node };
    let expected = [
        coverage(Group::Intel, 0, 5),
        coverage(Group::Intel, 1, 5),
        coverage(Group::Intel, 1, 6),
        coverage(Group::Amd, 0, 2),
    ];
    assert_eq!(coverage_jobs(&groups), expected);
    let implicit = implicit_jobs(&groups);
    assert_eq!(implicit.len(), ALL_FORMATS.len());
    assert_eq!(implicit[1], Job::Implicit { node: 2, format: 1 });
}

#[test]
fn simple_jobs_cover_every_node_and_format_of_each_driver() {
    let mut text = String::new();
    for (index, driver) in simple::DRIVERS.iter().enumerate() {
        text.push_str(&format!(
            "{} {} {}\n",
            driver.name,
            10 * index,
            10 * index + index % 3
        ));
    }
    let jobs = simple_jobs(&Registry::parse(&text));
    let nodes: usize = (0..simple::DRIVERS.len()).map(|index| index % 3).sum();
    assert_eq!(jobs.len(), nodes * ALL_FORMATS.len());
    let first = Job::Simple {
        driver: 1,
        node: 10,
        format: 0,
    };
    assert_eq!(jobs[0], first);
    for job in jobs {
        assert_eq!(Job::decode(job.encode()), job);
    }
}

#[test]
fn the_worker_environment_names_each_variable_once() {
    let names: BTreeSet<&str> = WORKER_ENVIRONMENT
        .iter()
        .map(|variable| variable.name)
        .collect();
    assert_eq!(names.len(), WORKER_ENVIRONMENT.len());
    assert!(!names.contains("MESA_LOADER_DRIVER_OVERRIDE"));
}
