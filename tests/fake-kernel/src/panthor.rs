use crate::Buffers;
use crate::DriverFeature;
use crate::DriverVersion;
use crate::Drm;
use crate::Export;
use crate::Fence;
use crate::Kernel;
use crate::KernelVersion;
use crate::Master;
use crate::NewHandle;
use crate::PAGE_SIZE;
use crate::Platform;
use crate::Prime;
use crate::SZ_4G;
use crate::Syncobjs;
use crate::UserMemory;
use crate::handles::lowest_free;
use crate::id_fields;
use crate::ioctls;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::string::strndup_user;
use crate::uapi::panthor::DRM_PANTHOR_BO_NO_MMAP;
use crate::uapi::panthor::DRM_PANTHOR_BO_WB_MMAP;
use crate::uapi::panthor::DRM_PANTHOR_DEV_QUERY_CSIF_INFO;
use crate::uapi::panthor::DRM_PANTHOR_DEV_QUERY_GPU_INFO;
use crate::uapi::panthor::DRM_PANTHOR_DEV_QUERY_GROUP_PRIORITIES_INFO;
use crate::uapi::panthor::DRM_PANTHOR_DEV_QUERY_TIMESTAMP_INFO;
use crate::uapi::panthor::DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_MASK;
use crate::uapi::panthor::DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_SYNCOBJ;
use crate::uapi::panthor::DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_TIMELINE_SYNCOBJ;
use crate::uapi::panthor::DRM_PANTHOR_SYNC_OP_SIGNAL;
use crate::uapi::panthor::DRM_PANTHOR_USER_MMIO_OFFSET_32BIT;
use crate::uapi::panthor::DRM_PANTHOR_USER_MMIO_OFFSET_64BIT;
use crate::uapi::panthor::DRM_PANTHOR_VM_BIND_ASYNC;
use crate::uapi::panthor::DRM_PANTHOR_VM_BIND_OP_MAP_NOEXEC;
use crate::uapi::panthor::DRM_PANTHOR_VM_BIND_OP_MAP_READONLY;
use crate::uapi::panthor::DRM_PANTHOR_VM_BIND_OP_MAP_UNCACHED;
use crate::uapi::panthor::DRM_PANTHOR_VM_BIND_OP_TYPE_MAP;
use crate::uapi::panthor::DRM_PANTHOR_VM_BIND_OP_TYPE_MASK;
use crate::uapi::panthor::DRM_PANTHOR_VM_BIND_OP_TYPE_SYNC_ONLY;
use crate::uapi::panthor::DRM_PANTHOR_VM_BIND_OP_TYPE_UNMAP;
use crate::uapi::panthor::PANTHOR_GROUP_PRIORITY_LOW;
use crate::uapi::panthor::PANTHOR_GROUP_PRIORITY_MEDIUM;
use crate::uapi::panthor::drm_panthor_bo_create;
use crate::uapi::panthor::drm_panthor_bo_mmap_offset;
use crate::uapi::panthor::drm_panthor_bo_set_label;
use crate::uapi::panthor::drm_panthor_csif_info;
use crate::uapi::panthor::drm_panthor_dev_query;
use crate::uapi::panthor::drm_panthor_gpu_info;
use crate::uapi::panthor::drm_panthor_group_priorities_info;
use crate::uapi::panthor::drm_panthor_obj_array;
use crate::uapi::panthor::drm_panthor_set_user_mmio_offset;
use crate::uapi::panthor::drm_panthor_sync_op;
use crate::uapi::panthor::drm_panthor_timestamp_info;
use crate::uapi::panthor::drm_panthor_vm_bind;
use crate::uapi::panthor::drm_panthor_vm_bind_op;
use crate::uapi::panthor::drm_panthor_vm_create;
use crate::uapi::panthor::drm_panthor_vm_destroy;
use crate::user::from_bytes;
use bytemuck::AnyBitPattern;
use bytemuck::NoUninit;
use bytemuck::bytes_of;
use libc::E2BIG;
use libc::EBUSY;
use libc::EINVAL;
use libc::ENOENT;
use libc::EPERM;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::mem::offset_of;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;

