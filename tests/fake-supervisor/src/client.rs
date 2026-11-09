use crate::SOCKET_VARIABLE;
use crate::receive_fd;
use allocator_trace::Traced;
use parking_lot::Mutex;
use std::cell::RefCell;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::ops::Range;
use std::os::fd::FromRawFd;
use std::os::unix::net::UnixStream;
use std::sync::LazyLock;

type Connection = BufReader<UnixStream>;

static SOCKET: LazyLock<Mutex<UnixStream>> = LazyLock::new(|| {
    let fd: i32 = std::env::var(SOCKET_VARIABLE)
        .ok()
        .and_then(|fd| fd.parse().ok())
        .unwrap_or_else(|| panic!("{SOCKET_VARIABLE} does not name the control socket"));
    assert_eq!(
        unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
        0
    );
    Mutex::new(unsafe { UnixStream::from_raw_fd(fd) })
});

thread_local! {
    static CONNECTION: RefCell<Option<Connection>> = const { RefCell::new(None) };
}

pub fn is_worker() -> bool {
    std::env::var_os(SOCKET_VARIABLE).is_some()
}

fn connect() -> Connection {
    let mut socket = SOCKET.lock();
    socket
        .write_all(b"connect\n")
        .expect("the supervisor closed the control socket");
    let connection = receive_fd(&socket).expect("the supervisor sent no connection");
    BufReader::new(UnixStream::from(connection))
}

fn with_connection<T>(f: impl FnOnce(&mut Connection) -> T) -> T {
    CONNECTION.with_borrow_mut(|connection| f(connection.get_or_insert_with(connect)))
}

fn read_line() -> String {
    let mut line = String::new();
    let read = with_connection(|connection| connection.read_line(&mut line));
    assert!(
        read.is_ok_and(|len| len > 0),
        "the supervisor closed the control connection"
    );
    line.truncate(line.trim_end_matches('\n').len());
    line
}

fn request(line: &str) -> String {
    with_connection(|connection| {
        connection
            .get_mut()
            .write_all(format!("{line}\n").as_bytes())
    })
    .expect("the supervisor closed the control connection");
    let reply = read_line();
    if let Some(error) = reply.strip_prefix("error ") {
        panic!("the supervisor rejected {line:?}: {error}");
    }
    reply
}

pub fn register(ids: &[String]) -> Range<usize> {
    let mut line = "register".to_string();
    for id in ids {
        assert!(
            !id.is_empty() && !id.contains([' ', '\n']),
            "the fake kernel id {id:?} cannot be sent to the supervisor"
        );
        line.push(' ');
        line.push_str(id);
    }
    let reply = request(&line);
    let (start, end) = reply.split_once(' ').expect("invalid register reply");
    let range: Range<usize> = start.parse().unwrap()..end.parse().unwrap();
    assert_eq!(range.len(), ids.len(), "invalid register reply {reply:?}");
    range
}

pub fn set_visible(range: Option<Range<usize>>) {
    let tid = unsafe { libc::gettid() };
    let line = match range {
        Some(range) => format!("visible {tid} {} {}", range.start, range.end),
        None => format!("visible {tid} all"),
    };
    assert_eq!(request(&line), "ok");
}

pub fn start_recording() {
    let tid = unsafe { libc::gettid() };
    assert_eq!(request(&format!("record {tid}")), "ok");
}

pub fn stop_recording() -> Vec<Traced> {
    let tid = unsafe { libc::gettid() };
    let count: usize = request(&format!("stop {tid}"))
        .parse()
        .expect("invalid stop reply");
    (0..count)
        .map(|_| {
            let line = read_line();
            let (handle, call) = line.split_once(' ').expect("invalid traced call");
            Traced {
                handle: match handle {
                    "none" => None,
                    handle => Some(handle.parse().expect("invalid traced handle")),
                },
                call: call
                    .parse()
                    .unwrap_or_else(|e| panic!("invalid traced call {call:?}: {e}")),
            }
        })
        .collect()
}
