use super::*;

#[test]
fn a_fourcc_is_its_bytes_in_little_endian_order() {
    assert_eq!(fourcc_code("XR24"), 0x3432_5258);
}

#[test]
#[should_panic(expected = "must be 4 bytes")]
fn a_short_fourcc_is_refused() {
    fourcc_code("XR2");
}

#[test]
fn bytes_are_an_eighth_of_the_bits() {
    let block = Block {
        bits: 96,
        width: 1,
        height: 1,
    };
    assert_eq!(block.bytes(), 12);
}

#[test]
#[should_panic(expected = "not a whole number of bytes")]
fn partial_bytes_are_refused() {
    let block = Block {
        bits: 12,
        width: 1,
        height: 1,
    };
    block.bytes();
}

#[test]
fn every_vulkan_block_has_whole_bytes() {
    for format in FORMATS {
        for block in format.planes {
            assert!(block.bytes() >= 1, "{}", format.fourcc);
        }
    }
}

#[test]
fn the_largest_block_spans_all_planes() {
    let planes = [
        Block {
            bits: 8,
            width: 1,
            height: 1,
        },
        Block {
            bits: 16,
            width: 2,
            height: 2,
        },
        Block {
            bits: 8,
            width: 4,
            height: 1,
        },
    ];
    let expected = Size {
        width: 4,
        height: 2,
    };
    assert_eq!(largest_block(&planes), expected);
}

#[test]
fn the_largest_block_follows_the_vulkan_subsampling_names() {
    for format in FORMATS {
        let name = format!("{:?}", format.format);
        let expected = Size {
            width: 1 + (name.contains("_422") || name.contains("_420")) as u32,
            height: 1 + name.contains("_420") as u32,
        };
        assert_eq!(largest_block(format.planes), expected, "{name}");
    }
}

#[test]
fn every_vulkan_format_is_in_the_table_of_all_formats() {
    for format in FORMATS {
        assert!(
            ALL_FORMATS.iter().any(|all| all.fourcc == format.fourcc),
            "{}",
            format.fourcc
        );
    }
}
