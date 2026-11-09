use allocator_trace::Call;
use allocator_trace::Traced;
use allocator_trace::vectors;
use allocator_trace::vectors::Case;
use allocator_trace::vectors::Flag;
use allocator_trace::vectors::Plane;
use clap::Parser;
use clap::Subcommand;
use dmabuf_allocator_impl::allocator::Allocator;
use dmabuf_allocator_impl::allocator::Config;
use dmabuf_allocator_impl::format::get_format;
use error_reporter::Report;
use fake_device::DEVICE_FD;
use fake_device::FakeDevice;
use fake_kernel::amd::devices::GfxLevel;
use fake_kernel::uapi::amdgpu::AMDGPU_GEM_CREATE_EXPLICIT_SYNC;
use fake_kernel::uapi::amdgpu::AMDGPU_GEM_CREATE_NO_CPU_ACCESS;
use fake_kernel::uapi::amdgpu::AMDGPU_GEM_CREATE_VRAM_CLEARED;
use fake_kernel::uapi::virtgpu::VIRTGPU_BLOB_FLAG_USE_MAPPABLE;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[cfg(test)]
mod tests;

const AMDGPU_TILING_SCANOUT_SHIFT: u64 = 63;
const GFX12_DCC_NUMBER_TYPE_MASK: u64 = 0x7 << 5;
const GFX12_DCC_DATA_FORMAT_MASK: u64 = 0x3f << 8;
const GFX12_DCC_WRITE_COMPRESS_DISABLE: u64 = 1 << 14;
const PRINTED_FAILURES: usize = 10;

struct Device {
    id: String,
    node: fake_kernel::Node,
    gfx_level: Option<GfxLevel>,
}

impl Device {
    fn new(device: &vectors::Device) -> Device {
        let fake = fake_kernel::from_id(&device.fake);
        assert_eq!(
            fake.name(),
            device.driver,
            "driver {} and fake kernel {} differ",
            device.driver,
            device.fake
        );
        Device {
            id: fake.id(),
            node: device.node,
            gfx_level: fake.drm().and_then(|drm| drm.amd_gfx_level()),
        }
    }

    fn replay(&self) -> FakeDevice {
        FakeDevice::new(fake_kernel::from_id(&self.id), self.node)
    }

    fn gfx9_to_11(&self) -> bool {
        self.gfx_level
            .is_some_and(|level| (GfxLevel::Gfx9..=GfxLevel::Gfx11_7).contains(&level))
    }
}

struct Failure {
    reason: String,
    kinds: Vec<String>,
}

fn failure(reason: String) -> Failure {
    Failure {
        kinds: vec![reason.clone()],
        reason,
    }
}

fn categorize(expected: &[Call], got: &[Call]) -> Vec<String> {
    let mut categories = Vec::new();
    let mut paired = vec![false; got.len()];
    for expected in expected {
        let kind = expected.name();
        let unpaired = (0..got.len()).find(|&index| !paired[index] && got[index].name() == kind);
        let Some(index) = unpaired else {
            categories.push(format!("{kind}: missing"));
            continue;
        };
        paired[index] = true;
        let expected_fields = expected.fields();
        let got_fields = got[index].fields();
        for expected in &expected_fields {
            let key = expected.key;
            let got_value = got_fields
                .iter()
                .find(|got| got.key == key)
                .map_or("-", |got| got.value.as_str());
            if got_value == expected.value {
                continue;
            }
            let category = match key {
                "size" | "alignment" | "stride" => {
                    let parse = |v: &str| v.parse::<u64>().unwrap_or(0);
                    let relation = match parse(got_value) < parse(&expected.value) {
                        true => "smaller",
                        false => "larger",
                    };
                    format!("{kind}.{key}: {relation}")
                }
                _ => format!("{kind}.{key}: {} -> {got_value}", expected.value),
            };
            categories.push(category);
        }
        for got in &got_fields {
            if !expected_fields.iter().any(|field| field.key == got.key) {
                categories.push(format!("{kind}.{}: - -> {}", got.key, got.value));
            }
        }
    }
    for (index, got) in got.iter().enumerate() {
        if !paired[index] {
            categories.push(format!("{}: extra", got.name()));
        }
    }
    categories
}

