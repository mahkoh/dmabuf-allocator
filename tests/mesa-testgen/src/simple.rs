use crate::gbm::GbmDevice;
use allocator_trace::vectors;
use allocator_trace::vectors::Flag;
use fake_kernel::Kernel;
use fake_kernel::KernelVersion;
use fake_kernel::Node;
use fake_kernel::uapi::drm_fourcc::AFBC_FORMAT_MOD_BLOCK_SIZE_16x16;
use fake_kernel::uapi::drm_fourcc::AFBC_FORMAT_MOD_BLOCK_SIZE_32x8;
use fake_kernel::uapi::drm_fourcc::AFBC_FORMAT_MOD_SC;
use fake_kernel::uapi::drm_fourcc::AFBC_FORMAT_MOD_SPARSE;
use fake_kernel::uapi::drm_fourcc::AFBC_FORMAT_MOD_SPLIT;
use fake_kernel::uapi::drm_fourcc::AFBC_FORMAT_MOD_TILED;
use fake_kernel::uapi::drm_fourcc::AFBC_FORMAT_MOD_YTR;
use fake_kernel::uapi::drm_fourcc::AFRC_FORMAT_MOD_CU_SIZE_16;
use fake_kernel::uapi::drm_fourcc::AFRC_FORMAT_MOD_CU_SIZE_24;
use fake_kernel::uapi::drm_fourcc::AFRC_FORMAT_MOD_CU_SIZE_32;
use fake_kernel::uapi::drm_fourcc::AFRC_FORMAT_MOD_CU_SIZE_P0;
use fake_kernel::uapi::drm_fourcc::AFRC_FORMAT_MOD_LAYOUT_SCAN;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_APPLE_GPU_TILED;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_ARM_AFBC;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_ARM_AFRC;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_ARM_INTERLEAVED_64K;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_BROADCOM_SAND128;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_BROADCOM_UIF;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_LINEAR;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_VIVANTE_SUPER_TILED;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_VIVANTE_TILED;
use vulkan_testgen::cases;
use vulkan_testgen::sizes::MAX_EXTENT;
use vulkan_testgen::sizes::Size;

#[cfg(test)]
mod tests;

