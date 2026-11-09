use super::OpenFile;
use super::Supervisor;
use super::fd_path;
use crate::node::DrmNode;
use fake_kernel::AlignedBytes;
use fake_kernel::drm;
use libc::AT_FDCWD;
use libc::DT_BLK;
use libc::DT_CHR;
use libc::DT_DIR;
use libc::DT_FIFO;
use libc::DT_LNK;
use libc::DT_REG;
use libc::DT_SOCK;
use libc::DT_UNKNOWN;
use libc::ENOENT;
use libc::ENOTDIR;
use libc::S_IFCHR;
use libc::S_IFDIR;
use libc::S_IFLNK;
use libc::S_IFMT;
use libc::S_IFREG;
use libc::STATX_BLOCKS;
use libc::STATX_GID;
use libc::STATX_INO;
use libc::STATX_MODE;
use libc::STATX_NLINK;
use libc::STATX_SIZE;
use libc::STATX_TYPE;
use libc::STATX_UID;
use libc::makedev;
use libc::stat;
use libc::statx;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs::FileType;
use std::ops::Range;
use std::os::fd::OwnedFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

#[cfg(test)]
mod tests;

const DRM_MAJOR: u32 = 226;
const FILE_MODE: u32 = 0o444;
const EMULATED_ROOTS: [&str; 8] = [
    "/dev/char",
    "/dev/dri",
    "/dev/fd/",
    "/dev/std",
    "/proc/",
    "/sys/class/drm",
    "/sys/dev/char",
    "/sys/devices/fake",
];
const DRM_VERSION: &str = "drm 1.1.0 20060810\n";
const BLOCK_SIZE: u64 = 4096;

#[derive(Clone)]
pub(super) struct DirEntry {
    pub(super) name: String,
    pub(super) kind: u8,
}

fn decimal(text: &str) -> Option<u32> {
    let canonical = text == "0" || !text.starts_with('0');
    let digits = !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit());
    (canonical && digits).then(|| text.parse().ok())?
}

fn drm_minor(name: &str) -> Option<&str> {
    let (major, minor) = name.split_once(':')?;
    (decimal(major)? == DRM_MAJOR).then_some(minor)
}

fn entry(name: &str, kind: u8) -> DirEntry {
    DirEntry {
        name: name.to_string(),
        kind,
    }
}

fn entries(names: &[String], kind: u8) -> Vec<DirEntry> {
    names.iter().map(|name| entry(name, kind)).collect()
}

fn with_dots(entries: Vec<DirEntry>) -> Vec<DirEntry> {
    let mut dotted = vec![entry(".", DT_DIR), entry("..", DT_DIR)];
    dotted.extend(entries);
    dotted
}

fn directory(entries: Vec<DirEntry>) -> Node {
    Node::Directory(Directory {
        entries: with_dots(entries),
        real: None,
    })
}

fn char_directory(real: &'static str, nodes: impl Iterator<Item = DrmNode>) -> Node {
    let entries = nodes
        .map(|node| entry(&format!("{DRM_MAJOR}:{}", node.minor()), DT_LNK))
        .collect();
    Node::Directory(Directory {
        entries: with_dots(entries),
        real: Some(real),
    })
}

fn bus_link(bus: &str, depth: usize) -> Node {
    Node::Link {
        target: format!("{}bus/{bus}", "../".repeat(depth)),
        resolved: format!("/sys/bus/{bus}"),
    }
}

fn bus_device(names: &[String], tail: &str, subsystem: Node, uevent: String) -> Result<Node, i32> {
    match tail {
        "" => Ok(directory(vec![
            entry("drm", DT_DIR),
            entry("subsystem", DT_LNK),
            entry("uevent", DT_REG),
        ])),
        "drm" => Ok(directory(entries(names, DT_DIR))),
        "subsystem" => Ok(subsystem),
        "uevent" => Ok(Node::File(uevent)),
        _ => Err(ENOENT),
    }
}

fn dirent_type(kind: FileType) -> u8 {
    match () {
        _ if kind.is_dir() => DT_DIR,
        _ if kind.is_symlink() => DT_LNK,
        _ if kind.is_file() => DT_REG,
        _ if kind.is_char_device() => DT_CHR,
        _ if kind.is_block_device() => DT_BLK,
        _ if kind.is_fifo() => DT_FIFO,
        _ if kind.is_socket() => DT_SOCK,
        _ => DT_UNKNOWN,
    }
}

