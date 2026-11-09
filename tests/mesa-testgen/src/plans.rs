use vulkan_testgen::generate;
use vulkan_testgen::generate::PairInfo;
use vulkan_testgen::vulkan;
use vulkan_testgen::vulkan::Described;

#[cfg(test)]
mod tests;

const SCANOUTS: [bool; 2] = [false, true];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GbmPair {
    pub base: usize,
    pub scanout: bool,
}

impl GbmPair {
    pub fn index(self) -> usize {
        self.base * SCANOUTS.len() + self.scanout as usize
    }

    pub fn from_index(index: usize) -> Self {
        Self {
            base: index / SCANOUTS.len(),
            scanout: SCANOUTS[index % SCANOUTS.len()],
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GbmClassKey {
    vulkan: vulkan::ClassKey,
    scanout: bool,
}

pub struct GbmPlan<'a> {
    pub described: &'a Described,
}

impl generate::Backend for GbmPlan<'_> {
    type ClassKey = GbmClassKey;

    fn pairs(&self) -> Vec<PairInfo<'_>> {
        let pairs = self.described.pair_infos();
        pairs
            .iter()
            .flat_map(|pair| {
                SCANOUTS.map(|scanout| {
                    let gbm = GbmPair {
                        base: pair.index,
                        scanout,
                    };
                    PairInfo {
                        index: gbm.index(),
                        fourcc: pair.fourcc,
                        modifier: pair.modifier,
                        scanout: Some(scanout),
                        sizes: pair.sizes,
                    }
                })
            })
            .collect()
    }

    fn class_key(&self, pair: usize) -> Self::ClassKey {
        let pair = GbmPair::from_index(pair);
        GbmClassKey {
            vulkan: self.described.class_key(pair.base),
            scanout: pair.scanout,
        }
    }
}
