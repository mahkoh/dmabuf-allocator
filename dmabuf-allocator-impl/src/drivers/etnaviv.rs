use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::allocator::Extra;
use crate::drm;
use crate::drm::DRM_COMMAND_BASE;
use crate::drm::DRM_IOCTL_BASE;
use crate::drm::DrmError;
use crate::drm::GemHandle;
use crate::errno::Errno;
use crate::fd::OwnedFd;
use crate::kernel::Kernel;
use crate::layout::DRM_FORMAT_MOD_LINEAR;
use crate::layout::DRM_FORMAT_MOD_VENDOR_VIVANTE;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::PAGE_SIZE;
use crate::layout::PlaneLayout;
use crate::layout::PlaneVec;
use crate::layout::align;
use crate::layout::fourcc_mod_code;
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use alloc::sync::Arc;
use core::ffi::c_int;
use run_on_drop::on_drop;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EtnavivError {
    #[error("there is no 3D GPU")]
    NoGpu,
    #[error("could not create the buffer")]
    GemNew(#[source] Errno),
    #[error("could not export the buffer")]
    Export(#[source] DrmError),
}

const DRM_FORMAT_MOD_VIVANTE_TILED: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_VIVANTE, 1);
const DRM_FORMAT_MOD_VIVANTE_SUPER_TILED: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_VIVANTE, 2);
const DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED: u64 = fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_VIVANTE, 3);
const DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED: u64 =
    fourcc_mod_code(DRM_FORMAT_MOD_VENDOR_VIVANTE, 4);

const ETNAVIV_PARAM_GPU_MODEL: u32 = 0x01;
const ETNAVIV_PARAM_GPU_FEATURES_0: u32 = 0x03;
const ETNAVIV_PARAM_GPU_FEATURES_1: u32 = 0x04;
const ETNAVIV_PARAM_GPU_FEATURES_2: u32 = 0x05;
const ETNAVIV_PARAM_GPU_FEATURES_5: u32 = 0x08;
const ETNAVIV_PARAM_GPU_FEATURES_6: u32 = 0x09;
const ETNAVIV_PARAM_GPU_PIXEL_PIPES: u32 = 0x15;

const CHIP_FEATURES_PIPE_3D: u64 = 0x00000004;
const CHIP_MINOR_FEATURES0_TEXTURE_8K: u64 = 0x00000008;
const CHIP_MINOR_FEATURES0_SUPER_TILED: u64 = 0x00001000;
const CHIP_MINOR_FEATURES1_HALF_FLOAT: u64 = 0x00000800;
const CHIP_MINOR_FEATURES1_TEXTURE_HALIGN: u64 = 0x00100000;
const CHIP_MINOR_FEATURES1_HALTI0: u64 = 0x00800000;
const CHIP_MINOR_FEATURES4_SINGLE_BUFFER: u64 = 0x00000040;
const CHIP_MINOR_FEATURES4_HALTI2: u64 = 0x00010000;
const CHIP_MINOR_FEATURES5_HALTI5: u64 = 0x20000000;
const CHIP_MINOR_FEATURES5_BLT_ENGINE: u64 = 0x80000000;

const ETNA_BO_WC: u32 = 0x00020000;

const ETNA_MAX_PIXELPIPES: u64 = 2;
const ETNA_MAX_PIPES: u32 = 4;

const MAX_SIZE: u32 = 2048;
const MAX_SIZE_TEXTURE_8K: u32 = 8192;
const TILE: u64 = 4;
const SUPER_TILE: u64 = 64;
const RS_ALIGNMENT: u64 = 16;
const ETNA_PE_ALIGNMENT: u64 = 64;
const MAX_BUCKET_SIZE: u64 = 112 << 20;
const BUCKET_STEPS_LOG2: u32 = 2;

#[repr(C)]
struct drm_etnaviv_param {
    pipe: u32,
    param: u32,
    value: u64,
}

#[repr(C)]
struct drm_etnaviv_gem_new {
    size: u64,
    flags: u32,
    handle: GemHandle,
}

#[allow(clippy::identity_op)]
const DRM_IOCTL_ETNAVIV_GET_PARAM: Ioctl =
    _IOWR::<drm_etnaviv_param>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x00);
const DRM_IOCTL_ETNAVIV_GEM_NEW: Ioctl =
    _IOWR::<drm_etnaviv_gem_new>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x02);

pub(crate) enum Feature {
    HalfFloat,
    Halti0,
    Halti2,
    Halti5,
}

pub(crate) struct EtnavivFormat {
    pub(crate) render: Option<&'static [Feature]>,
    pub(crate) sample: Option<&'static [Feature]>,
    pub(crate) emulated_128bit: bool,
}

struct Candidate {
    modifier: u64,
    supported: bool,
    align_x: u64,
    align_y: u64,
}

pub(crate) struct Device<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
    minor_features0: u64,
    minor_features1: u64,
    minor_features4: u64,
    minor_features5: u64,
    pixel_pipes: u64,
}

