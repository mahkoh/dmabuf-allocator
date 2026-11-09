use crate::cases::exported_calls;
use crate::cases::vulkan_flags;
use crate::drm;
use crate::formats::Block;
use crate::formats::FORMATS;
use crate::formats::VulkanFormat;
use crate::formats::fourcc_code;
use crate::formats::largest_block;
use crate::generate;
use crate::generate::Evaluation;
use crate::generate::Layout;
use crate::generate::MAX_PLANES;
use crate::generate::PairInfo;
use crate::generate::PlaneLayout;
use crate::sizes::Size;
use allocator_trace::vectors;
use ash::vk;
use error_reporter::Report;
use fake_supervisor::DrmNode;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_void;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Range;
use std::os::fd::FromRawFd;
use std::os::fd::OwnedFd;

#[cfg(test)]
mod tests;

const MEMORY_PLANES: [vk::ImageAspectFlags; MAX_PLANES] = [
    vk::ImageAspectFlags::MEMORY_PLANE_0_EXT,
    vk::ImageAspectFlags::MEMORY_PLANE_1_EXT,
    vk::ImageAspectFlags::MEMORY_PLANE_2_EXT,
    vk::ImageAspectFlags::MEMORY_PLANE_3_EXT,
];

pub trait Driver {
    type Device: Copy;

    fn icd(&self) -> &Icd;
    fn nodes(&self) -> Range<usize>;
    fn device(&self, node: u32) -> Self::Device;
    fn rank(&self, device: Self::Device) -> u64;
    fn skip(&self, modifier: u64, device: Self::Device) -> bool;
    fn size_set(&self, modifier: u64, planes: &[Block], device: Self::Device) -> BTreeSet<Size>;
    fn output_file(&self, modifier: u64) -> &'static str;
    fn exact_export_size(&self) -> bool {
        false
    }
    fn variants(&self, _node: u32) -> Vec<String> {
        Vec::new()
    }
}

type IcdNegotiate = unsafe extern "C" fn(*mut u32) -> vk::Result;

const ICD_INTERFACE_VERSION: u32 = 7;
const VULKAN_LOADER: &CStr = c"libvulkan.so.1";

pub struct Icd {
    entry: ash::Entry,
    get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddrLUNARG,
}

fn open_library(path: &CStr) -> *mut c_void {
    let library = unsafe { libc::dlopen(path.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL) };
    assert!(!library.is_null(), "cannot load {path:?}");
    library
}

fn symbol(library: *mut c_void, name: &CStr) -> Option<*mut c_void> {
    let symbol = unsafe { libc::dlsym(library, name.as_ptr()) };
    (!symbol.is_null()).then_some(symbol)
}

pub unsafe fn load_icd(path: &str) -> Icd {
    let path = CString::new(path).expect("the ICD path contains a NUL byte");
    let library = open_library(&path);
    if let Some(negotiate) = symbol(library, c"vk_icdNegotiateLoaderICDInterfaceVersion") {
        let negotiate = unsafe { std::mem::transmute::<*mut c_void, IcdNegotiate>(negotiate) };
        let mut version = ICD_INTERFACE_VERSION;
        let result = unsafe { negotiate(&mut version) };
        assert_eq!(
            result,
            vk::Result::SUCCESS,
            "{path:?} rejects loader interface version {ICD_INTERFACE_VERSION}"
        );
    }
    let proc_addr = symbol(library, c"vk_icdGetInstanceProcAddr")
        .unwrap_or_else(|| panic!("{path:?} has no vk_icdGetInstanceProcAddr"));
    let loader = open_library(VULKAN_LOADER);
    let loader_proc_addr = symbol(loader, c"vkGetInstanceProcAddr")
        .unwrap_or_else(|| panic!("{VULKAN_LOADER:?} has no vkGetInstanceProcAddr"));
    let loader_proc_addr = unsafe {
        std::mem::transmute::<*mut c_void, vk::PFN_vkGetInstanceProcAddr>(loader_proc_addr)
    };
    let entry = unsafe {
        ash::Entry::from_static_fn(ash::StaticFn {
            get_instance_proc_addr: loader_proc_addr,
        })
    };
    let get_instance_proc_addr = unsafe {
        std::mem::transmute::<*mut c_void, vk::PFN_vkGetInstanceProcAddrLUNARG>(proc_addr)
    };
    Icd {
        entry,
        get_instance_proc_addr,
    }
}

