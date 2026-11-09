use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_BANK_HEIGHT_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_BANK_HEIGHT_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_BANK_WIDTH_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_BANK_WIDTH_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_PIPE_ALIGN_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_PIPE_ALIGN_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_RETILE_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_RETILE_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_MACRO_TILE_ASPECT_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_MACRO_TILE_ASPECT_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_MICROTILE_DISPLAY;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_MICROTILE_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_MICROTILE_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_MICROTILE_THIN;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_NUM_BANKS_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_NUM_BANKS_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P2;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P4_8x16;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P4_16x16;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P4_16x32;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P4_32x32;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P8_16x16_8x16;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P8_16x32_8x16;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P8_16x32_16x16;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_16x16;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_16x32;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P8_32x64_32x32;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_8x16;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_XOR_BITS_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_XOR_BITS_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_RB_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_RB_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX6_1D_TILED_THIN1;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX9_4K_D_X;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX9_64K_D;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX9_64K_D_X;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX9_64K_R_X;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX9_64K_S;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX9_64K_S_X;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX11_256K_R_X;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX12_4K_2D;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX12_64K_2D;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX12_256B_2D;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX12_256K_2D;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VER_GFX6;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VER_GFX9;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VER_GFX10;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VER_GFX11;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VER_GFX12;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VERSION_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VERSION_SHIFT;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_LINEAR;

#[cfg(test)]
mod tests;

