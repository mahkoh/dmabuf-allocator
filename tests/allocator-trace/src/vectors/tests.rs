use super::*;

const COMPACT: &str = "\
device driver=v3d fake=v3d:gpu=bcm2711:kernel=7.2 node=render
format \"AR24\" 0x0
64x64 _ 0 256
v3d_create_bo size=16384 flags=0x0
prime_handle_to_fd flags=0x80002
128x64 _ 0 512 = size=32768
64x64 s 0 256 = size=16384
128x64 s 0 512 = size=32768
format \"NV12\" 0x0 implicit_sync=0 linear=1
64x64 _ 0 64 | 4096 64
v3d_create_bo size=8192 flags=0x0
prime_handle_to_fd flags=0x80002
64x64 s 0 64 | 4096 64 =
";

const VERBOSE: &str = "\
# a comment
device driver=v3d fake=v3d:gpu=bcm2711:kernel=7.2 node=render
case \"NV12\" 0x0 64x64 scanout=1 implicit_sync=0 linear=1
layout 0 64 | 4096 64
v3d_create_bo size=8192 flags=0x0
prime_handle_to_fd flags=0x80002
case \"AR24\" 0x0 128x64 scanout=0
layout 0 512
v3d_create_bo size=32768 flags=0x0
prime_handle_to_fd flags=0x80002
case \"AR24\" 0x0 64x64 scanout=0
layout 0 256
v3d_create_bo size=16384 flags=0x0
prime_handle_to_fd flags=0x80002
case \"AR24\" 0x0 64x64 scanout=1
layout 0 256
v3d_create_bo size=16384 flags=0x0
prime_handle_to_fd flags=0x80002
case \"AR24\" 0x0 128x64 scanout=1
layout 0 512
v3d_create_bo size=32768 flags=0x0
prime_handle_to_fd flags=0x80002
case \"NV12\" 0x0 64x64 scanout=0 implicit_sync=0 linear=1
layout 0 64 | 4096 64
v3d_create_bo size=8192 flags=0x0
prime_handle_to_fd flags=0x80002
";

fn write(devices: &[DeviceCases]) -> String {
    let mut out = String::new();
    for device in devices {
        write_compact_device(&mut out, &device.device, &device.cases);
    }
    out
}

fn error(text: &str) -> String {
    parse(text).unwrap_err().to_string()
}

#[test]
fn compact_vectors_are_parsed() {
    let devices = parse(COMPACT).unwrap();
    let [device] = devices.as_slice() else {
        panic!("{} devices were parsed", devices.len());
    };
    assert_eq!(device.device.driver, "v3d");
    assert_eq!(device.device.fake, "v3d:gpu=bcm2711:kernel=7.2");
    assert_eq!(device.device.node, Node::Render);
    assert_eq!(device.cases.len(), 6);
    let wide = &device.cases[1];
    assert_eq!(wide.fourcc, u32::from_le_bytes(*b"AR24"));
    assert_eq!(wide.width, 128);
    assert_eq!(wide.height, 64);
    assert!(!wide.scanout);
    assert_eq!(wide.flags, Flags::default());
    let plane = Plane {
        offset: 0,
        stride: 512,
    };
    assert_eq!(wide.layout, [plane]);
    let create = Call::V3dCreateBo {
        size: 32768,
        flags: 0,
    };
    let export = Call::PrimeHandleToFd { flags: 0x80002 };
    assert_eq!(wide.calls, [create, export]);
    let planar = &device.cases[5];
    assert!(planar.scanout);
    assert!(planar.flags.contains(Flag::ExplicitSync));
    assert!(planar.flags.contains(Flag::Linear));
    assert!(!planar.flags.contains(Flag::IgnoreSize));
    assert_eq!(planar.layout.len(), 2);
    assert_eq!(planar.calls, device.cases[4].calls);
}

#[test]
fn compact_vectors_are_written_as_they_were_read() {
    assert_eq!(write(&parse(COMPACT).unwrap()), COMPACT);
}

#[test]
fn verbose_vectors_are_written_compactly() {
    assert_eq!(write(&parse(VERBOSE).unwrap()), COMPACT);
}

#[test]
fn errors_name_the_line() {
    assert_eq!(
        error("case \"AR24\" 0x0 64x64 scanout=0"),
        "line 1: case before the first device"
    );
    let device = "device driver=v3d fake=v3d:gpu=bcm2711:kernel=7.2 node=render\n";
    let with = |lines: &str| error(&format!("{device}{lines}"));
    assert_eq!(
        with("case \"AR24\" 0x0 64x64 scanout=0 ignore_sise=1"),
        "line 2: unknown flag \"ignore_sise=1\""
    );
    assert_eq!(
        with("case \"AR24\" 0x0 64x64 scanout=0\nv3d_create_bo size=1 flags=0"),
        "line 2: case without layout"
    );
    assert_eq!(
        with("case \"AR24\" 0x0 64x64 scanout=2\nlayout 0 256"),
        "line 2: invalid scanout \"scanout=2\""
    );
    assert_eq!(
        with("case \"AR24\" 0 64x64 scanout=0\nlayout 0 256"),
        "line 2: invalid modifier \"0\""
    );
    assert_eq!(
        with("64x64 _ 0 256"),
        "line 2: case line outside a format group"
    );
    assert_eq!(
        with("format \"AR24\" 0x0\n64x64 _ 0 256 ="),
        "line 3: = without a previous case"
    );
    assert_eq!(
        with(
            "format \"AR24\" 0x0\n64x64 _ 0 256\nv3d_create_bo size=1 flags=0\n64x64 s 0 256 = stride=4"
        ),
        "line 5: override stride matches no field"
    );
    assert_eq!(with("frobnicate"), "line 2: unknown call frobnicate");
    assert_eq!(
        error("device driver=v3d fake=x node=render colour=1"),
        "line 1: unknown device field colour"
    );
    assert_eq!(
        error("device fake=x node=render"),
        "line 1: device without driver"
    );
}
