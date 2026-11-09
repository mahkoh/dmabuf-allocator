use fake_kernel::Node;

#[cfg(test)]
mod tests;

const POOL_SIZE: u32 = 64;
const RENDER_POOL: u32 = 128;
const EXTENDED_POOL: u32 = 192;
const MINORS: u32 = 1 << 20;

pub const MAX_DEVICES: usize = (POOL_SIZE + (MINORS - EXTENDED_POOL) / 2) as usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DrmNode {
    pub device: usize,
    pub kind: Node,
}

impl DrmNode {
    pub fn minor(&self) -> u32 {
        assert!(
            self.device < MAX_DEVICES,
            "device {} has no minor",
            self.device
        );
        let device = self.device as u32;
        match (device.checked_sub(POOL_SIZE), self.kind) {
            (None, Node::Primary) => device,
            (None, Node::Render) => RENDER_POOL + device,
            (Some(extended), Node::Render) => EXTENDED_POOL + 2 * extended,
            (Some(extended), Node::Primary) => EXTENDED_POOL + 2 * extended + 1,
        }
    }

    pub fn from_minor(minor: u32) -> Option<DrmNode> {
        let node = |device: u32, kind: Node| DrmNode {
            device: device as usize,
            kind,
        };
        if minor < POOL_SIZE {
            return Some(node(minor, Node::Primary));
        }
        if (RENDER_POOL..RENDER_POOL + POOL_SIZE).contains(&minor) {
            return Some(node(minor - RENDER_POOL, Node::Render));
        }
        let extended = minor
            .checked_sub(EXTENDED_POOL)
            .filter(|_| minor < MINORS)?;
        let kind = match extended % 2 {
            0 => Node::Render,
            _ => Node::Primary,
        };
        Some(node(POOL_SIZE + extended / 2, kind))
    }

    pub fn name(&self) -> String {
        match self.kind {
            Node::Primary => format!("card{}", self.minor()),
            Node::Render => format!("renderD{}", self.minor()),
        }
    }
}
