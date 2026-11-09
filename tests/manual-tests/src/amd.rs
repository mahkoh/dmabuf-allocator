use crate::complete_id;
pub use fake_kernel::amd::devices::DEVICES;
pub use fake_kernel::amd::devices::Device as DeviceValues;
pub use fake_kernel::amd::devices::IpVersion;
pub use fake_kernel::amd::kernel::Device as AmdKernel;
use fake_kernel::uapi::amdgpu::AMDGPU_TILING_TILE_SPLIT_MASK;
use fake_kernel::uapi::amdgpu::AMDGPU_TILING_TILE_SPLIT_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_BANK_HEIGHT_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_BANK_HEIGHT_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_BANK_WIDTH_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_BANK_WIDTH_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_BANK_XOR_BITS_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_BANK_XOR_BITS_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_CONSTANT_ENCODE_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_CONSTANT_ENCODE_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_INDEPENDENT_64B_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_INDEPENDENT_64B_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_INDEPENDENT_128B_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_INDEPENDENT_128B_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_MAX_COMPRESSED_BLOCK_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_MAX_COMPRESSED_BLOCK_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_PIPE_ALIGN_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_PIPE_ALIGN_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_RETILE_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_RETILE_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_DCC_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_MACRO_TILE_ASPECT_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_MACRO_TILE_ASPECT_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_MICROTILE_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_MICROTILE_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_NUM_BANKS_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_NUM_BANKS_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PACKERS_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PACKERS_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_XOR_BITS_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_XOR_BITS_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_RB_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_RB_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_SPLIT_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_SPLIT_SHIFT;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VER_GFX6;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VERSION_MASK;
use fake_kernel::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VERSION_SHIFT;
use std::ops::Range;

pub const ARRAY_LINEAR_ALIGNED: u64 = 1;
pub const ARRAY_1D_TILED_THIN1: u64 = 2;
pub const ARRAY_2D_TILED_THIN1: u64 = 4;

pub const ADDR_SURF_P2: u64 = 0;
pub const ADDR_SURF_P2_RESERVED0: u64 = 1;
pub const ADDR_SURF_P4_16X16: u64 = 5;
pub const ADDR_SURF_P8_32X32_8X16: u64 = 10;
pub const ADDR_SURF_P16_32X32_16X16: u64 = 17;

pub const ADDR_SURF_TILE_SPLIT_64B: u64 = 0;
pub const ADDR_SURF_TILE_SPLIT_256B: u64 = 2;
pub const ADDR_SURF_TILE_SPLIT_512B: u64 = 3;
pub const ADDR_SURF_TILE_SPLIT_1KB: u64 = 4;
pub const ADDR_SURF_TILE_SPLIT_2KB: u64 = 5;
pub const ADDR_SURF_TILE_SPLIT_4KB: u64 = 6;

pub const ADDR_SURF_DISPLAY_MICRO_TILING: u64 = 0;
pub const ADDR_SURF_THIN_MICRO_TILING: u64 = 1;
pub const ADDR_SURF_DEPTH_MICRO_TILING: u64 = 2;
pub const ADDR_SURF_THICK_MICRO_TILING_GFX6: u64 = 3;

pub const ADDR_SURF_BANK_WIDTH_1: u64 = 0;
pub const ADDR_SURF_BANK_WIDTH_2: u64 = 1;
pub const ADDR_SURF_BANK_HEIGHT_1: u64 = 0;
pub const ADDR_SURF_BANK_HEIGHT_2: u64 = 1;
pub const ADDR_SURF_BANK_HEIGHT_4: u64 = 2;
pub const ADDR_SURF_BANK_HEIGHT_8: u64 = 3;
pub const ADDR_SURF_MACRO_ASPECT_1: u64 = 0;
pub const ADDR_SURF_MACRO_ASPECT_2: u64 = 1;
pub const ADDR_SURF_8_BANK: u64 = 2;
pub const ADDR_SURF_16_BANK: u64 = 3;