pub struct VulkanDevice<T> {
    instance: ash::Instance,
    physical: vk::PhysicalDevice,
    device: ash::Device,
    maintenance5: bool,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub node: u32,
    pub driver_device: T,
}

impl<T> Drop for VulkanDevice<T> {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

pub struct DeviceUnit {
    pub format_id: usize,
    pub modifier: u64,
    pub plane_count: u32,
    pub max_extent: vk::Extent3D,
    pub sizes: BTreeSet<Size>,
}

pub struct DeviceDescription {
    pub node: u32,
    pub rank: u64,
    pub units: Vec<DeviceUnit>,
}

#[derive(Copy, Clone)]
pub struct Support {
    pub node: u32,
    pub plane_count: u32,
    pub max_extent: vk::Extent3D,
}

pub struct DescribedPair {
    pub format_id: usize,
    pub modifier: u64,
    pub sizes: BTreeSet<Size>,
    pub latest: Support,
    pub nodes: Vec<u32>,
}

impl DescribedPair {
    pub fn format(&self) -> &'static VulkanFormat {
        &FORMATS[self.format_id]
    }
}

#[derive(Default)]
pub struct Described {
    pub pairs: Vec<DescribedPair>,
}

pub struct VulkanPlan<'a> {
    pub described: &'a Described,
}

const DMA_BUF: vk::ExternalMemoryHandleTypeFlags = vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT;

const USAGE: vk::ImageUsageFlags = vk::ImageUsageFlags::TRANSFER_SRC;

fn modifier_properties(
    instance: &ash::Instance,
    physical: vk::PhysicalDevice,
    format: vk::Format,
) -> Vec<vk::DrmFormatModifierPropertiesEXT> {
    let mut list = vk::DrmFormatModifierPropertiesListEXT::default();
    let mut properties = vk::FormatProperties2::default().push_next(&mut list);
    unsafe {
        instance.get_physical_device_format_properties2(physical, format, &mut properties);
    }
    let count = list.drm_format_modifier_count as usize;
    let mut out = vec![vk::DrmFormatModifierPropertiesEXT::default(); count];
    let mut list =
        vk::DrmFormatModifierPropertiesListEXT::default().drm_format_modifier_properties(&mut out);
    let mut properties = vk::FormatProperties2::default().push_next(&mut list);
    unsafe {
        instance.get_physical_device_format_properties2(physical, format, &mut properties);
    }
    out
}

fn image_supported(
    instance: &ash::Instance,
    physical: vk::PhysicalDevice,
    format: vk::Format,
    modifier: u64,
) -> Option<vk::Extent3D> {
    let mut modifier_info = vk::PhysicalDeviceImageDrmFormatModifierInfoEXT::default()
        .drm_format_modifier(modifier)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let mut external_info =
        vk::PhysicalDeviceExternalImageFormatInfo::default().handle_type(DMA_BUF);
    let info = vk::PhysicalDeviceImageFormatInfo2::default()
        .format(format)
        .ty(vk::ImageType::TYPE_2D)
        .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT)
        .usage(USAGE)
        .push_next(&mut modifier_info)
        .push_next(&mut external_info);
    let mut external = vk::ExternalImageFormatProperties::default();
    let mut properties = vk::ImageFormatProperties2::default().push_next(&mut external);
    let supported = unsafe {
        instance
            .get_physical_device_image_format_properties2(physical, &info, &mut properties)
            .is_ok()
    };
    let max_extent = properties.image_format_properties.max_extent;
    let exportable = external
        .external_memory_properties
        .external_memory_features
        .contains(vk::ExternalMemoryFeatureFlags::EXPORTABLE);
    (supported && exportable).then_some(max_extent)
}

fn needs_maintenance5(format: vk::Format) -> bool {
    matches!(
        format,
        vk::Format::A1B5G5R5_UNORM_PACK16 | vk::Format::A8_UNORM
    )
}

fn has_extension(instance: &ash::Instance, physical: vk::PhysicalDevice, name: &CStr) -> bool {
    let extensions = unsafe { instance.enumerate_device_extension_properties(physical) };
    let extensions = extensions.unwrap_or_else(|error| {
        panic!(
            "vkEnumerateDeviceExtensionProperties: {}",
            Report::new(error)
        )
    });
    extensions
        .iter()
        .any(|extension| extension.extension_name_as_c_str() == Ok(name))
}

