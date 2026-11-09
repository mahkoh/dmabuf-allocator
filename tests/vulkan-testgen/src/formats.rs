use crate::sizes::Size;
use ash::vk;

#[cfg(test)]
mod tests;

pub struct Block {
    pub bits: u32,
    pub width: u32,
    pub height: u32,
}

impl Block {
    pub fn bytes(&self) -> u32 {
        assert!(
            self.bits.is_multiple_of(8),
            "a block of {} bits is not a whole number of bytes",
            self.bits
        );
        self.bits / 8
    }
}

pub struct VulkanFormat {
    pub fourcc: &'static str,
    pub format: vk::Format,
    pub planes: &'static [Block],
}

pub struct AnyFormat {
    pub fourcc: &'static str,
    pub planes: &'static [Block],
}

include!(concat!(env!("OUT_DIR"), "/formats.rs"));

pub fn fourcc_code(fourcc: &str) -> u32 {
    let bytes: [u8; 4] = fourcc
        .as_bytes()
        .try_into()
        .unwrap_or_else(|_| panic!("fourcc {fourcc:?} must be 4 bytes"));
    u32::from_le_bytes(bytes)
}

pub fn largest_block(planes: &[Block]) -> Size {
    let largest = |dimension: fn(&Block) -> u32| {
        planes
            .iter()
            .map(dimension)
            .max()
            .expect("a format has at least one plane")
    };
    Size {
        width: largest(|block| block.width),
        height: largest(|block| block.height),
    }
}
