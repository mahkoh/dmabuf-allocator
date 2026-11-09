use crate::jobs::Api;
use crate::jobs::Backend;
use crate::jobs::DESCRIBE_PHASE;
use crate::jobs::Group;
use crate::jobs::Job;
use crate::plans::GbmPlan;
use crate::simple;
use crate::worker::DESCRIPTION_FILE;
use crate::worker::REGISTRY_FILE;
use crate::worker::split_groups;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::BTreeMap;
use std::fs::DirBuilder;
use std::ops::Range;
use std::os::unix::fs::DirBuilderExt;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use std::process::ExitCode;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use vulkan_testgen::distribute;
use vulkan_testgen::distribute::Block;
use vulkan_testgen::distribute::Mapping;
use vulkan_testgen::formats::ALL_FORMATS;
use vulkan_testgen::generate;
use vulkan_testgen::generate::Evaluation;
use vulkan_testgen::generate::PairSize;
use vulkan_testgen::generate::Plan;
use vulkan_testgen::generate::Recorded;
use vulkan_testgen::vulkan;
use vulkan_testgen::vulkan::Described;
use vulkan_testgen::vulkan::VulkanPlan;

#[cfg(test)]
mod tests;

pub const DIRECTORY: &str = "MESA_TESTGEN_WORK_DIR";
pub const RANK: &str = "MESA_TESTGEN_RANK";

const WORK_DIRECTORY_NAME: &str = "mesa-testgen";
const WORK_DIRECTORY_MODE: u32 = 0o700;
const POLL_INTERVAL: Duration = Duration::from_millis(1);
const WORKER_ENVIRONMENT: [Variable; 9] = [
    Variable {
        name: "RAYON_NUM_THREADS",
        value: "1",
    },
    Variable {
        name: "FAKE_SUPERVISOR_HANDLERS",
        value: "1",
    },
    Variable {
        name: "MESA_SHADER_CACHE_DISABLE",
        value: "true",
    },
    Variable {
        name: "GALLIUM_THREAD",
        value: "false",
    },
    Variable {
        name: "INTEL_FORCE_PROBE",
        value: "*",
    },
    Variable {
        name: "GBM_BACKENDS_PATH",
        value: env!("GBM_BACKENDS"),
    },
    Variable {
        name: "VK_DRIVER_FILES",
        value: env!("GBM_VK_DRIVER_FILES"),
    },
    Variable {
        name: "VK_LOADER_LAYERS_DISABLE",
        value: "~implicit~",
    },
    Variable {
        name: "ETNA_MESA_DEBUG",
        value: "no_ts",
    },
];

struct Variable {
    name: &'static str,
    value: &'static str,
}

struct WorkDirectory {
    path: PathBuf,
}

fn cache_directory() -> PathBuf {
    let home_cache = || Some(PathBuf::from(std::env::var_os("HOME")?).join(".cache"));
    std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(home_cache)
        .unwrap_or_else(std::env::temp_dir)
}

impl WorkDirectory {
    fn create() -> Self {
        Self::create_in(&cache_directory().join(WORK_DIRECTORY_NAME))
    }

    fn create_in(parent: &Path) -> Self {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|error| panic!("cannot create {}: {error}", parent.display()));
        let since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("the clock is before the epoch");
        let name = format!("{}-{}", std::process::id(), since_epoch.as_nanos());
        let path = parent.join(name);
        DirBuilder::new()
            .mode(WORK_DIRECTORY_MODE)
            .create(&path)
            .unwrap_or_else(|error| panic!("cannot create {}: {error}", path.display()));
        Self { path }
    }
}

impl Drop for WorkDirectory {
    fn drop(&mut self) {
        if let Err(error) = std::fs::remove_dir_all(&self.path) {
            eprintln!("cannot remove {}: {error}", self.path.display());
        }
    }
}

struct Children {
    children: Vec<Child>,
}

