use super::*;

fn size(width: u32, height: u32) -> Size {
    Size { width, height }
}

fn vector(width: u32, layout: u32) -> Vector {
    Vector {
        size: size(width, 1),
        layout,
    }
}

fn pair<const N: usize>(class: usize, vectors: [Vector; N]) -> Pair {
    Pair {
        class,
        vectors: vectors.into(),
    }
}

fn key(class: usize, fourcc: u32, modifier: u64) -> NeighborKey {
    NeighborKey {
        class,
        fourcc,
        modifier,
    }
}

fn rule_of(pairs: &[Pair]) -> impl Fn(usize, Size) -> Option<u32> + Sync + '_ {
    |pair, size| {
        let vectors = pairs[pair].vectors.iter();
        vectors
            .clone()
            .find(|vector| vector.size == size)
            .map(|vector| vector.layout)
    }
}

#[test]
fn neighbors_share_a_fourcc_or_a_modifier_but_not_the_class() {
    let keys = [
        key(0, 1, 10),
        key(1, 1, 11),
        key(2, 2, 10),
        key(0, 2, 11),
        key(3, 3, 12),
    ];
    let expected: [&[usize]; 5] = [&[1, 2], &[0, 3], &[0, 3], &[1, 2], &[]];
    assert_eq!(neighbors(&keys), expected);
}

#[test]
fn a_witness_differs_or_is_absent() {
    assert!(is_witness(Kind::Value, 1, Some(2)));
    assert!(!is_witness(Kind::Value, 1, Some(1)));
    assert!(!is_witness(Kind::Value, 1, None));
    assert!(is_witness(Kind::Absence, 1, None));
    assert!(!is_witness(Kind::Absence, 1, Some(2)));
}

#[test]
fn the_cover_keeps_every_pair_and_every_size_of_a_class() {
    let pairs = [
        pair(0, [vector(1, 0), vector(2, 0), vector(3, 0)]),
        pair(0, [vector(1, 0), vector(2, 0)]),
        pair(1, [vector(4, 0)]),
        pair(1, []),
    ];
    let retained = cover(&pairs);
    assert!(!retained[0].is_empty());
    assert!(!retained[1].is_empty());
    assert_eq!(retained[2], BTreeSet::from([0]));
    assert!(retained[3].is_empty());
    let mut widths = BTreeSet::new();
    for (pair, kept) in pairs.iter().zip(&retained).take(2) {
        for &vector in kept {
            widths.insert(pair.vectors[vector].size.width);
        }
    }
    assert_eq!(widths, BTreeSet::from([1, 2, 3]));
    assert_eq!(count(&retained), 4);
}

#[test]
fn the_selection_keeps_a_witness_against_each_neighbor() {
    let pairs = [
        pair(0, [vector(1, 0), vector(2, 0), vector(3, 1)]),
        pair(1, [vector(1, 0), vector(2, 0), vector(3, 2)]),
    ];
    let neighbors = [vec![1], vec![0]];
    let rule = rule_of(&pairs);
    let selection = select(&pairs, &neighbors, &rule);
    assert_eq!(selection.cover, 6);
    assert!(selection.witnesses >= selection.cover);
    assert!(selection.final_pass <= selection.witnesses);
    assert_eq!(selection.final_pass, count(&selection.retained));
    for (pair, kept) in pairs.iter().zip(&selection.retained) {
        let widths: Vec<u32> = kept
            .iter()
            .map(|&vector| pair.vectors[vector].size.width)
            .collect();
        assert!(widths.contains(&3), "{widths:?}");
    }
}

#[test]
fn the_selection_keeps_a_size_that_the_neighbor_rejects() {
    let pairs = [
        pair(0, [vector(1, 0), vector(2, 0)]),
        pair(1, [vector(1, 0)]),
    ];
    let neighbors = [vec![1], vec![0]];
    let rule = rule_of(&pairs);
    let selection = select(&pairs, &neighbors, &rule);
    let kept: Vec<u32> = selection.retained[0]
        .iter()
        .map(|&vector| pairs[0].vectors[vector].size.width)
        .collect();
    assert!(kept.contains(&2), "{kept:?}");
}

#[test]
fn the_set_bits_are_listed_in_order() {
    let bits: Vec<usize> = set_bits(&[0b101, 1 << 63, 1]).collect();
    assert_eq!(bits, [0, 2, 127, 128]);
}