fn get(modifier: u64, shift: u32, mask: u64) -> u64 {
    (modifier >> shift) & mask
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TileVersion {
    Gfx6,
    Gfx9,
    Gfx10,
    Gfx10Rbplus,
    Gfx11,
    Gfx12,
}

pub fn tile_version(modifier: u64) -> TileVersion {
    let version = get(
        modifier,
        AMD_FMT_MOD_TILE_VERSION_SHIFT,
        AMD_FMT_MOD_TILE_VERSION_MASK,
    );
    match version {
        AMD_FMT_MOD_TILE_VER_GFX6 => TileVersion::Gfx6,
        AMD_FMT_MOD_TILE_VER_GFX9 => TileVersion::Gfx9,
        AMD_FMT_MOD_TILE_VER_GFX10 => TileVersion::Gfx10,
        AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS => TileVersion::Gfx10Rbplus,
        AMD_FMT_MOD_TILE_VER_GFX11 => TileVersion::Gfx11,
        AMD_FMT_MOD_TILE_VER_GFX12 => TileVersion::Gfx12,
        other => panic!("unknown AMD tile version {other}"),
    }
}

fn tile(modifier: u64) -> u64 {
    get(modifier, AMD_FMT_MOD_TILE_SHIFT, AMD_FMT_MOD_TILE_MASK)
}

pub fn dcc(modifier: u64) -> bool {
    get(modifier, AMD_FMT_MOD_DCC_SHIFT, AMD_FMT_MOD_DCC_MASK) == 1
}

pub fn dcc_retile(modifier: u64) -> bool {
    get(
        modifier,
        AMD_FMT_MOD_DCC_RETILE_SHIFT,
        AMD_FMT_MOD_DCC_RETILE_MASK,
    ) == 1
}

pub fn dcc_pipe_align(modifier: u64) -> bool {
    get(
        modifier,
        AMD_FMT_MOD_DCC_PIPE_ALIGN_SHIFT,
        AMD_FMT_MOD_DCC_PIPE_ALIGN_MASK,
    ) == 1
}

pub fn pipe_xor_bits(modifier: u64) -> u32 {
    get(
        modifier,
        AMD_FMT_MOD_PIPE_XOR_BITS_SHIFT,
        AMD_FMT_MOD_PIPE_XOR_BITS_MASK,
    ) as u32
}

pub fn rb(modifier: u64) -> u32 {
    get(modifier, AMD_FMT_MOD_RB_SHIFT, AMD_FMT_MOD_RB_MASK) as u32
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Microtile {
    Display,
    Thin,
}

fn microtile(modifier: u64) -> Microtile {
    let microtile = get(
        modifier,
        AMD_FMT_MOD_MICROTILE_SHIFT,
        AMD_FMT_MOD_MICROTILE_MASK,
    );
    match microtile {
        AMD_FMT_MOD_MICROTILE_DISPLAY => Microtile::Display,
        AMD_FMT_MOD_MICROTILE_THIN => Microtile::Thin,
        other => panic!("unknown AMD micro tile mode {other}"),
    }
}

struct PipeConfigs {
    pipes: u32,
    configs: &'static [u64],
}

const PIPE_CONFIGS: [PipeConfigs; 4] = [
    PipeConfigs {
        pipes: 2,
        configs: &[AMD_FMT_MOD_PIPE_CONFIG_P2],
    },
    PipeConfigs {
        pipes: 4,
        configs: &[
            AMD_FMT_MOD_PIPE_CONFIG_P4_8x16,
            AMD_FMT_MOD_PIPE_CONFIG_P4_16x16,
            AMD_FMT_MOD_PIPE_CONFIG_P4_16x32,
            AMD_FMT_MOD_PIPE_CONFIG_P4_32x32,
        ],
    },
    PipeConfigs {
        pipes: 8,
        configs: &[
            AMD_FMT_MOD_PIPE_CONFIG_P8_16x16_8x16,
            AMD_FMT_MOD_PIPE_CONFIG_P8_16x32_8x16,
            AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16,
            AMD_FMT_MOD_PIPE_CONFIG_P8_16x32_16x16,
            AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_16x16,
            AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_16x32,
            AMD_FMT_MOD_PIPE_CONFIG_P8_32x64_32x32,
        ],
    },
    PipeConfigs {
        pipes: 16,
        configs: &[
            AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_8x16,
            AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16,
        ],
    },
];

pub fn pipes(modifier: u64) -> u32 {
    let config = get(
        modifier,
        AMD_FMT_MOD_PIPE_CONFIG_SHIFT,
        AMD_FMT_MOD_PIPE_CONFIG_MASK,
    );
    let known = PIPE_CONFIGS
        .iter()
        .find(|known| known.configs.contains(&config));
    known
        .unwrap_or_else(|| panic!("unknown AMD pipe config {config}"))
        .pipes
}

pub fn bank_width(modifier: u64) -> u32 {
    1 << get(
        modifier,
        AMD_FMT_MOD_BANK_WIDTH_SHIFT,
        AMD_FMT_MOD_BANK_WIDTH_MASK,
    )
}

pub fn bank_height(modifier: u64) -> u32 {
    1 << get(
        modifier,
        AMD_FMT_MOD_BANK_HEIGHT_SHIFT,
        AMD_FMT_MOD_BANK_HEIGHT_MASK,
    )
}

pub fn aspect(modifier: u64) -> u32 {
    1 << get(
        modifier,
        AMD_FMT_MOD_MACRO_TILE_ASPECT_SHIFT,
        AMD_FMT_MOD_MACRO_TILE_ASPECT_MASK,
    )
}

pub fn banks(modifier: u64) -> u32 {
    2 << get(
        modifier,
        AMD_FMT_MOD_NUM_BANKS_SHIFT,
        AMD_FMT_MOD_NUM_BANKS_MASK,
    )
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Class {
    Linear,
    Gfx6OneD { microtile: Microtile },
    Gfx6TwoD { dcc: bool },
    Gfx9To11 { block_log2: u32 },
    Gfx12 { block_log2: u32 },
}

pub fn classify(modifier: u64) -> Class {
    if modifier == DRM_FORMAT_MOD_LINEAR {
        return Class::Linear;
    }
    match tile_version(modifier) {
        TileVersion::Gfx6 => match tile(modifier) {
            AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1 => Class::Gfx6TwoD { dcc: dcc(modifier) },
            AMD_FMT_MOD_TILE_GFX6_1D_TILED_THIN1 => Class::Gfx6OneD {
                microtile: microtile(modifier),
            },
            other => panic!("unknown GFX6 tile mode {other}"),
        },
        TileVersion::Gfx9 | TileVersion::Gfx10 | TileVersion::Gfx10Rbplus | TileVersion::Gfx11 => {
            Class::Gfx9To11 {
                block_log2: gfx9_block_log2(modifier),
            }
        }
        TileVersion::Gfx12 => Class::Gfx12 {
            block_log2: gfx12_block_log2(modifier),
        },
    }
}

fn gfx9_block_log2(modifier: u64) -> u32 {
    match tile(modifier) {
        AMD_FMT_MOD_TILE_GFX9_4K_D_X => 12,
        AMD_FMT_MOD_TILE_GFX9_64K_S
        | AMD_FMT_MOD_TILE_GFX9_64K_D
        | AMD_FMT_MOD_TILE_GFX9_64K_S_X
        | AMD_FMT_MOD_TILE_GFX9_64K_D_X
        | AMD_FMT_MOD_TILE_GFX9_64K_R_X => 16,
        AMD_FMT_MOD_TILE_GFX11_256K_R_X => 18,
        other => panic!("unknown GFX9 tile mode {other}"),
    }
}

fn gfx12_block_log2(modifier: u64) -> u32 {
    match tile(modifier) {
        AMD_FMT_MOD_TILE_GFX12_256B_2D => 8,
        AMD_FMT_MOD_TILE_GFX12_4K_2D => 12,
        AMD_FMT_MOD_TILE_GFX12_64K_2D => 16,
        AMD_FMT_MOD_TILE_GFX12_256K_2D => 18,
        other => panic!("unknown GFX12 tile mode {other}"),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BlockDims {
    pub width: u32,
    pub height: u32,
}

pub fn block_dims(block_log2: u32, element_log2: u32) -> BlockDims {
    let log2 = block_log2
        .checked_sub(element_log2)
        .unwrap_or_else(|| panic!("an element of 2^{element_log2} bytes exceeds the block"));
    let width = 1 << log2.div_ceil(2);
    BlockDims {
        width,
        height: (1 << log2) / width,
    }
}