const LINEAR_ONLY: [u64; 1] = [DRM_FORMAT_MOD_LINEAR];
const ASAHI_MODIFIERS: [u64; 3] = [
    DRM_FORMAT_MOD_LINEAR,
    DRM_FORMAT_MOD_APPLE_GPU_TILED,
    DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED,
];
const PANFROST_MODIFIERS: [u64; 17] = [
    DRM_FORMAT_MOD_ARM_AFBC(AFBC_FORMAT_MOD_BLOCK_SIZE_32x8 | AFBC_FORMAT_MOD_SPARSE),
    DRM_FORMAT_MOD_ARM_AFBC(
        AFBC_FORMAT_MOD_BLOCK_SIZE_32x8 | AFBC_FORMAT_MOD_SPARSE | AFBC_FORMAT_MOD_SPLIT,
    ),
    DRM_FORMAT_MOD_ARM_AFBC(
        AFBC_FORMAT_MOD_BLOCK_SIZE_32x8
            | AFBC_FORMAT_MOD_SPARSE
            | AFBC_FORMAT_MOD_SPLIT
            | AFBC_FORMAT_MOD_YTR,
    ),
    DRM_FORMAT_MOD_ARM_AFBC(
        AFBC_FORMAT_MOD_BLOCK_SIZE_16x16
            | AFBC_FORMAT_MOD_SPARSE
            | AFBC_FORMAT_MOD_YTR
            | AFBC_FORMAT_MOD_TILED
            | AFBC_FORMAT_MOD_SC,
    ),
    DRM_FORMAT_MOD_ARM_AFBC(
        AFBC_FORMAT_MOD_BLOCK_SIZE_16x16
            | AFBC_FORMAT_MOD_SPARSE
            | AFBC_FORMAT_MOD_TILED
            | AFBC_FORMAT_MOD_SC,
    ),
    DRM_FORMAT_MOD_ARM_AFBC(
        AFBC_FORMAT_MOD_BLOCK_SIZE_16x16 | AFBC_FORMAT_MOD_SPARSE | AFBC_FORMAT_MOD_YTR,
    ),
    DRM_FORMAT_MOD_ARM_AFBC(AFBC_FORMAT_MOD_BLOCK_SIZE_16x16 | AFBC_FORMAT_MOD_SPARSE),
    DRM_FORMAT_MOD_ARM_AFBC(
        AFBC_FORMAT_MOD_BLOCK_SIZE_16x16 | AFBC_FORMAT_MOD_SPARSE | AFBC_FORMAT_MOD_SPLIT,
    ),
    DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED,
    DRM_FORMAT_MOD_ARM_INTERLEAVED_64K,
    DRM_FORMAT_MOD_LINEAR,
    DRM_FORMAT_MOD_ARM_AFRC(AFRC_FORMAT_MOD_CU_SIZE_P0(AFRC_FORMAT_MOD_CU_SIZE_16)),
    DRM_FORMAT_MOD_ARM_AFRC(AFRC_FORMAT_MOD_CU_SIZE_P0(AFRC_FORMAT_MOD_CU_SIZE_24)),
    DRM_FORMAT_MOD_ARM_AFRC(AFRC_FORMAT_MOD_CU_SIZE_P0(AFRC_FORMAT_MOD_CU_SIZE_32)),
    DRM_FORMAT_MOD_ARM_AFRC(
        AFRC_FORMAT_MOD_CU_SIZE_P0(AFRC_FORMAT_MOD_CU_SIZE_16) | AFRC_FORMAT_MOD_LAYOUT_SCAN,
    ),
    DRM_FORMAT_MOD_ARM_AFRC(
        AFRC_FORMAT_MOD_CU_SIZE_P0(AFRC_FORMAT_MOD_CU_SIZE_24) | AFRC_FORMAT_MOD_LAYOUT_SCAN,
    ),
    DRM_FORMAT_MOD_ARM_AFRC(
        AFRC_FORMAT_MOD_CU_SIZE_P0(AFRC_FORMAT_MOD_CU_SIZE_32) | AFRC_FORMAT_MOD_LAYOUT_SCAN,
    ),
];
const V3D_MODIFIERS: [u64; 3] = [
    DRM_FORMAT_MOD_BROADCOM_UIF,
    DRM_FORMAT_MOD_LINEAR,
    DRM_FORMAT_MOD_BROADCOM_SAND128,
];
const ETNAVIV_MODIFIERS: [u64; 5] = [
    DRM_FORMAT_MOD_LINEAR,
    DRM_FORMAT_MOD_VIVANTE_TILED,
    DRM_FORMAT_MOD_VIVANTE_SUPER_TILED,
    DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED,
    DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED,
];
const LIMA_MODIFIERS: [u64; 2] = [
    DRM_FORMAT_MOD_LINEAR,
    DRM_FORMAT_MOD_ARM_16X16_BLOCK_U_INTERLEAVED,
];
const VC4_MODIFIERS: [u64; 2] = [DRM_FORMAT_MOD_LINEAR, DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED];

const DUMB_WIDTHS: [u32; 10] = [1, 63, 64, 65, 127, 128, 1080, 1920, 4096, MAX_EXTENT];
const DUMB_HEIGHTS: [u32; 7] = [1, 63, 64, 65, 127, 128, 1080];
const VIRTIO_NATIVE_WIDTHS: [u32; 8] = [1, 16, 17, 64, 65, 256, 257, 1920];
const VIRTIO_NATIVE_HEIGHTS: [u32; 8] = [1, 16, 17, 64, 65, 256, 257, 1080];
const ASAHI_WIDTHS: [u32; 11] = [1, 15, 16, 17, 63, 64, 65, 127, 128, 129, 1920];
const ASAHI_HEIGHTS: [u32; 11] = [1, 15, 16, 17, 63, 64, 65, 127, 128, 129, 1080];
const PANFROST_WIDTHS: [u32; 11] = [1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 1920];
const PANFROST_HEIGHTS: [u32; 11] = [1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 1080];
const V3D_WIDTHS: [u32; 9] = [1, 7, 8, 9, 31, 32, 33, 65, 1920];
const V3D_HEIGHTS: [u32; 25] = [
    1, 8, 9, 16, 17, 47, 48, 49, 96, 97, 207, 208, 209, 255, 256, 257, 300, 415, 416, 417, 511,
    512, 513, 700, 1080,
];
const ETNAVIV_EXTENTS: [u32; 15] = [
    1, 3, 4, 5, 15, 16, 17, 63, 64, 65, 127, 128, 129, 1080, 8192,
];
const LIMA_EXTENTS: [u32; 10] = [1, 15, 16, 17, 31, 32, 33, 1080, 4096, 4097];
const VC4_EXTENTS: [u32; 13] = [1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 1080, 2048, 2049];

