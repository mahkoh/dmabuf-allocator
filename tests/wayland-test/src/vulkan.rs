use crate::formats::Channel;
use crate::formats::Format;
use crate::formats::Subsampling;
use ash::vk;
use dmabuf_allocator::Layout;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;
use std::mem::offset_of;
use std::os::fd::AsRawFd;
use std::os::fd::BorrowedFd;
use std::os::fd::FromRawFd;
use std::os::fd::IntoRawFd;
use std::os::fd::OwnedFd;
use std::ptr;
use std::slice;

#[cfg(test)]
mod tests;

const USAGE: vk::ImageUsageFlags = vk::ImageUsageFlags::COLOR_ATTACHMENT;
const DMA_BUF: vk::ExternalMemoryHandleTypeFlags = vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT;
const PLANE_ASPECTS: [vk::ImageAspectFlags; 3] = [
    vk::ImageAspectFlags::PLANE_0,
    vk::ImageAspectFlags::PLANE_1,
    vk::ImageAspectFlags::PLANE_2,
];

pub struct Vulkan {
    _entry: ash::Entry,
    instance: ash::Instance,
    physical: vk::PhysicalDevice,
    device: ash::Device,
    memory_fd: ash::khr::external_memory_fd::Device,
    semaphore_fd: ash::khr::external_semaphore_fd::Device,
    queue: vk::Queue,
    queue_family: u32,
    command_pool: vk::CommandPool,
    vertex_shader: vk::ShaderModule,
    fragment_shader: vk::ShaderModule,
    pipeline_layout: vk::PipelineLayout,
    pipelines: RefCell<HashMap<vk::Format, vk::Pipeline>>,
    pub render_node: u64,
    pub devices: Vec<u64>,
    maintenance5: bool,
}

pub struct Modifier {
    pub modifier: u64,
    pub planes: u32,
    pub max_width: u32,
    pub max_height: u32,
}

pub struct Image {
    image: vk::Image,
    memory: vk::DeviceMemory,
    views: Vec<vk::ImageView>,
}

pub struct Slot {
    command_buffer: vk::CommandBuffer,
    fence: vk::Fence,
    wait: vk::Semaphore,
    signal: vk::Semaphore,
}

pub enum Sync {
    Implicit(OwnedFd),
    Explicit {
        semaphore: vk::Semaphore,
        point: u64,
    },
}

pub struct Timeline {
    pub semaphore: vk::Semaphore,
    pub fd: OwnedFd,
}

pub struct Extent {
    pub width: u32,
    pub height: u32,
}

struct Matched {
    physical: vk::PhysicalDevice,
    render_node: u64,
    devices: Vec<u64>,
}

struct Created {
    device: ash::Device,
    maintenance5: bool,
}

#[repr(C)]
struct PushConstants {
    channels: [i32; 4],
    size: [f32; 2],
    subsampling: [f32; 2],
    angle: f32,
}

const _: () = assert!(offset_of!(PushConstants, channels) == 0);
const _: () = assert!(offset_of!(PushConstants, size) == 16);
const _: () = assert!(offset_of!(PushConstants, subsampling) == 24);
const _: () = assert!(offset_of!(PushConstants, angle) == 32);
const _: () = assert!(size_of::<PushConstants>() == 36);

impl PushConstants {
    fn bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(ptr::from_ref(self).cast(), size_of::<Self>()) }
    }
}

fn create_flags(format: &Format) -> vk::ImageCreateFlags {
    if format.planes.len() > 1 {
        vk::ImageCreateFlags::MUTABLE_FORMAT | vk::ImageCreateFlags::EXTENDED_USAGE
    } else {
        vk::ImageCreateFlags::empty()
    }
}

fn view_formats(format: &Format) -> Vec<vk::Format> {
    let mut formats = vec![format.vulkan];
    for plane in format.planes {
        if !formats.contains(&plane.view) {
            formats.push(plane.view);
        }
    }
    formats
}

