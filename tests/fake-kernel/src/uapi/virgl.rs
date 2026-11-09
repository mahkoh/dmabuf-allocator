use crate::bitfield::bitfields;
use core::ffi::c_char;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct virgl_caps_bool_set1 {
    pub _bitfield_1: u32,
}

impl virgl_caps_bool_set1 {
    bitfields! {
        _bitfield_1 {
            indep_blend_enable, set_indep_blend_enable: 0, 1;
            indep_blend_func, set_indep_blend_func: 1, 1;
            cube_map_array, set_cube_map_array: 2, 1;
            shader_stencil_export, set_shader_stencil_export: 3, 1;
            conditional_render, set_conditional_render: 4, 1;
            start_instance, set_start_instance: 5, 1;
            primitive_restart, set_primitive_restart: 6, 1;
            blend_eq_sep, set_blend_eq_sep: 7, 1;
            instanceid, set_instanceid: 8, 1;
            vertex_element_instance_divisor, set_vertex_element_instance_divisor: 9, 1;
            seamless_cube_map, set_seamless_cube_map: 10, 1;
            occlusion_query, set_occlusion_query: 11, 1;
            timer_query, set_timer_query: 12, 1;
            streamout_pause_resume, set_streamout_pause_resume: 13, 1;
            texture_multisample, set_texture_multisample: 14, 1;
            fragment_coord_conventions, set_fragment_coord_conventions: 15, 1;
            depth_clip_disable, set_depth_clip_disable: 16, 1;
            seamless_cube_map_per_texture, set_seamless_cube_map_per_texture: 17, 1;
            ubo, set_ubo: 18, 1;
            color_clamping, set_color_clamping: 19, 1;
            poly_stipple, set_poly_stipple: 20, 1;
            mirror_clamp, set_mirror_clamp: 21, 1;
            texture_query_lod, set_texture_query_lod: 22, 1;
            has_fp64, set_has_fp64: 23, 1;
            has_tessellation_shaders, set_has_tessellation_shaders: 24, 1;
            has_indirect_draw, set_has_indirect_draw: 25, 1;
            has_sample_shading, set_has_sample_shading: 26, 1;
            has_cull, set_has_cull: 27, 1;
            conditional_render_inverted, set_conditional_render_inverted: 28, 1;
            derivative_control, set_derivative_control: 29, 1;
            polygon_offset_clamp, set_polygon_offset_clamp: 30, 1;
            transform_feedback_overflow_query, set_transform_feedback_overflow_query: 31, 1;
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct virgl_supported_format_mask {
    pub bitmask: [u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct virgl_caps_v1 {
    pub max_version: u32,
    pub sampler: virgl_supported_format_mask,
    pub render: virgl_supported_format_mask,
    pub depthstencil: virgl_supported_format_mask,
    pub vertexbuffer: virgl_supported_format_mask,
    pub bset: virgl_caps_bool_set1,
    pub glsl_level: u32,
    pub max_texture_array_layers: u32,
    pub max_streamout_buffers: u32,
    pub max_dual_source_render_targets: u32,
    pub max_render_targets: u32,
    pub max_samples: u32,
    pub prim_mask: u32,
    pub max_tbo_size: u32,
    pub max_uniform_blocks: u32,
    pub max_viewports: u32,
    pub max_texture_gather_components: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct virgl_video_caps {
    pub _bitfield_1: u32,
    pub _bitfield_2: u32,
    pub _bitfield_3: u32,
    pub _bitfield_4: u32,
}

impl virgl_video_caps {
    bitfields! {
        _bitfield_1 {
            profile, set_profile: 0, 8;
            entrypoint, set_entrypoint: 8, 8;
            max_level, set_max_level: 16, 8;
            stacked_frames, set_stacked_frames: 24, 8;
        }
        _bitfield_2 {
            max_width, set_max_width: 0, 16;
            max_height, set_max_height: 16, 16;
        }
        _bitfield_3 {
            prefered_format, set_prefered_format: 0, 16;
            max_macroblocks, set_max_macroblocks: 16, 16;
        }
        _bitfield_4 {
            npot_texture, set_npot_texture: 0, 1;
            supports_progressive, set_supports_progressive: 1, 1;
            supports_interlaced, set_supports_interlaced: 2, 1;
            prefers_interlaced, set_prefers_interlaced: 3, 1;
            max_temporal_layers, set_max_temporal_layers: 4, 8;
            reserved, set_reserved: 12, 20;
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct virgl_caps_v2 {
    pub v1: virgl_caps_v1,
    pub min_aliased_point_size: f32,
    pub max_aliased_point_size: f32,
    pub min_smooth_point_size: f32,
    pub max_smooth_point_size: f32,
    pub min_aliased_line_width: f32,
    pub max_aliased_line_width: f32,
    pub min_smooth_line_width: f32,
    pub max_smooth_line_width: f32,
    pub max_texture_lod_bias: f32,
    pub max_geom_output_vertices: u32,
    pub max_geom_total_output_components: u32,
    pub max_vertex_outputs: u32,
    pub max_vertex_attribs: u32,
    pub max_shader_patch_varyings: u32,
    pub min_texel_offset: i32,
    pub max_texel_offset: i32,
    pub min_texture_gather_offset: i32,
    pub max_texture_gather_offset: i32,
    pub texture_buffer_offset_alignment: u32,
    pub uniform_buffer_offset_alignment: u32,
    pub shader_buffer_offset_alignment: u32,
    pub capability_bits: u32,
    pub sample_locations: [u32; 8],
    pub max_vertex_attrib_stride: u32,
    pub max_shader_buffer_frag_compute: u32,
    pub max_shader_buffer_other_stages: u32,
    pub max_shader_image_frag_compute: u32,
    pub max_shader_image_other_stages: u32,
    pub max_image_samples: u32,
    pub max_compute_work_group_invocations: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_grid_size: [u32; 3],
    pub max_compute_block_size: [u32; 3],
    pub max_texture_2d_size: u32,
    pub max_texture_3d_size: u32,
    pub max_texture_cube_size: u32,
    pub max_combined_shader_buffers: u32,
    pub max_atomic_counters: [u32; 6],
    pub max_atomic_counter_buffers: [u32; 6],
    pub max_combined_atomic_counters: u32,
    pub max_combined_atomic_counter_buffers: u32,
    pub host_feature_check_version: u32,
    pub supported_readback_formats: virgl_supported_format_mask,
    pub scanout: virgl_supported_format_mask,
    pub capability_bits_v2: u32,
    pub max_video_memory: u32,
    pub renderer: [c_char; 64],
    pub max_anisotropy: f32,
    pub max_texture_samplers: u32,
    pub supported_multisample_formats: virgl_supported_format_mask,
    pub max_const_buffer_size: [u32; 6],
    pub num_video_caps: u32,
    pub video_caps: [virgl_video_caps; 32],
    pub max_uniform_block_size: u32,
    pub max_tcs_outputs: u32,
    pub max_tes_outputs: u32,
    pub max_shader_storage_blocks: [u32; 6],
}

pub const VIRGL_CAP_HOST_IS_GLES: u32 = 1 << 19;
pub const VIRGL_CAP_APP_TWEAK_SUPPORT: u32 = 1 << 28;
pub const VIRGL_CAP_V2_QUERY_FORMAT_MODIFIER: u32 = 1 << 19;