fn real_entries(path: &str) -> Vec<DirEntry> {
    let Ok(directory) = std::fs::read_dir(path) else {
        return Vec::new();
    };
    let entries = directory.flatten().filter_map(|entry| {
        let name = entry.file_name().into_string().ok()?;
        let kind = dirent_type(entry.file_type().ok()?);
        drm_minor(&name)
            .is_none()
            .then_some(DirEntry { name, kind })
    });
    entries.collect()
}

fn inode_number(path: &str) -> u64 {
    path.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ byte as u64).wrapping_mul(0x100_0000_01b3)
    })
}

pub(super) struct LastComponent<'a> {
    pub(super) parent: Vec<u8>,
    pub(super) name: &'a [u8],
    pub(super) trailing_slash: bool,
}

pub(super) fn last_component(path: &[u8]) -> LastComponent<'_> {
    let end = path.iter().rposition(|&b| b != b'/').map_or(0, |i| i + 1);
    let start = path[..end]
        .iter()
        .rposition(|&b| b == b'/')
        .map_or(0, |i| i + 1);
    LastComponent {
        parent: match start {
            0 => b"./".to_vec(),
            _ => path[..start].to_vec(),
        },
        name: &path[start..end],
        trailing_slash: end < path.len(),
    }
}

fn thread_group(tid: u32) -> Option<u32> {
    let status = std::fs::read_to_string(format!("/proc/{tid}/status")).ok()?;
    let line = status.lines().find_map(|line| line.strip_prefix("Tgid:"))?;
    line.trim().parse().ok()
}

fn worker_path(tid: u32, path: &[u8]) -> Option<Vec<u8>> {
    let links = [
        ("/proc/self", ""),
        ("/proc/thread-self", &format!("/task/{tid}")),
        ("/proc/net", "/net"),
        ("/dev/fd", "/fd"),
        ("/dev/stdin", "/fd/0"),
        ("/dev/stdout", "/fd/1"),
        ("/dev/stderr", "/fd/2"),
    ];
    for (link, target) in links {
        if let Some(rest) = path.strip_prefix(link.as_bytes())
            && (rest.is_empty() || rest.starts_with(b"/"))
        {
            let mut path = format!("/proc/{}{target}", thread_group(tid)?).into_bytes();
            path.extend_from_slice(rest);
            return Some(path);
        }
    }
    Some(path.to_vec())
}

fn is_process_link(path: &str) -> bool {
    if let Some(name) = path.strip_prefix("/dev/") {
        let fd = name.strip_prefix("fd/");
        return matches!(name, "stdin" | "stdout" | "stderr")
            || fd.is_some_and(|fd| !fd.contains('/'));
    }
    let Some(rest) = path.strip_prefix("/proc/") else {
        return false;
    };
    let components: Vec<&str> = rest.split('/').collect();
    let link = match components.as_slice() {
        [_, "task", _, link @ ..] | [_, link @ ..] => link,
        [] => return false,
    };
    matches!(link, ["root" | "cwd"] | ["fd", _])
}

fn real_directory(path: &[u8]) -> Result<Vec<u8>, i32> {
    let path = Path::new(OsStr::from_bytes(path));
    let errno = |e: std::io::Error| e.raw_os_error().unwrap();
    match std::fs::metadata(path).map_err(errno)?.is_dir() {
        true => Ok(std::fs::canonicalize(path)
            .map_err(errno)?
            .into_os_string()
            .into_vec()),
        false => Err(ENOTDIR),
    }
}

fn read_link(path: impl AsRef<Path>) -> Option<Vec<u8>> {
    std::fs::read_link(path)
        .ok()
        .map(|target| target.into_os_string().into_vec())
}

fn text(path: &[u8]) -> Cow<'_, str> {
    String::from_utf8_lossy(path)
}

impl Supervisor {
    fn visible(&self, tid: u32) -> Range<usize> {
        let count = self.devices.read().len();
        let visible = self
            .with_thread(tid, |thread| thread.visible.clone())
            .flatten();
        match visible {
            Some(range) => range.start.min(count)..range.end.min(count),
            None => 0..count,
        }
    }

    fn is_registered(&self, index: usize) -> bool {
        index < self.devices.read().len()
    }

