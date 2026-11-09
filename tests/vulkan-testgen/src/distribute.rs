use crate::generate::Evaluation;
use crate::generate::Layout;
use crate::generate::MAX_PLANES;
use crate::generate::PlaneLayout;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::atomic::fence;
use std::time::Duration;
use std::time::Instant;

#[cfg(test)]
mod tests;

pub const TIMEOUT: Duration = Duration::from_secs(600);
pub const JOB_WORDS: usize = 6;

const CHUNK: usize = 16;
const BLOCK_PREFIX: &str = "@@ ";
const POLL_INTERVAL: Duration = Duration::from_millis(1);

pub type JobWords = [u32; JOB_WORDS];

pub trait Job: Copy {
    fn encode(self) -> JobWords;
    fn decode(words: JobWords) -> Self;
}

const TAG_EMPTY: u64 = 0;
const TAG_VECTOR: u64 = 1;
const TAG_REJECTED: u64 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
struct RawPlane {
    offset: u64,
    stride: u64,
    size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RawEvaluation {
    tag: u64,
    len: u64,
    planes: [RawPlane; MAX_PLANES],
}

impl From<Evaluation> for RawEvaluation {
    fn from(evaluation: Evaluation) -> Self {
        let mut planes = [RawPlane {
            offset: 0,
            stride: 0,
            size: 0,
        }; MAX_PLANES];
        match evaluation {
            Evaluation::Vector(layout) => {
                for (slot, plane) in planes.iter_mut().zip(layout.planes()) {
                    *slot = RawPlane {
                        offset: plane.offset,
                        stride: plane.stride,
                        size: plane.size,
                    };
                }
                Self {
                    tag: TAG_VECTOR,
                    len: layout.planes().len() as u64,
                    planes,
                }
            }
            Evaluation::Rejected => Self {
                tag: TAG_REJECTED,
                len: 0,
                planes,
            },
        }
    }
}

impl RawEvaluation {
    fn decode(self) -> Option<Evaluation> {
        match self.tag {
            TAG_EMPTY => None,
            TAG_VECTOR => {
                let len = self.len as usize;
                assert!(len <= MAX_PLANES, "an evaluation has {len} planes");
                let planes = self.planes[..len].iter().map(|plane| PlaneLayout {
                    offset: plane.offset,
                    stride: plane.stride,
                    size: plane.size,
                });
                Some(Evaluation::Vector(planes.collect::<Layout>()))
            }
            TAG_REJECTED => Some(Evaluation::Rejected),
            tag => panic!("invalid evaluation tag {tag}"),
        }
    }
}

#[repr(C)]
struct Header {
    next: AtomicUsize,
    count: usize,
}

const JOBS_OFFSET: usize = size_of::<Header>();

const _: () = assert!(JOBS_OFFSET.is_multiple_of(align_of::<JobWords>()));
const _: () = assert!(std::mem::offset_of!(Header, next) == 0);
const _: () = assert!(std::mem::offset_of!(Header, count) == size_of::<usize>());

struct FileLayout {
    results_offset: usize,
    len: usize,
}

fn file_layout(count: usize) -> Option<FileLayout> {
    let jobs_end = count
        .checked_mul(size_of::<JobWords>())?
        .checked_add(JOBS_OFFSET)?;
    let results_offset = jobs_end.checked_next_multiple_of(align_of::<RawEvaluation>())?;
    let len = count
        .checked_mul(size_of::<RawEvaluation>())?
        .checked_add(results_offset)?;
    Some(FileLayout {
        results_offset,
        len,
    })
}

pub struct Mapping {
    ptr: *mut u8,
    count: usize,
    layout: FileLayout,
}

impl Mapping {
    fn create(path: &Path, jobs: &[JobWords]) -> io::Result<Self> {
        let count = jobs.len();
        let layout = file_layout(count).expect("the jobs do not fit into the address space");
        let mut file = OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
            .mode(0o600)
            .open(path)?;
        {
            let mut writer = BufWriter::new(&mut file);
            writer.write_all(&0usize.to_ne_bytes())?;
            writer.write_all(&count.to_ne_bytes())?;
            for job in jobs {
                for word in job {
                    writer.write_all(&word.to_ne_bytes())?;
                }
            }
            writer.flush()?;
        }
        file.set_len(layout.len as u64)?;
        Self::map(&file, count, layout)
    }

    fn open(path: &Path) -> io::Result<Self> {
        let mut file = OpenOptions::new().read(true).write(true).open(path)?;
        let mut header = [0u8; size_of::<Header>()];
        file.read_exact(&mut header)?;
        let mut count = [0u8; size_of::<usize>()];
        count.copy_from_slice(&header[std::mem::offset_of!(Header, count)..]);
        let count = usize::from_ne_bytes(count);
        let len = file.metadata()?.len();
        let layout = file_layout(count)
            .filter(|layout| layout.len as u64 == len)
            .unwrap_or_else(|| panic!("a work file of {len} bytes cannot hold {count} jobs"));
        Self::map(&file, count, layout)
    }