impl Vulkan {
    pub fn new(dev: u64) -> Vulkan {
        let entry = unsafe { ash::Entry::load().expect("could not load Vulkan") };
        let application = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3);
        let info = vk::InstanceCreateInfo::default().application_info(&application);
        let instance = unsafe { entry.create_instance(&info, None).unwrap() };
        let physicals = unsafe { instance.enumerate_physical_devices().unwrap() };
        let Matched {
            physical,
            render_node,
            devices,
        } = physicals
            .into_iter()
            .find_map(|physical| matches_device(&instance, physical, dev))
            .expect("no Vulkan device for the main device of the compositor");
        let families = unsafe { instance.get_physical_device_queue_family_properties(physical) };
        let queue_family = families
            .iter()
            .position(|family| family.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .expect("no graphics queue") as u32;
        let Created {
            device,
            maintenance5,
        } = create_device(&instance, physical, queue_family);
        let memory_fd = ash::khr::external_memory_fd::Device::new(&instance, &device);
        let semaphore_fd = ash::khr::external_semaphore_fd::Device::new(&instance, &device);
        let queue = unsafe { device.get_device_queue(queue_family, 0) };
        let info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family);
        let command_pool = unsafe { device.create_command_pool(&info, None).unwrap() };
        let module = |code: &[u8]| {
            let code = ash::util::read_spv(&mut Cursor::new(code)).unwrap();
            let info = vk::ShaderModuleCreateInfo::default().code(&code);
            unsafe { device.create_shader_module(&info, None).unwrap() }
        };
        let vertex_shader = module(include_bytes!("../shaders/triangle.vert.spv"));
        let fragment_shader = module(include_bytes!("../shaders/triangle.frag.spv"));
        let push_constants = [vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::FRAGMENT,
            offset: 0,
            size: size_of::<PushConstants>() as u32,
        }];
        let info = vk::PipelineLayoutCreateInfo::default().push_constant_ranges(&push_constants);
        let pipeline_layout = unsafe { device.create_pipeline_layout(&info, None).unwrap() };
        Vulkan {
            _entry: entry,
            instance,
            physical,
            device,
            memory_fd,
            semaphore_fd,
            queue,
            queue_family,
            command_pool,
            vertex_shader,
            fragment_shader,
            pipeline_layout,
            pipelines: Default::default(),
            render_node,
            devices,
            maintenance5,
        }
    }

    fn modifier_properties(&self, format: vk::Format) -> Vec<vk::DrmFormatModifierPropertiesEXT> {
        let mut list = vk::DrmFormatModifierPropertiesListEXT::default();
        let mut properties = vk::FormatProperties2::default().push_next(&mut list);
        unsafe {
            self.instance.get_physical_device_format_properties2(
                self.physical,
                format,
                &mut properties,
            );
        }
        let mut modifiers = vec![
            vk::DrmFormatModifierPropertiesEXT::default();
            list.drm_format_modifier_count as _
        ];
        let mut list = vk::DrmFormatModifierPropertiesListEXT::default()
            .drm_format_modifier_properties(&mut modifiers);
        let mut properties = vk::FormatProperties2::default().push_next(&mut list);
        unsafe {
            self.instance.get_physical_device_format_properties2(
                self.physical,
                format,
                &mut properties,
            );
        }
        modifiers
    }

    pub fn modifiers(&self, format: &Format) -> Vec<Modifier> {
        if format.vulkan == vk::Format::A1B5G5R5_UNORM_PACK16_KHR && !self.maintenance5 {
            return Vec::new();
        }
        let renderable = |modifiers: &[vk::DrmFormatModifierPropertiesEXT], modifier: u64| {
            modifiers.iter().any(|m| {
                m.drm_format_modifier == modifier
                    && m.drm_format_modifier_tiling_features
                        .contains(vk::FormatFeatureFlags::COLOR_ATTACHMENT)
            })
        };
        let planes: Vec<_> = format
            .planes
            .iter()
            .map(|plane| self.modifier_properties(plane.view))
            .collect();
        self.modifier_properties(format.vulkan)
            .iter()
            .filter(|m| {
                planes
                    .iter()
                    .all(|plane| renderable(plane, m.drm_format_modifier))
            })
            .filter_map(|m| {
                let extent = self.max_import_extent(format, m.drm_format_modifier)?;
                Some(Modifier {
                    modifier: m.drm_format_modifier,
                    planes: m.drm_format_modifier_plane_count,
                    max_width: extent.width,
                    max_height: extent.height,
                })
            })
            .collect()
    }

    fn max_import_extent(&self, format: &Format, modifier: u64) -> Option<vk::Extent3D> {
        let formats = view_formats(format);
        let mut format_list = vk::ImageFormatListCreateInfo::default().view_formats(&formats);
        let mut modifier_info = vk::PhysicalDeviceImageDrmFormatModifierInfoEXT::default()
            .drm_format_modifier(modifier)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let mut external_info =
            vk::PhysicalDeviceExternalImageFormatInfo::default().handle_type(DMA_BUF);
        let info = vk::PhysicalDeviceImageFormatInfo2::default()
            .format(format.vulkan)
            .ty(vk::ImageType::TYPE_2D)
            .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT)
            .usage(USAGE)
            .flags(create_flags(format))
            .push_next(&mut format_list)
            .push_next(&mut modifier_info)
            .push_next(&mut external_info);
        let mut external = vk::ExternalImageFormatProperties::default();
        let mut properties = vk::ImageFormatProperties2::default().push_next(&mut external);
        let supported = unsafe {
            self.instance
                .get_physical_device_image_format_properties2(self.physical, &info, &mut properties)
                .is_ok()
        };
        let max_extent = properties.image_format_properties.max_extent;
        let importable = external
            .external_memory_properties
            .external_memory_features
            .contains(vk::ExternalMemoryFeatureFlags::IMPORTABLE);
        (supported && importable).then_some(max_extent)
    }

    pub fn import(
        &self,
        dmabuf: BorrowedFd<'_>,
        format: &Format,
        layout: &Layout,
        extent: &Extent,
    ) -> Result<Image, String> {
        let planes: Vec<_> = layout
            .planes()
            .iter()
            .map(|plane| vk::SubresourceLayout {
                offset: plane.offset(),
                size: 0,
                row_pitch: plane.stride(),
                array_pitch: 0,
                depth_pitch: 0,
            })
            .collect();
        let formats = view_formats(format);
        let mut format_list = vk::ImageFormatListCreateInfo::default().view_formats(&formats);
        let mut modifier_info = vk::ImageDrmFormatModifierExplicitCreateInfoEXT::default()
            .drm_format_modifier(layout.modifier())
            .plane_layouts(&planes);
        let mut external_info = vk::ExternalMemoryImageCreateInfo::default().handle_types(DMA_BUF);
        let info = vk::ImageCreateInfo::default()
            .flags(create_flags(format))
            .image_type(vk::ImageType::TYPE_2D)
            .format(format.vulkan)
            .extent(vk::Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT)
            .usage(USAGE)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .push_next(&mut format_list)
            .push_next(&mut modifier_info)
            .push_next(&mut external_info);
        let created = unsafe { self.device.create_image(&info, None) };
        let image = created.map_err(|e| format!("vkCreateImage: {e}"))?;
        let mut image = Image {
            image,
            memory: vk::DeviceMemory::null(),
            views: Vec::new(),
        };
        match self.bind(&mut image, dmabuf, format) {
            Ok(()) => Ok(image),
            Err(e) => {
                self.destroy_image(&image);
                Err(e)
            }
        }
    }

    fn bind(
        &self,
        image: &mut Image,
        dmabuf: BorrowedFd<'_>,
        format: &Format,
    ) -> Result<(), String> {
        let device = &self.device;
        let requirements = unsafe { device.get_image_memory_requirements(image.image) };
        let fd = dmabuf
            .try_clone_to_owned()
            .map_err(|e| format!("dup: {e}"))?;
        let mut fd_properties = vk::MemoryFdPropertiesKHR::default();
        unsafe {
            self.memory_fd
                .get_memory_fd_properties(DMA_BUF, fd.as_raw_fd(), &mut fd_properties)
                .map_err(|e| format!("vkGetMemoryFdPropertiesKHR: {e}"))?;
        }
        let types = requirements.memory_type_bits & fd_properties.memory_type_bits;
        if types == 0 {
            return Err("no memory type for the dmabuf".to_string());
        }
        let mut dedicated = vk::MemoryDedicatedAllocateInfo::default().image(image.image);
        let mut import = vk::ImportMemoryFdInfoKHR::default()
            .handle_type(DMA_BUF)
            .fd(fd.as_raw_fd());
        let info = vk::MemoryAllocateInfo::default()
            .allocation_size(requirements.size)
            .memory_type_index(types.trailing_zeros())
            .push_next(&mut dedicated)
            .push_next(&mut import);
        let allocated = unsafe { device.allocate_memory(&info, None) };
        image.memory = allocated.map_err(|e| format!("vkAllocateMemory: {e}"))?;
        let _ = fd.into_raw_fd();
        let bound = unsafe { device.bind_image_memory(image.image, image.memory, 0) };
        bound.map_err(|e| format!("vkBindImageMemory: {e}"))?;
        for (i, plane) in format.planes.iter().enumerate() {
            let aspect_mask = if format.planes.len() > 1 {
                PLANE_ASPECTS[i]
            } else {
                vk::ImageAspectFlags::COLOR
            };
            let info = vk::ImageViewCreateInfo::default()
                .image(image.image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(plane.view)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask,
                    ..COLOR_RANGE
                });
            let created = unsafe { device.create_image_view(&info, None) };
            let view = created.map_err(|e| format!("vkCreateImageView: {e}"))?;
            image.views.push(view);
        }
        Ok(())
    }

    pub fn destroy_image(&self, image: &Image) {
        unsafe {
            for &view in &image.views {
                self.device.destroy_image_view(view, None);
            }
            self.device.destroy_image(image.image, None);
            self.device.free_memory(image.memory, None);
        }
    }

    pub fn create_slot(&self) -> Slot {
        let info = vk::CommandBufferAllocateInfo::default()
            .command_pool(self.command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        let command_buffer = unsafe { self.device.allocate_command_buffers(&info).unwrap()[0] };
        let info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        let fence = unsafe { self.device.create_fence(&info, None).unwrap() };
        let wait = unsafe {
            self.device
                .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
                .unwrap()
        };
        let mut export = vk::ExportSemaphoreCreateInfo::default()
            .handle_types(vk::ExternalSemaphoreHandleTypeFlags::SYNC_FD);
        let info = vk::SemaphoreCreateInfo::default().push_next(&mut export);
        let signal = unsafe { self.device.create_semaphore(&info, None).unwrap() };
        Slot {
            command_buffer,
            fence,
            wait,
            signal,
        }
    }

    pub fn destroy_slot(&self, slot: &Slot) {
        unsafe {
            self.device.destroy_semaphore(slot.signal, None);
            self.device.destroy_semaphore(slot.wait, None);
            self.device.destroy_fence(slot.fence, None);
            self.device
                .free_command_buffers(self.command_pool, &[slot.command_buffer]);
        }
    }

    pub fn create_timeline(&self) -> Timeline {
        let mut ty = vk::SemaphoreTypeCreateInfo::default()
            .semaphore_type(vk::SemaphoreType::TIMELINE)
            .initial_value(0);
        let mut export = vk::ExportSemaphoreCreateInfo::default()
            .handle_types(vk::ExternalSemaphoreHandleTypeFlags::OPAQUE_FD);
        let info = vk::SemaphoreCreateInfo::default()
            .push_next(&mut ty)
            .push_next(&mut export);
        let semaphore = unsafe { self.device.create_semaphore(&info, None).unwrap() };
        let info = vk::SemaphoreGetFdInfoKHR::default()
            .semaphore(semaphore)
            .handle_type(vk::ExternalSemaphoreHandleTypeFlags::OPAQUE_FD);
        let fd = unsafe { self.semaphore_fd.get_semaphore_fd(&info).unwrap() };
        Timeline {
            semaphore,
            fd: unsafe { OwnedFd::from_raw_fd(fd) },
        }
    }

    pub fn destroy_semaphore(&self, semaphore: vk::Semaphore) {
        unsafe {
            self.device.destroy_semaphore(semaphore, None);
        }
    }

    pub fn timeline_value(&self, semaphore: vk::Semaphore) -> u64 {
        unsafe { self.device.get_semaphore_counter_value(semaphore).unwrap() }
    }

    pub fn wait_idle(&self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
        }
    }

    fn pipeline(&self, format: vk::Format) -> vk::Pipeline {
        *self
            .pipelines
            .borrow_mut()
            .entry(format)
            .or_insert_with(|| self.create_pipeline(format))
    }

    pub fn render(
        &self,
        slot: &Slot,
        image: &Image,
        format: &Format,
        extent: &Extent,
        angle: f32,
        sync: Sync,
    ) -> Option<OwnedFd> {
        let device = &self.device;
        unsafe {
            device
                .wait_for_fences(&[slot.fence], true, u64::MAX)
                .unwrap();
            device.reset_fences(&[slot.fence]).unwrap();
        }
        self.record(slot.command_buffer, image, format, extent, angle);
        let command_buffers =
            [vk::CommandBufferSubmitInfo::default().command_buffer(slot.command_buffer)];
        let mut waits = vec![];
        let signals;
        let implicit = match sync {
            Sync::Implicit(fence) => {
                let info = vk::ImportSemaphoreFdInfoKHR::default()
                    .semaphore(slot.wait)
                    .flags(vk::SemaphoreImportFlags::TEMPORARY)
                    .handle_type(vk::ExternalSemaphoreHandleTypeFlags::SYNC_FD)
                    .fd(fence.as_raw_fd());
                unsafe {
                    self.semaphore_fd.import_semaphore_fd(&info).unwrap();
                }
                let _ = fence.into_raw_fd();
                waits.push(
                    vk::SemaphoreSubmitInfo::default()
                        .semaphore(slot.wait)
                        .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS),
                );
                signals = [vk::SemaphoreSubmitInfo::default()
                    .semaphore(slot.signal)
                    .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS)];
                true
            }
            Sync::Explicit { semaphore, point } => {
                signals = [vk::SemaphoreSubmitInfo::default()
                    .semaphore(semaphore)
                    .value(point)
                    .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS)];
                false
            }
        };
        let submit = vk::SubmitInfo2::default()
            .wait_semaphore_infos(&waits)
            .command_buffer_infos(&command_buffers)
            .signal_semaphore_infos(&signals);
        unsafe {
            device
                .queue_submit2(self.queue, &[submit], slot.fence)
                .unwrap();
        }
        if !implicit {
            return None;
        }
        let info = vk::SemaphoreGetFdInfoKHR::default()
            .semaphore(slot.signal)
            .handle_type(vk::ExternalSemaphoreHandleTypeFlags::SYNC_FD);
        let fd = unsafe { self.semaphore_fd.get_semaphore_fd(&info).unwrap() };
        Some(unsafe { OwnedFd::from_raw_fd(fd) })
    }

    fn record(
        &self,
        cmd: vk::CommandBuffer,
        image: &Image,
        format: &Format,
        extent: &Extent,
        angle: f32,
    ) {
        let device = &self.device;
        unsafe {
            device
                .reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty())
                .unwrap();
            let info = vk::CommandBufferBeginInfo::default()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
            device.begin_command_buffer(cmd, &info).unwrap();
            let acquire = [vk::ImageMemoryBarrier2::default()
                .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .src_queue_family_index(vk::QUEUE_FAMILY_FOREIGN_EXT)
                .dst_queue_family_index(self.queue_family)
                .image(image.image)
                .subresource_range(COLOR_RANGE)];
            device.cmd_pipeline_barrier2(
                cmd,
                &vk::DependencyInfo::default().image_memory_barriers(&acquire),
            );
            for (plane, &view) in format.planes.iter().zip(&image.views) {
                let plane_extent = vk::Extent2D {
                    width: extent.width.div_ceil(plane.subsampling.x),
                    height: extent.height.div_ceil(plane.subsampling.y),
                };
                let attachments = [vk::RenderingAttachmentInfo::default()
                    .image_view(view)
                    .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                    .load_op(vk::AttachmentLoadOp::DONT_CARE)
                    .store_op(vk::AttachmentStoreOp::STORE)];
                let area = vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: plane_extent,
                };
                let info = vk::RenderingInfo::default()
                    .render_area(area)
                    .layer_count(1)
                    .color_attachments(&attachments);
                device.cmd_begin_rendering(cmd, &info);
                device.cmd_bind_pipeline(
                    cmd,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.pipeline(plane.view),
                );
                let viewport = vk::Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: plane_extent.width as f32,
                    height: plane_extent.height as f32,
                    min_depth: 0.0,
                    max_depth: 1.0,
                };
                device.cmd_set_viewport(cmd, 0, &[viewport]);
                device.cmd_set_scissor(cmd, 0, &[area]);
                let push = push_constants(plane.channels, extent, plane.subsampling, angle);
                device.cmd_push_constants(
                    cmd,
                    self.pipeline_layout,
                    vk::ShaderStageFlags::FRAGMENT,
                    0,
                    push.bytes(),
                );
                device.cmd_draw(cmd, 3, 1, 0, 0);
                device.cmd_end_rendering(cmd);
            }
            let release = [vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(vk::ImageLayout::GENERAL)
                .src_queue_family_index(self.queue_family)
                .dst_queue_family_index(vk::QUEUE_FAMILY_FOREIGN_EXT)
                .image(image.image)
                .subresource_range(COLOR_RANGE)];
            device.cmd_pipeline_barrier2(
                cmd,
                &vk::DependencyInfo::default().image_memory_barriers(&release),
            );
            device.end_command_buffer(cmd).unwrap();
        }
    }

    fn create_pipeline(&self, format: vk::Format) -> vk::Pipeline {
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(self.vertex_shader)
                .name(c"main"),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(self.fragment_shader)
                .name(c"main"),
        ];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);
        let viewport = vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .scissor_count(1);
        let rasterization = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::NONE)
            .line_width(1.0);
        let multisample = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let attachments = [vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)];
        let blend = vk::PipelineColorBlendStateCreateInfo::default().attachments(&attachments);
        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
        let formats = [format];
        let mut rendering =
            vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport)
            .rasterization_state(&rasterization)
            .multisample_state(&multisample)
            .color_blend_state(&blend)
            .dynamic_state(&dynamic)
            .layout(self.pipeline_layout)
            .push_next(&mut rendering);
        let pipelines = unsafe {
            self.device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None)
        };
        pipelines.map_err(|(_, e)| e).unwrap()[0]
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.device_wait_idle();
            for &pipeline in self.pipelines.get_mut().values() {
                self.device.destroy_pipeline(pipeline, None);
            }
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_shader_module(self.vertex_shader, None);
            self.device
                .destroy_shader_module(self.fragment_shader, None);
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

