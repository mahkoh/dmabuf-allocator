use manual_tests::AllocatorError;
use manual_tests::AmdgpuAllocateError;
use manual_tests::AmdgpuGfxVersionError;
use manual_tests::AmdgpuOpenError;
use manual_tests::AmdgpuQueryHwIpError;
use manual_tests::AmdgpuReadRegistersError;
use manual_tests::AmdgpuVirtioCommandError;
use manual_tests::AmdgpuVirtioOpenError;
use manual_tests::Buffer;
use manual_tests::Call;
use manual_tests::Device;
use manual_tests::DeviceBuilder;
use manual_tests::DriverName;
use manual_tests::Errno;
use manual_tests::Fault;
use manual_tests::INVALID;
use manual_tests::LINEAR;
use manual_tests::LayoutError;
use manual_tests::Pitch;
use manual_tests::Plane;
use manual_tests::Request;
use manual_tests::VirtioGpuError;
use manual_tests::amd;
use manual_tests::amd::ADDR_SURF_8_BANK;
use manual_tests::amd::ADDR_SURF_16_BANK;
use manual_tests::amd::ADDR_SURF_BANK_HEIGHT_1;
use manual_tests::amd::ADDR_SURF_BANK_HEIGHT_2;
use manual_tests::amd::ADDR_SURF_BANK_HEIGHT_4;
use manual_tests::amd::ADDR_SURF_BANK_HEIGHT_8;
use manual_tests::amd::ADDR_SURF_BANK_WIDTH_1;
use manual_tests::amd::ADDR_SURF_BANK_WIDTH_2;
use manual_tests::amd::ADDR_SURF_DEPTH_MICRO_TILING;
use manual_tests::amd::ADDR_SURF_DISPLAY_MICRO_TILING;
use manual_tests::amd::ADDR_SURF_MACRO_ASPECT_1;
use manual_tests::amd::ADDR_SURF_MACRO_ASPECT_2;
use manual_tests::amd::ADDR_SURF_P2;
use manual_tests::amd::ADDR_SURF_P2_RESERVED0;
use manual_tests::amd::ADDR_SURF_P4_16X16;
use manual_tests::amd::ADDR_SURF_P8_32X32_8X16;
use manual_tests::amd::ADDR_SURF_P16_32X32_16X16;
use manual_tests::amd::ADDR_SURF_THICK_MICRO_TILING_GFX6;
use manual_tests::amd::ADDR_SURF_THIN_MICRO_TILING;
use manual_tests::amd::ADDR_SURF_TILE_SPLIT_1KB;
use manual_tests::amd::ADDR_SURF_TILE_SPLIT_2KB;
use manual_tests::amd::ADDR_SURF_TILE_SPLIT_4KB;
use manual_tests::amd::ADDR_SURF_TILE_SPLIT_64B;
use manual_tests::amd::ADDR_SURF_TILE_SPLIT_256B;
use manual_tests::amd::ADDR_SURF_TILE_SPLIT_512B;
use manual_tests::amd::AMDGPU_GFX1013_RANGE;
use manual_tests::amd::AMDGPU_IDS_FLAGS_FUSION;
use manual_tests::amd::AMDGPU_MI100_RANGE;
use manual_tests::amd::ARRAY_1D_TILED_THIN1;
use manual_tests::amd::ARRAY_2D_TILED_THIN1;
use manual_tests::amd::ARRAY_LINEAR_ALIGNED;
use manual_tests::amd::Bits;
use manual_tests::amd::COLOR_8_8_8_8;
use manual_tests::amd::COLOR_INVALID;
use manual_tests::amd::DEVICES;
use manual_tests::amd::Descriptor;
use manual_tests::amd::DeviceValues;
use manual_tests::amd::FAMILY_AI;
use manual_tests::amd::FAMILY_NV;
use manual_tests::amd::GB_MACROTILE_MODE_BANK_HEIGHT;
use manual_tests::amd::GB_MACROTILE_MODE_BANK_WIDTH;
use manual_tests::amd::GB_MACROTILE_MODE_MACRO_TILE_ASPECT;
use manual_tests::amd::GB_MACROTILE_MODE_NUM_BANKS;
use manual_tests::amd::GB_TILE_MODE_BANK_WIDTH;
use manual_tests::amd::GB_TILE_MODE_MICRO_TILE_MODE;
use manual_tests::amd::GB_TILE_MODE_MICRO_TILE_MODE_NEW;
use manual_tests::amd::GB_TILE_MODE_NUM_BANKS;
use manual_tests::amd::GB_TILE_MODE_PIPE_CONFIG;
use manual_tests::amd::GB_TILE_MODE_SAMPLE_SPLIT;
use manual_tests::amd::GB_TILE_MODE_TILE_SPLIT;
use manual_tests::amd::GFX6_GB_ADDR_CONFIG_ROW_SIZE;
use manual_tests::amd::GFX6_TILE_INDEX_1D_DISPLAY;
use manual_tests::amd::GFX6_TILE_INDEX_1D_THIN;
use manual_tests::amd::GFX6_TILE_INDEX_2D_DISPLAY_32BPP;
use manual_tests::amd::GFX6_TILE_INDEX_2D_THIN_8BPP;
use manual_tests::amd::GFX6_TILE_INDEX_2D_THIN_16BPP;
use manual_tests::amd::GFX6_TILE_INDEX_2D_THIN_32BPP;
use manual_tests::amd::GFX7_TILE_INDEX_2D_DISPLAY;
use manual_tests::amd::GFX7_TILE_INDEX_2D_THIN;
use manual_tests::amd::GFX9_GB_ADDR_CONFIG_NUM_BANKS;
use manual_tests::amd::GFX9_GB_ADDR_CONFIG_NUM_PIPES;
use manual_tests::amd::GFX9_GB_ADDR_CONFIG_NUM_RB_PER_SE;
use manual_tests::amd::GFX9_GB_ADDR_CONFIG_NUM_SHADER_ENGINES;
use manual_tests::amd::GFX10_GB_ADDR_CONFIG_NUM_PIPES;
use manual_tests::amd::GFX10_GB_ADDR_CONFIG_NUM_PKRS;
use manual_tests::amd::Gfx6Modifier;
use manual_tests::amd::Gfx6Tiling;
use manual_tests::amd::Gfx9Modifier;
use manual_tests::amd::Gfx9Tiling;
use manual_tests::amd::Gfx12Tiling;
use manual_tests::amd::IMG_DATA_FORMAT_8_8_8_8;
use manual_tests::amd::IMG_DATA_FORMAT_16_16_16_16;
use manual_tests::amd::IMG_NUM_FORMAT_FLOAT;
use manual_tests::amd::IMG_NUM_FORMAT_UNORM;
use manual_tests::amd::IP_DISCOVERY_VERSION_MAJOR;
use manual_tests::amd::IpVersion;
use manual_tests::amd::NUMBER_FLOAT;
use manual_tests::amd::NUMBER_UNORM;
use manual_tests::amd::SQ_RSRC_IMG_2D;
use manual_tests::amd::SQ_SEL_W;
use manual_tests::amd::SQ_SEL_X;
use manual_tests::amd::SQ_SEL_Y;
use manual_tests::amd::SQ_SEL_Z;
use manual_tests::amd::TILEINDEX_LINEAR_ALIGNED;
use manual_tests::amd::UmdMetadata;
use manual_tests::amd::Word7;
use manual_tests::amd::ip_discovery_version;
use manual_tests::planes;
use manual_tests::request;
use manual_tests::uapi::amdgpu::AMDGPU_GEM_CREATE_CPU_GTT_USWC;
use manual_tests::uapi::amdgpu::AMDGPU_GEM_CREATE_EXPLICIT_SYNC;
use manual_tests::uapi::amdgpu::AMDGPU_GEM_CREATE_GFX12_DCC;
use manual_tests::uapi::amdgpu::AMDGPU_GEM_CREATE_NO_CPU_ACCESS;
use manual_tests::uapi::amdgpu::AMDGPU_GEM_CREATE_VRAM_CLEARED;
use manual_tests::uapi::amdgpu::AMDGPU_GEM_DOMAIN_GTT;
use manual_tests::uapi::amdgpu::AMDGPU_GEM_DOMAIN_VRAM;
use manual_tests::uapi::amdgpu::DRM_IOCTL_AMDGPU_GEM_METADATA;
use manual_tests::uapi::amdgpu::DRM_IOCTL_AMDGPU_INFO;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_DCC_BLOCK_64B;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_DCC_BLOCK_128B;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_DCC_BLOCK_256B;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_MICROTILE_DISPLAY;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_MICROTILE_THIN;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_MASK;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P2;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P4_8x16;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P4_16x16;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P4_32x32;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P8_16x16_8x16;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P8_32x64_32x32;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_8x16;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX6_1D_TILED_THIN1;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX9_4K_D_X;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX9_64K_D;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX9_64K_D_X;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX9_64K_R_X;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX9_64K_S;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX9_64K_S_X;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX11_256K_R_X;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX12_4K_2D;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX12_64K_2D;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX12_256B_2D;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_GFX12_256K_2D;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VER_GFX9;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VER_GFX10;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VER_GFX11;
use manual_tests::uapi::drm_fourcc::AMD_FMT_MOD_TILE_VER_GFX12;
use manual_tests::uapi::drm_fourcc::DRM_FORMAT_MOD_VENDOR_INTEL;
use manual_tests::uapi::drm_fourcc::fourcc_mod_code;
use manual_tests::uapi::vdrm::AMDGPU_CCMD_QUERY_INFO;
use manual_tests::uapi::vdrm::AMDGPU_CCMD_SET_METADATA;
use manual_tests::uapi::virtgpu::DRM_IOCTL_VIRTGPU_EXECBUFFER;
use manual_tests::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_MAPPABLE;

const TAHITI: &str = "amdgpu:tahiti";
const TAHITI_4K_FRAGMENT: &str = "amdgpu:tahiti:pte_fragment_size=4096";
const BONAIRE: &str = "amdgpu:bonaire";
const STONEY: &str = "amdgpu:stoney";
const FIJI: &str = "amdgpu:fiji";
const FIJI_4K_FRAGMENT: &str = "amdgpu:fiji:pte_fragment_size=4096";
const VEGA10: &str = "amdgpu:vega10";
const VEGA10_4K_FRAGMENT: &str = "amdgpu:vega10:pte_fragment_size=4096";
const RAVEN: &str = "amdgpu:raven";
const RAVEN2: &str = "amdgpu:raven2";
const NAVI10: &str = "amdgpu:navi10";
const NAVI21: &str = "amdgpu:navi21";
const NAVI21_16K_PAGES: &str = "amdgpu:navi21:page_size=16384";
const NAVI21_KERNEL_6_13: &str = "amdgpu:navi21:kernel=6.13";
const NAVI31: &str = "amdgpu:navi31";
const NAVI33: &str = "amdgpu:navi33";
const PHOENIX: &str = "amdgpu:phoenix";
const STRIX1: &str = "amdgpu:strix1";
const GFX1170: &str = "amdgpu:gfx1170";
const GFX1201: &str = "amdgpu:gfx1201";
const GFX1201_4K_FRAGMENT: &str = "amdgpu:gfx1201:pte_fragment_size=4096";
const VIRTIO_TAHITI: &str = "virtio_gpu:host=amdgpu_tahiti";
const VIRTIO_FIJI: &str = "virtio_gpu:host=amdgpu_fiji";
const VIRTIO_VEGA10: &str = "virtio_gpu:host=amdgpu_vega10";
const VIRTIO_NAVI21: &str = "virtio_gpu:host=amdgpu_navi21";
const VIRTIO_NAVI21_64K: &str = "virtio_gpu:host=amdgpu_navi21_64k";

const MAX_EXTENT: u32 = 16384;
const BLOB_ALIGNMENT: u64 = 65536;
const HW_IP_GFX_QUERY: usize = 2;
const MICRO_TILE_WIDTH: u64 = 8;
const AR24_BYTES: u64 = 4;
const SAMPLE_SPLIT_8_BYTES: u64 = 3;
const GFX6_TILE_LINEAR_ALIGNED: u64 = 1;
const GFX6_MICROTILE_DEPTH: u64 = 2;

const TAHITI_2D_DISPLAY_32: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
    microtile: AMD_FMT_MOD_MICROTILE_DISPLAY,
    pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16,
    tile_split: ADDR_SURF_TILE_SPLIT_512B,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_1,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_1,
    num_banks: ADDR_SURF_16_BANK,
    ..Gfx6Modifier::NONE
});

const TAHITI_2D_THIN_32: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
    microtile: AMD_FMT_MOD_MICROTILE_THIN,
    pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16,
    tile_split: ADDR_SURF_TILE_SPLIT_512B,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_1,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_1,
    num_banks: ADDR_SURF_16_BANK,
    ..Gfx6Modifier::NONE
});

const TAHITI_2D_DISPLAY_16: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
    microtile: AMD_FMT_MOD_MICROTILE_DISPLAY,
    pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16,
    tile_split: ADDR_SURF_TILE_SPLIT_256B,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_2,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
    num_banks: ADDR_SURF_16_BANK,
    ..Gfx6Modifier::NONE
});

const TAHITI_2D_THIN_16: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
    microtile: AMD_FMT_MOD_MICROTILE_THIN,
    pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16,
    tile_split: ADDR_SURF_TILE_SPLIT_256B,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_2,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_1,
    num_banks: ADDR_SURF_16_BANK,
    ..Gfx6Modifier::NONE
});

const FIJI_2D_DISPLAY: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
    microtile: AMD_FMT_MOD_MICROTILE_DISPLAY,
    pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16,
    tile_split: ADDR_SURF_TILE_SPLIT_256B,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_4,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
    num_banks: ADDR_SURF_8_BANK,
    ..Gfx6Modifier::NONE
});

const FIJI_2D_THIN_32_DCC: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
    microtile: AMD_FMT_MOD_MICROTILE_THIN,
    pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16,
    tile_split: ADDR_SURF_TILE_SPLIT_512B,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_4,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
    num_banks: ADDR_SURF_8_BANK,
    dcc: true,
});

const GFX9_64K_S: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_S,
    ..Gfx9Modifier::NONE
});

const GFX9_64K_D: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_D,
    ..Gfx9Modifier::NONE
});

const VEGA10_4K_D_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_4K_D_X,
    pipe_xor_bits: 4,
    ..Gfx9Modifier::NONE
});

const VEGA10_64K_D_X_DCC: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_D_X,
    dcc: true,
    dcc_pipe_align: true,
    dcc_independent_64b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
    pipe_xor_bits: 4,
    bank_xor_bits: 4,
    rb: 4,
    pipe: 2,
    ..Gfx9Modifier::NONE
});

const RAVEN_64K_S_X_DCC: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_S_X,
    dcc: true,
    dcc_pipe_align: true,
    dcc_independent_64b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
    pipe_xor_bits: 2,
    rb: 1,
    pipe: 2,
    ..Gfx9Modifier::NONE
});

const RAVEN2_64K_S_X_DCC_CONSTANT_ENCODE: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_S_X,
    dcc: true,
    dcc_pipe_align: true,
    dcc_independent_64b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
    dcc_constant_encode: true,
    pipe_xor_bits: 1,
    bank_xor_bits: 3,
    rb: 1,
    pipe: 1,
    ..Gfx9Modifier::NONE
});

const NAVI10_64K_R_X_DCC: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX10,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    dcc: true,
    dcc_pipe_align: true,
    dcc_independent_128b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_128B,
    dcc_constant_encode: true,
    pipe_xor_bits: 4,
    ..Gfx9Modifier::NONE
});

const NAVI21_64K_R_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    pipe_xor_bits: 4,
    packers: 4,
    ..Gfx9Modifier::NONE
});

const NAVI21_64K_R_X_DCC_RETILE: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    dcc: true,
    dcc_retile: true,
    dcc_independent_128b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_128B,
    dcc_constant_encode: true,
    pipe_xor_bits: 4,
    packers: 4,
    ..Gfx9Modifier::NONE
});

const NAVI21_64K_R_X_DCC: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    dcc: true,
    dcc_pipe_align: true,
    dcc_independent_128b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_128B,
    dcc_constant_encode: true,
    pipe_xor_bits: 4,
    packers: 4,
    ..Gfx9Modifier::NONE
});

const NAVI31_256K_R_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX11_256K_R_X,
    pipe_xor_bits: 5,
    packers: 5,
    ..Gfx9Modifier::NONE
});

const NAVI31_256K_R_X_DCC: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX11_256K_R_X,
    dcc: true,
    dcc_pipe_align: true,
    dcc_independent_128b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_128B,
    pipe_xor_bits: 5,
    packers: 5,
    ..Gfx9Modifier::NONE
});

const PHOENIX_64K_R_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    pipe_xor_bits: 2,
    packers: 2,
    ..Gfx9Modifier::NONE
});

const STRIX1_64K_R_X_DCC_256B: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    dcc: true,
    dcc_pipe_align: true,
    dcc_independent_128b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_256B,
    pipe_xor_bits: 2,
    packers: 2,
    ..Gfx9Modifier::NONE
});

