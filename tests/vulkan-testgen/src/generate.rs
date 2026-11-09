use crate::select::NeighborKey;
use crate::select::Pair;
use crate::select::Selection;
use crate::select::Vector;
use crate::select::neighbors;
use crate::select::select;
use crate::sizes::Size;
use allocator_trace::vectors;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelSliceMut;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::BufWriter;
use std::io::Write;
use std::ops::Range;
use std::path::Path;

#[cfg(test)]
mod tests;

pub const MAX_PLANES: usize = 4;

const ZSTD_LEVEL: i32 = 11;
pub const FILE_SUFFIX: &str = ".txt.zst";
const VULKAN_SUFFIX: &str = "_vulkan";

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlaneLayout {
    pub offset: u64,
    pub stride: u64,
    pub size: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Layout {
    planes: [PlaneLayout; MAX_PLANES],
    len: u8,
}

impl Layout {
    pub fn planes(&self) -> &[PlaneLayout] {
        &self.planes[..self.len as usize]
    }
}

impl FromIterator<PlaneLayout> for Layout {
    fn from_iter<I: IntoIterator<Item = PlaneLayout>>(planes: I) -> Self {
        let mut layout = Layout {
            planes: [PlaneLayout::default(); MAX_PLANES],
            len: 0,
        };
        for plane in planes {
            layout.planes[layout.len as usize] = plane;
            layout.len += 1;
        }
        layout
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Evaluation {
    Vector(Layout),
    Rejected,
}

pub struct PairInfo<'a> {
    pub index: usize,
    pub fourcc: u32,
    pub modifier: u64,
    pub scanout: Option<bool>,
    pub sizes: &'a BTreeSet<Size>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PairSize {
    pub pair: usize,
    pub size: Size,
}

pub trait Backend {
    type ClassKey: Eq + Hash;

    fn pairs(&self) -> Vec<PairInfo<'_>>;
    fn class_key(&self, pair: usize) -> Self::ClassKey;
}

fn class_ids<B: Backend>(backend: &B, pairs: &[PairInfo<'_>]) -> Vec<usize> {
    let mut ids: HashMap<B::ClassKey, usize> = HashMap::new();
    pairs
        .iter()
        .map(|pair| {
            let next = ids.len();
            *ids.entry(backend.class_key(pair.index)).or_insert(next)
        })
        .collect()
}

pub struct Run {
    pub pair: usize,
    pub range: Range<usize>,
}

pub fn pair_runs(sizes: &[PairSize]) -> Vec<Run> {
    let mut runs = Vec::new();
    let mut start = 0;
    while start < sizes.len() {
        let pair = sizes[start].pair;
        let end = start + sizes[start..].partition_point(|size| size.pair == pair);
        runs.push(Run {
            pair,
            range: start..end,
        });
        start = end;
    }
    runs
}

pub struct Plan<'a> {
    pub pairs: Vec<PairInfo<'a>>,
    pub own_sizes: Vec<PairSize>,
    pub own_ranges: Vec<Range<usize>>,
    pub classes: Vec<usize>,
    pub neighbors: Vec<Vec<usize>>,
}

impl<'a> Plan<'a> {
    pub fn new<B: Backend>(backend: &'a B) -> Self {
        let mut pairs = backend.pairs();
        pairs.sort_by_key(|pair| (pair.modifier, pair.fourcc, pair.scanout));
        let own_sizes: Vec<PairSize> = pairs
            .iter()
            .enumerate()
            .flat_map(|(pair, info)| info.sizes.iter().map(move |&size| PairSize { pair, size }))
            .collect();
        let mut own_ranges = Vec::with_capacity(pairs.len());
        let mut start = 0;
        for info in &pairs {
            own_ranges.push(start..start + info.sizes.len());
            start += info.sizes.len();
        }
        let classes = class_ids(backend, &pairs);
        let keys: Vec<NeighborKey> = pairs
            .iter()
            .zip(&classes)
            .map(|(pair, &class)| NeighborKey {
                class,
                fourcc: pair.fourcc,
                modifier: pair.modifier,
            })
            .collect();
        let neighbors = neighbors(&keys);
        Self {
            pairs,
            own_sizes,
            own_ranges,
            classes,
            neighbors,
        }
    }

    pub fn extra_sizes(&self, own: &[Evaluation]) -> Vec<PairSize> {
        assert_eq!(
            own.len(),
            self.own_sizes.len(),
            "the evaluations do not match the plan"
        );
        let vector_sizes = |pair: usize| {
            let range = self.own_ranges[pair].clone();
            self.own_sizes[range.clone()]
                .iter()
                .zip(&own[range])
                .filter(|(_, evaluation)| matches!(evaluation, Evaluation::Vector(_)))
                .map(|(own, _)| own.size)
        };
        let pair_sizes = |pair: usize| {
            let mut sizes: BTreeSet<Size> = self.neighbors[pair]
                .iter()
                .flat_map(|&neighbor| vector_sizes(neighbor))
                .collect();
            for size in self.pairs[pair].sizes {
                sizes.remove(size);
            }
            sizes
        };
        (0..self.pairs.len())
            .into_par_iter()
            .flat_map_iter(|pair| {
                pair_sizes(pair)
                    .into_iter()
                    .map(move |size| PairSize { pair, size })
            })
            .collect()
    }
}

pub struct Selected {
    pub retained: Vec<PairSize>,
    pub computed: usize,
    pub rejected: usize,
    pub cover: usize,
    pub witnesses: usize,
    pub final_pass: usize,
}

struct LayoutIds {
    own: Vec<Option<u32>>,
    extra: Vec<Option<u32>>,
}

fn layout_ids(own: &[Evaluation], extra: &[Evaluation]) -> LayoutIds {
    let mut layouts: Vec<Layout> = own
        .par_iter()
        .chain(extra.par_iter())
        .filter_map(|evaluation| match evaluation {
            Evaluation::Vector(layout) => Some(*layout),
            Evaluation::Rejected => None,
        })
        .collect();
    layouts.par_sort_unstable();
    layouts.dedup();
    let id_of = |evaluation: &Evaluation| match evaluation {
        Evaluation::Vector(layout) => {
            let id = layouts
                .binary_search(layout)
                .expect("every layout was collected");
            Some(id as u32)
        }
        Evaluation::Rejected => None,
    };
    LayoutIds {
        own: own.par_iter().map(id_of).collect(),
        extra: extra.par_iter().map(id_of).collect(),
    }
}

struct Evaluated {
    size: Size,
    layout: Option<u32>,
}

fn evaluated_tables(
    plan: &Plan<'_>,
    extra_sizes: &[PairSize],
    ids: &LayoutIds,
) -> Vec<Vec<Evaluated>> {
    let mut extra_ranges = vec![0..0; plan.pairs.len()];
    for run in pair_runs(extra_sizes) {
        extra_ranges[run.pair] = run.range;
    }
    let table_of = |pair: usize| {
        let own = plan.own_ranges[pair].clone();
        let extra = extra_ranges[pair].clone();
        let mut table: Vec<Evaluated> = plan.own_sizes[own.clone()]
            .iter()
            .zip(&ids.own[own])
            .chain(extra_sizes[extra.clone()].iter().zip(&ids.extra[extra]))
            .map(|(pair_size, &layout)| Evaluated {
                size: pair_size.size,
                layout,
            })
            .collect();
        table.sort_unstable_by_key(|evaluated| evaluated.size);
        table
    };
    (0..plan.pairs.len())
        .into_par_iter()
        .map(table_of)
        .collect()
}

fn own_vectors(plan: &Plan<'_>, ids: &LayoutIds) -> Vec<Pair> {
    let vectors_of = |pair: usize| {
        let range = plan.own_ranges[pair].clone();
        let vectors = plan.own_sizes[range.clone()]
            .iter()
            .zip(&ids.own[range])
            .filter_map(|(pair_size, &layout)| {
                Some(Vector {
                    size: pair_size.size,
                    layout: layout?,
                })
            })
            .collect();
        Pair {
            class: plan.classes[pair],
            vectors,
        }
    };
    (0..plan.pairs.len())
        .into_par_iter()
        .map(vectors_of)
        .collect()
}

pub fn select_vectors(
    plan: &Plan<'_>,
    own: &[Evaluation],
    extra_sizes: &[PairSize],
    extra: &[Evaluation],
) -> Selected {
    assert_eq!(
        own.len(),
        plan.own_sizes.len(),
        "the evaluations do not match the plan"
    );
    assert_eq!(
        extra.len(),
        extra_sizes.len(),
        "the extra evaluations do not match the extra sizes"
    );
    let ids = layout_ids(own, extra);
    let tables = evaluated_tables(plan, extra_sizes, &ids);
    let pairs = own_vectors(plan, &ids);
    let rejected = own
        .par_iter()
        .filter(|evaluation| matches!(evaluation, Evaluation::Rejected))
        .count();
    let computed = pairs.iter().map(|pair| pair.vectors.len()).sum();
    let rule = |pair: usize, size: Size| {
        let table = &tables[pair];
        let position = table
            .binary_search_by_key(&size, |evaluated| evaluated.size)
            .expect("the selection asks only for evaluated sizes");
        table[position].layout
    };
    let Selection {
        retained,
        cover,
        witnesses,
        final_pass,
    } = select(&pairs, &plan.neighbors, &rule);
    let retained = retained
        .iter()
        .enumerate()
        .flat_map(|(index, kept)| {
            let pairs = &pairs;
            kept.iter().map(move |&vector| PairSize {
                pair: plan.pairs[index].index,
                size: pairs[index].vectors[vector].size,
            })
        })
        .collect();
    Selected {
        retained,
        computed,
        rejected,
        cover,
        witnesses,
        final_pass,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    Gbm,
    Vulkan,
}

impl Source {
    fn name(self) -> &'static str {
        match self {
            Source::Gbm => "gbm",
            Source::Vulkan => "vulkan",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recorded {
    pub file: String,
    pub source: Source,
    pub device: vectors::Device,
    pub case: vectors::Case,
}

impl Recorded {
    pub fn to_text(&self) -> String {
        let mut text = format!("record {} {}\n", self.file, self.source.name());
        vectors::write_compact_device(&mut text, &self.device, std::slice::from_ref(&self.case));
        text
    }

    pub fn parse(text: &str) -> Recorded {
        let (header, cases) = text.split_once('\n').expect("a record has a header line");
        let mut fields = header.split(' ');
        assert_eq!(fields.next(), Some("record"), "invalid record {header:?}");
        let file = fields.next().expect("a record names its file").to_string();
        let source = match fields.next() {
            Some("gbm") => Source::Gbm,
            Some("vulkan") => Source::Vulkan,
            source => panic!("invalid record source {source:?}"),
        };
        let mut devices = vectors::parse(cases)
            .unwrap_or_else(|error| panic!("invalid record for {file}: {error}"));
        let device = devices.pop().expect("a record has a device");
        assert!(devices.is_empty(), "a record has one device");
        let [case] = <[vectors::Case; 1]>::try_from(device.cases)
            .unwrap_or_else(|cases| panic!("a record has {} cases", cases.len()));
        Recorded {
            file,
            source,
            device: device.device,
            case,
        }
    }

    fn file_name(&self) -> String {
        match self.source {
            Source::Gbm => format!("{}{FILE_SUFFIX}", self.file),
            Source::Vulkan => format!("{}{VULKAN_SUFFIX}{FILE_SUFFIX}", self.file),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DeviceKey {
    driver: String,
    fake: String,
    render: bool,
}

impl DeviceKey {
    fn new(device: &vectors::Device) -> Self {
        Self {
            driver: device.driver.clone(),
            fake: device.fake.clone(),
            render: matches!(device.node, fake_kernel::Node::Render),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct RequestKey {
    fourcc: u32,
    modifier: u64,
    rejected: Option<u64>,
    size: Size,
    scanout: bool,
}

impl RequestKey {
    fn new(case: &vectors::Case) -> Self {
        Self {
            fourcc: case.fourcc,
            modifier: case.modifier,
            rejected: case.rejected,
            size: Size {
                width: case.width,
                height: case.height,
            },
            scanout: case.scanout,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct GbmKey {
    file: String,
    device: DeviceKey,
    request: RequestKey,
}

impl GbmKey {
    fn new(record: &Recorded) -> Self {
        Self {
            file: record.file.clone(),
            device: DeviceKey::new(&record.device),
            request: RequestKey::new(&record.case),
        }
    }
}

pub fn without_gbm_duplicates(recorded: Vec<Recorded>) -> Vec<Recorded> {
    let (mut gbm, vulkan): (Vec<Recorded>, Vec<Recorded>) = recorded
        .into_iter()
        .partition(|record| record.source == Source::Gbm);
    let unflagged: HashSet<GbmKey> = gbm
        .iter()
        .filter(|record| record.case.flags == vectors::Flags::default())
        .map(GbmKey::new)
        .collect();
    gbm.extend(
        vulkan
            .into_iter()
            .filter(|record| !unflagged.contains(&GbmKey::new(record))),
    );
    gbm
}

struct DeviceCases {
    device: vectors::Device,
    cases: Vec<vectors::Case>,
}

fn same_request(a: &vectors::Case, b: &vectors::Case) -> bool {
    RequestKey::new(a) == RequestKey::new(b) && a.flags == b.flags
}

fn add_case(
    file: &str,
    device: &mut DeviceCases,
    positions: &mut HashMap<RequestKey, Vec<usize>>,
    case: vectors::Case,
) {
    let same = positions.entry(RequestKey::new(&case)).or_default();
    let existing = same
        .iter()
        .map(|&position| &device.cases[position])
        .find(|existing| same_request(existing, &case));
    match existing {
        Some(existing) => assert!(
            *existing == case,
            "{file}: {} records one request with two results:\n{existing:?}\n{case:?}",
            device.device.fake
        ),
        None => {
            same.push(device.cases.len());
            device.cases.push(case);
        }
    }
}

fn group_records(records: Vec<Recorded>) -> BTreeMap<String, BTreeMap<DeviceKey, DeviceCases>> {
    let mut files: BTreeMap<String, BTreeMap<DeviceKey, DeviceCases>> = BTreeMap::new();
    let mut positions: HashMap<(String, DeviceKey), HashMap<RequestKey, Vec<usize>>> =
        HashMap::new();
    for record in records {
        let file = record.file_name();
        let key = DeviceKey::new(&record.device);
        let device = files
            .entry(file.clone())
            .or_default()
            .entry(key.clone())
            .or_insert_with(|| DeviceCases {
                device: record.device,
                cases: Vec::new(),
            });
        let positions = positions.entry((file.clone(), key)).or_default();
        add_case(&file, device, positions, record.case);
    }
    files
}

pub fn write_zst(path: &Path, text: &str, threads: usize) {
    let create = File::create(path);
    let file = create.unwrap_or_else(|error| panic!("cannot create {}: {error}", path.display()));
    let written = (|| {
        let mut encoder = zstd::Encoder::new(BufWriter::new(file), ZSTD_LEVEL)?;
        if threads > 1 {
            encoder.multithread(threads as u32)?;
        }
        encoder.write_all(text.as_bytes())?;
        encoder.finish()?.flush()
    })();
    written.unwrap_or_else(|error| panic!("cannot write {}: {error}", path.display()));
}

pub fn write_recorded(directory: &Path, records: Vec<Recorded>, threads: usize) -> usize {
    let files = group_records(records);
    files
        .par_iter()
        .map(|(file, devices)| {
            let mut text = String::new();
            let mut written = 0;
            for device in devices.values() {
                vectors::write_compact_device(&mut text, &device.device, &device.cases);
                written += device.cases.len();
            }
            write_zst(&directory.join(file), &text, threads);
            written
        })
        .sum()
}
