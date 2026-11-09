pub(crate) mod amdgpu;
mod gfx6;
mod modifier;
mod uapi;
pub(crate) mod virtio;

pub(crate) struct AmdColor {
    pub(crate) data_format: u8,
    pub(crate) number_type: u8,
}

pub(crate) struct AmdGfx6Image {
    pub(crate) data_format: u8,
    pub(crate) num_format: u8,
    pub(crate) dst_sel: [u8; 4],
    pub(crate) alpha_is_on_msb: bool,
}
