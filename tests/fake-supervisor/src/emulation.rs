use crate::memory::RemoteMemory;
use crate::node::DrmNode;
use crate::node::MAX_DEVICES;
use allocator_trace::Traced;
use fake_kernel::AlignedBytes;
use fake_kernel::Drm;
use fake_kernel::Export;
use fake_kernel::FILE_SIZE;
use fake_kernel::Kernel;
use fake_kernel::UserMemory;
use fake_kernel::drm;
use fake_kernel::uapi::drm::DRM_IOCTL_GEM_CLOSE;
use fake_kernel::uapi::drm::drm_gem_close;
use fake_kernel::uapi::drm::drm_prime_handle;
use libc::AT_EACCESS;
use libc::AT_EMPTY_PATH;
use libc::AT_FDCWD;
use libc::AT_NO_AUTOMOUNT;
use libc::AT_STATX_SYNC_TYPE;
use libc::AT_SYMLINK_NOFOLLOW;
use libc::EACCES;
use libc::EBADF;
use libc::EBUSY;
use libc::EEXIST;
use libc::EINVAL;
use libc::EIO;
use libc::EISDIR;
use libc::ELOOP;
use libc::ENOENT;
use libc::ENOTDIR;
use libc::ENXIO;
use libc::EPERM;
use libc::ESRCH;
use libc::F_GETFL;
use libc::F_SETFL;
use libc::IN_CLOEXEC;
use libc::IN_CLOSE_NOWRITE;
use libc::IN_CLOSE_WRITE;
use libc::IN_IGNORED;
use libc::IN_Q_OVERFLOW;
use libc::MFD_CLOEXEC;
use libc::O_ACCMODE;
use libc::O_APPEND;
use libc::O_ASYNC;
use libc::O_CLOEXEC;
use libc::O_CREAT;
use libc::O_DIRECT;
use libc::O_DIRECTORY;
use libc::O_DSYNC;
use libc::O_EXCL;
use libc::O_NOATIME;
use libc::O_NOFOLLOW;
use libc::O_NONBLOCK;
use libc::O_PATH;
use libc::O_RDONLY;
use libc::O_RDWR;
use libc::O_SYNC;
use libc::O_TMPFILE;
use libc::O_TRUNC;
use libc::O_WRONLY;
use libc::PATH_MAX;
use libc::PIDFD_THREAD;
use libc::POLLHUP;
use libc::POLLIN;
use libc::SECCOMP_ADDFD_FLAG_SEND;
use libc::SECCOMP_IOCTL_NOTIF_ADDFD;
use libc::SECCOMP_IOCTL_NOTIF_RECV;
use libc::SECCOMP_IOCTL_NOTIF_SEND;
use libc::SECCOMP_IOCTL_NOTIF_SET_FLAGS;
use libc::SECCOMP_USER_NOTIF_FLAG_CONTINUE;
use libc::SEEK_CUR;
use libc::SEEK_SET;
use libc::SIGKILL;
use libc::STATX__RESERVED;
use libc::SYS_access;
use libc::SYS_creat;
use libc::SYS_faccessat;
use libc::SYS_faccessat2;
use libc::SYS_fcntl;
use libc::SYS_fstat;
use libc::SYS_getdents64;
use libc::SYS_ioctl;
use libc::SYS_lstat;
use libc::SYS_newfstatat;
use libc::SYS_open;
use libc::SYS_openat;
use libc::SYS_pidfd_getfd;
use libc::SYS_pidfd_open;
use libc::SYS_pidfd_send_signal;
use libc::SYS_readlink;
use libc::SYS_readlinkat;
use libc::SYS_stat;
use libc::SYS_statx;
use libc::dirent64;
use libc::inotify_event;
use libc::pollfd;
use libc::seccomp_notif;
use libc::seccomp_notif_addfd;
use libc::seccomp_notif_resp;
use libc::stat;
use libc::statx;
use parking_lot::Mutex;
use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::mem::offset_of;
use std::num::NonZeroUsize;
use std::ops::Range;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::OwnedFd;
use std::os::unix::fs::FileExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use std::sync::OnceLock;
use tree::Attributes;
use tree::DirEntry;
use tree::Node;
use tree::Target;
use tree::last_component;
use tree::permits;

#[cfg(test)]
mod tests;
mod tree;

const HANDLERS_VARIABLE: &str = "FAKE_SUPERVISOR_HANDLERS";
const DEFAULT_HANDLERS: NonZeroUsize = NonZeroUsize::new(4).unwrap();
const SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP: u64 = 1 << 0;
const STAT_FLAGS: i32 = AT_SYMLINK_NOFOLLOW | AT_NO_AUTOMOUNT | AT_EMPTY_PATH | AT_STATX_SYNC_TYPE;
const PATH_FLAGS: i32 = O_PATH | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC;
const ACCESS_FLAGS: i32 = AT_SYMLINK_NOFOLLOW | AT_EACCESS | AT_EMPTY_PATH;
const LARGEFILE: i32 = 0o100000;
const KEPT_FLAGS: i32 = O_ACCMODE
    | O_APPEND
    | O_NONBLOCK
    | O_SYNC
    | O_DSYNC
    | O_ASYNC
    | O_DIRECT
    | LARGEFILE
    | O_DIRECTORY
    | O_NOFOLLOW
    | O_NOATIME;
const SETFL_FLAGS: i32 = O_APPEND | O_NONBLOCK | O_NOATIME;
const NO_ACCESS: i32 = O_ACCMODE;
const TMPFILE_FLAG: i32 = O_TMPFILE & !O_DIRECTORY;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Member {
    device: usize,
    object: u32,
}

struct DmaBuf {
    own: Option<OwnedFd>,
    export: Export,
    members: Vec<Member>,
    flags: i32,
}

enum OpenFile {
    Node { path: String, node: Node },
    Real(OwnedFd),
}

struct OpenNode {
    file: OpenFile,
    flags: i32,
}

impl OpenNode {
    fn target(&self) -> Target {
        match &self.file {
            OpenFile::Node { path, node } => Target::Node {
                path: path.clone(),
                node: node.clone(),
            },
            OpenFile::Real(file) => Target::File(file.try_clone().expect("dup failed")),
        }
    }
}

struct TrackedFile {
    file: OwnedFd,
    inode: u64,
    watch: i32,
}

#[derive(Default)]
struct Files {
    nodes: BTreeMap<u64, OpenNode>,
    dma_bufs: BTreeMap<u64, DmaBuf>,
    members: BTreeMap<Member, u64>,
    watches: BTreeMap<i32, u64>,
}

