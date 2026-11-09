pub mod amd;
pub mod intel;
pub mod nvidia;

pub use allocator_trace::Call;
pub use allocator_trace::Region;
use dmabuf_allocator_impl::allocator::Allocator;
pub use dmabuf_allocator_impl::allocator::AllocatorError;
pub use dmabuf_allocator_impl::allocator::AmdgpuAllocateError;
pub use dmabuf_allocator_impl::allocator::AmdgpuGfxVersionError;
pub use dmabuf_allocator_impl::allocator::AmdgpuOpenError;
pub use dmabuf_allocator_impl::allocator::AmdgpuQueryHwIpError;
pub use dmabuf_allocator_impl::allocator::AmdgpuReadRegistersError;
pub use dmabuf_allocator_impl::allocator::AmdgpuVirtioCommandError;
pub use dmabuf_allocator_impl::allocator::AmdgpuVirtioOpenError;
pub use dmabuf_allocator_impl::allocator::AsahiError;
use dmabuf_allocator_impl::allocator::Config;
pub use dmabuf_allocator_impl::allocator::DriverName;
pub use dmabuf_allocator_impl::allocator::DrmError;
pub use dmabuf_allocator_impl::allocator::DumbError;
pub use dmabuf_allocator_impl::allocator::EtnavivError;
pub use dmabuf_allocator_impl::allocator::I915Error;
pub use dmabuf_allocator_impl::allocator::Kernel;
pub use dmabuf_allocator_impl::allocator::LimaError;
pub use dmabuf_allocator_impl::allocator::PanfrostError;
pub use dmabuf_allocator_impl::allocator::UdmabufError;
pub use dmabuf_allocator_impl::allocator::Vc4Error;
pub use dmabuf_allocator_impl::allocator::VdrmError;
pub use dmabuf_allocator_impl::allocator::VirtioGpuError;
pub use dmabuf_allocator_impl::errno::Errno;
pub use dmabuf_allocator_impl::format::Format;
use dmabuf_allocator_impl::format::get_format;
pub use dmabuf_allocator_impl::formats::FORMATS;
pub use dmabuf_allocator_impl::layout::Layout;
pub use dmabuf_allocator_impl::layout::LayoutError;
use error_reporter::Report;
pub use fake_device::DEVICE_FD;
pub use fake_device::FakeDevice;
pub use fake_device::Fault;
pub use fake_kernel::KernelVersion;
use fake_kernel::Node;
pub use fake_kernel::uapi;
pub use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_INVALID as INVALID;
pub use fake_kernel::uapi::drm_fourcc::DRM_FORMAT_MOD_LINEAR as LINEAR;
use std::thread;

pub const PAGE_SIZE: u64 = 4096;

const AMDGPU_PTE_FRAGMENT_SIZE: u64 = 2097152;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Plane {
    pub offset: u64,
    pub stride: u64,
    pub size: u64,
}

impl Plane {
    pub const fn at(offset: u64, stride: u64, size: u64) -> Self {
        Self {
            offset,
            stride,
            size,
        }
    }

    pub const fn first(stride: u64, size: u64) -> Self {
        Self::at(0, stride, size)
    }

