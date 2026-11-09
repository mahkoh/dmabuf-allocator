#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct virgl_renderer_capset_venus {
    pub wire_format_version: u32,
    pub vk_xml_version: u32,
    pub vk_ext_command_serialization_spec_version: u32,
    pub vk_mesa_venus_protocol_spec_version: u32,
    pub supports_blob_id_0: u32,
    pub vk_extension_mask1: [u32; 32],
    pub allow_vk_wait_syncs: u32,
    pub supports_multiple_timelines: u32,
    pub use_guest_vram: u32,
}
