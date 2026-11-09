use super::*;
use vulkan_testgen::generate::Backend as _;

fn described() -> Described {
    let text = "device 3 0\n\
                unit 0 0x7 1 64 64 1 1x1\n\
                unit 1 0x7 1 64 64 1 1x1\n";
    vulkan::merge(vulkan::parse_descriptions(text))
}

#[test]
fn a_gbm_pair_index_holds_the_base_pair_and_scanout() {
    for base in [0, 1, 500] {
        for scanout in SCANOUTS {
            let pair = GbmPair { base, scanout };
            assert_eq!(GbmPair::from_index(pair.index()), pair);
        }
    }
    let scanout = GbmPair {
        base: 3,
        scanout: true,
    };
    assert_eq!(scanout.index(), 7);
}

#[test]
fn the_gbm_plan_has_each_vulkan_pair_with_and_without_scanout() {
    let described = described();
    let plan = GbmPlan {
        described: &described,
    };
    let pairs = plan.pairs();
    let indices: Vec<usize> = pairs.iter().map(|pair| pair.index).collect();
    let scanouts: Vec<Option<bool>> = pairs.iter().map(|pair| pair.scanout).collect();
    assert_eq!(indices, [0, 1, 2, 3]);
    assert_eq!(scanouts, [Some(false), Some(true), Some(false), Some(true)]);
    assert!(plan.class_key(0) != plan.class_key(1));
    assert!(plan.class_key(0) != plan.class_key(2));
    assert!(plan.class_key(1) == plan.class_key(1));
}
