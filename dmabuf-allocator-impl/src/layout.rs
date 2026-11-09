use crate::allocator::Config;
use crate::format::Fmt;
use arrayvec::ArrayVec;
use thiserror::Error;

pub mod linear;

pub(crate) const DRM_FORMAT_MOD_LINEAR: u64 = 0;
pub const DRM_FORMAT_MOD_INVALID: u64 = 0x00ff_ffff_ffff_ffff;

pub(crate) const DRM_FORMAT_MOD_VENDOR_INTEL: u64 = 0x01;
pub(crate) const DRM_FORMAT_MOD_VENDOR_AMD: u64 = 0x02;
pub(crate) const DRM_FORMAT_MOD_VENDOR_NVIDIA: u64 = 0x03;
pub(crate) const DRM_FORMAT_MOD_VENDOR_VIVANTE: u64 = 0x06;
pub(crate) const DRM_FORMAT_MOD_VENDOR_BROADCOM: u64 = 0x07;
pub(crate) const DRM_FORMAT_MOD_VENDOR_ARM: u64 = 0x08;
pub(crate) const DRM_FORMAT_MOD_VENDOR_APPLE: u64 = 0x0c;

pub(crate) const VENDOR: Field = Field { shift: 56, bits: 8 };

pub(crate) const fn fourcc_mod_code(vendor: u64, value: u64) -> u64 {
    vendor << VENDOR.shift | value
}

const DRM_FORMAT_MOD_ARM_TYPE_MISC: u64 = 0x01;

pub(crate) const DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED: u64 = fourcc_mod_code(
    DRM_FORMAT_MOD_VENDOR_ARM,
    DRM_FORMAT_MOD_ARM_TYPE_MISC << 52 | 1,
);

pub(crate) const PAGE_SIZE: u64 = 4096;

pub struct Layout {
    pub(crate) format: &'static Fmt,
    pub modifier: u64,
    pub planes: PlaneVec<PlaneLayout>,
}

pub type PlaneVec<T> = ArrayVec<T, 4>;

#[derive(Copy, Clone)]
pub struct PlaneLayout {
    pub offset: u64,
    pub stride: u64,
    pub size: u64,
}

#[derive(Debug, Error)]
pub enum LayoutError {
    #[error("the device does not support the format")]
    Format,
    #[error("the device does not support the size")]
    Size,
    #[error("none of the requested modifiers can be used")]
    Modifier,
    #[error("the buffer is too large")]
    TooLarge,
    #[error("the device configuration is not supported")]
    Unsupported,
    #[error("the driver determines the layout during the allocation")]
    Dumb,
}

pub(crate) fn best_layout<T, F, L, R>(
    config: &Config<'_>,
    linear: F,
    layout: L,
    rank: R,
) -> Result<T, LayoutError>
where
    F: Fn(Layout) -> T,
    L: Fn(u64) -> Option<T>,
    R: Fn(u64) -> Option<u32>,
{
    let rank = |modifier| match modifier {
        DRM_FORMAT_MOD_LINEAR => Some(0),
        _ => rank(modifier).map(|rank| rank + 1),
    };
    let mut best = Err(LayoutError::Modifier);
    let mut best_rank = None;
    for &modifier in config.modifiers {
        let Some(rank) = rank(modifier) else {
            continue;
        };
        if best_rank.is_some_and(|best_rank| rank <= best_rank) {
            continue;
        }
        let candidate = match modifier {
            DRM_FORMAT_MOD_LINEAR => {
                linear::compute_layout(config.format.0, config.width, config.height).map(&linear)
            }
            _ => layout(modifier).ok_or(LayoutError::Modifier),
        };
        match candidate {
            Ok(layout) => {
                best = Ok(layout);
                best_rank = Some(rank);
            }
            Err(error) if best.is_err() => best = Err(error),
            Err(_) => {}
        }
    }
    best
}

pub(crate) fn end(planes: &PlaneVec<PlaneLayout>) -> u64 {
    match planes.last() {
        Some(plane) => plane.offset.saturating_add(plane.size),
        None => 0,
    }
}

pub(crate) const fn align(value: u64, alignment: u64) -> u64 {
    value.div_ceil(alignment).saturating_mul(alignment)
}

pub(crate) const fn blocks(block: u32, size: u32) -> u64 {
    size.div_ceil(block) as u64
}

#[derive(Copy, Clone)]
pub(crate) struct Field {
    pub(crate) shift: u32,
    pub(crate) bits: u32,
}

pub(crate) const fn field(modifier: u64, field: Field) -> u64 {
    (modifier >> field.shift) & ((1 << field.bits) - 1)
}
