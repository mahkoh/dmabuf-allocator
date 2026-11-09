use ash::vk;

pub struct Format {
    pub fourcc: [u8; 4],
    pub format: dmabuf_allocator::Format,
    pub name: &'static str,
    pub vulkan: vk::Format,
    pub planes: &'static [Plane],
}

#[derive(Copy, Clone)]
pub struct Subsampling {
    pub x: u32,
    pub y: u32,
}

#[derive(Copy, Clone)]
pub enum Channel {
    R,
    G,
    B,
    Y,
    U,
    V,
    One,
}

pub struct Plane {
    pub view: vk::Format,
    pub subsampling: Subsampling,
    pub channels: [Channel; 4],
}

include!(concat!(env!("OUT_DIR"), "/formats.rs"));

pub fn find(fourcc: [u8; 4]) -> Option<&'static Format> {
    FORMATS.iter().find(|format| format.fourcc == fourcc)
}
