use std::collections::BTreeSet;
use std::ops::RangeInclusive;
use vulkan_testgen::formats::Block;
use vulkan_testgen::sizes::MAX_EXTENT;
use vulkan_testgen::sizes::Size;

#[cfg(test)]
mod tests;

const PERIOD_MULTIPLES: RangeInclusive<i64> = 1..=3;
const TILE_COUNTS: [u32; 2] = [1, 4];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Period {
    pub length: u32,
    pub step: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Split {
    pub columns: u32,
    pub rows: u32,
}

pub fn fixed_edges() -> BTreeSet<i64> {
    BTreeSet::from([1, 2, MAX_EXTENT as i64 - 1, MAX_EXTENT as i64])
}

pub fn edges(periods: &[Period]) -> BTreeSet<u32> {
    let mut out = fixed_edges();
    for period in periods {
        for multiple in PERIOD_MULTIPLES {
            for offset in [-(period.step as i64), -1, 0, 1] {
                out.insert(multiple * period.length as i64 + offset);
            }
        }
    }
    clamp(out)
}

pub fn clamp(values: BTreeSet<i64>) -> BTreeSet<u32> {
    values
        .into_iter()
        .filter(|value| (1..=MAX_EXTENT as i64).contains(value))
        .map(|value| value as u32)
        .collect()
}

pub fn largest(periods: &[Period]) -> Option<u32> {
    periods.iter().map(|period| period.length).max()
}

pub fn base_set(
    widths: &BTreeSet<u32>,
    heights: &BTreeSet<u32>,
    largest_x: Option<u32>,
    largest_y: Option<u32>,
) -> BTreeSet<Size> {
    let mut sizes = BTreeSet::new();
    for &width in widths {
        for height in std::iter::once(1).chain(largest_y) {
            sizes.insert(Size { width, height });
        }
    }
    for width in std::iter::once(1).chain(largest_x) {
        for &height in heights {
            sizes.insert(Size { width, height });
        }
    }
    sizes
}

pub fn split_count(count: u32, unit_x: u32, unit_y: u32) -> Option<Split> {
    if count == 0 {
        return None;
    }
    if count * unit_x <= MAX_EXTENT {
        return Some(Split {
            columns: count,
            rows: 1,
        });
    }
    let columns = MAX_EXTENT / unit_x;
    if columns == 0 {
        return None;
    }
    let rows = count.div_ceil(columns);
    if rows * unit_y > MAX_EXTENT {
        return None;
    }
    Some(Split { columns, rows })
}

pub fn tiled_sizes(
    block: &Block,
    tile_width_bytes: u32,
    tile_height: Option<u32>,
) -> BTreeSet<Size> {
    let bytes = block.bytes();
    let x = TILE_COUNTS.map(|tiles| Period {
        length: (tiles * tile_width_bytes / bytes).max(1) * block.width,
        step: block.width,
    });
    let y: Vec<Period> = tile_height
        .map(|rows| Period {
            length: rows * block.height,
            step: block.height,
        })
        .into_iter()
        .collect();
    base_set(&edges(&x), &edges(&y), largest(&x), largest(&y))
}