impl<K: Kernel> Device<K> {
    pub(crate) fn new(fd: &Arc<OwnedFd<K>>) -> Result<Self, EtnavivError> {
        let get_param = |pipe, param| {
            let mut req = drm_etnaviv_param {
                pipe,
                param,
                value: 0,
            };
            unsafe {
                drm::ioctl(fd, DRM_IOCTL_ETNAVIV_GET_PARAM, &mut req).map_or(0, |_| req.value)
            }
        };
        let core = (0..ETNA_MAX_PIPES)
            .take_while(|&core| get_param(core, ETNAVIV_PARAM_GPU_MODEL) != 0)
            .find(|&core| {
                get_param(core, ETNAVIV_PARAM_GPU_FEATURES_0) & CHIP_FEATURES_PIPE_3D != 0
            })
            .ok_or(EtnavivError::NoGpu)?;
        Ok(Self {
            fd: fd.clone(),
            minor_features0: get_param(core, ETNAVIV_PARAM_GPU_FEATURES_1),
            minor_features1: get_param(core, ETNAVIV_PARAM_GPU_FEATURES_2),
            minor_features4: get_param(core, ETNAVIV_PARAM_GPU_FEATURES_5),
            minor_features5: get_param(core, ETNAVIV_PARAM_GPU_FEATURES_6),
            pixel_pipes: get_param(core, ETNAVIV_PARAM_GPU_PIXEL_PIPES).min(ETNA_MAX_PIXELPIPES),
        })
    }

    fn supports(&self, features: Option<&[Feature]>) -> bool {
        features.is_some_and(|features| {
            features.iter().all(|feature| match feature {
                Feature::HalfFloat => self.minor_features1 & CHIP_MINOR_FEATURES1_HALF_FLOAT != 0,
                Feature::Halti0 => self.minor_features1 & CHIP_MINOR_FEATURES1_HALTI0 != 0,
                Feature::Halti2 => self.minor_features4 & CHIP_MINOR_FEATURES4_HALTI2 != 0,
                Feature::Halti5 => self.minor_features5 & CHIP_MINOR_FEATURES5_HALTI5 != 0,
            })
        })
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        let fmt = config.format.0;
        let etnaviv = fmt.etnaviv.as_ref().ok_or(LayoutError::Format)?;
        let max_size = if self.minor_features0 & CHIP_MINOR_FEATURES0_TEXTURE_8K != 0 {
            MAX_SIZE_TEXTURE_8K
        } else {
            MAX_SIZE
        };
        if !(1..=max_size).contains(&config.width) || !(1..=max_size).contains(&config.height) {
            return Err(LayoutError::Size);
        }
        let render_target = self.supports(etnaviv.render);
        if !render_target && !self.supports(etnaviv.sample) {
            return Err(LayoutError::Format);
        }
        let plane = &fmt.planes[0];
        let block_size = (plane.bits as u64 / 8).max(1);
        let blt_engine = self.minor_features5 & CHIP_MINOR_FEATURES5_BLT_ENGINE != 0;
        let supertile = self.minor_features0 & CHIP_MINOR_FEATURES0_SUPER_TILED != 0
            && (blt_engine || matches!(block_size, 2 | 4));
        let pipes = self.pixel_pipes;
        let linear_only = config.implicit_interop;
        let tiled = !linear_only
            && (pipes < 2 || self.minor_features4 & CHIP_MINOR_FEATURES4_SINGLE_BUFFER != 0);
        let split = !linear_only && pipes > 1;
        let rs_align = if !blt_engine
            && (render_target || self.minor_features1 & CHIP_MINOR_FEATURES1_TEXTURE_HALIGN != 0)
        {
            RS_ALIGNMENT
        } else {
            TILE
        };
        let candidate = [
            Candidate {
                modifier: DRM_FORMAT_MOD_VIVANTE_SUPER_TILED,
                supported: tiled && supertile,
                align_x: SUPER_TILE,
                align_y: SUPER_TILE,
            },
            Candidate {
                modifier: DRM_FORMAT_MOD_VIVANTE_TILED,
                supported: tiled,
                align_x: rs_align,
                align_y: TILE,
            },
            Candidate {
                modifier: DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED,
                supported: split && supertile,
                align_x: SUPER_TILE,
                align_y: SUPER_TILE * pipes,
            },
            Candidate {
                modifier: DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED,
                supported: split,
                align_x: RS_ALIGNMENT,
                align_y: TILE * pipes,
            },
            Candidate {
                modifier: DRM_FORMAT_MOD_LINEAR,
                supported: true,
                align_x: rs_align,
                align_y: if blt_engine { 1 } else { TILE },
            },
        ]
        .into_iter()
        .find(|c| c.supported && config.modifiers.contains(&c.modifier))
        .ok_or(LayoutError::Modifier)?;
        let width = align(config.width as u64, candidate.align_x);
        let height = align(config.height as u64, candidate.align_y);
        let stride = width.div_ceil(plane.block_width as u64) * block_size;
        Ok(DeviceLayout {
            layout: Layout {
                format: fmt,
                modifier: candidate.modifier,
                planes: PlaneVec::from_iter([PlaneLayout {
                    offset: 0,
                    stride: stride >> etnaviv.emulated_128bit as u32,
                    size: align(
                        stride * height.div_ceil(plane.block_height as u64),
                        ETNA_PE_ALIGNMENT,
                    ),
                }]),
            },
            extra: Extra::None,
        })
    }

    pub(crate) fn allocate(&self, device_layout: &DeviceLayout) -> Result<c_int, EtnavivError> {
        let size = align(device_layout.layout.planes[0].size, PAGE_SIZE);
        let mut req = drm_etnaviv_gem_new {
            size: if size > MAX_BUCKET_SIZE {
                size
            } else {
                align(size, 1 << (size.ilog2() - BUCKET_STEPS_LOG2))
            },
            flags: ETNA_BO_WC,
            handle: GemHandle(0),
        };
        unsafe {
            drm::ioctl(&self.fd, DRM_IOCTL_ETNAVIV_GEM_NEW, &mut req)
                .map_err(EtnavivError::GemNew)?;
        }
        let _close = on_drop(|| drm::gem_close(&self.fd, req.handle));
        drm::handle_to_fd(&self.fd, req.handle).map_err(EtnavivError::Export)
    }
}