    fn render_capable(&self, index: usize) -> bool {
        self.devices.read()[index].render_node()
    }

    fn drm_nodes(&self, index: usize) -> Vec<DrmNode> {
        let mut nodes = vec![DrmNode {
            device: index,
            kind: fake_kernel::Node::Primary,
        }];
        if self.render_capable(index) {
            nodes.push(DrmNode {
                device: index,
                kind: fake_kernel::Node::Render,
            });
        }
        nodes
    }

    fn registered_nodes(&self) -> impl Iterator<Item = DrmNode> {
        let count = self.devices.read().len();
        (0..count).flat_map(|index| self.drm_nodes(index))
    }

    fn visible_nodes(&self, tid: u32) -> impl Iterator<Item = DrmNode> {
        self.visible(tid).flat_map(|index| self.drm_nodes(index))
    }

    fn node_by_minor(&self, minor: &str) -> Option<DrmNode> {
        let node = DrmNode::from_minor(decimal(minor)?)?;
        let exists = self.is_registered(node.device)
            && (node.kind == fake_kernel::Node::Primary || self.render_capable(node.device));
        exists.then_some(node)
    }

    fn visible_node_by_minor(&self, tid: u32, minor: &str) -> Option<DrmNode> {
        let node = self.node_by_minor(minor)?;
        self.visible(tid).contains(&node.device).then_some(node)
    }

    pub(super) fn attributes(&self, path: &str, node: &Node) -> Attributes {
        let in_dev = path.starts_with("/dev/");
        let Mode { mode, rdev } = node.mode();
        Attributes {
            dev: match in_dev {
                true => self.dev_device,
                false => self.sys_device,
            },
            ino: inode_number(path),
            mode,
            nlink: match node {
                Node::Directory(directory) => {
                    let entries = directory.entries();
                    entries.iter().filter(|e| e.kind == DT_DIR).count() as u64
                }
                _ => 1,
            },
            gid: match node {
                Node::Drm(_) => unsafe { libc::getegid() },
                _ => 0,
            },
            rdev,
            size: match node {
                Node::File(contents) => contents.len() as u64,
                Node::Link { target, .. } if in_dev => target.len() as u64,
                _ => 0,
            },
        }
    }

    pub(super) fn inode_at(&self, tid: u32, path: &str) -> u64 {
        let real = match self.lookup(tid, path.as_bytes()) {
            Some(Target::Node { path, .. }) => return inode_number(&path),
            Some(Target::Real(real)) => real,
            None => path.as_bytes().to_vec(),
            Some(_) => return 0,
        };
        let real = OsStr::from_bytes(&real);
        std::fs::symlink_metadata(real).map_or(0, |metadata| metadata.ino())
    }

    pub(super) fn lookup_at(&self, tid: u32, dirfd: i32, path: &[u8]) -> Option<Target> {
        if path.first() == Some(&b'/') {
            return self.lookup(tid, path);
        }
        if path.is_empty() {
            return Some(Target::Error(ENOENT));
        }
        let mut base = match dirfd {
            AT_FDCWD => read_link(format!("/proc/{tid}/cwd"))?,
            _ => read_link(format!("/proc/{tid}/fd/{dirfd}"))?,
        };
        let mut real = false;
        if base.starts_with(b"/memfd:")
            && let Some(inode) = self.tracked_inode(tid, dirfd)
        {
            base = match &self.files.lock().nodes.get(&inode)?.file {
                OpenFile::Node {
                    path,
                    node: Node::Directory(_),
                } => path.clone().into_bytes(),
                OpenFile::Real(file) => {
                    real = true;
                    read_link(fd_path(file).to_str().unwrap())?
                }
                _ => return None,
            };
        }
        if base.last() != Some(&b'/') {
            base.push(b'/');
        }
        base.extend_from_slice(path);
        let target = self.lookup(tid, &base);
        match real {
            true => target.or(Some(Target::Real(base))),
            false => target,
        }
    }

    pub(super) fn follow_unless(&self, tid: u32, target: Target, nofollow: i32) -> Target {
        match target {
            Target::Node {
                node: Node::Link { resolved, .. },
                ..
            } if nofollow == 0 => self
                .lookup(tid, resolved.as_bytes())
                .unwrap_or(Target::Real(resolved.into_bytes())),
            Target::Opened { inode, link } if nofollow == 0 => self.opened_target(inode, link),
            Target::ProcessLink { directory, .. } if nofollow == 0 => self
                .lookup(tid, &directory)
                .unwrap_or(Target::Real(directory)),
            Target::ProcessLink { link, .. } => Target::Real(link),
            target => target,
        }
    }