const PANTHOR_VM_MIN_KERNEL_VA_SIZE: u64 = 256 << 20;
const PANTHOR_MAX_VMS_PER_FILE: u32 = 32;
const TASK_SIZE: u64 = 1 << 48;
const PANTHOR_BO_LABEL_MAXLEN: u64 = 4096;

struct Hardware {
    gpu_rev: u32,
    csf_id: u32,
    l2_features: u32,
    tiler_features: u32,
    mem_features: u32,
    mmu_features: u32,
    thread_features: u32,
    max_threads: u32,
    thread_max_workgroup_size: u32,
    thread_max_barrier_size: u32,
    coherency_features: u32,
    texture_features: [u32; 4],
    as_present: u32,
    shader_present: u64,
    l2_present: u64,
    tiler_present: u64,
    csg_slot_count: u32,
    cs_slot_count: u32,
    unpreserved_cs_reg_count: u32,
    timestamp_frequency: u64,
    current_timestamp: u64,
    timestamp_offset: u64,
}

const MESA_DRM_SHIM: Hardware = Hardware {
    gpu_rev: 0,
    csf_id: 0x40a0412,
    l2_features: 0x7120306,
    tiler_features: 0x809,
    mem_features: 0x301,
    mmu_features: 0x2830,
    thread_features: 0x4010000,
    max_threads: 2048,
    thread_max_workgroup_size: 1024,
    thread_max_barrier_size: 1024,
    coherency_features: 0,
    texture_features: [0xc1ffff9e, 0, 0, 0],
    as_present: 0xff,
    shader_present: 0x50005,
    l2_present: 1,
    tiler_present: 1,
    csg_slot_count: 8,
    cs_slot_count: 8,
    unpreserved_cs_reg_count: 4,
    timestamp_frequency: 0,
    current_timestamp: 0,
    timestamp_offset: 0,
};

pub struct Gpu {
    pub name: &'static str,
    gpu_id: u32,
    core_features: u32,
    hardware: Hardware,
    platform: Option<Platform>,
}

pub static GPUS: &[Gpu] = &[
    Gpu {
        name: "g610",
        gpu_id: 0xa807 << 16,
        core_features: 0,
        hardware: MESA_DRM_SHIM,
        platform: Some(Platform {
            fullname: "/gpu@fb000000",
            compatible: &["rockchip,rk3588-mali", "arm,mali-valhall-csf"],
        }),
    },
    Gpu {
        name: "g310",
        gpu_id: 0xac04 << 16,
        core_features: 0,
        hardware: MESA_DRM_SHIM,
        platform: Some(Platform {
            fullname: "/soc/gpu@4d900000",
            compatible: &["nxp,imx95-mali", "arm,mali-valhall-csf"],
        }),
    },
];

static TEST_GPUS: &[Gpu] = &[Gpu {
    name: "g720",
    gpu_id: 0xc800 << 16,
    core_features: 4,
    hardware: MESA_DRM_SHIM,
    platform: None,
}];

#[derive(Copy, Clone)]
struct Buffer {
    flags: u32,
    exclusive_vm_root: Option<u64>,
}

#[derive(Copy, Clone)]
struct Vm {
    root: u64,
}

struct SyncSignal {
    handle: u32,
    point: u64,
    has_fence: bool,
    first_job: usize,
}

#[derive(Default)]
struct FileState {
    vms: BTreeMap<u32, Vm>,
}

pub struct Device {
    gpu: &'static Gpu,
    kernel: KernelVersion,
    buffers: Buffers<Buffer>,
    files: Mutex<BTreeMap<u64, FileState>>,
    next_vm_root: AtomicU64,
    syncobjs: Syncobjs,
    master: Master,
}

impl Device {
    pub fn new(name: &str, kernel: KernelVersion) -> Self {
        let gpu = GPUS
            .iter()
            .chain(TEST_GPUS)
            .find(|gpu| gpu.name == name)
            .unwrap_or_else(|| panic!("unknown panthor GPU {name:?}"));
        Self {
            gpu,
            kernel,
            buffers: Buffers::default(),
            files: Mutex::new(BTreeMap::new()),
            next_vm_root: AtomicU64::new(1),
            syncobjs: Syncobjs::default(),
            master: Master::default(),
        }
    }