const GFX12_64K_2D_DCC_128B: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX12,
    tile: AMD_FMT_MOD_TILE_GFX12_64K_2D,
    dcc: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_128B,
    ..Gfx9Modifier::NONE
});

const GFX12_64K_2D_DCC_256B: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX12,
    tile: AMD_FMT_MOD_TILE_GFX12_64K_2D,
    dcc: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_256B,
    ..Gfx9Modifier::NONE
});

const TAHITI_1D_DISPLAY: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_1D_TILED_THIN1,
    microtile: AMD_FMT_MOD_MICROTILE_DISPLAY,
    ..Gfx6Modifier::NONE
});

const TAHITI_1D_THIN: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_1D_TILED_THIN1,
    microtile: AMD_FMT_MOD_MICROTILE_THIN,
    ..Gfx6Modifier::NONE
});

const RAVEN_64K_S_X_DCC_RETILE: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_S_X,
    dcc: true,
    dcc_retile: true,
    dcc_independent_64b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
    pipe_xor_bits: 2,
    rb: 1,
    pipe: 2,
    ..Gfx9Modifier::NONE
});

const RAVEN2_64K_S_X_DCC_UNALIGNED: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_S_X,
    dcc: true,
    dcc_independent_64b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
    dcc_constant_encode: true,
    pipe_xor_bits: 1,
    bank_xor_bits: 3,
    ..Gfx9Modifier::NONE
});

const VEGA10_64K_S_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_S_X,
    pipe_xor_bits: 4,
    bank_xor_bits: 4,
    ..Gfx9Modifier::NONE
});

const NAVI21_64K_R_X_DCC_RETILE_64B: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    dcc: true,
    dcc_retile: true,
    dcc_independent_64b: true,
    dcc_independent_128b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
    dcc_constant_encode: true,
    pipe_xor_bits: 4,
    packers: 4,
    ..Gfx9Modifier::NONE
});

const GFX12_64K_2D: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX12,
    tile: AMD_FMT_MOD_TILE_GFX12_64K_2D,
    ..Gfx9Modifier::NONE
});

const GFX11_64K_D: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_D,
    ..Gfx9Modifier::NONE
});

const TAHITI_2D_THIN_32_WIDE_BANKS: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
    microtile: AMD_FMT_MOD_MICROTILE_THIN,
    pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16,
    tile_split: ADDR_SURF_TILE_SPLIT_512B,
    bank_width: ADDR_SURF_BANK_WIDTH_2,
    num_banks: ADDR_SURF_16_BANK,
    ..Gfx6Modifier::NONE
});

const NAVI33_64K_R_X_DCC_RETILE_128B: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    dcc: true,
    dcc_retile: true,
    dcc_independent_128b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_128B,
    pipe_xor_bits: 3,
    packers: 3,
    ..Gfx9Modifier::NONE
});

const GFX6_LINEAR_ALIGNED: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: GFX6_TILE_LINEAR_ALIGNED,
    microtile: AMD_FMT_MOD_MICROTILE_THIN,
    ..Gfx6Modifier::NONE
});

const TAHITI_2D_DEPTH_32: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
    microtile: GFX6_MICROTILE_DEPTH,
    pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16,
    tile_split: ADDR_SURF_TILE_SPLIT_512B,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_1,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_1,
    num_banks: ADDR_SURF_16_BANK,
    ..Gfx6Modifier::NONE
});

const GFX9_64K_R_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    ..Gfx9Modifier::NONE
});

const GFX11_64K_S_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_S_X,
    ..Gfx9Modifier::NONE
});

const STONEY_2D_THIN_32: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
    microtile: AMD_FMT_MOD_MICROTILE_THIN,
    pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P2,
    tile_split: ADDR_SURF_TILE_SPLIT_512B,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_1,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
    num_banks: ADDR_SURF_8_BANK,
    ..Gfx6Modifier::NONE
});

const BONAIRE_2D_THIN_32: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
    microtile: AMD_FMT_MOD_MICROTILE_THIN,
    pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P4_16x16,
    tile_split: ADDR_SURF_TILE_SPLIT_512B,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_1,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
    num_banks: ADDR_SURF_16_BANK,
    ..Gfx6Modifier::NONE
});

const FIJI_2D_THIN_32: u64 = Gfx6Modifier::encode(&Gfx6Modifier {
    tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
    microtile: AMD_FMT_MOD_MICROTILE_THIN,
    pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16,
    tile_split: ADDR_SURF_TILE_SPLIT_512B,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_4,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
    num_banks: ADDR_SURF_8_BANK,
    ..Gfx6Modifier::NONE
});

const GFX12_256B_2D: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX12,
    tile: AMD_FMT_MOD_TILE_GFX12_256B_2D,
    ..Gfx9Modifier::NONE
});

const GFX12_4K_2D_DCC: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX12,
    tile: AMD_FMT_MOD_TILE_GFX12_4K_2D,
    dcc: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_256B,
    ..Gfx9Modifier::NONE
});

const GFX12_256K_2D_DCC: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX12,
    tile: AMD_FMT_MOD_TILE_GFX12_256K_2D,
    dcc: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_256B,
    ..Gfx9Modifier::NONE
});

const VEGA10_64K_S_X_DCC: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_S_X,
    dcc: true,
    dcc_pipe_align: true,
    dcc_independent_64b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
    pipe_xor_bits: 4,
    bank_xor_bits: 4,
    rb: 4,
    pipe: 2,
    ..Gfx9Modifier::NONE
});

const VEGA10_4K_D_X_DCC: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_4K_D_X,
    dcc: true,
    dcc_pipe_align: true,
    dcc_independent_64b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
    pipe_xor_bits: 4,
    rb: 4,
    pipe: 2,
    ..Gfx9Modifier::NONE
});

const VEGA10_64K_D_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_D_X,
    pipe_xor_bits: 4,
    bank_xor_bits: 4,
    ..Gfx9Modifier::NONE
});

const RAVEN_4K_D_X_DCC: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_4K_D_X,
    dcc: true,
    dcc_pipe_align: true,
    dcc_independent_64b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
    pipe_xor_bits: 2,
    rb: 1,
    pipe: 2,
    ..Gfx9Modifier::NONE
});

const RAVEN2_4K_D_X_DCC: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
    tile: AMD_FMT_MOD_TILE_GFX9_4K_D_X,
    dcc: true,
    dcc_pipe_align: true,
    dcc_independent_64b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
    dcc_constant_encode: true,
    pipe_xor_bits: 1,
    bank_xor_bits: 3,
    rb: 1,
    pipe: 1,
    ..Gfx9Modifier::NONE
});

const NAVI10_4K_D_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX10,
    tile: AMD_FMT_MOD_TILE_GFX9_4K_D_X,
    pipe_xor_bits: 4,
    ..Gfx9Modifier::NONE
});

const NAVI10_64K_R_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX10,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    pipe_xor_bits: 4,
    ..Gfx9Modifier::NONE
});

const NAVI21_4K_D_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS,
    tile: AMD_FMT_MOD_TILE_GFX9_4K_D_X,
    pipe_xor_bits: 4,
    ..Gfx9Modifier::NONE
});

const NAVI31_4K_D_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_4K_D_X,
    pipe_xor_bits: 4,
    packers: 5,
    ..Gfx9Modifier::NONE
});

const NAVI31_64K_R_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    pipe_xor_bits: 5,
    packers: 5,
    ..Gfx9Modifier::NONE
});

const NAVI31_64K_R_X_DCC_RETILE_64B: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    dcc: true,
    dcc_retile: true,
    dcc_independent_64b: true,
    dcc_independent_128b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
    pipe_xor_bits: 5,
    packers: 5,
    ..Gfx9Modifier::NONE
});

const NAVI31_64K_R_X_DCC_RETILE_128B: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    dcc: true,
    dcc_retile: true,
    dcc_independent_128b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_128B,
    pipe_xor_bits: 5,
    packers: 5,
    ..Gfx9Modifier::NONE
});

const NAVI31_64K_R_X_DCC_128B: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    dcc: true,
    dcc_pipe_align: true,
    dcc_independent_128b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_128B,
    pipe_xor_bits: 5,
    packers: 5,
    ..Gfx9Modifier::NONE
});

const NAVI33_64K_R_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    pipe_xor_bits: 3,
    packers: 3,
    ..Gfx9Modifier::NONE
});

const NAVI33_256K_R_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX11_256K_R_X,
    pipe_xor_bits: 3,
    packers: 3,
    ..Gfx9Modifier::NONE
});

const PHOENIX_4K_D_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_4K_D_X,
    pipe_xor_bits: 2,
    packers: 2,
    ..Gfx9Modifier::NONE
});

const PHOENIX_64K_R_X_DCC_RETILE_64B: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    dcc: true,
    dcc_retile: true,
    dcc_independent_64b: true,
    dcc_independent_128b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
    pipe_xor_bits: 2,
    packers: 2,
    ..Gfx9Modifier::NONE
});

const PHOENIX_64K_R_X_DCC_RETILE_128B: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
    dcc: true,
    dcc_retile: true,
    dcc_independent_128b: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_128B,
    pipe_xor_bits: 2,
    packers: 2,
    ..Gfx9Modifier::NONE
});

const GFX12_256B_2D_DCC: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX12,
    tile: AMD_FMT_MOD_TILE_GFX12_256B_2D,
    dcc: true,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_256B,
    ..Gfx9Modifier::NONE
});

const NAVI21_64K_S_X: u64 = Gfx9Modifier::encode(&Gfx9Modifier {
    tile_version: AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS,
    tile: AMD_FMT_MOD_TILE_GFX9_64K_S_X,
    pipe_xor_bits: 4,
    packers: 4,
    ..Gfx9Modifier::NONE
});

const TAHITI_MODIFIERS: [u64; 10] = [
    TAHITI_1D_DISPLAY,
    TAHITI_1D_THIN,
    TAHITI_2D_DISPLAY_32,
    TAHITI_2D_THIN_32,
    Gfx6Modifier::encode(&Gfx6Modifier {
        tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
        microtile: AMD_FMT_MOD_MICROTILE_THIN,
        pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16,
        tile_split: ADDR_SURF_TILE_SPLIT_2KB,
        bank_width: ADDR_SURF_BANK_WIDTH_1,
        bank_height: ADDR_SURF_BANK_HEIGHT_1,
        macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_1,
        num_banks: ADDR_SURF_16_BANK,
        ..Gfx6Modifier::NONE
    }),
    TAHITI_2D_THIN_16,
    Gfx6Modifier::encode(&Gfx6Modifier {
        tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
        microtile: AMD_FMT_MOD_MICROTILE_THIN,
        pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16,
        tile_split: ADDR_SURF_TILE_SPLIT_256B,
        bank_width: ADDR_SURF_BANK_WIDTH_1,
        bank_height: ADDR_SURF_BANK_HEIGHT_4,
        macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_1,
        num_banks: ADDR_SURF_16_BANK,
        ..Gfx6Modifier::NONE
    }),
    TAHITI_2D_DISPLAY_16,
    Gfx6Modifier::encode(&Gfx6Modifier {
        tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
        microtile: AMD_FMT_MOD_MICROTILE_DISPLAY,
        pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16,
        tile_split: ADDR_SURF_TILE_SPLIT_256B,
        bank_width: ADDR_SURF_BANK_WIDTH_1,
        bank_height: ADDR_SURF_BANK_HEIGHT_4,
        macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
        num_banks: ADDR_SURF_16_BANK,
        ..Gfx6Modifier::NONE
    }),
    LINEAR,
];

const FIJI_MODIFIERS: [u64; 12] = [
    TAHITI_1D_DISPLAY,
    TAHITI_1D_THIN,
    FIJI_2D_DISPLAY,
    Gfx6Modifier::encode(&Gfx6Modifier {
        tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
        microtile: AMD_FMT_MOD_MICROTILE_THIN,
        pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16,
        tile_split: ADDR_SURF_TILE_SPLIT_256B,
        bank_width: ADDR_SURF_BANK_WIDTH_1,
        bank_height: ADDR_SURF_BANK_HEIGHT_4,
        macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
        num_banks: ADDR_SURF_8_BANK,
        ..Gfx6Modifier::NONE
    }),
    Gfx6Modifier::encode(&Gfx6Modifier {
        tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
        microtile: AMD_FMT_MOD_MICROTILE_THIN,
        pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16,
        tile_split: ADDR_SURF_TILE_SPLIT_256B,
        bank_width: ADDR_SURF_BANK_WIDTH_1,
        bank_height: ADDR_SURF_BANK_HEIGHT_4,
        macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
        num_banks: ADDR_SURF_8_BANK,
        dcc: true,
    }),
    Gfx6Modifier::encode(&Gfx6Modifier {
        tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
        microtile: AMD_FMT_MOD_MICROTILE_DISPLAY,
        pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16,
        tile_split: ADDR_SURF_TILE_SPLIT_512B,
        bank_width: ADDR_SURF_BANK_WIDTH_1,
        bank_height: ADDR_SURF_BANK_HEIGHT_4,
        macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
        num_banks: ADDR_SURF_8_BANK,
        ..Gfx6Modifier::NONE
    }),
    FIJI_2D_THIN_32,
    FIJI_2D_THIN_32_DCC,
    Gfx6Modifier::encode(&Gfx6Modifier {
        tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
        microtile: AMD_FMT_MOD_MICROTILE_DISPLAY,
        pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16,
        tile_split: ADDR_SURF_TILE_SPLIT_1KB,
        bank_width: ADDR_SURF_BANK_WIDTH_1,
        bank_height: ADDR_SURF_BANK_HEIGHT_4,
        macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
        num_banks: ADDR_SURF_8_BANK,
        ..Gfx6Modifier::NONE
    }),
    Gfx6Modifier::encode(&Gfx6Modifier {
        tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
        microtile: AMD_FMT_MOD_MICROTILE_THIN,
        pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16,
        tile_split: ADDR_SURF_TILE_SPLIT_1KB,
        bank_width: ADDR_SURF_BANK_WIDTH_1,
        bank_height: ADDR_SURF_BANK_HEIGHT_4,
        macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
        num_banks: ADDR_SURF_8_BANK,
        ..Gfx6Modifier::NONE
    }),
    Gfx6Modifier::encode(&Gfx6Modifier {
        tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
        microtile: AMD_FMT_MOD_MICROTILE_THIN,
        pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16,
        tile_split: ADDR_SURF_TILE_SPLIT_1KB,
        bank_width: ADDR_SURF_BANK_WIDTH_1,
        bank_height: ADDR_SURF_BANK_HEIGHT_4,
        macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
        num_banks: ADDR_SURF_8_BANK,
        dcc: true,
    }),
    LINEAR,
];

const VEGA10_MODIFIERS: [u64; 9] = [
    GFX9_64K_S,
    GFX9_64K_D,
    VEGA10_4K_D_X,
    VEGA10_64K_S_X,
    VEGA10_64K_D_X,
    VEGA10_4K_D_X_DCC,
    VEGA10_64K_S_X_DCC,
    VEGA10_64K_D_X_DCC,
    LINEAR,
];

const NAVI10_MODIFIERS: [u64; 6] = [
    GFX9_64K_S,
    GFX9_64K_D,
    NAVI10_4K_D_X,
    NAVI10_64K_R_X,
    NAVI10_64K_R_X_DCC,
    LINEAR,
];

const NAVI21_MODIFIERS: [u64; 8] = [
    GFX9_64K_S,
    GFX9_64K_D,
    NAVI21_4K_D_X,
    NAVI21_64K_R_X,
    NAVI21_64K_R_X_DCC_RETILE_64B,
    NAVI21_64K_R_X_DCC_RETILE,
    NAVI21_64K_R_X_DCC,
    LINEAR,
];

const NAVI31_MODIFIERS: [u64; 11] = [
    GFX11_64K_D,
    NAVI31_4K_D_X,
    NAVI31_64K_R_X,
    NAVI31_256K_R_X,
    NAVI31_64K_R_X_DCC_RETILE_64B,
    Gfx9Modifier::encode(&Gfx9Modifier {
        tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
        tile: AMD_FMT_MOD_TILE_GFX11_256K_R_X,
        dcc: true,
        dcc_retile: true,
        dcc_independent_64b: true,
        dcc_independent_128b: true,
        dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
        pipe_xor_bits: 5,
        packers: 5,
        ..Gfx9Modifier::NONE
    }),
    NAVI31_64K_R_X_DCC_RETILE_128B,
    Gfx9Modifier::encode(&Gfx9Modifier {
        tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
        tile: AMD_FMT_MOD_TILE_GFX11_256K_R_X,
        dcc: true,
        dcc_retile: true,
        dcc_independent_128b: true,
        dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_128B,
        pipe_xor_bits: 5,
        packers: 5,
        ..Gfx9Modifier::NONE
    }),
    NAVI31_64K_R_X_DCC_128B,
    NAVI31_256K_R_X_DCC,
    LINEAR,
];

