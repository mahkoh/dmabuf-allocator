use super::*;

fn size(width: u32, height: u32) -> Size {
    Size { width, height }
}

fn extent(width: u32, height: u32) -> vk::Extent3D {
    vk::Extent3D {
        width,
        height,
        depth: 1,
    }
}

fn unit(format_id: usize, modifier: u64, sizes: &[Size]) -> DeviceUnit {
    DeviceUnit {
        format_id,
        modifier,
        plane_count: 2,
        max_extent: extent(4096, 2048),
        sizes: sizes.iter().copied().collect(),
    }
}

fn description(node: u32, rank: u64, units: Vec<DeviceUnit>) -> DeviceDescription {
    DeviceDescription { node, rank, units }
}

#[test]
fn a_description_is_read_as_it_was_written() {
    let written = description(
        7,
        (120 << 32) | 3,
        vec![
            unit(0, 0x0100_0000_0000_0002, &[size(1, 1), size(256, 64)]),
            unit(5, 0, &[]),
        ],
    );
    let text = write_description(&written);
    assert_eq!(
        text,
        "device 7 515396075523\n\
         unit 0 0x100000000000002 2 4096 2048 1 1x1 256x64\n\
         unit 5 0x0 2 4096 2048 1\n"
    );
    let parsed = parse_descriptions(&format!("{text}{text}"));
    assert_eq!(parsed.len(), 2);
    for read in &parsed {
        assert_eq!(write_description(read), text);
    }
}

#[test]
#[should_panic(expected = "unknown description line")]
fn an_unknown_description_line_is_refused() {
    parse_descriptions("gbm 1 0x0\n");
}

#[test]
#[should_panic(expected = "unit before the first device")]
fn a_unit_needs_a_device() {
    parse_descriptions("unit 0 0x0 1 1 1 1\n");
}

#[test]
fn merging_unites_the_sizes_and_keeps_the_highest_rank() {
    let descriptions = vec![
        description(9, 1, vec![unit(0, 5, &[size(3, 3)])]),
        description(
            4,
            8,
            vec![unit(0, 5, &[size(1, 1)]), unit(1, 5, &[size(2, 2)])],
        ),
        description(6, 2, vec![unit(0, 5, &[size(1, 1), size(4, 4)])]),
    ];
    let described = merge(descriptions);
    assert_eq!(described.pairs.len(), 2);
    let first = &described.pairs[0];
    assert_eq!(first.format_id, 0);
    assert_eq!(first.modifier, 5);
    assert_eq!(first.nodes, [4, 6, 9]);
    assert_eq!(first.latest.node, 4);
    let sizes: Vec<Size> = first.sizes.iter().copied().collect();
    assert_eq!(sizes, [size(1, 1), size(3, 3), size(4, 4)]);
    assert_eq!(described.pairs[1].format_id, 1);
    assert_eq!(described.pairs[1].nodes, [4]);
}

#[test]
fn the_plan_lists_one_pair_per_format_and_modifier() {
    let described = merge(vec![description(
        2,
        0,
        vec![unit(0, 5, &[size(1, 1)]), unit(1, 5, &[])],
    )]);
    let plan = VulkanPlan {
        described: &described,
    };
    let pairs = generate::Backend::pairs(&plan);
    assert_eq!(pairs.len(), 2);
    for (index, pair) in pairs.iter().enumerate() {
        assert_eq!(pair.index, index);
        assert_eq!(pair.fourcc, fourcc_code(FORMATS[index].fourcc));
        assert_eq!(pair.modifier, 5);
        assert_eq!(pair.scanout, None);
    }
    let first = generate::Backend::class_key(&plan, 0);
    let second = generate::Backend::class_key(&plan, 1);
    assert_eq!(first.node, 2);
    assert_eq!(first.modifier, 5);
    assert!(first != second);
}

#[test]
fn an_extent_must_fit_the_device_and_the_blocks() {
    let subsampled = FORMATS
        .iter()
        .find(|format| largest_block(format.planes) == size(2, 2))
        .unwrap();
    let max_extent = extent(64, 32);
    assert!(extent_valid(subsampled, max_extent, size(64, 32)));
    assert!(!extent_valid(subsampled, max_extent, size(63, 32)));
    assert!(!extent_valid(subsampled, max_extent, size(64, 31)));
    assert!(!extent_valid(subsampled, max_extent, size(66, 32)));
    assert!(!extent_valid(subsampled, max_extent, size(64, 34)));
}

#[test]
fn export_errors_name_the_failing_call() {
    let error = ExportError::AllocateMemory(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY);
    assert!(error.to_string().starts_with("vkAllocateMemory: "));
    assert_eq!(
        ExportError::NoMemoryType.to_string(),
        "no device-local memory type"
    );
}
