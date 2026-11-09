mod modifier;
mod sizes;

use self::modifier::TileVersion;
use fake_kernel::Kernel;
use fake_kernel::KernelVersion;
use fake_kernel::amd::devices::DEVICES;
use fake_kernel::amd::devices::GfxLevel;
use fake_kernel::amd::kernel;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_LINEAR;
use std::collections::BTreeSet;
use std::ops::Range;
use vulkan_testgen::drm;
use vulkan_testgen::formats::Block;
use vulkan_testgen::sizes::Size;
use vulkan_testgen::vulkan;
use vulkan_testgen::vulkan::Icd;
use vulkan_testgen::vulkan::load_icd;

const SMALL_PTE_FRAGMENT_SIZE: u32 = 64 << 10;

pub struct Driver {
    icd: Icd,
    nodes: Range<usize>,
}

impl Driver {
    pub fn new() -> Self {
        let ids = DEVICES
            .iter()
            .map(|device| kernel::Device::new(device.name, KernelVersion::NEWEST).id())
            .collect();
        let nodes = drm::register(ids);
        let icd = unsafe { load_icd(env!("RADV_LIBRARY")) };
        Self { icd, nodes }
    }

    pub fn pre_gfx9(&self, node: u32) -> bool {
        DEVICES[node as usize - self.nodes.start].gfx_level <= GfxLevel::Gfx8
    }

    pub fn node_of(&self, name: &str) -> u32 {
        let index = DEVICES
            .iter()
            .position(|device| device.name == name)
            .unwrap_or_else(|| panic!("no AMD device is named {name}"));
        (self.nodes.start + index) as u32
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
        node as usize - self.nodes.start
    }

    fn rank(&self, device: Self::Device) -> u64 {
        device as u64
    }

    fn skip(&self, modifier: u64, device: Self::Device) -> bool {
        modifier == DRM_FORMAT_MOD_LINEAR && DEVICES[device].gfx_level != GfxLevel::Gfx10
    }

    fn size_set(&self, modifier: u64, planes: &[Block], device: Self::Device) -> BTreeSet<Size> {
        sizes::size_set(planes, DEVICES[device].gfx_level, modifier)
    }

    fn variants(&self, node: u32) -> Vec<String> {
        let device = &DEVICES[self.device(node)];
        let kernel = kernel::Device::new(device.name, KernelVersion::NEWEST)
            .with_pte_fragment_size(SMALL_PTE_FRAGMENT_SIZE);
        vec![kernel.id()]
    }

    fn exact_export_size(&self) -> bool {
        true
    }

    fn output_file(&self, modifier: u64) -> &'static str {
        if modifier == DRM_FORMAT_MOD_LINEAR {
            return "amd_linear";
        }
        match modifier::tile_version(modifier) {
            TileVersion::Gfx6 => "amd_gfx6",
            TileVersion::Gfx9 => "amd_gfx9",
            TileVersion::Gfx10 => "amd_gfx10",
            TileVersion::Gfx10Rbplus => "amd_gfx10_rbplus",
            TileVersion::Gfx11 => "amd_gfx11",
            TileVersion::Gfx12 => "amd_gfx12",
        }
    }
}