const STRIX1_MODIFIERS: [u64; 8] = [
    GFX11_64K_D,
    PHOENIX_4K_D_X,
    PHOENIX_64K_R_X,
    PHOENIX_64K_R_X_DCC_RETILE_64B,
    PHOENIX_64K_R_X_DCC_RETILE_128B,
    Gfx9Modifier::encode(&Gfx9Modifier {
        tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
        tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
        dcc: true,
        dcc_pipe_align: true,
        dcc_independent_128b: true,
        dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_128B,
        pipe_xor_bits: 2,
        packers: 2,
        ..Gfx9Modifier::NONE
    }),
    STRIX1_64K_R_X_DCC_256B,
    LINEAR,
];

const GFX1201_MODIFIERS: [u64; 9] = [
    GFX12_256B_2D,
    GFX12_64K_2D,
    GFX11_64K_D,
    GFX12_64K_2D_DCC_128B,
    GFX12_256B_2D_DCC,
    GFX12_4K_2D_DCC,
    GFX12_64K_2D_DCC_256B,
    GFX12_256K_2D_DCC,
    LINEAR,
];

const TAHITI_LINEAR_ALIGNED_TILING: u64 = Gfx6Tiling::encode(&Gfx6Tiling {
    array_mode: ARRAY_LINEAR_ALIGNED,
    pipe_config: ADDR_SURF_P2,
    tile_split: ADDR_SURF_TILE_SPLIT_64B,
    micro_tile_mode: ADDR_SURF_DISPLAY_MICRO_TILING,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_1,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_1,
    num_banks: ADDR_SURF_16_BANK,
});

const TAHITI_1D_THIN_TILING: u64 = Gfx6Tiling::encode(&Gfx6Tiling {
    array_mode: ARRAY_1D_TILED_THIN1,
    pipe_config: ADDR_SURF_P8_32X32_8X16,
    tile_split: ADDR_SURF_TILE_SPLIT_64B,
    micro_tile_mode: ADDR_SURF_THIN_MICRO_TILING,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_1,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_1,
    num_banks: ADDR_SURF_16_BANK,
});

const TAHITI_2D_THIN_64_TILING: u64 = Gfx6Tiling::encode(&Gfx6Tiling {
    array_mode: ARRAY_2D_TILED_THIN1,
    pipe_config: ADDR_SURF_P8_32X32_8X16,
    tile_split: ADDR_SURF_TILE_SPLIT_2KB,
    micro_tile_mode: ADDR_SURF_THIN_MICRO_TILING,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_1,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_1,
    num_banks: ADDR_SURF_16_BANK,
});

const FIJI_2D_THIN_32_TILING: u64 = Gfx6Tiling::encode(&Gfx6Tiling {
    array_mode: ARRAY_2D_TILED_THIN1,
    pipe_config: ADDR_SURF_P16_32X32_16X16,
    tile_split: ADDR_SURF_TILE_SPLIT_512B,
    micro_tile_mode: ADDR_SURF_THIN_MICRO_TILING,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_4,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
    num_banks: ADDR_SURF_8_BANK,
});

const GFX9_64K_S_X_DCC_SCANOUT_TILING: u64 = Gfx9Tiling::encode(&Gfx9Tiling {
    swizzle_mode: AMD_FMT_MOD_TILE_GFX9_64K_S_X,
    dcc_offset_256b: 1024,
    dcc_pitch_max: 511,
    dcc_independent_64b: true,
    dcc_independent_128b: false,
    dcc_max_compressed_block_size: AMD_FMT_MOD_DCC_BLOCK_64B,
    scanout: true,
});

const GFX12_64K_2D_DCC_256B_SCANOUT_TILING: u64 = Gfx12Tiling::encode(&Gfx12Tiling {
    swizzle_mode: AMD_FMT_MOD_TILE_GFX12_64K_2D,
    dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_256B,
    dcc_number_type: NUMBER_UNORM,
    dcc_data_format: COLOR_8_8_8_8,
    dcc_write_compress_disable: false,
    scanout: true,
});

const FIJI_1D_THIN_TILING: u64 = Gfx6Tiling::encode(&Gfx6Tiling {
    array_mode: ARRAY_1D_TILED_THIN1,
    pipe_config: ADDR_SURF_P16_32X32_16X16,
    tile_split: ADDR_SURF_TILE_SPLIT_64B,
    micro_tile_mode: ADDR_SURF_THIN_MICRO_TILING,
    bank_width: ADDR_SURF_BANK_WIDTH_1,
    bank_height: ADDR_SURF_BANK_HEIGHT_1,
    macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_1,
    num_banks: ADDR_SURF_16_BANK,
});

const AR24_DESCRIPTOR: Descriptor = Descriptor {
    data_format: IMG_DATA_FORMAT_8_8_8_8,
    num_format: IMG_NUM_FORMAT_UNORM,
    width: 0,
    height: 0,
    perf_mod: 4,
    dst_sel_x: SQ_SEL_Z,
    dst_sel_y: SQ_SEL_Y,
    dst_sel_z: SQ_SEL_X,
    dst_sel_w: SQ_SEL_W,
    tiling_index: 0,
    kind: SQ_RSRC_IMG_2D,
    pitch: 0,
    compression_en: false,
    alpha_is_on_msb: false,
    word7: Word7::Zero,
};

