use crate::calls::Call;
use crate::calls::Field;
use fake_kernel::Node;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write as _;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    pub driver: String,
    pub fake: String,
    pub node: Node,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Flag {
    ExplicitSync,
    AllowWriteCompressDisable,
    IgnoreDomains,
    IgnoreAlignment,
    IgnoreSize,
    IgnoreRegions,
    IgnoreSetTiling,
    IgnoreSetDomain,
    IgnoreUmdMetadata,
    IgnoreCpuAccess,
    Linear,
}

struct FlagToken {
    flag: Flag,
    token: &'static str,
}

const REJECTED_PREFIX: &str = "rejected=";

const FLAG_TOKENS: [FlagToken; 11] = [
    FlagToken {
        flag: Flag::ExplicitSync,
        token: "implicit_sync=0",
    },
    FlagToken {
        flag: Flag::AllowWriteCompressDisable,
        token: "allow_write_compress_disable=1",
    },
    FlagToken {
        flag: Flag::IgnoreDomains,
        token: "ignore_domains=1",
    },
    FlagToken {
        flag: Flag::IgnoreAlignment,
        token: "ignore_alignment=1",
    },
    FlagToken {
        flag: Flag::IgnoreSize,
        token: "ignore_size=1",
    },
    FlagToken {
        flag: Flag::IgnoreRegions,
        token: "ignore_regions=1",
    },
    FlagToken {
        flag: Flag::IgnoreSetTiling,
        token: "ignore_set_tiling=1",
    },
    FlagToken {
        flag: Flag::IgnoreSetDomain,
        token: "ignore_set_domain=1",
    },
    FlagToken {
        flag: Flag::IgnoreUmdMetadata,
        token: "ignore_umd_metadata=1",
    },
    FlagToken {
        flag: Flag::IgnoreCpuAccess,
        token: "ignore_cpu_access=1",
    },
    FlagToken {
        flag: Flag::Linear,
        token: "linear=1",
    },
];

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Flags {
    bits: u16,
}

impl Flags {
    pub fn contains(self, flag: Flag) -> bool {
        self.bits & 1 << flag as u16 != 0
    }

    pub fn insert(&mut self, flag: Flag) {
        self.bits |= 1 << flag as u16;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Plane {
    pub offset: u64,
    pub stride: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Case {
    pub fourcc: u32,
    pub modifier: u64,
    pub rejected: Option<u64>,
    pub width: u32,
    pub height: u32,
    pub scanout: bool,
    pub flags: Flags,
    pub layout: Vec<Plane>,
    pub calls: Vec<Call>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceCases {
    pub device: Device,
    pub cases: Vec<Case>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub line: usize,
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

struct Group {
    fourcc: u32,
    modifier: u64,
    rejected: Option<u64>,
    flags: Flags,
}

struct Size {
    width: u32,
    height: u32,
}

struct Override<'a> {
    key: &'a str,
    value: &'a str,
}

fn device(text: &str) -> Result<Device, String> {
    let mut driver = None;
    let mut fake = None;
    let mut node = None;
    for part in text.split(' ') {
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("device field {part:?} has no value"))?;
        let duplicate = match key {
            "driver" => driver.replace(value).is_some(),
            "fake" => fake.replace(value).is_some(),
            "node" => {
                let kind = match value {
                    "card" => Node::Primary,
                    "render" => Node::Render,
                    _ => return Err(format!("unknown node {value}")),
                };
                node.replace(kind).is_some()
            }
            _ => return Err(format!("unknown device field {key}")),
        };
        if duplicate {
            return Err(format!("duplicate device field {key}"));
        }
    }
    Ok(Device {
        driver: driver.ok_or("device without driver")?.to_string(),
        fake: fake.ok_or("device without fake kernel")?.to_string(),
        node: node.ok_or("device without node")?,
    })
}

struct First<'a> {
    word: &'a str,
    rest: Option<&'a str>,
}

fn first(text: &str) -> First<'_> {
    match text.split_once(' ') {
        Some((word, rest)) => First {
            word,
            rest: Some(rest),
        },
        None => First {
            word: text,
            rest: None,
        },
    }
}

struct Quoted<'a> {
    fourcc: u32,
    modifier: u64,
    rest: Option<&'a str>,
}

