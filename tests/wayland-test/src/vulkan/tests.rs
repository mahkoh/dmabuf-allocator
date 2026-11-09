use super::*;

#[test]
fn push_constants_are_serialized_in_shader_order() {
    let extent = Extent {
        width: 800,
        height: 600,
    };
    let subsampling = Subsampling { x: 2, y: 1 };
    let channels = [Channel::Y, Channel::U, Channel::V, Channel::One];
    let push = push_constants(channels, &extent, subsampling, 0.5);
    let mut expected = Vec::new();
    for channel in [3i32, 4, 5, 6] {
        expected.extend_from_slice(&channel.to_ne_bytes());
    }
    for float in [800.0f32, 600.0, 2.0, 1.0, 0.5] {
        expected.extend_from_slice(&float.to_ne_bytes());
    }
    assert_eq!(push.bytes(), expected);
}
