use super::*;
use std::sync::atomic::AtomicU32;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct TestJob {
    words: JobWords,
}

impl Job for TestJob {
    fn encode(self) -> JobWords {
        self.words
    }

    fn decode(words: JobWords) -> Self {
        Self { words }
    }
}

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new() -> Self {
        static NEXT: AtomicU32 = AtomicU32::new(0);
        let name = format!(
            "distribute-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        );
        let path = Path::new(env!("OUT_DIR")).join(name);
        std::fs::create_dir(&path).unwrap();
        Self { path }
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path).unwrap();
    }
}

fn jobs(count: u32) -> Vec<TestJob> {
    (0..count)
        .map(|job| TestJob {
            words: [job, job + 1, job + 2, job + 3, job + 4, job + 5],
        })
        .collect()
}

fn vector(job: usize) -> Evaluation {
    let planes = (0..job % (MAX_PLANES + 1)).map(|plane| PlaneLayout {
        offset: plane as u64,
        stride: job as u64,
        size: 1 << 40,
    });
    Evaluation::Vector(planes.collect())
}

fn evaluation(job: usize) -> Evaluation {
    match job % 7 {
        0 => Evaluation::Rejected,
        _ => vector(job),
    }
}

#[test]
fn jobs_and_results_cross_the_work_file() {
    let directory = TestDirectory::new();
    let jobs = jobs(100);
    let published = publish(&directory.path, 0, &jobs);
    let opened = next_phase(&directory.path, 0).unwrap();
    assert_eq!(opened.count(), jobs.len());
    let mut seen = Vec::new();
    work(
        &directory.path,
        0,
        0,
        &opened,
        |index, job: TestJob, output| {
            seen.push(job);
            output.emit(index, &format!("job {index}\n"));
            Some(evaluation(index))
        },
    );
    assert_eq!(seen, jobs);
    wait_done(&directory.path, 0, 1, &mut || {});
    let expected: Vec<Evaluation> = (0..jobs.len()).map(evaluation).collect();
    assert_eq!(published.evaluations(), expected);
    let blocks = blocks(&directory.path, 0, 1);
    assert_eq!(blocks.len(), jobs.len());
    for (index, block) in blocks.iter().enumerate() {
        assert_eq!(block.job, index);
        assert_eq!(block.text, format!("job {index}\n"));
    }
}

#[test]
fn two_workers_share_the_jobs() {
    let directory = TestDirectory::new();
    let jobs = jobs(50);
    let published = publish(&directory.path, 3, &jobs);
    let first = next_phase(&directory.path, 3).unwrap();
    let second = next_phase(&directory.path, 3).unwrap();
    let mut ran = vec![0; jobs.len()];
    let start = first.next().fetch_add(CHUNK, Ordering::SeqCst);
    for (index, ran) in ran.iter_mut().enumerate().skip(start).take(CHUNK) {
        first.set_result(index, evaluation(index));
        *ran += 1;
    }
    work(&directory.path, 3, 1, &second, |index, _: TestJob, _| {
        ran[index] += 1;
        Some(evaluation(index))
    });
    assert_eq!(ran, vec![1; jobs.len()]);
    let expected: Vec<Evaluation> = (0..jobs.len()).map(evaluation).collect();
    assert_eq!(published.evaluations(), expected);
}

#[test]
#[should_panic(expected = "no worker evaluated job 1")]
fn an_unwritten_result_is_not_an_evaluation() {
    let directory = TestDirectory::new();
    let published = publish(&directory.path, 0, &jobs(2));
    published.set_result(0, Evaluation::Rejected);
    published.evaluations();
}

#[test]
#[should_panic(expected = "cannot hold 3 jobs")]
fn a_truncated_work_file_is_refused() {
    let directory = TestDirectory::new();
    let published = publish(&directory.path, 0, &jobs(3));
    let file = OpenOptions::new()
        .write(true)
        .open(jobs_path(&directory.path, 0))
        .unwrap();
    file.set_len(published.layout.len as u64 - 1).unwrap();
    let _ = Mapping::open(&jobs_path(&directory.path, 0));
}

#[test]
fn no_phase_follows_the_exit() {
    let directory = TestDirectory::new();
    publish_exit(&directory.path);
    assert!(next_phase(&directory.path, 0).is_none());
}

#[test]
fn the_results_are_aligned_behind_the_jobs() {
    for count in 0..10 {
        let layout = file_layout(count).unwrap();
        assert!(layout.results_offset >= JOBS_OFFSET + count * size_of::<JobWords>());
        assert_eq!(layout.results_offset % align_of::<RawEvaluation>(), 0);
        assert_eq!(
            layout.len,
            layout.results_offset + count * size_of::<RawEvaluation>()
        );
    }
    assert!(file_layout(usize::MAX).is_none());
}

#[test]
fn an_empty_slot_decodes_to_nothing() {
    let empty = RawEvaluation {
        tag: TAG_EMPTY,
        len: 0,
        planes: [RawPlane {
            offset: 0,
            stride: 0,
            size: 0,
        }; MAX_PLANES],
    };
    assert_eq!(empty.decode(), None);
    for job in 0..20 {
        let evaluation = evaluation(job);
        assert_eq!(RawEvaluation::from(evaluation).decode(), Some(evaluation));
    }
}

#[test]
fn blocks_are_split_at_their_headers() {
    let blocks = parse_blocks("@@ 7\nfirst\nsecond\n@@ 2\n@@ 9\nthird\n");
    let jobs: Vec<usize> = blocks.iter().map(|block| block.job).collect();
    let texts: Vec<&str> = blocks.iter().map(|block| block.text.as_str()).collect();
    assert_eq!(jobs, [7, 2, 9]);
    assert_eq!(texts, ["first\nsecond\n", "", "third\n"]);
}

#[test]
#[should_panic(expected = "text before the first block")]
fn text_without_a_block_is_refused() {
    parse_blocks("first\n");
}

#[test]
fn a_block_writer_prefixes_the_job() {
    let mut output = BlockWriter { writer: Vec::new() };
    output.emit(4, "text\n");
    assert_eq!(output.writer, b"@@ 4\ntext\n");
}
