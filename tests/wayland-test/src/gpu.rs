use crate::State;
use crate::dmabuf;
use crate::feedback::Feedback;
use crate::formats;
use crate::formats::Format;
use crate::protocols::linux_dmabuf_v1::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1Flags;
use crate::protocols::linux_dmabuf_v1::zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1;
use crate::protocols::linux_drm_syncobj_v1::wp_linux_drm_syncobj_manager_v1::WpLinuxDrmSyncobjManagerV1;
use crate::protocols::linux_drm_syncobj_v1::wp_linux_drm_syncobj_surface_v1::WpLinuxDrmSyncobjSurfaceV1;
use crate::protocols::linux_drm_syncobj_v1::wp_linux_drm_syncobj_timeline_v1::WpLinuxDrmSyncobjTimelineV1;
use crate::protocols::wayland::wl_buffer::WlBuffer;
use crate::vulkan;
use crate::vulkan::Extent;
use crate::vulkan::Vulkan;
use ash::vk;
use dmabuf_allocator::AllocationBuilder;
use dmabuf_allocator::Allocator;
use error_reporter::Report;
use std::collections::HashMap;
use std::fs::File;
use std::os::fd::AsFd;
use std::os::fd::AsRawFd;
use std::os::fd::OwnedFd;
use std::os::unix::fs::MetadataExt;
use std::time::Duration;
use std::time::Instant;
use wl_client::proxy;

#[cfg(test)]
mod tests;

const DRM_FORMAT_MOD_INVALID: u64 = 0x00ff_ffff_ffff_ffff;
const BUFFERS: usize = 3;
const DEFAULT_DWELL: Duration = Duration::from_secs(2);
const MIN_DWELL: Duration = Duration::from_millis(125);
const MAX_DWELL: Duration = Duration::from_secs(64);

pub struct Gpu {
    vulkan: Vulkan,
    allocator: Allocator,
    modifiers: HashMap<[u8; 4], Vec<vulkan::Modifier>>,
    syncobj_manager: Option<WpLinuxDrmSyncobjManagerV1>,
    acquire: Option<Timeline>,
    acquire_point: u64,
    sweep: Vec<Entry>,
    index: usize,
    dwell: Duration,
    shown: Option<Instant>,
    swapchain: Option<Swapchain>,
    generation: u64,
}

#[derive(Copy, Clone)]
struct Entry {
    format: &'static Format,
    modifier: u64,
    scanout: bool,
}

impl Entry {
    fn is(&self, fourcc: [u8; 4], modifier: u64) -> bool {
        self.format.fourcc == fourcc && self.modifier == modifier
    }

    fn same(&self, other: &Entry) -> bool {
        self.is(other.format.fourcc, other.modifier) && self.scanout == other.scanout
    }
}

struct Timeline {
    semaphore: vk::Semaphore,
    proxy: WpLinuxDrmSyncobjTimelineV1,
}

struct Swapchain {
    entry: Entry,
    requested: Extent,
    extent: Extent,
    generation: u64,
    buffers: Vec<Buffer>,
}

struct Buffer {
    dmabuf: OwnedFd,
    wl_buffer: WlBuffer,
    image: vulkan::Image,
    slot: vulkan::Slot,
    busy: bool,
    release: Option<Timeline>,
    release_point: u64,
}

impl Timeline {
    fn new(vulkan: &Vulkan, manager: &WpLinuxDrmSyncobjManagerV1) -> Timeline {
        let timeline = vulkan.create_timeline();
        let proxy = manager.import_timeline(timeline.fd.as_fd());
        Timeline {
            semaphore: timeline.semaphore,
            proxy,
        }
    }

    fn destroy(&self, vulkan: &Vulkan) {
        self.proxy.destroy();
        vulkan.destroy_semaphore(self.semaphore);
    }
}

struct Halves {
    hi: u32,
    lo: u32,
}

struct PlaneLayout {
    offset: u32,
    stride: u32,
}

fn split(value: u64) -> Halves {
    Halves {
        hi: (value >> 32) as u32,
        lo: value as u32,
    }
}

