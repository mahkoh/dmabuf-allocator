use crate::amd;
use crate::driver_override;
use crate::gbm::GbmDevice;
use crate::gbm::Implicit;
use crate::intel;
use crate::jobs::Api;
use crate::jobs::DESCRIBE_PHASE;
use crate::jobs::Group;
use crate::jobs::Job;
use crate::nouveau;
use crate::plans::GbmPair;
use crate::simple;
use crate::simple::Modifiers;
use crate::simple::SimpleDriver;
use allocator_trace::vectors;
use allocator_trace::vectors::Flags;
use dmabuf_allocator_impl::format::get_format;
use fake_kernel::Node;
use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_LINEAR;
use std::collections::BTreeMap;
use std::io::Write;
use std::ops::Range;
use std::path::Path;
use vulkan_testgen::cases;
use vulkan_testgen::distribute;
use vulkan_testgen::distribute::WorkerOutput;
use vulkan_testgen::drm;
use vulkan_testgen::formats::ALL_FORMATS;
use vulkan_testgen::formats::fourcc_code;
use vulkan_testgen::generate::Evaluation;
use vulkan_testgen::generate::Recorded;
use vulkan_testgen::generate::Source;
use vulkan_testgen::sizes::Size;
use vulkan_testgen::vulkan;
use vulkan_testgen::vulkan::Described;
use vulkan_testgen::vulkan::DescribedPair;
use vulkan_testgen::vulkan::Driver;
use vulkan_testgen::vulkan::VulkanDevice;

pub const REGISTRY_FILE: &str = "registry.txt";
pub const DESCRIPTION_FILE: &str = "description.txt";
pub const IMPLICIT_FILE: &str = "amd_implicit";
const RANK_SUFFIX: &str = "_rank";

const COVERAGE_SIZES: [Size; 2] = [
    Size {
        width: 256,
        height: 256,
    },
    Size {
        width: 1920,
        height: 1080,
    },
];

const IMPLICIT_SIZES: [Size; 14] = [
    Size {
        width: 1,
        height: 1,
    },
    Size {
        width: 64,
        height: 2,
    },
    Size {
        width: 2,
        height: 64,
    },
    Size {
        width: 3,
        height: 3,
    },
    Size {
        width: 100,
        height: 100,
    },
    Size {
        width: 300,
        height: 200,
    },
    Size {
        width: 500,
        height: 300,
    },
    Size {
        width: 16,
        height: 16,
    },
    Size {
        width: 17,
        height: 17,
    },
    Size {
        width: 64,
        height: 64,
    },
    Size {
        width: 100,
        height: 37,
    },
    Size {
        width: 37,
        height: 100,
    },
    Size {
        width: 256,
        height: 256,
    },
    Size {
        width: 1920,
        height: 1080,
    },
];

#[derive(Default)]
struct GbmDevices {
    devices: BTreeMap<u32, GbmDevice>,
}

impl GbmDevices {
    fn get(&mut self, node: u32, open: impl FnOnce() -> GbmDevice) -> &GbmDevice {
        self.devices.entry(node).or_insert_with(open)
    }

    fn render(&mut self, node: u32) -> &GbmDevice {
        self.get(node, || GbmDevice::open(node, Node::Render))
    }
}

struct Records<'a> {
    job: usize,
    output: &'a mut WorkerOutput,
}

impl Records<'_> {
    fn emit(&mut self, file: &str, source: Source, device: &vectors::Device, case: vectors::Case) {
        let record = Recorded {
            file: file.to_string(),
            source,
            device: device.clone(),
            case,
        };
        self.output.emit(self.job, &record.to_text());
    }
}

struct RankRequest<'a> {
    file: &'a str,
    fourcc: u32,
    modifiers: Vec<u64>,
    scanout: &'a [bool],
    flags: Flags,
}

