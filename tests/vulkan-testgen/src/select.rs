use crate::sizes::Size;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

const WORD_BITS: usize = u64::BITS as usize;

pub struct Vector {
    pub size: Size,
    pub layout: u32,
}

pub struct Pair {
    pub class: usize,
    pub vectors: Vec<Vector>,
}

pub struct NeighborKey {
    pub class: usize,
    pub fourcc: u32,
    pub modifier: u64,
}

pub struct Selection {
    pub retained: Vec<BTreeSet<usize>>,
    pub cover: usize,
    pub witnesses: usize,
    pub final_pass: usize,
}

#[derive(Copy, Clone)]
enum Kind {
    Value,
    Absence,
}

const KINDS: [Kind; 2] = [Kind::Value, Kind::Absence];

fn is_witness(kind: Kind, layout: u32, rule_layout: Option<u32>) -> bool {
    match kind {
        Kind::Value => rule_layout.is_some_and(|rule_layout| rule_layout != layout),
        Kind::Absence => rule_layout.is_none(),
    }
}

pub fn neighbors(keys: &[NeighborKey]) -> Vec<Vec<usize>> {
    let mut by_fourcc: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut by_modifier: HashMap<u64, Vec<usize>> = HashMap::new();
    for (index, key) in keys.iter().enumerate() {
        by_fourcc.entry(key.fourcc).or_default().push(index);
        by_modifier.entry(key.modifier).or_default().push(index);
    }
    let compute = |key: &NeighborKey| {
        let mut neighbors: Vec<usize> = by_fourcc[&key.fourcc]
            .iter()
            .chain(&by_modifier[&key.modifier])
            .copied()
            .filter(|&other| keys[other].class != key.class)
            .collect();
        neighbors.sort_unstable();
        neighbors.dedup();
        neighbors
    };
    keys.par_iter().map(compute).collect()
}

pub fn select<R>(pairs: &[Pair], neighbors: &[Vec<usize>], rule: &R) -> Selection
where
    R: Fn(usize, Size) -> Option<u32> + Sync,
{
    let mut retained = cover(pairs);
    let cover = count(&retained);
    let requirements = add_witnesses(pairs, neighbors, rule, &mut retained);
    let witnesses = count(&retained);
    final_pass(pairs, rule, &requirements, &mut retained);
    let final_pass = count(&retained);
    Selection {
        retained,
        cover,
        witnesses,
        final_pass,
    }
}

fn count(retained: &[BTreeSet<usize>]) -> usize {
    retained.iter().map(|kept| kept.len()).sum()
}

struct Retained {
    pair: usize,
    vector: usize,
}

fn cover(pairs: &[Pair]) -> Vec<BTreeSet<usize>> {
    let mut classes: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (index, pair) in pairs.iter().enumerate() {
        if !pair.vectors.is_empty() {
            classes.entry(pair.class).or_default().push(index);
        }
    }
    let classes: Vec<Vec<usize>> = classes.into_values().collect();
    let edges: Vec<Vec<Retained>> = classes
        .par_iter()
        .map(|members| cover_class(pairs, members))
        .collect();
    let mut retained = vec![BTreeSet::new(); pairs.len()];
    for edge in edges.into_iter().flatten() {
        retained[edge.pair].insert(edge.vector);
    }
    retained
}

#[derive(Copy, Clone)]
struct Edge {
    size: usize,
    vector: usize,
}

#[derive(Copy, Clone)]
struct MemberVector {
    member: usize,
    vector: usize,
}

fn cover_class(pairs: &[Pair], members: &[usize]) -> Vec<Retained> {
    let sizes: BTreeSet<Size> = members
        .iter()
        .flat_map(|&pair| pairs[pair].vectors.iter().map(|vector| vector.size))
        .collect();
    let size_index: HashMap<Size, usize> = sizes
        .iter()
        .enumerate()
        .map(|(index, &size)| (size, index))
        .collect();
    let adjacency: Vec<Vec<Edge>> = members
        .iter()
        .map(|&pair| {
            pairs[pair]
                .vectors
                .iter()
                .enumerate()
                .map(|(vector, entry)| Edge {
                    size: size_index[&entry.size],
                    vector,
                })
                .collect()
        })
        .collect();

    let mut partner: Vec<Option<usize>> = vec![None; sizes.len()];
    let mut matched = vec![false; members.len()];
    for (member, matched) in matched.iter_mut().enumerate() {
        let mut visited = vec![false; sizes.len()];
        *matched = augment(member, &adjacency, &mut visited, &mut partner);
    }

    let mut edges = Vec::new();
    let mut first_edge: Vec<Option<MemberVector>> = vec![None; sizes.len()];
    for (member, edge_list) in adjacency.iter().enumerate() {
        for edge in edge_list {
            first_edge[edge.size].get_or_insert(MemberVector {
                member,
                vector: edge.vector,
            });
            if partner[edge.size] == Some(member) {
                edges.push(Retained {
                    pair: members[member],
                    vector: edge.vector,
                });
            }
        }
        if !matched[member] {
            edges.push(Retained {
                pair: members[member],
                vector: edge_list[0].vector,
            });
        }
    }
    for size in 0..sizes.len() {
        if partner[size].is_none() {
            let first = first_edge[size].expect("every size has a vector");
            edges.push(Retained {
                pair: members[first.member],
                vector: first.vector,
            });
        }
    }
    edges
}