fn open_render_node(dev: u64) -> File {
    for entry in std::fs::read_dir("/dev/dri").expect("could not read /dev/dri") {
        let path = entry.unwrap().path();
        let is_render_node = path
            .file_name()
            .is_some_and(|name| name.to_string_lossy().starts_with("renderD"));
        if is_render_node && path.metadata().is_ok_and(|m| m.rdev() == dev) {
            return File::options()
                .read(true)
                .write(true)
                .open(&path)
                .expect("could not open the render node");
        }
    }
    panic!("no render node for device {dev:#x}");
}

impl Gpu {
    pub fn new(main_device: u64, syncobj_manager: Option<WpLinuxDrmSyncobjManagerV1>) -> Gpu {
        let vulkan = Vulkan::new(main_device);
        let render_node = open_render_node(vulkan.render_node);
        let allocator =
            Allocator::new(render_node.as_raw_fd()).expect("the device is not supported");
        let acquire = syncobj_manager
            .as_ref()
            .map(|manager| Timeline::new(&vulkan, manager));
        Gpu {
            vulkan,
            allocator,
            modifiers: HashMap::new(),
            syncobj_manager,
            acquire,
            acquire_point: 0,
            sweep: Vec::new(),
            index: 0,
            dwell: DEFAULT_DWELL,
            shown: None,
            swapchain: None,
            generation: 0,
        }
    }

    pub fn explicit_sync(&self) -> bool {
        self.acquire.is_some()
    }

    pub fn set_feedback(&mut self, feedback: &Feedback) {
        let current = self.sweep.get(self.index).copied();
        self.sweep.clear();
        for tranche in &feedback.tranches {
            if !self.vulkan.devices.contains(&tranche.target_device) {
                continue;
            }
            for pair in &tranche.pairs {
                let fourcc = pair.fourcc.to_le_bytes();
                let modifier = pair.modifier;
                if self.sweep.iter().any(|entry| entry.is(fourcc, modifier)) {
                    continue;
                }
                let Some(format) = formats::find(fourcc) else {
                    continue;
                };
                let vulkan = &self.vulkan;
                let modifiers = self
                    .modifiers
                    .entry(fourcc)
                    .or_insert_with(|| vulkan.modifiers(format));
                if modifier != DRM_FORMAT_MOD_INVALID
                    && !modifiers.iter().any(|m| m.modifier == modifier)
                {
                    continue;
                }
                self.sweep.push(Entry {
                    format,
                    modifier,
                    scanout: tranche.scanout,
                });
            }
        }
        let index = current.and_then(|current| {
            self.sweep
                .iter()
                .position(|entry| entry.is(current.format.fourcc, current.modifier))
        });
        if index.is_none() {
            self.shown = None;
        }
        self.index = index.unwrap_or(0);
        eprintln!("{} format/modifier pairs", self.sweep.len());
    }

    pub fn title(&self) -> String {
        let Some(swapchain) = &self.swapchain else {
            return "no supported format".to_owned();
        };
        let entry = &swapchain.entry;
        format!(
            "{} {:#018x}{} ({}/{}) {:?}",
            entry.format.name,
            entry.modifier,
            if entry.scanout { " scanout" } else { "" },
            self.index + 1,
            self.sweep.len(),
            self.dwell,
        )
    }

    pub fn faster(&mut self) {
        self.dwell = (self.dwell / 2).max(MIN_DWELL);
    }

    pub fn slower(&mut self) {
        self.dwell = (self.dwell * 2).min(MAX_DWELL);
    }

    fn advance(&mut self) {
        self.index = (self.index + 1) % self.sweep.len();
        self.shown = None;
    }

