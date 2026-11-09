use crate::Request;
use crate::complete_id;
pub use dmabuf_allocator_impl::drivers::intel::device_info::DeviceInfo;
pub use dmabuf_allocator_impl::drivers::intel::devices::DEVICES;
use dmabuf_allocator_impl::drivers::intel::layout::compute_layout;
use dmabuf_allocator_impl::layout::Layout;
pub use fake_kernel::intel::devices::DEVICES as KERNEL_DEVICES;
pub use fake_kernel::intel::devices::Device as DeviceValues;
pub use fake_kernel::intel::kernel::Device as IntelKernel;
pub use fake_kernel::intel::kernel::XE_VERX10;

pub fn kernel(id: &str) -> IntelKernel {
    let id = complete_id(id);
    let fields = id
        .strip_prefix("intel:")
        .unwrap_or_else(|| panic!("{id} is not an intel device"));
    IntelKernel::from_id_unchecked(fields)
}

pub fn layout(request: &Request, modifier: u64, info: &DeviceInfo) -> Option<Layout> {
    compute_layout(&request.config(), modifier, info)
}