fn physical_node(instance: &ash::Instance, physical: vk::PhysicalDevice) -> u32 {
    if has_extension(instance, physical, ash::ext::pci_bus_info::NAME) {
        let mut bus = vk::PhysicalDevicePCIBusInfoPropertiesEXT::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut bus);
        unsafe {
            instance.get_physical_device_properties2(physical, &mut properties);
        }
        return bus.pci_device;
    }
    let mut drm = vk::PhysicalDeviceDrmPropertiesEXT::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut drm);
    unsafe {
        instance.get_physical_device_properties2(physical, &mut properties);
    }
    let render = DrmNode::from_minor(drm.render_minor as u32);
    render
        .expect("the render minor names no fake device")
        .device as u32
}

fn extent_valid(format: &VulkanFormat, max_extent: vk::Extent3D, size: Size) -> bool {
    let alignment = largest_block(format.planes);
    size.width.is_multiple_of(alignment.width)
        && size.height.is_multiple_of(alignment.height)
        && size.width <= max_extent.width
        && size.height <= max_extent.height
}

fn create_device(
    instance: &ash::Instance,
    physical: vk::PhysicalDevice,
    maintenance5: bool,
) -> ash::Device {
    let mut extensions = vec![
        ash::ext::image_drm_format_modifier::NAME.as_ptr(),
        ash::khr::external_memory_fd::NAME.as_ptr(),
        ash::ext::external_memory_dma_buf::NAME.as_ptr(),
        ash::khr::maintenance9::NAME.as_ptr(),
    ];
    if maintenance5 {
        extensions.push(ash::khr::maintenance5::NAME.as_ptr());
    }
    let mut maintenance9 = vk::PhysicalDeviceMaintenance9FeaturesKHR::default().maintenance9(true);
    let priorities = [1.0];
    let queues = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(0)
        .queue_priorities(&priorities)];
    let info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queues)
        .enabled_extension_names(&extensions)
        .push_next(&mut maintenance9);
    let device = unsafe { instance.create_device(physical, &info, None) };
    device.unwrap_or_else(|error| panic!("vkCreateDevice: {}", Report::new(error)))
}

fn create_instance(icd: &Icd) -> ash::Instance {
    let application = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3);
    let drivers = [vk::DirectDriverLoadingInfoLUNARG::default()
        .pfn_get_instance_proc_addr(icd.get_instance_proc_addr)];
    let mut direct = vk::DirectDriverLoadingListLUNARG::default()
        .mode(vk::DirectDriverLoadingModeLUNARG::EXCLUSIVE)
        .drivers(&drivers);
    let extensions = [ash::lunarg::direct_driver_loading::NAME.as_ptr()];
    let info = vk::InstanceCreateInfo::default()
        .application_info(&application)
        .enabled_extension_names(&extensions)
        .push_next(&mut direct);
    let instance = unsafe { icd.entry.create_instance(&info, None) };
    instance.unwrap_or_else(|error| panic!("vkCreateInstance: {}", Report::new(error)))
}