    pub fn update(&mut self, dmabuf: &ZwpLinuxDmabufV1, width: u32, height: u32) {
        if self.sweep.is_empty() {
            self.destroy_swapchain();
            return;
        }
        if self
            .shown
            .is_some_and(|shown| shown.elapsed() >= self.dwell)
        {
            self.advance();
        }
        for _ in 0..self.sweep.len() {
            let entry = self.sweep[self.index];
            if let Some(swapchain) = &self.swapchain
                && swapchain.entry.same(&entry)
                && swapchain.requested.width == width
                && swapchain.requested.height == height
            {
                break;
            }
            self.destroy_swapchain();
            self.generation += 1;
            match self.create_swapchain(dmabuf, entry, Extent { width, height }) {
                Ok(swapchain) => {
                    self.swapchain = Some(swapchain);
                    self.shown.get_or_insert_with(Instant::now);
                    break;
                }
                Err(e) => {
                    eprintln!("{} {:#018x}: {e}", entry.format.name, entry.modifier);
                    self.advance();
                }
            }
        }
    }

    fn create_swapchain(
        &self,
        dmabuf: &ZwpLinuxDmabufV1,
        entry: Entry,
        requested: Extent,
    ) -> Result<Swapchain, String> {
        let planes = entry.format.planes;
        let sx = planes.iter().map(|p| p.subsampling.x).max().unwrap_or(1);
        let sy = planes.iter().map(|p| p.subsampling.y).max().unwrap_or(1);
        let extent = Extent {
            width: requested.width.next_multiple_of(sx),
            height: requested.height.next_multiple_of(sy),
        };
        let Extent { width, height } = extent;
        let modifiers = [entry.modifier];
        let builder = self
            .allocator
            .builder()
            .format(entry.format.format)
            .modifiers(&modifiers)
            .size(width, height)
            .scanout(entry.scanout)
            .implicit_sync(self.acquire.is_none());
        let layout = builder
            .layout()
            .map_err(|error| format!("dmabuf-allocator cannot compute the layout: {error}"))?;
        let modifier = self.modifiers[&entry.format.fourcc]
            .iter()
            .find(|m| m.modifier == layout.modifier())
            .ok_or_else(|| format!("Vulkan does not support {:#018x}", layout.modifier()))?;
        if width > modifier.max_width || height > modifier.max_height {
            return Err(format!(
                "{width}x{height} exceeds the maximum size of Vulkan"
            ));
        }
        let planes = layout.planes().len();
        if planes != modifier.planes as usize {
            return Err(format!(
                "dmabuf-allocator has {planes} planes but Vulkan has {}",
                modifier.planes,
            ));
        }
        let mut buffers = Vec::new();
        for index in 0..BUFFERS {
            match self.create_buffer(dmabuf, entry, &builder, &extent, index) {
                Ok(buffer) => buffers.push(buffer),
                Err(e) => {
                    for buffer in &buffers {
                        self.destroy_buffer(buffer);
                    }
                    return Err(e);
                }
            }
        }
        eprintln!(
            "allocated {width}x{height} {} buffers with modifier {:#018x}, scanout: {}",
            entry.format.name, entry.modifier, entry.scanout,
        );
        Ok(Swapchain {
            entry,
            requested,
            extent,
            generation: self.generation,
            buffers,
        })
    }

    fn create_buffer(
        &self,
        dmabuf: &ZwpLinuxDmabufV1,
        entry: Entry,
        builder: &AllocationBuilder<'_>,
        extent: &Extent,
        index: usize,
    ) -> Result<Buffer, String> {
        let allocation = builder.allocate().map_err(|e| Report::new(e).to_string())?;
        let layout = &allocation.layout;
        let fd = allocation.dmabuf.into_std();
        let mut planes = Vec::new();
        for plane in layout.planes() {
            let offset = u32::try_from(plane.offset());
            let stride = u32::try_from(plane.stride());
            let (Ok(offset), Ok(stride)) = (offset, stride) else {
                return Err("the plane layout does not fit the protocol".to_string());
            };
            planes.push(PlaneLayout { offset, stride });
        }
        let image = self
            .vulkan
            .import(fd.as_fd(), entry.format, layout, extent)?;
        let params = dmabuf.create_params();
        let modifier = split(entry.modifier);
        for (plane_idx, plane) in planes.iter().enumerate() {
            params.add(
                fd.as_fd(),
                plane_idx as u32,
                plane.offset,
                plane.stride,
                modifier.hi,
                modifier.lo,
            );
        }
        let wl_buffer = params.create_immed(
            extent.width as i32,
            extent.height as i32,
            u32::from_le_bytes(entry.format.fourcc),
            ZwpLinuxBufferParamsV1Flags::empty(),
        );
        params.destroy();
        let generation = self.generation;
        proxy::set_event_handler_local(
            &wl_buffer,
            WlBuffer::on_release(move |state: &mut State, _| {
                state.window().buffer_released(generation, index);
            }),
        );
        let release = self
            .syncobj_manager
            .as_ref()
            .map(|manager| Timeline::new(&self.vulkan, manager));
        Ok(Buffer {
            dmabuf: fd,
            wl_buffer,
            image,
            slot: self.vulkan.create_slot(),
            busy: false,
            release,
            release_point: 0,
        })
    }