pub(super) struct WorkerThread {
    pidfd: Arc<OwnedFd>,
    pub(super) visible: Option<Range<usize>>,
    recording: Option<Vec<Traced>>,
}

struct DeviceFile {
    node: DrmNode,
    inode: u64,
}

enum StatSource {
    Path(CString),
    File(OwnedFd),
    Node(Attributes),
}

pub struct Supervisor {
    pidfd: OwnedFd,
    listener: OwnedFd,
    inotify: OwnedFd,
    memfd_device: OnceLock<u64>,
    dev_device: u64,
    sys_device: u64,
    devices: RwLock<Vec<Box<dyn Kernel>>>,
    files: Mutex<Files>,
    threads: Mutex<BTreeMap<u32, WorkerThread>>,
}

fn open_thread_pidfd(tid: u32) -> Option<OwnedFd> {
    let pidfd = unsafe { libc::syscall(SYS_pidfd_open, tid, PIDFD_THREAD) };
    if pidfd < 0 {
        return None;
    }
    Some(unsafe { OwnedFd::from_raw_fd(pidfd as i32) })
}

fn exited(pidfd: &OwnedFd) -> bool {
    let mut poll = pollfd {
        fd: pidfd.as_raw_fd(),
        events: POLLIN,
        revents: 0,
    };
    unsafe { libc::poll(&mut poll, 1, 0) > 0 }
}

enum Reply {
    Value(i64),
    Continue,
    Sent,
}

fn errno(errno: i32) -> Reply {
    Reply::Value(-(errno as i64))
}

fn last_errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap()
}

fn write_reply(user: &RemoteMemory, buf: u64, bytes: &[u8], value: i64) -> Reply {
    match user.write(buf, bytes) {
        Ok(()) => Reply::Value(value),
        Err(e) => errno(e),
    }
}

fn path_argument(user: &RemoteMemory, path: u64, flags: i32) -> Result<Vec<u8>, i32> {
    match path == 0 && flags & AT_EMPTY_PATH != 0 {
        true => Ok(Vec::new()),
        false => user.path(path),
    }
}

fn reported_flags(flags: i32) -> i32 {
    match flags & O_PATH {
        0 => flags & KEPT_FLAGS | LARGEFILE,
        _ => flags & (O_PATH | O_DIRECTORY | O_NOFOLLOW),
    }
}

fn fd_path(file: &OwnedFd) -> CString {
    CString::new(format!("/proc/self/fd/{}", file.as_raw_fd())).unwrap()
}

fn reopen(file: &OwnedFd, access: i32) -> OwnedFd {
    let reopened = unsafe { libc::open(fd_path(file).as_ptr(), access | O_CLOEXEC) };
    assert!(reopened >= 0, "reopening a memfd failed");
    unsafe { OwnedFd::from_raw_fd(reopened) }
}

fn device_of(path: &str) -> u64 {
    std::fs::metadata(path).map_or(0, |metadata| metadata.dev())
}

fn create_memfd(name: &CStr, size: u64, contents: &[u8]) -> OwnedFd {
    let memfd = unsafe { libc::memfd_create(name.as_ptr(), MFD_CLOEXEC) };
    assert!(memfd >= 0, "memfd_create failed");
    let memfd = unsafe { OwnedFd::from_raw_fd(memfd) };
    if !contents.is_empty() {
        let written = unsafe {
            libc::pwrite(
                memfd.as_raw_fd(),
                contents.as_ptr().cast(),
                contents.len(),
                0,
            )
        };
        assert_eq!(written, contents.len() as isize);
    }
    if size > contents.len() as u64 {
        let truncated = unsafe { libc::ftruncate(memfd.as_raw_fd(), size as i64) };
        assert_eq!(truncated, 0, "ftruncate failed");
    }
    memfd
}

fn stat_of(file: &OwnedFd) -> stat {
    let mut st: stat = unsafe { std::mem::zeroed() };
    assert_eq!(unsafe { libc::fstat(file.as_raw_fd(), &mut st) }, 0);
    st
}

fn stat_bytes_of(file: &OwnedFd) -> AlignedBytes {
    let mut st = AlignedBytes::zeroed(size_of::<stat>());
    let stated = unsafe { libc::fstat(file.as_raw_fd(), st.as_mut_ptr().cast()) };
    assert_eq!(stated, 0);
    st
}

fn dirent64_record(index: usize, inode: u64, entry: &DirEntry) -> Vec<u8> {
    let name = entry.name.as_bytes();
    let name_offset = offset_of!(dirent64, d_name);
    assert!(name.len() < 256);
    let length = (name_offset + name.len() + 1).next_multiple_of(align_of::<dirent64>());
    let mut dirent = vec![0u8; length];
    dirent[offset_of!(dirent64, d_ino)..][..size_of::<u64>()].copy_from_slice(&inode.to_ne_bytes());
    dirent[offset_of!(dirent64, d_off)..][..size_of::<i64>()]
        .copy_from_slice(&(index as i64 + 1).to_ne_bytes());
    dirent[offset_of!(dirent64, d_reclen)..][..size_of::<u16>()]
        .copy_from_slice(&(length as u16).to_ne_bytes());
    dirent[offset_of!(dirent64, d_type)] = entry.kind;
    dirent[name_offset..][..name.len()].copy_from_slice(name);
    dirent
}

