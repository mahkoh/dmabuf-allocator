use fake_kernel::uapi::drm_fourcc::fourcc_code;

pub const GBM_BO_FORMAT_XRGB8888: u32 = 0;
pub const GBM_BO_FORMAT_ARGB8888: u32 = 1;

pub const GBM_FORMAT_XRGB8888: u32 = fourcc_code(b'X', b'R', b'2', b'4');
pub const GBM_FORMAT_ARGB8888: u32 = fourcc_code(b'A', b'R', b'2', b'4');

pub const GBM_BO_USE_SCANOUT: u32 = 1 << 0;
pub const GBM_BO_USE_RENDERING: u32 = 1 << 2;
pub const GBM_BO_USE_LINEAR: u32 = 1 << 4;
