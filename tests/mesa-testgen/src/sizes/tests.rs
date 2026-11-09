use super::*;

fn size(width: u32, height: u32) -> Size {
    Size { width, height }
}

fn period(length: u32, step: u32) -> Period {
    Period { length, step }
}

#[test]
fn edges_surround_three_multiples_of_each_period() {
    let found: Vec<u32> = edges(&[period(64, 2)]).into_iter().collect();
    let expected = [
        1,
        2,
        62,
        63,
        64,
        65,
        126,
        127,
        128,
        129,
        190,
        191,
        192,
        193,
        MAX_EXTENT - 1,
        MAX_EXTENT,
    ];
    assert_eq!(found, expected);
}

#[test]
fn edges_outside_the_extent_are_dropped() {
    let found = edges(&[period(MAX_EXTENT, 1), period(1, 1)]);
    assert_eq!(found.first(), Some(&1));
    assert_eq!(found.last(), Some(&MAX_EXTENT));
    let clamped: Vec<u32> = clamp(BTreeSet::from([-1, 0, 1, 16384, 16385]))
        .into_iter()
        .collect();
    assert_eq!(clamped, [1, MAX_EXTENT]);
}

#[test]
fn the_largest_period_is_found() {
    assert_eq!(
        largest(&[period(8, 1), period(64, 2), period(16, 1)]),
        Some(64)
    );
    assert_eq!(largest(&[]), None);
}

#[test]
fn the_base_set_varies_one_dimension_at_a_time() {
    let widths = BTreeSet::from([1, 5]);
    let heights = BTreeSet::from([2]);
    let sizes: Vec<Size> = base_set(&widths, &heights, Some(64), Some(32))
        .into_iter()
        .collect();
    let expected = [
        size(1, 1),
        size(1, 2),
        size(1, 32),
        size(5, 1),
        size(5, 32),
        size(64, 2),
    ];
    assert_eq!(sizes, expected);
    let sizes: Vec<Size> = base_set(&widths, &heights, None, None)
        .into_iter()
        .collect();
    assert_eq!(sizes, [size(1, 1), size(1, 2), size(5, 1)]);
}

#[test]
fn a_count_is_split_into_rows_at_the_maximal_extent() {
    let split = |columns: u32, rows: u32| Some(Split { columns, rows });
    assert_eq!(split_count(0, 8, 8), None);
    assert_eq!(split_count(3, 8, 8), split(3, 1));
    assert_eq!(split_count(2048, 8, 8), split(2048, 1));
    assert_eq!(split_count(2049, 8, 8), split(2048, 2));
    assert_eq!(split_count(1, 2 * MAX_EXTENT, 8), None);
    assert_eq!(split_count(3, MAX_EXTENT, MAX_EXTENT), None);
}

#[test]
fn tiled_sizes_follow_the_tile_in_pixels() {
    let block = Block {
        bits: 32,
        width: 1,
        height: 1,
    };
    let sizes = tiled_sizes(&block, 512, Some(8));
    for expected in [
        size(128, 1),
        size(127, 1),
        size(129, 8),
        size(512, 1),
        size(1, 8),
        size(512, 24),
        size(1, 1),
    ] {
        assert!(sizes.contains(&expected), "{expected:?}");
    }
    let untiled = tiled_sizes(&block, 256, None);
    let varies_one = |size: &Size| size.height == 1 || size.width == 1 || size.width == 256;
    assert!(untiled.iter().all(varies_one));
    assert!(untiled.contains(&size(64, 1)));
}