    fn opened_link(&self, tid: u32, path: &str) -> Option<OpenedLink> {
        let (owner, task, fd) = match path.strip_prefix("/dev/") {
            Some("stdin") => ("self", None, "0"),
            Some("stdout") => ("self", None, "1"),
            Some("stderr") => ("self", None, "2"),
            Some(rest) => ("self", None, rest.strip_prefix("fd/")?),
            None => {
                let (owner, rest) = path.strip_prefix("/proc/")?.split_once('/')?;
                match rest.strip_prefix("task/") {
                    Some(rest) => {
                        let (task, fd) = rest.split_once("/fd/")?;
                        (owner, Some(task), fd)
                    }
                    None => (owner, None, rest.strip_prefix("fd/")?),
                }
            }
        };
        let fd = decimal(fd)?;
        let task = match task {
            Some(task) => Some(decimal(task)?),
            None => None,
        };
        let owner = match owner {
            "self" => thread_group(tid)?,
            "thread-self" if task.is_none() => tid,
            owner => decimal(owner)?,
        };
        let link = match task {
            Some(task) => format!("/proc/{owner}/task/{task}/fd/{fd}"),
            None => format!("/proc/{owner}/fd/{fd}"),
        };
        let metadata = std::fs::metadata(&link).ok()?;
        let inode = self.memfd_inode(metadata.dev(), metadata.ino())?;
        let files = self.files.lock();
        let tracked = files.nodes.contains_key(&inode) || files.dma_bufs.contains_key(&inode);
        tracked.then_some(OpenedLink { inode, link })
    }

    fn lookup(&self, tid: u32, path: &[u8]) -> Option<Target> {
        let relative = path.strip_prefix(b"/")?;
        let plain = relative
            .split(|&b| b == b'/')
            .all(|c| !matches!(c, b"" | b"." | b".."));
        if plain
            && !EMULATED_ROOTS
                .iter()
                .any(|root| path.starts_with(root.as_bytes()))
        {
            return None;
        }
        let trailing_slash = path.ends_with(b"/");
        let components: Vec<&[u8]> = relative
            .split(|&b| b == b'/')
            .filter(|c| !c.is_empty())
            .collect();
        let mut resolved = Vec::new();
        let mut emulated = false;
        let mut canonical = true;
        for (index, component) in components.iter().enumerate() {
            match *component {
                b"." => continue,
                b".." => {
                    if !canonical {
                        let real = match real_directory(&worker_path(tid, &resolved)?) {
                            Ok(real) => real,
                            Err(errno) => return emulated.then_some(Target::Error(errno)),
                        };
                        resolved = real;
                        canonical = true;
                    }
                    resolved.truncate(resolved.iter().rposition(|&b| b == b'/').unwrap_or(0));
                    continue;
                }
                name => {
                    if !resolved.ends_with(b"/") {
                        resolved.push(b'/');
                    }
                    resolved.extend_from_slice(name);
                }
            }
            if index + 1 == components.len() && !trailing_slash {
                break;
            }
            let mut node = match self.opened_link(tid, &text(&resolved)) {
                Some(OpenedLink { inode, link }) => {
                    emulated = true;
                    match self.opened_target(inode, link) {
                        Target::Node {
                            path,
                            node: node @ Node::Directory(_),
                        } => {
                            resolved = path.into_bytes();
                            Some(Ok(node))
                        }
                        Target::File(file) => {
                            resolved = read_link(fd_path(&file).to_str().unwrap())?;
                            None
                        }
                        Target::Error(errno) => Some(Err(errno)),
                        _ => Some(Err(ENOTDIR)),
                    }
                }
                None => self.lookup_canonical(tid, &text(&resolved)),
            };
            let mut followed = false;
            if node.is_none() && is_process_link(&text(&resolved)) {
                resolved = match real_directory(&worker_path(tid, &resolved)?) {
                    Ok(real) => real,
                    Err(errno) => return emulated.then_some(Target::Error(errno)),
                };
                node = self.lookup_canonical(tid, &text(&resolved));
                followed = true;
            }
            if let Some(Ok(Node::Link {
                resolved: target, ..
            })) = node
            {
                node = self.lookup_canonical(tid, &target);
                resolved = target.into_bytes();
                emulated = true;
            }
            canonical = followed || node.is_some();
            match node {
                None => {}
                Some(Ok(Node::Directory(_))) => emulated = true,
                Some(Ok(_)) => return Some(Target::Error(ENOTDIR)),
                Some(Err(errno)) => return Some(Target::Error(errno)),
            }
        }
        if resolved.is_empty() {
            resolved.push(b'/');
        }
        let path = text(&resolved);
        if let Some(OpenedLink { inode, link }) = self.opened_link(tid, &path) {
            return Some(Target::Opened { inode, link });
        }
        match self.lookup_canonical(tid, &path) {
            Some(Ok(node)) => Some(Target::Node {
                path: path.into_owned(),
                node,
            }),
            Some(Err(errno)) => Some(Target::Error(errno)),
            None if !trailing_slash && is_process_link(&path) => {
                let link = worker_path(tid, &resolved)?;
                match real_directory(&link) {
                    Ok(directory) if self.lookup_canonical(tid, &text(&directory)).is_some() => {
                        Some(Target::ProcessLink { link, directory })
                    }
                    _ => emulated.then_some(Target::Real(link)),
                }
            }
            None if emulated => {
                if trailing_slash && !resolved.ends_with(b"/") {
                    resolved.push(b'/');
                }
                Some(Target::Real(worker_path(tid, &resolved)?))
            }
            None => None,
        }
    }