#[derive(Debug, PartialEq, Eq)]
struct AmdCreated {
    modifier: u64,
    planes: Vec<Plane>,
    size: u64,
    alignment: u64,
    tiling_info: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Gem {
    stride: u64,
    size: u64,
    alignment: u64,
    tiling_info: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Create {
    domains: u64,
    flags: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct BlobSizes {
    size: u64,
    alloc_size: u64,
}

struct Metadata {
    flags: u64,
    data: Vec<u32>,
}

struct ImplicitTiling {
    array_mode: u64,
    tiling_index: u32,
}

struct Cus {
    shader_engine: usize,
    shader_array: usize,
    mask: u32,
}

impl Cus {
    fn bitmap(&self) -> [[u32; 4]; 4] {
        let mut bitmap = [[0; 4]; 4];
        bitmap[self.shader_engine][self.shader_array] = self.mask;
        bitmap
    }
}

const fn pitch(offset: u64, stride: u64) -> Pitch {
    Pitch { offset, stride }
}

fn request_64x64(fourcc: &str, modifiers: &[u64]) -> Request {
    request(fourcc, modifiers, 64, 64)
}

fn chosen(chip: &str, request: &Request) -> Option<u64> {
    Device::new(chip).chosen(request)
}

fn chosen_regardless_of_scanout(chip: &str, modifiers: &[u64]) -> Option<u64> {
    let plain = chosen(chip, &request_64x64("AR24", modifiers));
    let scanout = chosen(chip, &request_64x64("AR24", modifiers).scanout());
    assert_eq!(
        plain, scanout,
        "{chip}: the scanout flag changes the choice"
    );
    plain
}

fn prefers(chip: &str, lower: u64, higher: u64) {
    let request = request_64x64("AR24", &[lower, higher]);
    assert_eq!(chosen(chip, &request), Some(higher));
}

fn metadata(buffer: &Buffer) -> Metadata {
    buffer.only(|call| match call {
        Call::AmdgpuGemMetadata { flags, data, .. } => Some(Metadata {
            flags: *flags,
            data: data.clone(),
        }),
        _ => None,
    })
}

fn tiling_info(buffer: &Buffer) -> u64 {
    buffer.only(|call| match *call {
        Call::AmdgpuGemMetadata { tiling_info, .. } => Some(tiling_info),
        _ => None,
    })
}

fn scanout_bit(request: &Request) -> bool {
    let buffer = Device::new(NAVI21).allocate(request).unwrap();
    Gfx9Tiling::decode(tiling_info(&buffer)).scanout
}

fn created_on(device: &Device, request: &Request) -> AmdCreated {
    let buffer = device.allocate(request).unwrap();
    let mut created = AmdCreated {
        modifier: buffer.modifier(),
        planes: buffer.planes(),
        size: 0,
        alignment: 0,
        tiling_info: 0,
    };
    for call in &buffer.calls {
        match *call {
            Call::AmdgpuGemCreate {
                size, alignment, ..
            } => {
                created.size = size;
                created.alignment = alignment;
            }
            Call::VirtgpuAmdgpuGemNew {
                alloc_size,
                phys_alignment,
                ..
            } => {
                created.size = alloc_size;
                created.alignment = phys_alignment;
            }
            Call::AmdgpuGemMetadata { tiling_info, .. }
            | Call::VirtgpuAmdgpuSetMetadata { tiling_info, .. } => {
                created.tiling_info = tiling_info;
            }
            _ => {}
        }
    }
    created
}

fn created(chip: &str, request: &Request) -> AmdCreated {
    created_on(&Device::new(chip), request)
}

fn virtio_equals_native(virtio: &str, native: &str, modifiers: &[u64]) {
    for modifiers in [modifiers, &[LINEAR], &[INVALID]] {
        let request = request_64x64("AR24", modifiers).scanout();
        assert_eq!(
            created(virtio, &request),
            created(native, &request),
            "{virtio} {modifiers:x?}"
        );
    }
}

fn gem(chip: &str, request: &Request) -> Gem {
    let created = created(chip, request);
    Gem {
        stride: created.planes[0].stride,
        size: created.size,
        alignment: created.alignment,
        tiling_info: created.tiling_info,
    }
}

fn pitches(created: &AmdCreated) -> Vec<Pitch> {
    created.planes.iter().map(Plane::pitch).collect()
}

fn offsets(created: &AmdCreated) -> Vec<u64> {
    created.planes.iter().map(|plane| plane.offset).collect()
}

fn strides(created: &AmdCreated) -> Vec<u64> {
    created.planes.iter().map(|plane| plane.stride).collect()
}

fn stride(chip: &str, modifier: u64, width: u32) -> u64 {
    let layout = Device::new(chip)
        .layout(&request("AR24", &[modifier], width, 16))
        .unwrap();
    planes(&layout)[0].stride
}

fn dcc_plane(chip: &str, modifier: u64) -> Plane {
    Device::new(chip)
        .allocate(&request_64x64("AR24", &[modifier]))
        .unwrap()
        .planes()[1]
}

fn create(chip: &str, request: &Request) -> Create {
    let buffer = Device::new(chip).allocate(request).unwrap();
    buffer.only(|call| match *call {
        Call::AmdgpuGemCreate { domains, flags, .. } => Some(Create { domains, flags }),
        _ => None,
    })
}

fn umd(chip: &str, request: &Request) -> Vec<u32> {
    metadata(&Device::new(chip).allocate(request).unwrap()).data
}

fn tiling_index(chip: &str, modifier: u64) -> u32 {
    UmdMetadata::tiling_index(&umd(chip, &request("AR24", &[modifier], 200, 100)))
}

fn implicit_tiling(chip: &str, request: &Request) -> ImplicitTiling {
    let buffer = Device::new(chip).allocate(request).unwrap();
    let metadata = metadata(&buffer);
    ImplicitTiling {
        array_mode: Gfx6Tiling::decode(tiling_info(&buffer)).array_mode,
        tiling_index: UmdMetadata::tiling_index(&metadata.data),
    }
}

fn tile_split_flag(tiling_info: u64) -> u64 {
    Gfx6Tiling::decode(tiling_info).tile_split
}

fn micro_tile_flag(tiling_info: u64) -> u64 {
    Gfx6Tiling::decode(tiling_info).micro_tile_mode
}

fn device_id(chip: &str) -> u32 {
    let name = chip.strip_prefix("amdgpu:").unwrap();
    u32::from(like(name).device_id)
}

fn virtio_sizes(device: &str, request: &Request) -> BlobSizes {
    let buffer = Device::new(device).allocate(request).unwrap();
    buffer.only(|call| match *call {
        Call::VirtgpuAmdgpuGemNew {
            size, alloc_size, ..
        } => Some(BlobSizes { size, alloc_size }),
        _ => None,
    })
}

fn virtio_gem_flags(chip: &str, request: &Request) -> u64 {
    let buffer = Device::new(chip).allocate(request).unwrap();
    buffer.only(|call| match *call {
        Call::VirtgpuAmdgpuGemNew { flags, .. } => Some(flags),
        _ => None,
    })
}

fn like(name: &str) -> DeviceValues {
    DEVICES
        .iter()
        .find(|device| device.name == name)
        .unwrap()
        .clone()
}

fn set_tile_mode(device: &mut DeviceValues, index: u32, field: Bits, value: u64) {
    let mode = &mut device.gb_tile_mode[usize::try_from(index).unwrap()];
    *mode = field.with(*mode, value);
}

fn set_tile_mode_bits(device: &mut DeviceValues, index: u32, bits: u32) {
    device.gb_tile_mode[usize::try_from(index).unwrap()] |= bits;
}

fn wide_banks() -> DeviceValues {
    let mut device = like("tahiti");
    set_tile_mode(
        &mut device,
        GFX6_TILE_INDEX_2D_THIN_32BPP,
        GB_TILE_MODE_BANK_WIDTH,
        ADDR_SURF_BANK_WIDTH_2,
    );
    device
}

fn adhoc_builder(id: &str, values: DeviceValues) -> DeviceBuilder {
    Device::with_kernel(amd::kernel(id).with_values(values))
}

fn adhoc_device(id: &str, values: DeviceValues) -> Device {
    adhoc_builder(id, values).open().unwrap()
}

fn adhoc_chosen(id: &str, values: DeviceValues, request: &Request) -> Option<u64> {
    adhoc_device(id, values).chosen(request)
}

fn adhoc_modifier(id: &str, values: DeviceValues, modifier: u64) -> Option<u64> {
    adhoc_chosen(id, values, &request_64x64("AR24", &[modifier]))
}

fn adhoc_created(id: &str, values: DeviceValues, request: &Request) -> AmdCreated {
    created_on(&adhoc_device(id, values), request)
}

fn open_without_the_gfx_query(values: DeviceValues) -> Result<Device, AllocatorError> {
    let fault = Fault::new(DRM_IOCTL_AMDGPU_INFO).nth(HW_IP_GFX_QUERY);
    adhoc_builder(VEGA10, values).fail(fault).open()
}

#[test]
fn zero_extent_has_no_layout() {
    let device = Device::new(NAVI21);
    let width = request("AR24", &[LINEAR], 0, 16);
    let height = request("AR24", &[LINEAR], 16, 0);
    assert!(device.layout(&width).is_none());
    assert!(device.layout(&height).is_none());
}

#[test]
fn zero_extent_cannot_be_allocated() {
    let device = Device::new(NAVI21);
    let width = request("AR24", &[LINEAR], 0, 16);
    let result = device.allocate(&width);
    assert!(matches!(
        result,
        Err(AllocatorError::Layout(LayoutError::Size))
    ));
}

#[test]
fn largest_extent_is_16384() {
    let device = Device::new(NAVI21);
    let layout = |width, height| device.layout(&request("AR24", &[LINEAR], width, height));
    assert!(layout(MAX_EXTENT, 16).is_some());
    assert!(layout(16, MAX_EXTENT).is_some());
    assert!(layout(MAX_EXTENT + 1, 16).is_none());
    assert!(layout(16, MAX_EXTENT + 1).is_none());
}

#[test]
fn format_without_amd_flag_has_no_layout() {
    assert_eq!(chosen(NAVI21, &request_64x64("C8  ", &[LINEAR])), None);
}

#[test]
fn empty_modifier_list_has_no_layout() {
    assert_eq!(chosen(NAVI21, &request_64x64("AR24", &[])), None);
}

#[test]
fn formats_above_64_bits_are_linear_only() {
    let tiled = request_64x64("AB8F", &[NAVI21_64K_R_X]);
    let both = request_64x64("AB8F", &[NAVI21_64K_R_X, LINEAR]);
    assert_eq!(chosen(NAVI21, &tiled), None);
    assert_eq!(chosen(NAVI21, &both), Some(LINEAR));
}

#[test]
fn dcc_with_pipe_align_takes_64_bits() {
    let dcc = [NAVI21_64K_R_X_DCC];
    assert_eq!(
        chosen(NAVI21, &request_64x64("AB4H", &dcc)),
        Some(NAVI21_64K_R_X_DCC)
    );
}

#[test]
fn implicit_layout_needs_an_implicit_format() {
    assert_eq!(
        chosen(TAHITI, &request_64x64("AR24", &[INVALID])),
        Some(INVALID)
    );
    assert_eq!(chosen(TAHITI, &request_64x64("NV12", &[INVALID])), None);
    assert_eq!(chosen(TAHITI, &request_64x64("BGRF", &[INVALID])), None);
}

#[test]
fn gfx10_3_implicit_modifier_has_no_layout() {
    assert_eq!(chosen(NAVI21, &request_64x64("AR24", &[INVALID])), None);
}

#[test]
fn gfx9_implicit_modifier_has_no_layout() {
    assert_eq!(chosen(VEGA10, &request_64x64("AR24", &[INVALID])), None);
}

#[test]
fn implicit_interop_refuses_dcc_before_gfx12() {
    let dcc = request_64x64("AR24", &[NAVI21_64K_R_X_DCC]).implicit_interop();
    let plain = request_64x64("AR24", &[NAVI21_64K_R_X]).implicit_interop();
    let gfx12 = request_64x64("AR24", &[GFX12_64K_2D_DCC_128B]).implicit_interop();
    assert_eq!(chosen(NAVI21, &dcc), None);
    assert_eq!(chosen(NAVI21, &plain), Some(NAVI21_64K_R_X));
    assert_eq!(chosen(GFX1201, &gfx12), Some(GFX12_64K_2D_DCC_128B));
}

#[test]
fn implicit_interop_clears_the_scanout_bit_before_gfx12() {
    let request = request_64x64("AR24", &[NAVI21_64K_R_X])
        .scanout()
        .implicit_interop();
    assert!(!scanout_bit(&request));
}

#[test]
fn failed_address_config_read_fails_the_allocator() {
    let address_config_read = 4;
    let fault = Fault::new(DRM_IOCTL_AMDGPU_INFO)
        .nth(address_config_read)
        .errno(libc::EINVAL);
    let error = Device::builder(FIJI).fail(fault).open().err().unwrap();
    assert!(matches!(
        error,
        AllocatorError::AmdgpuOpen(AmdgpuOpenError::Registers(AmdgpuReadRegistersError {
            count: 1,
            errno,
            ..
        })) if errno.0 == libc::EINVAL
    ));
}

#[test]
fn failed_tile_table_read_fails_the_allocator() {
    let tile_mode_read = 5;
    let fault = Fault::new(DRM_IOCTL_AMDGPU_INFO).nth(tile_mode_read);
    let error = Device::builder(FIJI).fail(fault).open().err().unwrap();
    assert!(matches!(
        error,
        AllocatorError::AmdgpuOpen(AmdgpuOpenError::Registers(AmdgpuReadRegistersError {
            count: 32,
            ..
        }))
    ));
}

#[test]
fn failed_allocation_leaves_the_device_usable() {
    let device = Device::new(NAVI21);
    let request = request_64x64("AR24", &[NAVI21_64K_R_X]);
    device.fail(Fault::new(DRM_IOCTL_AMDGPU_GEM_METADATA).errno(libc::ENOMEM));
    assert!(device.allocate(&request).is_err());
    assert!(device.allocate(&request).is_ok());
}

#[test]
fn linear_buffers_are_created_write_combined_in_vram() {
    let device = Device::new(NAVI21);
    let buffer = device.allocate(&request_64x64("AR24", &[LINEAR])).unwrap();
    let plane = Plane::first(256, 16384);
    let create = Call::AmdgpuGemCreate {
        size: 16384,
        alignment: 16384,
        domains: AMDGPU_GEM_DOMAIN_VRAM,
        flags: AMDGPU_GEM_CREATE_CPU_GTT_USWC
            | AMDGPU_GEM_CREATE_VRAM_CLEARED
            | AMDGPU_GEM_CREATE_EXPLICIT_SYNC,
    };
    assert_eq!(buffer.modifier(), LINEAR);
    assert_eq!(buffer.planes(), [plane]);
    assert_eq!(buffer.calls[0], create);
    assert_eq!(buffer.calls.len(), 3);
}

#[test]
fn tiled_buffers_are_aligned_to_64_kib() {
    let device = Device::new(NAVI21);
    let buffer = device
        .allocate(&request_64x64("AR24", &[NAVI21_64K_R_X]))
        .unwrap();
    let plane = Plane::first(512, 65536);
    assert_eq!(buffer.modifier(), NAVI21_64K_R_X);
    assert_eq!(buffer.planes(), [plane]);
    assert!(matches!(
        buffer.calls[0],
        Call::AmdgpuGemCreate {
            size: 65536,
            alignment: 65536,
            ..
        }
    ));
}

#[test]
fn layout_and_allocation_have_the_same_planes() {
    let device = Device::new(NAVI21);
    let request = request_64x64("AR24", &[NAVI21_64K_R_X_DCC]);
    let layout = device.layout(&request).unwrap();
    let buffer = device.allocate(&request).unwrap();
    assert_eq!(planes(&layout), buffer.planes());
}

#[test]
fn gfx6_full_list_gives_2d_thin() {
    assert_eq!(
        chosen_regardless_of_scanout(TAHITI, &TAHITI_MODIFIERS),
        Some(TAHITI_2D_THIN_32)
    );
}

#[test]
fn gfx9_full_list_gives_64k_d_x_dcc() {
    assert_eq!(
        chosen_regardless_of_scanout(VEGA10, &VEGA10_MODIFIERS),
        Some(VEGA10_64K_D_X_DCC)
    );
}

#[test]
fn gfx10_full_list_gives_64k_r_x_dcc() {
    assert_eq!(
        chosen_regardless_of_scanout(NAVI10, &NAVI10_MODIFIERS),
        Some(NAVI10_64K_R_X_DCC)
    );
}

#[test]
fn gfx10_3_full_list_gives_64k_r_x_dcc() {
    assert_eq!(
        chosen_regardless_of_scanout(NAVI21, &NAVI21_MODIFIERS),
        Some(NAVI21_64K_R_X_DCC)
    );
}

#[test]
fn gfx11_full_list_gives_256k_r_x_dcc() {
    assert_eq!(
        chosen_regardless_of_scanout(NAVI31, &NAVI31_MODIFIERS),
        Some(NAVI31_256K_R_X_DCC)
    );
}

#[test]
fn gfx11_5_full_list_gives_64k_r_x_dcc_256b() {
    assert_eq!(
        chosen_regardless_of_scanout(STRIX1, &STRIX1_MODIFIERS),
        Some(STRIX1_64K_R_X_DCC_256B)
    );
}

#[test]
fn gfx12_full_list_gives_64k_2d_dcc_256b() {
    assert_eq!(
        chosen_regardless_of_scanout(GFX1201, &GFX1201_MODIFIERS),
        Some(GFX12_64K_2D_DCC_256B)
    );
}

#[test]
fn gfx6_linear_stride_is_a_multiple_of_256() {
    let device = Device::new(TAHITI);
    let layout = device.layout(&request("AB4H", &[LINEAR], 4, 21)).unwrap();
    assert_eq!(layout.planes[0].stride, 256);
}

#[test]
fn gfx12_linear_stride_of_12_byte_pixels_is_a_multiple_of_256() {
    let device = Device::new(GFX1201);
    let layout = device.layout(&request("BGRF", &[LINEAR], 22, 30)).unwrap();
    assert_eq!(layout.planes[0].stride, 512);
}

#[test]
fn invalid_with_linear_gives_the_implicit_layout() {
    let request = request_64x64("AR24", &[INVALID, LINEAR]);
    assert_eq!(chosen(TAHITI, &request), Some(INVALID));
}

#[test]
fn invalid_with_a_chip_modifier_gives_the_chip_modifier() {
    let request = request_64x64("AR24", &[TAHITI_2D_DISPLAY_32, INVALID]);
    assert_eq!(chosen(TAHITI, &request), Some(TAHITI_2D_DISPLAY_32));
}

#[test]
fn scanout_bit_follows_the_scanout_flag() {
    let plain = request_64x64("AR24", &[NAVI21_64K_R_X]);
    let scanout = request_64x64("AR24", &[NAVI21_64K_R_X]).scanout();
    assert!(!scanout_bit(&plain));
    assert!(scanout_bit(&scanout));
}

#[test]
fn buffer_size_and_alignment_are_multiples_of_the_page_size() {
    let request = request("AR24", &[LINEAR], 16, 16);
    let pages_4k = created(NAVI21, &request);
    let pages_16k = created(NAVI21_16K_PAGES, &request);
    assert_eq!(pages_4k.size, 4096);
    assert_eq!(pages_4k.alignment, 4096);
    assert_eq!(pages_16k.size, 16384);
    assert_eq!(pages_16k.alignment, 16384);
}

#[test]
fn virtio_host_is_the_virtio_driver() {
    assert_eq!(
        Device::new(VIRTIO_TAHITI).driver(),
        DriverName::VirtioGpuAmdgpu
    );
}

#[test]
fn virtio_gfx6_host_allocates_like_the_native_device() {
    virtio_equals_native(VIRTIO_TAHITI, TAHITI, &TAHITI_MODIFIERS);
}

#[test]
fn virtio_gfx8_host_allocates_like_the_native_device() {
    virtio_equals_native(VIRTIO_FIJI, FIJI, &FIJI_MODIFIERS);
}

#[test]
fn virtio_gfx10_3_host_allocates_like_the_native_device() {
    let request = request_64x64("AR24", &NAVI21_MODIFIERS).scanout();
    assert_eq!(created(VIRTIO_NAVI21, &request), created(NAVI21, &request));
}

#[test]
fn implicit_linear_flag_is_linear() {
    let plain = request_64x64("AR24", &[INVALID]).linear();
    let scanout = request_64x64("AR24", &[INVALID]).linear().scanout();
    let expected = Gem {
        stride: 256,
        size: 16384,
        alignment: 16384,
        tiling_info: TAHITI_LINEAR_ALIGNED_TILING,
    };
    assert_eq!(gem(TAHITI, &plain), expected);
    assert_eq!(gem(TAHITI, &scanout), expected);
}

#[test]
fn implicit_two_rows_are_linear() {
    let two = request("AR24", &[INVALID], 65, 2);
    let one = request("AR24", &[INVALID], 37, 1).scanout();
    let two_expected = Gem {
        stride: 512,
        size: 4096,
        alignment: 4096,
        tiling_info: TAHITI_LINEAR_ALIGNED_TILING,
    };
    let one_expected = Gem {
        stride: 256,
        size: 4096,
        alignment: 4096,
        tiling_info: TAHITI_LINEAR_ALIGNED_TILING,
    };
    assert_eq!(gem(TAHITI, &two), two_expected);
    assert_eq!(gem(TAHITI, &one), one_expected);
}

#[test]
fn implicit_16_pixels_are_1d() {
    let narrow = request("AR24", &[INVALID], 16, 200);
    let bytes_1 = request("R8  ", &[INVALID], 13, 40);
    let narrow_expected = Gem {
        stride: 64,
        size: 16384,
        alignment: 16384,
        tiling_info: TAHITI_1D_THIN_TILING,
    };
    let bytes_1_expected = Gem {
        stride: 32,
        size: 4096,
        alignment: 4096,
        tiling_info: TAHITI_1D_THIN_TILING,
    };
    assert_eq!(gem(TAHITI, &narrow), narrow_expected);
    assert_eq!(gem(TAHITI, &bytes_1), bytes_1_expected);
}

#[test]
fn implicit_full_hd_is_2d_tiled() {
    let request = request("AR24", &[INVALID], 1920, 1080);
    let expected = Gem {
        stride: 7680,
        size: 8847360,
        alignment: 2097152,
        tiling_info: Gfx6Tiling::encode(&Gfx6Tiling {
            array_mode: ARRAY_2D_TILED_THIN1,
            pipe_config: ADDR_SURF_P8_32X32_8X16,
            tile_split: ADDR_SURF_TILE_SPLIT_512B,
            micro_tile_mode: ADDR_SURF_THIN_MICRO_TILING,
            bank_width: ADDR_SURF_BANK_WIDTH_1,
            bank_height: ADDR_SURF_BANK_HEIGHT_1,
            macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_1,
            num_banks: ADDR_SURF_16_BANK,
        }),
    };
    assert_eq!(gem(TAHITI, &request), expected);
}

#[test]
fn implicit_2d_degrades_to_1d() {
    let request = request("AR24", &[INVALID], 200, 100);
    let expected = Gem {
        stride: 800,
        size: 86016,
        alignment: 65536,
        tiling_info: TAHITI_1D_THIN_TILING,
    };
    assert_eq!(gem(TAHITI, &request), expected);
}

#[test]
fn implicit_8_and_16_byte_pixels_are_2d_tiled() {
    let bytes_8 = request("AB4H", &[INVALID], 256, 256);
    let bytes_16 = request("AB8F", &[INVALID], 256, 256);
    let bytes_8_expected = Gem {
        stride: 2048,
        size: 524288,
        alignment: 524288,
        tiling_info: TAHITI_2D_THIN_64_TILING,
    };
    let bytes_16_expected = Gem {
        stride: 4096,
        size: 1048576,
        alignment: 1048576,
        tiling_info: TAHITI_2D_THIN_64_TILING,
    };
    assert_eq!(gem(TAHITI, &bytes_8), bytes_8_expected);
    assert_eq!(gem(TAHITI, &bytes_16), bytes_16_expected);
}

#[test]
fn implicit_2d_uses_the_4_pipe_config() {
    let request = request("AR24", &[INVALID], 256, 256);
    let expected = Gem {
        stride: 1024,
        size: 262144,
        alignment: 262144,
        tiling_info: Gfx6Tiling::encode(&Gfx6Tiling {
            array_mode: ARRAY_2D_TILED_THIN1,
            pipe_config: ADDR_SURF_P4_16X16,
            tile_split: ADDR_SURF_TILE_SPLIT_512B,
            micro_tile_mode: ADDR_SURF_THIN_MICRO_TILING,
            bank_width: ADDR_SURF_BANK_WIDTH_1,
            bank_height: ADDR_SURF_BANK_HEIGHT_1,
            macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
            num_banks: ADDR_SURF_16_BANK,
        }),
    };
    assert_eq!(gem(BONAIRE, &request), expected);
}

#[test]
fn implicit_gfx8_buffers_have_dcc() {
    let request = request("AR24", &[INVALID], 1920, 1080);
    let expected = Gem {
        stride: 8192,
        size: 9474048,
        alignment: 2097152,
        tiling_info: FIJI_2D_THIN_32_TILING,
    };
    assert_eq!(gem(FIJI, &request), expected);
}

#[test]
fn implicit_gfx8_scanout_has_no_dcc() {
    let request = request("AR24", &[INVALID], 1920, 1080).scanout();
    let expected = Gem {
        stride: 8192,
        size: 9437184,
        alignment: 2097152,
        tiling_info: Gfx6Tiling::encode(&Gfx6Tiling {
            array_mode: ARRAY_2D_TILED_THIN1,
            pipe_config: ADDR_SURF_P16_32X32_16X16,
            tile_split: ADDR_SURF_TILE_SPLIT_512B,
            micro_tile_mode: ADDR_SURF_DISPLAY_MICRO_TILING,
            bank_width: ADDR_SURF_BANK_WIDTH_1,
            bank_height: ADDR_SURF_BANK_HEIGHT_4,
            macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
            num_banks: ADDR_SURF_8_BANK,
        }),
    };
    assert_eq!(gem(FIJI, &request), expected);
}

#[test]
fn implicit_gfx8_scanout_of_non_displayable_format_has_dcc() {
    let request = request("GR88", &[INVALID], 1920, 1080).scanout();
    let expected = Gem {
        stride: 4096,
        size: 4739072,
        alignment: 2097152,
        tiling_info: Gfx6Tiling::encode(&Gfx6Tiling {
            array_mode: ARRAY_2D_TILED_THIN1,
            pipe_config: ADDR_SURF_P16_32X32_16X16,
            tile_split: ADDR_SURF_TILE_SPLIT_256B,
            micro_tile_mode: ADDR_SURF_DISPLAY_MICRO_TILING,
            bank_width: ADDR_SURF_BANK_WIDTH_1,
            bank_height: ADDR_SURF_BANK_HEIGHT_4,
            macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
            num_banks: ADDR_SURF_8_BANK,
        }),
    };
    assert_eq!(gem(FIJI, &request), expected);
}

#[test]
fn gfx6_1d_modifier_is_1d_tiled() {
    let ar24 = request_64x64("AR24", &[TAHITI_1D_THIN]);
    let r8 = request("R8  ", &[TAHITI_1D_THIN], 8, 8);
    let ar24_expected = Gem {
        stride: 256,
        size: 16384,
        alignment: 16384,
        tiling_info: TAHITI_1D_THIN_TILING,
    };
    let r8_expected = Gem {
        stride: 32,
        size: 4096,
        alignment: 4096,
        tiling_info: TAHITI_1D_THIN_TILING,
    };
    assert_eq!(gem(TAHITI, &ar24), ar24_expected);
    assert_eq!(gem(TAHITI, &r8), r8_expected);
}

#[test]
fn implicit_interop_refuses_1d_thin_with_one_row() {
    let request = request("AR24", &[TAHITI_1D_THIN], 64, 1).implicit_interop();
    assert_eq!(chosen(TAHITI, &request), None);
}

#[test]
fn implicit_interop_keeps_1d_display_with_one_row() {
    let request = request("AR24", &[TAHITI_1D_DISPLAY], 64, 1).implicit_interop();
    let expected = Gem {
        stride: 256,
        size: 4096,
        alignment: 4096,
        tiling_info: Gfx6Tiling::encode(&Gfx6Tiling {
            array_mode: ARRAY_1D_TILED_THIN1,
            pipe_config: ADDR_SURF_P8_32X32_8X16,
            tile_split: ADDR_SURF_TILE_SPLIT_64B,
            micro_tile_mode: ADDR_SURF_DISPLAY_MICRO_TILING,
            bank_width: ADDR_SURF_BANK_WIDTH_1,
            bank_height: ADDR_SURF_BANK_HEIGHT_1,
            macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_1,
            num_banks: ADDR_SURF_16_BANK,
        }),
    };
    assert_eq!(gem(TAHITI, &request), expected);
}

#[test]
fn implicit_interop_keeps_1d_thin_with_two_rows() {
    let request = request("AR24", &[TAHITI_1D_THIN], 64, 2).implicit_interop();
    let expected = Gem {
        stride: 256,
        size: 4096,
        alignment: 4096,
        tiling_info: TAHITI_1D_THIN_TILING,
    };
    assert_eq!(gem(TAHITI, &request), expected);
}

#[test]
fn implicit_interop_refuses_gfx8_scanout_dcc() {
    let dcc = request("AR24", &[FIJI_2D_THIN_32_DCC], 256, 256)
        .scanout()
        .implicit_interop();
    assert_eq!(chosen(FIJI, &dcc), None);
}

#[test]
fn implicit_interop_keeps_gfx8_dcc_without_scanout() {
    let request = request("AR24", &[FIJI_2D_THIN_32_DCC], 256, 256).implicit_interop();
    let created = created(FIJI, &request);
    assert_eq!(pitches(&created), [pitch(0, 1024), pitch(262144, 1)]);
    assert_eq!(created.size, 266240);
    assert_eq!(created.alignment, 262144);
    assert_eq!(created.tiling_info, FIJI_2D_THIN_32_TILING);
}

#[test]
fn gfx9_dcc_without_pipe_align_has_a_dcc_plane() {
    let request = request("AR24", &[RAVEN2_64K_S_X_DCC_UNALIGNED], 256, 256).scanout();
    let created = created(RAVEN2, &request);
    assert_eq!(pitches(&created), [pitch(0, 1024), pitch(262144, 512)]);
    assert_eq!(created.size, 327680);
    assert_eq!(created.alignment, 262144);
    assert_eq!(created.tiling_info, GFX9_64K_S_X_DCC_SCANOUT_TILING);
}

#[test]
fn gfx9_dcc_retile_has_two_dcc_planes() {
    let request = request("AR24", &[RAVEN_64K_S_X_DCC_RETILE], 256, 256).scanout();
    let created = created(RAVEN, &request);
    assert_eq!(
        pitches(&created),
        [pitch(0, 1024), pitch(262144, 512), pitch(327680, 512)]
    );
    assert_eq!(created.size, 393216);
    assert_eq!(created.alignment, 262144);
    assert_eq!(created.tiling_info, GFX9_64K_S_X_DCC_SCANOUT_TILING);
}

#[test]
fn gfx9_tiling_info_without_dcc_has_no_dcc_offset() {
    let request = request("AR24", &[VEGA10_64K_S_X], 256, 256);
    assert_eq!(
        gem(VEGA10, &request).tiling_info,
        Gfx9Tiling::encode(&Gfx9Tiling {
            swizzle_mode: AMD_FMT_MOD_TILE_GFX9_64K_S_X,
            dcc_offset_256b: 0,
            dcc_pitch_max: 0,
            dcc_independent_64b: true,
            dcc_independent_128b: false,
            dcc_max_compressed_block_size: AMD_FMT_MOD_DCC_BLOCK_64B,
            scanout: false,
        })
    );
}

#[test]
fn gfx11_5_tiling_info_without_dcc_has_no_dcc_offset() {
    let request = request("AR24", &[PHOENIX_64K_R_X], 256, 256).scanout();
    assert_eq!(
        gem(STRIX1, &request).tiling_info,
        Gfx9Tiling::encode(&Gfx9Tiling {
            swizzle_mode: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
            dcc_offset_256b: 0,
            dcc_pitch_max: 0,
            dcc_independent_64b: false,
            dcc_independent_128b: true,
            dcc_max_compressed_block_size: AMD_FMT_MOD_DCC_BLOCK_256B,
            scanout: true,
        })
    );
}

#[test]
fn large_buffer_is_aligned_to_the_pte_fragment() {
    let request = request("AR24", &[NAVI21_64K_R_X], 1024, 1024);
    let created = created(NAVI21, &request);
    assert_eq!(created.size, 4194304);
    assert_eq!(created.alignment, 2097152);
}

#[test]
fn dcc_without_independent_64b_is_limited_to_2560() {
    let above_limit = request("AR24", &NAVI21_MODIFIERS, 3840, 2160);
    let at_limit = request("AR24", &NAVI21_MODIFIERS, 2560, 2560);
    assert_eq!(
        chosen(NAVI21, &above_limit),
        Some(NAVI21_64K_R_X_DCC_RETILE_64B)
    );
    assert_eq!(chosen(NAVI21, &at_limit), Some(NAVI21_64K_R_X_DCC));
}

#[test]
fn apu_buffers_are_in_vram_and_gtt() {
    let request = request("AR24", &[PHOENIX_64K_R_X], 256, 256);
    let domains = create(PHOENIX, &request).domains;
    assert_eq!(domains, AMDGPU_GEM_DOMAIN_VRAM | AMDGPU_GEM_DOMAIN_GTT);
}

#[test]
fn linear_allows_cpu_access() {
    let request = request_64x64("AR24", &[LINEAR]);
    let flags = create(NAVI21, &request).flags;
    assert_eq!(flags & AMDGPU_GEM_CREATE_NO_CPU_ACCESS, 0);
}

#[test]
fn implicit_sync_drops_explicit_sync() {
    let request = request_64x64("AR24", &[NAVI21_64K_R_X]).implicit_sync();
    let flags = create(NAVI21, &request).flags;
    assert_eq!(flags & AMDGPU_GEM_CREATE_EXPLICIT_SYNC, 0);
}

#[test]
fn gfx12_dcc_buffers_set_the_gfx12_dcc_flag() {
    let request = request("AR24", &[GFX12_64K_2D_DCC_256B], 256, 256);
    let expected = Gem {
        stride: 1024,
        size: 262144,
        alignment: 262144,
        tiling_info: Gfx12Tiling::encode(&Gfx12Tiling {
            swizzle_mode: AMD_FMT_MOD_TILE_GFX12_64K_2D,
            dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_256B,
            dcc_number_type: NUMBER_UNORM,
            dcc_data_format: COLOR_8_8_8_8,
            dcc_write_compress_disable: false,
            scanout: false,
        }),
    };
    let flags = create(GFX1201, &request).flags;
    assert_eq!(gem(GFX1201, &request), expected);
    assert_ne!(flags & AMDGPU_GEM_CREATE_GFX12_DCC, 0);
    assert_ne!(flags & AMDGPU_GEM_CREATE_NO_CPU_ACCESS, 0);
}

#[test]
fn gfx12_scanout_sets_the_scanout_bit() {
    let request = request("AR24", &[GFX12_64K_2D_DCC_256B], 256, 256).scanout();
    assert_eq!(
        gem(GFX1201, &request).tiling_info,
        GFX12_64K_2D_DCC_256B_SCANOUT_TILING
    );
}

#[test]
fn gfx12_buffers_without_dcc_clear_the_gfx12_dcc_flag() {
    let gfx12 = request("AR24", &[GFX12_64K_2D], 256, 256);
    let gfx11 = request("AR24", &[GFX11_64K_D], 256, 256);
    let expected = Gem {
        stride: 1024,
        size: 262144,
        alignment: 262144,
        tiling_info: Gfx12Tiling::encode(&Gfx12Tiling {
            swizzle_mode: AMD_FMT_MOD_TILE_GFX12_64K_2D,
            dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
            dcc_number_type: NUMBER_UNORM,
            dcc_data_format: COLOR_8_8_8_8,
            dcc_write_compress_disable: false,
            scanout: false,
        }),
    };
    assert_eq!(gem(GFX1201, &gfx12), expected);
    assert_eq!(gem(GFX1201, &gfx11), expected);
    assert_eq!(
        create(GFX1201, &gfx12).flags & AMDGPU_GEM_CREATE_GFX12_DCC,
        0
    );
}

#[test]
fn gfx12_dcc_of_format_without_dcc_format_disables_write_compression() {
    let request = request("NV12", &[GFX12_64K_2D_DCC_256B], 2, 256);
    let created = created(GFX1201, &request);
    assert_eq!(pitches(&created), [pitch(0, 256), pitch(65536, 512)]);
    assert_eq!(
        created.tiling_info,
        Gfx12Tiling::encode(&Gfx12Tiling {
            swizzle_mode: AMD_FMT_MOD_TILE_GFX12_64K_2D,
            dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_256B,
            dcc_number_type: NUMBER_UNORM,
            dcc_data_format: COLOR_INVALID,
            dcc_write_compress_disable: true,
            scanout: false,
        })
    );
}

#[test]
fn virtio_linear_is_mappable() {
    let request = request_64x64("AR24", &[LINEAR]);
    let buffer = Device::new(VIRTIO_NAVI21).allocate(&request).unwrap();
    let blob_flags = buffer.only(|call| match *call {
        Call::VirtgpuAmdgpuGemNew { blob_flags, .. } => Some(blob_flags),
        _ => None,
    });
    assert_ne!(blob_flags & VIRTGPU_BLOB_FLAG_USE_MAPPABLE, 0);
}

#[test]
fn failed_gfx_version_query_fails_the_allocator() {
    let fault = Fault::new(DRM_IOCTL_AMDGPU_INFO).nth(HW_IP_GFX_QUERY);
    let error = Device::builder(TAHITI).fail(fault).open().err().unwrap();
    assert!(matches!(
        error,
        AllocatorError::AmdgpuOpen(AmdgpuOpenError::GfxVersion(AmdgpuGfxVersionError::Query(
            AmdgpuQueryHwIpError {
                errno: Errno(libc::EINVAL),
                ..
            }
        )))
    ));
}

#[test]
fn failed_virtio_gfx_version_query_fails_the_allocator() {
    let fault = Fault::new(DRM_IOCTL_VIRTGPU_EXECBUFFER);
    let error = Device::builder(VIRTIO_NAVI21)
        .fail(fault)
        .open()
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::VirtioGpuOpen(VirtioGpuError::AmdgpuOpen(
            AmdgpuVirtioOpenError::GfxVersion(AmdgpuGfxVersionError::Query(
                AmdgpuVirtioCommandError::Send(_)
            ))
        ))
    ));
}

#[test]
fn implicit_linear_pitch_is_padded_to_the_slice_alignment() {
    let request = request("AB4H", &[INVALID], 32, 3).linear();
    let expected = Gem {
        stride: 512,
        size: 4096,
        alignment: 4096,
        tiling_info: TAHITI_LINEAR_ALIGNED_TILING,
    };
    assert_eq!(gem(TAHITI, &request), expected);
}

#[test]
fn host_error_on_the_version_query_fails_the_allocator() {
    let gfx_query = Fault::new(AMDGPU_CCMD_QUERY_INFO);
    let error = Device::builder(VIRTIO_NAVI21)
        .fail_host(gfx_query)
        .open()
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::VirtioGpuOpen(VirtioGpuError::AmdgpuOpen(
            AmdgpuVirtioOpenError::GfxVersion(AmdgpuGfxVersionError::Query(
                AmdgpuVirtioCommandError::Host(Errno(libc::EINVAL))
            ))
        ))
    ));
}

