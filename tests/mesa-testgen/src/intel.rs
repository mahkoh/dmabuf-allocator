use crate::sizes::tiled_sizes;
use fake_kernel::Kernel;
use fake_kernel::KernelVersion;
use fake_kernel::intel::devices::DEVICES;
use fake_kernel::intel::kernel;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_LINEAR;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_BMG_CCS;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_DG2_MC_CCS;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_DG2_RC_CCS;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_LNL_CCS;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_MTL_MC_CCS;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_MTL_RC_CCS;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_X_TILED;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_Y_TILED;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_Y_TILED_CCS;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS;
use fake_kernel::uapi::drm_fourcc::I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC;
use std::collections::BTreeSet;
use std::ops::Range;
use vulkan_testgen::drm;
use vulkan_testgen::formats::Block;
use vulkan_testgen::sizes::Size;
use vulkan_testgen::vulkan;
use vulkan_testgen::vulkan::Icd;
use vulkan_testgen::vulkan::load_icd;

#[cfg(test)]
mod tests;

const XE_MIN_VERX10: u32 = 120;
const RANK_VERSION_SHIFT: u32 = 32;

struct Tile {
    width_bytes: u32,
    height: Option<u32>,
}

const LINEAR_TILE: Tile = Tile {
    width_bytes: 256,
    height: None,
};
const X_TILE: Tile = Tile {
    width_bytes: 512,
    height: Some(8),
};
const Y_TILE: Tile = Tile {
    width_bytes: 128,
    height: Some(32),
};

fn tile(modifier: u64) -> Tile {
    match modifier {
        DRM_FORMAT_MOD_LINEAR => LINEAR_TILE,
        I915_FORMAT_MOD_X_TILED => X_TILE,
        I915_FORMAT_MOD_Y_TILED
        | I915_FORMAT_MOD_Y_TILED_CCS
        | I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS
        | I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS
        | I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC
        | I915_FORMAT_MOD_4_TILED
        | I915_FORMAT_MOD_4_TILED_DG2_RC_CCS
        | I915_FORMAT_MOD_4_TILED_DG2_MC_CCS
        | I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC
        | I915_FORMAT_MOD_4_TILED_MTL_RC_CCS
        | I915_FORMAT_MOD_4_TILED_MTL_MC_CCS
        | I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC
        | I915_FORMAT_MOD_4_TILED_LNL_CCS
        | I915_FORMAT_MOD_4_TILED_BMG_CCS => Y_TILE,
        other => panic!("unknown Intel modifier {other:#x}"),
    }
}

pub struct Driver {
    icd: Icd,
    nodes: Range<usize>,
    devices: Vec<usize>,
}

impl Driver {
    pub fn new() -> Self {
        let devices: Vec<usize> = (0..DEVICES.len())
            .filter(|&device| kernel::Device::new(device, KernelVersion::NEWEST).supported())
            .collect();
        let ids = devices
            .iter()
            .map(|&device| kernel::Device::new(device, KernelVersion::NEWEST).id())
            .collect();
        let nodes = drm::register(ids);
        let icd = unsafe { load_icd(env!("ANV_LIBRARY")) };
        Self {
            icd,
            nodes,
            devices,
        }
    }

    pub fn i915_node_of(&self, name: &str) -> u32 {
        let position = self
            .devices
            .iter()
            .position(|&device| DEVICES[device].name == name)
            .unwrap_or_else(|| panic!("no supported Intel device is named {name}"));
        let device = &DEVICES[self.devices[position]];
        assert!(
            device.verx10 < kernel::XE_VERX10,
            "the Intel device {name} uses xe"
        );
        (self.nodes.start + position) as u32
    }
}

impl vulkan::Driver for Driver {
    type Device = usize;

    fn icd(&self) -> &Icd {
        &self.icd
    }

    fn nodes(&self) -> Range<usize> {
        self.nodes.clone()
    }

    fn device(&self, node: u32) -> Self::Device {
        self.devices[node as usize - self.nodes.start]
    }

    fn rank(&self, device: Self::Device) -> u64 {
        (DEVICES[device].verx10 as u64) << RANK_VERSION_SHIFT | device as u64
    }

    fn skip(&self, modifier: u64, _device: Self::Device) -> bool {
        modifier == DRM_FORMAT_MOD_LINEAR
    }

    fn size_set(&self, modifier: u64, planes: &[Block], _device: Self::Device) -> BTreeSet<Size> {
        let tile = tile(modifier);
        tiled_sizes(&planes[0], tile.width_bytes, tile.height)
    }

    fn variants(&self, node: u32) -> Vec<String> {
        let index = self.device(node);
        let device = &DEVICES[index];
        let default_xe = device.verx10 >= kernel::XE_VERX10;
        let variant = |xe: bool, small_bar: bool| {
            kernel::Device::variant(index, KernelVersion::NEWEST, xe, small_bar)
        };
        let mut kernels = Vec::new();
        if device.has_local_mem {
            kernels.push(variant(default_xe, true));
        }
        if !default_xe && device.verx10 >= XE_MIN_VERX10 {
            kernels.push(variant(true, false));
            if device.has_local_mem {
                kernels.push(variant(true, true));
            }
        }
        kernels
            .into_iter()
            .filter(|kernel| kernel.supported())
            .map(|kernel| kernel.id())
            .collect()
    }

    fn output_file(&self, _modifier: u64) -> &'static str {
        "intel"
    }
}
