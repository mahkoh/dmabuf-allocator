use super::*;

struct Multiple {
    fourcc: &'static str,
    width: u32,
    height: u32,
}

const fn multiple(fourcc: &'static str, width: u32, height: u32) -> Multiple {
    Multiple {
        fourcc,
        width,
        height,
    }
}

const SUBSAMPLED: [Multiple; 28] = [
    multiple("NV12", 2, 2),
    multiple("NV21", 2, 2),
    multiple("NV16", 2, 1),
    multiple("NV61", 2, 1),
    multiple("P010", 2, 2),
    multiple("P012", 2, 2),
    multiple("P210", 2, 1),
    multiple("P016", 2, 2),
    multiple("S016", 2, 2),
    multiple("S216", 2, 1),
    multiple("UYVY", 2, 1),
    multiple("VYUY", 2, 1),
    multiple("YUYV", 2, 1),
    multiple("YVYU", 2, 1),
    multiple("Y210", 2, 1),
    multiple("Y212", 2, 1),
    multiple("Y216", 2, 1),
    multiple("YUV9", 4, 4),
    multiple("YVU9", 4, 4),
    multiple("YU11", 4, 1),
    multiple("YV11", 4, 1),
    multiple("YU12", 2, 2),
    multiple("YV12", 2, 2),
    multiple("YU16", 2, 1),
    multiple("YV16", 2, 1),
    multiple("NV24", 1, 1),
    multiple("Q410", 1, 1),
    multiple("YU24", 1, 1),
];

fn literal_modifier(device: &DeviceConfig, compressed: bool, height_log2: u64) -> u64 {
    let kind = match compressed {
        true => device.compressed_kind,
        false => device.kind,
    };
    let sector_layout = device.sector_layout as u64;
    0x03 << 56
        | (sector_layout & 6) << 25
        | (compressed as u64) << 23
        | (sector_layout & 1) << 22
        | (device.generation as u64) << 20
        | (kind as u64) << 12
        | 0x10
        | height_log2
}

#[test]
fn the_modifier_is_the_block_linear_macro_of_the_device() {
    for device in &DEVICES {
        for compressed in [false, true] {
            for height_log2 in BLOCK_HEIGHTS_LOG2 {
                assert_eq!(
                    device.block_linear_modifier(compressed, height_log2),
                    literal_modifier(device, compressed, height_log2)
                );
            }
        }
    }
    assert_eq!(
        DEVICES[0].block_linear_modifier(false, 0),
        0x0300_0000_004f_e010
    );
}

#[test]
fn the_size_multiples_are_the_largest_blocks() {
    for expected in &SUBSAMPLED {
        let format = FORMATS
            .iter()
            .find(|format| format.fourcc == expected.fourcc);
        let found = size_multiple(format.unwrap());
        assert_eq!(found.width, expected.width, "{}", expected.fourcc);
        assert_eq!(found.height, expected.height, "{}", expected.fourcc);
    }
    for format in &FORMATS {
        let listed = SUBSAMPLED.iter().any(|known| known.fourcc == format.fourcc);
        if !listed {
            let one = Size {
                width: 1,
                height: 1,
            };
            assert_eq!(size_multiple(format), one, "{}", format.fourcc);
        }
    }
}

#[test]
fn every_format_is_listed_once() {
    let mut names: Vec<&str> = FORMATS.iter().map(|format| format.fourcc).collect();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), FORMATS.len());
    for format in &FORMATS {
        assert_eq!(
            fourcc_code(format.fourcc).to_le_bytes(),
            format.fourcc.as_bytes()
        );
    }
}

#[test]
fn the_sizes_surround_the_block_and_the_edge_width() {
    let size = |width: u32, height: u32| Size { width, height };
    let single = Size {
        width: 1,
        height: 1,
    };
    let expected = [
        size(1, 1),
        size(256, 16),
        size(257, 16),
        size(256, 17),
        size(257, 1025),
        size(17, 17),
        size(1920, 1080),
        size(16384, 16384),
    ];
    assert_eq!(sizes(single, 1), expected);
    let subsampled = Size {
        width: 4,
        height: 2,
    };
    let expected = [
        size(4, 2),
        size(256, 16),
        size(260, 16),
        size(256, 18),
        size(260, 2050),
        size(68, 34),
        size(1920, 1080),
        size(16384, 16384),
    ];
    assert_eq!(sizes(subsampled, 0), expected);
}

#[test]
fn only_the_legacy_formats_are_canonicalized() {
    let canonical = |format: u32| unsafe { format_canonicalize(format) };
    assert_eq!(canonical(GBM_BO_FORMAT_XRGB8888), fourcc_code("XR24"));
    assert_eq!(canonical(GBM_BO_FORMAT_ARGB8888), fourcc_code("AR24"));
    assert_eq!(canonical(fourcc_code("NV12")), fourcc_code("NV12"));
}

#[test]
fn every_device_has_its_own_kernel() {
    let mut ids: Vec<String> = DEVICES.iter().map(DeviceConfig::kernel_id).collect();
    assert!(ids[1].contains("pci_id=0x2684"), "{}", ids[1]);
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), DEVICES.len());
}
