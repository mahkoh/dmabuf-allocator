use crate::format::Fmt;
use crate::layout::DRM_FORMAT_MOD_LINEAR;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::PlaneLayout;
use crate::layout::PlaneVec;
use crate::layout::align;
use crate::layout::blocks;

pub(crate) const ALIGNMENT: u64 = 256;

pub fn compute_layout(fmt: &'static Fmt, width: u32, height: u32) -> Result<Layout, LayoutError> {
    let mut planes = PlaneVec::new();
    let mut offset = 0u64;
    for plane in fmt.planes {
        let blocks_x = blocks(plane.block_width, width);
        let blocks_y = blocks(plane.block_height, height);
        let stride = align((blocks_x * plane.bits as u64).div_ceil(8), ALIGNMENT);
        let size = stride.checked_mul(blocks_y).ok_or(LayoutError::TooLarge)?;
        let next = offset.checked_add(size).ok_or(LayoutError::TooLarge)?;
        planes.push(PlaneLayout {
            offset,
            stride,
            size,
        });
        offset = next;
    }
    Ok(Layout {
        format: fmt,
        modifier: DRM_FORMAT_MOD_LINEAR,
        planes,
    })
}