fn quoted(text: &str) -> Result<Quoted<'_>, String> {
    let (name, rest) = text
        .strip_prefix('"')
        .and_then(|text| text.split_once('"'))
        .ok_or_else(|| format!("{text:?} does not start with a quoted fourcc"))?;
    let bytes: [u8; 4] = name
        .as_bytes()
        .try_into()
        .map_err(|_| format!("fourcc {name:?} must be 4 bytes"))?;
    let rest = rest
        .strip_prefix(' ')
        .ok_or_else(|| format!("no modifier after fourcc {name:?}"))?;
    let First { word, rest } = first(rest);
    let modifier = word
        .strip_prefix("0x")
        .and_then(|digits| u64::from_str_radix(digits, 16).ok())
        .ok_or_else(|| format!("invalid modifier {word:?}"))?;
    Ok(Quoted {
        fourcc: u32::from_le_bytes(bytes),
        modifier,
        rest,
    })
}

fn size(text: &str) -> Result<Size, String> {
    let parsed = text.split_once('x').and_then(|(width, height)| {
        Some(Size {
            width: width.parse().ok()?,
            height: height.parse().ok()?,
        })
    });
    parsed.ok_or_else(|| format!("invalid size {text:?}"))
}

struct Options {
    rejected: Option<u64>,
    flags: Flags,
}

fn options(tokens: Option<&str>) -> Result<Options, String> {
    let mut options = Options {
        rejected: None,
        flags: Flags::default(),
    };
    for token in tokens.into_iter().flat_map(|tokens| tokens.split(' ')) {
        if let Some(value) = token.strip_prefix(REJECTED_PREFIX) {
            let rejected = value
                .strip_prefix("0x")
                .and_then(|digits| u64::from_str_radix(digits, 16).ok())
                .ok_or_else(|| format!("invalid rejected modifier {value:?}"))?;
            if options.rejected.replace(rejected).is_some() {
                return Err(format!("duplicate option {token:?}"));
            }
            continue;
        }
        let known = FLAG_TOKENS
            .iter()
            .find(|known| known.token == token)
            .ok_or_else(|| format!("unknown flag {token:?}"))?;
        if options.flags.contains(known.flag) {
            return Err(format!("duplicate flag {token:?}"));
        }
        options.flags.insert(known.flag);
    }
    Ok(options)
}

fn layout(text: &str) -> Result<Vec<Plane>, String> {
    let planes = text.split(" | ").map(|plane| {
        let parsed = plane.split_once(' ').and_then(|(offset, stride)| {
            Some(Plane {
                offset: offset.parse().ok()?,
                stride: stride.parse().ok()?,
            })
        });
        parsed.ok_or_else(|| format!("invalid plane {plane:?}"))
    });
    planes.collect()
}

fn overrides(text: &str) -> Result<Vec<Override<'_>>, String> {
    if text.is_empty() {
        return Ok(Vec::new());
    }
    let words = text
        .strip_prefix(' ')
        .ok_or_else(|| format!("invalid overrides {text:?}"))?;
    let overrides = words.split(' ').map(|word| {
        let (key, value) = word
            .split_once('=')
            .ok_or_else(|| format!("override {word:?} has no value"))?;
        Ok(Override { key, value })
    });
    overrides.collect()
}