fn foreign_handle(traced: &[Traced]) -> Option<String> {
    let exported = traced
        .iter()
        .find(|t| matches!(t.call, Call::PrimeHandleToFd { .. }))?
        .handle?;
    traced.iter().find_map(|t| match t.handle {
        Some(handle) if handle != exported => Some(format!(
            "{} on handle {handle}, exported {exported}",
            t.call
        )),
        _ => None,
    })
}

fn merge<T: Clone>(
    expected: &mut [Call],
    got: &[Call],
    field: impl Fn(&mut Call) -> Option<&mut T>,
    merge: impl Fn(&mut T, &T),
) {
    let mut got = got.to_vec();
    let got = got.iter_mut().filter_map(&field);
    let expected = expected.iter_mut().filter_map(&field);
    for (expected, got) in expected.zip(got) {
        merge(expected, got);
    }
}

fn adopt<T: Clone>(
    expected: &mut [Call],
    got: &[Call],
    field: impl Fn(&mut Call) -> Option<&mut T>,
) {
    merge(expected, got, field, T::clone_from);
}

fn ignore_domains(expected: &mut [Call], got: &[Call]) {
    adopt(expected, got, |call| match call {
        Call::AmdgpuGemCreate { domains, .. } => Some(domains),
        _ => None,
    });
}

fn ignore_cpu_access(expected: &mut [Call], got: &[Call]) {
    merge(
        expected,
        got,
        |call| match call {
            Call::AmdgpuGemCreate { flags, .. } | Call::VirtgpuAmdgpuGemNew { flags, .. } => {
                Some(flags)
            }
            _ => None,
        },
        |expected: &mut u64, got| {
            *expected = *expected & !AMDGPU_GEM_CREATE_NO_CPU_ACCESS
                | got & AMDGPU_GEM_CREATE_NO_CPU_ACCESS;
        },
    );
    merge(
        expected,
        got,
        |call| match call {
            Call::VirtgpuAmdgpuGemNew { blob_flags, .. } => Some(blob_flags),
            _ => None,
        },
        |expected: &mut u32, got| {
            *expected =
                *expected & !VIRTGPU_BLOB_FLAG_USE_MAPPABLE | got & VIRTGPU_BLOB_FLAG_USE_MAPPABLE;
        },
    );
}

fn ignore_umd_metadata(expected: &mut [Call], got: &[Call]) {
    adopt(expected, got, |call| match call {
        Call::AmdgpuGemMetadata { data, .. } => Some(data),
        _ => None,
    });
}

fn ignore_alignment(expected: &mut [Call], got: &[Call]) {
    adopt(expected, got, |call| match call {
        Call::AmdgpuGemCreate { alignment, .. } => Some(alignment),
        _ => None,
    });
}

fn ignore_size(expected: &mut [Call], got: &[Call]) {
    adopt(expected, got, |call| match call {
        Call::I915GemCreate { size }
        | Call::I915GemCreateExt { size, .. }
        | Call::XeGemCreate { size, .. } => Some(size),
        _ => None,
    });
}

fn ignore_regions(expected: &mut [Call], got: &[Call]) {
    adopt(expected, got, |call| match call {
        Call::I915GemCreateExt { regions, .. } => Some(regions),
        _ => None,
    });
}

fn without_set_domain(calls: Vec<Call>) -> Vec<Call> {
    calls
        .into_iter()
        .filter(|call| !matches!(call, Call::I915GemSetDomain { .. }))
        .collect()
}

fn without_set_tiling(calls: Vec<Call>) -> Vec<Call> {
    calls
        .into_iter()
        .filter(|call| !matches!(call, Call::I915GemSetTiling { .. }))
        .collect()
}

