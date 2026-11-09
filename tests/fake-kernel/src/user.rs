use bytemuck::AnyBitPattern;
use bytemuck::NoUninit;
use libc::EFAULT;
use libc::ENOMEM;

pub trait UserMemory {
    fn read(&self, address: u64, data: &mut [u8]) -> Result<(), i32>;
    fn write(&self, address: u64, data: &[u8]) -> Result<(), i32>;
}

impl dyn UserMemory + '_ {
    pub fn read_value<T: AnyBitPattern>(&self, address: u64) -> Result<T, i32> {
        let mut bytes = vec![0u8; size_of::<T>()];
        self.read(address, &mut bytes)?;
        Ok(bytemuck::pod_read_unaligned(&bytes))
    }

    pub fn read_array<T: AnyBitPattern>(&self, address: u64, count: usize) -> Result<Vec<T>, i32> {
        let size = count.checked_mul(size_of::<T>()).ok_or(EFAULT)?;
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(size).map_err(|_| ENOMEM)?;
        bytes.resize(size, 0u8);
        self.read(address, &mut bytes)?;
        Ok(bytes
            .chunks_exact(size_of::<T>())
            .map(bytemuck::pod_read_unaligned)
            .collect())
    }

    pub fn write_value<T: NoUninit>(&self, address: u64, value: &T) -> Result<(), i32> {
        self.write(address, bytemuck::bytes_of(value))
    }
}

pub(crate) fn from_bytes<T: AnyBitPattern>(bytes: &[u8]) -> T {
    let mut value = vec![0u8; size_of::<T>()];
    let len = bytes.len().min(value.len());
    value[..len].copy_from_slice(&bytes[..len]);
    bytemuck::pod_read_unaligned(&value)
}

pub(crate) fn put<T: NoUninit>(bytes: &mut [u8], offset: usize, value: T) {
    bytes[offset..][..size_of::<T>()].copy_from_slice(bytemuck::bytes_of(&value));
}

pub struct LocalMemory {
    _private: (),
}

impl LocalMemory {
    pub unsafe fn new() -> Self {
        LocalMemory { _private: () }
    }
}

impl UserMemory for LocalMemory {
    fn read(&self, address: u64, data: &mut [u8]) -> Result<(), i32> {
        if data.is_empty() {
            return Ok(());
        }
        unsafe {
            data.as_mut_ptr()
                .copy_from_nonoverlapping(address as usize as *const u8, data.len());
        }
        Ok(())
    }

    fn write(&self, address: u64, data: &[u8]) -> Result<(), i32> {
        if data.is_empty() {
            return Ok(());
        }
        unsafe {
            (address as usize as *mut u8).copy_from_nonoverlapping(data.as_ptr(), data.len());
        }
        Ok(())
    }
}

pub struct AlignedBytes {
    words: Vec<u64>,
    len: usize,
}

impl AlignedBytes {
    pub fn zeroed(len: usize) -> Self {
        Self {
            words: vec![0u64; len.div_ceil(size_of::<u64>())],
            len,
        }
    }

    pub fn new(bytes: &[u8]) -> Self {
        let mut aligned = Self::zeroed(bytes.len());
        aligned.bytes_mut().copy_from_slice(bytes);
        aligned
    }

    pub fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.words.as_ptr().cast(), self.len) }
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.words.as_mut_ptr().cast(), self.len) }
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.words.as_mut_ptr().cast()
    }
}
