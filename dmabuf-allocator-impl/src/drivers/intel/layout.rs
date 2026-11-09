use crate::allocator::Config;
use crate::drivers::intel::device_info::DeviceInfo;
use crate::layout::DRM_FORMAT_MOD_VENDOR_INTEL;
use crate::layout::Layout;
use crate::layout::PAGE_SIZE;
use crate::layout::PlaneLayout;
use crate::layout::PlaneVec;
use crate::layout::align;
use crate::layout::end;
use crate::layout::fourcc_mod_code;

const fn intel_mod_code(value: u64) -> u64 {
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, value)
}

const TILE_SIZE: u64 = 4096;
const X_TILE_WIDTH: u64 = 512;
const Y_TILE_WIDTH: u64 = 128;
const Y_TILE_HEIGHT: u64 = TILE_SIZE / Y_TILE_WIDTH;
const MAX_TILED_WIDTH_GEN8_SKL: u64 = 16382;
const CCS_PITCH_ALIGNMENT: u64 = 512;
const GEN9_CCS_PITCH_ALIGNMENT_MIN_WIDTH: u64 = 3840;
const MAX_SURFACE_SIZE_GEN11: u64 = 1 << 44;
const MAX_SURFACE_SIZE_GEN9: u64 = 1 << 38;
const MAX_SURFACE_SIZE: u64 = 1 << 31;
const INTEL_AUX_MAP_MAIN_PITCH_SCALEDOWN: u64 = 8;
const INTEL_AUX_MAP_MAIN_SIZE_SCALEDOWN: u64 = 256;
const CCS_BLOCK_BITS: u64 = 256;
const CCS_BLOCK_HEIGHT: u64 = 16;
const CLEAR_COLOR_PITCH: u64 = 64;

pub(crate) const I915_FORMAT_MOD_X_TILED: u64 = intel_mod_code(1);
pub(crate) const I915_FORMAT_MOD_Y_TILED: u64 = intel_mod_code(2);
pub(crate) const I915_FORMAT_MOD_Y_TILED_CCS: u64 = intel_mod_code(4);
pub(crate) const I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS: u64 = intel_mod_code(6);
pub(crate) const I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS: u64 = intel_mod_code(7);
pub(crate) const I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC: u64 = intel_mod_code(8);
pub(crate) const I915_FORMAT_MOD_4_TILED: u64 = intel_mod_code(9);
pub(crate) const I915_FORMAT_MOD_4_TILED_DG2_RC_CCS: u64 = intel_mod_code(10);
pub(crate) const I915_FORMAT_MOD_4_TILED_DG2_MC_CCS: u64 = intel_mod_code(11);
pub(crate) const I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC: u64 = intel_mod_code(12);
pub(crate) const I915_FORMAT_MOD_4_TILED_MTL_RC_CCS: u64 = intel_mod_code(13);
pub(crate) const I915_FORMAT_MOD_4_TILED_MTL_MC_CCS: u64 = intel_mod_code(14);
pub(crate) const I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC: u64 = intel_mod_code(15);
pub(crate) const I915_FORMAT_MOD_4_TILED_LNL_CCS: u64 = intel_mod_code(16);
pub(crate) const I915_FORMAT_MOD_4_TILED_BMG_CCS: u64 = intel_mod_code(17);

pub(crate) const RANKED: [u64; 12] = [
    I915_FORMAT_MOD_X_TILED,
    I915_FORMAT_MOD_Y_TILED,
    I915_FORMAT_MOD_Y_TILED_CCS,
    I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS,
    I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC,
    I915_FORMAT_MOD_4_TILED,
    I915_FORMAT_MOD_4_TILED_DG2_RC_CCS,
    I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC,
    I915_FORMAT_MOD_4_TILED_MTL_RC_CCS,
    I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC,
    I915_FORMAT_MOD_4_TILED_LNL_CCS,
    I915_FORMAT_MOD_4_TILED_BMG_CCS,
];

