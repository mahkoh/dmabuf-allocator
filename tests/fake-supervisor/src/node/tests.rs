use super::*;

fn node(device: usize, kind: Node) -> DrmNode {
    DrmNode { device, kind }
}

#[test]
fn first_devices_use_the_pools_of_their_type() {
    assert_eq!(node(0, Node::Primary).minor(), 0);
    assert_eq!(node(0, Node::Render).minor(), 128);
    assert_eq!(node(63, Node::Primary).minor(), 63);
    assert_eq!(node(63, Node::Render).minor(), 191);
}

#[test]
fn later_devices_use_the_extended_pool() {
    assert_eq!(node(64, Node::Render).minor(), 192);
    assert_eq!(node(64, Node::Primary).minor(), 193);
    assert_eq!(node(65, Node::Render).minor(), 194);
    assert_eq!(node(200, Node::Primary).minor(), 465);
}

#[test]
fn minors_name_one_node() {
    let mut minors = std::collections::BTreeSet::new();
    for device in 0..1000 {
        for kind in [Node::Primary, Node::Render] {
            let node = node(device, kind);
            assert!(minors.insert(node.minor()), "{node:?} shares its minor");
            assert_eq!(DrmNode::from_minor(node.minor()), Some(node));
        }
    }
}

#[test]
fn unused_minors_name_no_node() {
    assert_eq!(DrmNode::from_minor(64), None);
    assert_eq!(DrmNode::from_minor(127), None);
    assert_eq!(DrmNode::from_minor(1 << 20), None);
}

#[test]
fn last_device_has_the_last_minor() {
    assert_eq!(node(MAX_DEVICES - 1, Node::Primary).minor(), (1 << 20) - 1);
}

#[test]
fn names_contain_the_minor() {
    assert_eq!(node(3, Node::Primary).name(), "card3");
    assert_eq!(node(3, Node::Render).name(), "renderD131");
    assert_eq!(node(64, Node::Primary).name(), "card193");
    assert_eq!(node(64, Node::Render).name(), "renderD192");
}
