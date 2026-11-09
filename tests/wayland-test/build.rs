use codegen::formats::Plane;
use std::fmt::Write;
use std::path::Path;
use wl_client_builder::Builder;

fn main() {
    Builder::default().with_mutable_data(true).build().unwrap();
    formats();
}

fn channel(channel: char) -> &'static str {
    match channel {
        'R' => "Channel::R",
        'G' => "Channel::G",
        'B' => "Channel::B",
        'Y' => "Channel::Y",
        'U' => "Channel::U",
        'V' => "Channel::V",
        'A' | 'X' => "Channel::One",
        _ => panic!("unknown channel {channel:?}"),
    }
}

fn is_packed_subsampled(planes: &[Plane]) -> bool {
    matches!(planes, [plane] if plane.block_width != 1 || plane.block_height != 1)
}

fn plane_view(bits: u32, channels: usize) -> &'static str {
    match (bits, channels) {
        (8, 1) => "R8_UNORM",
        (16, 1) => "R16_UNORM",
        (16, 2) => "R8G8_UNORM",
        (32, 2) => "R16G16_UNORM",
        _ => panic!("no view format for {channels} channels in {bits} bits"),
    }
}

fn formats() {
    let path = codegen::formats::path();
    println!("cargo::rerun-if-changed={}", path.display());
    let mut out = String::from("pub static FORMATS: &[Format] = &[\n");
    for format in codegen::formats::load() {
        let Some(vulkan) = &format.vulkan else {
            continue;
        };
        let vulkan = vulkan.strip_prefix("VK_FORMAT_").unwrap();
        let planes = &format.planes;
        if is_packed_subsampled(planes) {
            continue;
        }
        let fourcc = &format.fourcc;
        let channels: Vec<&str> = match &format.channels {
            Some(channels) => channels.iter().map(String::as_str).collect(),
            None => vec!["RGBA"],
        };
        assert_eq!(channels.len(), planes.len(), "channels of {fourcc:?}");
        writeln!(out, "    Format {{").unwrap();
        writeln!(out, "        fourcc: *b{fourcc:?},").unwrap();
        writeln!(
            out,
            "        format: dmabuf_allocator::get_format(b{fourcc:?}).unwrap(),"
        )
        .unwrap();
        writeln!(out, "        name: {:?},", fourcc.trim_end()).unwrap();
        writeln!(out, "        vulkan: vk::Format::{vulkan},").unwrap();
        writeln!(out, "        planes: &[").unwrap();
        for (plane, channels) in planes.iter().zip(&channels) {
            let view = if planes.len() == 1 {
                vulkan
            } else {
                plane_view(plane.bits, channels.len())
            };
            let mut selectors: Vec<&str> = channels.chars().map(channel).collect();
            assert!(
                selectors.len() <= 4,
                "plane {channels:?} of {fourcc:?} has more than 4 channels"
            );
            selectors.resize(4, channel('A'));
            writeln!(out, "            Plane {{").unwrap();
            writeln!(out, "                view: vk::Format::{view},").unwrap();
            let width = plane.block_width;
            let height = plane.block_height;
            writeln!(
                out,
                "                subsampling: Subsampling {{ x: {width}, y: {height} }},"
            )
            .unwrap();
            writeln!(out, "                channels: [{}],", selectors.join(", ")).unwrap();
            writeln!(out, "            }},").unwrap();
        }
        writeln!(out, "        ],").unwrap();
        writeln!(out, "    }},").unwrap();
    }
    out.push_str("];\n");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    std::fs::write(Path::new(&out_dir).join("formats.rs"), out).unwrap();
}
