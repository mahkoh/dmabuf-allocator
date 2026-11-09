use allocator_trace::Traced;
use fake_kernel::Node;
use fake_supervisor::DrmNode;
use fake_supervisor::client;
use parking_lot::Mutex;
use std::fs::File;
use std::fs::OpenOptions;
use std::ops::Range;
use std::os::fd::OwnedFd;

static IDS: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn kernel_id(node: u32) -> String {
    IDS.lock()[node as usize].clone()
}

pub fn set_visible(range: Option<Range<usize>>) {
    client::set_visible(range);
}

pub fn register(ids: Vec<String>) -> Range<usize> {
    let mut registered = IDS.lock();
    let range = client::register(&ids);
    assert_eq!(
        range.start,
        registered.len(),
        "the supervisor numbered the nodes differently"
    );
    registered.extend(ids);
    range
}

pub fn start_recording() {
    client::start_recording();
}

pub fn stop_recording() -> Vec<Traced> {
    client::stop_recording()
}

pub fn open_node(node: u32, kind: Node) -> OwnedFd {
    let drm_node = DrmNode {
        device: node as usize,
        kind,
    };
    let path = format!("/dev/dri/{}", drm_node.name());
    let file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .unwrap_or_else(|error| panic!("cannot open {path}: {error}"));
    file.into()
}
