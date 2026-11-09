use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::allocator::Extra;
use crate::drivers::amd::amdgpu::Bo;
use crate::drivers::amd::amdgpu::Device;
use crate::drivers::amd::amdgpu::GFX7;
use crate::drivers::amd::amdgpu::GFX8;
use crate::drivers::amd::modifier::Planes;
use crate::drivers::amd::modifier::TILE;
use crate::drivers::amd::modifier::TILE_VERSION;
use crate::drivers::amd::modifier::VER_GFX6;
use crate::drivers::amd::modifier::has_dcc;
use crate::drivers::amd::modifier::mask;
use crate::drivers::amd::modifier::push_padded;
use crate::drivers::amd::modifier::set;
use crate::drivers::amd::uapi::drm_amdgpu_gem_metadata__data;
use crate::format::Fmt;
use crate::kernel::Kernel;
use crate::layout::DRM_FORMAT_MOD_INVALID;
use crate::layout::DRM_FORMAT_MOD_LINEAR;
use crate::layout::Field;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::PlaneLayout;
use crate::layout::PlaneVec;
use crate::layout::align;
use crate::layout::blocks;
use crate::layout::end;
use crate::layout::field;
use crate::layout::linear;

const MICROTILE: Field = Field { shift: 14, bits: 3 };
const PIPE_CONFIG: Field = Field { shift: 17, bits: 5 };
const TILE_SPLIT: Field = Field { shift: 22, bits: 3 };
const BANK_WIDTH: Field = Field { shift: 25, bits: 2 };
const BANK_HEIGHT: Field = Field { shift: 27, bits: 2 };
const MACRO_TILE_ASPECT: Field = Field { shift: 29, bits: 2 };
const NUM_BANKS: Field = Field { shift: 31, bits: 2 };
pub(crate) const GFX6_CHIP_FIELDS: u64 = mask(PIPE_CONFIG)
    | mask(TILE_SPLIT)
    | mask(BANK_WIDTH)
    | mask(BANK_HEIGHT)
    | mask(MACRO_TILE_ASPECT)
    | mask(NUM_BANKS);

pub(crate) const GFX6_1D_TILED_THIN1: u64 = 2;
pub(crate) const GFX6_2D_TILED_THIN1: u64 = 4;
const MICROTILE_DISPLAY: u64 = 0;
const MICROTILE_THIN: u64 = 1;
const MICROTILE_DEPTH: u64 = 2;
const MICROTILE_ROTATED: u64 = 3;
pub(crate) const THIN: u64 = set(MICROTILE, MICROTILE_THIN);

const PIPE_INTERLEAVE: u64 = 256;
const LINEAR_PITCH_ALIGNMENT: u64 = 256;

const ATI_VENDOR_ID: u32 = 0x1002;
const METADATA_VERSION: u32 = 1;
pub(crate) const GFX6_METADATA_WORDS: usize = 11;

const DATA_FORMAT: Field = Field { shift: 20, bits: 6 };
const NUM_FORMAT: Field = Field { shift: 26, bits: 4 };
const WIDTH: Field = Field { shift: 0, bits: 14 };
const HEIGHT: Field = Field {
    shift: 14,
    bits: 14,
};
const PERF_MOD: Field = Field { shift: 28, bits: 3 };
const DST_SEL_X: Field = Field { shift: 0, bits: 3 };
const DST_SEL_Y: Field = Field { shift: 3, bits: 3 };
const DST_SEL_Z: Field = Field { shift: 6, bits: 3 };
const DST_SEL_W: Field = Field { shift: 9, bits: 3 };
const TILING_INDEX: Field = Field { shift: 20, bits: 5 };
const TYPE: Field = Field { shift: 28, bits: 4 };
const PITCH: Field = Field {
    shift: 13,
    bits: 14,
};
const COMPRESSION_EN: Field = Field { shift: 21, bits: 1 };
const ALPHA_IS_ON_MSB: Field = Field { shift: 22, bits: 1 };
const C_008F30_MAX_ANISO_RATIO: u32 = 0xfffff1ff;
const SQ_RSRC_IMG_2D: u64 = 9;
const PERF_MOD_DEFAULT: u64 = 4;

