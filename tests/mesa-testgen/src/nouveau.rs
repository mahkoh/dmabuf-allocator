use crate::sizes::tiled_sizes;
use fake_kernel::Kernel;
use fake_kernel::KernelVersion;
use fake_kernel::nouveau::chips::CHIPS;
use fake_kernel::nouveau::kernel;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_LINEAR;
use std::collections::BTreeSet;
use std::ops::Range;
use vulkan_testgen::drm;
use vulkan_testgen::formats::Block;
use vulkan_testgen::sizes::BLOCK_LINEAR_GOB_HEIGHT;
use vulkan_testgen::sizes::Size;
use vulkan_testgen::vulkan;
use vulkan_testgen::vulkan::Icd;
use vulkan_testgen::vulkan::load_icd;

const GOB_WIDTH_BYTES: u32 = 64;
const BLOCK_HEIGHT_LOG2_MASK: u64 = 0xf;

pub struct Driver {
    icd: Icd,
    nodes: Range<usize>,
}

impl Driver {
    pub fn new() -> Self {
        let ids = CHIPS
            .iter()
            .map(|chip| kernel::Device::new(chip.name, KernelVersion::NEWEST).id())
            .collect();
        let nodes = drm::register(ids);
        let icd = unsafe { load_icd(env!("NVK_LIBRARY")) };
        Self { icd, nodes }
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

    fn skip(&self, modifier: u64, _device: Self::Device) -> bool {
        modifier == DRM_FORMAT_MOD_LINEAR
    }

    fn size_set(&self, modifier: u64, planes: &[Block], _device: Self::Device) -> BTreeSet<Size> {
        let rows = BLOCK_LINEAR_GOB_HEIGHT << (modifier & BLOCK_HEIGHT_LOG2_MASK);
        tiled_sizes(&planes[0], GOB_WIDTH_BYTES, Some(rows))
    }

    fn output_file(&self, _modifier: u64) -> &'static str {
        "nouveau"
    }
}
