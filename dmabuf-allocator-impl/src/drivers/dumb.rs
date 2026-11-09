use crate::allocator::Config;
use crate::drm;
use crate::drm::DrmError;
use crate::fd::OwnedFd;
use crate::kernel::Kernel;
use crate::layout::DRM_FORMAT_MOD_LINEAR;
use crate::layout::Layout;
use crate::layout::PlaneLayout;
use crate::layout::PlaneVec;
use core::ffi::c_int;
use run_on_drop::on_drop;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DumbError {
    #[error("only linear layouts are supported")]
    Modifier,
    #[error("the format is not supported")]
    Format,
    #[error("the size is not supported")]
    Size,
    #[error("could not create the buffer")]
    Create(#[source] DrmError),
    #[error("the plane is larger than the buffer")]
    PlaneTooBig,
    #[error("could not export the buffer")]
    Export(#[source] DrmError),
}

const MAX_EXTENT: u32 = 16384;
const EXTENT_ALIGNMENT: u32 = 64;

pub(crate) struct DumbAllocation {
    pub(crate) fd: c_int,
    pub(crate) layout: Layout,
}

pub(crate) fn allocate<K: Kernel>(
    dev: &OwnedFd<K>,
    config: &Config<'_>,
) -> Result<DumbAllocation, DumbError> {
    let valid_size = |n| (1..=MAX_EXTENT).contains(&n);
    if !valid_size(config.width) || !valid_size(config.height) {
        return Err(DumbError::Size);
    }
    let format = config.format.0;
    if !format.dumb {
        return Err(DumbError::Format);
    }
    if !config.modifiers.contains(&DRM_FORMAT_MOD_LINEAR) {
        return Err(DumbError::Modifier);
    }
    let height = config.height.next_multiple_of(EXTENT_ALIGNMENT);
    let plane = &format.planes[0];
    let buf = drm::create_dumb(
        dev,
        config.width.next_multiple_of(EXTENT_ALIGNMENT),
        height,
        plane.bits,
    )
    .map_err(DumbError::Create)?;
    let _close = on_drop(|| drm::gem_close(dev, buf.handle));
    let stride = buf.pitch as u64;
    let size = height.div_ceil(plane.block_height) as u64 * stride;
    if size > buf.size as u32 as u64 {
        return Err(DumbError::PlaneTooBig);
    }
    Ok(DumbAllocation {
        fd: drm::handle_to_fd(dev, buf.handle).map_err(DumbError::Export)?,
        layout: Layout {
            format,
            modifier: DRM_FORMAT_MOD_LINEAR,
            planes: PlaneVec::from_iter([PlaneLayout {
                offset: 0,
                stride,
                size,
            }]),
        },
    })
}