impl Supervisor {
    pub fn new(worker: u32, listener: OwnedFd) -> Self {
        let pidfd = unsafe { libc::syscall(SYS_pidfd_open, worker, 0) };
        assert!(pidfd >= 0, "pidfd_open failed");
        let inotify = unsafe { libc::inotify_init1(IN_CLOEXEC) };
        assert!(inotify >= 0, "inotify_init1 failed");
        Self {
            pidfd: unsafe { OwnedFd::from_raw_fd(pidfd as i32) },
            listener,
            inotify: unsafe { OwnedFd::from_raw_fd(inotify) },
            memfd_device: OnceLock::new(),
            dev_device: device_of("/dev"),
            sys_device: device_of("/sys"),
            devices: RwLock::new(Vec::new()),
            files: Mutex::new(Files::default()),
            threads: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn kill_worker(&self) {
        let info = std::ptr::null::<libc::siginfo_t>();
        unsafe {
            libc::syscall(
                SYS_pidfd_send_signal,
                self.pidfd.as_raw_fd(),
                SIGKILL,
                info,
                0,
            );
        }
    }

    pub fn start(self: &Arc<Self>, control: UnixStream) {
        let handlers = match std::env::var(HANDLERS_VARIABLE) {
            Ok(handlers) => handlers.parse().unwrap_or_else(|_| {
                panic!("{HANDLERS_VARIABLE}={handlers:?} is not a positive number")
            }),
            Err(_) => DEFAULT_HANDLERS,
        };
        let set = unsafe {
            libc::ioctl(
                self.listener.as_raw_fd(),
                SECCOMP_IOCTL_NOTIF_SET_FLAGS,
                SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP,
            )
        };
        assert_eq!(set, 0, "SECCOMP_IOCTL_NOTIF_SET_FLAGS failed");
        for _ in 0..handlers.get() {
            let supervisor = self.clone();
            crate::spawn(move || supervisor.serve_notifications());
        }
        let supervisor = self.clone();
        crate::spawn(move || supervisor.watch_releases());
        let supervisor = self.clone();
        crate::spawn(move || supervisor.serve_control(control));
    }

    fn serve_notifications(&self) {
        loop {
            let mut request: seccomp_notif = unsafe { std::mem::zeroed() };
            let received = unsafe {
                libc::ioctl(
                    self.listener.as_raw_fd(),
                    SECCOMP_IOCTL_NOTIF_RECV,
                    &mut request,
                )
            };
            if received == 0 {
                self.respond(&request);
                continue;
            }
            let mut poll = pollfd {
                fd: self.listener.as_raw_fd(),
                events: POLLIN,
                revents: 0,
            };
            let ready = unsafe { libc::poll(&mut poll, 1, -1) };
            if ready == 1 && poll.revents & POLLHUP != 0 {
                return;
            }
        }
    }

    fn respond(&self, request: &seccomp_notif) {
        let mut response = seccomp_notif_resp {
            id: request.id,
            val: 0,
            error: 0,
            flags: 0,
        };
        match self.handle(request) {
            Reply::Sent => return,
            Reply::Continue => response.flags = SECCOMP_USER_NOTIF_FLAG_CONTINUE as u32,
            Reply::Value(value) if value < 0 => response.error = value as i32,
            Reply::Value(value) => response.val = value,
        }
        unsafe {
            libc::ioctl(
                self.listener.as_raw_fd(),
                SECCOMP_IOCTL_NOTIF_SEND,
                &response,
            );
        }
    }

    #[allow(non_upper_case_globals)]
    fn handle(&self, request: &seccomp_notif) -> Reply {
        let tid = request.pid;
        let args = request.data.args;
        let user = RemoteMemory { tid };
        match request.data.nr as i64 {
            SYS_ioctl => self.ioctl(request, args[0] as i32, args[1] as u32, args[2]),
            SYS_fstat => self.fstat(&user, args[0] as i32, args[1]),
            SYS_stat => self.newfstatat(&user, AT_FDCWD, args[0], args[1], 0),
            SYS_lstat => {
                let flags = AT_SYMLINK_NOFOLLOW as u64;
                self.newfstatat(&user, AT_FDCWD, args[0], args[1], flags)
            }
            SYS_newfstatat => self.newfstatat(&user, args[0] as i32, args[1], args[2], args[3]),
            SYS_statx => self.statx(&user, args[0] as i32, args[1], args[2], args[3], args[4]),
            SYS_open => self.openat(request, AT_FDCWD, args[0], args[1] as i32, args[2] as u32),
            SYS_creat => {
                let flags = O_CREAT | O_WRONLY | O_TRUNC;
                self.openat(request, AT_FDCWD, args[0], flags, args[1] as u32)
            }
            SYS_openat => self.openat(
                request,
                args[0] as i32,
                args[1],
                args[2] as i32,
                args[3] as u32,
            ),
            SYS_readlink => self.readlink(&user, AT_FDCWD, args[0], args[1], args[2] as i32),
            SYS_readlinkat => {
                self.readlink(&user, args[0] as i32, args[1], args[2], args[3] as i32)
            }
            SYS_getdents64 => self.getdents64(&user, args[0] as i32, args[1], args[2] as u32),
            SYS_fcntl => self.fcntl(&user, args[0] as i32, args[1] as i32, args[2]),
            SYS_access => self.access(&user, AT_FDCWD, args[0], args[1] as i32, 0),
            SYS_faccessat => self.access(&user, args[0] as i32, args[1], args[2] as i32, 0),
            SYS_faccessat2 => self.access(
                &user,
                args[0] as i32,
                args[1],
                args[2] as i32,
                args[3] as i32,
            ),
            nr => unreachable!("syscall {nr} is not notified"),
        }
    }

    fn memfd_inode(&self, device: u64, inode: u64) -> Option<u64> {
        (self.memfd_device.get() == Some(&device)).then_some(inode)
    }

    pub(super) fn with_thread<T>(
        &self,
        tid: u32,
        f: impl FnOnce(&mut WorkerThread) -> T,
    ) -> Option<T> {
        let mut threads = self.threads.lock();
        let running = threads
            .get(&tid)
            .is_some_and(|thread| !exited(&thread.pidfd));
        if !running {
            threads.retain(|_, thread| !exited(&thread.pidfd));
            let thread = WorkerThread {
                pidfd: Arc::new(open_thread_pidfd(tid)?),
                visible: None,
                recording: None,
            };
            threads.insert(tid, thread);
        }
        threads.get_mut(&tid).map(f)
    }

    fn worker_fd(&self, tid: u32, fd: i32) -> Option<OwnedFd> {
        let pidfd = self.with_thread(tid, |thread| thread.pidfd.clone())?;
        let file = unsafe { libc::syscall(SYS_pidfd_getfd, pidfd.as_raw_fd(), fd, 0) };
        if file >= 0 {
            return Some(unsafe { OwnedFd::from_raw_fd(file as i32) });
        }
        match last_errno() {
            EBADF | ESRCH => None,
            errno => panic!(
                "pidfd_getfd of file descriptor {fd} of thread {tid} failed: {}",
                std::io::Error::from_raw_os_error(errno)
            ),
        }
    }

    fn worker_file(&self, tid: u32, fd: i32) -> Option<OwnedFd> {
        let file = self.worker_fd(tid, fd)?;
        let flags = unsafe { libc::fcntl(file.as_raw_fd(), F_GETFL) };
        (flags & O_PATH == 0).then_some(file)
    }

    pub(super) fn tracked_inode(&self, tid: u32, fd: i32) -> Option<u64> {
        if fd < 0 {
            return None;
        }
        let st = stat_of(&self.worker_fd(tid, fd)?);
        self.memfd_inode(st.st_dev, st.st_ino)
    }

    fn device_file(&self, tid: u32, fd: i32) -> Option<Result<DeviceFile, i32>> {
        let inode = self.tracked_inode(tid, fd)?;
        let files = self.files.lock();
        let open = files.nodes.get(&inode)?;
        match open.file {
            _ if open.flags & O_PATH != 0 => Some(Err(EBADF)),
            OpenFile::Node {
                node: Node::Drm(node),
                ..
            } => Some(Ok(DeviceFile { node, inode })),
            _ => None,
        }
    }

    fn fd_target(&self, tid: u32, fd: i32) -> Option<Target> {
        let inode = self.tracked_inode(tid, fd)?;
        Some(self.files.lock().nodes.get(&inode)?.target())
    }

    fn target_at(&self, tid: u32, dirfd: i32, name: &[u8], flags: i32) -> Option<Target> {
        if flags & AT_EMPTY_PATH != 0 && name.is_empty() {
            return self.fd_target(tid, dirfd);
        }
        let target = self.lookup_at(tid, dirfd, name)?;
        Some(self.follow_unless(tid, target, flags & AT_SYMLINK_NOFOLLOW))
    }

    fn opened_target(&self, inode: u64, link: String) -> Target {
        let files = self.files.lock();
        match files.nodes.get(&inode) {
            Some(open) => open.target(),
            None if files.dma_bufs.contains_key(&inode) => Target::DmaBuf { link },
            None => Target::Error(ENOENT),
        }
    }

    fn ioctl(&self, request: &seccomp_notif, fd: i32, cmd: u32, arg: u64) -> Reply {
        let tid = request.pid;
        let DeviceFile { node, inode: file } = match self.device_file(tid, fd) {
            Some(Ok(file)) => file,
            Some(Err(e)) => return errno(e),
            None => return Reply::Continue,
        };
        let device = node.device;
        let user = RemoteMemory { tid };
        let recording = self
            .with_thread(tid, |thread| thread.recording.is_some())
            .unwrap_or(false);
        let devices = self.devices.read();
        let kernel = devices[device].as_ref();
        let pending = match recording {
            true => allocator_trace::decode(kernel, file, cmd, arg, &user),
            false => Vec::new(),
        };
        let closed = match cmd == DRM_IOCTL_GEM_CLOSE {
            true => {
                let close = (&user as &dyn UserMemory).read_value::<drm_gem_close>(arg);
                close
                    .ok()
                    .and_then(|close| drm(kernel).object(file, close.handle))
            }
            false => None,
        };
        let mut prime = Prime {
            supervisor: self,
            request,
            device,
            fd,
        };
        let ret = fake_kernel::ioctl(kernel, file, node.kind, cmd, arg, &user, &mut prime);
        if ret >= 0 && !pending.is_empty() {
            let traced: Vec<Traced> = pending
                .into_iter()
                .map(|pending| {
                    pending
                        .finish(kernel, file, &user)
                        .expect("the ioctl arguments became unreadable")
                })
                .collect();
            self.with_thread(tid, |thread| {
                if let Some(recording) = &mut thread.recording {
                    recording.extend(traced);
                }
            });
        }
        if ret == 0
            && let Some(object) = closed
        {
            let member = Member { device, object };
            let mut files = self.files.lock();
            if let Some(&inode) = files.members.get(&member) {
                drop_unreferenced(&devices, &mut files, &[inode]);
            }
        }
        Reply::Value(ret)
    }

    fn stat_source(&self, target: Option<Target>) -> Result<StatSource, Reply> {
        match target {
            None | Some(Target::Opened { .. } | Target::DmaBuf { .. }) => Err(Reply::Continue),
            Some(Target::Error(e)) => Err(errno(e)),
            Some(Target::Real(path)) => Ok(StatSource::Path(CString::new(path).unwrap())),
            Some(Target::File(file)) => Ok(StatSource::File(file)),
            Some(Target::Node { path, node }) => {
                Ok(StatSource::Node(self.attributes(&path, &node)))
            }
            Some(Target::ProcessLink { .. }) => unreachable!("process links have been followed"),
        }
    }

    fn stat_source_at(
        &self,
        user: &RemoteMemory,
        dirfd: i32,
        path: u64,
        flags: i32,
        valid_query: bool,
    ) -> Result<StatSource, Reply> {
        let name = path_argument(user, path, flags).map_err(errno)?;
        if !valid_query {
            return Err(errno(EINVAL));
        }
        let by_fd = name.is_empty() && flags & AT_EMPTY_PATH != 0 && dirfd >= 0;
        if !by_fd && flags & !STAT_FLAGS != 0 {
            return Err(errno(EINVAL));
        }
        self.stat_source(self.target_at(user.tid, dirfd, &name, flags))
    }

    fn stat_reply(
        &self,
        user: &RemoteMemory,
        source: Result<StatSource, Reply>,
        buf: u64,
        flags: i32,
    ) -> Reply {
        let st = match source {
            Err(reply) => return reply,
            Ok(StatSource::Path(path)) => {
                let mut st = AlignedBytes::zeroed(size_of::<stat>());
                let stated = unsafe {
                    libc::fstatat(AT_FDCWD, path.as_ptr(), st.as_mut_ptr().cast(), flags)
                };
                if stated != 0 {
                    return errno(last_errno());
                }
                st
            }
            Ok(StatSource::File(file)) => stat_bytes_of(&file),
            Ok(StatSource::Node(attributes)) => attributes.stat(),
        };
        write_reply(user, buf, st.bytes(), 0)
    }

    fn statx_reply(
        &self,
        user: &RemoteMemory,
        source: Result<StatSource, Reply>,
        buf: u64,
        flags: i32,
        mask: u32,
    ) -> Reply {
        let mut st = AlignedBytes::zeroed(size_of::<statx>());
        let stated = match source {
            Err(reply) => return reply,
            Ok(StatSource::Path(path)) => unsafe {
                libc::statx(AT_FDCWD, path.as_ptr(), flags, mask, st.as_mut_ptr().cast())
            },
            Ok(StatSource::File(file)) => {
                let flags = AT_EMPTY_PATH | flags & AT_STATX_SYNC_TYPE;
                unsafe {
                    libc::statx(
                        file.as_raw_fd(),
                        c"".as_ptr(),
                        flags,
                        mask,
                        st.as_mut_ptr().cast(),
                    )
                }
            }
            Ok(StatSource::Node(attributes)) => {
                st = attributes.statx();
                0
            }
        };
        if stated != 0 {
            return errno(last_errno());
        }
        write_reply(user, buf, st.bytes(), 0)
    }

    fn fstat(&self, user: &RemoteMemory, fd: i32, buf: u64) -> Reply {
        let source = self.stat_source(self.fd_target(user.tid, fd));
        self.stat_reply(user, source, buf, 0)
    }

    fn newfstatat(
        &self,
        user: &RemoteMemory,
        dirfd: i32,
        path: u64,
        buf: u64,
        flags: u64,
    ) -> Reply {
        let flags = flags as i32;
        let source = self.stat_source_at(user, dirfd, path, flags, true);
        self.stat_reply(user, source, buf, flags)
    }

    fn statx(
        &self,
        user: &RemoteMemory,
        dirfd: i32,
        path: u64,
        flags: u64,
        mask: u64,
        buf: u64,
    ) -> Reply {
        let flags = flags as i32;
        let valid_query = mask as u32 & STATX__RESERVED as u32 == 0
            && flags & AT_STATX_SYNC_TYPE != AT_STATX_SYNC_TYPE;
        let source = self.stat_source_at(user, dirfd, path, flags, valid_query);
        self.statx_reply(user, source, buf, flags, mask as u32)
    }

    fn track(&self, memfd: OwnedFd, access: i32) -> TrackedFile {
        let file = reopen(&memfd, access);
        drop(memfd);
        let st = stat_of(&file);
        let device = *self.memfd_device.get_or_init(|| st.st_dev);
        assert_eq!(device, st.st_dev);
        let link = fd_path(&file);
        let watch = unsafe {
            libc::inotify_add_watch(
                self.inotify.as_raw_fd(),
                link.as_ptr(),
                IN_CLOSE_WRITE | IN_CLOSE_NOWRITE,
            )
        };
        assert!(watch >= 0, "inotify_add_watch failed");
        assert_eq!(unsafe { libc::fchmod(file.as_raw_fd(), 0) }, 0);
        TrackedFile {
            file,
            inode: st.st_ino,
            watch,
        }
    }

    fn add_fd(
        &self,
        request: &seccomp_notif,
        fd: &OwnedFd,
        flags: i32,
        send: bool,
    ) -> Result<i32, i32> {
        let add = seccomp_notif_addfd {
            id: request.id,
            flags: match send {
                true => SECCOMP_ADDFD_FLAG_SEND as u32,
                false => 0,
            },
            srcfd: fd.as_raw_fd() as u32,
            newfd: 0,
            newfd_flags: (flags & O_CLOEXEC) as u32,
        };
        let ret =
            unsafe { libc::ioctl(self.listener.as_raw_fd(), SECCOMP_IOCTL_NOTIF_ADDFD, &add) };
        match ret {
            ret if ret >= 0 => Ok(ret),
            _ => Err(last_errno()),
        }
    }

    fn reply_with_fd(&self, request: &seccomp_notif, fd: &OwnedFd, flags: i32) -> Reply {
        match self.add_fd(request, fd, flags, true) {
            Ok(_) => Reply::Sent,
            Err(e) => errno(e),
        }
    }

    fn open_tracked(
        &self,
        request: &seccomp_notif,
        flags: i32,
        memfd: OwnedFd,
        access: i32,
        open: OpenNode,
    ) -> Reply {
        let tracked = self.track(memfd, access);
        {
            let devices = self.devices.read();
            let mut files = self.files.lock();
            if let OpenFile::Node {
                node: Node::Drm(node),
                ..
            } = &open.file
                && open.flags & O_PATH == 0
            {
                let kernel = devices[node.device].as_ref();
                fake_kernel::open(kernel, tracked.inode, node.kind);
            }
            files.watches.insert(tracked.watch, tracked.inode);
            files.nodes.insert(tracked.inode, open);
        }
        self.reply_with_fd(request, &tracked.file, flags)
    }

    fn openat(
        &self,
        request: &seccomp_notif,
        dirfd: i32,
        path: u64,
        flags: i32,
        mode: u32,
    ) -> Reply {
        let flags = match flags & O_PATH {
            0 => flags,
            _ => flags & PATH_FLAGS,
        };
        let invalid_tmpfile = flags & TMPFILE_FLAG != 0
            && (flags & O_DIRECTORY == 0 || flags & O_ACCMODE == O_RDONLY);
        if flags & O_CREAT != 0 && flags & O_DIRECTORY != 0 || invalid_tmpfile {
            return errno(EINVAL);
        }
        let tid = request.pid;
        let name = match (RemoteMemory { tid }).path(path) {
            Ok(name) => name,
            Err(e) => return errno(e),
        };
        let Some(target) = self.lookup_at(tid, dirfd, &name) else {
            return Reply::Continue;
        };
        if flags & O_CREAT != 0
            && let Some(e) = self.create_error(tid, dirfd, &name, &target)
        {
            return errno(e);
        }
        let exists = matches!(
            target,
            Target::Node { .. } | Target::Opened { .. } | Target::ProcessLink { .. }
        );
        if flags & O_CREAT != 0 && flags & O_EXCL != 0 && exists {
            return errno(EEXIST);
        }
        match self.follow_unless(tid, target, flags & O_NOFOLLOW) {
            Target::Error(e) => errno(e),
            Target::Real(path) => {
                let path = CString::new(path).unwrap();
                self.open_real(request, &path, flags, mode)
            }
            Target::File(file) => self.open_real(request, &fd_path(&file), flags, mode),
            Target::Node { path, node } => self.open_node(request, path, node, flags),
            Target::Opened { .. } => match flags & O_PATH {
                0 => errno(ELOOP),
                _ => Reply::Continue,
            },
            Target::DmaBuf { link } => match flags & O_PATH {
                0 => errno(ENXIO),
                _ => self.open_real(request, &CString::new(link).unwrap(), flags, mode),
            },
            Target::ProcessLink { .. } => unreachable!("process links have been followed"),
        }
    }

    fn create_error(&self, tid: u32, dirfd: i32, name: &[u8], target: &Target) -> Option<i32> {
        if matches!(target, Target::Real(_)) {
            return None;
        }
        let last = last_component(name);
        if matches!(last.name, b"" | b"." | b"..") {
            return None;
        }
        match self.lookup_at(tid, dirfd, &last.parent) {
            Some(Target::Error(e)) => Some(e),
            _ if last.trailing_slash => Some(EISDIR),
            _ => matches!(target, Target::Error(ENOENT)).then_some(EACCES),
        }
    }

    fn open_node(&self, request: &seccomp_notif, path: String, node: Node, flags: i32) -> Reply {
        let is_directory = matches!(node, Node::Directory(_));
        if flags & O_PATH != 0 {
            if flags & O_DIRECTORY != 0 && !is_directory {
                return errno(ENOTDIR);
            }
            return self.open_path(request, flags, OpenFile::Node { path, node });
        }
        let access = flags & O_ACCMODE;
        let writes = access != O_RDONLY || flags & O_TRUNC != 0;
        match &node {
            Node::Directory(_) if flags & TMPFILE_FLAG != 0 => return errno(EACCES),
            Node::Directory(_) if writes || flags & O_CREAT != 0 => return errno(EISDIR),
            _ if flags & O_DIRECTORY != 0 && !is_directory => return errno(ENOTDIR),
            Node::Link { .. } => return errno(ELOOP),
            Node::File(_) if writes => return errno(EACCES),
            _ => {}
        }
        if flags & O_NOATIME != 0 {
            return errno(EPERM);
        }
        if flags & O_EXCL != 0 && matches!(node, Node::Drm(_)) {
            return errno(EBUSY);
        }
        if flags & O_DIRECT != 0 {
            return errno(EINVAL);
        }
        let (memfd, access) = match &node {
            Node::Directory(_) => (create_memfd(c"dir", 0, &[]), O_RDONLY),
            Node::Drm(_) => (create_memfd(c"drm", FILE_SIZE as u64, &[]), access),
            Node::File(contents) => {
                let size = contents.len() as u64;
                let memfd = create_memfd(c"sysfs", size, contents.as_bytes());
                (memfd, O_RDONLY)
            }
            Node::Link { .. } => unreachable!("opening a symlink fails with ELOOP"),
        };
        let open = OpenNode {
            file: OpenFile::Node { path, node },
            flags: reported_flags(flags),
        };
        self.open_tracked(request, flags, memfd, access, open)
    }

    fn open_real(&self, request: &seccomp_notif, path: &CStr, flags: i32, mode: u32) -> Reply {
        let fd = unsafe { libc::open(path.as_ptr(), flags | O_CLOEXEC, mode) };
        if fd < 0 {
            return errno(last_errno());
        }
        let fd = unsafe { OwnedFd::from_raw_fd(fd) };
        match flags & O_PATH {
            0 => self.reply_with_fd(request, &fd, flags),
            _ => self.open_path(request, flags, OpenFile::Real(fd)),
        }
    }

    fn open_path(&self, request: &seccomp_notif, flags: i32, file: OpenFile) -> Reply {
        let memfd = create_memfd(c"path", 0, &[]);
        let open = OpenNode {
            file,
            flags: reported_flags(flags),
        };
        self.open_tracked(request, flags, memfd, NO_ACCESS, open)
    }

    fn readlink(&self, user: &RemoteMemory, dirfd: i32, path: u64, buf: u64, size: i32) -> Reply {
        if size <= 0 {
            return errno(EINVAL);
        }
        let name = match user.path(path) {
            Ok(name) => name,
            Err(e) => return errno(e),
        };
        let write_link = |link: &[u8]| {
            let len = link.len().min(size as usize);
            write_reply(user, buf, &link[..len], len as i64)
        };
        let read_real = |dirfd: i32, path: &CStr| {
            let mut bytes = vec![0u8; (size as usize).min(PATH_MAX as usize)];
            let len = unsafe {
                libc::readlinkat(dirfd, path.as_ptr(), bytes.as_mut_ptr().cast(), bytes.len())
            };
            match len {
                0.. => write_reply(user, buf, &bytes[..len as usize], len as i64),
                _ => errno(last_errno()),
            }
        };
        if name.is_empty() {
            return match self.fd_target(user.tid, dirfd) {
                Some(Target::Node {
                    node: Node::Link { target, .. },
                    ..
                }) => write_link(target.as_bytes()),
                Some(Target::File(file)) => read_real(file.as_raw_fd(), c""),
                _ => Reply::Continue,
            };
        }
        let Some(target) = self.lookup_at(user.tid, dirfd, &name) else {
            return Reply::Continue;
        };
        match target {
            Target::Node {
                node: Node::Link { target, .. },
                ..
            } => write_link(target.as_bytes()),
            Target::Node { .. } => errno(EINVAL),
            Target::Error(e) => errno(e),
            Target::Real(path) | Target::ProcessLink { link: path, .. } => {
                read_real(AT_FDCWD, &CString::new(path).unwrap())
            }
            Target::Opened { inode, link } => match self.opened_target(inode, link) {
                Target::Node { path, .. } => write_link(path.as_bytes()),
                Target::File(file) => read_real(AT_FDCWD, &fd_path(&file)),
                Target::Error(e) => errno(e),
                _ => Reply::Continue,
            },
            Target::File(_) | Target::DmaBuf { .. } => unreachable!("lookups follow no link"),
        }
    }

    fn getdents64(&self, user: &RemoteMemory, fd: i32, buf: u64, size: u32) -> Reply {
        let Some(inode) = self.tracked_inode(user.tid, fd) else {
            return Reply::Continue;
        };
        let Some(file) = self.worker_file(user.tid, fd) else {
            return errno(EBADF);
        };
        let position = unsafe { libc::lseek(file.as_raw_fd(), 0, SEEK_CUR) };
        assert!(position >= 0, "a memfd has no file position");
        let position = position as usize;
        let (path, directory) = match self.files.lock().nodes.get(&inode) {
            Some(OpenNode { flags, .. }) if flags & O_PATH != 0 => return errno(EBADF),
            Some(OpenNode {
                file:
                    OpenFile::Node {
                        path,
                        node: Node::Directory(directory),
                    },
                ..
            }) => (path.clone(), directory.clone()),
            _ => return Reply::Continue,
        };
        let entries = directory.entries();
        let remaining = entries.get(position..).unwrap_or_default();
        let mut out = Vec::new();
        let mut next = position;
        for (index, entry) in (position..).zip(remaining) {
            let inode = self.inode_at(user.tid, &format!("{path}/{}", entry.name));
            let record = dirent64_record(index, inode, entry);
            if out.len() + record.len() > size as usize {
                break;
            }
            out.extend(record);
            next = index + 1;
        }
        if out.is_empty() && !remaining.is_empty() {
            return errno(EINVAL);
        }
        let reply = write_reply(user, buf, &out, out.len() as i64);
        if let Reply::Value(0..) = reply {
            unsafe {
                libc::lseek(file.as_raw_fd(), next as i64, SEEK_SET);
            }
        }
        reply
    }

    fn access(&self, user: &RemoteMemory, dirfd: i32, path: u64, mode: i32, flags: i32) -> Reply {
        if mode & !0o7 != 0 || flags & !ACCESS_FLAGS != 0 {
            return errno(EINVAL);
        }
        let name = match user.path(path) {
            Ok(name) => name,
            Err(e) => return errno(e),
        };
        let checked = match self.target_at(user.tid, dirfd, &name, flags) {
            None | Some(Target::Opened { .. } | Target::DmaBuf { .. }) => return Reply::Continue,
            Some(Target::Error(e)) => return errno(e),
            Some(Target::Real(path)) => {
                let path = CString::new(path).unwrap();
                unsafe { libc::faccessat(AT_FDCWD, path.as_ptr(), mode, flags) }
            }
            Some(Target::File(file)) => {
                let flags = AT_EMPTY_PATH | flags & AT_EACCESS;
                let fd = file.as_raw_fd();
                unsafe { libc::syscall(SYS_faccessat2, fd, c"".as_ptr(), mode, flags) as i32 }
            }
            Some(Target::Node { node, .. }) => match permits(node.mode().mode, mode) {
                true => 0,
                false => return errno(EACCES),
            },
            Some(Target::ProcessLink { .. }) => unreachable!("process links have been followed"),
        };
        match checked {
            0 => Reply::Value(0),
            _ => errno(last_errno()),
        }
    }

    fn fcntl(&self, user: &RemoteMemory, fd: i32, cmd: i32, arg: u64) -> Reply {
        let Some(inode) = self.tracked_inode(user.tid, fd) else {
            return Reply::Continue;
        };
        if self.worker_file(user.tid, fd).is_none() {
            return Reply::Continue;
        }
        let mut files = self.files.lock();
        let Files {
            nodes, dma_bufs, ..
        } = &mut *files;
        let (flags, owned) = match (nodes.get_mut(&inode), dma_bufs.get_mut(&inode)) {
            (Some(open), _) => (&mut open.flags, false),
            (None, Some(dma_buf)) => (&mut dma_buf.flags, true),
            (None, None) => return Reply::Continue,
        };
        let arg = arg as i32;
        match cmd {
            F_GETFL => Reply::Value(*flags as i64),
            F_SETFL if *flags & O_PATH != 0 => errno(EBADF),
            F_SETFL if arg & O_NOATIME != 0 && !owned => errno(EPERM),
            F_SETFL if arg & O_DIRECT != 0 => errno(EINVAL),
            F_SETFL => {
                *flags = *flags & !SETFL_FLAGS | arg & SETFL_FLAGS;
                Reply::Value(0)
            }
            _ => unreachable!("fcntl {cmd} is not notified"),
        }
    }

    fn watch_releases(&self) {
        let mut buf = vec![0u8; 64 << 10];
        let mut hung_up = false;
        loop {
            if hung_up && self.files.lock().watches.is_empty() {
                return;
            }
            let mut polls = [
                pollfd {
                    fd: self.inotify.as_raw_fd(),
                    events: POLLIN,
                    revents: 0,
                },
                pollfd {
                    fd: self.listener.as_raw_fd(),
                    events: 0,
                    revents: 0,
                },
            ];
            let count = match hung_up {
                true => 1,
                false => 2,
            };
            if unsafe { libc::poll(polls.as_mut_ptr(), count, -1) < 0 } {
                continue;
            }
            if polls[1].revents & POLLHUP != 0 {
                hung_up = true;
            }
            if polls[0].revents & POLLIN == 0 {
                continue;
            }
            let len =
                unsafe { libc::read(self.inotify.as_raw_fd(), buf.as_mut_ptr().cast(), buf.len()) };
            if len <= 0 {
                continue;
            }
            let mut pos = 0;
            while pos < len as usize {
                let event = unsafe {
                    buf.as_ptr()
                        .add(pos)
                        .cast::<inotify_event>()
                        .read_unaligned()
                };
                pos += size_of::<inotify_event>() + event.len as usize;
                assert_eq!(
                    event.mask & IN_Q_OVERFLOW,
                    0,
                    "the inotify queue overflowed and lost releases"
                );
                if event.mask & IN_IGNORED != 0 {
                    continue;
                }
                if event.mask & (IN_CLOSE_WRITE | IN_CLOSE_NOWRITE) != 0 {
                    self.release(event.wd);
                }
            }
        }
    }

    fn release(&self, watch: i32) {
        let devices = self.devices.read();
        let mut files = self.files.lock();
        let Some(inode) = files.watches.remove(&watch) else {
            return;
        };
        if let Some(open) = files.nodes.remove(&inode) {
            if let OpenFile::Node {
                node: Node::Drm(DrmNode { device, .. }),
                ..
            } = open.file
                && open.flags & O_PATH == 0
            {
                fake_kernel::release(devices[device].as_ref(), inode);
                let all: Vec<u64> = files.dma_bufs.keys().copied().collect();
                drop_unreferenced(&devices, &mut files, &all);
            }
            return;
        }
        if let Some(dma_buf) = files.dma_bufs.remove(&inode) {
            for member in &dma_buf.members {
                files.members.remove(member);
                drm(devices[member.device].as_ref()).release_export(member.object);
            }
        }
    }

    fn serve_control(self: &Arc<Self>, control: UnixStream) {
        let mut reader = BufReader::new(control);
        let mut line = String::new();
        loop {
            line.clear();
            if !matches!(reader.read_line(&mut line), Ok(1..)) {
                return;
            }
            let request = line.trim_end_matches('\n');
            let replied = match request {
                "connect" => self.connect(reader.get_ref()),
                _ => reader.get_mut().write_all(self.control(request).as_bytes()),
            };
            if replied.is_err() {
                return;
            }
        }
    }

    fn connect(self: &Arc<Self>, control: &UnixStream) -> std::io::Result<()> {
        let (connection, worker) = UnixStream::pair()?;
        let supervisor = self.clone();
        crate::spawn(move || supervisor.serve_control(connection));
        crate::send_fd(control.as_raw_fd(), worker.as_raw_fd())
    }

    fn control(&self, line: &str) -> String {
        match self.execute(line) {
            Ok(reply) => reply,
            Err(error) => format!("error {error}\n"),
        }
    }

    fn execute(&self, line: &str) -> Result<String, String> {
        let (command, arguments) = match line.split_once(' ') {
            Some((command, arguments)) => (command, arguments.split(' ').collect()),
            None => (line, Vec::new()),
        };
        let number = |text: &str| -> Result<usize, String> {
            text.parse()
                .map_err(|_| format!("{text:?} is not a number"))
        };
        let thread = |arguments: &[&str]| -> Result<u32, String> {
            let tid = arguments.first().ok_or("the request names no thread")?;
            tid.parse()
                .map_err(|_| format!("{tid:?} is not a thread id"))
        };
        let gone = |tid: u32| format!("thread {tid} does not run");
        match command {
            "register" => {
                let mut devices = self.devices.write();
                let start = devices.len();
                if start + arguments.len() > MAX_DEVICES {
                    return Err(format!("more than {MAX_DEVICES} devices"));
                }
                for id in arguments {
                    let kernel = fake_kernel::from_id(id);
                    assert_eq!(kernel.id(), id, "the fake kernel id does not round-trip");
                    devices.push(kernel);
                }
                Ok(format!("{start} {}\n", devices.len()))
            }
            "visible" => {
                let tid = thread(&arguments)?;
                let range = match arguments[1..] {
                    ["all"] => None,
                    [start, end] => Some(number(start)?..number(end)?),
                    _ => return Err("the request names no devices".to_string()),
                };
                self.with_thread(tid, |thread| thread.visible = range)
                    .ok_or_else(|| gone(tid))?;
                Ok("ok\n".to_string())
            }
            "record" => {
                let tid = thread(&arguments)?;
                self.with_thread(tid, |thread| thread.recording = Some(Vec::new()))
                    .ok_or_else(|| gone(tid))?;
                Ok("ok\n".to_string())
            }
            "stop" => {
                let tid = thread(&arguments)?;
                let recording = self
                    .with_thread(tid, |thread| thread.recording.take())
                    .ok_or_else(|| gone(tid))?
                    .unwrap_or_default();
                let mut reply = format!("{}\n", recording.len());
                for traced in recording {
                    let handle = match traced.handle {
                        Some(handle) => handle.to_string(),
                        None => "none".to_string(),
                    };
                    reply.push_str(&format!("{handle} {}\n", traced.call));
                }
                Ok(reply)
            }
            _ => Err("unknown request".to_string()),
        }
    }
}

fn drop_unreferenced(devices: &[Box<dyn Kernel>], files: &mut Files, candidates: &[u64]) {
    for inode in candidates {
        let Some(dma_buf) = files.dma_bufs.get_mut(inode) else {
            continue;
        };
        let referenced = dma_buf
            .members
            .iter()
            .any(|m| drm(devices[m.device].as_ref()).object_has_handles(m.object));
        if !referenced {
            dma_buf.own = None;
        }
    }
}

struct Prime<'a> {
    supervisor: &'a Supervisor,
    request: &'a seccomp_notif,
    device: usize,
    fd: i32,
}

impl fake_kernel::Prime for Prime<'_> {
    fn sync_file(&mut self) -> Result<i32, i32> {
        let fence = unsafe { libc::eventfd(1, libc::EFD_CLOEXEC) };
        if fence < 0 {
            return Err(last_errno());
        }
        let fence = unsafe { OwnedFd::from_raw_fd(fence) };
        self.supervisor
            .add_fd(self.request, &fence, O_CLOEXEC, false)
    }