fn augment(
    member: usize,
    adjacency: &[Vec<Edge>],
    visited: &mut [bool],
    partner: &mut [Option<usize>],
) -> bool {
    for edge in &adjacency[member] {
        let size = edge.size;
        if visited[size] {
            continue;
        }
        visited[size] = true;
        let free = match partner[size] {
            Some(other) => augment(other, adjacency, visited, partner),
            None => true,
        };
        if free {
            partner[size] = Some(member);
            return true;
        }
    }
    false
}

struct Requirement {
    neighbor: usize,
    kind: Kind,
}

fn add_witnesses<R>(
    pairs: &[Pair],
    neighbors: &[Vec<usize>],
    rule: &R,
    retained: &mut [BTreeSet<usize>],
) -> Vec<Vec<Requirement>>
where
    R: Fn(usize, Size) -> Option<u32> + Sync,
{
    let compute = |(index, kept): (usize, &mut BTreeSet<usize>)| {
        let pair = &pairs[index];
        let mut requirements = Vec::new();
        if pair.vectors.is_empty() {
            return requirements;
        }
        for &neighbor in &neighbors[index] {
            for kind in KINDS {
                let witnesses = |vector: usize| {
                    let vector = &pair.vectors[vector];
                    is_witness(kind, vector.layout, rule(neighbor, vector.size))
                };
                if kept.iter().any(|&vector| witnesses(vector)) {
                    requirements.push(Requirement { neighbor, kind });
                } else if let Some(vector) = (0..pair.vectors.len()).find(|&v| witnesses(v)) {
                    kept.insert(vector);
                    requirements.push(Requirement { neighbor, kind });
                }
            }
        }
        requirements
    };
    retained.par_iter_mut().enumerate().map(compute).collect()
}

fn set_bits(words: &[u64]) -> impl Iterator<Item = usize> + '_ {
    words.iter().enumerate().flat_map(|(index, &word)| {
        (0..WORD_BITS)
            .filter(move |bit| word >> bit & 1 == 1)
            .map(move |bit| index * WORD_BITS + bit)
    })
}

#[derive(PartialEq, Eq, Hash)]
struct ClassSize {
    class: usize,
    size: Size,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Candidate {
    class: usize,
    pair: usize,
    vector: usize,
}

fn final_pass<R>(
    pairs: &[Pair],
    rule: &R,
    requirements: &[Vec<Requirement>],
    retained: &mut [BTreeSet<usize>],
) where
    R: Fn(usize, Size) -> Option<u32> + Sync,
{
    let compute = |(index, kept): (usize, &BTreeSet<usize>)| {
        let pair = &pairs[index];
        kept.iter()
            .map(|&vector| {
                let entry = &pair.vectors[vector];
                let mut words = vec![0u64; requirements[index].len().div_ceil(WORD_BITS)];
                for (bit, requirement) in requirements[index].iter().enumerate() {
                    let rule_layout = rule(requirement.neighbor, entry.size);
                    if is_witness(requirement.kind, entry.layout, rule_layout) {
                        words[bit / WORD_BITS] |= 1 << (bit % WORD_BITS);
                    }
                }
                (vector, words)
            })
            .collect::<BTreeMap<_, _>>()
    };
    let witnessed: Vec<BTreeMap<usize, Vec<u64>>> =
        retained.par_iter().enumerate().map(compute).collect();

    let mut remaining_witnesses: Vec<Vec<u32>> = requirements
        .iter()
        .map(|list| vec![0; list.len()])
        .collect();
    let mut at_size: HashMap<ClassSize, u32> = HashMap::new();
    let mut order = Vec::new();
    for (pair, kept) in retained.iter().enumerate() {
        let class = pairs[pair].class;
        for &vector in kept {
            for requirement in set_bits(&witnessed[pair][&vector]) {
                remaining_witnesses[pair][requirement] += 1;
            }
            let size = pairs[pair].vectors[vector].size;
            *at_size.entry(ClassSize { class, size }).or_default() += 1;
            order.push(Candidate {
                class,
                pair,
                vector,
            });
        }
    }
    order.sort_unstable();

    for candidate in order.iter().rev() {
        let pair = candidate.pair;
        let vector = candidate.vector;
        let key = ClassSize {
            class: candidate.class,
            size: pairs[pair].vectors[vector].size,
        };
        let words = &witnessed[pair][&vector];
        let removable = at_size[&key] >= 2
            && retained[pair].len() >= 2
            && set_bits(words).all(|requirement| remaining_witnesses[pair][requirement] >= 2);
        if removable {
            retained[pair].remove(&vector);
            *at_size.get_mut(&key).expect("the size was counted") -= 1;
            for requirement in set_bits(words) {
                remaining_witnesses[pair][requirement] -= 1;
            }
        }
    }
}