const GB_MICRO_TILE_MODE: Field = Field { shift: 0, bits: 2 };
const GB_ARRAY_MODE: Field = Field { shift: 2, bits: 4 };
const GB_PIPE_CONFIG: Field = Field { shift: 6, bits: 5 };
const GB_TILE_SPLIT: Field = Field { shift: 11, bits: 3 };
const GB_BANKS: Field = Field { shift: 14, bits: 8 };
const GB_MICRO_TILE_MODE_NEW: Field = Field { shift: 22, bits: 3 };
const GB_SAMPLE_SPLIT: Field = Field { shift: 25, bits: 2 };
const GB_MACRO_BANK_WIDTH: Field = Field { shift: 0, bits: 2 };
const GB_MACRO_BANK_HEIGHT: Field = Field { shift: 2, bits: 2 };
const GB_MACRO_MACRO_TILE_ASPECT: Field = Field { shift: 4, bits: 2 };
const GB_MACRO_NUM_BANKS: Field = Field { shift: 6, bits: 2 };
const GB_ROW_SIZE: Field = Field { shift: 28, bits: 2 };

const ARRAY_LINEAR_ALIGNED: u64 = 1;
const LINEAR_ALIGNED_TILING_INDEX: u64 = 8;
const TILE_INDEX_1D_DISPLAY: u64 = 9;
const TILE_INDEX_2D_DISPLAY_8BPP: u64 = 10;
const TILE_INDEX_2D_DISPLAY_16BPP: u64 = 11;
const TILE_INDEX_2D_DISPLAY_32BPP: u64 = 12;
const TILE_INDEX_1D_THIN: u64 = 13;
const TILE_INDEX_2D_THIN_8BPP: u64 = 14;
const TILE_INDEX_2D_THIN_16BPP: u64 = 15;
const TILE_INDEX_2D_THIN_32BPP: u64 = 16;
const TILE_INDEX_2D_THIN_64BPP: u64 = 17;

const TILE_SPLIT_8192: u64 = 7;
const EG_TILE_SPLIT_1024: u64 = 4;

const SQ_TEX_RESOURCE_WORD1: usize = 1;
const SQ_TEX_RESOURCE_WORD2: usize = 2;
const SQ_TEX_RESOURCE_WORD3: usize = 3;
const SQ_TEX_RESOURCE_WORD4: usize = 4;
const SQ_TEX_RESOURCE_WORD6: usize = 6;
const SQ_TEX_RESOURCE_WORD7: usize = 7;

const NUM_BANKS_NONE: u64 = 3;
const ARRAY_MODE_FLAG: Field = Field { shift: 0, bits: 4 };
const PIPE_CONFIG_FLAG: Field = Field { shift: 4, bits: 5 };
const TILE_SPLIT_FLAG: Field = Field { shift: 9, bits: 3 };
const MICRO_TILE_MODE_FLAG: Field = Field { shift: 12, bits: 3 };
const BANK_WIDTH_FLAG: Field = Field { shift: 15, bits: 2 };
const BANK_HEIGHT_FLAG: Field = Field { shift: 17, bits: 2 };
const MACRO_TILE_ASPECT_FLAG: Field = Field { shift: 19, bits: 2 };
const NUM_BANKS_FLAG: Field = Field { shift: 21, bits: 2 };

fn pipes(tiling: u64) -> Option<u64> {
    match field(tiling, PIPE_CONFIG) + 1 {
        1 => Some(2),
        5..=8 => Some(4),
        9..=15 => Some(8),
        17..=18 => Some(16),
        _ => None,
    }
}

