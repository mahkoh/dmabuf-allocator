pub const MAX_EXTENT: u32 = 16384;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub const BLOCK_LINEAR_GOB_HEIGHT: u32 = 8;
