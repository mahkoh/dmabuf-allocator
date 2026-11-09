use super::*;
use std::mem::offset_of;

#[test]
fn the_device_matches_the_c_layout() {
    assert_eq!(size_of::<gbm_device>(), 200);
    assert_eq!(align_of::<gbm_device>(), 8);
    assert_eq!(offset_of!(gbm_device, dummy), 0);
    assert_eq!(offset_of!(gbm_device, v0), 8);
    assert_eq!(size_of::<gbm_device_v0>(), 192);
    assert_eq!(offset_of!(gbm_device_v0, backend_desc), 0);
    assert_eq!(offset_of!(gbm_device_v0, backend_version), 8);
    assert_eq!(offset_of!(gbm_device_v0, fd), 12);
    assert_eq!(offset_of!(gbm_device_v0, name), 16);
    assert_eq!(offset_of!(gbm_device_v0, destroy), 24);
    assert_eq!(offset_of!(gbm_device_v0, is_format_supported), 32);
    assert_eq!(
        offset_of!(gbm_device_v0, get_format_modifier_plane_count),
        40
    );
    assert_eq!(offset_of!(gbm_device_v0, bo_create), 48);
    assert_eq!(offset_of!(gbm_device_v0, bo_import), 56);
    assert_eq!(offset_of!(gbm_device_v0, bo_map), 64);
    assert_eq!(offset_of!(gbm_device_v0, bo_unmap), 72);
    assert_eq!(offset_of!(gbm_device_v0, bo_write), 80);
    assert_eq!(offset_of!(gbm_device_v0, bo_get_fd), 88);
    assert_eq!(offset_of!(gbm_device_v0, bo_get_planes), 96);
    assert_eq!(offset_of!(gbm_device_v0, bo_get_handle), 104);
    assert_eq!(offset_of!(gbm_device_v0, bo_get_plane_fd), 112);
    assert_eq!(offset_of!(gbm_device_v0, bo_get_stride), 120);
    assert_eq!(offset_of!(gbm_device_v0, bo_get_offset), 128);
    assert_eq!(offset_of!(gbm_device_v0, bo_get_modifier), 136);
    assert_eq!(offset_of!(gbm_device_v0, bo_destroy), 144);
    assert_eq!(offset_of!(gbm_device_v0, surface_create), 152);
    assert_eq!(offset_of!(gbm_device_v0, surface_lock_front_buffer), 160);
    assert_eq!(offset_of!(gbm_device_v0, surface_release_buffer), 168);
    assert_eq!(offset_of!(gbm_device_v0, surface_has_free_buffers), 176);
    assert_eq!(offset_of!(gbm_device_v0, surface_destroy), 184);
}

#[test]
fn the_backend_and_the_core_match_the_c_layout() {
    assert_eq!(size_of::<gbm_backend>(), 24);
    assert_eq!(offset_of!(gbm_backend, v0), 0);
    assert_eq!(offset_of!(gbm_backend_v0, backend_version), 0);
    assert_eq!(offset_of!(gbm_backend_v0, backend_name), 8);
    assert_eq!(offset_of!(gbm_backend_v0, create_device), 16);
    assert_eq!(size_of::<gbm_core>(), 16);
    assert_eq!(offset_of!(gbm_core, v0), 0);
    assert_eq!(offset_of!(gbm_core_v0, core_version), 0);
    assert_eq!(offset_of!(gbm_core_v0, format_canonicalize), 8);
    assert_eq!(size_of::<gbm_bo_handle>(), 8);
    assert_eq!(align_of::<gbm_bo_handle>(), 8);
}
