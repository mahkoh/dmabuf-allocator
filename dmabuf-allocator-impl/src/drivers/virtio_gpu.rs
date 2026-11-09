use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::allocator::Extra;
use crate::drivers::amd::amdgpu;
use crate::drivers::amd::amdgpu::AmdgpuAllocateError;
use crate::drivers::amd::amdgpu::AmdgpuVirtioOpenError;
use crate::drivers::asahi;
use crate::drivers::asahi::AsahiError;
use crate::drivers::intel::i915;
use crate::drivers::intel::i915::I915Error;
use crate::drivers::open_render_node;
use crate::drivers::panfrost;
use crate::drivers::panfrost::PanfrostError;
use crate::drm;
use crate::drm::DRM_COMMAND_BASE;
use crate::drm::DRM_IOCTL_BASE;
use crate::drm::DrmError;
use crate::drm::GemHandle;
use crate::errno::Errno;
use crate::fd::OwnedFd;
use crate::kernel::Kernel;
use crate::layout::DRM_FORMAT_MOD_LINEAR;
use crate::layout::Layout;
use crate::layout::LayoutError;
use crate::layout::PlaneLayout;
use crate::layout::PlaneVec;
use crate::layout::align;
use crate::sys::_IOWR;
use crate::sys::Ioctl;
use crate::vdrm;
use crate::vdrm::DRM_IOCTL_VIRTGPU_CONTEXT_INIT;
use crate::vdrm::VIRTGPU_CONTEXT_PARAM_CAPSET_ID;
use crate::vdrm::VIRTGPU_DRM_CONTEXT_AMDGPU;
use crate::vdrm::VIRTGPU_DRM_CONTEXT_ASAHI;
use crate::vdrm::VIRTGPU_DRM_CONTEXT_I915;
use crate::vdrm::VIRTGPU_DRM_CONTEXT_PANFROST;
use crate::vdrm::drm_virtgpu_context_init;
use crate::vdrm::drm_virtgpu_context_set_param;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::ffi::c_int;
use core::mem;
use core::ptr;
use run_on_drop::on_drop;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VirtioGpuError {
    #[error("could not open the render node")]
    RenderNode(#[source] Errno),
    #[error("the format is not supported by virgl")]
    Format,
    #[error("could not create the resource")]
    ResourceCreate(#[source] Errno),
    #[error("virgl could not export the buffer")]
    Export(#[source] DrmError),
    #[error(transparent)]
    AmdgpuOpen(#[from] AmdgpuVirtioOpenError),
    #[error(transparent)]
    AmdgpuAllocate(#[from] AmdgpuAllocateError),
    #[error(transparent)]
    I915(#[from] I915Error),
    #[error(transparent)]
    Asahi(#[from] AsahiError),
    #[error(transparent)]
    Panfrost(#[from] PanfrostError),
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub(crate) struct VirtioHandle(pub(crate) u32);

const VIRTGPU_PARAM_3D_FEATURES: u64 = 1;
const VIRTGPU_PARAM_CAPSET_QUERY_FIX: u64 = 2;
const VIRTGPU_PARAM_CONTEXT_INIT: u64 = 6;
#[allow(non_upper_case_globals)]
const VIRTGPU_PARAM_SUPPORTED_CAPSET_IDs: u64 = 7;
const VIRTGPU_DRM_CAPSET_VIRGL: u32 = 1;
const VIRTGPU_DRM_CAPSET_VIRGL2: u32 = 2;
const PIPE_TEXTURE_2D: u32 = 2;
const VIRGL_FORMAT_B8G8R8A8_UNORM: u32 = 1;
const VIRGL_FORMAT_B8G8R8X8_UNORM: u32 = 2;
const VIRGL_FORMAT_R32G32B32_FLOAT: u32 = 30;
const VIRGL_FORMAT_B8G8R8A8_SRGB: u32 = 100;
const VIRGL_FORMAT_B8G8R8X8_SRGB: u32 = 101;
const VIRGL_FORMAT_R8G8B8A8_SRGB: u32 = 104;
const VIRGL_FORMAT_R32G32B32_UINT: u32 = 195;
const VIRGL_FORMAT_R32G32B32_SINT: u32 = 199;
const VIRGL_FORMAT_R8G8B8X8_SRGB: u32 = 230;
const VIRGL_CAP_APP_TWEAK_SUPPORT: u32 = 1 << 28;
const VIRGL_CAP_V2_QUERY_FORMAT_MODIFIER: u32 = 1 << 19;
const VIRGL_BIND_RENDER_TARGET: u32 = 1 << 1;
const VIRGL_BIND_SAMPLER_VIEW: u32 = 1 << 3;
const VIRGL_BIND_SCANOUT: u32 = 1 << 18;
const VIRGL_BIND_SHARED: u32 = 1 << 20;
const VIRGL_BIND_PREFER_EMULATED_BGRA: u32 = 1 << 21;
const VIRGL_BIND_LINEAR: u32 = 1 << 22;

const DEFAULT_MAX_EXTENT: u32 = 16384;
const SHARED_TEX_HEIGHT_ALIGNMENT: u64 = 64;
const SHARED_TEX_STRIDE_ALIGNMENT: u64 = 1024;

#[repr(C)]
struct drm_virtgpu_getparam {
    param: u64,
    value: u64,
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct drm_virtgpu_get_caps {
    pub(crate) cap_set_id: u32,
    pub(crate) cap_set_ver: u32,
    pub(crate) addr: u64,
    pub(crate) size: u32,
    pub(crate) pad: u32,
}

#[repr(C)]
#[derive(Default)]
struct drm_virtgpu_resource_create {
    target: u32,
    format: u32,
    bind: u32,
    width: u32,
    height: u32,
    depth: u32,
    array_size: u32,
    last_level: u32,
    nr_samples: u32,
    flags: u32,
    bo_handle: u32,
    res_handle: u32,
    size: u32,
    stride: u32,
}

#[repr(C)]
struct virgl_supported_format_mask {
    bitmask: [u32; 16],
}

#[repr(C)]
struct virgl_caps_v1 {
    max_version: u32,
    sampler: virgl_supported_format_mask,
    render: virgl_supported_format_mask,
    depthstencil: virgl_supported_format_mask,
    vertexbuffer: virgl_supported_format_mask,
    bset: u32,
    glsl_level: u32,
    max_texture_array_layers: u32,
    max_streamout_buffers: u32,
    max_dual_source_render_targets: u32,
    max_render_targets: u32,
    max_samples: u32,
    prim_mask: u32,
    max_tbo_size: u32,
    max_uniform_blocks: u32,
    max_viewports: u32,
    max_texture_gather_components: u32,
}

#[repr(C)]
struct virgl_video_caps {
    profile_entrypoint_level_frames: u32,
    max_width_height: u32,
    format_macroblocks: u32,
    flags_temporal_layers: u32,
}

#[repr(C)]
struct virgl_caps_v2 {
    v1: virgl_caps_v1,
    min_aliased_point_size: f32,
    max_aliased_point_size: f32,
    min_smooth_point_size: f32,
    max_smooth_point_size: f32,
    min_aliased_line_width: f32,
    max_aliased_line_width: f32,
    min_smooth_line_width: f32,
    max_smooth_line_width: f32,
    max_texture_lod_bias: f32,
    max_geom_output_vertices: u32,
    max_geom_total_output_components: u32,
    max_vertex_outputs: u32,
    max_vertex_attribs: u32,
    max_shader_patch_varyings: u32,
    min_texel_offset: i32,
    max_texel_offset: i32,
    min_texture_gather_offset: i32,
    max_texture_gather_offset: i32,
    texture_buffer_offset_alignment: u32,
    uniform_buffer_offset_alignment: u32,
    shader_buffer_offset_alignment: u32,
    capability_bits: u32,
    sample_locations: [u32; 8],
    max_vertex_attrib_stride: u32,
    max_shader_buffer_frag_compute: u32,
    max_shader_buffer_other_stages: u32,
    max_shader_image_frag_compute: u32,
    max_shader_image_other_stages: u32,
    max_image_samples: u32,
    max_compute_work_group_invocations: u32,
    max_compute_shared_memory_size: u32,
    max_compute_grid_size: [u32; 3],
    max_compute_block_size: [u32; 3],
    max_texture_2d_size: u32,
    max_texture_3d_size: u32,
    max_texture_cube_size: u32,
    max_combined_shader_buffers: u32,
    max_atomic_counters: [u32; 6],
    max_atomic_counter_buffers: [u32; 6],
    max_combined_atomic_counters: u32,
    max_combined_atomic_counter_buffers: u32,
    host_feature_check_version: u32,
    supported_readback_formats: virgl_supported_format_mask,
    scanout: virgl_supported_format_mask,
    capability_bits_v2: u32,
    max_video_memory: u32,
    renderer: [u8; 64],
    max_anisotropy: f32,
    max_texture_samplers: u32,
    supported_multisample_formats: virgl_supported_format_mask,
    max_const_buffer_size: [u32; 6],
    num_video_caps: u32,
    video_caps: [virgl_video_caps; 32],
    max_uniform_block_size: u32,
    max_tcs_outputs: u32,
    max_tes_outputs: u32,
    max_shader_storage_blocks: [u32; 6],
}

const DRM_IOCTL_VIRTGPU_GETPARAM: Ioctl =
    _IOWR::<drm_virtgpu_getparam>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x03);
const DRM_IOCTL_VIRTGPU_RESOURCE_CREATE: Ioctl =
    _IOWR::<drm_virtgpu_resource_create>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x04);
pub(crate) const DRM_IOCTL_VIRTGPU_GET_CAPS: Ioctl =
    _IOWR::<drm_virtgpu_get_caps>(DRM_IOCTL_BASE, DRM_COMMAND_BASE + 0x09);

pub(crate) fn get_param<K: Kernel>(fd: &OwnedFd<K>, param: u64) -> Option<i32> {
    let mut value: i32 = 0;
    let mut get = drm_virtgpu_getparam {
        param,
        value: ptr::from_mut(&mut value).expose_provenance() as u64,
    };
    unsafe {
        drm::ioctl(fd, DRM_IOCTL_VIRTGPU_GETPARAM, &mut get).ok()?;
    }
    Some(value)
}

fn init_context<K: Kernel>(fd: &OwnedFd<K>, supported_capset_ids: i32) -> bool {
    let Some(capset) = [VIRTGPU_DRM_CAPSET_VIRGL2, VIRTGPU_DRM_CAPSET_VIRGL]
        .into_iter()
        .find(|capset| supported_capset_ids & (1 << capset) != 0)
    else {
        return false;
    };
    let mut param = drm_virtgpu_context_set_param {
        param: VIRTGPU_CONTEXT_PARAM_CAPSET_ID,
        value: capset as u64,
    };
    let mut init = drm_virtgpu_context_init {
        num_params: 1,
        pad: 0,
        ctx_set_params: ptr::from_mut(&mut param).expose_provenance() as u64,
    };
    matches!(
        unsafe { drm::ioctl(fd, DRM_IOCTL_VIRTGPU_CONTEXT_INIT, &mut init) },
        Ok(()) | Err(Errno(libc::EEXIST))
    )
}

fn get_caps<K: Kernel>(fd: &OwnedFd<K>, capset_fix: bool) -> virgl_caps_v2 {
    let mut caps = unsafe { mem::zeroed::<virgl_caps_v2>() };
    let mut get_capset = |cap_set_id, size: usize| {
        let mut get = drm_virtgpu_get_caps {
            cap_set_id,
            addr: ptr::from_mut(&mut caps).expose_provenance() as u64,
            size: size as u32,
            ..Default::default()
        };
        unsafe { drm::ioctl(fd, DRM_IOCTL_VIRTGPU_GET_CAPS, &mut get) }
    };
    if !capset_fix
        || get_capset(VIRTGPU_DRM_CAPSET_VIRGL2, size_of::<virgl_caps_v2>())
            == Err(Errno(libc::EINVAL))
    {
        let _ = get_capset(VIRTGPU_DRM_CAPSET_VIRGL, size_of::<virgl_caps_v1>());
    }
    caps
}

fn has_format(mask: &virgl_supported_format_mask, format: u32) -> bool {
    mask.bitmask[(format / 32) as usize] & (1 << (format % 32)) != 0
}

fn shared_tex_size(stride: u64, height: u32) -> u64 {
    align(height as u64, SHARED_TEX_HEIGHT_ALIGNMENT)
        .saturating_mul(align(stride, SHARED_TEX_STRIDE_ALIGNMENT))
}

pub(crate) struct Virgl<K: Kernel> {
    fd: Arc<OwnedFd<K>>,
    caps: Box<virgl_caps_v2>,
    emulate_bgra: bool,
}

impl<K: Kernel> Virgl<K> {
    fn probe(fd: &Arc<OwnedFd<K>>) -> Result<Option<Self>, VirtioGpuError> {
        if get_param(fd, VIRTGPU_PARAM_3D_FEATURES).unwrap_or(0) == 0 {
            return Ok(None);
        }
        let fd = open_render_node(fd).map_err(VirtioGpuError::RenderNode)?;
        let param = |param| get_param(&fd, param).unwrap_or(0);
        if param(VIRTGPU_PARAM_CONTEXT_INIT) != 0
            && !init_context(&fd, param(VIRTGPU_PARAM_SUPPORTED_CAPSET_IDs))
        {
            return Ok(None);
        }
        let caps = get_caps(&fd, param(VIRTGPU_PARAM_CAPSET_QUERY_FIX) != 0);
        if caps.capability_bits_v2 & VIRGL_CAP_V2_QUERY_FORMAT_MODIFIER != 0 {
            return Ok(None);
        }
        let emulate_bgra = caps.capability_bits & VIRGL_CAP_APP_TWEAK_SUPPORT != 0
            && !has_format(&caps.v1.render, VIRGL_FORMAT_B8G8R8A8_SRGB);
        Ok(Some(Self {
            fd,
            caps: Box::new(caps),
            emulate_bgra,
        }))
    }

    fn tex_usage(&self, format: u32) -> u32 {
        let emulated = match format {
            VIRGL_FORMAT_B8G8R8A8_SRGB => VIRGL_FORMAT_R8G8B8A8_SRGB,
            VIRGL_FORMAT_B8G8R8X8_SRGB => VIRGL_FORMAT_R8G8B8X8_SRGB,
            _ => format,
        };
        let supports =
            |mask| has_format(mask, format) || self.emulate_bgra && has_format(mask, emulated);
        if matches!(
            format,
            VIRGL_FORMAT_R32G32B32_FLOAT
                | VIRGL_FORMAT_R32G32B32_SINT
                | VIRGL_FORMAT_R32G32B32_UINT
        ) || !supports(&self.caps.v1.sampler)
        {
            0
        } else if supports(&self.caps.v1.render) {
            VIRGL_BIND_SAMPLER_VIEW | VIRGL_BIND_RENDER_TARGET
        } else {
            VIRGL_BIND_SAMPLER_VIEW
        }
    }

    fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        let fmt = config.format.0;
        let [plane] = fmt.planes else {
            return Err(LayoutError::Format);
        };
        let max_size = match self.caps.max_texture_2d_size {
            0 => DEFAULT_MAX_EXTENT,
            size => size,
        };
        let stride = config.width as u64 * (plane.bits / 8) as u64;
        let virgl = fmt.virgl.ok_or(LayoutError::Format)?;
        if self.tex_usage(virgl) == 0
            || plane.block_width != 1
            || plane.block_height != 1
            || !plane.bits.is_multiple_of(8)
        {
            return Err(LayoutError::Format);
        }
        if config.width.min(config.height) == 0 || config.width.max(config.height) > max_size {
            return Err(LayoutError::Size);
        }
        if !config.modifiers.contains(&DRM_FORMAT_MOD_LINEAR) {
            return Err(LayoutError::Modifier);
        }
        if u32::try_from(shared_tex_size(stride, config.height)).is_err() {
            return Err(LayoutError::TooLarge);
        }
        Ok(DeviceLayout {
            layout: Layout {
                format: fmt,
                modifier: DRM_FORMAT_MOD_LINEAR,
                planes: PlaneVec::from_iter([PlaneLayout {
                    offset: 0,
                    stride,
                    size: stride * config.height as u64,
                }]),
            },
            extra: Extra::None,
        })
    }

    fn allocate(
        &self,
        device_layout: &DeviceLayout,
        config: &Config<'_>,
    ) -> Result<c_int, VirtioGpuError> {
        let format = device_layout
            .layout
            .format
            .virgl
            .ok_or(VirtioGpuError::Format)?;
        let stride = device_layout.layout.planes[0].stride;
        let mut bind = self.tex_usage(format) | VIRGL_BIND_SHARED | VIRGL_BIND_LINEAR;
        if config.scanout {
            bind |= VIRGL_BIND_SCANOUT;
        }
        if self.emulate_bgra
            && matches!(
                format,
                VIRGL_FORMAT_B8G8R8A8_SRGB
                    | VIRGL_FORMAT_B8G8R8A8_UNORM
                    | VIRGL_FORMAT_B8G8R8X8_SRGB
                    | VIRGL_FORMAT_B8G8R8X8_UNORM
            )
        {
            bind |= VIRGL_BIND_PREFER_EMULATED_BGRA;
        }
        let mut create = drm_virtgpu_resource_create {
            target: PIPE_TEXTURE_2D,
            format,
            bind,
            width: config.width,
            height: config.height,
            depth: 1,
            array_size: 1,
            size: shared_tex_size(stride, config.height) as u32,
            stride: stride as u32,
            ..Default::default()
        };
        unsafe {
            drm::ioctl(&self.fd, DRM_IOCTL_VIRTGPU_RESOURCE_CREATE, &mut create)
                .map_err(VirtioGpuError::ResourceCreate)?;
        }
        let handle = GemHandle(create.bo_handle);
        let _close = on_drop(|| drm::gem_close(&self.fd, handle));
        drm::handle_to_fd(&self.fd, handle).map_err(VirtioGpuError::Export)
    }
}

pub(crate) enum Device<K: Kernel> {
    Virgl(Virgl<K>),
    Amdgpu(amdgpu::Device<K>),
    I915(i915::Device<K>),
    Asahi(asahi::Device<K>),
    Panfrost(panfrost::Device<K>),
}

impl<K: Kernel> Device<K> {
    pub(crate) fn probe(fd: &Arc<OwnedFd<K>>) -> Result<Option<Self>, VirtioGpuError> {
        let Some(caps) = vdrm::capset(fd) else {
            return Ok(Virgl::probe(fd)?.map(Self::Virgl));
        };
        Ok(Some(match caps.context_type {
            VIRTGPU_DRM_CONTEXT_AMDGPU => Self::Amdgpu(amdgpu::Device::virtio(fd, &caps)?),
            VIRTGPU_DRM_CONTEXT_I915 => Self::I915(i915::Device::virtio(fd, &caps)?),
            VIRTGPU_DRM_CONTEXT_ASAHI => Self::Asahi(asahi::Device::virtio(fd)?),
            VIRTGPU_DRM_CONTEXT_PANFROST => Self::Panfrost(panfrost::Device::virtio(fd)?),
            _ => return Ok(Virgl::probe(fd)?.map(Self::Virgl)),
        }))
    }

    pub(crate) fn layout(&self, config: &Config<'_>) -> Result<DeviceLayout, LayoutError> {
        match self {
            Self::Virgl(device) => device.layout(config),
            Self::Amdgpu(device) => device.layout(config),
            Self::I915(device) => device.layout(config),
            Self::Asahi(device) => device.layout(config),
            Self::Panfrost(device) => device.layout(config),
        }
    }

    pub(crate) fn allocate(
        &self,
        device_layout: &DeviceLayout,
        config: &Config<'_>,
    ) -> Result<c_int, VirtioGpuError> {
        Ok(match self {
            Self::Virgl(device) => device.allocate(device_layout, config)?,
            Self::Amdgpu(device) => device.allocate(device_layout, config)?,
            Self::I915(device) => device.allocate(device_layout, config)?,
            Self::Asahi(device) => device.allocate(device_layout)?,
            Self::Panfrost(device) => device.allocate(device_layout)?,
        })
    }
}
