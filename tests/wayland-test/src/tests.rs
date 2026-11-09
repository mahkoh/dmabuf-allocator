use super::*;

#[test]
fn sizes_are_rounded_halfway_away_from_zero() {
    assert_eq!(scaled(100, 120), 100);
    assert_eq!(scaled(100, 180), 150);
    assert_eq!(scaled(50, 180), 75);
    assert_eq!(scaled(3, 150), 4);
    assert_eq!(scaled(1, 150), 1);
    assert_eq!(scaled(1, 180), 2);
    assert_eq!(scaled(5, 132), 6);
    assert_eq!(scaled(1, 132), 1);
}
