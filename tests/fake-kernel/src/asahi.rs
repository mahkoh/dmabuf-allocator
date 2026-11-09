use crate::Buffers;
use crate::DriverFeature;
use crate::DriverVersion;
use crate::Drm;
use crate::Export;
use crate::Kernel;
use crate::KernelVersion;
use crate::Master;
use crate::NewHandle;
use crate::Platform;
use crate::Prime;
use crate::Syncobjs;
use crate::UserMemory;
use crate::handles::lowest_free;
use crate::id_fields;
use crate::ioctls;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::time::NSEC_PER_SEC;
use crate::time::ktime_get_ns;
use crate::uapi::asahi::DRM_ASAHI_BIND_OBJECT_OP_BIND;
use crate::uapi::asahi::DRM_ASAHI_BIND_OBJECT_OP_UNBIND;
use crate::uapi::asahi::DRM_ASAHI_BIND_OBJECT_USAGE_TIMESTAMPS;
use crate::uapi::asahi::DRM_ASAHI_BIND_READ;
use crate::uapi::asahi::DRM_ASAHI_BIND_SINGLE_PAGE;
use crate::uapi::asahi::DRM_ASAHI_BIND_UNBIND;
use crate::uapi::asahi::DRM_ASAHI_BIND_WRITE;
use crate::uapi::asahi::DRM_ASAHI_GEM_VM_PRIVATE;
use crate::uapi::asahi::DRM_ASAHI_GEM_WRITEBACK;
use crate::uapi::asahi::DRM_ASAHI_MAX_CLUSTERS;
use crate::uapi::asahi::DRM_ASAHI_PRIORITY_HIGH;
use crate::uapi::asahi::DRM_ASAHI_PRIORITY_REALTIME;
use crate::uapi::asahi::drm_asahi_gem_bind_object;
use crate::uapi::asahi::drm_asahi_gem_bind_op;
use crate::uapi::asahi::drm_asahi_gem_create;
use crate::uapi::asahi::drm_asahi_gem_mmap_offset;
use crate::uapi::asahi::drm_asahi_get_params;
use crate::uapi::asahi::drm_asahi_get_time;
use crate::uapi::asahi::drm_asahi_params_global;
use crate::uapi::asahi::drm_asahi_queue_create;
use crate::uapi::asahi::drm_asahi_queue_destroy;
use crate::uapi::asahi::drm_asahi_vm_bind;
use crate::uapi::asahi::drm_asahi_vm_create;
use crate::uapi::asahi::drm_asahi_vm_destroy;
use crate::user::from_bytes;
use libc::EFAULT;
use libc::EINVAL;
use libc::ENOENT;
use libc::ENOMEM;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ops::Range;

const UAT_PGSZ: u64 = 1 << 14;
const UAT_PGMSK: u64 = UAT_PGSZ - 1;
const BUFFER_PAGE_SIZE: u64 = 1 << 15;
const IOVA_USER_BASE: u64 = UAT_PGSZ;
const IOVA_USER_TOP: u64 = 1 << 39;
const IOVA_UNK_PAGE: u64 = IOVA_USER_TOP - 2 * UAT_PGSZ;
const VM_USER_RANGE: Range<u64> = IOVA_USER_BASE..IOVA_UNK_PAGE;
const VM_KERNEL_MIN_SIZE: u64 = 0x20000000;
const MAX_COMMANDS_PER_SUBMISSION: u32 = 64;
const MAX_ATTACHMENTS: u32 = 16;
const MODELLED_KERNEL: KernelVersion = KernelVersion::V7_2;

pub struct Chip {
    pub name: &'static str,
    chip_id: u32,
    gpu_generation: u32,
    gpu_variant: u8,
    gpu_revision: u32,
    num_dies: u32,
    num_cores: u32,
    core_masks: &'static [u32],
    max_frequency_khz: u32,
    platform: Platform,
}