#[test]
fn host_error_on_set_metadata_fails_the_allocation() {
    let device = Device::new(VIRTIO_NAVI21);
    device.fail_host(Fault::new(AMDGPU_CCMD_SET_METADATA).errno(libc::ENOMEM));
    let error = device
        .allocate(&request_64x64("AR24", &[NAVI21_64K_R_X]))
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::VirtioGpuAllocate(VirtioGpuError::AmdgpuAllocate(
            AmdgpuAllocateError::VirtioSetMetadata(AmdgpuVirtioCommandError::Host(errno))
        )) if errno.0 == libc::ENOMEM
    ));
}

#[test]
fn gfx6_refuses_other_tile_versions() {
    assert_eq!(chosen(TAHITI, &request_64x64("AR24", &[GFX9_64K_S])), None);
}

#[test]
fn gfx6_refuses_depth_micro_tiles() {
    assert_eq!(
        chosen(TAHITI, &request_64x64("AR24", &[TAHITI_2D_DEPTH_32])),
        None
    );
}

#[test]
fn gfx6_refuses_other_array_modes() {
    let request = request_64x64("AR24", &[GFX6_LINEAR_ALIGNED]);
    assert_eq!(chosen(TAHITI, &request), None);
}

#[test]
fn gfx9_refuses_r_x_tiles() {
    assert_eq!(
        chosen(VEGA10, &request_64x64("AR24", &[GFX9_64K_R_X])),
        None
    );
}

#[test]
fn gfx11_refuses_s_x_tiles() {
    assert_eq!(
        chosen(NAVI31, &request_64x64("AR24", &[GFX11_64K_S_X])),
        None
    );
}

#[test]
fn gfx6_umd_metadata_describes_the_image() {
    let request = request("AR24", &[TAHITI_2D_THIN_32], 200, 100);
    let expected = UmdMetadata {
        pci_id: device_id(TAHITI),
        descriptor: Descriptor {
            width: 200,
            height: 100,
            tiling_index: GFX6_TILE_INDEX_2D_THIN_32BPP,
            pitch: 256,
            word7: Word7::MaxAnisoRatioCleared,
            ..AR24_DESCRIPTOR
        },
        level_offset_256b: 0,
    };
    assert_eq!(umd(TAHITI, &request), expected.encode());
}

#[test]
fn gfx8_umd_metadata_without_dcc_has_no_compression() {
    let request = request("AR24", &[INVALID], 200, 100).scanout();
    let expected = UmdMetadata {
        pci_id: device_id(FIJI),
        descriptor: Descriptor {
            width: 200,
            height: 100,
            tiling_index: GFX7_TILE_INDEX_2D_DISPLAY,
            pitch: 256,
            ..AR24_DESCRIPTOR
        },
        level_offset_256b: 0,
    };
    assert_eq!(umd(FIJI, &request), expected.encode());
}

#[test]
fn gfx8_umd_metadata_with_implicit_dcc_enables_compression() {
    let request = request("AR24", &[INVALID], 1920, 1080);
    let expected = UmdMetadata {
        pci_id: device_id(FIJI),
        descriptor: Descriptor {
            width: 1920,
            height: 1080,
            tiling_index: GFX7_TILE_INDEX_2D_THIN,
            pitch: 2048,
            compression_en: true,
            alpha_is_on_msb: true,
            word7: Word7::MetaOffset(9437184),
            ..AR24_DESCRIPTOR
        },
        level_offset_256b: 0,
    };
    assert_eq!(umd(FIJI, &request), expected.encode());
}

#[test]
fn gfx8_umd_metadata_with_explicit_dcc_enables_compression() {
    let request = request("AR24", &[FIJI_2D_THIN_32_DCC], 256, 128);
    let expected = UmdMetadata {
        pci_id: device_id(FIJI),
        descriptor: Descriptor {
            width: 256,
            height: 128,
            tiling_index: GFX7_TILE_INDEX_2D_THIN,
            pitch: 256,
            compression_en: true,
            alpha_is_on_msb: true,
            word7: Word7::MetaOffset(131072),
            ..AR24_DESCRIPTOR
        },
        level_offset_256b: 0,
    };
    assert_eq!(umd(FIJI, &request), expected.encode());
}

#[test]
fn virtio_dgpu_buffers_are_in_vram() {
    let request = request_64x64("AR24", &[NAVI21_64K_R_X]);
    let buffer = Device::new(VIRTIO_NAVI21).allocate(&request).unwrap();
    let heap = buffer.only(|call| match *call {
        Call::VirtgpuAmdgpuGemNew { preferred_heap, .. } => Some(preferred_heap),
        _ => None,
    });
    assert_eq!(u64::from(heap), AMDGPU_GEM_DOMAIN_VRAM);
}

#[test]
fn virtio_gfx9_host_accepts_gfx9_modifiers() {
    let request = request_64x64("AR24", &[VEGA10_4K_D_X]);
    assert_eq!(chosen(VIRTIO_VEGA10, &request), Some(VEGA10_4K_D_X));
}

#[test]
fn implicit_at_the_macro_tile_size_is_2d() {
    let request = request("AR24", &[INVALID], 256, 128);
    assert_eq!(gem(FIJI, &request).tiling_info, FIJI_2D_THIN_32_TILING);
}

#[test]
fn implicit_narrower_than_a_macro_tile_is_1d() {
    let request = request("AR24", &[INVALID], 200, 128);
    assert_eq!(gem(FIJI, &request).tiling_info, FIJI_1D_THIN_TILING);
}

#[test]
fn implicit_lower_than_a_macro_tile_is_1d() {
    let request = request("AR24", &[INVALID], 256, 100);
    assert_eq!(gem(FIJI, &request).tiling_info, FIJI_1D_THIN_TILING);
}

#[test]
fn implicit_2d_with_too_much_padding_is_1d() {
    let request = request("AR24", &[INVALID], 257, 256);
    assert_eq!(gem(FIJI, &request).tiling_info, FIJI_1D_THIN_TILING);
}

#[test]
fn gfx6_2d_with_2_pipes_takes_8_kib() {
    let created = created(STONEY, &request("AR24", &[STONEY_2D_THIN_32], 33, 17));
    assert_eq!(strides(&created), [256]);
    assert_eq!(created.size, 8192);
}

