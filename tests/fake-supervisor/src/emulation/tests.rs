use super::*;
use std::sync::mpsc;

fn supervisor() -> Supervisor {
    let listener = unsafe { libc::eventfd(0, libc::EFD_CLOEXEC) };
    assert!(listener >= 0);
    Supervisor::new(std::process::id(), unsafe {
        OwnedFd::from_raw_fd(listener)
    })
}

fn tid() -> u32 {
    unsafe { libc::gettid() as u32 }
}

fn wait_for_exit(pidfd: &OwnedFd) {
    let mut poll = pollfd {
        fd: pidfd.as_raw_fd(),
        events: POLLIN,
        revents: 0,
    };
    assert_eq!(unsafe { libc::poll(&mut poll, 1, 5000) }, 1);
}

#[test]
fn malformed_requests_get_an_error_reply() {
    let supervisor = supervisor();
    assert_eq!(supervisor.control("bogus"), "error unknown request\n");
    assert_eq!(
        supervisor.control("visible"),
        "error the request names no thread\n"
    );
    assert_eq!(
        supervisor.control("record x"),
        "error \"x\" is not a thread id\n"
    );
    assert_eq!(
        supervisor.control(&format!("visible {}", tid())),
        "error the request names no devices\n"
    );
    assert_eq!(
        supervisor.control(&format!("visible {} 0 x", tid())),
        "error \"x\" is not a number\n"
    );
}

#[test]
fn recording_starts_and_stops() {
    let supervisor = supervisor();
    let tid = tid();
    assert_eq!(supervisor.control(&format!("stop {tid}")), "0\n");
    assert_eq!(supervisor.control(&format!("record {tid}")), "ok\n");
    let recording = supervisor.with_thread(tid, |thread| thread.recording.is_some());
    assert_eq!(recording, Some(true));
    assert_eq!(supervisor.control(&format!("stop {tid}")), "0\n");
    let recording = supervisor.with_thread(tid, |thread| thread.recording.is_some());
    assert_eq!(recording, Some(false));
}

#[test]
fn state_of_exited_threads_is_dropped() {
    let supervisor = supervisor();
    let (send_tid, receive_tid) = mpsc::channel();
    let (send_exit, receive_exit) = mpsc::channel::<()>();
    let thread = std::thread::spawn(move || {
        send_tid.send(tid()).unwrap();
        let _ = receive_exit.recv();
    });
    let thread_tid = receive_tid.recv().unwrap();
    let reply = supervisor.control(&format!("visible {thread_tid} 0 1"));
    assert_eq!(reply, "ok\n");
    let pidfd = supervisor
        .with_thread(thread_tid, |thread| thread.pidfd.clone())
        .unwrap();
    assert!(!exited(&pidfd));
    drop(send_exit);
    thread.join().unwrap();
    wait_for_exit(&pidfd);
    assert!(exited(&pidfd));
    supervisor.with_thread(tid(), |_| ());
    assert!(!supervisor.threads.lock().contains_key(&thread_tid));
}

#[test]
fn registration_returns_consecutive_ranges() {
    let supervisor = supervisor();
    let ids = ["vkms:kernel=7.2"; 3].join(" ");
    assert_eq!(supervisor.control(&format!("register {ids}")), "0 3\n");
    assert_eq!(supervisor.control(&format!("register {ids}")), "3 6\n");
}
