use super::*;
use std::collections::BTreeSet;

#[test]
fn the_modifier_lists_have_their_drm_fourcc_values() {
    assert_eq!(
        ASAHI_MODIFIERS,
        [0, 0x0c00_0000_0000_0001, 0x0c00_0000_0000_0002]
    );
    assert_eq!(
        PANFROST_MODIFIERS,
        [
            0x0800_0000_0000_0042,
            0x0800_0000_0000_0062,
            0x0800_0000_0000_0072,
            0x0800_0000_0000_0351,
            0x0800_0000_0000_0341,
            0x0800_0000_0000_0051,
            0x0800_0000_0000_0041,
            0x0800_0000_0000_0061,
            0x0810_0000_0000_0001,
            0x0810_0000_0000_0002,
            0x0000_0000_0000_0000,
            0x0820_0000_0000_0001,
            0x0820_0000_0000_0002,
            0x0820_0000_0000_0003,
            0x0820_0000_0000_0101,
            0x0820_0000_0000_0102,
            0x0820_0000_0000_0103,
        ]
    );
    assert_eq!(
        V3D_MODIFIERS,
        [0x0700_0000_0000_0006, 0, 0x0700_0000_0000_0004]
    );
    assert_eq!(
        ETNAVIV_MODIFIERS,
        [
            0,
            0x0600_0000_0000_0001,
            0x0600_0000_0000_0002,
            0x0600_0000_0000_0003,
            0x0600_0000_0000_0004,
        ]
    );
    assert_eq!(LIMA_MODIFIERS, [0, 0x0810_0000_0000_0001]);
    assert_eq!(VC4_MODIFIERS, [0, 0x0700_0000_0000_0001]);
}

#[test]
fn the_extent_lists_end_at_the_common_display_size() {
    assert_eq!(DUMB_WIDTHS[..7], DUMB_HEIGHTS);
    assert_eq!(DUMB_WIDTHS[7..], [1920, 4096, 16384]);
    assert_eq!(ASAHI_WIDTHS[..10], ASAHI_HEIGHTS[..10]);
    assert_eq!(PANFROST_WIDTHS[..10], PANFROST_HEIGHTS[..10]);
    assert_eq!(VIRTIO_NATIVE_WIDTHS[..7], VIRTIO_NATIVE_HEIGHTS[..7]);
    for widths in [&ASAHI_WIDTHS[..], &PANFROST_WIDTHS, &VIRTIO_NATIVE_WIDTHS] {
        assert_eq!(widths.last(), Some(&1920));
    }
    for heights in [
        &ASAHI_HEIGHTS[..],
        &PANFROST_HEIGHTS,
        &VIRTIO_NATIVE_HEIGHTS,
    ] {
        assert_eq!(heights.last(), Some(&1080));
    }
}

#[test]
fn every_driver_has_its_own_name_and_file() {
    let names: BTreeSet<&str> = DRIVERS.iter().map(|driver| driver.name).collect();
    let files: BTreeSet<&str> = DRIVERS.iter().map(|driver| driver.file).collect();
    assert_eq!(names.len(), DRIVERS.len());
    assert_eq!(files.len(), DRIVERS.len());
    let expected = [
        "dumb",
        "virtio_gpu_2d",
        "virtio_gpu_virgl",
        "virtio_gpu_amdgpu",
        "virtio_gpu_i915",
        "virtio_gpu_asahi",
        "virtio_gpu_panfrost",
        "asahi",
        "panfrost",
        "panthor",
        "v3d",
        "etnaviv",
        "lima",
        "vc4",
    ];
    let files: Vec<&str> = DRIVERS.iter().map(|driver| driver.file).collect();
    assert_eq!(files, expected);
}

#[test]
fn only_the_dumb_drivers_use_the_primary_node() {
    for driver in &DRIVERS {
        let dumb = matches!(driver.name, "dumb" | "virtio_gpu_2d");
        assert_eq!(
            matches!(driver.node, Node::Primary),
            dumb,
            "{}",
            driver.name
        );
        assert_eq!(driver.scanout == SCANOUT_ONLY, dumb, "{}", driver.name);
    }
}

#[test]
fn every_driver_registers_the_kernels_of_its_table() {
    let count = |name: &str| {
        let driver = DRIVERS.iter().find(|driver| driver.name == name).unwrap();
        (driver.kernels)().len()
    };
    assert_eq!(count("dumb"), 1);
    assert_eq!(count("virtio_gpu_amdgpu"), 1);
    assert_eq!(count("virtio_gpu_i915"), VIRTIO_I915_CHIPS.len());
    assert_eq!(count("virtio_gpu_asahi"), VIRTIO_ASAHI_CHIPS.len());
    assert_eq!(
        count("virtio_gpu_panfrost"),
        fake_kernel::panfrost::GPUS.len()
    );
    assert_eq!(count("asahi"), fake_kernel::asahi::CHIPS.len());
    assert_eq!(count("vc4"), fake_kernel::vc4::GPUS.len());
    for driver in &DRIVERS {
        let kernels = (driver.kernels)();
        let distinct: BTreeSet<&String> = kernels.iter().collect();
        assert_eq!(distinct.len(), kernels.len(), "{}", driver.name);
    }
}

#[test]
fn the_host_drivers_name_their_bare_metal_chips() {
    let i915 = DRIVERS
        .iter()
        .find(|driver| driver.name == "virtio_gpu_i915");
    let Modifiers::IntelHost { chips } = i915.unwrap().modifiers else {
        panic!("virtio_gpu_i915 does not take its modifiers from an Intel host");
    };
    for chip in chips {
        let devices = fake_kernel::intel::devices::DEVICES.iter();
        assert!(devices.clone().any(|device| device.name == *chip), "{chip}");
    }
    let amdgpu = DRIVERS
        .iter()
        .find(|driver| driver.name == "virtio_gpu_amdgpu");
    let Modifiers::AmdHost { chip } = amdgpu.unwrap().modifiers else {
        panic!("virtio_gpu_amdgpu does not take its modifiers from an AMD host");
    };
    let mut devices = fake_kernel::amd::devices::DEVICES.iter();
    assert!(devices.any(|device| device.name == chip));
}
