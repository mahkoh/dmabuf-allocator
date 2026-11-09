#![allow(clippy::missing_safety_doc)]
#![no_std]

extern crate alloc;

pub mod allocator;
pub mod drivers;
mod drm;
pub mod errno;
pub mod fd;
pub mod format;
pub mod formats;
mod kernel;
pub mod layout;
mod mutex;
mod sys;
mod vdrm;
