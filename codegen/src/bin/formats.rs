use codegen::formats;
use codegen::rustfmt;
use std::fmt::Write;
use std::path::Path;

fn flags(out: &mut String, flags: &[(&str, bool)]) {
    for (name, value) in flags {
        writeln!(out, "{name}: {value},").unwrap();
    }
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut out = String::from(
        "use crate::drivers::amd::AmdColor;\n\
         use crate::drivers::amd::AmdGfx6Image;\n\
         use crate::drivers::etnaviv::EtnavivFormat;\n\
         use crate::drivers::etnaviv::Feature;\n\
         use crate::drivers::panfrost::AfbcMode;\n\
         use crate::drivers::panfrost::PanfrostAfbc;\n\
         use crate::drivers::panfrost::PanfrostFormat;\n\
         use crate::format::Fmt;\n\
         use crate::format::Plane;\n\
         use crate::format::fourcc;\n\n\
         pub static FORMATS: &[Fmt] = &[\n",
    );
    for format in formats::load() {
        let name = &format.fourcc;
        writeln!(out, "Fmt {{").unwrap();
        writeln!(out, "fourcc: fourcc(b{name:?}),").unwrap();
        flags(
            &mut out,
            &[
                ("amd", format.amd),
                ("amd_displayable", format.amd_displayable),
                ("amd_implicit", format.amd_implicit),
            ],
        );
        match &format.amd_gfx12 {
            Some(amd) => {
                writeln!(out, "amd_gfx12: Some(AmdColor {{").unwrap();
                writeln!(out, "data_format: {},", amd.data_format).unwrap();
                writeln!(out, "number_type: {},", amd.number_type).unwrap();
                writeln!(out, "}}),").unwrap();
            }
            None => writeln!(out, "amd_gfx12: None,").unwrap(),
        }
        match &format.amd_gfx6 {
            Some(image) => {
                writeln!(out, "amd_gfx6: Some(AmdGfx6Image {{").unwrap();
                writeln!(out, "data_format: {},", image.data_format).unwrap();
                writeln!(out, "num_format: {},", image.num_format).unwrap();
                writeln!(out, "dst_sel: {:?},", image.dst_sel).unwrap();
                writeln!(out, "alpha_is_on_msb: {},", image.alpha_is_on_msb).unwrap();
                writeln!(out, "}}),").unwrap();
            }
            None => writeln!(out, "amd_gfx6: None,").unwrap(),
        }
        flags(
            &mut out,
            &[
                ("nvidia", format.nvidia),
                ("nouveau", format.nouveau),
                ("dumb", format.dumb),
                ("asahi", format.asahi),
                ("v3d", format.v3d),
                ("lima", format.lima),
                ("vc4", format.vc4),
            ],
        );
        match format.virgl {
            Some(virgl) => writeln!(out, "virgl: Some({virgl}),").unwrap(),
            None => writeln!(out, "virgl: None,").unwrap(),
        }
        match &format.panfrost {
            Some(panfrost) => match (&panfrost.afbc, panfrost.afbc_min_arch) {
                (Some(mode), Some(min_arch)) => {
                    let no_v7 = panfrost.afbc_no_v7;
                    writeln!(
                        out,
                        "panfrost: Some(PanfrostFormat {{ afbc: Some(PanfrostAfbc {{ \
                         mode: AfbcMode::{mode}, min_arch: {min_arch}, no_v7: {no_v7} }}) }}),"
                    )
                    .unwrap();
                }
                _ => writeln!(out, "panfrost: Some(PanfrostFormat {{ afbc: None }}),").unwrap(),
            },
            None => writeln!(out, "panfrost: None,").unwrap(),
        }
        match &format.etnaviv {
            Some(etnaviv) => {
                let gates = |features: &Option<Vec<String>>| match features {
                    Some(features) => {
                        let features: Vec<String> = features
                            .iter()
                            .map(|feature| format!("Feature::{feature}"))
                            .collect();
                        format!("Some(&[{}])", features.join(", "))
                    }
                    None => "None".to_string(),
                };
                let emulated_128bit = etnaviv.emulated_128bit;
                writeln!(
                    out,
                    "etnaviv: Some(EtnavivFormat {{ render: {}, sample: {}, \
                     emulated_128bit: {emulated_128bit} }}),",
                    gates(&etnaviv.render),
                    gates(&etnaviv.sample),
                )
                .unwrap();
            }
            None => writeln!(out, "etnaviv: None,").unwrap(),
        }
        writeln!(out, "planes: &[").unwrap();
        for plane in &format.planes {
            writeln!(out, "Plane {{").unwrap();
            writeln!(out, "block_width: {},", plane.block_width).unwrap();
            writeln!(out, "block_height: {},", plane.block_height).unwrap();
            writeln!(out, "bits: {},", plane.bits).unwrap();
            writeln!(out, "}},").unwrap();
        }
        writeln!(out, "],").unwrap();
        writeln!(out, "}},").unwrap();
    }
    out.push_str("];\n");
    let formatted = rustfmt(root, &out);
    std::fs::write(root.join("dmabuf-allocator-impl/src/formats.rs"), formatted).unwrap();
}