const AMDGPU_TILING_ARRAY_MODE_SHIFT: u64 = 0;
const AMDGPU_TILING_ARRAY_MODE_MASK: u64 = 0xf;
const AMDGPU_TILING_PIPE_CONFIG_SHIFT: u64 = 4;
const AMDGPU_TILING_PIPE_CONFIG_MASK: u64 = 0x1f;
const AMDGPU_TILING_MICRO_TILE_MODE_SHIFT: u64 = 12;
const AMDGPU_TILING_MICRO_TILE_MODE_MASK: u64 = 0x7;
const AMDGPU_TILING_BANK_WIDTH_SHIFT: u64 = 15;
const AMDGPU_TILING_BANK_WIDTH_MASK: u64 = 0x3;
const AMDGPU_TILING_BANK_HEIGHT_SHIFT: u64 = 17;
const AMDGPU_TILING_BANK_HEIGHT_MASK: u64 = 0x3;
const AMDGPU_TILING_MACRO_TILE_ASPECT_SHIFT: u64 = 19;
const AMDGPU_TILING_MACRO_TILE_ASPECT_MASK: u64 = 0x3;
const AMDGPU_TILING_NUM_BANKS_SHIFT: u64 = 21;
const AMDGPU_TILING_NUM_BANKS_MASK: u64 = 0x3;
const AMDGPU_TILING_SWIZZLE_MODE_SHIFT: u64 = 0;
const AMDGPU_TILING_SWIZZLE_MODE_MASK: u64 = 0x1f;
const AMDGPU_TILING_DCC_OFFSET_256B_SHIFT: u64 = 5;
const AMDGPU_TILING_DCC_OFFSET_256B_MASK: u64 = 0xffffff;
const AMDGPU_TILING_DCC_PITCH_MAX_SHIFT: u64 = 29;
const AMDGPU_TILING_DCC_PITCH_MAX_MASK: u64 = 0x3fff;
const AMDGPU_TILING_DCC_INDEPENDENT_64B_SHIFT: u64 = 43;
const AMDGPU_TILING_DCC_INDEPENDENT_64B_MASK: u64 = 0x1;
const AMDGPU_TILING_DCC_INDEPENDENT_128B_SHIFT: u64 = 44;
const AMDGPU_TILING_DCC_INDEPENDENT_128B_MASK: u64 = 0x1;
const AMDGPU_TILING_DCC_MAX_COMPRESSED_BLOCK_SIZE_SHIFT: u64 = 45;
const AMDGPU_TILING_DCC_MAX_COMPRESSED_BLOCK_SIZE_MASK: u64 = 0x3;
const AMDGPU_TILING_SCANOUT_SHIFT: u64 = 63;
const AMDGPU_TILING_SCANOUT_MASK: u64 = 0x1;
const AMDGPU_TILING_GFX12_SWIZZLE_MODE_SHIFT: u64 = 0;
const AMDGPU_TILING_GFX12_SWIZZLE_MODE_MASK: u64 = 0x7;
const AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK_SHIFT: u64 = 3;
const AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK_MASK: u64 = 0x3;
const AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE_SHIFT: u64 = 5;
const AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE_MASK: u64 = 0x7;
const AMDGPU_TILING_GFX12_DCC_DATA_FORMAT_SHIFT: u64 = 8;
const AMDGPU_TILING_GFX12_DCC_DATA_FORMAT_MASK: u64 = 0x3f;
const AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE_SHIFT: u64 = 14;
const AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE_MASK: u64 = 0x1;
const AMDGPU_TILING_GFX12_SCANOUT_SHIFT: u64 = 63;
const AMDGPU_TILING_GFX12_SCANOUT_MASK: u64 = 0x1;

pub const AMDGPU_IDS_FLAGS_FUSION: u64 = 0x1;

pub const FAMILY_AI: u32 = 0x8d;
pub const FAMILY_NV: u32 = 0x8f;
pub const AMDGPU_MI100_RANGE: Range<u32> = 0x32..0x3c;
pub const AMDGPU_GFX1013_RANGE: Range<u32> = 0x82..0x86;

pub const COLOR_INVALID: u64 = 0;
pub const COLOR_8_8_8_8: u64 = 10;
pub const NUMBER_UNORM: u64 = 0;
pub const NUMBER_FLOAT: u64 = 7;

const ATI_VENDOR_ID: u32 = 0x1002;
pub const IMG_DATA_FORMAT_8_8_8_8: u32 = 10;
pub const IMG_DATA_FORMAT_16_16_16_16: u32 = 12;
pub const IMG_NUM_FORMAT_UNORM: u32 = 0;
pub const IMG_NUM_FORMAT_FLOAT: u32 = 7;
pub const SQ_SEL_X: u32 = 4;
pub const SQ_SEL_Y: u32 = 5;
pub const SQ_SEL_Z: u32 = 6;
pub const SQ_SEL_W: u32 = 7;
pub const SQ_RSRC_IMG_2D: u32 = 9;
const C_008F30_MAX_ANISO_RATIO: u32 = 0xfffff1ff;
const METADATA_FORMAT_VERSION: u32 = 1;

