use super::*;
use std::io::Write;

fn table(entries: &[Pair]) -> OwnedFd {
    let mut file = File::from(memfd());
    for entry in entries {
        file.write_all(&entry.fourcc.to_ne_bytes()).unwrap();
        file.write_all(&[0; 4]).unwrap();
        file.write_all(&entry.modifier.to_ne_bytes()).unwrap();
    }
    file.into()
}

fn memfd() -> OwnedFd {
    use std::os::fd::FromRawFd;
    let fd = unsafe { libc::memfd_create(c"format-table".as_ptr(), libc::MFD_CLOEXEC) };
    assert!(fd >= 0);
    unsafe { OwnedFd::from_raw_fd(fd) }
}

const XRGB: Pair = Pair {
    fourcc: u32::from_le_bytes(*b"XR24"),
    modifier: 0,
};
const NV12: Pair = Pair {
    fourcc: u32::from_le_bytes(*b"NV12"),
    modifier: 0x0100_0000_0000_0002,
};

#[test]
fn tranches_index_the_format_table() {
    let mut builder = FeedbackBuilder::default();
    builder.format_table(table(&[XRGB, NV12]), 32);
    builder.main_device(&0x1234u64.to_ne_bytes());
    builder.tranche_target_device(&0x5678u64.to_ne_bytes());
    builder.tranche_scanout(true);
    builder.tranche_formats(&[1, 0, 0, 0]);
    builder.tranche_done();
    builder.tranche_formats(&[0, 0, 7, 0]);
    builder.tranche_done();
    let feedback = builder.done();
    assert_eq!(feedback.main_device, 0x1234);
    assert_eq!(feedback.tranches.len(), 2);
    assert_eq!(feedback.tranches[0].target_device, 0x5678);
    assert!(feedback.tranches[0].scanout);
    assert_eq!(feedback.tranches[0].pairs, [NV12, XRGB]);
    assert!(!feedback.tranches[1].scanout);
    assert_eq!(feedback.tranches[1].pairs, [XRGB]);
    assert!(builder.done().tranches.is_empty());
}

#[test]
fn incomplete_table_entries_are_dropped() {
    let mut builder = FeedbackBuilder::default();
    builder.format_table(table(&[XRGB, NV12]), 24);
    builder.tranche_formats(&[0, 0, 1, 0]);
    builder.tranche_done();
    assert_eq!(builder.done().tranches[0].pairs, [XRGB]);
}
