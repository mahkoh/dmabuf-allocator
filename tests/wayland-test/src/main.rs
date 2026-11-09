mod dmabuf;
mod feedback;
mod formats;
mod gpu;
mod vulkan;

#[allow(dead_code, clippy::all)]
mod protocols {
    include!(concat!(env!("OUT_DIR"), "/wayland-protocols/mod.rs"));
}

use crate::feedback::FeedbackBuilder;
use crate::gpu::Gpu;
use crate::protocols::cursor_shape_v1::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1;
use crate::protocols::cursor_shape_v1::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1Shape;
use crate::protocols::cursor_shape_v1::wp_cursor_shape_manager_v1::WpCursorShapeManagerV1;
use crate::protocols::fractional_scale_v1::wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1;
use crate::protocols::fractional_scale_v1::wp_fractional_scale_v1::WpFractionalScaleV1;
use crate::protocols::linux_dmabuf_v1::zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1;
use crate::protocols::linux_dmabuf_v1::zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1EventHandler;
use crate::protocols::linux_dmabuf_v1::zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1Ref;
use crate::protocols::linux_dmabuf_v1::zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1TrancheFlags;
use crate::protocols::linux_dmabuf_v1::zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1;
use crate::protocols::linux_drm_syncobj_v1::wp_linux_drm_syncobj_manager_v1::WpLinuxDrmSyncobjManagerV1;
use crate::protocols::linux_drm_syncobj_v1::wp_linux_drm_syncobj_surface_v1::WpLinuxDrmSyncobjSurfaceV1;
use crate::protocols::viewporter::wp_viewport::WpViewport;
use crate::protocols::viewporter::wp_viewporter::WpViewporter;
use crate::protocols::wayland::wl_callback::WlCallback;
use crate::protocols::wayland::wl_compositor::WlCompositor;
use crate::protocols::wayland::wl_display::WlDisplay;
use crate::protocols::wayland::wl_keyboard::WlKeyboard;
use crate::protocols::wayland::wl_keyboard::WlKeyboardKeyState;
use crate::protocols::wayland::wl_pointer::WlPointer;
use crate::protocols::wayland::wl_registry::WlRegistry;
use crate::protocols::wayland::wl_registry::WlRegistryEventHandler;
use crate::protocols::wayland::wl_registry::WlRegistryRef;
use crate::protocols::wayland::wl_seat::WlSeat;
use crate::protocols::wayland::wl_seat::WlSeatCapability;
use crate::protocols::wayland::wl_surface::WlSurface;
use crate::protocols::xdg_decoration_unstable_v1::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1;
use crate::protocols::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1;
use crate::protocols::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1Mode;
use crate::protocols::xdg_shell::xdg_surface::XdgSurface;
use crate::protocols::xdg_shell::xdg_toplevel::XdgToplevel;
use crate::protocols::xdg_shell::xdg_toplevel::XdgToplevelEventHandler;
use crate::protocols::xdg_shell::xdg_toplevel::XdgToplevelRef;
use crate::protocols::xdg_shell::xdg_wm_base::XdgWmBase;
use std::collections::HashMap;
use std::io;
use std::os::fd::AsRawFd;
use std::os::fd::OwnedFd;
use std::time::Instant;
use wl_client::Libwayland;
use wl_client::proxy;
use wl_client::proxy::OwnedProxy;

#[cfg(test)]
mod tests;

const DEFAULT_SIZE: Size = Size {
    width: 800,
    height: 600,
};
const SCALE_DENOMINATOR: u32 = 120;
const KEY_UP: u32 = 103;
const KEY_DOWN: u32 = 108;
const RELEASE_POLL_MS: i32 = 1;

#[derive(Copy, Clone)]
struct Size {
    width: i32,
    height: i32,
}

#[derive(Copy, Clone)]
struct Global {
    name: u32,
    version: u32,
}

struct State {
    exit: bool,
    registry: WlRegistry,
    globals: HashMap<String, Global>,
    initial_seats: Vec<Global>,
    cursor_shape: Option<WpCursorShapeManagerV1>,
    seats: HashMap<u32, Seat>,
    window: Option<Window>,
}

struct Pointer {
    pointer: WlPointer,
    shape: WpCursorShapeDeviceV1,
}

