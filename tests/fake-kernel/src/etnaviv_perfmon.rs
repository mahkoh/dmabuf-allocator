pub struct PmDomain {
    pub name: &'static str,
    pub signals: &'static [&'static str],
}

pub const DOMS_3D: &[PmDomain] = &[
    PmDomain {
        name: "HI",
        signals: &[
            "TOTAL_READ_BYTES8",
            "TOTAL_WRITE_BYTES8",
            "TOTAL_CYCLES",
            "IDLE_CYCLES",
            "AXI_CYCLES_READ_REQUEST_STALLED",
            "AXI_CYCLES_WRITE_REQUEST_STALLED",
            "AXI_CYCLES_WRITE_DATA_STALLED",
        ],
    },
    PmDomain {
        name: "PE",
        signals: &[
            "PIXEL_COUNT_KILLED_BY_COLOR_PIPE",
            "PIXEL_COUNT_KILLED_BY_DEPTH_PIPE",
            "PIXEL_COUNT_DRAWN_BY_COLOR_PIPE",
            "PIXEL_COUNT_DRAWN_BY_DEPTH_PIPE",
        ],
    },
    PmDomain {
        name: "SH",
        signals: &[
            "SHADER_CYCLES",
            "PS_INST_COUNTER",
            "RENDERED_PIXEL_COUNTER",
            "VS_INST_COUNTER",
            "RENDERED_VERTICE_COUNTER",
            "VTX_BRANCH_INST_COUNTER",
            "VTX_TEXLD_INST_COUNTER",
            "PXL_BRANCH_INST_COUNTER",
            "PXL_TEXLD_INST_COUNTER",
        ],
    },
    PmDomain {
        name: "PA",
        signals: &[
            "INPUT_VTX_COUNTER",
            "INPUT_PRIM_COUNTER",
            "OUTPUT_PRIM_COUNTER",
            "DEPTH_CLIPPED_COUNTER",
            "TRIVIAL_REJECTED_COUNTER",
            "CULLED_COUNTER",
        ],
    },
    PmDomain {
        name: "SE",
        signals: &["CULLED_TRIANGLE_COUNT", "CULLED_LINES_COUNT"],
    },
    PmDomain {
        name: "RA",
        signals: &[
            "VALID_PIXEL_COUNT",
            "TOTAL_QUAD_COUNT",
            "VALID_QUAD_COUNT_AFTER_EARLY_Z",
            "TOTAL_PRIMITIVE_COUNT",
            "PIPE_CACHE_MISS_COUNTER",
            "PREFETCH_CACHE_MISS_COUNTER",
            "CULLED_QUAD_COUNT",
        ],
    },
    PmDomain {
        name: "TX",
        signals: &[
            "TOTAL_BILINEAR_REQUESTS",
            "TOTAL_TRILINEAR_REQUESTS",
            "TOTAL_DISCARDED_TEXTURE_REQUESTS",
            "TOTAL_TEXTURE_REQUESTS",
            "MEM_READ_COUNT",
            "MEM_READ_IN_8B_COUNT",
            "CACHE_MISS_COUNT",
            "CACHE_HIT_TEXEL_COUNT",
            "CACHE_MISS_TEXEL_COUNT",
        ],
    },
    PmDomain {
        name: "MC",
        signals: &[
            "TOTAL_READ_REQ_8B_FROM_PIPELINE",
            "TOTAL_READ_REQ_8B_FROM_IP",
            "TOTAL_WRITE_REQ_8B_FROM_PIPELINE",
        ],
    },
];
pub const DOMS_2D: &[PmDomain] = &[PmDomain {
    name: "PE",
    signals: &["PIXELS_RENDERED_2D"],
}];
pub const DOMS_VG: &[PmDomain] = &[];
