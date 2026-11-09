use crate::uapi::ioctl::_IOW;

pub const UDMABUF_FLAGS_CLOEXEC: u32 = 0x01;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct udmabuf_create {
    pub memfd: u32,
    pub flags: u32,
    pub offset: u64,
    pub size: u64,
}

pub const UDMABUF_CREATE: u32 = _IOW::<udmabuf_create>(b'u' as u32, 0x42);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udmabuf_create_item {
    pub memfd: u32,
    pub __pad: u32,
    pub offset: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udmabuf_create_list {
    pub flags: u32,
    pub count: u32,
    pub list: [udmabuf_create_item; 0],
}

pub const UDMABUF_CREATE_LIST: u32 = _IOW::<udmabuf_create_list>(b'u' as u32, 0x43);