const S_008F14_DATA_FORMAT_SHIFT: u32 = 20;
const S_008F14_DATA_FORMAT_MASK: u32 = 0x3f;
const S_008F14_NUM_FORMAT_SHIFT: u32 = 26;
const S_008F14_NUM_FORMAT_MASK: u32 = 0xf;
const S_008F18_WIDTH_SHIFT: u32 = 0;
const S_008F18_WIDTH_MASK: u32 = 0x3fff;
const S_008F18_HEIGHT_SHIFT: u32 = 14;
const S_008F18_HEIGHT_MASK: u32 = 0x3fff;
const S_008F18_PERF_MOD_SHIFT: u32 = 28;
const S_008F18_PERF_MOD_MASK: u32 = 0x7;
const S_008F1C_DST_SEL_X_SHIFT: u32 = 0;
const S_008F1C_DST_SEL_Y_SHIFT: u32 = 3;
const S_008F1C_DST_SEL_Z_SHIFT: u32 = 6;
const S_008F1C_DST_SEL_W_SHIFT: u32 = 9;
const S_008F1C_DST_SEL_MASK: u32 = 0x7;
const S_008F1C_TILING_INDEX_SHIFT: u32 = 20;
const S_008F1C_TILING_INDEX_MASK: u32 = 0x1f;
const S_008F1C_TYPE_SHIFT: u32 = 28;
const S_008F1C_TYPE_MASK: u32 = 0xf;
const S_008F20_PITCH_SHIFT: u32 = 13;
const S_008F20_PITCH_MASK: u32 = 0xffff;
const S_008F28_COMPRESSION_EN_SHIFT: u32 = 21;
const S_008F28_COMPRESSION_EN_MASK: u32 = 0x1;
const S_008F28_ALPHA_IS_ON_MSB_SHIFT: u32 = 22;
const S_008F28_ALPHA_IS_ON_MSB_MASK: u32 = 0x1;

#[derive(Copy, Clone)]
pub struct Bits {
    pub shift: u32,
    pub width: u32,
}

impl Bits {
    pub const fn mask(self) -> u32 {
        ((1 << self.width) - 1) << self.shift
    }

    pub fn with(self, register: u32, value: u64) -> u32 {
        let value = u32::try_from(value).unwrap() << self.shift;
        assert!(value & !self.mask() == 0);
        register & !self.mask() | value
    }

    pub const fn bit_above(self) -> u32 {
        1 << (self.shift + self.width)
    }
}

pub const GB_TILE_MODE_MICRO_TILE_MODE: Bits = Bits { shift: 0, width: 2 };
pub const GB_TILE_MODE_PIPE_CONFIG: Bits = Bits { shift: 6, width: 5 };
pub const GB_TILE_MODE_TILE_SPLIT: Bits = Bits {
    shift: 11,
    width: 3,
};
pub const GB_TILE_MODE_BANK_WIDTH: Bits = Bits {
    shift: 14,
    width: 2,
};
pub const GB_TILE_MODE_NUM_BANKS: Bits = Bits {
    shift: 20,
    width: 2,
};
pub const GB_TILE_MODE_MICRO_TILE_MODE_NEW: Bits = Bits {
    shift: 22,
    width: 3,
};
pub const GB_TILE_MODE_SAMPLE_SPLIT: Bits = Bits {
    shift: 25,
    width: 2,
};

pub const GB_MACROTILE_MODE_BANK_WIDTH: Bits = Bits { shift: 0, width: 2 };
pub const GB_MACROTILE_MODE_BANK_HEIGHT: Bits = Bits { shift: 2, width: 2 };
pub const GB_MACROTILE_MODE_MACRO_TILE_ASPECT: Bits = Bits { shift: 4, width: 2 };
pub const GB_MACROTILE_MODE_NUM_BANKS: Bits = Bits { shift: 6, width: 2 };

pub const GFX6_GB_ADDR_CONFIG_ROW_SIZE: Bits = Bits {
    shift: 28,
    width: 2,
};

pub const GFX9_GB_ADDR_CONFIG_NUM_PIPES: Bits = Bits { shift: 0, width: 3 };
pub const GFX9_GB_ADDR_CONFIG_NUM_BANKS: Bits = Bits {
    shift: 12,
    width: 3,
};
pub const GFX9_GB_ADDR_CONFIG_NUM_SHADER_ENGINES: Bits = Bits {
    shift: 19,
    width: 2,
};
pub const GFX9_GB_ADDR_CONFIG_NUM_RB_PER_SE: Bits = Bits {
    shift: 26,
    width: 2,
};

pub const GFX10_GB_ADDR_CONFIG_NUM_PIPES: Bits = Bits { shift: 0, width: 3 };
pub const GFX10_GB_ADDR_CONFIG_NUM_PKRS: Bits = Bits { shift: 8, width: 3 };

pub const IP_DISCOVERY_VERSION_MAJOR: Bits = Bits {
    shift: 16,
    width: 8,
};
const IP_DISCOVERY_VERSION_MINOR: Bits = Bits { shift: 8, width: 8 };

