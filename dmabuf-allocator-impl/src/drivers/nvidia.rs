use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::allocator::Extra;
use crate::format::Plane;
use crate::layout::DRM_FORMAT_MOD_VENDOR_NVIDIA;
use crate::layout::Field;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::PlaneLayout;
use crate::layout::PlaneVec;
use crate::layout::VENDOR;
use crate::layout::align;
use crate::layout::best_layout;
use crate::layout::blocks;
use crate::layout::end;
use crate::layout::field;

pub(crate) mod nouveau;
pub(crate) mod nvidia_drm;

const LOG2_GOBS: Field = Field { shift: 0, bits: 4 };
const BLOCK_LINEAR: Field = Field { shift: 4, bits: 1 };
const RESERVED_LOW: Field = Field { shift: 5, bits: 7 };
const PAGE_KIND: Field = Field { shift: 12, bits: 8 };
const GOB_GENERATION: Field = Field { shift: 20, bits: 2 };
const COMPRESSION: Field = Field { shift: 23, bits: 3 };
const SECTOR_LAYOUT_HIGH: Field = Field { shift: 26, bits: 2 };
const RESERVED_HIGH: Field = Field {
    shift: 28,
    bits: 28,
};

const THIRTYTWO_GOBS: u64 = 5;
const GOB_GENERATION_G80: u64 = 1;
const GOB_GENERATION_RESERVED: u64 = 3;
const MAX_SECTOR_LAYOUT_HIGH: u64 = 1;
const COMPRESSION_CDE_VERTICAL: u64 = 4;
const GOB_HEIGHT: u64 = 8;
const GOB_HEIGHT_G80: u64 = 4;
const GOB_WIDTH_BYTES: u64 = 64;
const PLANE_SIZE_ALIGNMENT: u64 = 1 << 16;

fn layout(
    config: &Config<'_>,
    tiled: bool,
    max_extent: u32,
    rank: impl Fn(u64) -> u32,
) -> Result<DeviceLayout, LayoutError> {
    let extents = 1..=max_extent;
    if !extents.contains(&config.width) || !extents.contains(&config.height) {
        return Err(LayoutError::Size);
    }
    let tiled = tiled && !config.implicit_interop;
    let device_layout = |layout| DeviceLayout {
        layout,
        extra: Extra::None,
    };
    best_layout(
        config,
        device_layout,
        |modifier| tiled_layout(config, modifier).map(device_layout),
        |modifier| (tiled && supported(modifier)).then(|| rank(modifier)),
    )
}

fn supported(modifier: u64) -> bool {
    field(modifier, VENDOR) == DRM_FORMAT_MOD_VENDOR_NVIDIA
        && field(modifier, BLOCK_LINEAR) == 1
        && field(modifier, RESERVED_LOW) == 0
        && field(modifier, RESERVED_HIGH) == 0
        && field(modifier, LOG2_GOBS) <= THIRTYTWO_GOBS
        && field(modifier, GOB_GENERATION) != GOB_GENERATION_RESERVED
        && field(modifier, SECTOR_LAYOUT_HIGH) <= MAX_SECTOR_LAYOUT_HIGH
        && field(modifier, COMPRESSION) <= COMPRESSION_CDE_VERTICAL
}

fn gob_height(modifier: u64) -> u64 {
    match field(modifier, GOB_GENERATION) {
        GOB_GENERATION_G80 => GOB_HEIGHT_G80,
        _ => GOB_HEIGHT,
    }
}

fn log2_gobs(config: &Config<'_>, modifier: u64, plane: &Plane) -> u64 {
    let height_gobs = blocks(plane.block_height, config.height).div_ceil(gob_height(modifier));
    field(modifier, LOG2_GOBS).min(height_gobs.next_power_of_two().ilog2() as u64)
}

fn tiled_layout(config: &Config<'_>, modifier: u64) -> Option<Layout> {
    let mut planes = PlaneVec::new();
    for plane in config.format.0.planes.iter() {
        let stride = align(
            blocks(plane.block_width, config.width) * plane.bits as u64 / 8,
            GOB_WIDTH_BYTES,
        );
        let rows = align(
            blocks(plane.block_height, config.height),
            gob_height(modifier) << log2_gobs(config, modifier, plane),
        );
        let offset = end(&planes).checked_next_multiple_of(PLANE_SIZE_ALIGNMENT)?;
        let size = stride.checked_mul(rows)?;
        offset.checked_add(size)?;
        planes.push(PlaneLayout {
            offset,
            stride,
            size,
        });
    }
    Some(Layout {
        format: config.format.0,
        modifier,
        planes,
    })
}