    fn lookup_canonical(&self, tid: u32, path: &str) -> Option<Result<Node, i32>> {
        if path == "/dev/dri" {
            let entries = self
                .visible_nodes(tid)
                .map(|node| entry(&node.name(), DT_CHR));
            return Some(Ok(directory(entries.collect())));
        }
        if let Some(name) = path.strip_prefix("/dev/dri/") {
            let node = self.visible_nodes(tid).find(|node| node.name() == name);
            return Some(match node {
                Some(node) => Ok(Node::Drm(node)),
                None => Err(ENOENT),
            });
        }
        if path == "/dev/char" {
            return Some(Ok(char_directory("/dev/char", self.visible_nodes(tid))));
        }
        if let Some(name) = path.strip_prefix("/dev/char/") {
            let minor = drm_minor(name)?;
            return Some(match self.visible_node_by_minor(tid, minor) {
                Some(node) => Ok(Node::Link {
                    target: format!("../dri/{}", node.name()),
                    resolved: format!("/dev/dri/{}", node.name()),
                }),
                None => Err(ENOENT),
            });
        }
        if path == "/sys/dev/char" {
            let nodes = self.registered_nodes();
            return Some(Ok(char_directory("/sys/dev/char", nodes)));
        }
        if let Some(name) = path.strip_prefix("/sys/dev/char/") {
            let minor = drm_minor(name)?;
            return Some(match self.node_by_minor(minor) {
                Some(node) => Ok(self.class_link(node)),
                None => Err(ENOENT),
            });
        }
        if path == "/sys/class/drm" {
            let mut entries: Vec<DirEntry> = self
                .registered_nodes()
                .map(|node| entry(&node.name(), DT_LNK))
                .collect();
            entries.push(entry("version", DT_REG));
            return Some(Ok(directory(entries)));
        }
        if let Some(name) = path.strip_prefix("/sys/class/drm/") {
            if name == "version" {
                return Some(Ok(Node::File(DRM_VERSION.to_string())));
            }
            let node = self.registered_nodes().find(|node| node.name() == name);
            return Some(match node {
                Some(node) => Ok(self.class_link(node)),
                None => Err(ENOENT),
            });
        }
        let rest = path.strip_prefix("/sys/devices/fake")?;
        let (index, tail) = rest.split_once('/').unwrap_or((rest, ""));
        let index = decimal(index)? as usize;
        match self.is_registered(index) {
            true => Some(self.device_node(index, tail)),
            false => Some(Err(ENOENT)),
        }
    }

    fn class_link(&self, node: DrmNode) -> Node {
        let parent = self.drm_parent(node.device);
        let name = node.name();
        Node::Link {
            target: format!("../../devices/{parent}/drm/{name}"),
            resolved: format!("/sys/devices/{parent}/drm/{name}"),
        }
    }

