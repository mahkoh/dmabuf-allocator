use std::fs::File;
use std::mem::offset_of;
use std::os::fd::OwnedFd;
use std::os::unix::fs::FileExt;
use std::ptr;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pair {
    pub fourcc: u32,
    pub modifier: u64,
}

pub struct Tranche {
    pub target_device: u64,
    pub scanout: bool,
    pub pairs: Vec<Pair>,
}

pub struct Feedback {
    pub main_device: u64,
    pub tranches: Vec<Tranche>,
}

#[derive(Default)]
pub struct FeedbackBuilder {
    table: Vec<Pair>,
    main_device: u64,
    tranches: Vec<Tranche>,
    target_device: u64,
    scanout: bool,
    pairs: Vec<Pair>,
}

#[repr(C)]
struct FormatTableEntry {
    format: u32,
    padding: u32,
    modifier: u64,
}

const FORMAT_TABLE_ENTRY_SIZE: usize = 16;

const _: () = assert!(size_of::<FormatTableEntry>() == FORMAT_TABLE_ENTRY_SIZE);
const _: () = assert!(offset_of!(FormatTableEntry, format) == 0);
const _: () = assert!(offset_of!(FormatTableEntry, padding) == 4);
const _: () = assert!(offset_of!(FormatTableEntry, modifier) == 8);

fn dev_t(bytes: &[u8]) -> u64 {
    u64::from_ne_bytes(bytes.try_into().expect("dev_t has 8 bytes"))
}

impl FeedbackBuilder {
    pub fn format_table(&mut self, fd: OwnedFd, size: u32) {
        let mut bytes = vec![0; size as usize];
        File::from(fd)
            .read_exact_at(&mut bytes, 0)
            .expect("could not read the format table");
        self.table = bytes
            .as_chunks::<FORMAT_TABLE_ENTRY_SIZE>()
            .0
            .iter()
            .map(|bytes| {
                let entry: FormatTableEntry = unsafe { ptr::read_unaligned(bytes.as_ptr().cast()) };
                Pair {
                    fourcc: entry.format,
                    modifier: entry.modifier,
                }
            })
            .collect();
    }

    pub fn main_device(&mut self, device: &[u8]) {
        self.main_device = dev_t(device);
    }

    pub fn tranche_target_device(&mut self, device: &[u8]) {
        self.target_device = dev_t(device);
    }

    pub fn tranche_formats(&mut self, indices: &[u8]) {
        for &index in indices.as_chunks::<2>().0 {
            let index = u16::from_ne_bytes(index) as usize;
            if let Some(&pair) = self.table.get(index) {
                self.pairs.push(pair);
            }
        }
    }

    pub fn tranche_scanout(&mut self, scanout: bool) {
        self.scanout = scanout;
    }

    pub fn tranche_done(&mut self) {
        self.tranches.push(Tranche {
            target_device: self.target_device,
            scanout: self.scanout,
            pairs: std::mem::take(&mut self.pairs),
        });
        self.scanout = false;
    }

    pub fn done(&mut self) -> Feedback {
        Feedback {
            main_device: self.main_device,
            tranches: std::mem::take(&mut self.tranches),
        }
    }
}