fn selector(channel: Channel) -> i32 {
    match channel {
        Channel::R => 0,
        Channel::G => 1,
        Channel::B => 2,
        Channel::Y => 3,
        Channel::U => 4,
        Channel::V => 5,
        Channel::One => 6,
    }
}

fn push_constants(
    channels: [Channel; 4],
    extent: &Extent,
    subsampling: Subsampling,
    angle: f32,
) -> PushConstants {
    PushConstants {
        channels: channels.map(selector),
        size: [extent.width as f32, extent.height as f32],
        subsampling: [subsampling.x as f32, subsampling.y as f32],
        angle,
    }
}

const COLOR_RANGE: vk::ImageSubresourceRange = vk::ImageSubresourceRange {
    aspect_mask: vk::ImageAspectFlags::COLOR,
    base_mip_level: 0,
    level_count: 1,
    base_array_layer: 0,
    layer_count: 1,
};

fn matches_device(
    instance: &ash::Instance,
    physical: vk::PhysicalDevice,
    dev: u64,
) -> Option<Matched> {
    let extensions = unsafe {
        instance
            .enumerate_device_extension_properties(physical)
            .unwrap()
    };
    let has_drm = extensions
        .iter()
        .any(|e| e.extension_name_as_c_str() == Ok(ash::ext::physical_device_drm::NAME));
    if !has_drm {
        return None;
    }
    let mut drm = vk::PhysicalDeviceDrmPropertiesEXT::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut drm);
    unsafe {
        instance.get_physical_device_properties2(physical, &mut properties);
    }
    if drm.has_render == 0 {
        return None;
    }
    let primary = libc::makedev(drm.primary_major as u32, drm.primary_minor as u32);
    let render = libc::makedev(drm.render_major as u32, drm.render_minor as u32);
    let mut devices = vec![render];
    if drm.has_primary != 0 {
        devices.push(primary);
    }
    devices.contains(&dev).then_some(Matched {
        physical,
        render_node: render,
        devices,
    })
}

