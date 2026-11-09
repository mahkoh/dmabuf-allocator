use super::*;

#[test]
fn decimal_accepts_canonical_numbers() {
    assert_eq!(decimal("0"), Some(0));
    assert_eq!(decimal("226"), Some(226));
    assert_eq!(decimal("007"), None);
    assert_eq!(decimal(""), None);
    assert_eq!(decimal("+1"), None);
    assert_eq!(decimal("1x"), None);
}

#[test]
fn drm_minor_requires_the_drm_major() {
    assert_eq!(drm_minor("226:128"), Some("128"));
    assert_eq!(drm_minor("227:128"), None);
    assert_eq!(drm_minor("226"), None);
}

#[test]
fn last_component_splits_the_parent() {
    let last = last_component(b"/dev/dri/card0");
    assert_eq!(last.parent, b"/dev/dri/");
    assert_eq!(last.name, b"card0");
    assert!(!last.trailing_slash);
    let last = last_component(b"card0//");
    assert_eq!(last.parent, b"./");
    assert_eq!(last.name, b"card0");
    assert!(last.trailing_slash);
}

#[test]
fn process_links_are_recognized() {
    assert!(is_process_link("/dev/stdin"));
    assert!(is_process_link("/dev/fd/3"));
    assert!(!is_process_link("/dev/fd/3/x"));
    assert!(is_process_link("/proc/self/cwd"));
    assert!(is_process_link("/proc/1/task/2/fd/3"));
    assert!(!is_process_link("/proc/self/maps"));
    assert!(!is_process_link("/sys/class/drm"));
}

#[test]
fn permits_checks_the_group_bits_of_devices() {
    assert!(permits(S_IFCHR | 0o660, libc::R_OK | libc::W_OK));
    assert!(!permits(S_IFCHR | 0o660, libc::X_OK));
    assert!(permits(S_IFREG | 0o444, libc::R_OK));
    assert!(!permits(S_IFREG | 0o444, libc::W_OK));
}

#[test]
fn bus_links_climb_to_the_sysfs_root() {
    let Node::Link { target, resolved } = bus_link("virtio", 3) else {
        panic!("bus_link returns no link");
    };
    assert_eq!(target, "../../../bus/virtio");
    assert_eq!(resolved, "/sys/bus/virtio");
}

#[test]
fn bus_devices_have_three_entries() {
    let names = ["card0".to_string()];
    let node = bus_device(&names, "", bus_link("faux", 2), String::new()).unwrap();
    let Node::Directory(directory) = node else {
        panic!("the device is no directory");
    };
    let listed: Vec<String> = directory.entries().into_iter().map(|e| e.name).collect();
    assert_eq!(listed, [".", "..", "drm", "subsystem", "uevent"]);
    let missing = bus_device(&names, "vendor", bus_link("faux", 2), String::new());
    assert!(matches!(missing, Err(ENOENT)));
}
