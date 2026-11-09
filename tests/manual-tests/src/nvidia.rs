use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D;

pub const NV_MMU_PTE_KIND_Z16: u64 = 0x01;
pub const NV_MMU_PTE_KIND_GENERIC_MEMORY: u64 = 0x06;
pub const NV_MMU_PTE_KIND_GENERIC_MEMORY_COMPRESSIBLE: u64 = 0x08;
pub const COMPRESSION_NONE: u64 = 0;
const COMPRESSION_ROP_3D_LAYOUT_1: u64 = 1;
pub const COMPRESSION_ROP_3D_LAYOUT_2: u64 = 2;
pub const COMPRESSION_CDE_VERTICAL: u64 = 4;
pub const COMPRESSION_RESERVED: u64 = 5;
pub const SECTOR_LAYOUT_DESKTOP: u64 = 1;
pub const SECTOR_LAYOUT_BLACKWELL_16BPP: u64 = 3;
pub const SECTOR_LAYOUT_RESERVED: u64 = 4;
pub const GOB_KIND_VERSION_FERMI: u64 = 0;
pub const GOB_KIND_VERSION_G80: u64 = 1;
pub const GOB_KIND_VERSION_TURING: u64 = 2;
pub const GOB_KIND_VERSION_RESERVED: u64 = 3;
pub const ONE_GOB: u64 = 0;
pub const TWO_GOBS: u64 = 1;
pub const FOUR_GOBS: u64 = 2;
pub const EIGHT_GOBS: u64 = 3;
pub const SIXTEEN_GOBS: u64 = 4;
pub const THIRTY_TWO_GOBS: u64 = 5;
pub const SIXTY_FOUR_GOBS: u64 = 6;

pub const fn turing(log2_gobs: u64) -> u64 {
    DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(
        COMPRESSION_NONE,
        SECTOR_LAYOUT_DESKTOP,
        GOB_KIND_VERSION_TURING,
        NV_MMU_PTE_KIND_GENERIC_MEMORY,
        log2_gobs,
    )
}

pub const fn turing_compressed(log2_gobs: u64) -> u64 {
    DRM_FORMAT_MOD_NVIDIA_BLOCK_LINEAR_2D(
        COMPRESSION_ROP_3D_LAYOUT_1,
        SECTOR_LAYOUT_DESKTOP,
        GOB_KIND_VERSION_TURING,
        NV_MMU_PTE_KIND_GENERIC_MEMORY_COMPRESSIBLE,
        log2_gobs,
    )
}