struct Seat {
    seat: WlSeat,
    pointer: Option<Pointer>,
    keyboard: Option<WlKeyboard>,
}

fn scaled(size: i32, scale: u32) -> u32 {
    (size as u32 * scale + SCALE_DENOMINATOR / 2) / SCALE_DENOMINATOR
}

struct Window {
    _compositor: WlCompositor,
    wm_base: XdgWmBase,
    dmabuf: ZwpLinuxDmabufV1,
    viewporter: Option<WpViewporter>,
    fractional_scale_manager: Option<WpFractionalScaleManagerV1>,
    decoration_manager: Option<ZxdgDecorationManagerV1>,
    syncobj_manager: Option<WpLinuxDrmSyncobjManagerV1>,
    surface: WlSurface,
    xdg_surface: XdgSurface,
    toplevel: XdgToplevel,
    viewport: Option<WpViewport>,
    fractional_scale: Option<WpFractionalScaleV1>,
    decoration: Option<ZxdgToplevelDecorationV1>,
    syncobj_surface: Option<WpLinuxDrmSyncobjSurfaceV1>,
    feedback_proxy: ZwpLinuxDmabufFeedbackV1,
    feedback_builder: FeedbackBuilder,
    frame_callback: Option<WlCallback>,
    configured: bool,
    pending_size: Size,
    scale: u32,
    waiting_for_buffer: bool,
    title: String,
    start: Instant,
    main_device: Option<u64>,
    gpu: Option<Gpu>,
}

impl State {
    fn window(&mut self) -> &mut Window {
        self.window.as_mut().unwrap()
    }

    fn bind<P: OwnedProxy>(&self, max: u32) -> Option<P> {
        let global = self.globals.get(P::INTERFACE)?;
        Some(self.registry.bind(global.name, global.version.min(max)))
    }

    fn add_seat(&mut self, name: u32, version: u32) {
        let seat: WlSeat = self.registry.bind(name, version.min(5));
        proxy::set_event_handler_local(
            &seat,
            WlSeat::on_capabilities(move |state: &mut State, _, capabilities| {
                state.seat_capabilities(name, capabilities);
            }),
        );
        self.seats.insert(
            name,
            Seat {
                seat,
                pointer: None,
                keyboard: None,
            },
        );
    }

    fn seat_capabilities(&mut self, name: u32, capabilities: WlSeatCapability) {
        let Some(seat) = self.seats.get_mut(&name) else {
            return;
        };
        let has_keyboard = capabilities.contains(WlSeatCapability::KEYBOARD);
        if has_keyboard && seat.keyboard.is_none() {
            let keyboard = seat.seat.get_keyboard();
            proxy::set_event_handler_local(
                &keyboard,
                WlKeyboard::on_key(|state: &mut State, _, _, _, key, key_state| {
                    if key_state == WlKeyboardKeyState::PRESSED {
                        state.window().key(key);
                    }
                }),
            );
            seat.keyboard = Some(keyboard);
        } else if !has_keyboard {
            seat.release_keyboard();
        }
        let Some(cursor_shape) = &self.cursor_shape else {
            return;
        };
        let has_pointer = capabilities.contains(WlSeatCapability::POINTER);
        if has_pointer && seat.pointer.is_none() {
            let pointer = seat.seat.get_pointer();
            let shape = cursor_shape.get_pointer(&pointer);
            proxy::set_event_handler_local(
                &pointer,
                WlPointer::on_enter(move |state: &mut State, _, serial, _, _, _| {
                    if let Some(seat) = state.seats.get(&name)
                        && let Some(pointer) = &seat.pointer
                    {
                        pointer
                            .shape
                            .set_shape(serial, WpCursorShapeDeviceV1Shape::DEFAULT);
                    }
                }),
            );
            seat.pointer = Some(Pointer { pointer, shape });
        } else if !has_pointer {
            seat.release_pointer();
        }
    }
}

impl Seat {
    fn release_keyboard(&mut self) {
        if let Some(keyboard) = self.keyboard.take()
            && proxy::version(&*keyboard) >= WlKeyboard::REQ__RELEASE__SINCE
        {
            keyboard.release();
        }
    }