fn allow_write_compress_disable(expected: &mut [Call], got: &[Call]) {
    let dcc_format = GFX12_DCC_NUMBER_TYPE_MASK | GFX12_DCC_DATA_FORMAT_MASK;
    let Some(got) = got.iter().find_map(|call| match *call {
        Call::AmdgpuGemMetadata { tiling_info, .. } => Some(tiling_info),
        _ => None,
    }) else {
        return;
    };
    if got & GFX12_DCC_WRITE_COMPRESS_DISABLE == 0 || got & dcc_format != 0 {
        return;
    }
    for call in expected {
        if let Call::AmdgpuGemMetadata { tiling_info, .. } = call {
            *tiling_info = *tiling_info & !dcc_format | GFX12_DCC_WRITE_COMPRESS_DISABLE;
        }
    }
}

fn last_state_only(calls: Vec<Call>) -> Vec<Call> {
    let calls = last_only(calls, |call| matches!(call, Call::AmdgpuGemMetadata { .. }));
    let calls = last_only(calls, |call| matches!(call, Call::I915GemSetTiling { .. }));
    dedup_exports(calls)
}

fn dedup_exports(calls: Vec<Call>) -> Vec<Call> {
    let mut seen: Vec<Call> = Vec::new();
    calls
        .into_iter()
        .filter(|call| {
            if !matches!(call, Call::PrimeHandleToFd { .. }) {
                return true;
            }
            if seen.contains(call) {
                return false;
            }
            seen.push(call.clone());
            true
        })
        .collect()
}

fn last_only(calls: Vec<Call>, is_kind: impl Fn(&Call) -> bool) -> Vec<Call> {
    let last = calls.iter().rposition(&is_kind);
    calls
        .into_iter()
        .enumerate()
        .filter(|(index, call)| !is_kind(call) || Some(*index) == last)
        .map(|(_, call)| call)
        .collect()
}

fn amdgpu_create_flags(flags: u64, config: &Config) -> u64 {
    let mut flags = flags | AMDGPU_GEM_CREATE_VRAM_CLEARED;
    match config.implicit_sync {
        true => flags &= !AMDGPU_GEM_CREATE_EXPLICIT_SYNC,
        false => flags |= AMDGPU_GEM_CREATE_EXPLICIT_SYNC,
    }
    flags
}

fn amdgpu_scanout(tiling_info: u64, config: &Config) -> u64 {
    tiling_info & !(1 << AMDGPU_TILING_SCANOUT_SHIFT)
        | (config.scanout as u64) << AMDGPU_TILING_SCANOUT_SHIFT
}

fn expected_call(device: &Device, config: &Config, call: &Call) -> Call {
    let mut call = call.clone();
    match &mut call {
        Call::AmdgpuGemCreate { flags, .. } | Call::VirtgpuAmdgpuGemNew { flags, .. } => {
            *flags = amdgpu_create_flags(*flags, config);
        }
        Call::AmdgpuGemMetadata { tiling_info, .. }
        | Call::VirtgpuAmdgpuSetMetadata { tiling_info, .. }
            if device.gfx9_to_11() =>
        {
            *tiling_info = amdgpu_scanout(*tiling_info, config);
        }
        _ => {}
    }
    call
}

fn show_layout(planes: &[Plane]) -> String {
    let planes: Vec<String> = planes
        .iter()
        .map(|plane| format!("{} {}", plane.offset, plane.stride))
        .collect();
    planes.join(" | ")
}

fn show_calls(calls: &[Call]) -> String {
    calls.iter().map(|call| format!("\n    {call}")).collect()
}