pub static CHIPS: &[Chip] = &[
    Chip {
        name: "t8103",
        chip_id: 0x8103,
        gpu_generation: 13,
        gpu_variant: b'G',
        gpu_revision: 0,
        num_dies: 1,
        num_cores: 8,
        core_masks: &[0xff],
        max_frequency_khz: 1278000,
        platform: Platform {
            fullname: "/soc/gpu@206400000",
            compatible: &["apple,agx-t8103", "apple,agx-g13g"],
        },
    },
    Chip {
        name: "t6000",
        chip_id: 0x6000,
        gpu_generation: 13,
        gpu_variant: b'S',
        gpu_revision: 0,
        num_dies: 1,
        num_cores: 8,
        core_masks: &[0xff, 0xff],
        max_frequency_khz: 1296000,
        platform: Platform {
            fullname: "/soc/gpu@406400000",
            compatible: &["apple,agx-t6000", "apple,agx-g13x"],
        },
    },
    Chip {
        name: "t6001",
        chip_id: 0x6001,
        gpu_generation: 13,
        gpu_variant: b'C',
        gpu_revision: 0,
        num_dies: 1,
        num_cores: 8,
        core_masks: &[0xff, 0xff, 0xff, 0xff],
        max_frequency_khz: 1296000,
        platform: Platform {
            fullname: "/soc/gpu@406400000",
            compatible: &["apple,agx-t6001", "apple,agx-g13c", "apple,agx-g13s"],
        },
    },
    Chip {
        name: "t6002",
        chip_id: 0x6002,
        gpu_generation: 13,
        gpu_variant: b'D',
        gpu_revision: 0,
        num_dies: 2,
        num_cores: 8,
        core_masks: &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        max_frequency_khz: 1296000,
        platform: Platform {
            fullname: "/soc@200000000/gpu@406400000",
            compatible: &["apple,agx-t6002", "apple,agx-g13d", "apple,agx-g13s"],
        },
    },
    Chip {
        name: "t8112",
        chip_id: 0x8112,
        gpu_generation: 14,
        gpu_variant: b'G',
        gpu_revision: 0,
        num_dies: 1,
        num_cores: 10,
        core_masks: &[0x3ff],
        max_frequency_khz: 1398000,
        platform: Platform {
            fullname: "/soc/gpu@206400000",
            compatible: &["apple,agx-t8112", "apple,agx-g14g"],
        },
    },
    Chip {
        name: "t6020",
        chip_id: 0x6020,
        gpu_generation: 14,
        gpu_variant: b'S',
        gpu_revision: 0,
        num_dies: 1,
        num_cores: 10,
        core_masks: &[0x3ff, 0x3ff],
        max_frequency_khz: 1398000,
        platform: Platform {
            fullname: "/soc/gpu@406400000",
            compatible: &["apple,agx-t6020", "apple,agx-g14x", "apple,agx-g14s"],
        },
    },
    Chip {
        name: "t6021",
        chip_id: 0x6021,
        gpu_generation: 14,
        gpu_variant: b'C',
        gpu_revision: 0,
        num_dies: 1,
        num_cores: 10,
        core_masks: &[0x3ff, 0x3ff, 0x3ff, 0x3ff],
        max_frequency_khz: 1398000,
        platform: Platform {
            fullname: "/soc/gpu@406400000",
            compatible: &[
                "apple,agx-t6021",
                "apple,agx-g14x",
                "apple,agx-g14c",
                "apple,agx-g14s",
            ],
        },
    },
    Chip {
        name: "t6022",
        chip_id: 0x6022,
        gpu_generation: 14,
        gpu_variant: b'D',
        gpu_revision: 0,
        num_dies: 2,
        num_cores: 10,
        core_masks: &[0x3ff, 0x3ff, 0x3ff, 0x3ff, 0x3ff, 0x3ff, 0x3ff, 0x3ff],
        max_frequency_khz: 1398000,
        platform: Platform {
            fullname: "/soc@200000000/gpu@406400000",
            compatible: &[
                "apple,agx-t6022",
                "apple,agx-g14x",
                "apple,agx-g14d",
                "apple,agx-g14s",
            ],
        },
    },
];

struct Buffer {
    exportable: bool,
}

struct Vm {
    kernel_range: Range<u64>,
}