    fn release_pointer(&mut self) {
        if let Some(Pointer { pointer, shape }) = self.pointer.take() {
            shape.destroy();
            if proxy::version(&*pointer) >= WlPointer::REQ__RELEASE__SINCE {
                pointer.release();
            }
        }
    }
}

impl Drop for Seat {
    fn drop(&mut self) {
        self.release_keyboard();
        self.release_pointer();
        if proxy::version(&*self.seat) >= WlSeat::REQ__RELEASE__SINCE {
            self.seat.release();
        }
    }
}

struct RegistryHandler;

impl WlRegistryEventHandler for RegistryHandler {
    type Data = State;

    fn global(
        &self,
        state: &mut State,
        _slf: &WlRegistryRef,
        name: u32,
        interface: &str,
        version: u32,
    ) {
        if interface == WlSeat::INTERFACE {
            if state.window.is_some() {
                state.add_seat(name, version);
            } else {
                state.initial_seats.push(Global { name, version });
            }
        } else {
            let global = Global { name, version };
            state.globals.insert(interface.to_owned(), global);
        }
    }

    fn global_remove(&self, state: &mut State, _slf: &WlRegistryRef, name: u32) {
        state.seats.remove(&name);
    }
}

struct ToplevelHandler;

impl XdgToplevelEventHandler for ToplevelHandler {
    type Data = State;

    fn configure(
        &self,
        state: &mut State,
        _slf: &XdgToplevelRef,
        width: i32,
        height: i32,
        _states: &[u8],
    ) {
        let window = state.window();
        if width > 0 && height > 0 {
            window.pending_size = Size { width, height };
        }
    }

    fn close(&self, state: &mut State, _slf: &XdgToplevelRef) {
        state.exit = true;
    }
}

struct FeedbackHandler;

impl ZwpLinuxDmabufFeedbackV1EventHandler for FeedbackHandler {
    type Data = State;

    fn done(&self, state: &mut State, _slf: &ZwpLinuxDmabufFeedbackV1Ref) {
        let window = state.window();
        let feedback = window.feedback_builder.done();
        if window.main_device != Some(feedback.main_device) {
            window.create_gpu(feedback.main_device);
        }
        window.gpu.as_mut().unwrap().set_feedback(&feedback);
        window.draw();
    }

    fn format_table(
        &self,
        state: &mut State,
        _slf: &ZwpLinuxDmabufFeedbackV1Ref,
        fd: OwnedFd,
        size: u32,
    ) {
        state.window().feedback_builder.format_table(fd, size);
    }

    fn main_device(&self, state: &mut State, _slf: &ZwpLinuxDmabufFeedbackV1Ref, device: &[u8]) {
        state.window().feedback_builder.main_device(device);
    }

    fn tranche_done(&self, state: &mut State, _slf: &ZwpLinuxDmabufFeedbackV1Ref) {
        state.window().feedback_builder.tranche_done();
    }

    fn tranche_target_device(
        &self,
        state: &mut State,
        _slf: &ZwpLinuxDmabufFeedbackV1Ref,
        device: &[u8],
    ) {
        state
            .window()
            .feedback_builder
            .tranche_target_device(device);
    }

    fn tranche_formats(
        &self,
        state: &mut State,
        _slf: &ZwpLinuxDmabufFeedbackV1Ref,
        indices: &[u8],
    ) {
        state.window().feedback_builder.tranche_formats(indices);
    }

    fn tranche_flags(
        &self,
        state: &mut State,
        _slf: &ZwpLinuxDmabufFeedbackV1Ref,
        flags: ZwpLinuxDmabufFeedbackV1TrancheFlags,
    ) {
        let scanout = flags.contains(ZwpLinuxDmabufFeedbackV1TrancheFlags::SCANOUT);
        state.window().feedback_builder.tranche_scanout(scanout);
    }
}