    fn drm_names(&self, index: usize) -> Vec<String> {
        let nodes = self.drm_nodes(index);
        nodes.iter().map(DrmNode::name).collect()
    }

    fn drm_parent(&self, index: usize) -> String {
        match drm(self.devices.read()[index].as_ref())
            .virtio_device()
            .is_some()
        {
            true => format!("fake{index}/virtio{index}"),
            false => format!("fake{index}"),
        }
    }

    fn drm_entry(
        names: &[String],
        tail: &str,
        device: &str,
        resolved: String,
    ) -> Result<Node, i32> {
        let (name, rest) = tail.split_once('/').unwrap_or((tail, ""));
        if !names.iter().any(|n| n == name) {
            return Err(ENOENT);
        }
        match rest {
            "" => Ok(directory(vec![entry("device", DT_LNK)])),
            "device" => Ok(Node::Link {
                target: format!("../../../{device}"),
                resolved,
            }),
            _ => Err(ENOENT),
        }
    }

    fn device_node(&self, index: usize, tail: &str) -> Result<Node, i32> {
        let names = self.drm_names(index);
        let devices = self.devices.read();
        let kernel = drm(devices[index].as_ref());
        let virtio = format!("virtio{index}");
        if let Some(virtio_device) = kernel.virtio_device() {
            if let Some(rest) = tail.strip_prefix(&*virtio) {
                if let Some(drm) = rest.strip_prefix("/drm/") {
                    let resolved = format!("/sys/devices/fake{index}/{virtio}");
                    return Self::drm_entry(&names, drm, &virtio, resolved);
                }
                let tail = match rest.strip_prefix('/') {
                    Some(tail) if !tail.is_empty() => tail,
                    None if rest.is_empty() => "",
                    _ => return Err(ENOENT),
                };
                let Some(pci) = kernel.pci() else {
                    unimplemented!("virtio devices that are not PCI devices");
                };
                let uevent = format!(
                    "DRIVER={}\nMODALIAS=virtio:d{virtio_device:08X}v{:08X}\n",
                    kernel.uevent_driver(),
                    pci.vendor
                );
                return bus_device(&names, tail, bus_link("virtio", 3), uevent);
            }
        } else if let Some(drm) = tail.strip_prefix("drm/") {
            let resolved = format!("/sys/devices/fake{index}");
            return Self::drm_entry(&names, drm, &format!("fake{index}"), resolved);
        }
        if kernel.faux() {
            let uevent = format!("DRIVER={}\n", kernel.uevent_driver());
            return bus_device(&names, tail, bus_link("faux", 2), uevent);
        }
        if let Some(name) = kernel.virtual_platform() {
            let uevent = format!(
                "DRIVER={}\nMODALIAS=platform:{name}\n",
                kernel.uevent_driver()
            );
            return bus_device(&names, tail, bus_link("platform", 2), uevent);
        }
        if let Some(platform) = kernel.platform() {
            let mut uevent = format!(
                "OF_FULLNAME={}\nOF_COMPATIBLE_N={}\n",
                platform.fullname,
                platform.compatible.len()
            );
            for (i, compatible) in platform.compatible.iter().enumerate() {
                uevent.push_str(&format!("OF_COMPATIBLE_{i}={compatible}\n"));
            }
            return bus_device(&names, tail, bus_link("platform", 2), uevent);
        }
        let Some(pci) = kernel.pci() else {
            unimplemented!("the bus of {}", kernel.id());
        };
        let (vendor, pci_id) = (pci.vendor, pci.device);
        let (child, driver) = match kernel.virtio_device().is_some() {
            true => (&*virtio, "virtio-pci"),
            false => ("drm", kernel.uevent_driver()),
        };
        Ok(match tail {
            "" => directory(vec![
                entry(child, DT_DIR),
                entry("subsystem", DT_LNK),
                entry("vendor", DT_REG),
                entry("device", DT_REG),
                entry("subsystem_vendor", DT_REG),
                entry("subsystem_device", DT_REG),
                entry("revision", DT_REG),
                entry("uevent", DT_REG),
            ]),
            "drm" if kernel.virtio_device().is_none() => directory(entries(&names, DT_DIR)),
            "subsystem" => bus_link("pci", 2),
            "vendor" | "subsystem_vendor" => Node::File(format!("0x{vendor:04x}\n")),
            "device" | "subsystem_device" => Node::File(format!("0x{pci_id:04x}\n")),
            "revision" => Node::File("0x00\n".to_string()),
            "uevent" => Node::File(format!(
                "DRIVER={driver}\nPCI_ID={vendor:04X}:{pci_id:04X}\nPCI_SUBSYS_ID={vendor:04X}:{pci_id:04X}\nPCI_SLOT_NAME=0000:00:{index:02x}.0\n"
            )),
            _ => return Err(ENOENT),
        })
    }
}