#[test]
fn gfx6_2d_with_4_pipes_takes_16_kib() {
    let created = created(BONAIRE, &request("AR24", &[BONAIRE_2D_THIN_32], 33, 17));
    assert_eq!(strides(&created), [256]);
    assert_eq!(created.size, 16384);
}

#[test]
fn gfx6_2d_with_8_pipes_takes_32_kib() {
    let created = created(TAHITI, &request("AR24", &[TAHITI_2D_THIN_32], 33, 17));
    assert_eq!(strides(&created), [256]);
    assert_eq!(created.size, 32768);
}

#[test]
fn gfx6_2d_with_16_pipes_takes_128_kib() {
    let created = created(FIJI, &request("AR24", &[FIJI_2D_THIN_32], 33, 17));
    assert_eq!(strides(&created), [1024]);
    assert_eq!(created.size, 131072);
}

#[test]
fn gfx6_1d_display_pitch_is_aligned_to_32_pixels() {
    let thin = created(TAHITI, &request("AR24", &[TAHITI_1D_THIN], 33, 8));
    let display = created(TAHITI, &request("AR24", &[TAHITI_1D_DISPLAY], 33, 8));
    assert_eq!(strides(&thin), [160]);
    assert_eq!(strides(&display), [256]);
}

#[test]
fn gfx6_2d_is_aligned_to_32_kib() {
    let request = request("AR24", &[TAHITI_2D_THIN_32], 256, 256);
    assert_eq!(created(TAHITI_4K_FRAGMENT, &request).alignment, 32768);
}

#[test]
fn gfx9_64k_is_aligned_to_64_kib() {
    let request = request("AR24", &[VEGA10_64K_S_X], 256, 256);
    assert_eq!(created(VEGA10_4K_FRAGMENT, &request).alignment, 65536);
}

#[test]
fn gfx12_64k_is_aligned_to_64_kib() {
    let request = request("AR24", &[GFX12_64K_2D], 256, 256);
    assert_eq!(created(GFX1201_4K_FRAGMENT, &request).alignment, 65536);
}

#[test]
fn gfx9_dcc_plane_follows_the_main_plane() {
    let request = request("AR24", &[VEGA10_64K_D_X_DCC], 300, 200).scanout();
    let created = created(VEGA10, &request);
    assert_eq!(offsets(&created), [0, 393216]);
    assert_eq!(strides(&created), [1536, 1024]);
    assert_eq!(created.size, 458752);
}

#[test]
fn gfx10_dcc_plane_follows_the_main_plane() {
    let request = request("AR24", &[NAVI10_64K_R_X_DCC], 300, 200).scanout();
    let created = created(NAVI10, &request);
    assert_eq!(offsets(&created), [0, 393216]);
    assert_eq!(strides(&created), [1536, 512]);
    assert_eq!(created.size, 397312);
}

#[test]
fn gfx10_3_dcc_plane_follows_the_main_plane() {
    let request = request("AR24", &[NAVI21_64K_R_X_DCC], 300, 200).scanout();
    let created = created(NAVI21, &request);
    assert_eq!(offsets(&created), [0, 393216]);
    assert_eq!(strides(&created), [1536, 1024]);
    assert_eq!(created.size, 401408);
}

#[test]
fn gfx10_3_dcc_retile_has_two_dcc_planes() {
    let request = request("AR24", &[NAVI21_64K_R_X_DCC_RETILE], 300, 200).scanout();
    let created = created(NAVI21, &request);
    assert_eq!(offsets(&created), [0, 393216, 401408]);
    assert_eq!(strides(&created), [1536, 512, 1024]);
    assert_eq!(created.size, 409600);
}

#[test]
fn gfx12_256b_tiles_are_8_pixels_wide() {
    let created = created(
        GFX1201,
        &request("AR24", &[GFX12_256B_2D], 300, 200).scanout(),
    );
    assert_eq!(strides(&created), [1216]);
    assert_eq!(created.size, 245760);
}

#[test]
fn gfx12_4k_tiles_are_32_pixels_wide() {
    let created = created(
        GFX1201,
        &request("AR24", &[GFX12_4K_2D_DCC], 300, 200).scanout(),
    );
    assert_eq!(strides(&created), [1280]);
    assert_eq!(created.size, 286720);
}

#[test]
fn gfx12_256k_tiles_are_256_pixels_wide() {
    let created = created(
        GFX1201,
        &request("AR24", &[GFX12_256K_2D_DCC], 300, 200).scanout(),
    );
    assert_eq!(strides(&created), [2048]);
    assert_eq!(created.size, 524288);
}

#[test]
fn gfx8_2d_alignment_follows_the_bank_height() {
    let request = request("AR24", &[FIJI_2D_THIN_32_DCC], 256, 256);
    assert_eq!(created(FIJI_4K_FRAGMENT, &request).alignment, 131072);
}

#[test]
fn gfx10_3_dcc_plane_covers_several_metadata_block_rows() {
    let request = request("AR24", &[NAVI21_64K_R_X_DCC], 300, 2000).scanout();
    let created = created(NAVI21, &request);
    assert_eq!(offsets(&created), [0, 3145728]);
    assert_eq!(strides(&created), [1536, 1024]);
    assert_eq!(created.size, 3178496);
}

#[test]
fn gfx6_1d_thin_is_preferred_over_1d_display() {
    prefers(TAHITI, TAHITI_1D_DISPLAY, TAHITI_1D_THIN);
}

#[test]
fn gfx6_2d_display_is_preferred_over_1d_thin() {
    prefers(TAHITI, TAHITI_1D_THIN, TAHITI_2D_DISPLAY_32);
}

#[test]
fn gfx9_64k_d_is_preferred_over_64k_s() {
    prefers(VEGA10, GFX9_64K_S, GFX9_64K_D);
}

#[test]
fn gfx9_4k_d_x_is_preferred_over_64k_d() {
    prefers(VEGA10, GFX9_64K_D, VEGA10_4K_D_X);
}

#[test]
fn gfx9_64k_s_x_is_preferred_over_4k_d_x() {
    prefers(VEGA10, VEGA10_4K_D_X, VEGA10_64K_S_X);
}

#[test]
fn gfx9_64k_d_x_is_preferred_over_64k_s_x() {
    prefers(VEGA10, VEGA10_64K_S_X, VEGA10_64K_D_X);
}

#[test]
fn gfx9_4k_d_x_dcc_is_preferred_over_64k_d_x() {
    prefers(VEGA10, VEGA10_64K_D_X, VEGA10_4K_D_X_DCC);
}

#[test]
fn gfx9_64k_s_x_dcc_retile_is_preferred_over_4k_d_x_dcc() {
    prefers(RAVEN, RAVEN_4K_D_X_DCC, RAVEN_64K_S_X_DCC_RETILE);
}

#[test]
fn gfx9_64k_s_x_dcc_is_preferred_over_4k_d_x_dcc() {
    prefers(RAVEN2, RAVEN2_4K_D_X_DCC, RAVEN2_64K_S_X_DCC_UNALIGNED);
}

#[test]
fn gfx9_64k_s_x_dcc_pipe_aligned_is_preferred_over_4k_d_x_dcc() {
    prefers(VEGA10, VEGA10_4K_D_X_DCC, VEGA10_64K_S_X_DCC);
}

#[test]
fn gfx10_4k_d_x_is_preferred_over_gfx9_64k_s() {
    prefers(NAVI10, GFX9_64K_S, NAVI10_4K_D_X);
    prefers(NAVI21, GFX9_64K_S, NAVI21_4K_D_X);
}

#[test]
fn gfx10_64k_r_x_is_preferred_over_4k_d_x() {
    prefers(NAVI10, NAVI10_4K_D_X, NAVI10_64K_R_X);
    prefers(NAVI21, NAVI21_4K_D_X, NAVI21_64K_R_X);
}

#[test]
fn gfx10_3_dcc_retile_128b_is_preferred_over_retile_64b() {
    prefers(
        NAVI21,
        NAVI21_64K_R_X_DCC_RETILE_64B,
        NAVI21_64K_R_X_DCC_RETILE,
    );
}

#[test]
fn gfx12_gfx11_64k_d_is_preferred_over_256b_2d() {
    prefers(GFX1201, GFX12_256B_2D, GFX11_64K_D);
}

#[test]
fn gfx12_64k_2d_is_preferred_over_gfx11_64k_d() {
    prefers(GFX1201, GFX11_64K_D, GFX12_64K_2D);
}

#[test]
fn gfx12_256b_2d_dcc_is_preferred_over_64k_2d() {
    prefers(GFX1201, GFX12_64K_2D, GFX12_256B_2D_DCC);
}

#[test]
fn gfx12_4k_2d_dcc_is_preferred_over_256b_2d_dcc() {
    prefers(GFX1201, GFX12_256B_2D_DCC, GFX12_4K_2D_DCC);
}

#[test]
fn gfx12_256k_2d_dcc_is_preferred_over_4k_2d_dcc() {
    prefers(GFX1201, GFX12_4K_2D_DCC, GFX12_256K_2D_DCC);
}

#[test]
fn gfx12_64k_2d_dcc_128b_is_preferred_over_256k_2d_dcc() {
    prefers(GFX1201, GFX12_256K_2D_DCC, GFX12_64K_2D_DCC_128B);
}

#[test]
fn gfx11_4k_d_x_is_preferred_over_64k_d() {
    prefers(NAVI31, GFX11_64K_D, NAVI31_4K_D_X);
}

#[test]
fn gfx11_64k_r_x_is_preferred_over_4k_d_x() {
    prefers(NAVI31, NAVI31_4K_D_X, NAVI31_64K_R_X);
    prefers(PHOENIX, PHOENIX_4K_D_X, PHOENIX_64K_R_X);
}

#[test]
fn gfx11_dcc_retile_64b_is_preferred_over_64k_r_x() {
    prefers(NAVI31, NAVI31_64K_R_X, NAVI31_64K_R_X_DCC_RETILE_64B);
    prefers(PHOENIX, PHOENIX_64K_R_X, PHOENIX_64K_R_X_DCC_RETILE_64B);
}

#[test]
fn gfx11_dcc_retile_128b_is_preferred_over_retile_64b() {
    prefers(
        NAVI31,
        NAVI31_64K_R_X_DCC_RETILE_64B,
        NAVI31_64K_R_X_DCC_RETILE_128B,
    );
    prefers(
        PHOENIX,
        PHOENIX_64K_R_X_DCC_RETILE_64B,
        PHOENIX_64K_R_X_DCC_RETILE_128B,
    );
}

#[test]
fn gfx11_dcc_128b_is_preferred_over_retile_128b() {
    prefers(
        NAVI31,
        NAVI31_64K_R_X_DCC_RETILE_128B,
        NAVI31_64K_R_X_DCC_128B,
    );
}

#[test]
fn gfx11_64k_r_x_is_preferred_over_256k_with_16_pipes() {
    prefers(NAVI33, NAVI33_256K_R_X, NAVI33_64K_R_X);
}

#[test]
fn gfx10_3_tiling_info_without_dcc_has_no_dcc_offset() {
    let request = request_64x64("AR24", &[NAVI21_64K_R_X]).scanout();
    assert_eq!(
        gem(NAVI21, &request).tiling_info,
        Gfx9Tiling::encode(&Gfx9Tiling {
            swizzle_mode: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
            dcc_offset_256b: 0,
            dcc_pitch_max: 0,
            dcc_independent_64b: false,
            dcc_independent_128b: true,
            dcc_max_compressed_block_size: AMD_FMT_MOD_DCC_BLOCK_128B,
            scanout: true,
        })
    );
}

#[test]
fn gfx6_tiling_indices_follow_the_modifier() {
    assert_eq!(
        tiling_index(TAHITI, TAHITI_1D_DISPLAY),
        GFX6_TILE_INDEX_1D_DISPLAY
    );
    assert_eq!(
        tiling_index(TAHITI, TAHITI_1D_THIN),
        GFX6_TILE_INDEX_1D_THIN
    );
    assert_eq!(
        tiling_index(TAHITI, TAHITI_2D_DISPLAY_32),
        GFX6_TILE_INDEX_2D_DISPLAY_32BPP
    );
}

#[test]
fn gfx11_accepts_gfx11_64k_d() {
    assert_eq!(
        chosen(NAVI31, &request_64x64("AR24", &[GFX11_64K_D])),
        Some(GFX11_64K_D)
    );
}

#[test]
fn gfx10_3_refuses_s_x_tiles() {
    assert_eq!(
        chosen(NAVI21, &request_64x64("AR24", &[NAVI21_64K_S_X])),
        None
    );
}

#[test]
fn implicit_interop_refuses_gfx9_dcc() {
    let request = request("AR24", &[VEGA10_64K_D_X_DCC], 256, 256).implicit_interop();
    assert_eq!(chosen(VEGA10, &request), None);
}

#[test]
fn implicit_interop_keeps_the_gfx12_scanout_bit() {
    let request = request("AR24", &[GFX12_64K_2D_DCC_256B], 256, 256)
        .scanout()
        .implicit_interop();
    assert_eq!(
        gem(GFX1201, &request).tiling_info,
        GFX12_64K_2D_DCC_256B_SCANOUT_TILING
    );
}

#[test]
fn old_kernels_keep_dcc_before_gfx12() {
    let request = request("AR24", &NAVI21_MODIFIERS, 256, 256);
    assert_eq!(
        chosen(NAVI21_KERNEL_6_13, &request),
        Some(NAVI21_64K_R_X_DCC)
    );
}

#[test]
fn virtio_blob_sizes_follow_the_blob_alignment() {
    let request = request("AR24", &[LINEAR], 16, 16);
    let unaligned = BlobSizes {
        size: 4096,
        alloc_size: 4096,
    };
    let aligned = BlobSizes {
        size: BLOB_ALIGNMENT,
        alloc_size: BLOB_ALIGNMENT,
    };
    assert_eq!(virtio_sizes(VIRTIO_NAVI21, &request), unaligned);
    assert_eq!(virtio_sizes(VIRTIO_NAVI21_64K, &request), aligned);
}

#[test]
fn gfx12_modifiers_are_not_limited_to_2560() {
    for modifier in [GFX12_64K_2D, GFX12_64K_2D_DCC_256B] {
        let request = request("AR24", &[modifier], 4096, 16);
        assert_eq!(chosen(GFX1201, &request), Some(modifier));
    }
}

#[test]
fn virtio_gfx10_dcc_does_not_set_the_gfx12_dcc_flag() {
    let flags = virtio_gem_flags(VIRTIO_NAVI21, &request_64x64("AR24", &[NAVI21_64K_R_X_DCC]));
    assert_eq!(flags & AMDGPU_GEM_CREATE_GFX12_DCC, 0);
}

#[test]
fn implicit_linear_allows_cpu_access() {
    let linear = create(TAHITI, &request_64x64("AR24", &[INVALID]).linear());
    let tiled = create(TAHITI, &request_64x64("AR24", &[INVALID]));
    assert_eq!(linear.flags & AMDGPU_GEM_CREATE_NO_CPU_ACCESS, 0);
    assert_ne!(tiled.flags & AMDGPU_GEM_CREATE_NO_CPU_ACCESS, 0);
}

#[test]
fn implicit_narrow_scanout_is_1d_tiled() {
    let created = created(TAHITI, &request("AR24", &[INVALID], 16, 64).scanout());
    let tiling = Gfx6Tiling::decode(created.tiling_info);
    assert_eq!(tiling.array_mode, ARRAY_1D_TILED_THIN1);
}

#[test]
fn implicit_linear_pitch_of_small_pixels_is_256_bytes() {
    for fourcc in ["R8  ", "R16 "] {
        let created = created(TAHITI, &request(fourcc, &[INVALID], 1, 3).linear());
        assert_eq!(created.planes[0].stride, 256, "{fourcc}");
    }
}

#[test]
fn modifiers_of_other_vendors_are_refused() {
    let intel = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_INTEL, NAVI21_64K_R_X);
    assert_eq!(chosen(NAVI21, &request_64x64("AR24", &[intel])), None);
}

#[test]
fn gfx8_dcc_size_is_aligned_to_the_pipe_interleave() {
    assert_eq!(dcc_plane(FIJI, FIJI_2D_THIN_32_DCC).size, 4096);
}

#[test]
fn gfx9_4k_dcc_is_aligned_to_pipes_and_render_backends() {
    let plane = dcc_plane(VEGA10, VEGA10_4K_D_X_DCC);
    assert_eq!(plane.offset, 65536);
    assert_eq!(plane.size, 65536);
}

#[test]
fn gfx9_64k_dcc_is_aligned_to_the_block_size() {
    let plane = dcc_plane(RAVEN, RAVEN_64K_S_X_DCC);
    assert_eq!(plane.offset, 65536);
    assert_eq!(plane.size, 65536);
}

#[test]
fn failed_gfx_query_falls_back_to_the_gfx9_compute_ip() {
    let fault = Fault::new(DRM_IOCTL_AMDGPU_INFO).nth(HW_IP_GFX_QUERY);
    let device = Device::builder(VEGA10).fail(fault).open().unwrap();
    assert_eq!(
        device.chosen(&request_64x64("AR24", &[VEGA10_4K_D_X])),
        Some(VEGA10_4K_D_X)
    );
}

