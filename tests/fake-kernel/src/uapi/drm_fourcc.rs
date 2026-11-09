pub const fn fourcc_code(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24)
}

pub const DRM_FORMAT_MOD_VENDOR_NONE: u64 = 0;
pub const DRM_FORMAT_MOD_VENDOR_INTEL: u64 = 0x01;
pub const DRM_FORMAT_MOD_VENDOR_AMD: u64 = 0x02;
pub const DRM_FORMAT_MOD_VENDOR_NVIDIA: u64 = 0x03;
pub const DRM_FORMAT_MOD_VENDOR_VIVANTE: u64 = 0x06;
pub const DRM_FORMAT_MOD_VENDOR_BROADCOM: u64 = 0x07;
pub const DRM_FORMAT_MOD_VENDOR_ARM: u64 = 0x08;
pub const DRM_FORMAT_MOD_VENDOR_APPLE: u64 = 0x0c;

pub const DRM_FORMAT_RESERVED: u64 = (1 << 56) - 1;

pub const fn fourcc_mod_code(vendor: u64, val: u64) -> u64 {
    (vendor << 56) | (val & 0x00ffffffffffffff)
}

pub const DRM_FORMAT_MOD_INVALID: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_NONE, DRM_FORMAT_RESERVED);

pub const DRM_FORMAT_MOD_LINEAR: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_NONE, 0);

pub const I915_FORMAT_MOD_X_TILED: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 1);
pub const I915_FORMAT_MOD_Y_TILED: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 2);
pub const I915_FORMAT_MOD_Yf_TILED: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 3);
pub const I915_FORMAT_MOD_Y_TILED_CCS: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 4);
pub const I915_FORMAT_MOD_Yf_TILED_CCS: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 5);
pub const I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 6);
pub const I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 7);
pub const I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 8);
pub const I915_FORMAT_MOD_4_TILED: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 9);
pub const I915_FORMAT_MOD_4_TILED_DG2_RC_CCS: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 10);
pub const I915_FORMAT_MOD_4_TILED_DG2_MC_CCS: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 11);
pub const I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 12);
pub const I915_FORMAT_MOD_4_TILED_MTL_RC_CCS: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 13);
pub const I915_FORMAT_MOD_4_TILED_MTL_MC_CCS: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 14);
pub const I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 15);
pub const I915_FORMAT_MOD_4_TILED_LNL_CCS: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 16);
pub const I915_FORMAT_MOD_4_TILED_BMG_CCS: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, 17);

pub const DRM_FORMAT_MOD_VIVANTE_TILED: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_VIVANTE, 1);
pub const DRM_FORMAT_MOD_VIVANTE_SUPER_TILED: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_VIVANTE, 2);
pub const DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_VIVANTE, 3);
pub const DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_VIVANTE, 4);

pub const fn DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(c: u64, s: u64, g: u64, k: u64, h: u64) -> u64 {
    fourcc_mod_code(
        DRM_FORMAT_MOD_VENDOR_NVIDIA,
        0x10 | (h & 0xf)
            | ((k & 0xff) << 12)
            | ((g & 0x3) << 20)
            | ((s & 0x1) << 22)
            | ((s & 0x6) << 25)
            | ((c & 0x7) << 23),
    )
}

pub const fn fourcc_mod_broadcom_code(val: u64, params: u64) -> u64 {
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_BROADCOM, (params << 8) | val)
}

pub const DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_BROADCOM, 1);

pub const fn DRM_FORMAT_MOD_BROADCOM_SAND128_COL_HEIGHT(v: u64) -> u64 {
    fourcc_mod_broadcom_code(4, v)
}

pub const DRM_FORMAT_MOD_BROADCOM_SAND128: u64 = DRM_FORMAT_MOD_BROADCOM_SAND128_COL_HEIGHT(0);

pub const DRM_FORMAT_MOD_BROADCOM_UIF: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_BROADCOM, 6);

pub const fn DRM_FORMAT_MOD_ARM_CODE(type_: u64, val: u64) -> u64 {
    fourcc_mod_code(
        DRM_FORMAT_MOD_VENDOR_ARM,
        (type_ << 52) | (val & 0x000fffffffffffff),
    )
}

pub const DRM_FORMAT_MOD_ARM_TYPE_AFBC: u64 = 0x00;
pub const DRM_FORMAT_MOD_ARM_TYPE_MISC: u64 = 0x01;

pub const fn DRM_FORMAT_MOD_ARM_AFBC(afbc_mode: u64) -> u64 {
    DRM_FORMAT_MOD_ARM_CODE(DRM_FORMAT_MOD_ARM_TYPE_AFBC, afbc_mode)
}