fn degrade_to_1d(tiling: u64, config: &Config<'_>) -> bool {
    let Some(pipes) = pipes(tiling) else {
        return false;
    };
    let width = config.width as u64;
    let height = config.height as u64;
    let aspect = 1 << field(tiling, MACRO_TILE_ASPECT);
    let pitch_align = 8 * pipes * (1 << field(tiling, BANK_WIDTH)) * aspect;
    let height_align =
        8 * (2 << field(tiling, NUM_BANKS)) * (1 << field(tiling, BANK_HEIGHT)) / aspect;
    if width < pitch_align || height < height_align {
        return true;
    }
    let aligned = width.next_multiple_of(pitch_align) * height.next_multiple_of(height_align);
    2 * aligned > 3 * width * height
}

fn tiling_info(tiling: u64, scanout: bool) -> u64 {
    let microtile = match scanout {
        true => MICROTILE_DISPLAY,
        false => MICROTILE_THIN,
    };
    let flags = set(ARRAY_MODE_FLAG, field(tiling, TILE))
        | set(PIPE_CONFIG_FLAG, field(tiling, PIPE_CONFIG))
        | set(MICRO_TILE_MODE_FLAG, microtile);
    if field(tiling, TILE) != GFX6_2D_TILED_THIN1 {
        return flags | set(NUM_BANKS_FLAG, NUM_BANKS_NONE);
    }
    let tile_split = match field(tiling, TILE_SPLIT) {
        TILE_SPLIT_8192 => EG_TILE_SPLIT_1024,
        tile_split => tile_split,
    };
    flags
        | set(TILE_SPLIT_FLAG, tile_split)
        | set(BANK_WIDTH_FLAG, field(tiling, BANK_WIDTH))
        | set(BANK_HEIGHT_FLAG, field(tiling, BANK_HEIGHT))
        | set(MACRO_TILE_ASPECT_FLAG, field(tiling, MACRO_TILE_ASPECT))
        | set(NUM_BANKS_FLAG, field(tiling, NUM_BANKS))
}

fn planes(
    tiling: u64,
    fmt: &Fmt,
    width: u32,
    height: u32,
    display: bool,
    dcc: bool,
) -> Option<Planes> {
    let pipes = pipes(tiling)?;
    let bank_width = 1 << field(tiling, BANK_WIDTH);
    let bank_height = 1 << field(tiling, BANK_HEIGHT);
    let aspect = 1 << field(tiling, MACRO_TILE_ASPECT);
    let banks = 2 << field(tiling, NUM_BANKS);
    let tile_split = 64 << field(tiling, TILE_SPLIT);
    let mut alignment = PIPE_INTERLEAVE;
    let mut planes = PlaneVec::new();
    for plane in fmt.planes.iter() {
        let bytes = plane.bits as u64 / 8;
        let mut padded_width = blocks(plane.block_width, width);
        let mut padded_height = blocks(plane.block_height, height);
        match field(tiling, TILE) {
            GFX6_1D_TILED_THIN1 => {
                let pitch_align = if display { 32 } else { 8 };
                padded_width = align(padded_width, pitch_align);
                padded_height = align(padded_height, 8);
                while !(padded_width * padded_height * bytes).is_multiple_of(PIPE_INTERLEAVE) {
                    padded_width += pitch_align;
                }
            }
            GFX6_2D_TILED_THIN1 => {
                let mut pitch_align = 8 * pipes * bank_width * aspect;
                if display {
                    pitch_align = align(pitch_align, 32);
                }
                padded_width = align(padded_width, pitch_align);
                padded_height = align(padded_height, 8 * banks * bank_height / aspect);
                let tile_size = tile_split.min(64 * bytes);
                alignment = alignment.max(pipes * bank_width * banks * bank_height * tile_size);
            }
            _ => return None,
        }
        push_padded(&mut planes, padded_width, padded_height, bytes);
    }
    if dcc {
        let base_align = banks * pipes * PIPE_INTERLEAVE;
        planes.push(PlaneLayout {
            offset: align(end(&planes), base_align),
            stride: 1,
            size: align(planes[0].size >> 8, pipes * PIPE_INTERLEAVE),
        });
        alignment = alignment.max(base_align);
    }
    Some(Planes { planes, alignment })
}