fn record_ranks(device: &GbmDevice, mut request: RankRequest<'_>, records: &mut Records<'_>) {
    let fourcc = request.fourcc;
    request.modifiers.sort_unstable();
    request.modifiers.dedup();
    request
        .modifiers
        .retain(|&modifier| device.modifier_supported(fourcc, modifier));
    for (index, &low) in request.modifiers.iter().enumerate() {
        for &high in &request.modifiers[index + 1..] {
            for size in COVERAGE_SIZES {
                for &scanout in request.scanout {
                    let case =
                        device.record_pair(fourcc, [low, high], size, scanout, request.flags);
                    if let Some(case) = case {
                        records.emit(request.file, Source::Gbm, device.description(), case);
                    }
                }
            }
        }
    }
}

type VulkanDevices<D> = BTreeMap<u32, Option<VulkanDevice<<D as Driver>::Device>>>;

fn vulkan_device<'a, D: Driver>(
    driver: &D,
    devices: &'a mut VulkanDevices<D>,
    node: u32,
) -> Option<&'a VulkanDevice<D::Device>> {
    devices
        .entry(node)
        .or_insert_with(|| vulkan::open_device(driver, node))
        .as_ref()
}

struct Latest<'a, D: Driver> {
    pair: &'a DescribedPair,
    device: &'a VulkanDevice<D::Device>,
}

struct GroupState<D: Driver> {
    group: Group,
    driver: D,
    vulkan: VulkanDevices<D>,
    variants: BTreeMap<u32, Vec<u32>>,
    described: Option<Described>,
}

impl<D: Driver> GroupState<D> {
    fn new(group: Group, driver: D) -> Self {
        Self {
            group,
            driver,
            vulkan: BTreeMap::new(),
            variants: BTreeMap::new(),
            described: None,
        }
    }

    fn described(&self) -> &Described {
        self.described.as_ref().expect("no description loaded")
    }

    fn pair(&self, pair: u32) -> &DescribedPair {
        &self.described().pairs[pair as usize]
    }

    fn host_modifiers(&self, fourcc: &str, node: u32) -> Vec<u64> {
        let pairs = self.described().pairs.iter();
        pairs
            .filter(|pair| pair.format().fourcc == fourcc && pair.nodes.contains(&node))
            .map(|pair| pair.modifier)
            .collect()
    }

    fn describe(&mut self, gbm: &mut GbmDevices, node: u32, records: &mut Records<'_>) {
        gbm.render(node);
        let Some(device) = vulkan_device(&self.driver, &mut self.vulkan, node) else {
            return;
        };
        let description = vulkan::describe(&self.driver, device);
        records
            .output
            .emit(records.job, &vulkan::write_description(&description));
    }