#[test]
fn only_mi100_ignores_the_second_shader_array() {
    let device = DeviceValues {
        external_rev: AMDGPU_MI100_RANGE.start,
        cu_bitmap: Cus {
            shader_engine: 0,
            shader_array: 1,
            mask: 0x1f,
        }
        .bitmap(),
        ..like("navi21")
    };
    assert_eq!(
        adhoc_modifier(NAVI21, device, NAVI21_64K_R_X_DCC_RETILE),
        Some(NAVI21_64K_R_X_DCC_RETILE)
    );
}

#[test]
fn gfx11_5_dgpu_takes_256k_dcc_with_256b_blocks() {
    let dcc = Gfx9Modifier::encode(&Gfx9Modifier {
        tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
        tile: AMD_FMT_MOD_TILE_GFX11_256K_R_X,
        dcc: true,
        dcc_pipe_align: true,
        dcc_independent_128b: true,
        dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_256B,
        pipe_xor_bits: 2,
        packers: 2,
        ..Gfx9Modifier::NONE
    });
    let device = DeviceValues {
        ids_flags: like("strix1").ids_flags & !AMDGPU_IDS_FLAGS_FUSION,
        ..like("strix1")
    };
    assert_eq!(adhoc_modifier(STRIX1, device, dcc), Some(dcc));
}

#[test]
fn rbplus_dcc_meta_block_adds_no_pipe_when_packers_differ() {
    let device = DeviceValues {
        gb_addr_config: GFX10_GB_ADDR_CONFIG_NUM_PKRS.with(like("navi21").gb_addr_config, 3),
        ..like("navi21")
    };
    let dcc = Gfx9Modifier::encode(&Gfx9Modifier {
        tile_version: AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS,
        tile: AMD_FMT_MOD_TILE_GFX9_64K_R_X,
        dcc: true,
        dcc_pipe_align: true,
        dcc_independent_128b: true,
        dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_128B,
        dcc_constant_encode: true,
        pipe_xor_bits: 4,
        packers: 3,
        ..Gfx9Modifier::NONE
    });
    let buffer = adhoc_device(NAVI21, device)
        .allocate(&request_64x64("AR24", &[dcc]))
        .unwrap();
    assert_eq!(buffer.planes()[1].stride, 512);
}

#[test]
fn gfx6_1d_thin_needs_a_thin_tile_mode_entry() {
    let mut device = like("tahiti");
    set_tile_mode(
        &mut device,
        GFX6_TILE_INDEX_1D_THIN,
        GB_TILE_MODE_MICRO_TILE_MODE,
        ADDR_SURF_DISPLAY_MICRO_TILING,
    );
    assert_eq!(adhoc_modifier(TAHITI, device, TAHITI_1D_THIN), None);
}

#[test]
fn gfx6_2d_pitch_follows_the_bank_width() {
    let buffer = adhoc_device(TAHITI, wide_banks())
        .allocate(&request_64x64("AR24", &[TAHITI_2D_THIN_32_WIDE_BANKS]))
        .unwrap();
    assert_eq!(buffer.modifier(), TAHITI_2D_THIN_32_WIDE_BANKS);
    assert_eq!(buffer.planes()[0].stride, 512);
}

#[test]
fn implicit_2d_degrades_below_a_macro_tile_of_wide_banks() {
    let buffer = adhoc_device(TAHITI, wide_banks())
        .allocate(&request("AR24", &[INVALID], 64, 512))
        .unwrap();
    let tiling = Gfx6Tiling::decode(tiling_info(&buffer));
    assert_eq!(tiling.array_mode, ARRAY_1D_TILED_THIN1);
}

#[test]
fn mi100_counts_only_the_first_shader_array() {
    let device = DeviceValues {
        family: FAMILY_AI,
        external_rev: AMDGPU_MI100_RANGE.start,
        cu_bitmap: Cus {
            shader_engine: 1,
            shader_array: 0,
            mask: 0x1f,
        }
        .bitmap(),
        ..like("raven")
    };
    assert_eq!(
        adhoc_modifier(RAVEN, device, RAVEN_64K_S_X_DCC_RETILE),
        Some(RAVEN_64K_S_X_DCC_RETILE)
    );
}

#[test]
fn gfx1013_compute_ip_is_ignored() {
    let gfx1013 = DeviceValues {
        family: FAMILY_NV,
        external_rev: AMDGPU_GFX1013_RANGE.start,
        ..like("vega10")
    };
    assert!(open_without_the_gfx_query(gfx1013).is_err());
}

#[test]
fn other_navi_compute_ips_are_used() {
    let navi = DeviceValues {
        family: FAMILY_NV,
        external_rev: AMDGPU_GFX1013_RANGE.start - 1,
        ..like("vega10")
    };
    assert!(open_without_the_gfx_query(navi).is_ok());
}

#[test]
fn gfx11_counts_the_compute_units_beyond_four_shader_engines() {
    let device = DeviceValues {
        num_shader_engines: 6,
        cu_bitmap: Cus {
            shader_engine: 0,
            shader_array: 2,
            mask: 0xf,
        }
        .bitmap(),
        ..like("navi33")
    };
    assert_eq!(
        adhoc_modifier(NAVI33, device, NAVI33_64K_R_X_DCC_RETILE_128B),
        Some(NAVI33_64K_R_X_DCC_RETILE_128B)
    );
}

#[test]
fn compute_only_gfx6_takes_modifiers_without_dcc() {
    let device = DeviceValues {
        gfx_rings: 0,
        ..like("tahiti")
    };
    assert_eq!(
        adhoc_modifier(TAHITI, device, TAHITI_2D_THIN_32),
        Some(TAHITI_2D_THIN_32)
    );
}

#[test]
fn unknown_gfx_version_is_reported() {
    let device = DeviceValues {
        gfx_ip_version: IpVersion {
            major: 10,
            minor: 0,
        },
        ..like("tahiti")
    };
    let error = adhoc_builder(TAHITI, device).open().err().unwrap();
    let AllocatorError::AmdgpuOpen(AmdgpuOpenError::GfxVersion(AmdgpuGfxVersionError::Unknown {
        major,
        minor,
    })) = error
    else {
        panic!("{error:?}");
    };
    assert_eq!((major, minor), (10, 0));
}

#[test]
fn unanswered_host_query_fails_with_einval() {
    let unanswered = Fault::new(AMDGPU_CCMD_QUERY_INFO).without_response();
    let error = Device::builder(VIRTIO_NAVI21)
        .fail_host(unanswered)
        .open()
        .err()
        .unwrap();
    assert!(matches!(
        error,
        AllocatorError::VirtioGpuOpen(VirtioGpuError::AmdgpuOpen(
            AmdgpuVirtioOpenError::GfxVersion(AmdgpuGfxVersionError::Query(
                AmdgpuVirtioCommandError::Host(Errno(libc::EINVAL))
            ))
        ))
    ));
}

#[test]
fn implicit_height_3_is_tiled() {
    let tiling = implicit_tiling(TAHITI, &request("AR24", &[INVALID], 64, 3));
    assert_eq!(tiling.array_mode, ARRAY_1D_TILED_THIN1);
}

#[test]
fn implicit_17_pixels_are_2d_tiled() {
    let wide = request("AR24", &[INVALID], 17, 64).scanout();
    let tall = request("AR24", &[INVALID], 64, 17).scanout();
    assert_eq!(
        implicit_tiling(TAHITI, &wide).array_mode,
        ARRAY_2D_TILED_THIN1
    );
    assert_eq!(
        implicit_tiling(TAHITI, &tall).array_mode,
        ARRAY_2D_TILED_THIN1
    );
}

#[test]
fn implicit_scanout_of_16_byte_pixels_is_thin() {
    let request = request("AB8F", &[INVALID], 64, 64).scanout();
    let tiling = implicit_tiling(TAHITI, &request);
    assert_eq!(tiling.tiling_index, GFX6_TILE_INDEX_1D_THIN);
}

#[test]
fn gfx6_pipe_configs_set_the_pipe_count() {
    for pipe_config in 0..=AMD_FMT_MOD_PIPE_CONFIG_MASK {
        let pipes = match pipe_config {
            AMD_FMT_MOD_PIPE_CONFIG_P2 => Some(2),
            AMD_FMT_MOD_PIPE_CONFIG_P4_8x16..=AMD_FMT_MOD_PIPE_CONFIG_P4_32x32 => Some(4),
            AMD_FMT_MOD_PIPE_CONFIG_P8_16x16_8x16..=AMD_FMT_MOD_PIPE_CONFIG_P8_32x64_32x32 => {
                Some(8)
            }
            AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_8x16..=AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16 => {
                Some(16)
            }
            _ => None,
        };
        let mut device = like("tahiti");
        set_tile_mode(
            &mut device,
            GFX6_TILE_INDEX_2D_THIN_32BPP,
            GB_TILE_MODE_PIPE_CONFIG,
            pipe_config,
        );
        let modifier = Gfx6Modifier::encode(&Gfx6Modifier {
            tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
            microtile: AMD_FMT_MOD_MICROTILE_THIN,
            pipe_config,
            tile_split: ADDR_SURF_TILE_SPLIT_512B,
            num_banks: ADDR_SURF_16_BANK,
            ..Gfx6Modifier::NONE
        });
        let layout = adhoc_device(TAHITI, device).layout(&request("AR24", &[modifier], 16, 16));
        let stride = layout.map(|layout| planes(&layout)[0].stride);
        assert_eq!(
            stride,
            pipes.map(|pipes| MICRO_TILE_WIDTH * pipes * AR24_BYTES),
            "{pipe_config}"
        );
    }
}

#[test]
fn gfx1013_range_ends_before_0x86() {
    let navi = DeviceValues {
        family: FAMILY_NV,
        external_rev: AMDGPU_GFX1013_RANGE.end,
        ..like("vega10")
    };
    assert!(open_without_the_gfx_query(navi).is_ok());
}

#[test]
fn gfx10_dcc_without_independent_64b_blocks_stops_at_2560() {
    let request = Request::new("AR24", 2561, 16).modifiers(&[NAVI21_64K_R_X_DCC]);
    assert_eq!(chosen(NAVI21, &request), None);
}

#[test]
fn gfx11_with_16_pipes_prefers_64k() {
    let device = DeviceValues {
        gb_addr_config: GFX10_GB_ADDR_CONFIG_NUM_PIPES.with(like("navi31").gb_addr_config, 4),
        ..like("navi31")
    };
    let r_x = |tile| {
        Gfx9Modifier::encode(&Gfx9Modifier {
            tile_version: AMD_FMT_MOD_TILE_VER_GFX11,
            tile,
            pipe_xor_bits: 4,
            packers: 5,
            ..Gfx9Modifier::NONE
        })
    };
    let r_x_64k = r_x(AMD_FMT_MOD_TILE_GFX9_64K_R_X);
    let r_x_256k = r_x(AMD_FMT_MOD_TILE_GFX11_256K_R_X);
    let chosen = adhoc_device(NAVI31, device)
        .layout(&request_64x64("AR24", &[r_x_256k, r_x_64k]))
        .map(|layout| layout.modifier);
    assert_eq!(chosen, Some(r_x_64k));
}

#[test]
fn addr_config_fields_ignore_their_neighbours() {
    let navi21 = DeviceValues {
        gb_addr_config: like("navi21").gb_addr_config
            | GFX10_GB_ADDR_CONFIG_NUM_PIPES.bit_above()
            | GFX10_GB_ADDR_CONFIG_NUM_PKRS.bit_above(),
        ..like("navi21")
    };
    assert_eq!(
        adhoc_modifier(NAVI21, navi21, NAVI21_64K_R_X),
        Some(NAVI21_64K_R_X)
    );
    let raven2 = like("raven2");
    let raven2 = DeviceValues {
        gb_addr_config: raven2.gb_addr_config
            | GFX9_GB_ADDR_CONFIG_NUM_BANKS.bit_above()
            | GFX9_GB_ADDR_CONFIG_NUM_SHADER_ENGINES.bit_above()
            | GFX9_GB_ADDR_CONFIG_NUM_RB_PER_SE.bit_above(),
        ..raven2
    };
    assert_eq!(
        adhoc_modifier(RAVEN2, raven2, RAVEN2_64K_S_X_DCC_CONSTANT_ENCODE),
        Some(RAVEN2_64K_S_X_DCC_CONSTANT_ENCODE)
    );
}

#[test]
fn gfx9_dcc_reads_the_pipe_xor_and_rb_fields_exactly() {
    let plane = dcc_plane(RAVEN2, RAVEN2_4K_D_X_DCC);
    assert_eq!(plane.offset, 16384);
    assert_eq!(plane.size, 4096);
    let plane = dcc_plane(RAVEN2, RAVEN2_64K_S_X_DCC_CONSTANT_ENCODE);
    assert_eq!(plane.offset, 65536);
    assert_eq!(plane.size, 65536);
}

#[test]
fn gfx9_strides_cover_whole_blocks() {
    assert_eq!(stride(VEGA10, VEGA10_4K_D_X, 65), 96 * 4);
    assert_eq!(stride(NAVI31, NAVI31_256K_R_X, 129), 256 * 4);
}

#[test]
fn gfx12_strides_cover_whole_blocks() {
    let dcc = |tile| {
        Gfx9Modifier::encode(&Gfx9Modifier {
            tile_version: AMD_FMT_MOD_TILE_VER_GFX12,
            tile,
            dcc: true,
            dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_256B,
            ..Gfx9Modifier::NONE
        })
    };
    assert_eq!(
        stride(GFX1201, dcc(AMD_FMT_MOD_TILE_GFX12_256B_2D), 17),
        24 * 4
    );
    assert_eq!(
        stride(GFX1201, dcc(AMD_FMT_MOD_TILE_GFX12_4K_2D), 65),
        96 * 4
    );
    assert_eq!(
        stride(GFX1201, dcc(AMD_FMT_MOD_TILE_GFX12_256K_2D), 129),
        256 * 4
    );
}

#[test]
fn multi_plane_tiled_layouts_have_every_plane() {
    let gfx6 = Device::new(TAHITI)
        .layout(&request_64x64("NV12", &[TAHITI_1D_THIN]))
        .unwrap();
    let gfx9 = Device::new(VEGA10)
        .layout(&request_64x64("NV12", &[GFX9_64K_S]))
        .unwrap();
    assert_eq!(planes(&gfx6).len(), 2);
    assert_eq!(planes(&gfx9).len(), 2);
}

#[test]
fn gfx6_2d_16_bit_pixels_use_the_16_bpp_tiling_index() {
    let data = umd(TAHITI, &request("GR88", &[TAHITI_2D_THIN_16], 200, 100));
    assert_eq!(
        UmdMetadata::tiling_index(&data),
        GFX6_TILE_INDEX_2D_THIN_16BPP
    );
}

#[test]
fn metadata_flags_are_zero() {
    let buffer = Device::new(TAHITI)
        .allocate(&request_64x64("AR24", &[TAHITI_2D_THIN_32]))
        .unwrap();
    let flags = buffer.only(|call| match *call {
        Call::AmdgpuGemMetadata { flags, .. } => Some(flags),
        _ => None,
    });
    assert_eq!(flags, 0);
    let buffer = Device::new(VIRTIO_TAHITI)
        .allocate(&request_64x64("AR24", &[TAHITI_2D_THIN_32]))
        .unwrap();
    let flags = buffer.only(|call| match *call {
        Call::VirtgpuAmdgpuSetMetadata { flags, .. } => Some(flags),
        _ => None,
    });
    assert_eq!(flags, 0);
}

#[test]
fn gfx6_tiling_info_carries_the_bank_width() {
    let buffer = adhoc_device(TAHITI, wide_banks())
        .allocate(&request_64x64("AR24", &[TAHITI_2D_THIN_32_WIDE_BANKS]))
        .unwrap();
    let tiling = Gfx6Tiling::decode(tiling_info(&buffer));
    assert_eq!(tiling.bank_width, ADDR_SURF_BANK_WIDTH_2);
}

#[test]
fn gfx12_tiling_info_carries_the_number_type() {
    let created = created(GFX1201, &request_64x64("AB4H", &[GFX12_64K_2D_DCC_256B]));
    let tiling = Gfx12Tiling::decode(created.tiling_info);
    assert_eq!(tiling.dcc_number_type, NUMBER_FLOAT);
}

#[test]
fn gfx11_7_takes_gfx11_modifiers() {
    assert_eq!(
        chosen(GFX1170, &request_64x64("AR24", &[PHOENIX_64K_R_X])),
        Some(PHOENIX_64K_R_X)
    );
}

#[test]
fn versions_above_gfx12_are_treated_like_gfx12() {
    let versions = [
        IpVersion {
            major: 12,
            minor: 1,
        },
        IpVersion {
            major: 13,
            minor: 0,
        },
    ];
    for version in versions {
        let device = DeviceValues {
            gfx_ip_version: version,
            ip_discovery_version: ip_discovery_version(version),
            ..like("gfx1201")
        };
        assert_eq!(
            adhoc_modifier(GFX1201, device, GFX12_64K_2D),
            Some(GFX12_64K_2D),
            "{}.{}",
            version.major,
            version.minor
        );
    }
}