#[derive(Default)]
struct FileState {
    vms: BTreeMap<u32, Vm>,
    objects: BTreeSet<u32>,
    queues: BTreeSet<u32>,
}

fn next_id(used: impl Iterator<Item = u32>) -> u32 {
    lowest_free(1, used).expect("asahi ids exhausted")
}

fn contains(outer: &Range<u64>, inner: &Range<u64>) -> bool {
    outer.start <= inner.start && inner.end <= outer.end
}

fn overlaps(a: &Range<u64>, b: &Range<u64>) -> bool {
    a.start < b.end && b.start < a.end
}

pub struct Device {
    chip: &'static Chip,
    kernel: KernelVersion,
    buffers: Buffers<Buffer>,
    files: Mutex<BTreeMap<u64, FileState>>,
    syncobjs: Syncobjs,
    master: Master,
}

impl Device {
    pub fn new(name: &str, kernel: KernelVersion) -> Self {
        let chip = CHIPS
            .iter()
            .find(|chip| chip.name == name)
            .unwrap_or_else(|| panic!("unknown asahi chip {name:?}"));
        if kernel != MODELLED_KERNEL {
            unimplemented!("the asahi driver of kernel {kernel}");
        }
        Self {
            chip,
            kernel,
            buffers: Buffers::default(),
            files: Mutex::new(BTreeMap::new()),
            syncobjs: Syncobjs::default(),
            master: Master::default(),
        }
    }

    pub fn from_id(fields: &str) -> Self {
        let values = id_fields(fields, &["chip", "kernel"]);
        Self::new(
            values.get("chip"),
            KernelVersion::parse(values.get("kernel")),
        )
    }

    fn params(&self) -> drm_asahi_params_global {
        let chip = self.chip;
        let mut core_masks = [0; DRM_ASAHI_MAX_CLUSTERS];
        for (mask, &cores) in core_masks.iter_mut().zip(chip.core_masks) {
            *mask = cores as u64;
        }
        drm_asahi_params_global {
            features: 0,
            gpu_generation: chip.gpu_generation,
            gpu_variant: chip.gpu_variant as u32,
            gpu_revision: chip.gpu_revision,
            chip_id: chip.chip_id,
            num_dies: chip.num_dies,
            num_clusters_total: chip.core_masks.len() as u32,
            num_cores_per_cluster: chip.num_cores,
            max_frequency_khz: chip.max_frequency_khz,
            core_masks,
            vm_start: VM_USER_RANGE.start,
            vm_end: VM_USER_RANGE.end,
            vm_kernel_min_size: VM_KERNEL_MIN_SIZE,
            max_commands_per_submission: MAX_COMMANDS_PER_SUBMISSION,
            max_attachments: MAX_ATTACHMENTS,
            command_timestamp_frequency_hz: NSEC_PER_SEC,
        }
    }

    unsafe fn get_params(&self, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let data = unsafe { &*arg.cast::<drm_asahi_get_params>() };
        if data.param_group != 0 || data.pad != 0 {
            return -(EINVAL as i64);
        }
        let params = self.params();
        let bytes = bytemuck::bytes_of(&params);
        let size = bytes.len().min(data.size as usize);
        match user.write(data.pointer, &bytes[..size]) {
            Ok(()) => 0,
            Err(errno) => -(errno as i64),
        }
    }

    unsafe fn get_time(&self, arg: *mut u8) -> i64 {
        let data = unsafe { &mut *arg.cast::<drm_asahi_get_time>() };
        if data.flags != 0 {
            return -(EINVAL as i64);
        }
        data.gpu_timestamp = ktime_get_ns();
        0
    }

