use serde::Deserialize;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    format: Vec<Format>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Format {
    pub fourcc: String,
    pub channels: Option<Vec<String>>,
    pub vulkan: Option<String>,
    #[serde(default)]
    pub amd: bool,
    #[serde(default)]
    pub amd_displayable: bool,
    #[serde(default)]
    pub amd_implicit: bool,
    pub amd_gfx12: Option<AmdGfx12>,
    pub amd_gfx6: Option<AmdGfx6>,
    #[serde(default)]
    pub nvidia: bool,
    #[serde(default)]
    pub nouveau: bool,
    #[serde(default)]
    pub dumb: bool,
    #[serde(default)]
    pub asahi: bool,
    #[serde(default)]
    pub v3d: bool,
    #[serde(default)]
    pub lima: bool,
    #[serde(default)]
    pub vc4: bool,
    pub virgl: Option<u32>,
    pub panfrost: Option<Panfrost>,
    pub etnaviv: Option<Etnaviv>,
    pub planes: Vec<Plane>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plane {
    pub block_width: u32,
    pub block_height: u32,
    pub bits: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmdGfx6 {
    pub data_format: u8,
    pub num_format: u8,
    pub dst_sel: [u8; 4],
    pub alpha_is_on_msb: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmdGfx12 {
    pub data_format: u8,
    pub number_type: u8,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Panfrost {
    pub afbc: Option<String>,
    pub afbc_min_arch: Option<u32>,
    #[serde(default)]
    pub afbc_no_v7: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Etnaviv {
    pub render: Option<Vec<String>>,
    pub sample: Option<Vec<String>>,
    #[serde(default)]
    pub emulated_128bit: bool,
}

pub fn path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("formats.toml")
}

pub fn load() -> Vec<Format> {
    let path = path();
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    let file: File = toml::from_str(&source)
        .unwrap_or_else(|error| panic!("cannot parse {}: {error}", path.display()));
    let mut fourccs = HashSet::new();
    for format in &file.format {
        let fourcc = &format.fourcc;
        assert_eq!(fourcc.len(), 4, "fourcc {fourcc:?} must be 4 bytes");
        assert!(fourccs.insert(fourcc), "fourcc {fourcc:?} is defined twice");
        if let Some(panfrost) = &format.panfrost {
            let options = panfrost.afbc_min_arch.is_some() || panfrost.afbc_no_v7;
            match panfrost.afbc {
                Some(_) => assert!(
                    panfrost.afbc_min_arch.is_some(),
                    "afbc of {fourcc:?} needs afbc_min_arch"
                ),
                None => assert!(!options, "{fourcc:?} has afbc options without afbc"),
            }
        }
    }
    file.format
}