#[test]
fn gfx11_shader_engine_5_reads_the_second_row() {
    let device = DeviceValues {
        num_shader_engines: 6,
        cu_bitmap: Cus {
            shader_engine: 1,
            shader_array: 2,
            mask: 0xf,
        }
        .bitmap(),
        ..like("navi33")
    };
    assert_eq!(
        adhoc_modifier(NAVI33, device, NAVI33_64K_R_X_DCC_RETILE_128B),
        Some(NAVI33_64K_R_X_DCC_RETILE_128B)
    );
}

#[test]
fn mi100_with_one_shader_array_counts_it() {
    let device = DeviceValues {
        family: FAMILY_AI,
        external_rev: AMDGPU_MI100_RANGE.start,
        num_shader_arrays_per_engine: 1,
        cu_bitmap: Cus {
            shader_engine: 1,
            shader_array: 0,
            mask: 0x1f,
        }
        .bitmap(),
        ..like("raven")
    };
    assert_eq!(
        adhoc_modifier(RAVEN, device, RAVEN_64K_S_X_DCC_RETILE),
        Some(RAVEN_64K_S_X_DCC_RETILE)
    );
}

#[test]
fn gfx6_explicit_displayability_uses_the_tile_mode_of_the_pixel_size() {
    let mut device = like("tahiti");
    set_tile_mode(
        &mut device,
        GFX6_TILE_INDEX_2D_THIN_8BPP,
        GB_TILE_MODE_MICRO_TILE_MODE,
        ADDR_SURF_DISPLAY_MICRO_TILING,
    );
    let request = request("GR88", &[TAHITI_2D_THIN_16], 200, 100);
    let created = adhoc_created(TAHITI, device, &request);
    assert_eq!(
        micro_tile_flag(created.tiling_info),
        ADDR_SURF_THIN_MICRO_TILING
    );
}

#[test]
fn ip_discovery_major_is_8_bits() {
    let device = DeviceValues {
        ip_discovery_version: like("navi31").ip_discovery_version
            | IP_DISCOVERY_VERSION_MAJOR.bit_above(),
        ..like("navi31")
    };
    assert!(adhoc_builder(NAVI31, device).open().is_ok());
}

#[test]
fn gfx1013_with_one_compute_ring_ignores_it() {
    let gfx1013 = DeviceValues {
        family: FAMILY_NV,
        external_rev: AMDGPU_GFX1013_RANGE.start,
        compute_rings: 1,
        ..like("vega10")
    };
    assert!(open_without_the_gfx_query(gfx1013).is_err());
}

#[test]
fn gfx9_displayable_retile_dcc_is_not_rb_aligned() {
    let raven = like("raven").gb_addr_config;
    let raven = GFX9_GB_ADDR_CONFIG_NUM_SHADER_ENGINES.with(raven, 1);
    let device = DeviceValues {
        gb_addr_config: GFX9_GB_ADDR_CONFIG_NUM_RB_PER_SE.with(raven, 2),
        ..like("raven")
    };
    let retile = Gfx9Modifier::encode(&Gfx9Modifier {
        tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
        tile: AMD_FMT_MOD_TILE_GFX9_64K_S_X,
        dcc: true,
        dcc_retile: true,
        dcc_independent_64b: true,
        dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
        pipe_xor_bits: 3,
        rb: 3,
        pipe: 2,
        ..Gfx9Modifier::NONE
    });
    let created = adhoc_created(RAVEN, device, &request("AR24", &[retile], 256, 256));
    assert_eq!(created.modifier, retile);
    assert_eq!(created.planes[1].stride, 512);
}

#[test]
fn gfx9_dcc_pipe_alignment_stops_at_32_pipes() {
    let vega10 = like("vega10").gb_addr_config;
    let vega10 = GFX9_GB_ADDR_CONFIG_NUM_PIPES.with(vega10, 3);
    let device = DeviceValues {
        gb_addr_config: GFX9_GB_ADDR_CONFIG_NUM_SHADER_ENGINES.with(vega10, 3),
        ..like("vega10")
    };
    let dcc = Gfx9Modifier::encode(&Gfx9Modifier {
        tile_version: AMD_FMT_MOD_TILE_VER_GFX9,
        tile: AMD_FMT_MOD_TILE_GFX9_64K_S_X,
        dcc: true,
        dcc_pipe_align: true,
        dcc_independent_64b: true,
        dcc_max_compressed_block: AMD_FMT_MOD_DCC_BLOCK_64B,
        pipe_xor_bits: 6,
        bank_xor_bits: 2,
        rb: 5,
        pipe: 3,
        ..Gfx9Modifier::NONE
    });
    let created = adhoc_created(VEGA10, device, &request("AR24", &[dcc], 256, 256));
    assert_eq!(created.modifier, dcc);
    assert_eq!(created.planes[1].size, 262144);
}

#[test]
fn gfx9_4k_dcc_is_aligned_to_its_meta_block() {
    let created = created(RAVEN2, &request("AR24", &[RAVEN2_4K_D_X_DCC], 32, 32));
    assert_eq!(created.planes[1].offset, 4096);
}

#[test]
fn gfx6_metadata_carries_the_data_format_and_the_num_format() {
    let data = umd(TAHITI, &request_64x64("AB4H", &[TAHITI_1D_THIN]));
    assert_eq!(UmdMetadata::data_format(&data), IMG_DATA_FORMAT_16_16_16_16);
    assert_eq!(UmdMetadata::num_format(&data), IMG_NUM_FORMAT_FLOAT);
}

#[test]
fn gfx6_display_flag_reads_two_micro_tile_mode_bits() {
    let mut device = like("tahiti");
    set_tile_mode(
        &mut device,
        GFX6_TILE_INDEX_2D_THIN_32BPP,
        GB_TILE_MODE_MICRO_TILE_MODE,
        ADDR_SURF_DISPLAY_MICRO_TILING,
    );
    set_tile_mode_bits(
        &mut device,
        GFX6_TILE_INDEX_2D_THIN_32BPP,
        GB_TILE_MODE_MICRO_TILE_MODE.bit_above(),
    );
    let created = adhoc_created(TAHITI, device, &request("AR24", &[INVALID], 256, 256));
    assert_eq!(
        micro_tile_flag(created.tiling_info),
        ADDR_SURF_DISPLAY_MICRO_TILING
    );
}

#[test]
fn gfx6_num_banks_ignore_the_next_bit() {
    let mut device = like("tahiti");
    set_tile_mode_bits(
        &mut device,
        GFX6_TILE_INDEX_2D_THIN_32BPP,
        GB_TILE_MODE_NUM_BANKS.bit_above(),
    );
    assert_eq!(
        adhoc_modifier(TAHITI, device, TAHITI_2D_THIN_32),
        Some(TAHITI_2D_THIN_32)
    );
}

#[test]
fn gfx8_sample_split_ignores_the_next_bit() {
    let mut device = like("fiji");
    set_tile_mode_bits(
        &mut device,
        GFX7_TILE_INDEX_2D_THIN,
        GB_TILE_MODE_SAMPLE_SPLIT.bit_above(),
    );
    assert_eq!(
        adhoc_modifier(FIJI, device, FIJI_2D_THIN_32_DCC),
        Some(FIJI_2D_THIN_32_DCC)
    );
}

#[test]
fn gfx8_depth_tile_mode_takes_the_tile_split_of_the_register() {
    let mut device = like("fiji");
    set_tile_mode(
        &mut device,
        GFX7_TILE_INDEX_2D_THIN,
        GB_TILE_MODE_MICRO_TILE_MODE_NEW,
        ADDR_SURF_DEPTH_MICRO_TILING,
    );
    set_tile_mode(
        &mut device,
        GFX7_TILE_INDEX_2D_THIN,
        GB_TILE_MODE_TILE_SPLIT,
        ADDR_SURF_TILE_SPLIT_64B,
    );
    let created = adhoc_created(FIJI, device, &request("AR24", &[INVALID], 256, 256));
    assert_eq!(
        tile_split_flag(created.tiling_info),
        ADDR_SURF_TILE_SPLIT_64B
    );
}

#[test]
fn gfx6_64b_tile_split_bounds_the_alignment() {
    let mut device = like("tahiti");
    set_tile_mode(
        &mut device,
        GFX6_TILE_INDEX_2D_THIN_32BPP,
        GB_TILE_MODE_TILE_SPLIT,
        ADDR_SURF_TILE_SPLIT_64B,
    );
    let created = adhoc_created(
        TAHITI_4K_FRAGMENT,
        device,
        &request("AR24", &[INVALID], 64, 128),
    );
    assert_eq!(
        tile_split_flag(created.tiling_info),
        ADDR_SURF_TILE_SPLIT_64B
    );
    assert_eq!(created.alignment, 8192);
}

#[test]
fn gfx6_tile_split_above_4kb_is_reported_as_1kb() {
    let mut device = like("tahiti");
    set_tile_mode(
        &mut device,
        GFX6_TILE_INDEX_2D_THIN_32BPP,
        GB_TILE_MODE_TILE_SPLIT,
        ADDR_SURF_TILE_SPLIT_4KB + 1,
    );
    let created = adhoc_created(TAHITI, device, &request("AR24", &[INVALID], 256, 256));
    assert_eq!(
        tile_split_flag(created.tiling_info),
        ADDR_SURF_TILE_SPLIT_1KB
    );
}

#[test]
fn gfx8_macrotile_mode_fields_are_read_exactly() {
    let mut device = like("fiji");
    let macrotile_mode_of_4_byte_pixels = 2;
    let mode = GB_MACROTILE_MODE_BANK_WIDTH.with(0, ADDR_SURF_BANK_WIDTH_2);
    let mode = GB_MACROTILE_MODE_BANK_HEIGHT.with(mode, ADDR_SURF_BANK_HEIGHT_8);
    let mode = GB_MACROTILE_MODE_MACRO_TILE_ASPECT.with(mode, ADDR_SURF_MACRO_ASPECT_2);
    let mode = GB_MACROTILE_MODE_NUM_BANKS.with(mode, ADDR_SURF_8_BANK);
    device.gb_macrotile_mode[macrotile_mode_of_4_byte_pixels] =
        mode | GB_MACROTILE_MODE_NUM_BANKS.bit_above();
    let modifier = Gfx6Modifier::encode(&Gfx6Modifier {
        tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
        microtile: AMD_FMT_MOD_MICROTILE_THIN,
        pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16,
        tile_split: ADDR_SURF_TILE_SPLIT_512B,
        bank_width: ADDR_SURF_BANK_WIDTH_2,
        bank_height: ADDR_SURF_BANK_HEIGHT_8,
        macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
        num_banks: ADDR_SURF_8_BANK,
        ..Gfx6Modifier::NONE
    });
    assert_eq!(adhoc_modifier(FIJI, device, modifier), Some(modifier));
}

#[test]
fn gfx8_row_size_bounds_the_tile_split() {
    let mut device = like("fiji");
    device.gb_addr_config = GFX6_GB_ADDR_CONFIG_ROW_SIZE.with(device.gb_addr_config, 1)
        | GFX6_GB_ADDR_CONFIG_ROW_SIZE.bit_above();
    set_tile_mode(
        &mut device,
        GFX7_TILE_INDEX_2D_THIN,
        GB_TILE_MODE_SAMPLE_SPLIT,
        SAMPLE_SPLIT_8_BYTES,
    );
    let modifier = Gfx6Modifier::encode(&Gfx6Modifier {
        tile: AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1,
        microtile: AMD_FMT_MOD_MICROTILE_THIN,
        pipe_config: AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16,
        tile_split: ADDR_SURF_TILE_SPLIT_2KB,
        bank_width: ADDR_SURF_BANK_WIDTH_1,
        bank_height: ADDR_SURF_BANK_HEIGHT_4,
        macro_tile_aspect: ADDR_SURF_MACRO_ASPECT_2,
        num_banks: ADDR_SURF_8_BANK,
        ..Gfx6Modifier::NONE
    });
    assert_eq!(
        adhoc_chosen(FIJI, device, &request_64x64("AB4H", &[modifier])),
        Some(modifier)
    );
}

#[test]
fn gfx10_3_packers_are_bounded_by_the_block() {
    let device = DeviceValues {
        gb_addr_config: GFX10_GB_ADDR_CONFIG_NUM_PKRS.with(like("navi21").gb_addr_config, 5),
        ..like("navi21")
    };
    assert_eq!(
        adhoc_modifier(NAVI21, device, NAVI21_64K_R_X),
        Some(NAVI21_64K_R_X)
    );
}

#[test]
fn gfx11_packers_ignore_the_next_bit() {
    let device = DeviceValues {
        gb_addr_config: like("navi31").gb_addr_config | GFX10_GB_ADDR_CONFIG_NUM_PKRS.bit_above(),
        ..like("navi31")
    };
    assert_eq!(
        adhoc_modifier(NAVI31, device, NAVI31_256K_R_X),
        Some(NAVI31_256K_R_X)
    );
}

#[test]
fn mi100_shader_engine_4_reads_the_second_column() {
    let device = DeviceValues {
        family: FAMILY_AI,
        external_rev: AMDGPU_MI100_RANGE.start,
        num_shader_engines: 6,
        cu_bitmap: Cus {
            shader_engine: 0,
            shader_array: 1,
            mask: 0x1f,
        }
        .bitmap(),
        ..like("raven")
    };
    assert_eq!(
        adhoc_modifier(RAVEN, device, RAVEN_64K_S_X_DCC_RETILE),
        Some(RAVEN_64K_S_X_DCC_RETILE)
    );
}

#[test]
fn gfx6_render_micro_tile_mode_is_displayable() {
    let mut device = like("tahiti");
    set_tile_mode(
        &mut device,
        GFX6_TILE_INDEX_2D_THIN_32BPP,
        GB_TILE_MODE_MICRO_TILE_MODE,
        ADDR_SURF_THICK_MICRO_TILING_GFX6,
    );
    let created = adhoc_created(TAHITI, device, &request("AR24", &[INVALID], 256, 256));
    assert_eq!(
        micro_tile_flag(created.tiling_info),
        ADDR_SURF_DISPLAY_MICRO_TILING
    );
}

#[test]
fn gfx9_version_on_a_gfx1013_revision_is_gfx9() {
    let device = DeviceValues {
        family: FAMILY_NV,
        external_rev: AMDGPU_GFX1013_RANGE.start,
        ..like("vega10")
    };
    assert!(adhoc_builder(VEGA10, device).open().is_ok());
}

#[test]
fn implicit_invalid_pipe_config_has_no_layout() {
    let mut device = like("tahiti");
    set_tile_mode(
        &mut device,
        GFX6_TILE_INDEX_2D_THIN_32BPP,
        GB_TILE_MODE_PIPE_CONFIG,
        ADDR_SURF_P2_RESERVED0,
    );
    let request = request("AR24", &[INVALID], 256, 256);
    assert_eq!(adhoc_chosen(TAHITI, device, &request), None);
}

#[test]
fn gfx6_2d_alignment_follows_the_bank_width() {
    let created = adhoc_created(
        TAHITI_4K_FRAGMENT,
        wide_banks(),
        &request_64x64("AR24", &[TAHITI_2D_THIN_32_WIDE_BANKS]),
    );
    assert_eq!(created.modifier, TAHITI_2D_THIN_32_WIDE_BANKS);
    assert_eq!(created.alignment, 65536);
}

#[test]
fn gfx6_linear_is_linear_aligned() {
    let request = request_64x64("AR24", &[LINEAR]);
    let created = created(TAHITI, &request);
    let tiling = Gfx6Tiling::decode(created.tiling_info);
    assert_eq!(tiling.array_mode, ARRAY_LINEAR_ALIGNED);
    let data = umd(TAHITI, &request);
    assert_eq!(UmdMetadata::tiling_index(&data), TILEINDEX_LINEAR_ALIGNED);
}

#[test]
fn implicit_display_tile_mode_has_no_dcc() {
    let mut device = like("fiji");
    set_tile_mode(
        &mut device,
        GFX7_TILE_INDEX_2D_THIN,
        GB_TILE_MODE_MICRO_TILE_MODE_NEW,
        ADDR_SURF_DISPLAY_MICRO_TILING,
    );
    let created = adhoc_created(FIJI, device, &request("AR24", &[INVALID], 256, 256));
    assert_eq!(created.size, created.planes[0].size);
}

#[test]
fn gfx9_64k_s_strides_cover_whole_blocks() {
    assert_eq!(stride(VEGA10, GFX9_64K_S, 65), 128 * 4);
}

#[test]
fn gfx9_metadata_has_no_flags_or_umd_data() {
    let buffer = Device::new(NAVI21)
        .allocate(&request_64x64("AR24", &[NAVI21_64K_R_X]))
        .unwrap();
    let metadata = metadata(&buffer);
    assert_eq!(metadata.flags, 0);
    assert_eq!(metadata.data, []);
}