    unsafe fn vm_create(&self, file: u64, arg: *mut u8) -> i64 {
        let data = unsafe { &mut *arg.cast::<drm_asahi_vm_create>() };
        let kernel_range = data.kernel_start..data.kernel_end;
        if !contains(&VM_USER_RANGE, &kernel_range)
            || kernel_range.end.saturating_sub(kernel_range.start) < VM_KERNEL_MIN_SIZE
            || kernel_range.start & UAT_PGMSK != 0
            || kernel_range.end & UAT_PGMSK != 0
        {
            return -(EINVAL as i64);
        }
        let kernel_range = kernel_range.start.next_multiple_of(BUFFER_PAGE_SIZE)
            ..kernel_range.end / BUFFER_PAGE_SIZE * BUFFER_PAGE_SIZE;
        let mut files = self.files.lock();
        let state = files.entry(file).or_default();
        let id = next_id(state.vms.keys().copied());
        state.vms.insert(id, Vm { kernel_range });
        data.vm_id = id;
        0
    }

    unsafe fn vm_destroy(&self, file: u64, arg: *mut u8) -> i64 {
        let data = unsafe { &*arg.cast::<drm_asahi_vm_destroy>() };
        let mut files = self.files.lock();
        let removed = files
            .get_mut(&file)
            .and_then(|state| state.vms.remove(&data.vm_id));
        match removed {
            Some(_) => 0,
            None => -(ENOENT as i64),
        }
    }

    fn vm_kernel_range(&self, file: u64, vm_id: u32) -> Option<Range<u64>> {
        let files = self.files.lock();
        let vm = files.get(&file)?.vms.get(&vm_id)?;
        Some(vm.kernel_range.clone())
    }

    fn bind(&self, file: u64, vm_id: u32, op: &drm_asahi_gem_bind_op) -> i64 {
        if (op.addr | op.range | op.offset) & UAT_PGMSK != 0 {
            return -(EINVAL as i64);
        }
        if op.flags & !(DRM_ASAHI_BIND_READ | DRM_ASAHI_BIND_WRITE | DRM_ASAHI_BIND_SINGLE_PAGE)
            != 0
        {
            return -(EINVAL as i64);
        }
        let single_page = op.flags & DRM_ASAHI_BIND_SINGLE_PAGE != 0;
        let Some(size) = self.buffers.file(file).size(op.handle) else {
            return -(ENOENT as i64);
        };
        let Some(end) = op.addr.checked_add(op.range) else {
            return -(EINVAL as i64);
        };
        let range = op.addr..end;
        let accessed = match single_page {
            true => UAT_PGMSK,
            false => op.range,
        };
        match op.offset.checked_add(accessed) {
            Some(end_offset) if end_offset <= size => {}
            _ => return -(EINVAL as i64),
        }
        if !contains(&VM_USER_RANGE, &range) {
            return -(EINVAL as i64);
        }
        if op.flags & (DRM_ASAHI_BIND_READ | DRM_ASAHI_BIND_WRITE) == 0 {
            return -(EINVAL as i64);
        }
        let Some(kernel_range) = self.vm_kernel_range(file, vm_id) else {
            return -(ENOENT as i64);
        };
        if overlaps(&kernel_range, &range) {
            return -(EINVAL as i64);
        }
        0
    }

    fn unbind(&self, file: u64, vm_id: u32, op: &drm_asahi_gem_bind_op) -> i64 {
        if op.offset != 0 || op.flags != DRM_ASAHI_BIND_UNBIND || op.handle != 0 {
            return -(EINVAL as i64);
        }
        if (op.addr | op.range) & UAT_PGMSK != 0 {
            return -(EINVAL as i64);
        }
        let Some(end) = op.addr.checked_add(op.range) else {
            return -(EINVAL as i64);
        };
        let range = op.addr..end;
        if !contains(&VM_USER_RANGE, &range) {
            return -(EINVAL as i64);
        }
        let Some(kernel_range) = self.vm_kernel_range(file, vm_id) else {
            return -(ENOENT as i64);
        };
        if overlaps(&kernel_range, &range) {
            return -(EINVAL as i64);
        }
        0
    }

