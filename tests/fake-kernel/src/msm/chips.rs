use crate::Platform;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Family {
    A2xxGen1,
    A2xxGen2,
    A3xx,
    A4xx,
    A5xx,
    A6xxGen1,
    A6xxGen2,
    A6xxGen3,
    A6xxGen4,
    A7xxGen1,
    A7xxGen2,
    A7xxGen3,
    A8xxGen1,
    A8xxGen2,
}

pub const UBWC_SWIZZLE_ENABLE_LVL2: u32 = 1 << 1;
pub const UBWC_SWIZZLE_ENABLE_LVL3: u32 = 1 << 2;

pub struct Ubwc {
    pub ubwc_swizzle: u32,
    pub highest_bank_bit: u32,
    pub macrotile_mode: bool,
}

pub struct PerfcntrGroup {
    pub name: &'static str,
    pub num_counters: u32,
}

pub static A7XX_PERFCNTR_GROUPS: &[PerfcntrGroup] = &[
    PerfcntrGroup {
        name: "CP",
        num_counters: 13,
    },
    PerfcntrGroup {
        name: "RBBM",
        num_counters: 4,
    },
    PerfcntrGroup {
        name: "PC",
        num_counters: 8,
    },
    PerfcntrGroup {
        name: "VFD",
        num_counters: 8,
    },
    PerfcntrGroup {
        name: "HLSQ",
        num_counters: 6,
    },
    PerfcntrGroup {
        name: "VPC",
        num_counters: 6,
    },
    PerfcntrGroup {
        name: "TSE",
        num_counters: 4,
    },
    PerfcntrGroup {
        name: "RAS",
        num_counters: 4,
    },
    PerfcntrGroup {
        name: "UCHE",
        num_counters: 12,
    },
    PerfcntrGroup {
        name: "TP",
        num_counters: 12,
    },
    PerfcntrGroup {
        name: "SP",
        num_counters: 24,
    },
    PerfcntrGroup {
        name: "RB",
        num_counters: 8,
    },
    PerfcntrGroup {
        name: "VSC",
        num_counters: 2,
    },
    PerfcntrGroup {
        name: "CCU",
        num_counters: 5,
    },
    PerfcntrGroup {
        name: "LRZ",
        num_counters: 4,
    },
    PerfcntrGroup {
        name: "CMP",
        num_counters: 4,
    },
    PerfcntrGroup {
        name: "UFC",
        num_counters: 4,
    },
    PerfcntrGroup {
        name: "BV_CP",
        num_counters: 7,
    },
    PerfcntrGroup {
        name: "BV_PC",
        num_counters: 8,
    },
    PerfcntrGroup {
        name: "BV_VFD",
        num_counters: 8,
    },
    PerfcntrGroup {
        name: "BV_VPC",
        num_counters: 6,
    },
    PerfcntrGroup {
        name: "BV_TP",
        num_counters: 6,
    },
    PerfcntrGroup {
        name: "BV_SP",
        num_counters: 12,
    },
    PerfcntrGroup {
        name: "BV_UFC",
        num_counters: 2,
    },
    PerfcntrGroup {
        name: "BV_TSE",
        num_counters: 4,
    },
    PerfcntrGroup {
        name: "BV_RAS",
        num_counters: 4,
    },
    PerfcntrGroup {
        name: "BV_LRZ",
        num_counters: 4,
    },
    PerfcntrGroup {
        name: "BV_HLSQ",
        num_counters: 6,
    },
];

pub struct Chip {
    pub name: &'static str,
    pub chip_id: u32,
    pub revn: u32,
    pub family: Family,
    pub gmem: u64,
    pub max_freq: u64,
    pub preemption: bool,
    pub smmu_input_bits: u32,
    pub ray_tracing: bool,
    pub smmu_compatible: &'static [&'static str],
    pub perfcntr_groups: &'static [PerfcntrGroup],
    pub ubwc: Option<Ubwc>,
    pub platform: Platform,
}

pub static CHIPS: &[Chip] = &[Chip {
    name: "a750",
    chip_id: 0x43051401,
    revn: 0,
    family: Family::A7xxGen3,
    gmem: 3 << 20,
    max_freq: 834000000,
    preemption: true,
    smmu_input_bits: 48,
    ray_tracing: true,
    smmu_compatible: &[
        "qcom,sm8650-smmu-500",
        "qcom,adreno-smmu",
        "qcom,smmu-500",
        "arm,mmu-500",
    ],
    perfcntr_groups: A7XX_PERFCNTR_GROUPS,
    ubwc: Some(Ubwc {
        ubwc_swizzle: UBWC_SWIZZLE_ENABLE_LVL2 | UBWC_SWIZZLE_ENABLE_LVL3,
        highest_bank_bit: 16,
        macrotile_mode: true,
    }),
    platform: Platform {
        fullname: "/soc@0/gpu@3d00000",
        compatible: &["qcom,adreno-43051401", "qcom,adreno"],
    },
}];

pub fn chip(name: &str) -> &'static Chip {
    CHIPS
        .iter()
        .find(|chip| chip.name == name)
        .unwrap_or_else(|| panic!("unknown msm chip {name:?}"))
}
