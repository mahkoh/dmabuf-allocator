use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::allocator::Extra;
use crate::drivers::intel::device_info::DeviceInfo;
use crate::drivers::intel::devices::DEVICES;
use crate::drivers::intel::layout::I915_FORMAT_MOD_X_TILED;
use crate::drivers::intel::layout::I915_FORMAT_MOD_Y_TILED;
use crate::drivers::intel::layout::RANKED;
use crate::drivers::intel::layout::compute_layout;
use crate::drivers::intel::layout::has_ccs;
use crate::drivers::intel::layout::has_fast_clears;
use crate::drivers::intel::layout::supports_clear_color;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::PAGE_SIZE;
use crate::layout::align;
use crate::layout::best_layout;
use crate::layout::end;
use alloc::alloc::alloc_zeroed;
use alloc::vec::Vec;

pub mod device_info;
pub mod devices;
pub(crate) mod i915;
pub mod layout;
#[cfg(test)]
mod tests;
pub(crate) mod xe;

const MIB: u64 = 1024 * 1024;
const MAX_EXTENT: u32 = 16384;

struct Vram {
    size: u64,
    all_mappable: bool,
}

struct Heap {
    system: bool,
    vram: bool,
    needs_visible_vram: bool,
    compressed: bool,
}

struct UnsupportedDevice;

fn device_info(pci_id: u16) -> Result<Option<&'static DeviceInfo>, UnsupportedDevice> {
    match DEVICES.iter().find(|info| info.pci_id == pci_id) {
        Some(info) if info.ver < 8 => Err(UnsupportedDevice),
        info => Ok(info),
    }
}

fn choose_layout(
    config: &Config<'_>,
    info: Option<&DeviceInfo>,
    has_tiling_uapi: bool,
) -> Result<DeviceLayout, LayoutError> {
    let extents = 1..=MAX_EXTENT;
    if !extents.contains(&config.width) || !extents.contains(&config.height) {
        return Err(LayoutError::Size);
    }
    let layout = best_layout(
        config,
        |layout| layout,
        |modifier| compute_layout(config, modifier, info?),
        |modifier| {
            let interop = match modifier {
                I915_FORMAT_MOD_X_TILED => has_tiling_uapi,
                I915_FORMAT_MOD_Y_TILED => has_tiling_uapi && !config.scanout,
                _ => false,
            };
            if config.implicit_interop && !interop {
                return None;
            }
            let index = RANKED.iter().position(|&ranked| ranked == modifier)?;
            Some(index as u32)
        },
    )?;
    Ok(DeviceLayout {
        layout,
        extra: Extra::None,
    })
}

fn bo_size(layout: &Layout, info: Option<&DeviceInfo>, bucketed: bool, page_size: u64) -> u64 {
    let mut size = end(&layout.planes);
    if let Some(info) = info
        && has_fast_clears(layout.modifier)
        && info.clear_color_state_size != 0
        && !supports_clear_color(layout.modifier)
    {
        size = align(size, 64) + info.clear_color_state_size;
    }
    let page_size = page_size.max(1);
    let size = if !bucketed || size > 64 * MIB {
        align(size, page_size).max(page_size)
    } else if size <= 4 * MIB {
        size.next_power_of_two().max(PAGE_SIZE)
    } else if size <= 6 * MIB {
        6 * MIB
    } else if size <= 8 * MIB {
        8 * MIB
    } else {
        size.next_multiple_of(1 << (size.ilog2() - 2))
    };
    match size >= MIB {
        true => align(size, 2 * MIB),
        false => size,
    }
}

fn wants_compression(info: &DeviceInfo, modifier: u64) -> bool {
    info.ver >= 20 && has_ccs(modifier)
}

fn heap(info: Option<&DeviceInfo>, vram: &Vram, modifier: u64) -> Heap {
    let compressed = info.is_some_and(|info| wants_compression(info, modifier));
    if vram.size == 0 {
        return Heap {
            system: true,
            vram: false,
            needs_visible_vram: false,
            compressed: compressed && info.is_some_and(|info| !info.has_llc),
        };
    }
    let lmem = info.is_some_and(|info| info.has_local_mem) && has_ccs(modifier);
    Heap {
        system: !compressed && !lmem,
        vram: true,
        needs_visible_vram: !compressed
            && !vram.all_mappable
            && (!lmem || supports_clear_color(modifier)),
        compressed,
    }
}

unsafe fn records<H, R>(
    bytes: &[u8],
    count: impl FnOnce(H) -> u32,
) -> Option<impl Iterator<Item = R>> {
    let (header, records) = bytes.split_at_checked(size_of::<H>())?;
    let count = count(unsafe { header.as_ptr().cast::<H>().read_unaligned() });
    let records = records
        .chunks_exact(size_of::<R>())
        .take(count as usize)
        .map(|record| unsafe { record.as_ptr().cast::<R>().read_unaligned() });
    Some(records)
}

fn zeroed_buffer(len: usize) -> Option<Vec<u8>> {
    if len == 0 {
        return Some(Vec::new());
    }
    let layout = core::alloc::Layout::array::<u8>(len).ok()?;
    let ptr = unsafe { alloc_zeroed(layout) };
    if ptr.is_null() {
        return None;
    }
    Some(unsafe { Vec::from_raw_parts(ptr, len, len) })
}
