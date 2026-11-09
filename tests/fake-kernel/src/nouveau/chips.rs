use crate::KernelVersion;
use crate::nouveau::gpu;
use crate::nouveau::gpu::Gpu;
use crate::nouveau::mmu::GH100;
use crate::nouveau::mmu::Mmu;
use crate::nouveau::mmu::TU102;

const FERMI_TWOD_A: u32 = 0x902d;
const KEPLER_INLINE_TO_MEMORY_B: u32 = 0xa140;
const BLACKWELL_INLINE_TO_MEMORY_A: u32 = 0xcd40;
const ADA_A: u32 = 0xc997;
const BLACKWELL_B: u32 = 0xce97;
const AMPERE_DMA_COPY_B: u32 = 0xc7b5;
const BLACKWELL_DMA_COPY_B: u32 = 0xcab5;
const ADA_COMPUTE_A: u32 = 0xc9c0;
const BLACKWELL_COMPUTE_B: u32 = 0xcec0;

pub struct GrClasses {
    pub i2m: u32,
    pub twod: u32,
    pub threed: u32,
    pub compute: u32,
}

pub struct Hardware {
    pub gpu: &'static Gpu,
    pub mmu: &'static Mmu,
    pub gr: GrClasses,
    pub ce: u32,
}

pub struct Chip {
    pub name: &'static str,
    pub chipset: u16,
    pub pci_id: u16,
    pub first_kernel: Option<KernelVersion>,
    pub hardware: Option<Hardware>,
}

pub static CHIPS: &[Chip] = &[
    Chip {
        name: "AD102",
        chipset: 0x192,
        pci_id: 0x2684,
        first_kernel: None,
        hardware: Some(Hardware {
            gpu: &gpu::RTX_4090,
            mmu: &TU102,
            gr: GrClasses {
                i2m: KEPLER_INLINE_TO_MEMORY_B,
                twod: FERMI_TWOD_A,
                threed: ADA_A,
                compute: ADA_COMPUTE_A,
            },
            ce: AMPERE_DMA_COPY_B,
        }),
    },
    Chip {
        name: "GB202",
        chipset: 0x1b2,
        pci_id: 0x2b85,
        first_kernel: Some(KernelVersion::V6_16),
        hardware: Some(Hardware {
            gpu: &gpu::RTX_5090,
            mmu: &GH100,
            gr: GrClasses {
                i2m: BLACKWELL_INLINE_TO_MEMORY_A,
                twod: FERMI_TWOD_A,
                threed: BLACKWELL_B,
                compute: BLACKWELL_COMPUTE_B,
            },
            ce: BLACKWELL_DMA_COPY_B,
        }),
    },
];

const fn chipset(name: &'static str, chipset: u16, pci_id: u16) -> Chip {
    Chip {
        name,
        chipset,
        pci_id,
        first_kernel: None,
        hardware: None,
    }
}

static CHIPSETS: &[Chip] = &[
    chipset("GP100", 0x130, 0x15f0),
    chipset("GP102", 0x132, 0x1b00),
    chipset("GP104", 0x134, 0x1b80),
    chipset("GP106", 0x136, 0x1c03),
    chipset("GP107", 0x137, 0x1c82),
    chipset("GP108", 0x138, 0x1d01),
    chipset("GV100", 0x140, 0x1d81),
];

pub fn chip(name: &str) -> &'static Chip {
    CHIPS
        .iter()
        .chain(CHIPSETS)
        .find(|chip| chip.name == name)
        .unwrap_or_else(|| panic!("unknown nouveau chip {name:?}"))
}