pub(crate) fn modifier_planes(fmt: &Fmt, modifier: u64, width: u32, height: u32) -> Option<Planes> {
    let display = field(modifier, MICROTILE) == MICROTILE_DISPLAY;
    planes(modifier, fmt, width, height, display, has_dcc(modifier))
}

impl<K: Kernel> Device<K> {
    fn tiling_index(&self, tiling: u64, bytes: u64) -> u64 {
        match (field(tiling, TILE), field(tiling, MICROTILE)) {
            (ARRAY_LINEAR_ALIGNED, _) => LINEAR_ALIGNED_TILING_INDEX,
            (GFX6_1D_TILED_THIN1, MICROTILE_DISPLAY) => TILE_INDEX_1D_DISPLAY,
            (GFX6_1D_TILED_THIN1, _) => TILE_INDEX_1D_THIN,
            (_, MICROTILE_DISPLAY) if self.gfx_version >= GFX7 || bytes <= 1 => {
                TILE_INDEX_2D_DISPLAY_8BPP
            }
            (_, MICROTILE_DISPLAY) if bytes <= 2 => TILE_INDEX_2D_DISPLAY_16BPP,
            (_, MICROTILE_DISPLAY) => TILE_INDEX_2D_DISPLAY_32BPP,
            _ if self.gfx_version >= GFX7 || bytes <= 1 => TILE_INDEX_2D_THIN_8BPP,
            _ if bytes <= 2 => TILE_INDEX_2D_THIN_16BPP,
            _ if bytes <= 4 => TILE_INDEX_2D_THIN_32BPP,
            _ => TILE_INDEX_2D_THIN_64BPP,
        }
    }

    fn tile_mode(&self, tiling: u64, bytes: u64) -> u64 {
        self.tile_mode[self.tiling_index(tiling, bytes) as usize] as u64
    }

    fn micro_tile_mode(&self, tile_mode: u64) -> u64 {
        match self.gfx_version >= GFX7 {
            true => field(tile_mode, GB_MICRO_TILE_MODE_NEW),
            false => field(tile_mode, GB_MICRO_TILE_MODE),
        }
    }

    fn is_displayable(&self, tiling: u64, bytes: u64) -> bool {
        field(tiling, TILE) == ARRAY_LINEAR_ALIGNED
            || matches!(
                self.micro_tile_mode(self.tile_mode(tiling, bytes)),
                MICROTILE_DISPLAY | MICROTILE_ROTATED
            )
    }

    fn tiling(&self, bytes: u64, array_mode: u64, micro_tile_mode: u64) -> u64 {
        let tiling = set(TILE, array_mode) | set(MICROTILE, micro_tile_mode);
        let mode = self.tile_mode(tiling, bytes);
        let tiling = tiling | set(PIPE_CONFIG, field(mode, GB_PIPE_CONFIG));
        if array_mode != GFX6_2D_TILED_THIN1 {
            return tiling;
        }
        let bytes_log2 = bytes.ilog2() as u64;
        let tile_split = match self.gfx_version < GFX7
            || field(mode, GB_MICRO_TILE_MODE_NEW) == MICROTILE_DEPTH
        {
            true => field(mode, GB_TILE_SPLIT),
            false => (bytes_log2 + field(mode, GB_SAMPLE_SPLIT))
                .clamp(2, 4 + field(self.addr_config as u64, GB_ROW_SIZE)),
        };
        let banks = match self.gfx_version >= GFX7 {
            true => self.macrotile_mode[bytes_log2.min(tile_split) as usize] as u64,
            false => field(mode, GB_BANKS),
        };
        tiling
            | set(TILE_SPLIT, tile_split)
            | set(BANK_WIDTH, field(banks, GB_MACRO_BANK_WIDTH))
            | set(BANK_HEIGHT, field(banks, GB_MACRO_BANK_HEIGHT))
            | set(MACRO_TILE_ASPECT, field(banks, GB_MACRO_MACRO_TILE_ASPECT))
            | set(NUM_BANKS, field(banks, GB_MACRO_NUM_BANKS))
    }