fn create_image(
    device: &ash::Device,
    format: vk::Format,
    modifier: u64,
    size: Size,
) -> Result<vk::Image, vk::Result> {
    let modifiers = [modifier];
    let mut list =
        vk::ImageDrmFormatModifierListCreateInfoEXT::default().drm_format_modifiers(&modifiers);
    let mut external = vk::ExternalMemoryImageCreateInfo::default().handle_types(DMA_BUF);
    let info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .format(format)
        .extent(vk::Extent3D {
            width: size.width,
            height: size.height,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT)
        .usage(USAGE)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .push_next(&mut list)
        .push_next(&mut external);
    unsafe { device.create_image(&info, None) }
}

fn plane_layouts(device: &ash::Device, image: vk::Image, plane_count: u32) -> Layout {
    MEMORY_PLANES[..plane_count as usize]
        .iter()
        .map(|&aspect| {
            let subresource = vk::ImageSubresource {
                aspect_mask: aspect,
                mip_level: 0,
                array_layer: 0,
            };
            let layout = unsafe { device.get_image_subresource_layout(image, subresource) };
            PlaneLayout {
                offset: layout.offset,
                stride: layout.row_pitch,
                size: layout.size,
            }
        })
        .collect()
}

pub fn open_device<D: Driver>(driver: &D, node: u32) -> Option<VulkanDevice<D::Device>> {
    let index = node as usize;
    drm::set_visible(Some(index..index + 1));
    let instance = create_instance(driver.icd());
    let physicals = match unsafe { instance.enumerate_physical_devices() } {
        Ok(physicals) => physicals,
        Err(vk::Result::ERROR_INITIALIZATION_FAILED) => Vec::new(),
        Err(error) => panic!("vkEnumeratePhysicalDevices: {}", Report::new(error)),
    };
    drm::set_visible(None);
    let [physical] = physicals[..] else {
        assert!(
            physicals.is_empty(),
            "node {node} has {} physical devices",
            physicals.len()
        );
        unsafe {
            instance.destroy_instance(None);
        }
        return None;
    };
    assert_eq!(
        physical_node(&instance, physical),
        node,
        "the physical device is not node {node}"
    );
    let maintenance5 = has_extension(&instance, physical, ash::khr::maintenance5::NAME);
    let memory_properties = unsafe { instance.get_physical_device_memory_properties(physical) };
    let device = create_device(&instance, physical, maintenance5);
    Some(VulkanDevice {
        instance,
        physical,
        device,
        maintenance5,
        memory_properties,
        node,
        driver_device: driver.device(node),
    })
}

pub fn describe<D: Driver>(driver: &D, device: &VulkanDevice<D::Device>) -> DeviceDescription {
    let mut units = Vec::new();
    for (format_id, format) in FORMATS.iter().enumerate() {
        if !device.maintenance5 && needs_maintenance5(format.format) {
            continue;
        }
        for properties in modifier_properties(&device.instance, device.physical, format.format) {
            let modifier = properties.drm_format_modifier;
            if driver.skip(modifier, device.driver_device) {
                continue;
            }
            let Some(max_extent) =
                image_supported(&device.instance, device.physical, format.format, modifier)
            else {
                continue;
            };
            let sizes = driver
                .size_set(modifier, format.planes, device.driver_device)
                .into_iter()
                .filter(|&size| extent_valid(format, max_extent, size))
                .collect();
            units.push(DeviceUnit {
                format_id,
                modifier,
                plane_count: properties.drm_format_modifier_plane_count,
                max_extent,
                sizes,
            });
        }
    }
    DeviceDescription {
        node: device.node,
        rank: driver.rank(device.driver_device),
        units,
    }
}

pub fn write_description(description: &DeviceDescription) -> String {
    let mut text = format!("device {} {}\n", description.node, description.rank);
    for unit in &description.units {
        let extent = unit.max_extent;
        text.push_str(&format!(
            "unit {} {:#x} {} {} {} {}",
            unit.format_id,
            unit.modifier,
            unit.plane_count,
            extent.width,
            extent.height,
            extent.depth,
        ));
        for size in &unit.sizes {
            text.push_str(&format!(" {}x{}", size.width, size.height));
        }
        text.push('\n');
    }
    text
}

fn parse_unit<'a>(mut fields: impl Iterator<Item = &'a str>) -> DeviceUnit {
    let mut field = |name: &str| {
        fields
            .next()
            .unwrap_or_else(|| panic!("a unit has no {name}"))
    };
    let number = |name: &str, text: &str| -> u32 {
        text.parse()
            .unwrap_or_else(|_| panic!("invalid unit {name} {text:?}"))
    };
    let format_id = number("format", field("format")) as usize;
    let modifier = field("modifier");
    let modifier = modifier
        .strip_prefix("0x")
        .and_then(|digits| u64::from_str_radix(digits, 16).ok())
        .unwrap_or_else(|| panic!("invalid unit modifier {modifier:?}"));
    let plane_count = number("plane count", field("plane count"));
    let max_extent = vk::Extent3D {
        width: number("width", field("width")),
        height: number("height", field("height")),
        depth: number("depth", field("depth")),
    };
    let sizes = fields
        .map(|size| {
            let (width, height) = size
                .split_once('x')
                .unwrap_or_else(|| panic!("invalid unit size {size:?}"));
            Size {
                width: number("size", width),
                height: number("size", height),
            }
        })
        .collect();
    DeviceUnit {
        format_id,
        modifier,
        plane_count,
        max_extent,
        sizes,
    }
}

