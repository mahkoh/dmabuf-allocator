use crate::emulation::Supervisor;
use fake_kernel::uapi::drm::DRM_IOCTL_BASE;
use fake_kernel::uapi::ioctl::_IOC_TYPEMASK;
use fake_kernel::uapi::ioctl::_IOC_TYPESHIFT;
use libc::EINTR;
use libc::ENOSYS;
use libc::F_GETFL;
use libc::F_SETFD;
use libc::F_SETFL;
use libc::MSG_CMSG_CLOEXEC;
use libc::MSG_CTRUNC;
use libc::PR_SET_NO_NEW_PRIVS;
use libc::PR_SET_PDEATHSIG;
use libc::SCM_RIGHTS;
use libc::SECCOMP_FILTER_FLAG_NEW_LISTENER;
use libc::SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV;
use libc::SECCOMP_RET_ALLOW;
use libc::SECCOMP_RET_ERRNO;
use libc::SECCOMP_RET_KILL_PROCESS;
use libc::SECCOMP_RET_USER_NOTIF;
use libc::SECCOMP_SET_MODE_FILTER;
use libc::SIGKILL;
use libc::SOL_SOCKET;
use libc::SYS_access;
use libc::SYS_creat;
use libc::SYS_faccessat;
use libc::SYS_faccessat2;
use libc::SYS_fcntl;
use libc::SYS_fstat;
use libc::SYS_getdents64;
use libc::SYS_io_uring_setup;
use libc::SYS_ioctl;
use libc::SYS_lstat;
use libc::SYS_newfstatat;
use libc::SYS_open;
use libc::SYS_open_tree;
use libc::SYS_openat;
use libc::SYS_openat2;
use libc::SYS_readlink;
use libc::SYS_readlinkat;
use libc::SYS_seccomp;
use libc::SYS_stat;
use libc::SYS_statx;
use libc::c_int;
use libc::cmsghdr;
use libc::iovec;
use libc::msghdr;
use libc::seccomp_data;
use libc::sock_filter;
use libc::sock_fprog;
use parking_lot::Mutex;
use std::cell::Cell;
use std::ffi::OsString;
use std::io;
use std::mem::offset_of;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::OwnedFd;
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt;
use std::panic::PanicHookInfo;
use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;
use std::sync::Arc;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("the seccomp filter and the syscall numbers are those of x86_64");

pub mod client;
mod emulation;
mod memory;
mod node;

pub use node::DrmNode;
pub use node::MAX_DEVICES;

pub(crate) const SOCKET_VARIABLE: &str = "FAKE_SUPERVISOR_SOCKET";

const IOCTL_TYPE_MASK: u32 = _IOC_TYPEMASK << _IOC_TYPESHIFT;
const DRM_IOCTL_TYPE: u32 = DRM_IOCTL_BASE << _IOC_TYPESHIFT;
const AUDIT_ARCH_X86_64: u32 = 0xc000_003e;
const COMMAND: u32 = (offset_of!(seccomp_data, args) + size_of::<u64>()) as u32;

const X32_SYSCALL_BIT: u32 = 0x4000_0000;

const NOTIFIED: [i64; 14] = [
    SYS_fstat,
    SYS_stat,
    SYS_lstat,
    SYS_newfstatat,
    SYS_statx,
    SYS_open,
    SYS_creat,
    SYS_openat,
    SYS_readlink,
    SYS_readlinkat,
    SYS_getdents64,
    SYS_access,
    SYS_faccessat,
    SYS_faccessat2,
];

const UNSUPPORTED: [i64; 3] = [SYS_openat2, SYS_open_tree, SYS_io_uring_setup];

const PANIC_STATUS: i32 = 101;