    fn read_file(&mut self, offset: u64, data: &mut [u8]) -> Result<(), i32> {
        let file = self.supervisor.worker_file(self.request.pid, self.fd);
        let file = File::from(file.ok_or(EBADF)?);
        file.read_exact_at(data, offset)
            .map_err(|error| error.raw_os_error().unwrap_or(EIO))
    }

    fn write_file(&mut self, offset: u64, data: &[u8]) -> Result<(), i32> {
        let file = self.supervisor.worker_file(self.request.pid, self.fd);
        let file = File::from(file.ok_or(EBADF)?);
        file.write_all_at(data, offset)
            .map_err(|error| error.raw_os_error().unwrap_or(EIO))
    }

    unsafe fn handle_to_fd(
        &mut self,
        kernel: &dyn Drm,
        file: u64,
        args: &mut drm_prime_handle,
    ) -> i64 {
        let supervisor = self.supervisor;
        let mut files = supervisor.files.lock();
        let export = match kernel.export(file, args.handle) {
            Ok(export) => export,
            Err(errno) => return -(errno as i64),
        };
        let member = Member {
            device: self.device,
            object: export.object,
        };
        let inode = match files.members.get(&member) {
            Some(&inode) => inode,
            None => {
                let memfd = create_memfd(c"dmabuf", export.size, &[]);
                let tracked = supervisor.track(memfd, O_RDWR);
                files.watches.insert(tracked.watch, tracked.inode);
                files.dma_bufs.insert(
                    tracked.inode,
                    DmaBuf {
                        own: Some(tracked.file),
                        export,
                        members: vec![member],
                        flags: O_RDWR,
                    },
                );
                files.members.insert(member, tracked.inode);
                tracked.inode
            }
        };
        let own = files.dma_bufs[&inode]
            .own
            .as_ref()
            .expect("an exported object with handles keeps its dma-buf");
        match supervisor.add_fd(self.request, own, O_CLOEXEC, false) {
            Ok(fd) => {
                args.fd = fd;
                0
            }
            Err(e) => -(e as i64),
        }
    }