    pub const fn pitch(&self) -> Pitch {
        Pitch {
            offset: self.offset,
            stride: self.stride,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pitch {
    pub offset: u64,
    pub stride: u64,
}

pub fn planes(layout: &Layout) -> Vec<Plane> {
    layout
        .planes
        .iter()
        .map(|plane| Plane::at(plane.offset, plane.stride, plane.size))
        .collect()
}

pub fn fourcc(format: Format) -> String {
    String::from_utf8(format.0.fourcc.to_le_bytes().to_vec()).unwrap()
}

pub fn request(fourcc: &str, modifiers: &[u64], width: u32, height: u32) -> Request {
    Request::new(fourcc, width, height).modifiers(modifiers)
}

pub struct Request {
    format: Format,
    modifiers: Vec<u64>,
    width: u32,
    height: u32,
    scanout: bool,
    implicit_sync: bool,
    linear: bool,
    implicit_interop: bool,
}

impl Request {
    pub fn new(fourcc: &str, width: u32, height: u32) -> Self {
        let bytes: [u8; 4] = fourcc
            .as_bytes()
            .try_into()
            .unwrap_or_else(|_| panic!("fourcc {fourcc:?} must be 4 bytes"));
        let format = get_format(u32::from_le_bytes(bytes))
            .unwrap_or_else(|| panic!("unknown format {fourcc:?}"));
        Self::for_format(format, width, height)
    }

    pub fn for_format(format: Format, width: u32, height: u32) -> Self {
        Self {
            format,
            modifiers: Vec::new(),
            width,
            height,
            scanout: false,
            implicit_sync: false,
            linear: false,
            implicit_interop: false,
        }
    }

    pub fn modifiers(mut self, modifiers: &[u64]) -> Self {
        self.modifiers = modifiers.to_vec();
        self
    }

    pub fn scanout(mut self) -> Self {
        self.scanout = true;
        self
    }

    pub fn implicit_sync(mut self) -> Self {
        self.implicit_sync = true;
        self
    }

    pub fn linear(mut self) -> Self {
        self.linear = true;
        self
    }

    pub fn implicit_interop(mut self) -> Self {
        self.implicit_interop = true;
        self
    }

    pub(crate) fn config(&self) -> Config<'_> {
        Config {
            format: self.format,
            modifiers: &self.modifiers,
            width: self.width,
            height: self.height,
            scanout: self.scanout,
            implicit_sync: self.implicit_sync,
            linear: self.linear,
            implicit_interop: self.implicit_interop,
            name: None,
        }
    }
}

pub struct Buffer {
    pub layout: Layout,
    pub calls: Vec<Call>,
}

impl Buffer {
    pub fn modifier(&self) -> u64 {
        self.layout.modifier
    }

    pub fn planes(&self) -> Vec<Plane> {
        planes(&self.layout)
    }

    pub fn pitches(&self) -> Vec<Pitch> {
        self.planes().iter().map(Plane::pitch).collect()
    }

    pub fn only<T>(&self, select: impl Fn(&Call) -> Option<T>) -> T {
        let mut selected: Vec<T> = self.calls.iter().filter_map(select).collect();
        assert_eq!(
            selected.len(),
            1,
            "not exactly one matching call in {:?}",
            self.calls
        );
        selected.remove(0)
    }
}

pub struct DeviceBuilder {
    kernel: Box<dyn fake_kernel::Kernel>,
    faults: Vec<Fault>,
    render_node_errno: Option<i32>,
    host_faults: Vec<Fault>,
    #[allow(clippy::type_complexity)]
    before_open: Option<Box<dyn FnOnce(&FakeDevice)>>,
}

impl DeviceBuilder {
    pub fn before_open(mut self, setup: impl FnOnce(&FakeDevice) + 'static) -> Self {
        self.before_open = Some(Box::new(setup));
        self
    }

    pub fn fail_render_node(mut self, errno: i32) -> Self {
        self.render_node_errno = Some(errno);
        self
    }

    pub fn fail_host(mut self, fault: Fault) -> Self {
        self.host_faults.push(fault);
        self
    }

    pub fn fail(mut self, fault: Fault) -> Self {
        self.faults.push(fault);
        self
    }

    pub fn open(self) -> Result<Device, AllocatorError> {
        let node = match self.kernel.render_node() {
            true => Node::Render,
            false => Node::Primary,
        };
        let fake = FakeDevice::new(self.kernel, node);
        if let Some(errno) = self.render_node_errno {
            fake.fail_render_node(errno);
        }
        for fault in self.host_faults {
            fake.fail_host(fault);
        }
        for fault in self.faults {
            fake.fail(fault);
        }
        if let Some(setup) = self.before_open {
            setup(&fake);
        }
        match Allocator::with_kernel(fake.clone(), DEVICE_FD) {
            Ok(allocator) => Ok(Device {
                allocator,
                fake: Checked(fake),
            }),
            Err(error) => {
                check(&fake);
                Err(error)
            }
        }
    }
}

pub struct Device {
    allocator: Allocator<FakeDevice>,
    fake: Checked,
}

impl Device {
    pub fn new(id: &str) -> Self {
        Self::builder(id).open().unwrap_or_else(|error| {
            panic!(
                "cannot create the allocator on {id}: {}",
                Report::new(error)
            )
        })
    }

    pub fn builder(id: &str) -> DeviceBuilder {
        Self::with_kernel_box(fake_kernel::from_id(&complete_id(id)))
    }

    pub fn with_kernel(kernel: impl fake_kernel::Kernel + 'static) -> DeviceBuilder {
        Self::with_kernel_box(Box::new(kernel))
    }

    fn with_kernel_box(kernel: Box<dyn fake_kernel::Kernel>) -> DeviceBuilder {
        DeviceBuilder {
            kernel,
            faults: Vec::new(),
            render_node_errno: None,
            host_faults: Vec::new(),
            before_open: None,
        }
    }

    pub fn id(&self) -> String {
        self.fake.0.fake().id()
    }

    pub fn fake(&self) -> &FakeDevice {
        &self.fake.0
    }

    pub fn fail_host(&self, fault: Fault) {
        self.fake.0.fail_host(fault);
    }

    pub fn fail(&self, fault: Fault) {
        self.fake.0.fail(fault);
    }

    pub fn driver(&self) -> DriverName {
        self.allocator.driver()
    }

    pub fn layout(&self, request: &Request) -> Option<Layout> {
        self.allocator.layout(&request.config()).ok()
    }

    pub fn chosen(&self, request: &Request) -> Option<u64> {
        self.layout(request).map(|layout| layout.modifier)
    }

    pub fn allocate(&self, request: &Request) -> Result<Buffer, AllocatorError> {
        self.fake.0.take_calls();
        let layout = self
            .allocator
            .allocate(&request.config())
            .map(|allocation| allocation.layout);
        let calls = self.fake.0.take_calls();
        Ok(Buffer {
            layout: layout?,
            calls: calls.into_iter().map(|traced| traced.call).collect(),
        })
    }
}

struct Checked(FakeDevice);

impl Drop for Checked {
    fn drop(&mut self) {
        if !thread::panicking() {
            check(&self.0);
        }
    }
}

fn check(fake: &FakeDevice) {
    if let Some(error) = fake.leak_error().or_else(|| fake.handle_error()) {
        panic!("{}: {error}", fake.fake().id());
    }
}

pub(crate) fn complete_id(id: &str) -> String {
    let has = |key: &str| id.split(':').any(|field| field.starts_with(key));
    let mut completed = id.to_string();
    if !has("kernel=") {
        completed.push_str(&format!(":kernel={}", KernelVersion::NEWEST));
    }
    if id.starts_with("panfrost:") && !has("afbc_features=") {
        completed.push_str(":afbc_features=0");
    }
    if id.starts_with("amdgpu:") && !has("pte_fragment_size=") {
        completed.push_str(&format!(":pte_fragment_size={AMDGPU_PTE_FRAGMENT_SIZE}"));
    }
    completed
}
