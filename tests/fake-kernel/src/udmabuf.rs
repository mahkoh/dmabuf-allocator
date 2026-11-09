use crate::Drm;
use crate::Kernel;
use crate::KernelVersion;
use crate::PAGE_SHIFT;
use crate::PAGE_SIZE;
use crate::Prime;
use crate::UserMemory;
use crate::id_fields;
use crate::uapi::udmabuf::UDMABUF_CREATE;
use crate::uapi::udmabuf::UDMABUF_CREATE_LIST;
use crate::uapi::udmabuf::UDMABUF_FLAGS_CLOEXEC;
use crate::uapi::udmabuf::udmabuf_create;
use libc::EBADFD;
use libc::EFAULT;
use libc::EINVAL;
use libc::ENOTTY;
use libc::F_SEAL_FUTURE_WRITE;
use libc::F_SEAL_SHRINK;
use libc::F_SEAL_WRITE;

const SIZE_LIMIT_MB: u64 = 64;
const SEALS_WANTED: i32 = F_SEAL_SHRINK;
const SEALS_DENIED: i32 = F_SEAL_WRITE | F_SEAL_FUTURE_WRITE;

pub struct Device {
    kernel: KernelVersion,
}

impl Device {
    pub fn new(kernel: KernelVersion) -> Self {
        Self { kernel }
    }

    pub fn from_id(fields: &str) -> Self {
        let values = id_fields(fields, &["kernel"]);
        Self::new(KernelVersion::parse(values.get("kernel")))
    }

    fn create(&self, arg: u64, user: &dyn UserMemory, prime: &mut dyn Prime) -> i64 {
        let Ok(create) = user.read_value::<udmabuf_create>(arg) else {
            return -(EFAULT as i64);
        };
        if !create.offset.is_multiple_of(PAGE_SIZE) || !create.size.is_multiple_of(PAGE_SIZE) {
            return -(EINVAL as i64);
        }
        let pages = create.size >> PAGE_SHIFT;
        let limit = (SIZE_LIMIT_MB * 1024 * 1024) >> PAGE_SHIFT;
        if pages > limit || pages == 0 {
            return -(EINVAL as i64);
        }
        let Some(memfd) = prime.memfd(create.memfd as i32) else {
            return -(EBADFD as i64);
        };
        if memfd.seals & SEALS_WANTED != SEALS_WANTED || memfd.seals & SEALS_DENIED != 0 {
            return -(EINVAL as i64);
        }
        let end = create.offset.checked_add(create.size);
        if end.is_none_or(|end| end > memfd.size) {
            return -(EINVAL as i64);
        }
        if create.flags & UDMABUF_FLAGS_CLOEXEC == 0 {
            unimplemented!("udmabuf dma-bufs without O_CLOEXEC");
        }
        prime.dmabuf_fd() as i64
    }
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "udmabuf"
    }

    fn id(&self) -> String {
        format!("udmabuf:kernel={}", self.kernel)
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn render_node(&self) -> bool {
        false
    }

    fn drm(&self) -> Option<&dyn Drm> {
        None
    }

    fn misc_ioctl(
        &self,
        cmd: u32,
        arg: u64,
        user: &dyn UserMemory,
        prime: &mut dyn Prime,
    ) -> Option<i64> {
        match cmd {
            UDMABUF_CREATE => Some(self.create(arg, user, prime)),
            UDMABUF_CREATE_LIST => unimplemented!("UDMABUF_CREATE_LIST"),
            _ => Some(-(ENOTTY as i64)),
        }
    }
}