pub fn parse_descriptions(text: &str) -> Vec<DeviceDescription> {
    let mut descriptions: Vec<DeviceDescription> = Vec::new();
    for line in text.lines() {
        let mut fields = line.split(' ');
        match fields.next() {
            Some("device") => {
                let mut number = |name: &str| {
                    fields
                        .next()
                        .and_then(|field| field.parse::<u64>().ok())
                        .unwrap_or_else(|| panic!("invalid device {name} in {line:?}"))
                };
                descriptions.push(DeviceDescription {
                    node: number("node") as u32,
                    rank: number("rank"),
                    units: Vec::new(),
                });
            }
            Some("unit") => descriptions
                .last_mut()
                .expect("unit before the first device")
                .units
                .push(parse_unit(fields)),
            _ => panic!("unknown description line {line:?}"),
        }
    }
    descriptions
}

struct Ranked {
    rank: u64,
    support: Support,
}

#[derive(Default)]
struct Merging {
    sizes: BTreeSet<Size>,
    supports: Vec<Ranked>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct UnitKey {
    format_id: usize,
    modifier: u64,
}

pub fn merge(mut descriptions: Vec<DeviceDescription>) -> Described {
    descriptions.sort_by_key(|description| description.node);
    let mut units: BTreeMap<UnitKey, Merging> = BTreeMap::new();
    for description in &descriptions {
        for unit in &description.units {
            let key = UnitKey {
                format_id: unit.format_id,
                modifier: unit.modifier,
            };
            let merging = units.entry(key).or_default();
            merging.sizes.extend(&unit.sizes);
            merging.supports.push(Ranked {
                rank: description.rank,
                support: Support {
                    node: description.node,
                    plane_count: unit.plane_count,
                    max_extent: unit.max_extent,
                },
            });
        }
    }
    let pairs = units
        .into_iter()
        .map(|(key, merging)| DescribedPair {
            format_id: key.format_id,
            modifier: key.modifier,
            sizes: merging.sizes,
            nodes: merging
                .supports
                .iter()
                .map(|ranked| ranked.support.node)
                .collect(),
            latest: merging
                .supports
                .into_iter()
                .max_by_key(|ranked| ranked.rank)
                .expect("a merged unit has a device")
                .support,
        })
        .collect();
    Described { pairs }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ClassKey {
    pub modifier: u64,
    pub format: vk::Format,
    pub node: u32,
}

impl Described {
    pub fn pair_infos(&self) -> Vec<PairInfo<'_>> {
        self.pairs
            .iter()
            .enumerate()
            .map(|(index, pair)| PairInfo {
                index,
                fourcc: fourcc_code(pair.format().fourcc),
                modifier: pair.modifier,
                scanout: None,
                sizes: &pair.sizes,
            })
            .collect()
    }

    pub fn class_key(&self, pair: usize) -> ClassKey {
        let pair = &self.pairs[pair];
        ClassKey {
            modifier: pair.modifier,
            format: pair.format().format,
            node: pair.latest.node,
        }
    }
}

impl generate::Backend for VulkanPlan<'_> {
    type ClassKey = ClassKey;

    fn pairs(&self) -> Vec<PairInfo<'_>> {
        self.described.pair_infos()
    }

    fn class_key(&self, pair: usize) -> Self::ClassKey {
        self.described.class_key(pair)
    }
}

pub fn evaluate_pair<T>(device: &VulkanDevice<T>, pair: &DescribedPair, size: Size) -> Evaluation {
    let format = pair.format();
    if !extent_valid(format, pair.latest.max_extent, size) {
        return Evaluation::Rejected;
    }
    let vk = &device.device;
    let Ok(image) = create_image(vk, format.format, pair.modifier, size) else {
        return Evaluation::Rejected;
    };
    let layout = plane_layouts(vk, image, pair.latest.plane_count);
    unsafe {
        vk.destroy_image(image, None);
    }
    Evaluation::Vector(layout)
}