type PanicHook = Box<dyn Fn(&PanicHookInfo<'_>) + Send + Sync>;

thread_local! {
    static SUPERVISOR_THREAD: Cell<bool> = const { Cell::new(false) };
}

pub(crate) fn spawn(thread: impl FnOnce() + Send + 'static) {
    std::thread::spawn(move || {
        SUPERVISOR_THREAD.set(true);
        thread();
    });
}

const fn statement(code: u16, k: u32) -> sock_filter {
    sock_filter {
        code,
        jt: 0,
        jf: 0,
        k,
    }
}

const fn jump(condition: u32, k: u32, jt: u8, jf: u8) -> sock_filter {
    sock_filter {
        code: (libc::BPF_JMP | condition | libc::BPF_K) as u16,
        jt,
        jf,
        k,
    }
}

fn filter() -> Vec<sock_filter> {
    let load = (libc::BPF_LD | libc::BPF_W | libc::BPF_ABS) as u16;
    let ret = (libc::BPF_RET | libc::BPF_K) as u16;
    let and = (libc::BPF_ALU | libc::BPF_AND | libc::BPF_K) as u16;
    let nr = offset_of!(seccomp_data, nr) as u32;
    let drm_ioctl = [
        statement(load, COMMAND),
        statement(and, IOCTL_TYPE_MASK),
        jump(libc::BPF_JEQ, DRM_IOCTL_TYPE, 0, 1),
        statement(ret, SECCOMP_RET_USER_NOTIF),
        statement(ret, SECCOMP_RET_ALLOW),
    ];
    let fcntl = [
        statement(load, COMMAND),
        jump(libc::BPF_JEQ, F_GETFL as u32, 1, 0),
        jump(libc::BPF_JEQ, F_SETFL as u32, 0, 1),
        statement(ret, SECCOMP_RET_USER_NOTIF),
        statement(ret, SECCOMP_RET_ALLOW),
    ];
    let mut filter = vec![
        statement(load, offset_of!(seccomp_data, arch) as u32),
        jump(libc::BPF_JEQ, AUDIT_ARCH_X86_64, 1, 0),
        statement(ret, SECCOMP_RET_KILL_PROCESS),
        statement(load, nr),
        jump(libc::BPF_JGE, X32_SYSCALL_BIT, 0, 1),
        statement(ret, SECCOMP_RET_KILL_PROCESS),
        jump(libc::BPF_JEQ, SYS_ioctl as u32, 0, drm_ioctl.len() as u8),
    ];
    filter.extend(drm_ioctl);
    filter.push(jump(libc::BPF_JEQ, SYS_fcntl as u32, 0, fcntl.len() as u8));
    filter.extend(fcntl);
    for call in NOTIFIED {
        filter.extend([
            jump(libc::BPF_JEQ, call as u32, 0, 1),
            statement(ret, SECCOMP_RET_USER_NOTIF),
        ]);
    }
    for call in UNSUPPORTED {
        filter.extend([
            jump(libc::BPF_JEQ, call as u32, 0, 1),
            statement(ret, SECCOMP_RET_ERRNO | ENOSYS as u32),
        ]);
    }
    filter.push(statement(ret, SECCOMP_RET_ALLOW));
    filter
}

pub fn supervise(program: &Path, args: &[OsString]) -> ExitStatus {
    let filter = filter();
    let (control, worker_socket) = UnixStream::pair().expect("socketpair failed");
    let worker_fd = worker_socket.as_raw_fd();
    let mut command = Command::new(program);
    command
        .args(args)
        .env(SOCKET_VARIABLE, worker_fd.to_string());
    let supervisor_pid = std::process::id() as libc::pid_t;
    unsafe {
        command.pre_exec(move || install(&filter, worker_fd, supervisor_pid));
    }
    let mut child = command.spawn().expect("cannot start the worker");
    drop(worker_socket);
    let listener = receive_fd(&control).expect("the worker sent no seccomp listener");
    let supervisor = Arc::new(Supervisor::new(child.id(), listener));
    let previous_hook: Arc<Mutex<Option<PanicHook>>> =
        Arc::new(Mutex::new(Some(std::panic::take_hook())));
    let hook = previous_hook.clone();
    let worker = supervisor.clone();
    std::panic::set_hook(Box::new(move |info| {
        if let Some(previous) = &*hook.lock() {
            previous(info);
        }
        if SUPERVISOR_THREAD.get() {
            worker.kill_worker();
            std::process::exit(PANIC_STATUS);
        }
    }));
    supervisor.start(control);
    let status = child.wait().expect("waiting for the worker failed");
    drop(std::panic::take_hook());
    if let Some(previous) = previous_hook.lock().take() {
        std::panic::set_hook(previous);
    }
    status
}

fn install(filter: &[sock_filter], socket: c_int, supervisor: libc::pid_t) -> io::Result<()> {
    let program = sock_fprog {
        len: filter.len() as u16,
        filter: filter.as_ptr().cast_mut(),
    };
    unsafe {
        if libc::prctl(PR_SET_PDEATHSIG, SIGKILL, 0, 0, 0) != 0 {
            return Err(io::Error::last_os_error());
        }
        if libc::getppid() != supervisor {
            libc::_exit(1);
        }
        if libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
            return Err(io::Error::last_os_error());
        }
        let flags = SECCOMP_FILTER_FLAG_NEW_LISTENER | SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV;
        let listener = libc::syscall(SYS_seccomp, SECCOMP_SET_MODE_FILTER, flags, &program);
        if listener < 0 {
            return Err(io::Error::last_os_error());
        }
        let sent = send_fd(socket, listener as c_int);
        libc::close(listener as c_int);
        sent?;
        if libc::fcntl(socket, F_SETFD, 0) != 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

#[repr(C)]
union FdMessage {
    header: cmsghdr,
    bytes: [u8; unsafe { libc::CMSG_SPACE(size_of::<c_int>() as u32) as usize }],
}

fn fd_message(data: &mut iovec, control: &mut FdMessage) -> msghdr {
    let mut message: msghdr = unsafe { std::mem::zeroed() };
    message.msg_iov = data;
    message.msg_iovlen = 1;
    message.msg_control = (control as *mut FdMessage).cast();
    message.msg_controllen = size_of::<FdMessage>();
    message
}

fn send_fd(socket: c_int, fd: c_int) -> io::Result<()> {
    let mut byte = 0u8;
    let mut data = iovec {
        iov_base: (&raw mut byte).cast(),
        iov_len: 1,
    };
    let mut control = FdMessage {
        bytes: [0; size_of::<FdMessage>()],
    };
    let message = fd_message(&mut data, &mut control);
    unsafe {
        let header = libc::CMSG_FIRSTHDR(&message);
        (*header).cmsg_level = SOL_SOCKET;
        (*header).cmsg_type = SCM_RIGHTS;
        (*header).cmsg_len = libc::CMSG_LEN(size_of::<c_int>() as u32) as usize;
        libc::CMSG_DATA(header).cast::<c_int>().write_unaligned(fd);
        if libc::sendmsg(socket, &message, 0) != 1 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

fn receive_fd(socket: &UnixStream) -> Option<OwnedFd> {
    let mut byte = 0u8;
    let mut data = iovec {
        iov_base: (&raw mut byte).cast(),
        iov_len: 1,
    };
    let mut control = FdMessage {
        bytes: [0; size_of::<FdMessage>()],
    };
    let mut message = fd_message(&mut data, &mut control);
    unsafe {
        let received = loop {
            let received = libc::recvmsg(socket.as_raw_fd(), &mut message, MSG_CMSG_CLOEXEC);
            let interrupted = io::Error::last_os_error().raw_os_error() == Some(EINTR);
            if received != -1 || !interrupted {
                break received;
            }
        };
        if received != 1 {
            return None;
        }
        let header = libc::CMSG_FIRSTHDR(&message);
        let valid = message.msg_flags & MSG_CTRUNC == 0
            && !header.is_null()
            && (*header).cmsg_level == SOL_SOCKET
            && (*header).cmsg_type == SCM_RIGHTS
            && (*header).cmsg_len == libc::CMSG_LEN(size_of::<c_int>() as u32) as usize;
        if !valid {
            return None;
        }
        let fd = libc::CMSG_DATA(header).cast::<c_int>().read_unaligned();
        Some(OwnedFd::from_raw_fd(fd))
    }
}
