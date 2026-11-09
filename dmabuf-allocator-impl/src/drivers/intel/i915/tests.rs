use crate::drivers::intel::i915::I915_MEMORY_CLASS_DEVICE;
use crate::drivers::intel::i915::I915_MEMORY_CLASS_SYSTEM;
use crate::drivers::intel::i915::Regions;
use crate::drivers::intel::i915::drm_i915_gem_memory_class_instance;
use crate::drivers::intel::i915::drm_i915_memory_region_info;
use crate::drivers::intel::i915::drm_i915_query_memory_regions;
use crate::drivers::intel::i915::memory_regions;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::offset_of;

const GIB: u64 = 1 << 30;
const SYSTEM_INSTANCE: u16 = 1;

struct Region {
    memory_class: u16,
    memory_instance: u16,
    probed_size: u64,
    probed_cpu_visible_size: u64,
}

fn put(data: &mut [u8], offset: usize, bytes: &[u8]) {
    data[offset..offset + bytes.len()].copy_from_slice(bytes);
}

fn regions_data(regions: &[Region]) -> Vec<u8> {
    let header = size_of::<drm_i915_query_memory_regions>();
    let info_size = size_of::<drm_i915_memory_region_info>();
    let mut data = vec![0u8; header + regions.len() * info_size];
    let num_regions = offset_of!(drm_i915_query_memory_regions, num_regions);
    put(
        &mut data,
        num_regions,
        &(regions.len() as u32).to_ne_bytes(),
    );
    for (index, region) in regions.iter().enumerate() {
        let base = header + index * info_size;
        let instance = base + offset_of!(drm_i915_memory_region_info, region);
        let memory_class = instance + offset_of!(drm_i915_gem_memory_class_instance, memory_class);
        let memory_instance =
            instance + offset_of!(drm_i915_gem_memory_class_instance, memory_instance);
        let probed_size = base + offset_of!(drm_i915_memory_region_info, probed_size);
        let visible = base
            + offset_of!(
                drm_i915_memory_region_info,
                tail.cpu_visible.probed_cpu_visible_size
            );
        put(&mut data, memory_class, &region.memory_class.to_ne_bytes());
        put(
            &mut data,
            memory_instance,
            &region.memory_instance.to_ne_bytes(),
        );
        put(&mut data, probed_size, &region.probed_size.to_ne_bytes());
        put(
            &mut data,
            visible,
            &region.probed_cpu_visible_size.to_ne_bytes(),
        );
    }
    data
}

fn parse(data: &[u8], filled_length: i32) -> Option<Regions> {
    memory_regions(|buffer| match buffer.len() {
        0 => Ok(data.len() as i32),
        _ => {
            buffer.copy_from_slice(data);
            Ok(filled_length)
        }
    })
    .ok()
}

fn system(memory_instance: u16) -> Region {
    Region {
        memory_class: I915_MEMORY_CLASS_SYSTEM,
        memory_instance,
        probed_size: GIB,
        probed_cpu_visible_size: GIB,
    }
}

fn device(probed_cpu_visible_size: u64) -> Region {
    Region {
        memory_class: I915_MEMORY_CLASS_DEVICE,
        memory_instance: 0,
        probed_size: GIB,
        probed_cpu_visible_size,
    }
}

#[test]
fn every_region_is_read_at_its_offset() {
    let data = regions_data(&[system(SYSTEM_INSTANCE), device(GIB)]);
    let regions = parse(&data, data.len() as i32).unwrap();
    assert_eq!(regions.system.memory_class, I915_MEMORY_CLASS_SYSTEM);
    assert_eq!(regions.system.memory_instance, SYSTEM_INSTANCE);
    assert_eq!(
        regions.device.unwrap().memory_class,
        I915_MEMORY_CLASS_DEVICE
    );
    assert_eq!(regions.vram.size, GIB);
    assert!(regions.vram.all_mappable);
}

#[test]
fn unreported_cpu_visibility_maps_the_whole_region() {
    let data = regions_data(&[system(0), device(0)]);
    let regions = parse(&data, data.len() as i32).unwrap();
    assert_eq!(regions.vram.size, GIB);
    assert!(regions.vram.all_mappable);
}

#[test]
fn partial_cpu_visibility_is_not_all_mappable() {
    let data = regions_data(&[system(0), device(GIB / 4)]);
    let regions = parse(&data, data.len() as i32).unwrap();
    assert_eq!(regions.vram.size, GIB);
    assert!(!regions.vram.all_mappable);
}

#[test]
fn header_without_regions_uses_the_default_system_region() {
    let data = regions_data(&[]);
    let regions = parse(&data, data.len() as i32).unwrap();
    assert_eq!(regions.system.memory_class, I915_MEMORY_CLASS_SYSTEM);
    assert_eq!(regions.system.memory_instance, 0);
    assert!(regions.device.is_none());
    assert_eq!(regions.vram.size, 0);
}

#[test]
fn truncated_header_has_no_regions() {
    let data = regions_data(&[]);
    let truncated = size_of::<drm_i915_query_memory_regions>() as i32 / 2;
    assert!(parse(&data, truncated).is_none());
}

#[test]
fn failed_item_has_no_regions() {
    let data = regions_data(&[system(0)]);
    assert!(parse(&data, -libc::EINVAL).is_none());
}