    pub(crate) fn gfx6_metadata(
        &self,
        layout: &Layout,
        config: &Config<'_>,
        scanout: bool,
    ) -> drm_amdgpu_gem_metadata__data {
        let array_mode = match layout.modifier {
            DRM_FORMAT_MOD_LINEAR => ARRAY_LINEAR_ALIGNED,
            _ => field(layout.modifier, TILE),
        };
        let tiling = self.tiling(
            bytes(layout.format),
            array_mode,
            field(layout.modifier, MICROTILE),
        );
        let dcc_offset =
            has_dcc(layout.modifier).then(|| layout.planes[layout.format.planes.len()].offset);
        self.metadata(layout, tiling, config, scanout, dcc_offset)
    }

    fn metadata(
        &self,
        layout: &Layout,
        tiling: u64,
        config: &Config<'_>,
        scanout: bool,
        dcc_offset: Option<u64>,
    ) -> drm_amdgpu_gem_metadata__data {
        let bytes = bytes(layout.format);
        let mut data = [0; 64];
        let mut data_size_bytes = 0;
        if let Some(image) = &layout.format.amd_gfx6 {
            let pitch = layout.planes[0].stride / bytes;
            data_size_bytes = size_of::<[u32; GFX6_METADATA_WORDS]>() as u32;
            data[0] = METADATA_VERSION;
            data[1] = ATI_VENDOR_ID << 16 | self.pci_id;
            let desc = &mut data[2..];
            desc[SQ_TEX_RESOURCE_WORD1] = (set(DATA_FORMAT, image.data_format as u64)
                | set(NUM_FORMAT, image.num_format as u64))
                as u32;
            desc[SQ_TEX_RESOURCE_WORD2] = (set(WIDTH, config.width as u64 - 1)
                | set(HEIGHT, config.height as u64 - 1)
                | set(PERF_MOD, PERF_MOD_DEFAULT)) as u32;
            desc[SQ_TEX_RESOURCE_WORD3] = (set(DST_SEL_X, image.dst_sel[0] as u64)
                | set(DST_SEL_Y, image.dst_sel[1] as u64)
                | set(DST_SEL_Z, image.dst_sel[2] as u64)
                | set(DST_SEL_W, image.dst_sel[3] as u64)
                | set(TILING_INDEX, self.tiling_index(tiling, bytes))
                | set(TYPE, SQ_RSRC_IMG_2D)) as u32;
            desc[SQ_TEX_RESOURCE_WORD4] = set(PITCH, pitch - 1) as u32;
            match dcc_offset {
                Some(offset) => {
                    desc[SQ_TEX_RESOURCE_WORD6] =
                        (set(ALPHA_IS_ON_MSB, image.alpha_is_on_msb as u64)
                            | set(COMPRESSION_EN, 1)) as u32;
                    desc[SQ_TEX_RESOURCE_WORD7] = (offset >> 8) as u32;
                }
                None if self.gfx_version < GFX8 => {
                    desc[SQ_TEX_RESOURCE_WORD7] = C_008F30_MAX_ANISO_RATIO
                }
                None => {}
            }
        }
        drm_amdgpu_gem_metadata__data {
            flags: 0,
            tiling_info: tiling_info(tiling, scanout || self.is_displayable(tiling, bytes)),
            data_size_bytes,
            data,
        }
    }

