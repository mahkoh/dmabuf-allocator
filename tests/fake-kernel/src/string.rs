use crate::UserMemory;
use libc::EFAULT;
use libc::EINVAL;

pub(crate) trait Char: Copy {
    fn from_byte(byte: u8) -> Self;
}

impl Char for u8 {
    fn from_byte(byte: u8) -> Self {
        byte
    }
}

impl Char for i8 {
    fn from_byte(byte: u8) -> Self {
        byte as i8
    }
}

pub(crate) fn strscpy<C: Char>(dst: &mut [C], src: &str) -> usize {
    let len = src.len().min(dst.len() - 1);
    for (dst, &src) in dst[..len].iter_mut().zip(src.as_bytes()) {
        *dst = C::from_byte(src);
    }
    dst[len] = C::from_byte(0);
    len
}

pub(crate) fn strscpy_pad<C: Char>(dst: &mut [C], src: &str) {
    let len = strscpy(dst, src);
    dst[len..].fill(C::from_byte(0));
}

pub(crate) fn strndup_user(user: &dyn UserMemory, address: u64, max: u64) -> Result<Vec<u8>, i32> {
    let mut string = Vec::new();
    for index in 0..max {
        let mut byte = [0u8];
        user.read(address + index, &mut byte).map_err(|_| EFAULT)?;
        if byte[0] == 0 {
            return Ok(string);
        }
        string.push(byte[0]);
    }
    Err(EINVAL)
}