pub fn compute_layout(config: &Config<'_>, modifier: u64, info: &DeviceInfo) -> Option<Layout> {
    let width = config.width as u64;
    let height = config.height as u64;
    let fmt = config.format.0;
    let has_aux = has_fast_clears(modifier);
    let supported = match modifier {
        I915_FORMAT_MOD_X_TILED => true,
        I915_FORMAT_MOD_Y_TILED => info.verx10 < 125 && (info.ver > 8 || !config.scanout),
        I915_FORMAT_MOD_Y_TILED_CCS => matches!(info.ver, 9..=11),
        I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS
        | I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS
        | I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC => info.verx10 == 120,
        I915_FORMAT_MOD_4_TILED => info.verx10 >= 125,
        I915_FORMAT_MOD_4_TILED_DG2_RC_CCS
        | I915_FORMAT_MOD_4_TILED_DG2_MC_CCS
        | I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC => info.dg2,
        I915_FORMAT_MOD_4_TILED_MTL_RC_CCS
        | I915_FORMAT_MOD_4_TILED_MTL_MC_CCS
        | I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC => info.mtl_or_arl,
        I915_FORMAT_MOD_4_TILED_LNL_CCS => info.ver >= 20 && !info.has_local_mem,
        I915_FORMAT_MOD_4_TILED_BMG_CCS => info.ver >= 20 && info.has_local_mem,
        _ => false,
    };
    if !supported
        || has_aux && fmt.planes.len() > 1
        || width > MAX_TILED_WIDTH_GEN8_SKL && (info.ver == 8 || info.skl && info.gt != 4)
    {
        return None;
    }
    let tile_w = match modifier {
        I915_FORMAT_MOD_X_TILED => X_TILE_WIDTH,
        _ => Y_TILE_WIDTH,
    };
    let tile_h = TILE_SIZE / tile_w;
    let wide =
        has_aux && (info.ver == 12 || info.ver == 9 && width > GEN9_CCS_PITCH_ALIGNMENT_MIN_WIDTH);
    let stride_align = if wide { CCS_PITCH_ALIGNMENT } else { tile_w };
    let max_size: u64 = match info.ver {
        11.. => MAX_SURFACE_SIZE_GEN11,
        9 | 10 => MAX_SURFACE_SIZE_GEN9,
        _ => MAX_SURFACE_SIZE,
    };
    let mut planes = PlaneVec::new();
    for plane in fmt.planes {
        let bits = plane.bits as u64;
        let scale = if bits.is_multiple_of(3) { 3 } else { 1 };
        let bpb = bits / scale;
        if bpb < 8 || !bpb.is_power_of_two() {
            return None;
        }
        let width_el = width.div_ceil(plane.block_width as u64);
        let height_el = height.div_ceil(plane.block_height as u64);
        let stride = align(width_el * bits / 8, stride_align);
        let size = if info.verx10 == 120 && has_aux {
            align(height_el, tile_h) * stride
        } else {
            let x_tl = (width_el - 1) * (bpb / 8) / tile_w;
            tile_h * ((height_el - 1) / tile_h * stride + (x_tl * scale + 1) * tile_w)
        };
        if size > max_size {
            return None;
        }
        push(&mut planes, stride, size)?;
        if has_aux && info.has_aux_map {
            push(
                &mut planes,
                stride / INTEL_AUX_MAP_MAIN_PITCH_SCALEDOWN,
                size / INTEL_AUX_MAP_MAIN_SIZE_SCALEDOWN,
            )?;
        } else if has_aux && !info.has_flat_ccs {
            if !matches!(bits, 32 | 64 | 128) {
                return None;
            }
            let stride = align(width.div_ceil(CCS_BLOCK_BITS / bits), Y_TILE_WIDTH);
            let rows = align(height.div_ceil(CCS_BLOCK_HEIGHT), Y_TILE_HEIGHT);
            push(&mut planes, stride, rows * stride)?;
        }
    }
    if supports_clear_color(modifier) {
        push(&mut planes, CLEAR_COLOR_PITCH, info.clear_color_state_size)?;
    }
    Some(Layout {
        format: fmt,
        modifier,
        planes,
    })
}

#[must_use]
fn push(planes: &mut PlaneVec<PlaneLayout>, stride: u64, size: u64) -> Option<()> {
    let offset = align(end(planes), PAGE_SIZE);
    planes
        .try_push(PlaneLayout {
            offset,
            stride,
            size,
        })
        .ok()
}

pub(crate) fn has_ccs(modifier: u64) -> bool {
    has_fast_clears(modifier)
        || matches!(
            modifier,
            I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS
                | I915_FORMAT_MOD_4_TILED_DG2_MC_CCS
                | I915_FORMAT_MOD_4_TILED_MTL_MC_CCS
        )
}

pub(crate) fn has_fast_clears(modifier: u64) -> bool {
    matches!(
        modifier,
        I915_FORMAT_MOD_Y_TILED_CCS
            | I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS
            | I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC
            | I915_FORMAT_MOD_4_TILED_DG2_RC_CCS
            | I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC
            | I915_FORMAT_MOD_4_TILED_MTL_RC_CCS
            | I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC
            | I915_FORMAT_MOD_4_TILED_LNL_CCS
            | I915_FORMAT_MOD_4_TILED_BMG_CCS
    )
}

pub(crate) fn supports_clear_color(modifier: u64) -> bool {
    matches!(
        modifier,
        I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC
            | I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC
            | I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC
    )
}