    fn map(file: &File, count: usize, layout: FileLayout) -> io::Result<Self> {
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                layout.len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                file.as_raw_fd(),
                0,
            )
        };
        if ptr == libc::MAP_FAILED {
            return Err(io::Error::last_os_error());
        }
        Ok(Self {
            ptr: ptr.cast(),
            count,
            layout,
        })
    }

    fn next(&self) -> &AtomicUsize {
        unsafe { &*self.ptr.cast::<AtomicUsize>() }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    fn job(&self, index: usize) -> JobWords {
        assert!(index < self.count, "job {index} is out of bounds");
        let offset = JOBS_OFFSET + index * size_of::<JobWords>();
        let job = unsafe { self.ptr.add(offset) }.cast::<JobWords>();
        unsafe { job.read() }
    }

    fn result_ptr(&self, index: usize) -> *mut RawEvaluation {
        assert!(index < self.count, "result {index} is out of bounds");
        let offset = self.layout.results_offset + index * size_of::<RawEvaluation>();
        unsafe { self.ptr.add(offset).cast() }
    }

    fn set_result(&self, index: usize, evaluation: Evaluation) {
        unsafe {
            self.result_ptr(index).write(evaluation.into());
        }
    }

    pub fn evaluations(&self) -> Vec<Evaluation> {
        let raw: Vec<RawEvaluation> = (0..self.count)
            .map(|index| unsafe { self.result_ptr(index).read() })
            .collect();
        raw.into_par_iter()
            .enumerate()
            .map(|(index, raw)| {
                raw.decode()
                    .unwrap_or_else(|| panic!("no worker evaluated job {index}"))
            })
            .collect()
    }
}

impl Drop for Mapping {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr.cast(), self.layout.len);
        }
    }
}

fn jobs_path(directory: &Path, phase: usize) -> PathBuf {
    directory.join(format!("phase-{phase}.bin"))
}

fn ready_path(directory: &Path, phase: usize) -> PathBuf {
    directory.join(format!("phase-{phase}.ready"))
}

fn exit_path(directory: &Path) -> PathBuf {
    directory.join("exit")
}

fn done_path(directory: &Path, phase: usize, rank: usize) -> PathBuf {
    directory.join(format!("done-{phase}-{rank}"))
}

fn output_path(directory: &Path, phase: usize, rank: usize) -> PathBuf {
    directory.join(format!("output-{phase}-{rank}.txt"))
}

pub fn publish<J: Job + Sync>(directory: &Path, phase: usize, jobs: &[J]) -> Mapping {
    let words: Vec<JobWords> = jobs.par_iter().map(|job| job.encode()).collect();
    let mapping =
        Mapping::create(&jobs_path(directory, phase), &words).expect("cannot create jobs");
    fence(Ordering::SeqCst);
    File::create(ready_path(directory, phase)).expect("cannot publish jobs");
    mapping
}

pub fn publish_exit(directory: &Path) {
    File::create(exit_path(directory)).expect("cannot publish the exit");
}

pub fn wait_done(directory: &Path, phase: usize, worker_count: usize, alive: &mut dyn FnMut()) {
    let started = Instant::now();
    for rank in 0..worker_count {
        let path = done_path(directory, phase, rank);
        while !path.exists() {
            alive();
            assert!(
                started.elapsed() < TIMEOUT,
                "timed out waiting for worker {rank} in phase {phase}"
            );
            std::thread::sleep(POLL_INTERVAL);
        }
    }
    fence(Ordering::SeqCst);
}

pub struct Block {
    pub job: usize,
    pub text: String,
}

fn parse_blocks(text: &str) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    for line in text.split_inclusive('\n') {
        match line.strip_prefix(BLOCK_PREFIX) {
            Some(job) => blocks.push(Block {
                job: job.trim_end().parse().expect("invalid block header"),
                text: String::new(),
            }),
            None => blocks
                .last_mut()
                .expect("text before the first block")
                .text
                .push_str(line),
        }
    }
    blocks
}

pub fn blocks(directory: &Path, phase: usize, worker_count: usize) -> Vec<Block> {
    let per_rank: Vec<Vec<Block>> = (0..worker_count)
        .into_par_iter()
        .map(|rank| {
            let text = std::fs::read_to_string(output_path(directory, phase, rank))
                .expect("cannot read worker output");
            parse_blocks(&text)
        })
        .collect();
    let mut blocks: Vec<Block> = per_rank.into_iter().flatten().collect();
    blocks.par_sort_by_key(|block| block.job);
    blocks
}

pub fn next_phase(directory: &Path, phase: usize) -> Option<Mapping> {
    let ready = ready_path(directory, phase);
    let exit = exit_path(directory);
    let started = Instant::now();
    loop {
        if ready.exists() {
            fence(Ordering::SeqCst);
            return Some(Mapping::open(&jobs_path(directory, phase)).expect("cannot open jobs"));
        }
        if exit.exists() {
            return None;
        }
        assert!(
            started.elapsed() < TIMEOUT,
            "timed out waiting for phase {phase}"
        );
        std::thread::sleep(POLL_INTERVAL);
    }
}

pub struct BlockWriter<W: Write> {
    writer: W,
}

impl<W: Write> BlockWriter<W> {
    pub fn emit(&mut self, job: usize, text: &str) {
        write!(self.writer, "{BLOCK_PREFIX}{job}\n{text}").expect("cannot write worker output");
    }
}

pub type WorkerOutput = BlockWriter<BufWriter<File>>;

pub fn work<J: Job>(
    directory: &Path,
    phase: usize,
    rank: usize,
    mapping: &Mapping,
    mut run: impl FnMut(usize, J, &mut WorkerOutput) -> Option<Evaluation>,
) {
    let file = File::create(output_path(directory, phase, rank)).expect("cannot create output");
    let mut output = BlockWriter {
        writer: BufWriter::new(file),
    };
    let count = mapping.count();
    loop {
        let start = mapping.next().fetch_add(CHUNK, Ordering::SeqCst);
        if start >= count {
            break;
        }
        for index in start..(start + CHUNK).min(count) {
            if let Some(evaluation) = run(index, J::decode(mapping.job(index)), &mut output) {
                mapping.set_result(index, evaluation);
            }
        }
    }
    output.writer.flush().expect("cannot write worker output");
    fence(Ordering::SeqCst);
    File::create(done_path(directory, phase, rank)).expect("cannot mark the phase done");
}