    pub fn from_id(fields: &str) -> Self {
        let values = id_fields(fields, &["gpu", "kernel"]);
        Self::new(
            values.get("gpu"),
            KernelVersion::parse(values.get("kernel")),
        )
    }

    fn gpu_info(&self) -> drm_panthor_gpu_info {
        let hardware = &self.gpu.hardware;
        drm_panthor_gpu_info {
            gpu_id: self.gpu.gpu_id,
            gpu_rev: hardware.gpu_rev,
            csf_id: hardware.csf_id,
            l2_features: hardware.l2_features,
            tiler_features: hardware.tiler_features,
            mem_features: hardware.mem_features,
            mmu_features: hardware.mmu_features,
            thread_features: hardware.thread_features,
            max_threads: hardware.max_threads,
            thread_max_workgroup_size: hardware.thread_max_workgroup_size,
            thread_max_barrier_size: hardware.thread_max_barrier_size,
            coherency_features: hardware.coherency_features,
            texture_features: hardware.texture_features,
            as_present: hardware.as_present,
            selected_coherency: 0,
            shader_present: hardware.shader_present,
            l2_present: hardware.l2_present,
            tiler_present: hardware.tiler_present,
            core_features: self.gpu.core_features,
            pad: 0,
            gpu_features: 0,
        }
    }

    fn arch(&self) -> u32 {
        self.gpu.gpu_id >> 28
    }

    fn full_va_range(&self) -> u64 {
        1 << (self.gpu.hardware.mmu_features & 0xff)
    }

    fn vm(&self, file: u64, id: u32) -> Option<Vm> {
        self.files.lock().get(&file)?.vms.get(&id).copied()
    }

    fn set_uobj<T: NoUninit>(
        user: &dyn UserMemory,
        pointer: u64,
        size: u32,
        min_size: usize,
        value: &T,
    ) -> i64 {
        let size = size as usize;
        if size < min_size {
            return -(EINVAL as i64);
        }
        let bytes = bytes_of(value);
        let copied = size.min(bytes.len());
        if let Err(errno) = user.write(pointer, &bytes[..copied]) {
            return -(errno as i64);
        }
        if size > bytes.len() {
            let zeros = vec![0u8; size - bytes.len()];
            if let Err(errno) = user.write(pointer + bytes.len() as u64, &zeros) {
                return -(errno as i64);
            }
        }
        0
    }

    fn copy_struct_from_user<T: AnyBitPattern>(
        user: &dyn UserMemory,
        pointer: u64,
        size: usize,
    ) -> Result<T, i32> {
        let mut bytes = vec![0u8; size_of::<T>()];
        let copied = size.min(bytes.len());
        user.read(pointer, &mut bytes[..copied])?;
        if size > bytes.len() {
            let mut rest = vec![0u8; size - bytes.len()];
            user.read(pointer + bytes.len() as u64, &mut rest)?;
            if rest.iter().any(|&byte| byte != 0) {
                return Err(E2BIG);
            }
        }
        Ok(from_bytes(&bytes))
    }

    fn get_uobj_array<T: AnyBitPattern>(
        user: &dyn UserMemory,
        array: &drm_panthor_obj_array,
        min_stride: usize,
    ) -> Result<Vec<T>, i32> {
        if array.count == 0 {
            return Ok(Vec::new());
        }
        if (array.stride as usize) < min_stride {
            return Err(EINVAL);
        }
        (0..array.count as u64)
            .map(|index| {
                Self::copy_struct_from_user(
                    user,
                    array.array + index * array.stride as u64,
                    array.stride as usize,
                )
            })
            .collect()
    }