pub const AFBC_FORMAT_MOD_BLOCK_SIZE_16x16: u64 = 1;
pub const AFBC_FORMAT_MOD_BLOCK_SIZE_32x8: u64 = 2;
pub const AFBC_FORMAT_MOD_YTR: u64 = 1 << 4;
pub const AFBC_FORMAT_MOD_SPLIT: u64 = 1 << 5;
pub const AFBC_FORMAT_MOD_SPARSE: u64 = 1 << 6;
pub const AFBC_FORMAT_MOD_TILED: u64 = 1 << 8;
pub const AFBC_FORMAT_MOD_SC: u64 = 1 << 9;

pub const DRM_FORMAT_MOD_ARM_TYPE_AFRC: u64 = 0x02;

pub const fn DRM_FORMAT_MOD_ARM_AFRC(afrc_mode: u64) -> u64 {
    DRM_FORMAT_MOD_ARM_CODE(DRM_FORMAT_MOD_ARM_TYPE_AFRC, afrc_mode)
}

pub const AFRC_FORMAT_MOD_CU_SIZE_16: u64 = 1;
pub const AFRC_FORMAT_MOD_CU_SIZE_24: u64 = 2;
pub const AFRC_FORMAT_MOD_CU_SIZE_32: u64 = 3;

pub const fn AFRC_FORMAT_MOD_CU_SIZE_P0(afrc_cu_size: u64) -> u64 {
    afrc_cu_size
}

pub const AFRC_FORMAT_MOD_LAYOUT_SCAN: u64 = 1 << 8;

pub const DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED: u64 =
    DRM_FORMAT_MOD_ARM_CODE(DRM_FORMAT_MOD_ARM_TYPE_MISC, 1);
pub const DRM_FORMAT_MOD_ARM_INTERLEAVED_64K: u64 =
    DRM_FORMAT_MOD_ARM_CODE(DRM_FORMAT_MOD_ARM_TYPE_MISC, 2);

pub const DRM_FORMAT_MOD_APPLE_GPU_TILED: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_APPLE, 1);
pub const DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_APPLE, 2);

pub const AMD_FMT_MOD: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_AMD, 0);

pub const AMD_FMT_MOD_TILE_VER_GFX6: u64 = 0;
pub const AMD_FMT_MOD_TILE_VER_GFX9: u64 = 1;
pub const AMD_FMT_MOD_TILE_VER_GFX10: u64 = 2;
pub const AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS: u64 = 3;
pub const AMD_FMT_MOD_TILE_VER_GFX11: u64 = 4;
pub const AMD_FMT_MOD_TILE_VER_GFX12: u64 = 5;

pub const AMD_FMT_MOD_TILE_GFX6_1D_TILED_THIN1: u64 = 0x2;
pub const AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1: u64 = 0x4;
pub const AMD_FMT_MOD_TILE_GFX9_64K_S: u64 = 9;
pub const AMD_FMT_MOD_TILE_GFX9_64K_D: u64 = 10;
pub const AMD_FMT_MOD_TILE_GFX9_4K_D_X: u64 = 22;
pub const AMD_FMT_MOD_TILE_GFX9_64K_S_X: u64 = 25;
pub const AMD_FMT_MOD_TILE_GFX9_64K_D_X: u64 = 26;
pub const AMD_FMT_MOD_TILE_GFX9_64K_R_X: u64 = 27;
pub const AMD_FMT_MOD_TILE_GFX11_256K_R_X: u64 = 31;
pub const AMD_FMT_MOD_TILE_GFX12_256B_2D: u64 = 1;
pub const AMD_FMT_MOD_TILE_GFX12_4K_2D: u64 = 2;
pub const AMD_FMT_MOD_TILE_GFX12_64K_2D: u64 = 3;
pub const AMD_FMT_MOD_TILE_GFX12_256K_2D: u64 = 4;

pub const AMD_FMT_MOD_DCC_BLOCK_64B: u64 = 0;
pub const AMD_FMT_MOD_DCC_BLOCK_128B: u64 = 1;
pub const AMD_FMT_MOD_DCC_BLOCK_256B: u64 = 2;

