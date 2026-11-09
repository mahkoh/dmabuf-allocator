use crate::drivers::intel::Vram;
use crate::drivers::intel::device_info::DeviceInfo;
use crate::drivers::intel::device_info::Pat;
use crate::drivers::intel::device_info::PatEntry;
use crate::drivers::intel::heap;
use crate::drivers::intel::layout::I915_FORMAT_MOD_4_TILED_LNL_CCS;

#[test]
fn llc_without_vram_ignores_compression() {
    let pat_entry = PatEntry { index: 0 };
    let info = DeviceInfo {
        pci_id: 0,
        ver: 20,
        verx10: 200,
        dg2: false,
        mtl_or_arl: false,
        skl: false,
        gt: 0,
        has_llc: true,
        has_local_mem: false,
        has_aux_map: false,
        has_flat_ccs: false,
        clear_color_state_size: 0,
        pat: Pat {
            scanout: pat_entry,
            writecombining: pat_entry,
            compressed: pat_entry,
            compressed_scanout: pat_entry,
        },
    };
    let vram = Vram {
        size: 0,
        all_mappable: true,
    };
    assert!(!heap(Some(&info), &vram, I915_FORMAT_MOD_4_TILED_LNL_CCS).compressed);
}