pub const TILEINDEX_LINEAR_ALIGNED: u32 = 8;
pub const GFX6_TILE_INDEX_1D_DISPLAY: u32 = 9;
pub const GFX6_TILE_INDEX_2D_DISPLAY_32BPP: u32 = 12;
pub const GFX6_TILE_INDEX_1D_THIN: u32 = 13;
pub const GFX6_TILE_INDEX_2D_THIN_8BPP: u32 = 14;
pub const GFX6_TILE_INDEX_2D_THIN_16BPP: u32 = 15;
pub const GFX6_TILE_INDEX_2D_THIN_32BPP: u32 = 16;
pub const GFX7_TILE_INDEX_2D_DISPLAY: u32 = 10;
pub const GFX7_TILE_INDEX_2D_THIN: u32 = 14;

const UMD_DESCRIPTOR_WORD: usize = 2;
const DESCRIPTOR_WORD_008F14: usize = 1;
const DESCRIPTOR_WORD_008F1C: usize = 3;

const fn set(value: u64, shift: u32, mask: u64) -> u64 {
    assert!(value & !mask == 0);
    value << shift
}

const fn set32(value: u32, shift: u32, mask: u32) -> u32 {
    assert!(value & !mask == 0);
    value << shift
}

const fn get32(word: u32, shift: u32, mask: u32) -> u32 {
    (word >> shift) & mask
}

const fn tiling_set(value: u64, shift: u64, mask: u64) -> u64 {
    assert!(value & !mask == 0);
    value << shift
}

const fn tiling_get(tiling_info: u64, shift: u64, mask: u64) -> u64 {
    (tiling_info >> shift) & mask
}

pub fn kernel(id: &str) -> AmdKernel {
    let id = complete_id(id);
    let fields = id
        .strip_prefix("amdgpu:")
        .unwrap_or_else(|| panic!("{id} is not an amdgpu device"));
    AmdKernel::from_id(fields)
}

pub fn ip_discovery_version(version: IpVersion) -> u32 {
    IP_DISCOVERY_VERSION_MAJOR.with(0, u64::from(version.major))
        | IP_DISCOVERY_VERSION_MINOR.with(0, u64::from(version.minor))
}

#[derive(Copy, Clone)]
pub struct Gfx6Modifier {
    pub tile: u64,
    pub dcc: bool,
    pub microtile: u64,
    pub pipe_config: u64,
    pub tile_split: u64,
    pub bank_width: u64,
    pub bank_height: u64,
    pub macro_tile_aspect: u64,
    pub num_banks: u64,
}

impl Gfx6Modifier {
    pub const NONE: Self = Self {
        tile: 0,
        dcc: false,
        microtile: 0,
        pipe_config: 0,
        tile_split: 0,
        bank_width: 0,
        bank_height: 0,
        macro_tile_aspect: 0,
        num_banks: 0,
    };

    pub const fn encode(&self) -> u64 {
        AMD_FMT_MOD
            | set(
                AMD_FMT_MOD_TILE_VER_GFX6,
                AMD_FMT_MOD_TILE_VERSION_SHIFT,
                AMD_FMT_MOD_TILE_VERSION_MASK,
            )
            | set(self.tile, AMD_FMT_MOD_TILE_SHIFT, AMD_FMT_MOD_TILE_MASK)
            | set(self.dcc as u64, AMD_FMT_MOD_DCC_SHIFT, AMD_FMT_MOD_DCC_MASK)
            | set(
                self.microtile,
                AMD_FMT_MOD_MICROTILE_SHIFT,
                AMD_FMT_MOD_MICROTILE_MASK,
            )
            | set(
                self.pipe_config,
                AMD_FMT_MOD_PIPE_CONFIG_SHIFT,
                AMD_FMT_MOD_PIPE_CONFIG_MASK,
            )
            | set(
                self.tile_split,
                AMD_FMT_MOD_TILE_SPLIT_SHIFT,
                AMD_FMT_MOD_TILE_SPLIT_MASK,
            )
            | set(
                self.bank_width,
                AMD_FMT_MOD_BANK_WIDTH_SHIFT,
                AMD_FMT_MOD_BANK_WIDTH_MASK,
            )
            | set(
                self.bank_height,
                AMD_FMT_MOD_BANK_HEIGHT_SHIFT,
                AMD_FMT_MOD_BANK_HEIGHT_MASK,
            )
            | set(
                self.macro_tile_aspect,
                AMD_FMT_MOD_MACRO_TILE_ASPECT_SHIFT,
                AMD_FMT_MOD_MACRO_TILE_ASPECT_MASK,
            )
            | set(
                self.num_banks,
                AMD_FMT_MOD_NUM_BANKS_SHIFT,
                AMD_FMT_MOD_NUM_BANKS_MASK,
            )
    }
}

