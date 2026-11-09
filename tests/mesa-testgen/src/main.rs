mod amd;
mod driver_override;
mod gbm;
mod intel;
mod jobs;
mod nouveau;
mod plans;
mod pool;
mod simple;
mod sizes;
mod worker;

use clap::Parser;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

const SUPERVISE: &str = "supervise";
const VECTORS_DIRECTORY: &str = "generated-tests/vectors";

#[derive(Parser)]
struct Args {
    #[arg(long = SUPERVISE)]
    supervise: bool,
}

fn main() -> ExitCode {
    if fake_supervisor::client::is_worker() {
        run_worker();
        return ExitCode::SUCCESS;
    }
    let args = Args::parse();
    if args.supervise {
        let program = std::env::current_exe().expect("cannot find the mesa-testgen binary");
        let status = fake_supervisor::supervise(&program, &[]);
        std::process::exit(status.code().unwrap_or(1));
    }
    let tests = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the manifest directory has no parent");
    pool::run(&tests.join(VECTORS_DIRECTORY))
}

fn run_worker() {
    unsafe {
        driver_override::install();
    }
    let directory = PathBuf::from(std::env::var_os(pool::DIRECTORY).expect("no work directory"));
    let rank = std::env::var(pool::RANK)
        .expect("no worker rank")
        .parse()
        .expect("invalid worker rank");
    gbm::load();
    worker::run(&directory, rank);
}
