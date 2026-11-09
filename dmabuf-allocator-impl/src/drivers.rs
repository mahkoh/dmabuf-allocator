use crate::allocator::Kernel;
use crate::errno::Errno;
use crate::fd::OwnedFd;
use alloc::sync::Arc;

pub(crate) mod amd;
pub(crate) mod asahi;
pub(crate) mod dumb;
pub(crate) mod etnaviv;
pub mod intel;
pub(crate) mod lima;
pub(crate) mod nvidia;
pub(crate) mod panfrost;
pub(crate) mod udmabuf;
pub(crate) mod v3d;
pub(crate) mod vc4;
pub(crate) mod virtio_gpu;

fn open_render_node<K: Kernel>(fd: &OwnedFd<K>) -> Result<Arc<OwnedFd<K>>, Errno> {
    fd.kernel
        .open_render_node(fd.fd)
        .map(|v| OwnedFd::new(&fd.kernel, v))
        .map(Arc::new)
}