    unsafe fn vm_bind(&self, file: u64, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let data = unsafe { &*arg.cast::<drm_asahi_vm_bind>() };
        if data.stride == 0 || data.pad != 0 {
            return -(EINVAL as i64);
        }
        let Some(size) = data.stride.checked_mul(data.num_binds) else {
            unimplemented!("asahi vm_bind arrays of 4 GiB or more");
        };
        let mut ops = Vec::new();
        if ops.try_reserve_exact(size as usize).is_err() {
            return -(ENOMEM as i64);
        }
        ops.resize(size as usize, 0u8);
        if user.read(data.userptr, &mut ops).is_err() {
            return -(EFAULT as i64);
        }
        let len = (data.stride as usize).min(size_of::<drm_asahi_gem_bind_op>());
        for index in 0..data.num_binds as usize {
            let op: drm_asahi_gem_bind_op = from_bytes(&ops[index * len..(index + 1) * len]);
            let ret = match op.flags & DRM_ASAHI_BIND_UNBIND {
                0 => self.bind(file, data.vm_id, &op),
                _ => self.unbind(file, data.vm_id, &op),
            };
            if ret != 0 {
                return ret;
            }
        }
        0
    }

    unsafe fn gem_create(&self, file: u64, arg: *mut u8) -> i64 {
        let data = unsafe { &mut *arg.cast::<drm_asahi_gem_create>() };
        let private = data.flags & DRM_ASAHI_GEM_VM_PRIVATE != 0;
        if data.flags & !(DRM_ASAHI_GEM_WRITEBACK | DRM_ASAHI_GEM_VM_PRIVATE) != 0
            || !private && data.vm_id != 0
        {
            return -(EINVAL as i64);
        }
        if private && self.vm_kernel_range(file, data.vm_id).is_none() {
            return -(ENOENT as i64);
        }
        if data.size == 0 {
            unimplemented!("asahi GEM objects of size 0");
        }
        let size = data.size.next_multiple_of(UAT_PGSZ);
        let buffer = Buffer {
            exportable: !private,
        };
        let buffers = self.buffers.file(file);
        data.handle = buffers.create_with_offset(buffer, size).handle;
        0
    }

    unsafe fn gem_mmap_offset(&self, file: u64, arg: *mut u8) -> i64 {
        let data = unsafe { &mut *arg.cast::<drm_asahi_gem_mmap_offset>() };
        if data.flags != 0 {
            return -(EINVAL as i64);
        }
        match self.buffers.file(file).offset(data.handle) {
            Some(offset) => {
                data.offset = offset;
                0
            }
            None => -(ENOENT as i64),
        }
    }

    unsafe fn gem_bind_object(&self, file: u64, arg: *mut u8) -> i64 {
        let data = unsafe { &mut *arg.cast::<drm_asahi_gem_bind_object>() };
        if data.pad != 0 || data.vm_id != 0 {
            return -(EINVAL as i64);
        }
        match data.op {
            DRM_ASAHI_BIND_OBJECT_OP_BIND => {
                if (data.range | data.offset) & UAT_PGMSK != 0
                    || data.flags != DRM_ASAHI_BIND_OBJECT_USAGE_TIMESTAMPS
                {
                    return -(EINVAL as i64);
                }
                let Some(end) = data.offset.checked_add(data.range) else {
                    return -(EINVAL as i64);
                };
                let Some(size) = self.buffers.file(file).size(data.handle) else {
                    return -(ENOENT as i64);
                };
                if end > size {
                    return -(EINVAL as i64);
                }
                let mut files = self.files.lock();
                let state = files.entry(file).or_default();
                let id = next_id(state.objects.iter().copied());
                state.objects.insert(id);
                data.object_handle = id;
                0
            }
            DRM_ASAHI_BIND_OBJECT_OP_UNBIND => {
                if data.range != 0 || data.offset != 0 || data.flags != 0 || data.handle != 0 {
                    return -(EINVAL as i64);
                }
                let mut files = self.files.lock();
                let removed = files
                    .get_mut(&file)
                    .is_some_and(|state| state.objects.remove(&data.object_handle));
                match removed {
                    true => 0,
                    false => -(ENOENT as i64),
                }
            }
            _ => -(EINVAL as i64),
        }
    }

