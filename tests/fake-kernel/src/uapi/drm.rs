use crate::uapi::ioctl::_IO;
use crate::uapi::ioctl::_IOR;
use crate::uapi::ioctl::_IOW;
use crate::uapi::ioctl::_IOWR;
use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_long;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_ulonglong;
use core::ffi::c_ushort;

pub type __kernel_size_t = usize;

pub type drm_drawable_t = c_uint;
pub type drm_magic_t = c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_clip_rect {
    pub x1: c_ushort,
    pub y1: c_ushort,
    pub x2: c_ushort,
    pub y2: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_version {
    pub version_major: c_int,
    pub version_minor: c_int,
    pub version_patchlevel: c_int,
    pub name_len: __kernel_size_t,
    pub name: *mut c_char,
    pub date_len: __kernel_size_t,
    pub date: *mut c_char,
    pub desc_len: __kernel_size_t,
    pub desc: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_unique {
    pub unique_len: __kernel_size_t,
    pub unique: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_block {
    pub unused: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_client {
    pub idx: c_int,
    pub auth: c_int,
    pub pid: c_ulong,
    pub uid: c_ulong,
    pub magic: c_ulong,
    pub iocs: c_ulong,
}

pub type drm_stat_type = c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_stats__data {
    pub value: c_ulong,
    pub type_: drm_stat_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_stats {
    pub count: c_ulong,
    pub data: [drm_stats__data; 15],
}

pub type drm_lock_flags = c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lock {
    pub context: c_int,
    pub flags: drm_lock_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_draw {
    pub handle: drm_drawable_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_update_draw {
    pub handle: drm_drawable_t,
    pub type_: c_uint,
    pub num: c_uint,
    pub data: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_auth {
    pub magic: drm_magic_t,
}

pub type drm_vblank_seq_type = c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_wait_vblank_request {
    pub type_: drm_vblank_seq_type,
    pub sequence: c_uint,
    pub signal: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_wait_vblank_reply {
    pub type_: drm_vblank_seq_type,
    pub sequence: c_uint,
    pub tval_sec: c_long,
    pub tval_usec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_wait_vblank {
    pub request: drm_wait_vblank_request,
    pub reply: drm_wait_vblank_reply,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_set_version {
    pub drm_di_major: c_int,
    pub drm_di_minor: c_int,
    pub drm_dd_major: c_int,
    pub drm_dd_minor: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_gem_close {
    pub handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_flink {
    pub handle: u32,
    pub name: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_open {
    pub name: u32,
    pub handle: u32,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_change_handle {
    pub handle: u32,
    pub new_handle: u32,
}

pub const DRM_CAP_DUMB_BUFFER: u64 = 0x1;
pub const DRM_CAP_VBLANK_HIGH_CRTC: u64 = 0x2;
pub const DRM_CAP_DUMB_PREFERRED_DEPTH: u64 = 0x3;
pub const DRM_CAP_DUMB_PREFER_SHADOW: u64 = 0x4;
pub const DRM_CAP_PRIME: u64 = 0x5;
pub const DRM_PRIME_CAP_IMPORT: u64 = 0x1;
pub const DRM_PRIME_CAP_EXPORT: u64 = 0x2;
pub const DRM_CAP_TIMESTAMP_MONOTONIC: u64 = 0x6;
pub const DRM_CAP_ASYNC_PAGE_FLIP: u64 = 0x7;
pub const DRM_CAP_CURSOR_WIDTH: u64 = 0x8;
pub const DRM_CAP_CURSOR_HEIGHT: u64 = 0x9;
pub const DRM_CAP_ADDFB2_MODIFIERS: u64 = 0x10;
pub const DRM_CAP_PAGE_FLIP_TARGET: u64 = 0x11;
pub const DRM_CAP_CRTC_IN_VBLANK_EVENT: u64 = 0x12;
pub const DRM_CAP_SYNCOBJ: u64 = 0x13;
pub const DRM_CAP_SYNCOBJ_TIMELINE: u64 = 0x14;
pub const DRM_CAP_ATOMIC_ASYNC_PAGE_FLIP: u64 = 0x15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_get_cap {
    pub capability: u64,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_set_client_cap {
    pub capability: u64,
    pub value: u64,
}

pub const DRM_RDWR: u32 = libc::O_RDWR as u32;
pub const DRM_CLOEXEC: u32 = libc::O_CLOEXEC as u32;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_prime_handle {
    pub handle: u32,
    pub flags: u32,
    pub fd: i32,
}

pub const DRM_SYNCOBJ_CREATE_SIGNALED: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_syncobj_create {
    pub handle: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_destroy {
    pub handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_handle {
    pub handle: u32,
    pub flags: u32,
    pub fd: i32,
    pub pad: u32,
    pub point: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_transfer {
    pub src_handle: u32,
    pub dst_handle: u32,
    pub src_point: u64,
    pub dst_point: u64,
    pub flags: u32,
    pub pad: u32,
}

pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_ALL: u32 = 1 << 0;
pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT: u32 = 1 << 1;
pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_AVAILABLE: u32 = 1 << 2;
pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_DEADLINE: u32 = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_wait {
    pub handles: u64,
    pub timeout_nsec: i64,
    pub count_handles: u32,
    pub flags: u32,
    pub first_signaled: u32,
    pub pad: u32,
    pub deadline_nsec: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_timeline_wait {
    pub handles: u64,
    pub points: u64,
    pub timeout_nsec: i64,
    pub count_handles: u32,
    pub flags: u32,
    pub first_signaled: u32,
    pub pad: u32,
    pub deadline_nsec: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_eventfd {
    pub handle: u32,
    pub flags: u32,
    pub point: u64,
    pub fd: i32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_array {
    pub handles: u64,
    pub count_handles: u32,
    pub pad: u32,
}

pub const DRM_SYNCOBJ_QUERY_FLAGS_LAST_SUBMITTED: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_timeline_array {
    pub handles: u64,
    pub points: u64,
    pub count_handles: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_crtc_get_sequence {
    pub crtc_id: u32,
    pub active: u32,
    pub sequence: u64,
    pub sequence_ns: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_crtc_queue_sequence {
    pub crtc_id: u32,
    pub flags: u32,
    pub sequence: u64,
    pub user_data: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_set_client_name {
    pub name_len: u64,
    pub name: u64,
}

pub const DRM_DISPLAY_MODE_LEN: usize = 32;
pub const DRM_PROP_NAME_LEN: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_modeinfo {
    pub clock: u32,
    pub hdisplay: u16,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub vtotal: u16,
    pub vscan: u16,
    pub vrefresh: u32,
    pub flags: u32,
    pub type_: u32,
    pub name: [c_char; DRM_DISPLAY_MODE_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_card_res {
    pub fb_id_ptr: u64,
    pub crtc_id_ptr: u64,
    pub connector_id_ptr: u64,
    pub encoder_id_ptr: u64,
    pub count_fbs: u32,
    pub count_crtcs: u32,
    pub count_connectors: u32,
    pub count_encoders: u32,
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_crtc {
    pub set_connectors_ptr: u64,
    pub count_connectors: u32,
    pub crtc_id: u32,
    pub fb_id: u32,
    pub x: u32,
    pub y: u32,
    pub gamma_size: u32,
    pub mode_valid: u32,
    pub mode: drm_mode_modeinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_set_plane {
    pub plane_id: u32,
    pub crtc_id: u32,
    pub fb_id: u32,
    pub flags: u32,
    pub crtc_x: i32,
    pub crtc_y: i32,
    pub crtc_w: u32,
    pub crtc_h: u32,
    pub src_x: u32,
    pub src_y: u32,
    pub src_h: u32,
    pub src_w: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_plane {
    pub plane_id: u32,
    pub crtc_id: u32,
    pub fb_id: u32,
    pub possible_crtcs: u32,
    pub gamma_size: u32,
    pub count_format_types: u32,
    pub format_type_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_plane_res {
    pub plane_id_ptr: u64,
    pub count_planes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_encoder {
    pub encoder_id: u32,
    pub encoder_type: u32,
    pub crtc_id: u32,
    pub possible_crtcs: u32,
    pub possible_clones: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_connector {
    pub encoders_ptr: u64,
    pub modes_ptr: u64,
    pub props_ptr: u64,
    pub prop_values_ptr: u64,
    pub count_modes: u32,
    pub count_props: u32,
    pub count_encoders: u32,
    pub encoder_id: u32,
    pub connector_id: u32,
    pub connector_type: u32,
    pub connector_type_id: u32,
    pub connection: u32,
    pub mm_width: u32,
    pub mm_height: u32,
    pub subpixel: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_property_enum {
    pub value: u64,
    pub name: [c_char; DRM_PROP_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_property {
    pub values_ptr: u64,
    pub enum_blob_ptr: u64,
    pub prop_id: u32,
    pub flags: u32,
    pub name: [c_char; DRM_PROP_NAME_LEN],
    pub count_values: u32,
    pub count_enum_blobs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_connector_set_property {
    pub value: u64,
    pub prop_id: u32,
    pub connector_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_obj_get_properties {
    pub props_ptr: u64,
    pub prop_values_ptr: u64,
    pub count_props: u32,
    pub obj_id: u32,
    pub obj_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_obj_set_property {
    pub value: u64,
    pub prop_id: u32,
    pub obj_id: u32,
    pub obj_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_blob {
    pub blob_id: u32,
    pub length: u32,
    pub data: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_fb_cmd {
    pub fb_id: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u32,
    pub depth: u32,
    pub handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_fb_cmd2 {
    pub fb_id: u32,
    pub width: u32,
    pub height: u32,
    pub pixel_format: u32,
    pub flags: u32,
    pub handles: [u32; 4],
    pub pitches: [u32; 4],
    pub offsets: [u32; 4],
    pub modifier: [u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_fb_dirty_cmd {
    pub fb_id: u32,
    pub flags: u32,
    pub color: u32,
    pub num_clips: u32,
    pub clips_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_mode_cmd {
    pub connector_id: u32,
    pub mode: drm_mode_modeinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_cursor {
    pub flags: u32,
    pub crtc_id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_cursor2 {
    pub flags: u32,
    pub crtc_id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub handle: u32,
    pub hot_x: i32,
    pub hot_y: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_crtc_lut {
    pub crtc_id: u32,
    pub gamma_size: u32,
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_crtc_page_flip {
    pub crtc_id: u32,
    pub fb_id: u32,
    pub flags: u32,
    pub reserved: u32,
    pub user_data: u64,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
pub struct drm_mode_create_dumb {
    pub height: u32,
    pub width: u32,
    pub bpp: u32,
    pub flags: u32,
    pub handle: u32,
    pub pitch: u32,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_map_dumb {
    pub handle: u32,
    pub pad: u32,
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_destroy_dumb {
    pub handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_atomic {
    pub flags: u32,
    pub count_objs: u32,
    pub objs_ptr: u64,
    pub count_props_ptr: u64,
    pub props_ptr: u64,
    pub prop_values_ptr: u64,
    pub reserved: u64,
    pub user_data: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_create_blob {
    pub data: u64,
    pub length: u32,
    pub blob_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_destroy_blob {
    pub blob_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_create_lease {
    pub object_ids: u64,
    pub object_count: u32,
    pub flags: u32,
    pub lessee_id: u32,
    pub fd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_list_lessees {
    pub count_lessees: u32,
    pub pad: u32,
    pub lessees_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_lease {
    pub count_objects: u32,
    pub pad: u32,
    pub objects_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_revoke_lease {
    pub lessee_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_closefb {
    pub fb_id: u32,
    pub pad: u32,
}

pub const DRM_IOCTL_BASE: u32 = b'd' as u32;

pub const fn DRM_IO(nr: u32) -> u32 {
    _IO(DRM_IOCTL_BASE, nr)
}

pub const fn DRM_IOR<T>(nr: u32) -> u32 {
    _IOR::<T>(DRM_IOCTL_BASE, nr)
}

pub const fn DRM_IOW<T>(nr: u32) -> u32 {
    _IOW::<T>(DRM_IOCTL_BASE, nr)
}

pub const fn DRM_IOWR<T>(nr: u32) -> u32 {
    _IOWR::<T>(DRM_IOCTL_BASE, nr)
}

pub const DRM_IOCTL_VERSION: u32 = DRM_IOWR::<drm_version>(0x00);
pub const DRM_IOCTL_GET_UNIQUE: u32 = DRM_IOWR::<drm_unique>(0x01);
pub const DRM_IOCTL_GET_MAGIC: u32 = DRM_IOR::<drm_auth>(0x02);
pub const DRM_IOCTL_GET_CLIENT: u32 = DRM_IOWR::<drm_client>(0x05);
pub const DRM_IOCTL_GET_STATS: u32 = DRM_IOR::<drm_stats>(0x06);
pub const DRM_IOCTL_SET_VERSION: u32 = DRM_IOWR::<drm_set_version>(0x07);
pub const DRM_IOCTL_GEM_CLOSE: u32 = DRM_IOW::<drm_gem_close>(0x09);
pub const DRM_IOCTL_GEM_FLINK: u32 = DRM_IOWR::<drm_gem_flink>(0x0a);
pub const DRM_IOCTL_GEM_OPEN: u32 = DRM_IOWR::<drm_gem_open>(0x0b);
pub const DRM_IOCTL_GET_CAP: u32 = DRM_IOWR::<drm_get_cap>(0x0c);
pub const DRM_IOCTL_SET_CLIENT_CAP: u32 = DRM_IOW::<drm_set_client_cap>(0x0d);

pub const DRM_IOCTL_SET_UNIQUE: u32 = DRM_IOW::<drm_unique>(0x10);
pub const DRM_IOCTL_AUTH_MAGIC: u32 = DRM_IOW::<drm_auth>(0x11);
pub const DRM_IOCTL_BLOCK: u32 = DRM_IOWR::<drm_block>(0x12);
pub const DRM_IOCTL_UNBLOCK: u32 = DRM_IOWR::<drm_block>(0x13);

pub const DRM_IOCTL_SET_MASTER: u32 = DRM_IO(0x1e);
pub const DRM_IOCTL_DROP_MASTER: u32 = DRM_IO(0x1f);

pub const DRM_IOCTL_ADD_DRAW: u32 = DRM_IOWR::<drm_draw>(0x27);
pub const DRM_IOCTL_RM_DRAW: u32 = DRM_IOWR::<drm_draw>(0x28);
pub const DRM_IOCTL_FINISH: u32 = DRM_IOW::<drm_lock>(0x2c);

pub const DRM_IOCTL_PRIME_HANDLE_TO_FD: u32 = DRM_IOWR::<drm_prime_handle>(0x2d);
pub const DRM_IOCTL_PRIME_FD_TO_HANDLE: u32 = DRM_IOWR::<drm_prime_handle>(0x2e);

pub const DRM_IOCTL_WAIT_VBLANK: u32 = DRM_IOWR::<drm_wait_vblank>(0x3a);

pub const DRM_IOCTL_CRTC_GET_SEQUENCE: u32 = DRM_IOWR::<drm_crtc_get_sequence>(0x3b);
pub const DRM_IOCTL_CRTC_QUEUE_SEQUENCE: u32 = DRM_IOWR::<drm_crtc_queue_sequence>(0x3c);

pub const DRM_IOCTL_UPDATE_DRAW: u32 = DRM_IOW::<drm_update_draw>(0x3f);

pub const DRM_IOCTL_MODE_GETRESOURCES: u32 = DRM_IOWR::<drm_mode_card_res>(0xA0);
pub const DRM_IOCTL_MODE_GETCRTC: u32 = DRM_IOWR::<drm_mode_crtc>(0xA1);
pub const DRM_IOCTL_MODE_SETCRTC: u32 = DRM_IOWR::<drm_mode_crtc>(0xA2);
pub const DRM_IOCTL_MODE_CURSOR: u32 = DRM_IOWR::<drm_mode_cursor>(0xA3);
pub const DRM_IOCTL_MODE_GETGAMMA: u32 = DRM_IOWR::<drm_mode_crtc_lut>(0xA4);
pub const DRM_IOCTL_MODE_SETGAMMA: u32 = DRM_IOWR::<drm_mode_crtc_lut>(0xA5);
pub const DRM_IOCTL_MODE_GETENCODER: u32 = DRM_IOWR::<drm_mode_get_encoder>(0xA6);
pub const DRM_IOCTL_MODE_GETCONNECTOR: u32 = DRM_IOWR::<drm_mode_get_connector>(0xA7);
pub const DRM_IOCTL_MODE_ATTACHMODE: u32 = DRM_IOWR::<drm_mode_mode_cmd>(0xA8);
pub const DRM_IOCTL_MODE_DETACHMODE: u32 = DRM_IOWR::<drm_mode_mode_cmd>(0xA9);

pub const DRM_IOCTL_MODE_GETPROPERTY: u32 = DRM_IOWR::<drm_mode_get_property>(0xAA);
pub const DRM_IOCTL_MODE_SETPROPERTY: u32 = DRM_IOWR::<drm_mode_connector_set_property>(0xAB);
pub const DRM_IOCTL_MODE_GETPROPBLOB: u32 = DRM_IOWR::<drm_mode_get_blob>(0xAC);
pub const DRM_IOCTL_MODE_GETFB: u32 = DRM_IOWR::<drm_mode_fb_cmd>(0xAD);
pub const DRM_IOCTL_MODE_ADDFB: u32 = DRM_IOWR::<drm_mode_fb_cmd>(0xAE);
pub const DRM_IOCTL_MODE_RMFB: u32 = DRM_IOWR::<c_uint>(0xAF);
pub const DRM_IOCTL_MODE_PAGE_FLIP: u32 = DRM_IOWR::<drm_mode_crtc_page_flip>(0xB0);
pub const DRM_IOCTL_MODE_DIRTYFB: u32 = DRM_IOWR::<drm_mode_fb_dirty_cmd>(0xB1);

pub const DRM_IOCTL_MODE_CREATE_DUMB: u32 = DRM_IOWR::<drm_mode_create_dumb>(0xB2);
pub const DRM_IOCTL_MODE_MAP_DUMB: u32 = DRM_IOWR::<drm_mode_map_dumb>(0xB3);
pub const DRM_IOCTL_MODE_DESTROY_DUMB: u32 = DRM_IOWR::<drm_mode_destroy_dumb>(0xB4);
pub const DRM_IOCTL_MODE_GETPLANERESOURCES: u32 = DRM_IOWR::<drm_mode_get_plane_res>(0xB5);
pub const DRM_IOCTL_MODE_GETPLANE: u32 = DRM_IOWR::<drm_mode_get_plane>(0xB6);
pub const DRM_IOCTL_MODE_SETPLANE: u32 = DRM_IOWR::<drm_mode_set_plane>(0xB7);
pub const DRM_IOCTL_MODE_ADDFB2: u32 = DRM_IOWR::<drm_mode_fb_cmd2>(0xB8);
pub const DRM_IOCTL_MODE_OBJ_GETPROPERTIES: u32 = DRM_IOWR::<drm_mode_obj_get_properties>(0xB9);
pub const DRM_IOCTL_MODE_OBJ_SETPROPERTY: u32 = DRM_IOWR::<drm_mode_obj_set_property>(0xBA);
pub const DRM_IOCTL_MODE_CURSOR2: u32 = DRM_IOWR::<drm_mode_cursor2>(0xBB);
pub const DRM_IOCTL_MODE_ATOMIC: u32 = DRM_IOWR::<drm_mode_atomic>(0xBC);
pub const DRM_IOCTL_MODE_CREATEPROPBLOB: u32 = DRM_IOWR::<drm_mode_create_blob>(0xBD);
pub const DRM_IOCTL_MODE_DESTROYPROPBLOB: u32 = DRM_IOWR::<drm_mode_destroy_blob>(0xBE);

pub const DRM_IOCTL_SYNCOBJ_CREATE: u32 = DRM_IOWR::<drm_syncobj_create>(0xBF);
pub const DRM_IOCTL_SYNCOBJ_DESTROY: u32 = DRM_IOWR::<drm_syncobj_destroy>(0xC0);
pub const DRM_IOCTL_SYNCOBJ_HANDLE_TO_FD: u32 = DRM_IOWR::<drm_syncobj_handle>(0xC1);
pub const DRM_IOCTL_SYNCOBJ_FD_TO_HANDLE: u32 = DRM_IOWR::<drm_syncobj_handle>(0xC2);
pub const DRM_IOCTL_SYNCOBJ_WAIT: u32 = DRM_IOWR::<drm_syncobj_wait>(0xC3);
pub const DRM_IOCTL_SYNCOBJ_RESET: u32 = DRM_IOWR::<drm_syncobj_array>(0xC4);
pub const DRM_IOCTL_SYNCOBJ_SIGNAL: u32 = DRM_IOWR::<drm_syncobj_array>(0xC5);

pub const DRM_IOCTL_MODE_CREATE_LEASE: u32 = DRM_IOWR::<drm_mode_create_lease>(0xC6);
pub const DRM_IOCTL_MODE_LIST_LESSEES: u32 = DRM_IOWR::<drm_mode_list_lessees>(0xC7);
pub const DRM_IOCTL_MODE_GET_LEASE: u32 = DRM_IOWR::<drm_mode_get_lease>(0xC8);
pub const DRM_IOCTL_MODE_REVOKE_LEASE: u32 = DRM_IOWR::<drm_mode_revoke_lease>(0xC9);

pub const DRM_IOCTL_SYNCOBJ_TIMELINE_WAIT: u32 = DRM_IOWR::<drm_syncobj_timeline_wait>(0xCA);
pub const DRM_IOCTL_SYNCOBJ_QUERY: u32 = DRM_IOWR::<drm_syncobj_timeline_array>(0xCB);
pub const DRM_IOCTL_SYNCOBJ_TRANSFER: u32 = DRM_IOWR::<drm_syncobj_transfer>(0xCC);
pub const DRM_IOCTL_SYNCOBJ_TIMELINE_SIGNAL: u32 = DRM_IOWR::<drm_syncobj_timeline_array>(0xCD);

pub const DRM_IOCTL_MODE_GETFB2: u32 = DRM_IOWR::<drm_mode_fb_cmd2>(0xCE);

pub const DRM_IOCTL_SYNCOBJ_EVENTFD: u32 = DRM_IOWR::<drm_syncobj_eventfd>(0xCF);

pub const DRM_IOCTL_MODE_CLOSEFB: u32 = DRM_IOWR::<drm_mode_closefb>(0xD0);

pub const DRM_IOCTL_SET_CLIENT_NAME: u32 = DRM_IOWR::<drm_set_client_name>(0xD1);

pub const DRM_IOCTL_GEM_CHANGE_HANDLE: u32 = DRM_IOWR::<drm_gem_change_handle>(0xD2);

pub const DRM_COMMAND_BASE: u32 = 0x40;
pub const DRM_COMMAND_END: u32 = 0xA0;
