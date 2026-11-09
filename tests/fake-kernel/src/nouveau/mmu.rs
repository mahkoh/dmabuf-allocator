pub struct Page {
    pub shift: u32,
    pub vram: bool,
    pub host: bool,
    pub comp: bool,
}

const fn page(shift: u32, vram: bool, host: bool, comp: bool) -> Page {
    Page {
        shift,
        vram,
        host,
        comp,
    }
}

pub struct Mmu {
    pub kinds: &'static [u8],
    pub invalid_kind: u8,
    pub pages: &'static [Page],
}

impl Mmu {
    pub fn kind_valid(&self, kind: u8) -> bool {
        kind == 0
            || ((kind as usize) < self.kinds.len()
                && self.kinds[kind as usize] != self.invalid_kind)
    }
}

pub static TU102: Mmu = Mmu {
    kinds: &TU102_KINDS,
    invalid_kind: 0x07,
    pages: &[
        page(47, false, false, false),
        page(38, false, false, false),
        page(29, false, false, false),
        page(21, true, false, true),
        page(16, true, false, true),
        page(12, true, true, false),
    ],
};

pub static GH100: Mmu = Mmu {
    kinds: &TU102_KINDS,
    invalid_kind: 0x07,
    pages: &[
        page(56, false, false, false),
        page(47, false, false, false),
        page(38, false, false, false),
        page(29, true, false, true),
        page(21, true, false, true),
        page(16, true, false, true),
        page(12, true, true, false),
    ],
};

pub const TU102_KINDS: [u8; 16] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x06, 0x06, 0x02, 0x01, 0x03, 0x04, 0x05, 0x07,
];