impl Window {
    fn new(state: &State) -> Window {
        let compositor: WlCompositor = state.bind(6).expect("wl_compositor is required");
        assert!(
            proxy::version(&*compositor) >= WlSurface::REQ__DAMAGE_BUFFER__SINCE,
            "wl_compositor must support wl_surface.damage_buffer",
        );
        let wm_base: XdgWmBase = state.bind(6).expect("xdg_wm_base is required");
        let dmabuf: ZwpLinuxDmabufV1 = state.bind(5).expect("zwp_linux_dmabuf_v1 is required");
        assert!(
            proxy::version(&*dmabuf) >= ZwpLinuxDmabufV1::REQ__GET_SURFACE_FEEDBACK__SINCE,
            "zwp_linux_dmabuf_v1 must support feedback",
        );
        let viewporter: Option<WpViewporter> = state.bind(1);
        let fractional_scale_manager: Option<WpFractionalScaleManagerV1> = state.bind(1);
        let decoration_manager: Option<ZxdgDecorationManagerV1> = state.bind(1);
        let syncobj_manager: Option<WpLinuxDrmSyncobjManagerV1> = state.bind(1);
        proxy::set_event_handler_local(
            &wm_base,
            XdgWmBase::on_ping(|_: &mut State, slf, serial| slf.pong(serial)),
        );

        let surface = compositor.create_surface();
        let xdg_surface = wm_base.get_xdg_surface(&surface);
        proxy::set_event_handler_local(
            &xdg_surface,
            XdgSurface::on_configure(|state: &mut State, slf, serial| {
                slf.ack_configure(serial);
                let window = state.window();
                window.configured = true;
                window.draw();
            }),
        );
        let toplevel = xdg_surface.get_toplevel();
        proxy::set_event_handler_local(&toplevel, ToplevelHandler);
        toplevel.set_title("dmabuf-allocator");
        toplevel.set_app_id("wayland-test");
        let decoration = decoration_manager.as_ref().map(|manager| {
            let decoration = manager.get_toplevel_decoration(&toplevel);
            decoration.set_mode(ZxdgToplevelDecorationV1Mode::SERVER_SIDE);
            decoration
        });
        let viewport = viewporter
            .as_ref()
            .map(|viewporter| viewporter.get_viewport(&surface));
        let fractional_scale = match (&fractional_scale_manager, &viewport) {
            (Some(manager), Some(_)) => {
                let scale = manager.get_fractional_scale(&surface);
                proxy::set_event_handler_local(
                    &scale,
                    WpFractionalScaleV1::on_preferred_scale(|state: &mut State, _, scale| {
                        let window = state.window();
                        window.scale = scale;
                        window.draw();
                    }),
                );
                Some(scale)
            }
            _ => None,
        };
        let feedback_proxy = dmabuf.get_surface_feedback(&surface);
        proxy::set_event_handler_local(&feedback_proxy, FeedbackHandler);
        surface.commit();
        Window {
            _compositor: compositor,
            wm_base,
            dmabuf,
            viewporter,
            fractional_scale_manager,
            decoration_manager,
            syncobj_manager,
            surface,
            xdg_surface,
            toplevel,
            viewport,
            fractional_scale,
            decoration,
            syncobj_surface: None,
            feedback_proxy,
            feedback_builder: Default::default(),
            frame_callback: None,
            configured: false,
            pending_size: DEFAULT_SIZE,
            scale: SCALE_DENOMINATOR,
            waiting_for_buffer: false,
            title: String::new(),
            start: Instant::now(),
            main_device: None,
            gpu: None,
        }
    }

    fn create_gpu(&mut self, main_device: u64) {
        if let Some(manager) = &self.syncobj_manager
            && self.syncobj_surface.is_none()
        {
            self.syncobj_surface = Some(manager.get_surface(&self.surface));
        }
        self.gpu = None;
        self.gpu = Some(Gpu::new(main_device, self.syncobj_manager.clone()));
        self.main_device = Some(main_device);
    }

    fn polls_for_release(&self) -> bool {
        self.waiting_for_buffer && self.gpu.as_ref().is_some_and(Gpu::explicit_sync)
    }