const VIRTIO_AMDGPU_CHIP: &str = "navi21";
const VIRTIO_I915_CHIPS: [&str; 4] = ["skl_gt2", "tgl_gt2", "dg2_g10", "mtl_u"];
const VIRTIO_ASAHI_CHIPS: [&str; 5] = ["t8103", "t6000", "t6001", "t6002", "t8112"];
const ASAHI_DRIVER: &str = "asahi";

const SCANOUT_ONLY: [bool; 1] = [true];
const BOTH: [bool; 2] = [false, true];

pub enum Modifiers {
    List(&'static [u64]),
    AmdHost { chip: &'static str },
    IntelHost { chips: &'static [&'static str] },
}

#[derive(Copy, Clone)]
pub enum SupportCheck {
    Skip,
    NonLinear,
    All,
}

pub struct SimpleDriver {
    pub name: &'static str,
    pub file: &'static str,
    pub node: Node,
    pub kernels: fn() -> Vec<String>,
    pub modifiers: Modifiers,
    pub support_check: SupportCheck,
    pub widths: &'static [u32],
    pub heights: &'static [u32],
    pub scanout: &'static [bool],
    pub scanout_linear_only: bool,
    pub flags: &'static [Flag],
    pub driver_override: Option<&'static str>,
}

fn virtio_gpu(host: &str) -> String {
    fake_kernel::virtio_gpu::Device::new(host, KernelVersion::NEWEST).id()
}

fn dumb_kernels() -> Vec<String> {
    vec![fake_kernel::vkms::Device::new(KernelVersion::NEWEST).id()]
}

fn virtio_gpu_2d_kernels() -> Vec<String> {
    vec![virtio_gpu("2d")]
}

fn virgl_kernels() -> Vec<String> {
    vec![virtio_gpu("virgl")]
}

fn virtio_amdgpu_kernels() -> Vec<String> {
    vec![virtio_gpu(&format!("amdgpu_{VIRTIO_AMDGPU_CHIP}"))]
}

fn virtio_i915_kernels() -> Vec<String> {
    let hosts = VIRTIO_I915_CHIPS.iter();
    hosts
        .map(|chip| virtio_gpu(&format!("i915_{chip}")))
        .collect()
}

fn virtio_asahi_kernels() -> Vec<String> {
    let hosts = VIRTIO_ASAHI_CHIPS.iter();
    hosts
        .map(|chip| virtio_gpu(&format!("asahi_{chip}")))
        .collect()
}

fn virtio_panfrost_kernels() -> Vec<String> {
    let hosts = fake_kernel::panfrost::GPUS.iter();
    hosts
        .map(|gpu| virtio_gpu(&format!("panfrost_{}", gpu.name)))
        .collect()
}

macro_rules! native_kernels {
    ($name:ident, $module:ident, $table:ident) => {
        fn $name() -> Vec<String> {
            let devices = fake_kernel::$module::$table.iter();
            devices
                .map(|device| {
                    fake_kernel::$module::Device::new(device.name, KernelVersion::NEWEST).id()
                })
                .collect()
        }
    };
}

native_kernels!(asahi_kernels, asahi, CHIPS);
native_kernels!(panfrost_kernels, panfrost, GPUS);
native_kernels!(panthor_kernels, panthor, GPUS);
native_kernels!(v3d_kernels, v3d, GPUS);
native_kernels!(etnaviv_kernels, etnaviv, GPUS);
native_kernels!(lima_kernels, lima, GPUS);
native_kernels!(vc4_kernels, vc4, GPUS);

const fn render(
    name: &'static str,
    kernels: fn() -> Vec<String>,
    modifiers: &'static [u64],
    support_check: SupportCheck,
    widths: &'static [u32],
    heights: &'static [u32],
) -> SimpleDriver {
    SimpleDriver {
        name,
        file: name,
        node: Node::Render,
        kernels,
        modifiers: Modifiers::List(modifiers),
        support_check,
        widths,
        heights,
        scanout: &BOTH,
        scanout_linear_only: false,
        flags: &[],
        driver_override: None,
    }
}

pub static DRIVERS: [SimpleDriver; 14] = [
    SimpleDriver {
        node: Node::Primary,
        scanout: &SCANOUT_ONLY,
        ..render(
            "dumb",
            dumb_kernels,
            &LINEAR_ONLY,
            SupportCheck::Skip,
            &DUMB_WIDTHS,
            &DUMB_HEIGHTS,
        )
    },
    SimpleDriver {
        node: Node::Primary,
        scanout: &SCANOUT_ONLY,
        ..render(
            "virtio_gpu_2d",
            virtio_gpu_2d_kernels,
            &LINEAR_ONLY,
            SupportCheck::Skip,
            &DUMB_WIDTHS,
            &DUMB_HEIGHTS,
        )
    },
    render(
        "virtio_gpu_virgl",
        virgl_kernels,
        &LINEAR_ONLY,
        SupportCheck::Skip,
        &DUMB_WIDTHS,
        &DUMB_HEIGHTS,
    ),
    SimpleDriver {
        modifiers: Modifiers::AmdHost {
            chip: VIRTIO_AMDGPU_CHIP,
        },
        ..render(
            "virtio_gpu_amdgpu",
            virtio_amdgpu_kernels,
            &[],
            SupportCheck::NonLinear,
            &VIRTIO_NATIVE_WIDTHS,
            &VIRTIO_NATIVE_HEIGHTS,
        )
    },
    SimpleDriver {
        modifiers: Modifiers::IntelHost {
            chips: &VIRTIO_I915_CHIPS,
        },
        flags: &[Flag::IgnoreSetDomain],
        ..render(
            "virtio_gpu_i915",
            virtio_i915_kernels,
            &[],
            SupportCheck::All,
            &VIRTIO_NATIVE_WIDTHS,
            &VIRTIO_NATIVE_HEIGHTS,
        )
    },
    SimpleDriver {
        driver_override: Some(ASAHI_DRIVER),
        ..render(
            "virtio_gpu_asahi",
            virtio_asahi_kernels,
            &ASAHI_MODIFIERS,
            SupportCheck::Skip,
            &ASAHI_WIDTHS,
            &ASAHI_HEIGHTS,
        )
    },
    render(
        "virtio_gpu_panfrost",
        virtio_panfrost_kernels,
        &PANFROST_MODIFIERS,
        SupportCheck::NonLinear,
        &PANFROST_WIDTHS,
        &PANFROST_HEIGHTS,
    ),
    render(
        "asahi",
        asahi_kernels,
        &ASAHI_MODIFIERS,
        SupportCheck::Skip,
        &ASAHI_WIDTHS,
        &ASAHI_HEIGHTS,
    ),
    render(
        "panfrost",
        panfrost_kernels,
        &PANFROST_MODIFIERS,
        SupportCheck::NonLinear,
        &PANFROST_WIDTHS,
        &PANFROST_HEIGHTS,
    ),
    render(
        "panthor",
        panthor_kernels,
        &PANFROST_MODIFIERS,
        SupportCheck::NonLinear,
        &PANFROST_WIDTHS,
        &PANFROST_HEIGHTS,
    ),
    render(
        "v3d",
        v3d_kernels,
        &V3D_MODIFIERS,
        SupportCheck::NonLinear,
        &V3D_WIDTHS,
        &V3D_HEIGHTS,
    ),
    render(
        "etnaviv",
        etnaviv_kernels,
        &ETNAVIV_MODIFIERS,
        SupportCheck::NonLinear,
        &ETNAVIV_EXTENTS,
        &ETNAVIV_EXTENTS,
    ),
    SimpleDriver {
        scanout_linear_only: true,
        ..render(
            "lima",
            lima_kernels,
            &LIMA_MODIFIERS,
            SupportCheck::NonLinear,
            &LIMA_EXTENTS,
            &LIMA_EXTENTS,
        )
    },
    render(
        "vc4",
        vc4_kernels,
        &VC4_MODIFIERS,
        SupportCheck::NonLinear,
        &VC4_EXTENTS,
        &VC4_EXTENTS,
    ),
];

impl SimpleDriver {
    pub fn record(
        &self,
        device: &GbmDevice,
        fourcc: u32,
        modifiers: &[u64],
        mut emit: impl FnMut(vectors::Case),
    ) {
        let flags = cases::flags(self.flags);
        for &modifier in modifiers {
            let check = match self.support_check {
                SupportCheck::Skip => false,
                SupportCheck::NonLinear => modifier != DRM_FORMAT_MOD_LINEAR,
                SupportCheck::All => true,
            };
            if check && !device.modifier_supported(fourcc, modifier) {
                continue;
            }
            for &width in self.widths {
                for &height in self.heights {
                    for &scanout in self.scanout {
                        if self.scanout_linear_only && scanout && modifier != DRM_FORMAT_MOD_LINEAR
                        {
                            continue;
                        }
                        let size = Size { width, height };
                        if let Some(case) = device.record(fourcc, modifier, size, scanout, flags) {
                            emit(case);
                        }
                    }
                }
            }
        }
    }
}
