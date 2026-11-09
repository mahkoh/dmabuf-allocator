use crate::uapi::nvctrl::NV2080_CTRL_GR_GET_ZCULL_INFO_PARAMS;
use crate::uapi::nvctrl::NV2080_ENGINE_TYPE_COPY0;
use crate::uapi::nvctrl::NV2080_ENGINE_TYPE_COPY1;
use crate::uapi::nvctrl::NV2080_ENGINE_TYPE_COPY2;
use crate::uapi::nvctrl::NV2080_ENGINE_TYPE_GRAPHICS;

pub struct Gpu {
    pub revision: u8,
    pub total_fb_kib: u32,
    pub bar1_size: u64,
    pub tpcs_per_gpc: &'static [u16],
    pub zcull: NV2080_CTRL_GR_GET_ZCULL_INFO_PARAMS,
    pub zcull_ctxsw_size: u32,
    pub zcull_ctxsw_align: u32,
    pub engines: &'static [u32],
}

const RTX_4090_ZCULL: NV2080_CTRL_GR_GET_ZCULL_INFO_PARAMS = NV2080_CTRL_GR_GET_ZCULL_INFO_PARAMS {
    widthAlignPixels: 512,
    heightAlignPixels: 32,
    pixelSquaresByAliquots: 1024,
    aliquotTotal: 4096,
    zcullRegionByteMultiplier: 32,
    zcullRegionHeaderSize: 32,
    zcullSubregionHeaderSize: 256,
    subregionCount: 16,
    subregionWidthAlignPixels: 256,
    subregionHeightAlignPixels: 64,
};
const RTX_4090_ZCULL_CTXSW_SIZE: u32 = 657408;
const RTX_4090_ZCULL_CTXSW_ALIGN: u32 = 4096;

pub static RTX_4090: Gpu = Gpu {
    revision: 0xa1,
    total_fb_kib: 24 << 20,
    bar1_size: 256 << 20,
    tpcs_per_gpc: &[6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 5],
    zcull: RTX_4090_ZCULL,
    zcull_ctxsw_size: RTX_4090_ZCULL_CTXSW_SIZE,
    zcull_ctxsw_align: RTX_4090_ZCULL_CTXSW_ALIGN,
    engines: &[
        NV2080_ENGINE_TYPE_GRAPHICS,
        NV2080_ENGINE_TYPE_COPY0,
        NV2080_ENGINE_TYPE_COPY1,
        NV2080_ENGINE_TYPE_COPY2,
    ],
};

pub static RTX_5090: Gpu = Gpu {
    revision: 0xa1,
    total_fb_kib: 32 << 20,
    bar1_size: 256 << 20,
    tpcs_per_gpc: &[8, 8, 8, 8, 8, 8, 8, 8, 7, 7, 7],
    zcull: RTX_4090_ZCULL,
    zcull_ctxsw_size: RTX_4090_ZCULL_CTXSW_SIZE,
    zcull_ctxsw_align: RTX_4090_ZCULL_CTXSW_ALIGN,
    engines: &[
        NV2080_ENGINE_TYPE_GRAPHICS,
        NV2080_ENGINE_TYPE_COPY0,
        NV2080_ENGINE_TYPE_COPY1,
        NV2080_ENGINE_TYPE_COPY2,
    ],
};
