#![allow(non_camel_case_types)]

use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

#[cfg(test)]
mod tests;

pub const GBM_BACKEND_ABI_VERSION: u32 = 1;

#[repr(C)]
pub struct gbm_backend_desc {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct gbm_bo {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct gbm_surface {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union gbm_bo_handle {
    pub ptr: *mut c_void,
    pub s32: i32,
    pub u32: u32,
    pub s64: i64,
    pub u64: u64,
}

#[repr(C)]
pub struct gbm_device_v0 {
    pub backend_desc: *const gbm_backend_desc,
    pub backend_version: u32,
    pub fd: c_int,
    pub name: *const c_char,
    pub destroy: Option<unsafe extern "C" fn(gbm: *mut gbm_device)>,
    pub is_format_supported:
        Option<unsafe extern "C" fn(gbm: *mut gbm_device, format: u32, usage: u32) -> c_int>,
    pub get_format_modifier_plane_count:
        Option<unsafe extern "C" fn(device: *mut gbm_device, format: u32, modifier: u64) -> c_int>,
    pub bo_create: Option<
        unsafe extern "C" fn(
            gbm: *mut gbm_device,
            width: u32,
            height: u32,
            format: u32,
            usage: u32,
            modifiers: *const u64,
            count: c_uint,
        ) -> *mut gbm_bo,
    >,
    pub bo_import: Option<
        unsafe extern "C" fn(
            gbm: *mut gbm_device,
            type_: u32,
            buffer: *mut c_void,
            usage: u32,
        ) -> *mut gbm_bo,
    >,
    pub bo_map: Option<
        unsafe extern "C" fn(
            bo: *mut gbm_bo,
            x: u32,
            y: u32,
            width: u32,
            height: u32,
            flags: u32,
            stride: *mut u32,
            map_data: *mut *mut c_void,
        ) -> *mut c_void,
    >,
    pub bo_unmap: Option<unsafe extern "C" fn(bo: *mut gbm_bo, map_data: *mut c_void)>,
    pub bo_write:
        Option<unsafe extern "C" fn(bo: *mut gbm_bo, buf: *const c_void, data: usize) -> c_int>,
    pub bo_get_fd: Option<unsafe extern "C" fn(bo: *mut gbm_bo) -> c_int>,
    pub bo_get_planes: Option<unsafe extern "C" fn(bo: *mut gbm_bo) -> c_int>,
    pub bo_get_handle: Option<unsafe extern "C" fn(bo: *mut gbm_bo, plane: c_int) -> gbm_bo_handle>,
    pub bo_get_plane_fd: Option<unsafe extern "C" fn(bo: *mut gbm_bo, plane: c_int) -> c_int>,
    pub bo_get_stride: Option<unsafe extern "C" fn(bo: *mut gbm_bo, plane: c_int) -> u32>,
    pub bo_get_offset: Option<unsafe extern "C" fn(bo: *mut gbm_bo, plane: c_int) -> u32>,
    pub bo_get_modifier: Option<unsafe extern "C" fn(bo: *mut gbm_bo) -> u64>,
    pub bo_destroy: Option<unsafe extern "C" fn(bo: *mut gbm_bo)>,
    pub surface_create: Option<
        unsafe extern "C" fn(
            gbm: *mut gbm_device,
            width: u32,
            height: u32,
            format: u32,
            flags: u32,
            modifiers: *const u64,
            count: c_uint,
        ) -> *mut gbm_surface,
    >,
    pub surface_lock_front_buffer:
        Option<unsafe extern "C" fn(surface: *mut gbm_surface) -> *mut gbm_bo>,
    pub surface_release_buffer:
        Option<unsafe extern "C" fn(surface: *mut gbm_surface, bo: *mut gbm_bo)>,
    pub surface_has_free_buffers: Option<unsafe extern "C" fn(surface: *mut gbm_surface) -> c_int>,
    pub surface_destroy: Option<unsafe extern "C" fn(surface: *mut gbm_surface)>,
}

#[repr(C)]
pub struct gbm_device {
    pub dummy: Option<unsafe extern "C" fn(c_int) -> *mut gbm_device>,
    pub v0: gbm_device_v0,
}

#[repr(C)]
pub struct gbm_backend_v0 {
    pub backend_version: u32,
    pub backend_name: *const c_char,
    pub create_device:
        Option<unsafe extern "C" fn(fd: c_int, gbm_backend_version: u32) -> *mut gbm_device>,
}

#[repr(C)]
pub struct gbm_backend {
    pub v0: gbm_backend_v0,
}

#[repr(C)]
pub struct gbm_core_v0 {
    pub core_version: u32,
    pub format_canonicalize: Option<unsafe extern "C" fn(gbm_format: u32) -> u32>,
}

#[repr(C)]
pub struct gbm_core {
    pub v0: gbm_core_v0,
}

pub const GBM_GET_BACKEND_PROC_NAME: &std::ffi::CStr = c"gbmint_get_backend";

pub type GBM_GET_BACKEND_PROC_PTR =
    unsafe extern "C" fn(gbm_core: *const gbm_core) -> *const gbm_backend;
