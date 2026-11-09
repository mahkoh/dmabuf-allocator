pub const NV2080_CTRL_MC_ARCH_INFO_ARCHITECTURE_AD100: u32 = 0x190;
pub const NV2080_CTRL_MC_ARCH_INFO_IMPLEMENTATION_AD102: u32 = 0x2;
pub const NV2080_CTRL_MC_ARCH_INFO_ARCHITECTURE_GB200: u32 = 0x1b0;
pub const NV2080_CTRL_MC_ARCH_INFO_IMPLEMENTATION_GB202: u32 = 0x2;

pub const NV2080_ENGINE_TYPE_GRAPHICS: u32 = 0x1;
pub const NV2080_ENGINE_TYPE_COPY0: u32 = 0x9;
pub const NV2080_ENGINE_TYPE_COPY1: u32 = 0xa;
pub const NV2080_ENGINE_TYPE_COPY2: u32 = 0xb;
pub const NV2080_ENGINE_TYPE_COPY9: u32 = 0x12;
pub const NV2080_ENGINE_TYPE_BSP: u32 = 0x13;
pub const NV2080_ENGINE_TYPE_NVDEC0: u32 = NV2080_ENGINE_TYPE_BSP;
pub const NV2080_ENGINE_TYPE_COPY10: u32 = 0x34;
pub const NV2080_ENGINE_TYPE_COPY19: u32 = 0x3d;
pub const NV2080_ENGINE_TYPE_NVDEC_SIZE: u32 = 8;

pub const fn NV2080_ENGINE_TYPE_NVDEC(i: u32) -> u32 {
    NV2080_ENGINE_TYPE_NVDEC0 + i
}

pub const fn NV2080_ENGINE_TYPE_IS_COPY(i: u32) -> bool {
    (i >= NV2080_ENGINE_TYPE_COPY0 && i <= NV2080_ENGINE_TYPE_COPY9)
        || (i >= NV2080_ENGINE_TYPE_COPY10 && i <= NV2080_ENGINE_TYPE_COPY19)
}

pub const fn NV2080_ENGINE_TYPE_IS_NVDEC(i: u32) -> bool {
    i >= NV2080_ENGINE_TYPE_NVDEC0 && i < NV2080_ENGINE_TYPE_NVDEC(NV2080_ENGINE_TYPE_NVDEC_SIZE)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NV2080_CTRL_GR_GET_ZCULL_INFO_PARAMS {
    pub widthAlignPixels: u32,
    pub heightAlignPixels: u32,
    pub pixelSquaresByAliquots: u32,
    pub aliquotTotal: u32,
    pub zcullRegionByteMultiplier: u32,
    pub zcullRegionHeaderSize: u32,
    pub zcullSubregionHeaderSize: u32,
    pub subregionCount: u32,
    pub subregionWidthAlignPixels: u32,
    pub subregionHeightAlignPixels: u32,
}
