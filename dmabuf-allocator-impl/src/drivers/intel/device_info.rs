#[derive(Copy, Clone)]
pub(crate) struct PatEntry {
    pub(crate) index: u32,
}

#[derive(Copy, Clone)]
pub(crate) struct Pat {
    pub(crate) scanout: PatEntry,
    pub(crate) writecombining: PatEntry,
    pub(crate) compressed: PatEntry,
    pub(crate) compressed_scanout: PatEntry,
}

#[derive(Copy, Clone)]
pub struct DeviceInfo {
    pub pci_id: u16,
    pub ver: u32,
    pub verx10: u32,
    pub dg2: bool,
    pub mtl_or_arl: bool,
    pub skl: bool,
    pub gt: u32,
    pub has_llc: bool,
    pub has_local_mem: bool,
    pub has_aux_map: bool,
    pub has_flat_ccs: bool,
    pub clear_color_state_size: u64,
    pub(crate) pat: Pat,
}
