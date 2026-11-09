use super::modifier::Class;
use super::modifier::Microtile;
use super::modifier::aspect;
use super::modifier::bank_height;
use super::modifier::bank_width;
use super::modifier::banks;
use super::modifier::block_dims;
use super::modifier::classify;
use super::modifier::dcc;
use super::modifier::dcc_pipe_align;
use super::modifier::dcc_retile;
use super::modifier::pipe_xor_bits;
use super::modifier::pipes;
use super::modifier::rb;
use crate::sizes::Period;
use crate::sizes::base_set;
use crate::sizes::clamp;
use crate::sizes::edges;
use crate::sizes::fixed_edges;
use crate::sizes::largest;
use crate::sizes::split_count;
use fake_kernel::amd::devices::GfxLevel;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;
use vulkan_testgen::formats::Block;
use vulkan_testgen::sizes::MAX_EXTENT;
use vulkan_testgen::sizes::Size;

#[cfg(test)]
mod tests;

const EXPANDED_BYTES: u32 = 12;
const EXPANDED_ELEMENTS: u32 = 3;
const LINEAR_PITCH_BYTES: u32 = 256;
const LINEAR_EXPANDED_PITCH_ELEMENTS: u32 =
    LINEAR_PITCH_BYTES / (EXPANDED_BYTES / EXPANDED_ELEMENTS);
const LINEAR_EXPANDED_PITCH_MULTIPLES: RangeInclusive<u32> = 1..=9;
const MICRO_TILE: u32 = 8;
const DISPLAY_MICRO_TILE_WIDTH: u32 = 32;
const MACRO_TILE_MULTIPLES: [u32; 3] = [1, 2, 4];
const DOUBLE_ROW_BLOCK_BYTES: u32 = 8;
const DOUBLE_ROW_BLOCK_WIDTH: u32 = 2;
const DOUBLE_ROW_BLOCK_HEIGHT: u32 = 1;
const DCC_META_BLOCK_LOG2: RangeInclusive<u32> = 12..=15;
const DCC_META_COVERAGE_LOG2: u32 = 8;
const DCC_PIPE_ALIGN_BASE_LOG2: u32 = 8;
const DCC_MAX_PIPE_XOR_BITS: u32 = 5;
const GFX8_DCC_TILE_BYTES: u32 = 1024;

fn element_log2(bytes: u32) -> u32 {
    let element = match bytes {
        EXPANDED_BYTES => bytes / EXPANDED_ELEMENTS,
        _ => bytes,
    };
    assert!(
        element.is_power_of_two(),
        "a block of {bytes} bytes has no power-of-two element"
    );
    element.ilog2()
}

fn linear_expanded_widths() -> BTreeSet<u32> {
    let mut out = fixed_edges();
    for multiple in LINEAR_EXPANDED_PITCH_MULTIPLES {
        let elements = LINEAR_EXPANDED_PITCH_ELEMENTS * multiple;
        let width = elements.div_ceil(EXPANDED_ELEMENTS) as i64;
        for offset in -1..=1 {
            out.insert(width + offset);
        }
    }
    clamp(out)
}

#[derive(Default)]
struct Periods {
    x: Vec<Period>,
    y: Vec<Period>,
}

impl Periods {
    fn push_x(&mut self, length: u32, block: &Block) {
        self.x.push(Period {
            length: length * block.width,
            step: block.width,
        });
    }

    fn push_y(&mut self, length: u32, block: &Block) {
        self.y.push(Period {
            length: length * block.height,
            step: block.height,
        });
    }
}

fn is_double_row_block(block: &Block) -> bool {
    block.bytes() == DOUBLE_ROW_BLOCK_BYTES
        && block.width == DOUBLE_ROW_BLOCK_WIDTH
        && block.height == DOUBLE_ROW_BLOCK_HEIGHT
}

fn plane_periods(block: &Block, class: Class, modifier: u64) -> Periods {
    let mut periods = Periods::default();
    match class {
        Class::Linear => match block.bytes() {
            EXPANDED_BYTES => periods.push_x(LINEAR_EXPANDED_PITCH_ELEMENTS, block),
            bytes => periods.push_x(LINEAR_PITCH_BYTES / bytes, block),
        },
        Class::Gfx6OneD { microtile } => {
            let width = match microtile {
                Microtile::Display => DISPLAY_MICRO_TILE_WIDTH,
                Microtile::Thin => MICRO_TILE,
            };
            periods.push_x(width, block);
            periods.push_y(MICRO_TILE, block);
        }
        Class::Gfx6TwoD { .. } => {
            let width = MICRO_TILE * pipes(modifier) * bank_width(modifier) * aspect(modifier);
            let height = MICRO_TILE * banks(modifier) * bank_height(modifier) / aspect(modifier);
            for multiple in MACRO_TILE_MULTIPLES {
                periods.push_x(width * multiple, block);
            }
            for multiple in MACRO_TILE_MULTIPLES {
                periods.push_y(height * multiple, block);
            }
        }
        Class::Gfx9To11 { block_log2 } | Class::Gfx12 { block_log2 } => {
            let element_log2 = element_log2(block.bytes());
            let tile = block_dims(block_log2, element_log2);
            periods.push_x(tile.width, block);
            periods.push_y(tile.height, block);
            if is_double_row_block(block) {
                periods.push_y(2 * tile.height, block);
            }
            if matches!(class, Class::Gfx9To11 { .. }) && dcc(modifier) {
                for meta_log2 in DCC_META_BLOCK_LOG2 {
                    let meta = block_dims(meta_log2 + DCC_META_COVERAGE_LOG2, element_log2);
                    periods.push_x(meta.width, block);
                    periods.push_y(meta.height, block);
                }
            }
        }
    }
    periods
}