    fn prepare_bind_op(&self, file: u64, vm: &Vm, op: &drm_panthor_vm_bind_op) -> Result<(), i32> {
        if !(op.va | op.size | op.bo_offset).is_multiple_of(PAGE_SIZE) {
            return Err(EINVAL);
        }
        let has_bo = op.bo_handle != 0 || op.bo_offset != 0;
        match op.flags & DRM_PANTHOR_VM_BIND_OP_TYPE_MASK {
            DRM_PANTHOR_VM_BIND_OP_TYPE_MAP => {
                let buffers = self.buffers.file(file);
                let bo = buffers.with(op.bo_handle, |bo| *bo).ok_or(EINVAL)?;
                let bo_size = buffers.size(op.bo_handle).ok_or(EINVAL)?;
                let flags = DRM_PANTHOR_VM_BIND_OP_MAP_READONLY
                    | DRM_PANTHOR_VM_BIND_OP_MAP_NOEXEC
                    | DRM_PANTHOR_VM_BIND_OP_MAP_UNCACHED
                    | DRM_PANTHOR_VM_BIND_OP_TYPE_MASK;
                if op.flags & !flags != 0 {
                    return Err(EINVAL);
                }
                if op.size > bo_size || op.bo_offset > bo_size - op.size {
                    return Err(EINVAL);
                }
                if bo.exclusive_vm_root.is_some_and(|root| root != vm.root) {
                    return Err(EINVAL);
                }
                Ok(())
            }
            DRM_PANTHOR_VM_BIND_OP_TYPE_UNMAP => {
                if op.flags & !DRM_PANTHOR_VM_BIND_OP_TYPE_MASK != 0 || has_bo {
                    return Err(EINVAL);
                }
                Ok(())
            }
            DRM_PANTHOR_VM_BIND_OP_TYPE_SYNC_ONLY => {
                if op.flags & !DRM_PANTHOR_VM_BIND_OP_TYPE_MASK != 0 || has_bo {
                    return Err(EINVAL);
                }
                if op.va != 0 || op.size != 0 || op.syncs.count == 0 {
                    return Err(EINVAL);
                }
                Ok(())
            }
            _ => Err(EINVAL),
        }
    }

    fn check_sync_op(op: &drm_panthor_sync_op) -> Result<(), i32> {
        if op.flags & !(DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_MASK | DRM_PANTHOR_SYNC_OP_SIGNAL) != 0 {
            return Err(EINVAL);
        }
        match op.flags & DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_MASK {
            DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_SYNCOBJ if op.timeline_value != 0 => Err(EINVAL),
            DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_SYNCOBJ
            | DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_TIMELINE_SYNCOBJ => Ok(()),
            _ => Err(EINVAL),
        }
    }