    fn latest_device(&mut self, gbm: &mut GbmDevices, pair: u32) -> Latest<'_, D> {
        let described = self.described.as_ref().expect("no description loaded");
        let pair = &described.pairs[pair as usize];
        let node = pair.latest.node;
        gbm.render(node);
        let device = vulkan_device(&self.driver, &mut self.vulkan, node)
            .unwrap_or_else(|| panic!("the described node {node} has no Vulkan device"));
        Latest { pair, device }
    }

    fn evaluate_vulkan(&mut self, gbm: &mut GbmDevices, pair: u32, size: Size) -> Evaluation {
        let latest = self.latest_device(gbm, pair);
        vulkan::evaluate_pair(latest.device, latest.pair, size)
    }

    fn evaluate_gbm(&mut self, gbm: &mut GbmDevices, pair: u32, size: Size) -> Evaluation {
        let exact = self.driver.exact_export_size();
        let gbm_pair = GbmPair::from_index(pair as usize);
        let pair = self.pair(gbm_pair.base as u32);
        let fourcc = fourcc_code(pair.format().fourcc);
        for &node in &pair.nodes {
            let device = gbm.render(node);
            let layout = device.layout(fourcc, pair.modifier, size, gbm_pair.scanout, exact);
            if let Some(layout) = layout {
                return Evaluation::Vector(layout);
            }
        }
        Evaluation::Rejected
    }

    fn record_vulkan(
        &mut self,
        gbm: &mut GbmDevices,
        pair: u32,
        size: Size,
        records: &mut Records<'_>,
    ) {
        let group = self.group;
        let file = self.driver.output_file(self.pair(pair).modifier);
        let Latest { pair, device } = self.latest_device(gbm, pair);
        let node = device.node;
        match vulkan::record_pair(device, pair, size) {
            Ok(case) => {
                let description = gbm.render(node).description();
                records.emit(file, Source::Vulkan, description, case);
            }
            Err(error) => {
                let line = format!(
                    "{}: no Vulkan record of {:?} {:#x} {}x{} on node {node}: {error}\n",
                    group.name(),
                    pair.format().fourcc,
                    pair.modifier,
                    size.width,
                    size.height
                );
                let _ = std::io::stderr().lock().write_all(line.as_bytes());
            }
        }
    }

    fn record_gbm(
        &mut self,
        gbm: &mut GbmDevices,
        pair: u32,
        size: Size,
        records: &mut Records<'_>,
    ) {
        let gbm_pair = GbmPair::from_index(pair as usize);
        let pair = self.pair(gbm_pair.base as u32);
        let fourcc = fourcc_code(pair.format().fourcc);
        let file = self.driver.output_file(pair.modifier);
        for &node in &pair.nodes {
            let device = gbm.render(node);
            let scanout = gbm_pair.scanout;
            let case = device.record(fourcc, pair.modifier, size, scanout, Flags::default());
            if let Some(case) = case {
                records.emit(file, Source::Gbm, device.description(), case);
                break;
            }
        }
    }

    fn coverage(&mut self, gbm: &mut GbmDevices, pair: u32, node: u32, records: &mut Records<'_>) {
        let described = self.described.as_ref().expect("no description loaded");
        let pair = &described.pairs[pair as usize];
        let fourcc = fourcc_code(pair.format().fourcc);
        let modifier = pair.modifier;
        let file = self.driver.output_file(modifier);
        let driver = &self.driver;
        let variants = self.variants.entry(node).or_insert_with(|| {
            let kernels = driver.variants(node);
            match kernels.is_empty() {
                true => Vec::new(),
                false => drm::register(kernels)
                    .map(|variant| variant as u32)
                    .collect(),
            }
        });
        for &node in std::iter::once(&node).chain(variants.iter()) {
            let device = gbm.render(node);
            for size in COVERAGE_SIZES {
                for scanout in [false, true] {
                    let case = device.record(fourcc, modifier, size, scanout, Flags::default());
                    if let Some(case) = case {
                        records.emit(file, Source::Gbm, device.description(), case);
                    }
                }
            }
        }
    }

    fn rank(&mut self, gbm: &mut GbmDevices, node: u32, format: u32, records: &mut Records<'_>) {
        let fourcc = ALL_FORMATS[format as usize].fourcc;
        let mut modifiers = self.host_modifiers(fourcc, node);
        modifiers.push(DRM_FORMAT_MOD_LINEAR);
        let file = format!("{}{RANK_SUFFIX}", self.group.name());
        let request = RankRequest {
            file: &file,
            fourcc: fourcc_code(fourcc),
            modifiers,
            scanout: &[false, true],
            flags: Flags::default(),
        };
        record_ranks(gbm.render(node), request, records);
    }

    fn run(
        &mut self,
        gbm: &mut GbmDevices,
        job: Job,
        records: &mut Records<'_>,
    ) -> Option<Evaluation> {
        match job {
            Job::Describe { node, .. } => self.describe(gbm, node, records),
            Job::GroupRank { node, format, .. } => self.rank(gbm, node, format, records),
            Job::Evaluate {
                backend,
                pair,
                size,
            } => {
                return Some(match backend.api {
                    Api::Vulkan => self.evaluate_vulkan(gbm, pair, size),
                    Api::Gbm => self.evaluate_gbm(gbm, pair, size),
                });
            }
            Job::Record {
                backend,
                pair,
                size,
            } => match backend.api {
                Api::Vulkan => self.record_vulkan(gbm, pair, size, records),
                Api::Gbm => self.record_gbm(gbm, pair, size, records),
            },
            Job::Coverage { pair, node, .. } => self.coverage(gbm, pair, node, records),
            Job::Implicit { .. } | Job::Simple { .. } | Job::SimpleRank { .. } => {
                unreachable!("{job:?} belongs to no group")
            }
        }
        None
    }
}

