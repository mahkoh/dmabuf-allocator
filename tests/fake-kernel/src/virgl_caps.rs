use crate::uapi::virgl::VIRGL_CAP_APP_TWEAK_SUPPORT;
use crate::uapi::virgl::VIRGL_CAP_HOST_IS_GLES;
use crate::uapi::virgl::VIRGL_CAP_V2_QUERY_FORMAT_MODIFIER;
use crate::uapi::virgl::virgl_caps_bool_set1;
use crate::uapi::virgl::virgl_caps_v1;
use crate::uapi::virgl::virgl_caps_v2;
use crate::uapi::virgl::virgl_supported_format_mask;
use crate::uapi::virgl::virgl_video_caps;
use std::ffi::c_char;

pub const VIRGL_MAX_VERSION: u32 = 1;
pub const VIRGL2_MAX_VERSION: u32 = 2;

const fn renderer(name: &str) -> [c_char; 64] {
    let name = name.as_bytes();
    let mut renderer = [0; 64];
    let mut i = 0;
    while i < name.len() {
        renderer[i] = name[i] as c_char;
        i += 1;
    }
    renderer
}

const NO_VIDEO_CAPS: virgl_video_caps = virgl_video_caps {
    _bitfield_1: 0,
    _bitfield_2: 0,
    _bitfield_3: 0,
    _bitfield_4: 0,
};

const V1: virgl_caps_v1 = virgl_caps_v1 {
    max_version: VIRGL2_MAX_VERSION,
    sampler: virgl_supported_format_mask {
        bitmask: [
            0xf03727fe, 0x0b0b0000, 0xd8002c0b, 0x761fff30, 0x111029c8, 0x77760001, 0xaaaaabff,
            0xa0007fea, 0x00000007, 0x03900000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000,
        ],
    },
    render: virgl_supported_format_mask {
        bitmask: [
            0xf00027fe, 0x0b0b0000, 0xd8002c0b, 0x36000130, 0x111028c8, 0x77760001, 0xaaaaabff,
            0x20007fea, 0x00000000, 0x03900000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000,
        ],
    },
    depthstencil: virgl_supported_format_mask {
        bitmask: [
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000,
        ],
    },
    vertexbuffer: virgl_supported_format_mask {
        bitmask: [
            0x00000000, 0x00000000, 0x00000000, 0x10000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000,
        ],
    },
    bset: virgl_caps_bool_set1 {
        _bitfield_1: 0xffe3fd7f,
    },
    glsl_level: 460,
    max_texture_array_layers: 8192,
    max_streamout_buffers: 4,
    max_dual_source_render_targets: 1,
    max_render_targets: 8,
    max_samples: 8,
    prim_mask: 31871,
    max_tbo_size: 268435455,
    max_uniform_blocks: 15,
    max_viewports: 16,
    max_texture_gather_components: 4,
};

pub static VIRGL: virgl_caps_v1 = virgl_caps_v1 {
    max_version: VIRGL_MAX_VERSION,
    ..V1
};

