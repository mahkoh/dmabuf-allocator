#[cfg(any(
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6",
    target_arch = "sparc",
    target_arch = "sparc64",
))]
mod arch {
    pub const _IOC_SIZEBITS: u32 = 13;
    pub const _IOC_DIRBITS: u32 = 3;
    pub const _IOC_NONE: u32 = 1;
    pub const _IOC_READ: u32 = 2;
    pub const _IOC_WRITE: u32 = 4;
}

#[cfg(not(any(
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6",
    target_arch = "sparc",
    target_arch = "sparc64",
)))]
mod arch {
    pub const _IOC_SIZEBITS: u32 = 14;
    pub const _IOC_DIRBITS: u32 = 2;
    pub const _IOC_NONE: u32 = 0;
    pub const _IOC_WRITE: u32 = 1;
    pub const _IOC_READ: u32 = 2;
}

pub use arch::_IOC_DIRBITS;
pub use arch::_IOC_NONE;
pub use arch::_IOC_READ;
pub use arch::_IOC_SIZEBITS;
pub use arch::_IOC_WRITE;

pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;
pub const _IOC_NRMASK: u32 = (1 << _IOC_NRBITS) - 1;
pub const _IOC_TYPEMASK: u32 = (1 << _IOC_TYPEBITS) - 1;
pub const _IOC_SIZEMASK: u32 = (1 << _IOC_SIZEBITS) - 1;
pub const _IOC_DIRMASK: u32 = (1 << _IOC_DIRBITS) - 1;
pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
pub const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;
pub const IOC_IN: u32 = _IOC_WRITE << _IOC_DIRSHIFT;
pub const IOC_OUT: u32 = _IOC_READ << _IOC_DIRSHIFT;

pub const fn _IOC(dir: u32, ty: u32, nr: u32, size: usize) -> u32 {
    dir << _IOC_DIRSHIFT
        | ty << _IOC_TYPESHIFT
        | nr << _IOC_NRSHIFT
        | (size as u32) << _IOC_SIZESHIFT
}

pub const fn _IO(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_NONE, ty, nr, 0)
}

pub const fn _IOR<T>(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_READ, ty, nr, size_of::<T>())
}

pub const fn _IOW<T>(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_WRITE, ty, nr, size_of::<T>())
}

pub const fn _IOWR<T>(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_READ | _IOC_WRITE, ty, nr, size_of::<T>())
}

pub const fn _IOC_TYPE(cmd: u32) -> u32 {
    (cmd >> _IOC_TYPESHIFT) & _IOC_TYPEMASK
}

pub const fn _IOC_NR(cmd: u32) -> u32 {
    (cmd >> _IOC_NRSHIFT) & _IOC_NRMASK
}

#[cfg(not(any(target_arch = "sparc", target_arch = "sparc64")))]
pub const fn _IOC_SIZE(cmd: u32) -> u32 {
    (cmd >> _IOC_SIZESHIFT) & _IOC_SIZEMASK
}

#[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
pub const fn _IOC_SIZE(cmd: u32) -> u32 {
    const _IOC_XSIZEMASK: u32 = (1 << (_IOC_SIZEBITS + 1)) - 1;
    if (cmd >> _IOC_DIRSHIFT) & _IOC_DIRMASK & (_IOC_WRITE | _IOC_READ) == 0 {
        0
    } else {
        (cmd >> _IOC_SIZESHIFT) & _IOC_XSIZEMASK
    }
}