fn apply_overrides(calls: &mut [Call], overrides: &[Override<'_>]) -> Result<(), String> {
    for field in overrides {
        let mut applied = false;
        for call in calls.iter_mut() {
            applied |= call.set(field.key, field.value)?;
        }
        if !applied {
            return Err(format!("override {} matches no field", field.key));
        }
    }
    Ok(())
}

#[derive(Default)]
struct Parser {
    devices: Vec<DeviceCases>,
    group: Option<Group>,
    verbose_case: Option<usize>,
}

impl Parser {
    fn cases(&mut self) -> Result<&mut Vec<Case>, String> {
        match self.devices.last_mut() {
            Some(device) => Ok(&mut device.cases),
            None => Err("case before the first device".to_string()),
        }
    }

    fn last_case(&mut self) -> Result<&mut Case, String> {
        self.cases()?
            .last_mut()
            .ok_or_else(|| "no case precedes this line".to_string())
    }

    fn finish_case(&mut self) -> Result<(), Error> {
        let Some(line) = self.verbose_case.take() else {
            return Ok(());
        };
        let complete = self.last_case().is_ok_and(|case| !case.layout.is_empty());
        match complete {
            true => Ok(()),
            false => Err(Error {
                line,
                message: "case without layout".to_string(),
            }),
        }
    }

    fn device_line(&mut self, rest: &str) -> Result<(), String> {
        self.devices.push(DeviceCases {
            device: device(rest)?,
            cases: Vec::new(),
        });
        self.group = None;
        Ok(())
    }

    fn format_line(&mut self, rest: &str) -> Result<(), String> {
        let quoted = quoted(rest)?;
        let options = options(quoted.rest)?;
        self.group = Some(Group {
            fourcc: quoted.fourcc,
            modifier: quoted.modifier,
            rejected: options.rejected,
            flags: options.flags,
        });
        Ok(())
    }

    fn verbose_case_line(&mut self, rest: &str) -> Result<(), String> {
        let quoted = quoted(rest)?;
        let size_text = first(quoted.rest.ok_or("case without size")?);
        let size = size(size_text.word)?;
        let scanout_text = first(size_text.rest.ok_or("case without scanout")?);
        let scanout = match scanout_text.word {
            "scanout=0" => false,
            "scanout=1" => true,
            word => return Err(format!("invalid scanout {word:?}")),
        };
        let options = options(scanout_text.rest)?;
        let case = Case {
            fourcc: quoted.fourcc,
            modifier: quoted.modifier,
            rejected: options.rejected,
            width: size.width,
            height: size.height,
            scanout,
            flags: options.flags,
            layout: Vec::new(),
            calls: Vec::new(),
        };
        self.cases()?.push(case);
        Ok(())
    }

    fn layout_line(&mut self, rest: &str) -> Result<(), String> {
        let layout = layout(rest)?;
        self.last_case()?.layout = layout;
        Ok(())
    }

    fn compact_case_line(&mut self, word: &str, rest: &str) -> Result<(), String> {
        let group = self
            .group
            .as_ref()
            .ok_or("case line outside a format group")?;
        let size = size(word)?;
        let scanout_text = first(rest);
        let scanout = match scanout_text.word {
            "s" => true,
            "_" => false,
            word => return Err(format!("invalid scanout marker {word:?}")),
        };
        let rest = scanout_text.rest.ok_or("case without layout")?;
        let (planes, inherited) = match rest.split_once(" =") {
            Some((planes, overrides)) => (planes, Some(overrides)),
            None => (rest, None),
        };
        let mut case = Case {
            fourcc: group.fourcc,
            modifier: group.modifier,
            rejected: group.rejected,
            width: size.width,
            height: size.height,
            scanout,
            flags: group.flags,
            layout: layout(planes)?,
            calls: Vec::new(),
        };
        let cases = self.cases()?;
        if let Some(inherited) = inherited {
            let previous = cases.last().ok_or("= without a previous case")?;
            case.calls = previous.calls.clone();
            apply_overrides(&mut case.calls, &overrides(inherited)?)?;
        }
        cases.push(case);
        Ok(())
    }

    fn call_line(&mut self, line: &str) -> Result<(), String> {
        let call = line.parse()?;
        self.last_case()?.calls.push(call);
        Ok(())
    }

    fn line(&mut self, number: usize, line: &str) -> Result<(), Error> {
        if line.starts_with('#') {
            return Ok(());
        }
        let First { word, rest } = first(line);
        let starts_case = word.starts_with(|c: char| c.is_ascii_digit());
        if starts_case || matches!(word, "device" | "format" | "case") {
            self.finish_case()?;
        }
        let parsed = match (word, rest) {
            ("device", Some(rest)) => self.device_line(rest),
            ("format", Some(rest)) => self.format_line(rest),
            ("case", Some(rest)) => {
                self.verbose_case = Some(number);
                self.verbose_case_line(rest)
            }
            ("layout", Some(rest)) => self.layout_line(rest),
            (_, Some(rest)) if starts_case => self.compact_case_line(word, rest),
            _ => self.call_line(line),
        };
        parsed.map_err(|message| Error {
            line: number,
            message,
        })
    }
}

pub fn parse(text: &str) -> Result<Vec<DeviceCases>, Error> {
    let mut parser = Parser::default();
    for (index, line) in text.lines().enumerate() {
        parser.line(index + 1, line)?;
    }
    parser.finish_case()?;
    Ok(parser.devices)
}

fn group_text(case: &Case) -> String {
    let fourcc = case.fourcc.to_le_bytes();
    let fourcc = std::str::from_utf8(&fourcc).expect("the fourcc is not UTF-8");
    let mut group = format!("\"{fourcc}\" {:#x}", case.modifier);
    if let Some(rejected) = case.rejected {
        write!(group, " {REJECTED_PREFIX}{rejected:#x}").unwrap();
    }
    for known in &FLAG_TOKENS {
        if case.flags.contains(known.flag) {
            write!(group, " {}", known.token).unwrap();
        }
    }
    group
}

fn calls_delta(previous: &[Call], current: &[Call]) -> Option<Vec<Field>> {
    if previous.len() != current.len() {
        return None;
    }
    let mut overrides: Vec<Field> = Vec::new();
    for (previous, current) in previous.iter().zip(current) {
        let previous_fields = previous.fields();
        let current_fields = current.fields();
        if previous.name() != current.name() || previous_fields.len() != current_fields.len() {
            return None;
        }
        for (previous, current) in previous_fields.into_iter().zip(current_fields) {
            if previous.key != current.key {
                return None;
            }
            if previous.value == current.value {
                continue;
            }
            match overrides.iter().find(|field| field.key == current.key) {
                Some(field) if field.value != current.value => return None,
                Some(_) => {}
                None => overrides.push(current),
            }
        }
    }
    if overrides.is_empty() {
        return None;
    }
    let consistent = current.iter().flat_map(Call::fields).all(|current| {
        overrides
            .iter()
            .all(|field| field.key != current.key || field.value == current.value)
    });
    consistent.then_some(overrides)
}

struct Grouped<'a> {
    group: String,
    case: &'a Case,
}