    unsafe fn vm_bind(&self, file: u64, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_panthor_vm_bind>() };
        let Some(vm) = self.vm(file, args.vm_id) else {
            return -(EINVAL as i64);
        };
        let ops = match Self::get_uobj_array::<drm_panthor_vm_bind_op>(
            user,
            &args.ops,
            size_of::<drm_panthor_vm_bind_op>(),
        ) {
            Ok(ops) => ops,
            Err(errno) => return -(errno as i64),
        };
        let ret = match args.flags & DRM_PANTHOR_VM_BIND_ASYNC != 0 {
            true => self.vm_bind_async(file, &vm, &ops, user),
            false => self.vm_bind_sync(file, &vm, args, &ops),
        };
        match ret {
            Ok(()) => 0,
            Err(errno) => -(errno as i64),
        }
    }

    fn exec_sync_op(&self, file: u64, vm: &Vm, op: &drm_panthor_vm_bind_op) -> Result<(), i32> {
        if op.syncs.count != 0 {
            return Err(EINVAL);
        }
        if op.size == 0 {
            return Ok(());
        }
        self.prepare_bind_op(file, vm, op)?;
        let end = op.va.checked_add(op.size);
        match end.is_some_and(|end| end <= self.full_va_range()) {
            true => Ok(()),
            false => Err(EINVAL),
        }
    }

    fn vm_bind_sync(
        &self,
        file: u64,
        vm: &Vm,
        args: &mut drm_panthor_vm_bind,
        ops: &[drm_panthor_vm_bind_op],
    ) -> Result<(), i32> {
        for (index, op) in ops.iter().enumerate() {
            if let Err(errno) = self.exec_sync_op(file, vm, op) {
                args.ops.count = index as u32;
                return Err(errno);
            }
        }
        Ok(())
    }

    fn vm_bind_async(
        &self,
        file: u64,
        vm: &Vm,
        ops: &[drm_panthor_vm_bind_op],
        user: &dyn UserMemory,
    ) -> Result<(), i32> {
        let mut jobs = Vec::new();
        for op in ops {
            self.prepare_bind_op(file, vm, op)?;
            let min_stride = offset_of!(drm_panthor_sync_op, timeline_value) + size_of::<u64>();
            jobs.push(Self::get_uobj_array::<drm_panthor_sync_op>(
                user, &op.syncs, min_stride,
            )?);
        }
        let mut signals: Vec<SyncSignal> = Vec::new();
        for (job, syncs) in jobs.iter().enumerate() {
            for sync in syncs {
                if sync.flags & DRM_PANTHOR_SYNC_OP_SIGNAL == 0 {
                    continue;
                }
                Self::check_sync_op(sync)?;
                if signals.iter().any(|signal| {
                    signal.handle == sync.handle && signal.point == sync.timeline_value
                }) {
                    continue;
                }
                if self.syncobjs.object(file, sync.handle).is_none() {
                    return Err(EINVAL);
                }
                signals.push(SyncSignal {
                    handle: sync.handle,
                    point: sync.timeline_value,
                    has_fence: self
                        .syncobjs
                        .find_fence(file, sync.handle, sync.timeline_value, 0)
                        .is_ok(),
                    first_job: job,
                });
            }
        }
        for (job, syncs) in jobs.iter().enumerate() {
            for sync in syncs {
                if sync.flags & DRM_PANTHOR_SYNC_OP_SIGNAL != 0 {
                    continue;
                }
                Self::check_sync_op(sync)?;
                let signal = signals.iter().find(|signal| {
                    signal.handle == sync.handle && signal.point == sync.timeline_value
                });
                match signal {
                    Some(signal) if signal.has_fence || signal.first_job < job => {}
                    Some(_) => return Err(EINVAL),
                    None => {
                        self.syncobjs
                            .find_fence(file, sync.handle, sync.timeline_value, 0)?;
                    }
                }
            }
        }
        for signal in signals {
            let object = self.syncobjs.object(file, signal.handle).unwrap();
            match signal.point {
                0 => self.syncobjs.replace(object, Some(Fence::Plain)),
                point => self.syncobjs.add_point(object, point),
            }
        }
        Ok(())
    }

    unsafe fn dev_query(&self, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_panthor_dev_query>() };
        if args.pointer == 0 {
            let size = match args.type_ {
                DRM_PANTHOR_DEV_QUERY_GPU_INFO => size_of::<drm_panthor_gpu_info>(),
                DRM_PANTHOR_DEV_QUERY_CSIF_INFO => size_of::<drm_panthor_csif_info>(),
                DRM_PANTHOR_DEV_QUERY_TIMESTAMP_INFO => size_of::<drm_panthor_timestamp_info>(),
                DRM_PANTHOR_DEV_QUERY_GROUP_PRIORITIES_INFO => {
                    size_of::<drm_panthor_group_priorities_info>()
                }
                _ => return -(EINVAL as i64),
            };
            args.size = size as u32;
            return 0;
        }
        let hardware = &self.gpu.hardware;
        match args.type_ {
            DRM_PANTHOR_DEV_QUERY_GPU_INFO => Self::set_uobj(
                user,
                args.pointer,
                args.size,
                offset_of!(drm_panthor_gpu_info, tiler_present) + size_of::<u64>(),
                &self.gpu_info(),
            ),
            DRM_PANTHOR_DEV_QUERY_CSIF_INFO => {
                let arch = self.arch();
                let info = drm_panthor_csif_info {
                    csg_slot_count: hardware.csg_slot_count,
                    cs_slot_count: hardware.cs_slot_count,
                    cs_reg_count: match arch >= 12 {
                        true => 128,
                        false => 96,
                    },
                    scoreboard_slot_count: match arch >= 11 {
                        true => 16,
                        false => 8,
                    },
                    unpreserved_cs_reg_count: hardware.unpreserved_cs_reg_count,
                    pad: 0,
                };
                Self::set_uobj(
                    user,
                    args.pointer,
                    args.size,
                    size_of::<drm_panthor_csif_info>(),
                    &info,
                )
            }
            DRM_PANTHOR_DEV_QUERY_TIMESTAMP_INFO => {
                let mut info = match Self::copy_struct_from_user::<drm_panthor_timestamp_info>(
                    user,
                    args.pointer,
                    args.size as usize,
                ) {
                    Ok(info) => info,
                    Err(errno) => return -(errno as i64),
                };
                if info.flags != 0 {
                    unimplemented!("panthor timestamp flags {:#x}", info.flags);
                }
                info.timestamp_frequency = hardware.timestamp_frequency;
                info.current_timestamp = hardware.current_timestamp;
                info.timestamp_offset = hardware.timestamp_offset;
                info.cycle_count = 0;
                info.duration_nsec = 0;
                info.cpu_timestamp_sec = 0;
                info.cpu_timestamp_nsec = 0;
                Self::set_uobj(
                    user,
                    args.pointer,
                    args.size,
                    offset_of!(drm_panthor_timestamp_info, current_timestamp) + size_of::<u64>(),
                    &info,
                )
            }
            DRM_PANTHOR_DEV_QUERY_GROUP_PRIORITIES_INFO => {
                let info = drm_panthor_group_priorities_info {
                    allowed_mask: 1 << PANTHOR_GROUP_PRIORITY_LOW
                        | 1 << PANTHOR_GROUP_PRIORITY_MEDIUM,
                    pad: [0; 3],
                };
                Self::set_uobj(
                    user,
                    args.pointer,
                    args.size,
                    size_of::<drm_panthor_group_priorities_info>(),
                    &info,
                )
            }
            _ => -(EINVAL as i64),
        }
    }

    fn user_va_range(&self, requested: u64) -> u64 {
        let full = self.full_va_range();
        let range = if full < PANTHOR_VM_MIN_KERNEL_VA_SIZE {
            0
        } else if requested != 0 {
            requested
        } else if TASK_SIZE < full {
            TASK_SIZE
        } else if full > SZ_4G {
            full / 2
        } else {
            full - PANTHOR_VM_MIN_KERNEL_VA_SIZE
        };
        match range != 0 && full - PANTHOR_VM_MIN_KERNEL_VA_SIZE < range {
            true => full - PANTHOR_VM_MIN_KERNEL_VA_SIZE,
            false => range,
        }
    }

    unsafe fn vm_create(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_panthor_vm_create>() };
        let full = self.full_va_range();
        if args.flags != 0 {
            return -(EINVAL as i64);
        }
        let user_range = self.user_va_range(args.user_va_range);
        if user_range == 0 || args.user_va_range != 0 && args.user_va_range > user_range {
            return -(EINVAL as i64);
        }
        let kernel_range = 1u64 << (full - user_range).ilog2();
        let kernel_start = full - kernel_range;
        let mut files = self.files.lock();
        let state = files.entry(file).or_default();
        let id = lowest_free(1, state.vms.keys().copied());
        let Some(id) = id.filter(|&id| id <= PANTHOR_MAX_VMS_PER_FILE) else {
            return -(EBUSY as i64);
        };
        let root = self.next_vm_root.fetch_add(1, Relaxed);
        state.vms.insert(id, Vm { root });
        args.user_va_range = kernel_start;
        args.id = id;
        0
    }

    unsafe fn vm_destroy(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &*arg.cast::<drm_panthor_vm_destroy>() };
        if args.pad != 0 {
            return -(EINVAL as i64);
        }
        let mut files = self.files.lock();
        let removed = files
            .get_mut(&file)
            .and_then(|state| state.vms.remove(&args.id));
        match removed {
            Some(_) => 0,
            None => -(EINVAL as i64),
        }
    }

    unsafe fn bo_create(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_panthor_bo_create>() };
        let flags = match self.kernel >= KernelVersion::V7_0 {
            true => DRM_PANTHOR_BO_NO_MMAP | DRM_PANTHOR_BO_WB_MMAP,
            false => DRM_PANTHOR_BO_NO_MMAP,
        };
        if args.size == 0 || args.pad != 0 || args.flags & !flags != 0 {
            return -(EINVAL as i64);
        }
        if args.flags & DRM_PANTHOR_BO_NO_MMAP != 0 && args.flags & DRM_PANTHOR_BO_WB_MMAP != 0 {
            return -(EINVAL as i64);
        }
        let exclusive_vm_root = match args.exclusive_vm_id {
            0 => None,
            id => match self.vm(file, id) {
                Some(vm) => Some(vm.root),
                None => return -(EINVAL as i64),
            },
        };
        let size = args.size.next_multiple_of(PAGE_SIZE);
        let buffer = Buffer {
            flags: args.flags,
            exclusive_vm_root,
        };
        let buffers = self.buffers.file(file);
        args.handle = buffers.create_with_offset(buffer, size).handle;
        args.size = size;
        0
    }

    unsafe fn set_user_mmio_offset(&self, arg: *mut u8) -> i64 {
        let args = unsafe { &*arg.cast::<drm_panthor_set_user_mmio_offset>() };
        if args.offset != DRM_PANTHOR_USER_MMIO_OFFSET_32BIT
            && args.offset != DRM_PANTHOR_USER_MMIO_OFFSET_64BIT
        {
            return -(EINVAL as i64);
        }
        0
    }

    unsafe fn bo_set_label(&self, file: u64, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let args = unsafe { &*arg.cast::<drm_panthor_bo_set_label>() };
        if args.pad != 0 {
            return -(EINVAL as i64);
        }
        if !self.buffers.file(file).contains(args.handle) {
            return -(ENOENT as i64);
        }
        if args.label == 0 {
            return 0;
        }
        match strndup_user(user, args.label, PANTHOR_BO_LABEL_MAXLEN) {
            Ok(_) => 0,
            Err(EINVAL) => -(E2BIG as i64),
            Err(errno) => -(errno as i64),
        }
    }

    unsafe fn bo_mmap_offset(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_panthor_bo_mmap_offset>() };
        if args.pad != 0 {
            return -(EINVAL as i64);
        }
        let buffers = self.buffers.file(file);
        let Some(flags) = buffers.with(args.handle, |buffer| buffer.flags) else {
            return -(ENOENT as i64);
        };
        if flags & DRM_PANTHOR_BO_NO_MMAP != 0 {
            return -(EPERM as i64);
        }
        args.offset = buffers.offset(args.handle).unwrap();
        0
    }
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "panthor"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        format!("panthor:gpu={}:kernel={}", self.gpu.name, self.kernel)
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn desc(&self) -> &'static str {
        "Panthor DRM driver"
    }

    fn driver_version(&self) -> DriverVersion {
        self.kernel.panthor()
    }

    fn uevent_driver(&self) -> &'static str {
        "panthor"
    }

    fn platform(&self) -> Option<&'static Platform> {
        self.gpu.platform.as_ref()
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        ioctls::PANTHOR
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
                Func::PanthorIoctlDevQuery => self.dev_query(arg, user),
                Func::PanthorIoctlVmCreate => self.vm_create(file, arg),
                Func::PanthorIoctlVmDestroy => self.vm_destroy(file, arg),
                Func::PanthorIoctlBoCreate => self.bo_create(file, arg),
                Func::PanthorIoctlBoMmapOffset => self.bo_mmap_offset(file, arg),
                Func::PanthorIoctlSetUserMmioOffset => self.set_user_mmio_offset(arg),
                Func::PanthorIoctlVmBind => self.vm_bind(file, arg, user),
                Func::PanthorIoctlBoSetLabel => self.bo_set_label(file, arg, user),
                _ => return None,
            }
        };
        Some(ret)
    }

    fn export(&self, file: u64, handle: u32) -> Result<Export, i32> {
        self.buffers
            .file(file)
            .export(handle, |buffer| match buffer.exclusive_vm_root {
                None => Ok(()),
                Some(_) => Err(EINVAL),
            })
    }

    fn import(&self, file: u64, object: u32) -> Option<u32> {
        self.buffers.file(file).import(object)
    }

    fn import_foreign(&self, _file: u64, _export: &Export) -> NewHandle {
        unimplemented!("importing foreign dma-bufs into panthor");
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
