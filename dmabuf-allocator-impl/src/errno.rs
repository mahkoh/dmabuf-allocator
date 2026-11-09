use core::ffi::CStr;
use core::fmt;
use core::fmt::Display;
use thiserror::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Error)]
#[error("{}", description(*.0))]
pub struct Errno(pub i32);

fn description(errno: i32) -> impl Display {
    fmt::from_fn(move |f| {
        let mut buffer = [0u8; 256];
        let ret = unsafe { libc::strerror_r(errno, buffer.as_mut_ptr().cast(), buffer.len()) };
        let description = match ret {
            0 => CStr::from_bytes_until_nul(&buffer)
                .ok()
                .and_then(|description| description.to_str().ok()),
            _ => None,
        };
        match description {
            Some(description) => f.write_str(description),
            None => write!(f, "errno {errno}"),
        }
    })
}

pub(crate) fn errno() -> Errno {
    Errno(unsafe { *libc::__errno_location() })
}