impl Children {
    fn check(&mut self) {
        for (rank, child) in self.children.iter_mut().enumerate() {
            if let Some(status) = child.try_wait().expect("waiting for a worker failed") {
                panic!("worker {rank} exited early: {status}");
            }
        }
    }

    fn wait(&mut self) -> bool {
        let mut succeeded = true;
        for (rank, mut child) in self.children.drain(..).enumerate() {
            let status = child.wait().expect("waiting for a worker failed");
            if !status.success() {
                eprintln!("worker {rank} failed: {status}");
                succeeded = false;
            }
        }
        succeeded
    }
}

impl Drop for Children {
    fn drop(&mut self) {
        for child in &mut self.children {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn single_cpu_sets() -> Vec<libc::cpu_set_t> {
    let mut allowed: libc::cpu_set_t = unsafe { std::mem::zeroed() };
    if unsafe { libc::sched_getaffinity(0, size_of::<libc::cpu_set_t>(), &mut allowed) != 0 } {
        panic!(
            "cannot read the CPU affinity: {}",
            std::io::Error::last_os_error()
        );
    }
    (0..libc::CPU_SETSIZE as usize)
        .filter(|&cpu| unsafe { libc::CPU_ISSET(cpu, &allowed) })
        .map(|cpu| {
            let mut set: libc::cpu_set_t = unsafe { std::mem::zeroed() };
            unsafe {
                libc::CPU_SET(cpu, &mut set);
            }
            set
        })
        .collect()
}

fn confine(root: libc::pid_t, cpu: &libc::cpu_set_t) -> std::io::Result<()> {
    if unsafe { libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL, 0, 0, 0) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    if unsafe { libc::getppid() } != root {
        unsafe {
            libc::_exit(1);
        }
    }
    let result = unsafe {
        libc::syscall(
            libc::SYS_sched_setaffinity,
            0 as libc::c_long,
            size_of::<libc::cpu_set_t>(),
            cpu as *const libc::cpu_set_t,
        )
    };
    if result != 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

fn spawn(directory: &Path, worker_count: usize) -> Children {
    let program = std::env::current_exe().expect("cannot find the mesa-testgen binary");
    let root = std::process::id() as libc::pid_t;
    let cpus = single_cpu_sets();
    let children = (0..worker_count)
        .map(|rank| {
            let cpu = cpus[rank];
            let mut command = Command::new(&program);
            command
                .arg(format!("--{}", crate::SUPERVISE))
                .env(DIRECTORY, directory)
                .env(RANK, rank.to_string());
            for variable in &WORKER_ENVIRONMENT {
                command.env(variable.name, variable.value);
            }
            unsafe {
                command.pre_exec(move || confine(root, &cpu));
            }
            command.spawn().expect("cannot start a worker")
        })
        .collect();
    Children { children }
}

struct Registry {
    nodes: BTreeMap<String, Range<u32>>,
}

impl Registry {
    fn parse(text: &str) -> Self {
        let nodes = text
            .lines()
            .map(|line| {
                let fields: Vec<&str> = line.split(' ').collect();
                let [name, start, end] = fields[..] else {
                    panic!("invalid registry line {line:?}");
                };
                let node = |field: &str| -> u32 {
                    field
                        .parse()
                        .unwrap_or_else(|_| panic!("invalid registry line {line:?}"))
                };
                (name.to_string(), node(start)..node(end))
            })
            .collect();
        Self { nodes }
    }

    fn nodes(&self, name: &str) -> Range<u32> {
        let nodes = self.nodes.get(name);
        nodes
            .unwrap_or_else(|| panic!("the registry has no {name} nodes"))
            .clone()
    }
}

fn read_registry(directory: &Path, children: &mut Children) -> Registry {
    let path = directory.join(REGISTRY_FILE);
    let started = Instant::now();
    while !path.exists() {
        children.check();
        assert!(
            started.elapsed() < distribute::TIMEOUT,
            "timed out waiting for the registry"
        );
        std::thread::sleep(POLL_INTERVAL);
    }
    let text = std::fs::read_to_string(&path).expect("cannot read the registry");
    Registry::parse(&text)
}

struct Completed {
    phase: usize,
    mapping: Mapping,
    blocks: Vec<Block>,
}

struct Phases<'a> {
    directory: &'a Path,
    worker_count: usize,
    next: usize,
}

impl Phases<'_> {
    fn run(&mut self, jobs: &[Job], children: &mut Children) -> Completed {
        let phase = self.next;
        self.next += 1;
        let started = Instant::now();
        let mapping = distribute::publish(self.directory, phase, jobs);
        distribute::wait_done(self.directory, phase, self.worker_count, &mut || {
            children.check();
        });
        let blocks = distribute::blocks(self.directory, phase, self.worker_count);
        println!(
            "phase {phase}: {} jobs in {:.1} s",
            jobs.len(),
            started.elapsed().as_secs_f64()
        );
        Completed {
            phase,
            mapping,
            blocks,
        }
    }
}

fn interleave<T: Copy>(lists: &[Vec<T>]) -> Vec<T> {
    let longest = lists.iter().map(Vec::len).max().unwrap_or(0);
    let mut order = Vec::new();
    for offset in 0..longest {
        for list in lists {
            if let Some(&item) = list.get(offset) {
                order.push(item);
            }
        }
    }
    order
}

fn pair_order(sizes: &[PairSize]) -> Vec<usize> {
    let runs: Vec<Vec<usize>> = generate::pair_runs(sizes)
        .into_iter()
        .map(|run| run.range.collect())
        .collect();
    interleave(&runs)
}

struct DescribedGroup {
    group: Group,
    nodes: Vec<u32>,
    described: Described,
}

fn describe(
    phases: &mut Phases<'_>,
    children: &mut Children,
    registry: &Registry,
) -> Vec<DescribedGroup> {
    let jobs: Vec<Job> = Group::ALL
        .into_iter()
        .flat_map(|group| {
            let nodes = registry.nodes(group.name());
            nodes.map(move |node| Job::Describe { group, node })
        })
        .collect();
    let completed = phases.run(&jobs, children);
    assert_eq!(
        completed.phase, DESCRIBE_PHASE,
        "the workers load the description after another phase"
    );
    let mut description = String::new();
    let mut nodes: BTreeMap<Group, Vec<u32>> = BTreeMap::new();
    for group in Group::ALL {
        description.push_str(&format!("group {}\n", group.name()));
        for block in &completed.blocks {
            let Job::Describe {
                group: described,
                node,
            } = jobs[block.job]
            else {
                unreachable!("the describe phase runs only describe jobs");
            };
            if described == group {
                description.push_str(&block.text);
                nodes.entry(group).or_default().push(node);
            }
        }
    }
    std::fs::write(phases.directory.join(DESCRIPTION_FILE), &description)
        .expect("cannot write the description");
    let mut groups = split_groups(&description);
    let described = Group::ALL.map(|group| DescribedGroup {
        group,
        nodes: nodes.remove(&group).unwrap_or_default(),
        described: vulkan::merge(groups.remove(&group).unwrap_or_default()),
    });
    described.into()
}

struct BackendPlan<'a> {
    backend: Backend,
    plan: Plan<'a>,
}

#[derive(Copy, Clone)]
struct Origin {
    plan: usize,
    position: usize,
}

struct EvaluationJobs {
    jobs: Vec<Job>,
    origins: Vec<Origin>,
}

fn evaluation_jobs(plans: &[BackendPlan<'_>], sizes: &[&[PairSize]]) -> EvaluationJobs {
    let per_plan: Vec<Vec<Origin>> = sizes
        .par_iter()
        .enumerate()
        .map(|(plan, sizes)| {
            let order = pair_order(sizes).into_iter();
            order.map(|position| Origin { plan, position }).collect()
        })
        .collect();
    let origins = interleave(&per_plan);
    let jobs = origins
        .par_iter()
        .map(|origin| {
            let plan = &plans[origin.plan];
            let PairSize { pair, size } = sizes[origin.plan][origin.position];
            Job::Evaluate {
                backend: plan.backend,
                pair: plan.plan.pairs[pair].index as u32,
                size,
            }
        })
        .collect();
    EvaluationJobs { jobs, origins }
}

fn scatter(
    evaluations: Vec<Evaluation>,
    origins: &[Origin],
    sizes: &[&[PairSize]],
) -> Vec<Vec<Evaluation>> {
    let mut scattered: Vec<Vec<Option<Evaluation>>> =
        sizes.iter().map(|sizes| vec![None; sizes.len()]).collect();
    for (evaluation, origin) in evaluations.into_iter().zip(origins) {
        scattered[origin.plan][origin.position] = Some(evaluation);
    }
    let complete = |plan: Vec<Option<Evaluation>>| {
        let evaluations = plan.into_iter();
        evaluations
            .map(|evaluation| evaluation.expect("every size was evaluated"))
            .collect()
    };
    scattered.into_iter().map(complete).collect()
}

fn evaluate(
    phases: &mut Phases<'_>,
    children: &mut Children,
    plans: &[BackendPlan<'_>],
    sizes: &[&[PairSize]],
) -> Vec<Vec<Evaluation>> {
    let EvaluationJobs { jobs, origins } = evaluation_jobs(plans, sizes);
    let completed = phases.run(&jobs, children);
    scatter(completed.mapping.evaluations(), &origins, sizes)
}

fn selected_jobs(
    phases: &mut Phases<'_>,
    children: &mut Children,
    plans: &[BackendPlan<'_>],
) -> Vec<Job> {
    let own_sizes: Vec<&[PairSize]> = plans.iter().map(|plan| &plan.plan.own_sizes[..]).collect();
    let own = evaluate(phases, children, plans, &own_sizes);
    let extra_sizes: Vec<Vec<PairSize>> = plans
        .iter()
        .zip(&own)
        .map(|(plan, own)| plan.plan.extra_sizes(own))
        .collect();
    let extra_sizes: Vec<&[PairSize]> = extra_sizes.iter().map(|sizes| &sizes[..]).collect();
    let extra = evaluate(phases, children, plans, &extra_sizes);
    let mut lists: Vec<Vec<Job>> = Vec::new();
    for (index, plan) in plans.iter().enumerate() {
        let selected =
            generate::select_vectors(&plan.plan, &own[index], extra_sizes[index], &extra[index]);
        println!(
            "computed {} vectors, {} rejected",
            selected.computed, selected.rejected
        );
        println!("selection: cover keeps {} vectors", selected.cover);
        println!("selection: witnesses raise this to {}", selected.witnesses);
        println!("selection: final pass leaves {}", selected.final_pass);
        let retained = selected.retained.into_iter();
        lists.push(
            retained
                .map(|retained| Job::Record {
                    backend: plan.backend,
                    pair: retained.pair as u32,
                    size: retained.size,
                })
                .collect(),
        );
    }
    interleave(&lists)
}

fn coverage_jobs(groups: &[DescribedGroup]) -> Vec<Job> {
    let mut jobs = Vec::new();
    for group in groups {
        for &node in &group.nodes {
            for (pair, info) in group.described.pairs.iter().enumerate() {
                if info.nodes.contains(&node) {
                    jobs.push(Job::Coverage {
                        group: group.group,
                        pair: pair as u32,
                        node,
                    });
                }
            }
        }
    }
    jobs
}

fn implicit_jobs(groups: &[DescribedGroup]) -> Vec<Job> {
    let amd = groups.iter().filter(|group| group.group == Group::Amd);
    let nodes = amd.flat_map(|group| &group.nodes);
    nodes
        .flat_map(|&node| {
            (0..ALL_FORMATS.len() as u32).map(move |format| Job::Implicit { node, format })
        })
        .collect()
}

fn simple_jobs(registry: &Registry) -> Vec<Job> {
    let mut jobs = Vec::new();
    for (driver, simple) in simple::DRIVERS.iter().enumerate() {
        for node in registry.nodes(simple.name) {
            for format in 0..ALL_FORMATS.len() as u32 {
                jobs.push(Job::Simple {
                    driver: driver as u32,
                    node,
                    format,
                });
            }
        }
    }
    jobs
}

fn rank_jobs(groups: &[DescribedGroup], registry: &Registry) -> Vec<Job> {
    let formats = 0..ALL_FORMATS.len() as u32;
    let mut jobs = Vec::new();
    for group in groups {
        for &node in &group.nodes {
            for format in formats.clone() {
                jobs.push(Job::GroupRank {
                    group: group.group,
                    node,
                    format,
                });
            }
        }
    }
    for (driver, simple) in simple::DRIVERS.iter().enumerate() {
        for node in registry.nodes(simple.name) {
            for format in formats.clone() {
                jobs.push(Job::SimpleRank {
                    driver: driver as u32,
                    node,
                    format,
                });
            }
        }
    }
    jobs
}

fn generate_vectors(
    vectors: &Path,
    directory: &Path,
    worker_count: usize,
    children: &mut Children,
) {
    let registry = read_registry(directory, children);
    let mut phases = Phases {
        directory,
        worker_count,
        next: 0,
    };
    let groups = describe(&mut phases, children, &registry);
    let vulkan_plans: Vec<VulkanPlan<'_>> = groups
        .iter()
        .map(|group| VulkanPlan {
            described: &group.described,
        })
        .collect();
    let gbm_plans: Vec<GbmPlan<'_>> = groups
        .iter()
        .map(|group| GbmPlan {
            described: &group.described,
        })
        .collect();
    let plans: Vec<BackendPlan<'_>> = Backend::ALL
        .into_iter()
        .map(|backend| {
            let group = groups.iter().position(|group| group.group == backend.group);
            let group = group.expect("every group is described");
            let plan = match backend.api {
                Api::Vulkan => Plan::new(&vulkan_plans[group]),
                Api::Gbm => Plan::new(&gbm_plans[group]),
            };
            BackendPlan { backend, plan }
        })
        .collect();
    let mut jobs = selected_jobs(&mut phases, children, &plans);
    let selected = jobs.len();
    jobs.extend(coverage_jobs(&groups));
    jobs.extend(implicit_jobs(&groups));
    jobs.extend(simple_jobs(&registry));
    jobs.extend(rank_jobs(&groups, &registry));
    let completed = phases.run(&jobs, children);
    let recorded: Vec<Recorded> = completed
        .blocks
        .par_iter()
        .map(|block| Recorded::parse(&block.text))
        .collect();
    println!(
        "{selected} selected record jobs, {} records",
        recorded.len()
    );
    let recorded = generate::without_gbm_duplicates(recorded);
    let written = generate::write_recorded(vectors, recorded, rayon::current_num_threads());
    println!("wrote {written} cases");
}

pub fn run(vectors: &Path) -> ExitCode {
    let started = Instant::now();
    let worker_count = std::thread::available_parallelism().map_or(1, |cpus| cpus.get());
    let directory = WorkDirectory::create();
    let mut children = spawn(&directory.path, worker_count);
    generate_vectors(vectors, &directory.path, worker_count, &mut children);
    distribute::publish_exit(&directory.path);
    let succeeded = children.wait();
    drop(directory);
    println!("done in {:.1} s", started.elapsed().as_secs_f64());
    match succeeded {
        true => ExitCode::SUCCESS,
        false => ExitCode::FAILURE,
    }
}