    unsafe fn fd_to_handle(
        &mut self,
        kernel: &dyn Drm,
        file: u64,
        args: &mut drm_prime_handle,
    ) -> i64 {
        let supervisor = self.supervisor;
        let Some(dma_buf_file) = supervisor.worker_file(self.request.pid, args.fd) else {
            return -(EBADF as i64);
        };
        let st = stat_of(&dma_buf_file);
        let Some(inode) = supervisor.memfd_inode(st.st_dev, st.st_ino) else {
            return -(EINVAL as i64);
        };
        let mut files = supervisor.files.lock();
        let Files {
            dma_bufs, members, ..
        } = &mut *files;
        let Some(dma_buf) = dma_bufs.get_mut(&inode) else {
            return -(EINVAL as i64);
        };
        let member = dma_buf.members.iter().find(|m| m.device == self.device);
        let handle = match member.and_then(|m| kernel.import(file, m.object)) {
            Some(handle) => handle,
            None => {
                let import = kernel.import_foreign(file, &dma_buf.export);
                let member = Member {
                    device: self.device,
                    object: import.object,
                };
                for stale in dma_buf.members.iter().filter(|m| m.device == self.device) {
                    members.remove(stale);
                }
                dma_buf.members.retain(|m| m.device != self.device);
                dma_buf.members.push(member);
                members.insert(member, inode);
                import.handle
            }
        };
        if dma_buf.own.is_none() {
            dma_buf.own = Some(dma_buf_file);
        }
        args.handle = handle;
        0
    }
}
