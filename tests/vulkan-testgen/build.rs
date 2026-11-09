use codegen::formats;
use codegen::formats::Format;
use std::env;
use std::fmt::Write;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={}", formats::path().display());
    std::fs::write(out.join("formats.rs"), format_table()).unwrap();
}

fn planes(format: &Format) -> String {
    let mut planes = String::new();
    for plane in &format.planes {
        write!(
            planes,
            "Block {{ bits: {}, width: {}, height: {} }}, ",
            plane.bits, plane.block_width, plane.block_height
        )
        .unwrap();
    }
    planes.trim_end_matches(", ").to_string()
}

fn format_table() -> String {
    let formats = formats::load();
    let mut out = String::from("pub const FORMATS: &[VulkanFormat] = &[\n");
    for format in &formats {
        let Some(vulkan) = &format.vulkan else {
            continue;
        };
        let fourcc = &format.fourcc;
        let name = vulkan.strip_prefix("VK_FORMAT_").unwrap();
        writeln!(
            out,
            "    VulkanFormat {{ fourcc: {fourcc:?}, format: ash::vk::Format::{name}, planes: &[{}] }},",
            planes(format)
        )
        .unwrap();
    }
    out.push_str("];\n\npub const ALL_FORMATS: &[AnyFormat] = &[\n");
    for format in &formats {
        let fourcc = &format.fourcc;
        writeln!(
            out,
            "    AnyFormat {{ fourcc: {fourcc:?}, planes: &[{}] }},",
            planes(format)
        )
        .unwrap();
    }
    out.push_str("];\n");
    out
}