    pub(crate) fn gfx6_supports(&self, modifier: u64, config: &Config<'_>) -> bool {
        if field(modifier, TILE_VERSION) != VER_GFX6 {
            return false;
        }
        let array_mode = field(modifier, TILE);
        let micro_tile_mode = field(modifier, MICROTILE);
        let mode = self.tile_mode(modifier, bytes(config.format.0));
        if field(mode, GB_ARRAY_MODE) != array_mode {
            return false;
        }
        if self.micro_tile_mode(mode) != micro_tile_mode {
            return false;
        }
        if config.implicit_interop {
            if config.scanout && has_dcc(modifier) {
                return false;
            }
            if micro_tile_mode == MICROTILE_DISPLAY {
                return true;
            }
            if config.height == 1 {
                return false;
            }
            if array_mode == GFX6_2D_TILED_THIN1 && degrade_to_1d(modifier, config) {
                return false;
            }
        }
        true
    }

    pub(crate) fn implicit_layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        let fmt = config.format.0;
        if !fmt.amd_implicit {
            return Err(LayoutError::Format);
        }
        let bytes = bytes(fmt);
        let display_pitch = config.scanout && fmt.amd_displayable;
        let tiling = self.choose(bytes, display_pitch, config);
        let array_mode = field(tiling, TILE);
        let scanout = config.scanout || self.is_displayable(tiling, bytes);
        let dcc = self.gfx_version >= GFX8
            && self.has_graphics
            && array_mode == GFX6_2D_TILED_THIN1
            && !(fmt.amd_displayable && scanout);
        let mut planes = match array_mode {
            ARRAY_LINEAR_ALIGNED => {
                let pitch = linear_pitch(bytes, config, display_pitch);
                let mut planes = PlaneVec::new();
                push_padded(&mut planes, pitch, config.height as u64, bytes);
                Planes {
                    planes,
                    alignment: linear::ALIGNMENT,
                }
            }
            _ => planes(tiling, fmt, config.width, config.height, display_pitch, dcc)
                .ok_or(LayoutError::Unsupported)?,
        };
        let size = end(&planes.planes);
        let dcc_offset = (dcc && !config.scanout).then(|| planes.planes[fmt.planes.len()].offset);
        planes.planes.truncate(fmt.planes.len());
        let layout = Layout {
            format: fmt,
            modifier: DRM_FORMAT_MOD_INVALID,
            planes: planes.planes,
        };
        let bo = Bo {
            size,
            alignment: planes.alignment,
            linear: array_mode == ARRAY_LINEAR_ALIGNED,
            metadata: self.metadata(&layout, tiling, config, scanout, dcc_offset),
        };
        Ok(DeviceLayout {
            layout,
            extra: Extra::Amd(bo),
        })
    }

    fn choose(&self, bytes: u64, display: bool, config: &Config<'_>) -> u64 {
        if config.linear || config.height <= 2 {
            return self.tiling(bytes, ARRAY_LINEAR_ALIGNED, MICROTILE_DISPLAY);
        }
        let microtile = match config.scanout && bytes != 16 {
            true => MICROTILE_DISPLAY,
            false => MICROTILE_THIN,
        };
        let micro = self.tiling(bytes, GFX6_1D_TILED_THIN1, microtile);
        if config.width <= 16 || config.height <= 16 {
            return micro;
        }
        let macro_ = self.tiling(bytes, GFX6_2D_TILED_THIN1, microtile);
        if !display && degrade_to_1d(macro_, config) {
            return micro;
        }
        macro_
    }
}

fn bytes(fmt: &Fmt) -> u64 {
    fmt.planes[0].bits as u64 / 8
}

fn linear_pitch(bytes: u64, config: &Config<'_>, display: bool) -> u64 {
    let mut pitch_align = (64 / bytes).max(8);
    if display {
        pitch_align = pitch_align.next_multiple_of(32);
    }
    let slice_align = (256 / bytes).max(64);
    let width = (config.width as u64).next_multiple_of(LINEAR_PITCH_ALIGNMENT / bytes);
    let mut pitch = width.next_multiple_of(pitch_align);
    while !(pitch * config.height as u64).is_multiple_of(slice_align) {
        pitch += pitch_align;
    }
    pitch
}