fn check(device: &Device, case: &Case) -> Result<(), Failure> {
    let Some(format) = get_format(case.fourcc) else {
        return Err(failure("unknown fourcc".to_string()));
    };
    let replay = device.replay();
    let allocator = Allocator::with_kernel(replay.clone(), DEVICE_FD)
        .map_err(|error| failure(format!("Allocator::with_kernel: {}", Report::new(error))))?;
    let mut offered = vec![case.modifier];
    offered.extend(case.rejected);
    offered.sort_unstable();
    let config = Config {
        format,
        modifiers: &offered,
        width: case.width,
        height: case.height,
        scanout: case.scanout,
        implicit_sync: !case.flags.contains(Flag::ExplicitSync),
        linear: case.flags.contains(Flag::Linear),
        implicit_interop: false,
        name: None,
    };
    replay.take_calls();
    let result = allocator.allocate(&config);
    let traced = replay.take_calls();
    let layout = match result {
        Ok(allocation) => allocation.layout,
        Err(error) => return Err(failure(format!("allocate: {}", Report::new(error)))),
    };
    if layout.modifier != case.modifier {
        let mismatch = match case.rejected {
            Some(rejected) => Failure {
                reason: format!(
                    "allocate() chooses {:#x}, the reference chose {:#x} over {rejected:#x}",
                    layout.modifier, case.modifier
                ),
                kinds: vec!["modifier: other choice".to_string()],
            },
            None => Failure {
                reason: format!(
                    "allocate() returns {:#x}, not the requested {:#x}",
                    layout.modifier, case.modifier
                ),
                kinds: vec!["modifier: not requested".to_string()],
            },
        };
        return Err(mismatch);
    }
    let planes: Vec<Plane> = layout
        .planes
        .iter()
        .map(|plane| Plane {
            offset: plane.offset,
            stride: plane.stride,
        })
        .collect();
    if case.layout != planes {
        return Err(Failure {
            reason: format!(
                "layout: expected {}, computed {}",
                show_layout(&case.layout),
                show_layout(&planes)
            ),
            kinds: vec!["layout: differs".to_string()],
        });
    }
    drop(allocator);
    if let Some(reason) = replay.leak_error().or_else(|| foreign_handle(&traced)) {
        return Err(failure(reason));
    }
    let calls: Vec<Call> = last_state_only(traced.into_iter().map(|t| t.call).collect());
    let expected: Vec<Call> = case
        .calls
        .iter()
        .map(|call| expected_call(device, &config, call))
        .collect();
    let flags = case.flags;
    let (expected, calls) = match flags.contains(Flag::IgnoreSetTiling) {
        true => (
            without_set_tiling(last_state_only(expected)),
            without_set_tiling(calls),
        ),
        false => (last_state_only(expected), calls),
    };
    let (mut expected, calls) = match flags.contains(Flag::IgnoreSetDomain) {
        true => (without_set_domain(expected), without_set_domain(calls)),
        false => (expected, calls),
    };
    if flags.contains(Flag::IgnoreUmdMetadata)
        || device
            .gfx_level
            .is_some_and(|level| level >= GfxLevel::Gfx9)
    {
        ignore_umd_metadata(&mut expected, &calls);
    }
    if flags.contains(Flag::IgnoreSize) {
        ignore_size(&mut expected, &calls);
    }
    if flags.contains(Flag::IgnoreRegions) {
        ignore_regions(&mut expected, &calls);
    }
    if flags.contains(Flag::AllowWriteCompressDisable) && device.gfx_level == Some(GfxLevel::Gfx12)
    {
        allow_write_compress_disable(&mut expected, &calls);
    }
    if flags.contains(Flag::IgnoreAlignment) {
        ignore_alignment(&mut expected, &calls);
    }
    if flags.contains(Flag::IgnoreDomains) {
        ignore_domains(&mut expected, &calls);
    }
    if flags.contains(Flag::IgnoreCpuAccess) {
        ignore_cpu_access(&mut expected, &calls);
    }
    if calls != expected {
        return Err(Failure {
            reason: format!(
                "calls:\n  expected:{}\n  got:{}",
                show_calls(&expected),
                show_calls(&calls)
            ),
            kinds: categorize(&expected, &calls),
        });
    }
    Ok(())
}

