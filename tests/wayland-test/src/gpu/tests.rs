use super::*;

fn entry(scanout: bool) -> Entry {
    Entry {
        format: &formats::FORMATS[0],
        modifier: 0,
        scanout,
    }
}

#[test]
fn split_separates_the_halves() {
    let halves = split(0x1234_5678_9abc_def0);
    assert_eq!(halves.hi, 0x1234_5678);
    assert_eq!(halves.lo, 0x9abc_def0);
}

#[test]
fn entries_with_different_scanout_differ() {
    let fourcc = formats::FORMATS[0].fourcc;
    assert!(entry(false).is(fourcc, 0));
    assert!(!entry(false).is(fourcc, 1));
    assert!(entry(false).same(&entry(false)));
    assert!(!entry(false).same(&entry(true)));
}