pub(super) struct Mode {
    pub(super) mode: u32,
    pub(super) rdev: u64,
}

struct OpenedLink {
    inode: u64,
    link: String,
}

#[derive(Clone)]
pub(super) struct Directory {
    entries: Vec<DirEntry>,
    real: Option<&'static str>,
}

impl Directory {
    pub(super) fn entries(&self) -> Vec<DirEntry> {
        let mut entries = self.entries.clone();
        if let Some(real) = self.real {
            entries.extend(real_entries(real));
        }
        entries
    }
}

#[derive(Clone)]
pub(super) enum Node {
    Drm(DrmNode),
    Directory(Directory),
    File(String),
    Link { target: String, resolved: String },
}

impl Node {
    pub(super) fn mode(&self) -> Mode {
        let mode = match self {
            Node::Drm(_) => S_IFCHR | 0o660,
            Node::Directory(_) => S_IFDIR | 0o755,
            Node::File(_) => S_IFREG | FILE_MODE,
            Node::Link { .. } => S_IFLNK | 0o777,
        };
        let rdev = match self {
            Node::Drm(node) => makedev(DRM_MAJOR, node.minor()),
            _ => 0,
        };
        Mode { mode, rdev }
    }
}

pub(super) struct Attributes {
    dev: u64,
    ino: u64,
    mode: u32,
    nlink: u64,
    gid: u32,
    rdev: u64,
    size: u64,
}

impl Attributes {
    pub(super) fn stat(&self) -> AlignedBytes {
        const { assert!(align_of::<stat>() <= align_of::<u64>()) };
        let mut bytes = AlignedBytes::zeroed(size_of::<stat>());
        let st = unsafe { &mut *bytes.as_mut_ptr().cast::<stat>() };
        st.st_dev = self.dev;
        st.st_ino = self.ino;
        st.st_mode = self.mode;
        st.st_nlink = self.nlink;
        st.st_gid = self.gid;
        st.st_rdev = self.rdev;
        st.st_size = self.size as i64;
        st.st_blksize = BLOCK_SIZE as i64;
        bytes
    }

    pub(super) fn statx(&self) -> AlignedBytes {
        const { assert!(align_of::<statx>() <= align_of::<u64>()) };
        let mut bytes = AlignedBytes::zeroed(size_of::<statx>());
        let st = unsafe { &mut *bytes.as_mut_ptr().cast::<statx>() };
        st.stx_mask = STATX_TYPE
            | STATX_MODE
            | STATX_NLINK
            | STATX_UID
            | STATX_GID
            | STATX_INO
            | STATX_SIZE
            | STATX_BLOCKS;
        st.stx_blksize = BLOCK_SIZE as u32;
        st.stx_nlink = self.nlink as u32;
        st.stx_gid = self.gid;
        st.stx_mode = self.mode as u16;
        st.stx_ino = self.ino;
        st.stx_size = self.size;
        st.stx_rdev_major = libc::major(self.rdev);
        st.stx_rdev_minor = libc::minor(self.rdev);
        st.stx_dev_major = libc::major(self.dev);
        st.stx_dev_minor = libc::minor(self.dev);
        bytes
    }
}

pub(super) fn permits(file_mode: u32, mode: i32) -> bool {
    let allowed = match file_mode & S_IFMT {
        S_IFCHR => file_mode >> 3 & 0o7,
        _ => file_mode & 0o7,
    };
    mode as u32 & !allowed == 0
}

pub(super) enum Target {
    Node { path: String, node: Node },
    Real(Vec<u8>),
    File(OwnedFd),
    Opened { inode: u64, link: String },
    DmaBuf { link: String },
    ProcessLink { link: Vec<u8>, directory: Vec<u8> },
    Error(i32),
}