pub static VIRGL2: virgl_caps_v2 = virgl_caps_v2 {
    v1: V1,
    min_aliased_point_size: 0.125,
    max_aliased_point_size: 2048.0,
    min_smooth_point_size: 0.125,
    max_smooth_point_size: 2048.0,
    min_aliased_line_width: 1.0,
    max_aliased_line_width: 2048.0,
    min_smooth_line_width: 1.0,
    max_smooth_line_width: 2048.0,
    max_texture_lod_bias: 16.0,
    max_geom_output_vertices: 256,
    max_geom_total_output_components: 4095,
    max_vertex_outputs: 32,
    max_vertex_attribs: 16,
    max_shader_patch_varyings: 30,
    min_texel_offset: -32,
    max_texel_offset: 31,
    min_texture_gather_offset: -32,
    max_texture_gather_offset: 31,
    texture_buffer_offset_alignment: 4,
    uniform_buffer_offset_alignment: 4,
    shader_buffer_offset_alignment: 4,
    capability_bits: 0xdff7fbff,
    sample_locations: [52292, 3861556834, 4051425619, 2503851799, 0, 0, 0, 0],
    max_vertex_attrib_stride: 2048,
    max_shader_buffer_frag_compute: 16,
    max_shader_buffer_other_stages: 16,
    max_shader_image_frag_compute: 16,
    max_shader_image_other_stages: 16,
    max_image_samples: 8,
    max_compute_work_group_invocations: 1024,
    max_compute_shared_memory_size: 65536,
    max_compute_grid_size: [2147483646, 65535, 65535],
    max_compute_block_size: [1024, 1024, 1024],
    max_texture_2d_size: 16384,
    max_texture_3d_size: 8192,
    max_texture_cube_size: 16384,
    max_combined_shader_buffers: 32,
    max_atomic_counters: [4096, 4096, 4096, 4096, 4096, 4096],
    max_atomic_counter_buffers: [16, 16, 16, 16, 16, 16],
    max_combined_atomic_counters: 4096,
    max_combined_atomic_counter_buffers: 80,
    host_feature_check_version: 23,
    supported_readback_formats: virgl_supported_format_mask {
        bitmask: [
            0xf03727fe, 0x0b0b0000, 0xd8002c0b, 0x761fff30, 0x111029c8, 0x77760001, 0xaaaaabff,
            0xa0007fea, 0x00000007, 0x03900000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000,
        ],
    },
    scanout: virgl_supported_format_mask {
        bitmask: [
            0x00000186, 0x00080000, 0x40000009, 0x00000000, 0x00000048, 0x00000000, 0x00000000,
            0x00001600, 0x00000000, 0x00100000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000,
        ],
    },
    capability_bits_v2: 0x0003ff9f,
    max_video_memory: 8388608,
    renderer: renderer("fake-kernel virgl host"),
    max_anisotropy: 16.0,
    max_texture_samplers: 32,
    supported_multisample_formats: virgl_supported_format_mask {
        bitmask: [
            0xf03727ff, 0x0b0b0000, 0x58002c0b, 0x76000130, 0x111029c8, 0x77760001, 0xaaaaabff,
            0x20007fea, 0x00000000, 0x00900000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000,
        ],
    },
    max_const_buffer_size: [65536, 65536, 65536, 65536, 65536, 65536],
    num_video_caps: 0,
    video_caps: [NO_VIDEO_CAPS; 32],
    max_uniform_block_size: 2147483647,
    max_tcs_outputs: 1024,
    max_tes_outputs: 32,
    max_shader_storage_blocks: [16, 16, 16, 16, 16, 16],
};

pub static VIRGL2_GLES: virgl_caps_v2 = virgl_caps_v2 {
    v1: virgl_caps_v1 {
        sampler: virgl_supported_format_mask {
            bitmask: [
                0xf037a7fe, 0x0b0b0000, 0xd8002c0b, 0x761fff30, 0x111029c8, 0x77760001, 0xaaaaabff,
                0xa0007fea, 0x00000007, 0x03900000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
                0x00000000, 0x00000000,
            ],
        },
        render: virgl_supported_format_mask {
            bitmask: [
                0xf00027fe, 0x0b0b0000, 0xd8002c03, 0x36000120, 0x111028c8, 0x77760001, 0xaaaaabff,
                0x20007fea, 0x00000000, 0x03900000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
                0x00000000, 0x00000000,
            ],
        },
        ..VIRGL2.v1
    },
    max_texture_2d_size: 8192,
    capability_bits: VIRGL2.capability_bits | VIRGL_CAP_HOST_IS_GLES,
    ..VIRGL2
};

pub static VIRGL2_FORMAT_MODIFIER: virgl_caps_v2 = virgl_caps_v2 {
    capability_bits_v2: VIRGL2.capability_bits_v2 | VIRGL_CAP_V2_QUERY_FORMAT_MODIFIER,
    ..VIRGL2
};

pub static VIRGL2_GLES_NO_TWEAK: virgl_caps_v2 = virgl_caps_v2 {
    capability_bits: VIRGL2_GLES.capability_bits & !VIRGL_CAP_APP_TWEAK_SUPPORT,
    ..VIRGL2_GLES
};