    fn draw(&mut self) {
        if !self.configured || self.frame_callback.is_some() {
            return;
        }
        let Some(gpu) = &mut self.gpu else {
            return;
        };
        let Size { width, height } = self.pending_size;
        let buffer_width = scaled(width, self.scale);
        let buffer_height = scaled(height, self.scale);
        gpu.update(&self.dmabuf, buffer_width, buffer_height);
        let title = gpu.title();
        if title != self.title {
            self.toplevel.set_title(&title);
            self.title = title;
        }
        let angle = self.start.elapsed().as_secs_f32();
        let Some(buffer) = gpu.render(angle, self.syncobj_surface.as_ref()) else {
            self.waiting_for_buffer = true;
            return;
        };
        self.waiting_for_buffer = false;
        let callback = self.surface.frame();
        proxy::set_event_handler_local(
            &callback,
            WlCallback::on_done(|state: &mut State, _, _| {
                let window = state.window();
                window.frame_callback = None;
                window.draw();
            }),
        );
        self.frame_callback = Some(callback);
        self.surface.attach(Some(buffer), 0, 0);
        self.surface.damage_buffer(0, 0, i32::MAX, i32::MAX);
        if let Some(viewport) = &self.viewport {
            viewport.set_destination(width, height);
        }
        self.surface.commit();
    }

    fn key(&mut self, key: u32) {
        let Some(gpu) = &mut self.gpu else {
            return;
        };
        match key {
            KEY_UP => gpu.faster(),
            KEY_DOWN => gpu.slower(),
            _ => return,
        }
        let title = gpu.title();
        self.toplevel.set_title(&title);
        self.title = title;
    }

    fn buffer_released(&mut self, generation: u64, index: usize) {
        if let Some(gpu) = &mut self.gpu {
            gpu.buffer_released(generation, index);
        }
        if self.waiting_for_buffer {
            self.draw();
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.gpu = None;
        if let Some(syncobj_surface) = &self.syncobj_surface {
            syncobj_surface.destroy();
        }
        self.feedback_proxy.destroy();
        if let Some(scale) = &self.fractional_scale {
            scale.destroy();
        }
        if let Some(viewport) = &self.viewport {
            viewport.destroy();
        }
        if let Some(decoration) = &self.decoration {
            decoration.destroy();
        }
        self.toplevel.destroy();
        self.xdg_surface.destroy();
        self.surface.destroy();
        if let Some(manager) = &self.syncobj_manager {
            manager.destroy();
        }
        if let Some(manager) = &self.decoration_manager {
            manager.destroy();
        }
        if let Some(manager) = &self.fractional_scale_manager {
            manager.destroy();
        }
        if let Some(viewporter) = &self.viewporter {
            viewporter.destroy();
        }
        self.dmabuf.destroy();
        self.wm_base.destroy();
    }
}

fn main() {
    let lib = Libwayland::open().expect("could not load libwayland-client");
    let con = lib
        .connect_to_default_display()
        .expect("could not connect to the compositor");
    let (_queue_owner, queue) = con.create_local_queue_with_data::<State>(c"wayland-test");
    let registry = queue.display::<WlDisplay>().get_registry();
    proxy::set_event_handler_local(&registry, RegistryHandler);
    let mut state = State {
        exit: false,
        registry,
        globals: HashMap::new(),
        initial_seats: Vec::new(),
        cursor_shape: None,
        seats: HashMap::new(),
        window: None,
    };
    queue.dispatch_roundtrip_blocking(&mut state).unwrap();
    state.cursor_shape = state.bind(1);
    state.window = Some(Window::new(&state));
    for seat in std::mem::take(&mut state.initial_seats) {
        state.add_seat(seat.name, seat.version);
    }
    let watcher = queue
        .create_watcher()
        .expect("could not watch the event queue");
    while !state.exit {
        con.flush().unwrap();
        let timeout = match state.window().polls_for_release() {
            true => RELEASE_POLL_MS,
            false => -1,
        };
        let mut poll = libc::pollfd {
            fd: watcher.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        if unsafe { libc::poll(&mut poll, 1, timeout) } < 0 {
            let error = io::Error::last_os_error();
            assert_eq!(error.kind(), io::ErrorKind::Interrupted, "poll: {error}");
            continue;
        }
        queue.dispatch_pending(&mut state).unwrap();
        watcher.reset().unwrap();
        let window = state.window();
        if window.waiting_for_buffer {
            window.draw();
        }
    }
    state.window = None;
    state.seats.clear();
    if let Some(cursor_shape) = &state.cursor_shape {
        cursor_shape.destroy();
    }
}