    unsafe fn queue_create(&self, file: u64, arg: *mut u8) -> i64 {
        let data = unsafe { &mut *arg.cast::<drm_asahi_queue_create>() };
        if data.flags != 0 || data.priority > DRM_ASAHI_PRIORITY_REALTIME {
            return -(EINVAL as i64);
        }
        if data.priority >= DRM_ASAHI_PRIORITY_HIGH {
            return -(EINVAL as i64);
        }
        let mut files = self.files.lock();
        let Some(state) = files
            .get_mut(&file)
            .filter(|state| state.vms.contains_key(&data.vm_id))
        else {
            return -(ENOENT as i64);
        };
        let id = next_id(state.queues.iter().copied());
        state.queues.insert(id);
        data.queue_id = id;
        0
    }

    unsafe fn queue_destroy(&self, file: u64, arg: *mut u8) -> i64 {
        let data = unsafe { &*arg.cast::<drm_asahi_queue_destroy>() };
        let mut files = self.files.lock();
        let removed = files
            .get_mut(&file)
            .is_some_and(|state| state.queues.remove(&data.queue_id));
        match removed {
            true => 0,
            false => -(ENOENT as i64),
        }
    }
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "asahi"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        format!("asahi:chip={}:kernel={}", self.chip.name, self.kernel)
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn desc(&self) -> &'static str {
        "Apple AGX Graphics"
    }

    fn driver_version(&self) -> DriverVersion {
        DriverVersion::ASAHI
    }

    fn uevent_driver(&self) -> &'static str {
        "asahi"
    }

    fn platform(&self) -> Option<&'static Platform> {
        Some(&self.chip.platform)
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        ioctls::ASAHI
    }

    fn syncobjs(&self) -> Option<&Syncobjs> {
        Some(&self.syncobjs)
    }

    fn master(&self) -> &Master {
        &self.master
    }

    fn check_feature(&self, feature: DriverFeature) -> bool {
        match feature {
            DriverFeature::Modeset => false,
            DriverFeature::SyncobjTimeline => true,
        }
    }

    unsafe fn ioctl(
        &self,
        file: u64,
        func: Func,
        arg: *mut u8,
        user: &dyn UserMemory,
        _prime: &mut dyn Prime,
    ) -> Option<i64> {
        let ret = unsafe {
            match func {
                Func::AsahiFileFileGetParams => self.get_params(arg, user),
                Func::AsahiFileFileGetTime => self.get_time(arg),
                Func::AsahiFileFileVmCreate => self.vm_create(file, arg),
                Func::AsahiFileFileVmDestroy => self.vm_destroy(file, arg),
                Func::AsahiFileFileVmBind => self.vm_bind(file, arg, user),
                Func::AsahiFileFileGemCreate => self.gem_create(file, arg),
                Func::AsahiFileFileGemMmapOffset => self.gem_mmap_offset(file, arg),
                Func::AsahiFileFileGemBindObject => self.gem_bind_object(file, arg),
                Func::AsahiFileFileQueueCreate => self.queue_create(file, arg),
                Func::AsahiFileFileQueueDestroy => self.queue_destroy(file, arg),
                _ => return None,
            }
        };
        Some(ret)
    }

    fn export(&self, file: u64, handle: u32) -> Result<Export, i32> {
        self.buffers
            .file(file)
            .export(handle, |buffer| match buffer.exportable {
                true => Ok(()),
                false => Err(EINVAL),
            })
    }

    fn import(&self, file: u64, object: u32) -> Option<u32> {
        self.buffers.file(file).import(object)
    }

    fn import_foreign(&self, _file: u64, _export: &Export) -> NewHandle {
        unimplemented!("importing foreign dma-bufs into asahi");
    }

    fn close_buffer(&self, file: u64, handle: u32) -> bool {
        self.buffers.file(file).close(handle)
    }

    fn object(&self, file: u64, handle: u32) -> Option<u32> {
        self.buffers.file(file).object(handle)
    }

    fn object_has_handles(&self, object: u32) -> bool {
        self.buffers.has_handles(object)
    }

    fn release_export(&self, object: u32) {
        self.buffers.release_export(object);
    }

    fn release_file(&self, file: u64) {
        self.buffers.release_file(file);
        self.files.lock().remove(&file);
        self.syncobjs.release_file(file);
    }
}