    fn destroy_buffer(&self, buffer: &Buffer) {
        buffer.wl_buffer.destroy();
        if let Some(release) = &buffer.release {
            release.destroy(&self.vulkan);
        }
        self.vulkan.destroy_slot(&buffer.slot);
        self.vulkan.destroy_image(&buffer.image);
    }

    fn destroy_swapchain(&mut self) {
        let Some(swapchain) = self.swapchain.take() else {
            return;
        };
        self.vulkan.wait_idle();
        for buffer in &swapchain.buffers {
            self.destroy_buffer(buffer);
        }
    }

    pub fn buffer_released(&mut self, generation: u64, index: usize) {
        if let Some(swapchain) = &mut self.swapchain
            && swapchain.generation == generation
        {
            swapchain.buffers[index].busy = false;
        }
    }

    fn free_buffer(&self) -> Option<usize> {
        let swapchain = self.swapchain.as_ref()?;
        swapchain.buffers.iter().position(|b| match &b.release {
            Some(release) => self.vulkan.timeline_value(release.semaphore) >= b.release_point,
            None => !b.busy,
        })
    }

    pub fn render(
        &mut self,
        angle: f32,
        syncobj_surface: Option<&WpLinuxDrmSyncobjSurfaceV1>,
    ) -> Option<&WlBuffer> {
        let index = self.free_buffer()?;
        let swapchain = self.swapchain.as_mut()?;
        let extent = &swapchain.extent;
        let format = swapchain.entry.format;
        let buffer = &mut swapchain.buffers[index];
        match (&self.acquire, &buffer.release, syncobj_surface) {
            (Some(acquire), Some(release), Some(surface)) => {
                self.acquire_point += 1;
                let sync = vulkan::Sync::Explicit {
                    semaphore: acquire.semaphore,
                    point: self.acquire_point,
                };
                self.vulkan
                    .render(&buffer.slot, &buffer.image, format, extent, angle, sync);
                let point = split(self.acquire_point);
                surface.set_acquire_point(&acquire.proxy, point.hi, point.lo);
                buffer.release_point += 1;
                let point = split(buffer.release_point);
                surface.set_release_point(&release.proxy, point.hi, point.lo);
            }
            _ => {
                let fence = dmabuf::export_write_fence(buffer.dmabuf.as_fd())
                    .expect("DMA_BUF_IOCTL_EXPORT_SYNC_FILE failed");
                let sync = vulkan::Sync::Implicit(fence);
                let done = self
                    .vulkan
                    .render(&buffer.slot, &buffer.image, format, extent, angle, sync)
                    .unwrap();
                dmabuf::import_write_fence(buffer.dmabuf.as_fd(), done.as_fd())
                    .expect("DMA_BUF_IOCTL_IMPORT_SYNC_FILE failed");
                buffer.busy = true;
            }
        }
        Some(&buffer.wl_buffer)
    }
}

impl Drop for Gpu {
    fn drop(&mut self) {
        self.destroy_swapchain();
        if let Some(acquire) = &self.acquire {
            acquire.destroy(&self.vulkan);
        }
    }
}