struct Worker {
    amd: GroupState<amd::Driver>,
    intel: GroupState<intel::Driver>,
    nouveau: GroupState<nouveau::Driver>,
    simple: Vec<Range<usize>>,
    gbm: GbmDevices,
}

impl Worker {
    fn new() -> Self {
        let amd = GroupState::new(Group::Amd, amd::Driver::new());
        let intel = GroupState::new(Group::Intel, intel::Driver::new());
        let nouveau = GroupState::new(Group::Nouveau, nouveau::Driver::new());
        let simple = simple::DRIVERS
            .iter()
            .map(|driver| drm::register((driver.kernels)()))
            .collect();
        Self {
            amd,
            intel,
            nouveau,
            simple,
            gbm: GbmDevices::default(),
        }
    }

    fn registry(&self) -> String {
        let mut text = String::new();
        let mut line = |name: &str, nodes: &Range<usize>| {
            text.push_str(&format!("{name} {} {}\n", nodes.start, nodes.end));
        };
        line(self.amd.group.name(), &self.amd.driver.nodes());
        line(self.intel.group.name(), &self.intel.driver.nodes());
        line(self.nouveau.group.name(), &self.nouveau.driver.nodes());
        for (driver, nodes) in simple::DRIVERS.iter().zip(&self.simple) {
            line(driver.name, nodes);
        }
        text
    }

    fn publish_registry(&self, directory: &Path) {
        let temporary = directory.join(format!("{REGISTRY_FILE}.tmp"));
        std::fs::write(&temporary, self.registry()).expect("cannot write the registry");
        std::fs::rename(temporary, directory.join(REGISTRY_FILE))
            .expect("cannot publish the registry");
    }

    fn load_description(&mut self, directory: &Path) {
        let text = std::fs::read_to_string(directory.join(DESCRIPTION_FILE))
            .expect("cannot read the description");
        let mut groups = split_groups(&text);
        let mut described =
            |group: Group| Some(vulkan::merge(groups.remove(&group).unwrap_or_default()));
        self.amd.described = described(self.amd.group);
        self.intel.described = described(self.intel.group);
        self.nouveau.described = described(self.nouveau.group);
    }

    fn simple_modifiers(
        &self,
        driver: &SimpleDriver,
        nodes: &Range<usize>,
        node: u32,
        fourcc: &str,
    ) -> Vec<u64> {
        let mut modifiers = match driver.modifiers {
            Modifiers::List(modifiers) => return modifiers.to_vec(),
            Modifiers::AmdHost { chip } => {
                let host = self.amd.driver.node_of(chip);
                let mut modifiers = self.amd.host_modifiers(fourcc, host);
                modifiers.push(DRM_FORMAT_MOD_LINEAR);
                modifiers
            }
            Modifiers::IntelHost { chips } => {
                let chip = chips[node as usize - nodes.start];
                let host = self.intel.driver.i915_node_of(chip);
                self.intel.host_modifiers(fourcc, host)
            }
        };
        modifiers.sort();
        modifiers.dedup();
        modifiers
    }