#[derive(Debug)]
pub enum ExportError {
    Extent,
    CreateImage(vk::Result),
    NoMemoryType,
    AllocateMemory(vk::Result),
    BindImageMemory(vk::Result),
    GetMemoryFd(vk::Result),
}

impl Display for ExportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExportError::Extent => write!(f, "the extent is not valid for the format"),
            ExportError::CreateImage(error) => write!(f, "vkCreateImage: {}", Report::new(error)),
            ExportError::NoMemoryType => write!(f, "no device-local memory type"),
            ExportError::AllocateMemory(error) => {
                write!(f, "vkAllocateMemory: {}", Report::new(error))
            }
            ExportError::BindImageMemory(error) => {
                write!(f, "vkBindImageMemory: {}", Report::new(error))
            }
            ExportError::GetMemoryFd(error) => {
                write!(f, "vkGetMemoryFdKHR: {}", Report::new(error))
            }
        }
    }
}

fn bind_and_export<T>(
    device: &VulkanDevice<T>,
    image: vk::Image,
    memory: vk::DeviceMemory,
) -> Result<(), ExportError> {
    let vk = &device.device;
    unsafe { vk.bind_image_memory(image, memory, 0) }.map_err(ExportError::BindImageMemory)?;
    let info = vk::MemoryGetFdInfoKHR::default()
        .memory(memory)
        .handle_type(DMA_BUF);
    let external = ash::khr::external_memory_fd::Device::new(&device.instance, vk);
    let fd = unsafe { external.get_memory_fd(&info) }.map_err(ExportError::GetMemoryFd)?;
    drop(unsafe { OwnedFd::from_raw_fd(fd) });
    Ok(())
}

fn export<T>(device: &VulkanDevice<T>, image: vk::Image) -> Result<(), ExportError> {
    let vk = &device.device;
    let requirements = unsafe { vk.get_image_memory_requirements(image) };
    let memory_type = device
        .memory_properties
        .memory_types_as_slice()
        .iter()
        .enumerate()
        .position(|(index, memory_type)| {
            requirements.memory_type_bits & (1 << index) != 0
                && memory_type
                    .property_flags
                    .contains(vk::MemoryPropertyFlags::DEVICE_LOCAL)
                && !memory_type
                    .property_flags
                    .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        })
        .ok_or(ExportError::NoMemoryType)?;
    let mut dedicated = vk::MemoryDedicatedAllocateInfo::default().image(image);
    let mut export = vk::ExportMemoryAllocateInfo::default().handle_types(DMA_BUF);
    let info = vk::MemoryAllocateInfo::default()
        .allocation_size(requirements.size)
        .memory_type_index(memory_type as u32)
        .push_next(&mut dedicated)
        .push_next(&mut export);
    let memory = unsafe { vk.allocate_memory(&info, None) }.map_err(ExportError::AllocateMemory)?;
    let exported = bind_and_export(device, image, memory);
    unsafe {
        vk.free_memory(memory, None);
    }
    exported
}

fn create_and_export<T>(
    device: &VulkanDevice<T>,
    pair: &DescribedPair,
    size: Size,
) -> Result<Layout, ExportError> {
    let format = pair.format();
    if !extent_valid(format, pair.latest.max_extent, size) {
        return Err(ExportError::Extent);
    }
    let vk = &device.device;
    let image =
        create_image(vk, format.format, pair.modifier, size).map_err(ExportError::CreateImage)?;
    let layout = plane_layouts(vk, image, pair.latest.plane_count);
    let exported = export(device, image);
    unsafe {
        vk.destroy_image(image, None);
    }
    exported.map(|()| layout)
}

pub fn record_pair<T>(
    device: &VulkanDevice<T>,
    pair: &DescribedPair,
    size: Size,
) -> Result<vectors::Case, ExportError> {
    drm::start_recording();
    let result = create_and_export(device, pair, size);
    let traced = drm::stop_recording();
    let layout = result?;
    Ok(vectors::Case {
        fourcc: fourcc_code(pair.format().fourcc),
        modifier: pair.modifier,
        rejected: None,
        width: size.width,
        height: size.height,
        scanout: false,
        flags: vulkan_flags(),
        layout: layout
            .planes()
            .iter()
            .map(|plane| vectors::Plane {
                offset: plane.offset,
                stride: plane.stride,
            })
            .collect(),
        calls: exported_calls(traced),
    })
}