struct FileResult {
    output: String,
    failures: usize,
}

struct Category {
    count: usize,
    example: String,
}

fn read(path: &Path) -> String {
    let mut text = String::new();
    let file = std::fs::File::open(path)
        .unwrap_or_else(|error| panic!("cannot open {}: {error}", path.display()));
    let read = match path.extension().is_some_and(|extension| extension == "zst") {
        true => zstd::Decoder::new(file).and_then(|mut decoder| decoder.read_to_string(&mut text)),
        false => std::io::BufReader::new(file).read_to_string(&mut text),
    };
    if let Err(error) = read {
        panic!("cannot read {}: {error}", path.display());
    }
    text
}

fn run_file(path: &Path, categorize_failures: bool) -> FileResult {
    let devices =
        vectors::parse(&read(path)).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    let mut output = String::new();
    let mut total = 0;
    let mut failures = 0;
    let mut categories: BTreeMap<Vec<String>, Category> = BTreeMap::new();
    for cases in &devices {
        let device = Device::new(&cases.device);
        for case in &cases.cases {
            // TODO: etnaviv NV12. Every plane is a separate buffer.
            if cases.device.driver == "etnaviv" && case.fourcc == u32::from_le_bytes(*b"NV12") {
                continue;
            }
            total += 1;
            let Err(Failure { reason, kinds }) = check(&device, case) else {
                continue;
            };
            failures += 1;
            let name = format!(
                "{} {} {:#x} {}x{} scanout={}",
                device.id,
                String::from_utf8_lossy(&case.fourcc.to_le_bytes()),
                case.modifier,
                case.width,
                case.height,
                case.scanout as u8
            );
            if categorize_failures {
                let category = categories.entry(kinds).or_insert_with(|| Category {
                    count: 0,
                    example: format!("{name}: {reason}"),
                });
                category.count += 1;
            } else if failures <= PRINTED_FAILURES {
                writeln!(output, "FAIL {name}: {reason}").unwrap();
            }
        }
    }
    if !categorize_failures && failures > PRINTED_FAILURES {
        let hidden = failures - PRINTED_FAILURES;
        writeln!(output, "{hidden} more failures are not shown").unwrap();
    }
    let mut sorted: Vec<_> = categories.into_iter().collect();
    sorted.sort_by_key(|(_, category)| std::cmp::Reverse(category.count));
    for (kinds, category) in sorted {
        writeln!(output, "{} cases:", category.count).unwrap();
        for kind in kinds {
            writeln!(output, "  {kind}").unwrap();
        }
        writeln!(output, "  example {}", category.example).unwrap();
    }
    let passes = total - failures;
    writeln!(output, "{}", path.display()).unwrap();
    writeln!(output, "total={total} passes={passes} failures={failures}").unwrap();
    FileResult { output, failures }
}

fn is_vector_file(path: &Path) -> bool {
    let name = path.file_name().and_then(|name| name.to_str());
    name.is_some_and(|name| name.ends_with(".txt") || name.ends_with(".txt.zst"))
}

fn vector_files(directory: &Path) -> Vec<PathBuf> {
    let entries = std::fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", directory.display()));
    let mut paths: Vec<PathBuf> = entries
        .map(|entry| entry.unwrap().path())
        .filter(|path| is_vector_file(path))
        .collect();
    paths.sort();
    paths
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Categorize,
}

fn main() {
    let args = Args::parse();
    let categorize_failures = matches!(args.command, Some(Command::Categorize));
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("vectors");
    let results: Vec<FileResult> = vector_files(&directory)
        .par_iter()
        .map(|path| run_file(path, categorize_failures))
        .collect();
    let mut failed = 0;
    for result in results {
        print!("{}", result.output);
        failed += result.failures;
    }
    if failed != 0 {
        std::process::exit(1);
    }
}
