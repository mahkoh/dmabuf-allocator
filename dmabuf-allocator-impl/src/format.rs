use crate::drivers::amd::AmdColor;
use crate::drivers::amd::AmdGfx6Image;
use crate::drivers::etnaviv::EtnavivFormat;
use crate::drivers::panfrost::PanfrostFormat;
use crate::formats::FORMATS;

pub const fn get_format(fourcc: u32) -> Option<Format> {
    let mut i = 0;
    while i < FORMATS.len() {
        if FORMATS[i].fourcc == fourcc {
            return Some(Format(&FORMATS[i]));
        }
        i += 1;
    }
    None
}

#[derive(Copy, Clone)]
pub struct Format(pub &'static Fmt);

pub struct Fmt {
    pub fourcc: u32,
    pub(crate) amd: bool,
    pub(crate) amd_displayable: bool,
    pub(crate) amd_implicit: bool,
    pub(crate) amd_gfx12: Option<AmdColor>,
    pub(crate) amd_gfx6: Option<AmdGfx6Image>,
    pub(crate) nvidia: bool,
    pub(crate) nouveau: bool,
    pub(crate) dumb: bool,
    pub(crate) asahi: bool,
    pub(crate) v3d: bool,
    pub(crate) lima: bool,
    pub(crate) vc4: bool,
    pub(crate) virgl: Option<u32>,
    pub(crate) panfrost: Option<PanfrostFormat>,
    pub(crate) etnaviv: Option<EtnavivFormat>,
    pub planes: &'static [Plane],
}

pub struct Plane {
    pub block_width: u32,
    pub block_height: u32,
    pub bits: u32,
}

pub(crate) const fn fourcc(n: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*n)
}