#[derive(Copy, Clone)]
pub struct Gfx9Modifier {
    pub tile_version: u64,
    pub tile: u64,
    pub dcc: bool,
    pub dcc_retile: bool,
    pub dcc_pipe_align: bool,
    pub dcc_independent_64b: bool,
    pub dcc_independent_128b: bool,
    pub dcc_max_compressed_block: u64,
    pub dcc_constant_encode: bool,
    pub pipe_xor_bits: u64,
    pub bank_xor_bits: u64,
    pub packers: u64,
    pub rb: u64,
    pub pipe: u64,
}

impl Gfx9Modifier {
    pub const NONE: Self = Self {
        tile_version: 0,
        tile: 0,
        dcc: false,
        dcc_retile: false,
        dcc_pipe_align: false,
        dcc_independent_64b: false,
        dcc_independent_128b: false,
        dcc_max_compressed_block: 0,
        dcc_constant_encode: false,
        pipe_xor_bits: 0,
        bank_xor_bits: 0,
        packers: 0,
        rb: 0,
        pipe: 0,
    };

    pub const fn encode(&self) -> u64 {
        assert!(self.tile_version != AMD_FMT_MOD_TILE_VER_GFX6);
        AMD_FMT_MOD
            | set(
                self.tile_version,
                AMD_FMT_MOD_TILE_VERSION_SHIFT,
                AMD_FMT_MOD_TILE_VERSION_MASK,
            )
            | set(self.tile, AMD_FMT_MOD_TILE_SHIFT, AMD_FMT_MOD_TILE_MASK)
            | set(self.dcc as u64, AMD_FMT_MOD_DCC_SHIFT, AMD_FMT_MOD_DCC_MASK)
            | set(
                self.dcc_retile as u64,
                AMD_FMT_MOD_DCC_RETILE_SHIFT,
                AMD_FMT_MOD_DCC_RETILE_MASK,
            )
            | set(
                self.dcc_pipe_align as u64,
                AMD_FMT_MOD_DCC_PIPE_ALIGN_SHIFT,
                AMD_FMT_MOD_DCC_PIPE_ALIGN_MASK,
            )
            | set(
                self.dcc_independent_64b as u64,
                AMD_FMT_MOD_DCC_INDEPENDENT_64B_SHIFT,
                AMD_FMT_MOD_DCC_INDEPENDENT_64B_MASK,
            )
            | set(
                self.dcc_independent_128b as u64,
                AMD_FMT_MOD_DCC_INDEPENDENT_128B_SHIFT,
                AMD_FMT_MOD_DCC_INDEPENDENT_128B_MASK,
            )
            | set(
                self.dcc_max_compressed_block,
                AMD_FMT_MOD_DCC_MAX_COMPRESSED_BLOCK_SHIFT,
                AMD_FMT_MOD_DCC_MAX_COMPRESSED_BLOCK_MASK,
            )
            | set(
                self.dcc_constant_encode as u64,
                AMD_FMT_MOD_DCC_CONSTANT_ENCODE_SHIFT,
                AMD_FMT_MOD_DCC_CONSTANT_ENCODE_MASK,
            )
            | set(
                self.pipe_xor_bits,
                AMD_FMT_MOD_PIPE_XOR_BITS_SHIFT,
                AMD_FMT_MOD_PIPE_XOR_BITS_MASK,
            )
            | set(
                self.bank_xor_bits,
                AMD_FMT_MOD_BANK_XOR_BITS_SHIFT,
                AMD_FMT_MOD_BANK_XOR_BITS_MASK,
            )
            | set(
                self.packers,
                AMD_FMT_MOD_PACKERS_SHIFT,
                AMD_FMT_MOD_PACKERS_MASK,
            )
            | set(self.rb, AMD_FMT_MOD_RB_SHIFT, AMD_FMT_MOD_RB_MASK)
            | set(self.pipe, AMD_FMT_MOD_PIPE_SHIFT, AMD_FMT_MOD_PIPE_MASK)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Gfx6Tiling {
    pub array_mode: u64,
    pub pipe_config: u64,
    pub tile_split: u64,
    pub micro_tile_mode: u64,
    pub bank_width: u64,
    pub bank_height: u64,
    pub macro_tile_aspect: u64,
    pub num_banks: u64,
}

impl Gfx6Tiling {
    pub const fn encode(&self) -> u64 {
        tiling_set(
            self.array_mode,
            AMDGPU_TILING_ARRAY_MODE_SHIFT,
            AMDGPU_TILING_ARRAY_MODE_MASK,
        ) | tiling_set(
            self.pipe_config,
            AMDGPU_TILING_PIPE_CONFIG_SHIFT,
            AMDGPU_TILING_PIPE_CONFIG_MASK,
        ) | tiling_set(
            self.tile_split,
            AMDGPU_TILING_TILE_SPLIT_SHIFT,
            AMDGPU_TILING_TILE_SPLIT_MASK,
        ) | tiling_set(
            self.micro_tile_mode,
            AMDGPU_TILING_MICRO_TILE_MODE_SHIFT,
            AMDGPU_TILING_MICRO_TILE_MODE_MASK,
        ) | tiling_set(
            self.bank_width,
            AMDGPU_TILING_BANK_WIDTH_SHIFT,
            AMDGPU_TILING_BANK_WIDTH_MASK,
        ) | tiling_set(
            self.bank_height,
            AMDGPU_TILING_BANK_HEIGHT_SHIFT,
            AMDGPU_TILING_BANK_HEIGHT_MASK,
        ) | tiling_set(
            self.macro_tile_aspect,
            AMDGPU_TILING_MACRO_TILE_ASPECT_SHIFT,
            AMDGPU_TILING_MACRO_TILE_ASPECT_MASK,
        ) | tiling_set(
            self.num_banks,
            AMDGPU_TILING_NUM_BANKS_SHIFT,
            AMDGPU_TILING_NUM_BANKS_MASK,
        )
    }

    pub const fn decode(tiling_info: u64) -> Self {
        Self {
            array_mode: tiling_get(
                tiling_info,
                AMDGPU_TILING_ARRAY_MODE_SHIFT,
                AMDGPU_TILING_ARRAY_MODE_MASK,
            ),
            pipe_config: tiling_get(
                tiling_info,
                AMDGPU_TILING_PIPE_CONFIG_SHIFT,
                AMDGPU_TILING_PIPE_CONFIG_MASK,
            ),
            tile_split: tiling_get(
                tiling_info,
                AMDGPU_TILING_TILE_SPLIT_SHIFT,
                AMDGPU_TILING_TILE_SPLIT_MASK,
            ),
            micro_tile_mode: tiling_get(
                tiling_info,
                AMDGPU_TILING_MICRO_TILE_MODE_SHIFT,
                AMDGPU_TILING_MICRO_TILE_MODE_MASK,
            ),
            bank_width: tiling_get(
                tiling_info,
                AMDGPU_TILING_BANK_WIDTH_SHIFT,
                AMDGPU_TILING_BANK_WIDTH_MASK,
            ),
            bank_height: tiling_get(
                tiling_info,
                AMDGPU_TILING_BANK_HEIGHT_SHIFT,
                AMDGPU_TILING_BANK_HEIGHT_MASK,
            ),
            macro_tile_aspect: tiling_get(
                tiling_info,
                AMDGPU_TILING_MACRO_TILE_ASPECT_SHIFT,
                AMDGPU_TILING_MACRO_TILE_ASPECT_MASK,
            ),
            num_banks: tiling_get(
                tiling_info,
                AMDGPU_TILING_NUM_BANKS_SHIFT,
                AMDGPU_TILING_NUM_BANKS_MASK,
            ),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Gfx9Tiling {
    pub swizzle_mode: u64,
    pub dcc_offset_256b: u64,
    pub dcc_pitch_max: u64,
    pub dcc_independent_64b: bool,
    pub dcc_independent_128b: bool,
    pub dcc_max_compressed_block_size: u64,
    pub scanout: bool,
}

impl Gfx9Tiling {
    pub const fn encode(&self) -> u64 {
        tiling_set(
            self.swizzle_mode,
            AMDGPU_TILING_SWIZZLE_MODE_SHIFT,
            AMDGPU_TILING_SWIZZLE_MODE_MASK,
        ) | tiling_set(
            self.dcc_offset_256b,
            AMDGPU_TILING_DCC_OFFSET_256B_SHIFT,
            AMDGPU_TILING_DCC_OFFSET_256B_MASK,
        ) | tiling_set(
            self.dcc_pitch_max,
            AMDGPU_TILING_DCC_PITCH_MAX_SHIFT,
            AMDGPU_TILING_DCC_PITCH_MAX_MASK,
        ) | tiling_set(
            self.dcc_independent_64b as u64,
            AMDGPU_TILING_DCC_INDEPENDENT_64B_SHIFT,
            AMDGPU_TILING_DCC_INDEPENDENT_64B_MASK,
        ) | tiling_set(
            self.dcc_independent_128b as u64,
            AMDGPU_TILING_DCC_INDEPENDENT_128B_SHIFT,
            AMDGPU_TILING_DCC_INDEPENDENT_128B_MASK,
        ) | tiling_set(
            self.dcc_max_compressed_block_size,
            AMDGPU_TILING_DCC_MAX_COMPRESSED_BLOCK_SIZE_SHIFT,
            AMDGPU_TILING_DCC_MAX_COMPRESSED_BLOCK_SIZE_MASK,
        ) | tiling_set(
            self.scanout as u64,
            AMDGPU_TILING_SCANOUT_SHIFT,
            AMDGPU_TILING_SCANOUT_MASK,
        )
    }

    pub const fn decode(tiling_info: u64) -> Self {
        Self {
            swizzle_mode: tiling_get(
                tiling_info,
                AMDGPU_TILING_SWIZZLE_MODE_SHIFT,
                AMDGPU_TILING_SWIZZLE_MODE_MASK,
            ),
            dcc_offset_256b: tiling_get(
                tiling_info,
                AMDGPU_TILING_DCC_OFFSET_256B_SHIFT,
                AMDGPU_TILING_DCC_OFFSET_256B_MASK,
            ),
            dcc_pitch_max: tiling_get(
                tiling_info,
                AMDGPU_TILING_DCC_PITCH_MAX_SHIFT,
                AMDGPU_TILING_DCC_PITCH_MAX_MASK,
            ),
            dcc_independent_64b: tiling_get(
                tiling_info,
                AMDGPU_TILING_DCC_INDEPENDENT_64B_SHIFT,
                AMDGPU_TILING_DCC_INDEPENDENT_64B_MASK,
            ) != 0,
            dcc_independent_128b: tiling_get(
                tiling_info,
                AMDGPU_TILING_DCC_INDEPENDENT_128B_SHIFT,
                AMDGPU_TILING_DCC_INDEPENDENT_128B_MASK,
            ) != 0,
            dcc_max_compressed_block_size: tiling_get(
                tiling_info,
                AMDGPU_TILING_DCC_MAX_COMPRESSED_BLOCK_SIZE_SHIFT,
                AMDGPU_TILING_DCC_MAX_COMPRESSED_BLOCK_SIZE_MASK,
            ),
            scanout: tiling_get(
                tiling_info,
                AMDGPU_TILING_SCANOUT_SHIFT,
                AMDGPU_TILING_SCANOUT_MASK,
            ) != 0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Gfx12Tiling {
    pub swizzle_mode: u64,
    pub dcc_max_compressed_block: u64,
    pub dcc_number_type: u64,
    pub dcc_data_format: u64,
    pub dcc_write_compress_disable: bool,
    pub scanout: bool,
}

impl Gfx12Tiling {
    pub const fn encode(&self) -> u64 {
        tiling_set(
            self.swizzle_mode,
            AMDGPU_TILING_GFX12_SWIZZLE_MODE_SHIFT,
            AMDGPU_TILING_GFX12_SWIZZLE_MODE_MASK,
        ) | tiling_set(
            self.dcc_max_compressed_block,
            AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK_SHIFT,
            AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK_MASK,
        ) | tiling_set(
            self.dcc_number_type,
            AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE_SHIFT,
            AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE_MASK,
        ) | tiling_set(
            self.dcc_data_format,
            AMDGPU_TILING_GFX12_DCC_DATA_FORMAT_SHIFT,
            AMDGPU_TILING_GFX12_DCC_DATA_FORMAT_MASK,
        ) | tiling_set(
            self.dcc_write_compress_disable as u64,
            AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE_SHIFT,
            AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE_MASK,
        ) | tiling_set(
            self.scanout as u64,
            AMDGPU_TILING_GFX12_SCANOUT_SHIFT,
            AMDGPU_TILING_GFX12_SCANOUT_MASK,
        )
    }

    pub const fn decode(tiling_info: u64) -> Self {
        Self {
            swizzle_mode: tiling_get(
                tiling_info,
                AMDGPU_TILING_GFX12_SWIZZLE_MODE_SHIFT,
                AMDGPU_TILING_GFX12_SWIZZLE_MODE_MASK,
            ),
            dcc_max_compressed_block: tiling_get(
                tiling_info,
                AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK_SHIFT,
                AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK_MASK,
            ),
            dcc_number_type: tiling_get(
                tiling_info,
                AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE_SHIFT,
                AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE_MASK,
            ),
            dcc_data_format: tiling_get(
                tiling_info,
                AMDGPU_TILING_GFX12_DCC_DATA_FORMAT_SHIFT,
                AMDGPU_TILING_GFX12_DCC_DATA_FORMAT_MASK,
            ),
            dcc_write_compress_disable: tiling_get(
                tiling_info,
                AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE_SHIFT,
                AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE_MASK,
            ) != 0,
            scanout: tiling_get(
                tiling_info,
                AMDGPU_TILING_GFX12_SCANOUT_SHIFT,
                AMDGPU_TILING_GFX12_SCANOUT_MASK,
            ) != 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Descriptor {
    pub data_format: u32,
    pub num_format: u32,
    pub width: u32,
    pub height: u32,
    pub perf_mod: u32,
    pub dst_sel_x: u32,
    pub dst_sel_y: u32,
    pub dst_sel_z: u32,
    pub dst_sel_w: u32,
    pub tiling_index: u32,
    pub kind: u32,
    pub pitch: u32,
    pub compression_en: bool,
    pub alpha_is_on_msb: bool,
    pub word7: Word7,
}

impl Descriptor {
    pub fn encode(&self) -> [u32; 8] {
        [
            0,
            set32(
                self.data_format,
                S_008F14_DATA_FORMAT_SHIFT,
                S_008F14_DATA_FORMAT_MASK,
            ) | set32(
                self.num_format,
                S_008F14_NUM_FORMAT_SHIFT,
                S_008F14_NUM_FORMAT_MASK,
            ),
            set32(self.width - 1, S_008F18_WIDTH_SHIFT, S_008F18_WIDTH_MASK)
                | set32(self.height - 1, S_008F18_HEIGHT_SHIFT, S_008F18_HEIGHT_MASK)
                | set32(
                    self.perf_mod,
                    S_008F18_PERF_MOD_SHIFT,
                    S_008F18_PERF_MOD_MASK,
                ),
            set32(
                self.dst_sel_x,
                S_008F1C_DST_SEL_X_SHIFT,
                S_008F1C_DST_SEL_MASK,
            ) | set32(
                self.dst_sel_y,
                S_008F1C_DST_SEL_Y_SHIFT,
                S_008F1C_DST_SEL_MASK,
            ) | set32(
                self.dst_sel_z,
                S_008F1C_DST_SEL_Z_SHIFT,
                S_008F1C_DST_SEL_MASK,
            ) | set32(
                self.dst_sel_w,
                S_008F1C_DST_SEL_W_SHIFT,
                S_008F1C_DST_SEL_MASK,
            ) | set32(
                self.tiling_index,
                S_008F1C_TILING_INDEX_SHIFT,
                S_008F1C_TILING_INDEX_MASK,
            ) | set32(self.kind, S_008F1C_TYPE_SHIFT, S_008F1C_TYPE_MASK),
            set32(self.pitch - 1, S_008F20_PITCH_SHIFT, S_008F20_PITCH_MASK),
            0,
            set32(
                self.compression_en as u32,
                S_008F28_COMPRESSION_EN_SHIFT,
                S_008F28_COMPRESSION_EN_MASK,
            ) | set32(
                self.alpha_is_on_msb as u32,
                S_008F28_ALPHA_IS_ON_MSB_SHIFT,
                S_008F28_ALPHA_IS_ON_MSB_MASK,
            ),
            self.word7.encode(),
        ]
    }
}

#[derive(Copy, Clone)]
pub struct UmdMetadata {
    pub pci_id: u32,
    pub descriptor: Descriptor,
    pub level_offset_256b: u32,
}

impl UmdMetadata {
    pub fn encode(&self) -> [u32; 11] {
        let desc = self.descriptor.encode();
        [
            METADATA_FORMAT_VERSION,
            ATI_VENDOR_ID << 16 | self.pci_id,
            desc[0],
            desc[1],
            desc[2],
            desc[3],
            desc[4],
            desc[5],
            desc[6],
            desc[7],
            self.level_offset_256b,
        ]
    }

    pub fn data_format(data: &[u32]) -> u32 {
        get32(
            data[UMD_DESCRIPTOR_WORD + DESCRIPTOR_WORD_008F14],
            S_008F14_DATA_FORMAT_SHIFT,
            S_008F14_DATA_FORMAT_MASK,
        )
    }

    pub fn num_format(data: &[u32]) -> u32 {
        get32(
            data[UMD_DESCRIPTOR_WORD + DESCRIPTOR_WORD_008F14],
            S_008F14_NUM_FORMAT_SHIFT,
            S_008F14_NUM_FORMAT_MASK,
        )
    }

    pub fn tiling_index(data: &[u32]) -> u32 {
        get32(
            data[UMD_DESCRIPTOR_WORD + DESCRIPTOR_WORD_008F1C],
            S_008F1C_TILING_INDEX_SHIFT,
            S_008F1C_TILING_INDEX_MASK,
        )
    }
}

#[derive(Copy, Clone)]
pub enum Word7 {
    Zero,
    MaxAnisoRatioCleared,
    MetaOffset(u64),
}

impl Word7 {
    pub fn encode(&self) -> u32 {
        match *self {
            Word7::Zero => 0,
            Word7::MaxAnisoRatioCleared => C_008F30_MAX_ANISO_RATIO,
            Word7::MetaOffset(offset) => u32::try_from(offset >> 8).unwrap(),
        }
    }
}
