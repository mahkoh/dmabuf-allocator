use vulkan_testgen::distribute;
use vulkan_testgen::distribute::JobWords;
use vulkan_testgen::sizes::Size;

#[cfg(test)]
mod tests;

pub const DESCRIBE_PHASE: usize = 0;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    Amd,
    Intel,
    Nouveau,
}

impl Group {
    pub const ALL: [Group; 3] = [Group::Amd, Group::Intel, Group::Nouveau];

    pub fn name(self) -> &'static str {
        match self {
            Group::Amd => "amd",
            Group::Intel => "intel",
            Group::Nouveau => "nouveau",
        }
    }

    pub fn from_name(name: &str) -> Option<Group> {
        Group::ALL.into_iter().find(|group| group.name() == name)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Api {
    Vulkan,
    Gbm,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Backend {
    pub group: Group,
    pub api: Api,
}

impl Backend {
    pub const ALL: [Backend; 6] = [
        Backend {
            group: Group::Amd,
            api: Api::Vulkan,
        },
        Backend {
            group: Group::Amd,
            api: Api::Gbm,
        },
        Backend {
            group: Group::Intel,
            api: Api::Vulkan,
        },
        Backend {
            group: Group::Intel,
            api: Api::Gbm,
        },
        Backend {
            group: Group::Nouveau,
            api: Api::Vulkan,
        },
        Backend {
            group: Group::Nouveau,
            api: Api::Gbm,
        },
    ];
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Job {
    Describe {
        group: Group,
        node: u32,
    },
    Evaluate {
        backend: Backend,
        pair: u32,
        size: Size,
    },
    Record {
        backend: Backend,
        pair: u32,
        size: Size,
    },
    Coverage {
        group: Group,
        pair: u32,
        node: u32,
    },
    Implicit {
        node: u32,
        format: u32,
    },
    SimpleRank {
        driver: u32,
        node: u32,
        format: u32,
    },
    GroupRank {
        group: Group,
        node: u32,
        format: u32,
    },
    Simple {
        driver: u32,
        node: u32,
        format: u32,
    },
}

const DESCRIBE: u32 = 0;
const EVALUATE: u32 = 1;
const RECORD: u32 = 2;
const COVERAGE: u32 = 3;
const IMPLICIT: u32 = 4;
const SIMPLE: u32 = 5;
const SIMPLE_RANK: u32 = 6;
const GROUP_RANK: u32 = 7;

fn position<T: PartialEq>(all: &[T], value: &T) -> u32 {
    let position = all.iter().position(|other| other == value);
    position.expect("every value is listed") as u32
}

fn listed<T: Copy>(all: &[T], word: u32, name: &str) -> T {
    *all.get(word as usize)
        .unwrap_or_else(|| panic!("invalid {name} {word} in a job"))
}

fn group(word: u32) -> Group {
    listed(&Group::ALL, word, "group")
}

fn backend(word: u32) -> Backend {
    listed(&Backend::ALL, word, "backend")
}

impl distribute::Job for Job {
    fn encode(self) -> JobWords {
        match self {
            Job::Describe { group, node } => {
                [DESCRIBE, position(&Group::ALL, &group), node, 0, 0, 0]
            }
            Job::Evaluate {
                backend,
                pair,
                size,
            } => [
                EVALUATE,
                position(&Backend::ALL, &backend),
                pair,
                size.width,
                size.height,
                0,
            ],
            Job::Record {
                backend,
                pair,
                size,
            } => [
                RECORD,
                position(&Backend::ALL, &backend),
                pair,
                size.width,
                size.height,
                0,
            ],
            Job::Coverage { group, pair, node } => {
                [COVERAGE, position(&Group::ALL, &group), pair, node, 0, 0]
            }
            Job::Implicit { node, format } => [IMPLICIT, node, format, 0, 0, 0],
            Job::Simple {
                driver,
                node,
                format,
            } => [SIMPLE, driver, node, format, 0, 0],
            Job::SimpleRank {
                driver,
                node,
                format,
            } => [SIMPLE_RANK, driver, node, format, 0, 0],
            Job::GroupRank {
                group,
                node,
                format,
            } => [
                GROUP_RANK,
                position(&Group::ALL, &group),
                node,
                format,
                0,
                0,
            ],
        }
    }

    fn decode(words: JobWords) -> Self {
        let [kind, a, b, c, d, _] = words;
        match kind {
            DESCRIBE => Job::Describe {
                group: group(a),
                node: b,
            },
            EVALUATE => Job::Evaluate {
                backend: backend(a),
                pair: b,
                size: Size {
                    width: c,
                    height: d,
                },
            },
            RECORD => Job::Record {
                backend: backend(a),
                pair: b,
                size: Size {
                    width: c,
                    height: d,
                },
            },
            COVERAGE => Job::Coverage {
                group: group(a),
                pair: b,
                node: c,
            },
            IMPLICIT => Job::Implicit { node: a, format: b },
            SIMPLE => Job::Simple {
                driver: a,
                node: b,
                format: c,
            },
            SIMPLE_RANK => Job::SimpleRank {
                driver: a,
                node: b,
                format: c,
            },
            GROUP_RANK => Job::GroupRank {
                group: group(a),
                node: b,
                format: c,
            },
            kind => panic!("invalid job kind {kind}"),
        }
    }
}