    fn simple_device<'a>(
        gbm: &'a mut GbmDevices,
        driver: &SimpleDriver,
        node: u32,
    ) -> &'a GbmDevice {
        gbm.get(node, || match driver.driver_override {
            Some(name) => unsafe {
                driver_override::with(name, || GbmDevice::open(node, driver.node))
            },
            None => GbmDevice::open(node, driver.node),
        })
    }

    fn simple(&mut self, driver: u32, node: u32, format: u32, records: &mut Records<'_>) {
        let nodes = &self.simple[driver as usize];
        let driver = &simple::DRIVERS[driver as usize];
        let fourcc = ALL_FORMATS[format as usize].fourcc;
        let modifiers = self.simple_modifiers(driver, nodes, node, fourcc);
        let device = Self::simple_device(&mut self.gbm, driver, node);
        driver.record(device, fourcc_code(fourcc), &modifiers, |case| {
            records.emit(driver.file, Source::Gbm, device.description(), case);
        });
    }

    fn simple_rank(&mut self, driver: u32, node: u32, format: u32, records: &mut Records<'_>) {
        let nodes = &self.simple[driver as usize];
        let driver = &simple::DRIVERS[driver as usize];
        let fourcc = ALL_FORMATS[format as usize].fourcc;
        let file = format!("{}{RANK_SUFFIX}", driver.file);
        let request = RankRequest {
            file: &file,
            fourcc: fourcc_code(fourcc),
            modifiers: self.simple_modifiers(driver, nodes, node, fourcc),
            scanout: driver.scanout,
            flags: cases::flags(driver.flags),
        };
        let device = Self::simple_device(&mut self.gbm, driver, node);
        record_ranks(device, request, records);
    }

    fn implicit(&mut self, node: u32, format: u32, records: &mut Records<'_>) {
        let device = self.gbm.render(node);
        let fourcc = fourcc_code(ALL_FORMATS[format as usize].fourcc);
        if !self.amd.driver.pre_gfx9(node) || get_format(fourcc).is_none() {
            return;
        }
        for size in IMPLICIT_SIZES {
            for linear in [false, true] {
                for scanout in [false, true] {
                    let implicit = Implicit { scanout, linear };
                    if let Some(case) = device.record_implicit(fourcc, size, implicit) {
                        records.emit(IMPLICIT_FILE, Source::Gbm, device.description(), case);
                    }
                }
            }
        }
    }

    fn run(&mut self, index: usize, job: Job, output: &mut WorkerOutput) -> Option<Evaluation> {
        let mut records = Records { job: index, output };
        let group = match job {
            Job::Implicit { node, format } => {
                self.implicit(node, format, &mut records);
                return None;
            }
            Job::Simple {
                driver,
                node,
                format,
            } => {
                self.simple(driver, node, format, &mut records);
                return None;
            }
            Job::SimpleRank {
                driver,
                node,
                format,
            } => {
                self.simple_rank(driver, node, format, &mut records);
                return None;
            }
            Job::Describe { group, .. }
            | Job::Coverage { group, .. }
            | Job::GroupRank { group, .. } => group,
            Job::Evaluate { backend, .. } | Job::Record { backend, .. } => backend.group,
        };
        let gbm = &mut self.gbm;
        match group {
            Group::Amd => self.amd.run(gbm, job, &mut records),
            Group::Intel => self.intel.run(gbm, job, &mut records),
            Group::Nouveau => self.nouveau.run(gbm, job, &mut records),
        }
    }
}

pub fn split_groups(text: &str) -> BTreeMap<Group, Vec<vulkan::DeviceDescription>> {
    let mut texts: BTreeMap<Group, String> = BTreeMap::new();
    let mut current = None;
    for line in text.lines() {
        if let Some(name) = line.strip_prefix("group ") {
            let group = Group::from_name(name)
                .unwrap_or_else(|| panic!("the description names the unknown group {name}"));
            current = Some(texts.entry(group).or_default());
            continue;
        }
        let text = current.as_mut().expect("description line before a group");
        text.push_str(line);
        text.push('\n');
    }
    texts
        .into_iter()
        .map(|(group, text)| (group, vulkan::parse_descriptions(&text)))
        .collect()
}

pub fn run(directory: &Path, rank: usize) {
    let mut worker = Worker::new();
    if rank == 0 {
        worker.publish_registry(directory);
    }
    for phase in 0.. {
        let Some(mapping) = distribute::next_phase(directory, phase) else {
            break;
        };
        if phase == DESCRIBE_PHASE + 1 {
            worker.load_description(directory);
        }
        distribute::work(directory, phase, rank, &mapping, |index, job, output| {
            worker.run(index, job, output)
        });
    }
}