fn joint_gfx6_1d_thin(block: &Block, out: &mut BTreeSet<Size>) {
    if block.bytes() != 1 && block.bytes() != 2 {
        return;
    }
    for columns in 1..=4 {
        for rows in 1..=4 {
            out.insert(Size {
                width: MICRO_TILE * columns * block.width,
                height: MICRO_TILE * rows * block.height,
            });
        }
    }
}

fn joint_gfx8_dcc(block: &Block, modifier: u64, out: &mut BTreeSet<Size>) {
    let banks = banks(modifier);
    let bank_width = bank_width(modifier);
    let bank_height = bank_height(modifier);
    let aspect = aspect(modifier);
    let tiles = GFX8_DCC_TILE_BYTES / (banks * bank_width * bank_height * block.bytes());
    if tiles == 0 {
        return;
    }
    let width = MICRO_TILE * pipes(modifier) * bank_width * aspect * block.width;
    let height = MICRO_TILE * banks * bank_height / aspect * block.height;
    for count in [1, tiles / 2, tiles, tiles + 1] {
        if let Some(split) = split_count(count, width, height) {
            out.insert(Size {
                width: split.columns * width,
                height: split.rows * height,
            });
        }
    }
}

fn joint_gfx9_dcc(block: &Block, block_log2: u32, modifier: u64, out: &mut BTreeSet<Size>) {
    let element_log2 = element_log2(block.bytes());
    let tile = block_dims(block_log2, element_log2);
    let block_size = 1u32 << block_log2;
    let mut size_align = block_size;
    if dcc_pipe_align(modifier) || dcc_retile(modifier) {
        let xor_bits = pipe_xor_bits(modifier).min(DCC_MAX_PIPE_XOR_BITS);
        size_align = size_align.max(1 << (DCC_PIPE_ALIGN_BASE_LOG2 + rb(modifier) + xor_bits));
    }
    for meta_log2 in DCC_META_BLOCK_LOG2 {
        let meta_size = 1u32 << meta_log2;
        let meta = block_dims(meta_log2 + DCC_META_COVERAGE_LOG2, element_log2);
        let blocks = size_align / meta_size;
        for count in [1, blocks.saturating_sub(1), blocks, blocks + 1] {
            if let Some(split) = split_count(count, meta.width, meta.height) {
                out.insert(Size {
                    width: split.columns * meta.width * block.width,
                    height: split.rows * meta.height * block.height,
                });
            }
        }
        let align = meta_size.max(size_align);
        for count in [1, align / block_size] {
            if let Some(split) = split_count(count, tile.width, tile.height) {
                out.insert(Size {
                    width: split.columns * tile.width * block.width,
                    height: split.rows * tile.height * block.height,
                });
            }
        }
    }
}

pub fn size_set(planes: &[Block], gfx_level: GfxLevel, modifier: u64) -> BTreeSet<Size> {
    let class = classify(modifier);
    let linear_expanded =
        class == Class::Linear && planes.iter().any(|block| block.bytes() == EXPANDED_BYTES);
    let mut periods = Periods::default();
    for block in planes {
        let plane = plane_periods(block, class, modifier);
        periods.x.extend(plane.x);
        periods.y.extend(plane.y);
    }
    let widths = match linear_expanded {
        true => linear_expanded_widths(),
        false => edges(&periods.x),
    };
    let heights = edges(&periods.y);
    let mut out = base_set(&widths, &heights, largest(&periods.x), largest(&periods.y));
    match class {
        Class::Gfx6OneD {
            microtile: Microtile::Thin,
        } => {
            for block in planes {
                joint_gfx6_1d_thin(block, &mut out);
            }
        }
        Class::Gfx6TwoD { dcc: true } if gfx_level == GfxLevel::Gfx8 => {
            for block in planes {
                joint_gfx8_dcc(block, modifier, &mut out);
            }
        }
        Class::Gfx9To11 { block_log2 } if gfx_level == GfxLevel::Gfx9 && dcc(modifier) => {
            for block in planes {
                joint_gfx9_dcc(block, block_log2, modifier, &mut out);
            }
        }
        _ => {}
    }
    out.retain(|size| {
        (1..=MAX_EXTENT).contains(&size.width) && (1..=MAX_EXTENT).contains(&size.height)
    });
    out
}