fn create_device(
    instance: &ash::Instance,
    physical: vk::PhysicalDevice,
    queue_family: u32,
) -> Created {
    let priorities = [1.0];
    let queues = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family)
        .queue_priorities(&priorities)];
    let mut extensions = vec![
        ash::ext::image_drm_format_modifier::NAME.as_ptr(),
        ash::ext::external_memory_dma_buf::NAME.as_ptr(),
        ash::ext::queue_family_foreign::NAME.as_ptr(),
        ash::khr::external_memory_fd::NAME.as_ptr(),
        ash::khr::external_semaphore_fd::NAME.as_ptr(),
    ];
    let supported = unsafe {
        instance
            .enumerate_device_extension_properties(physical)
            .unwrap()
    };
    let maintenance5 = supported
        .iter()
        .any(|e| e.extension_name_as_c_str() == Ok(ash::khr::maintenance5::NAME));
    if maintenance5 {
        extensions.push(ash::khr::maintenance5::NAME.as_ptr());
    }
    let mut features12 = vk::PhysicalDeviceVulkan12Features::default().timeline_semaphore(true);
    let mut features13 = vk::PhysicalDeviceVulkan13Features::default()
        .synchronization2(true)
        .dynamic_rendering(true);
    let mut features_maintenance5 =
        vk::PhysicalDeviceMaintenance5FeaturesKHR::default().maintenance5(true);
    let mut info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queues)
        .enabled_extension_names(&extensions)
        .push_next(&mut features12)
        .push_next(&mut features13);
    if maintenance5 {
        info = info.push_next(&mut features_maintenance5);
    }
    let device = unsafe {
        instance
            .create_device(physical, &info, None)
            .expect("vkCreateDevice failed")
    };
    Created {
        device,
        maintenance5,
    }
}