fn order(a: &Grouped<'_>, b: &Grouped<'_>) -> Ordering {
    a.group
        .cmp(&b.group)
        .then(a.case.scanout.cmp(&b.case.scanout))
        .then(a.case.height.cmp(&b.case.height))
        .then(a.case.width.cmp(&b.case.width))
}

pub fn write_compact_device(out: &mut String, device: &Device, cases: &[Case]) {
    let mut cases: Vec<Grouped<'_>> = cases
        .iter()
        .map(|case| Grouped {
            group: group_text(case),
            case,
        })
        .collect();
    cases.sort_by(order);
    let node = match device.node {
        Node::Primary => "card",
        Node::Render => "render",
    };
    writeln!(
        out,
        "device driver={} fake={} node={node}",
        device.driver, device.fake
    )
    .unwrap();
    let mut group: Option<&str> = None;
    let mut previous: Option<&Case> = None;
    for Grouped {
        group: case_group,
        case,
    } in &cases
    {
        if group != Some(case_group.as_str()) {
            writeln!(out, "format {case_group}").unwrap();
            group = Some(case_group.as_str());
            previous = None;
        }
        let scanout = match case.scanout {
            true => "s",
            false => "_",
        };
        let planes: Vec<String> = case
            .layout
            .iter()
            .map(|plane| format!("{} {}", plane.offset, plane.stride))
            .collect();
        write!(
            out,
            "{}x{} {scanout} {}",
            case.width,
            case.height,
            planes.join(" | ")
        )
        .unwrap();
        let delta = match previous {
            Some(previous) if previous.calls == case.calls => Some(Vec::new()),
            Some(previous) => calls_delta(&previous.calls, &case.calls),
            None => None,
        };
        match delta {
            Some(fields) => {
                let overrides: Vec<Override<'_>> = fields
                    .iter()
                    .map(|field| Override {
                        key: field.key,
                        value: &field.value,
                    })
                    .collect();
                let mut calls = previous.map_or(Vec::new(), |previous| previous.calls.clone());
                let applied = apply_overrides(&mut calls, &overrides);
                assert_eq!(applied, Ok(()));
                assert_eq!(calls, case.calls);
                out.push_str(" =");
                for field in &fields {
                    write!(out, " {}={}", field.key, field.value).unwrap();
                }
                out.push('\n');
            }
            None => {
                out.push('\n');
                for call in &case.calls {
                    writeln!(out, "{call}").unwrap();
                }
            }
        }
        previous = Some(case);
    }
}