pub const AMD_FMT_MOD_TILE_VERSION_SHIFT: u32 = 0;
pub const AMD_FMT_MOD_TILE_VERSION_MASK: u64 = 0xff;
pub const AMD_FMT_MOD_TILE_SHIFT: u32 = 8;
pub const AMD_FMT_MOD_TILE_MASK: u64 = 0x1f;
pub const AMD_FMT_MOD_DCC_SHIFT: u32 = 13;
pub const AMD_FMT_MOD_DCC_MASK: u64 = 0x1;
pub const AMD_FMT_MOD_DCC_RETILE_SHIFT: u32 = 14;
pub const AMD_FMT_MOD_DCC_RETILE_MASK: u64 = 0x1;
pub const AMD_FMT_MOD_DCC_PIPE_ALIGN_SHIFT: u32 = 15;
pub const AMD_FMT_MOD_DCC_PIPE_ALIGN_MASK: u64 = 0x1;
pub const AMD_FMT_MOD_DCC_INDEPENDENT_64B_SHIFT: u32 = 16;
pub const AMD_FMT_MOD_DCC_INDEPENDENT_64B_MASK: u64 = 0x1;
pub const AMD_FMT_MOD_DCC_INDEPENDENT_128B_SHIFT: u32 = 17;
pub const AMD_FMT_MOD_DCC_INDEPENDENT_128B_MASK: u64 = 0x1;
pub const AMD_FMT_MOD_DCC_MAX_COMPRESSED_BLOCK_SHIFT: u32 = 18;
pub const AMD_FMT_MOD_DCC_MAX_COMPRESSED_BLOCK_MASK: u64 = 0x3;
pub const AMD_FMT_MOD_DCC_CONSTANT_ENCODE_SHIFT: u32 = 20;
pub const AMD_FMT_MOD_DCC_CONSTANT_ENCODE_MASK: u64 = 0x1;
pub const AMD_FMT_MOD_PIPE_XOR_BITS_SHIFT: u32 = 21;
pub const AMD_FMT_MOD_PIPE_XOR_BITS_MASK: u64 = 0x7;
pub const AMD_FMT_MOD_BANK_XOR_BITS_SHIFT: u32 = 24;
pub const AMD_FMT_MOD_BANK_XOR_BITS_MASK: u64 = 0x7;
pub const AMD_FMT_MOD_PACKERS_SHIFT: u32 = 27;
pub const AMD_FMT_MOD_PACKERS_MASK: u64 = 0x7;
pub const AMD_FMT_MOD_RB_SHIFT: u32 = 30;
pub const AMD_FMT_MOD_RB_MASK: u64 = 0x7;
pub const AMD_FMT_MOD_PIPE_SHIFT: u32 = 33;
pub const AMD_FMT_MOD_PIPE_MASK: u64 = 0x7;

pub const AMD_FMT_MOD_MICROTILE_SHIFT: u32 = 14;
pub const AMD_FMT_MOD_MICROTILE_MASK: u64 = 0x7;

pub const AMD_FMT_MOD_MICROTILE_DISPLAY: u64 = 0x0;
pub const AMD_FMT_MOD_MICROTILE_THIN: u64 = 0x1;

pub const AMD_FMT_MOD_PIPE_CONFIG_SHIFT: u32 = 17;
pub const AMD_FMT_MOD_PIPE_CONFIG_MASK: u64 = 0x1f;

pub const AMD_FMT_MOD_PIPE_CONFIG_P2: u64 = 0x0;
pub const AMD_FMT_MOD_PIPE_CONFIG_P4_8x16: u64 = 0x4;
pub const AMD_FMT_MOD_PIPE_CONFIG_P4_16x16: u64 = 0x5;
pub const AMD_FMT_MOD_PIPE_CONFIG_P4_16x32: u64 = 0x6;
pub const AMD_FMT_MOD_PIPE_CONFIG_P4_32x32: u64 = 0x7;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_16x16_8x16: u64 = 0x8;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_16x32_8x16: u64 = 0x9;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16: u64 = 0xa;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_16x32_16x16: u64 = 0xb;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_16x16: u64 = 0xc;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_16x32: u64 = 0xd;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_32x64_32x32: u64 = 0xe;
pub const AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_8x16: u64 = 0x10;
pub const AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16: u64 = 0x11;

pub const AMD_FMT_MOD_TILE_SPLIT_SHIFT: u32 = 22;
pub const AMD_FMT_MOD_TILE_SPLIT_MASK: u64 = 0x7;
pub const AMD_FMT_MOD_BANK_WIDTH_SHIFT: u32 = 25;
pub const AMD_FMT_MOD_BANK_WIDTH_MASK: u64 = 0x3;
pub const AMD_FMT_MOD_BANK_HEIGHT_SHIFT: u32 = 27;
pub const AMD_FMT_MOD_BANK_HEIGHT_MASK: u64 = 0x3;
pub const AMD_FMT_MOD_MACRO_TILE_ASPECT_SHIFT: u32 = 29;
pub const AMD_FMT_MOD_MACRO_TILE_ASPECT_MASK: u64 = 0x3;
pub const AMD_FMT_MOD_NUM_BANKS_SHIFT: u32 = 31;
pub const AMD_FMT_MOD_NUM_BANKS_MASK: u64 = 0x3;
