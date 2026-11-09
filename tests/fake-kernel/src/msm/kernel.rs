use crate::Buffers;
use crate::DriverFeature;
use crate::DriverVersion;
use crate::Drm;
use crate::Export;
use crate::Kernel;
use crate::KernelVersion;
use crate::Master;
use crate::NewHandle;
use crate::PAGE_SIZE;
use crate::Platform;
use crate::Prime;
use crate::Syncobjs;
use crate::UserMemory;
use crate::drm_mm::DrmMm;
use crate::drm_mm::Request;
use crate::id_fields;
use crate::ioctls;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::msm::chips::Chip;
use crate::msm::chips::Family;
use crate::msm::chips::Ubwc;
use crate::msm::chips::chip;
use crate::uapi::msm::MSM_BO_CACHE_MASK;
use crate::uapi::msm::MSM_BO_CACHED;
use crate::uapi::msm::MSM_BO_CACHED_COHERENT;
use crate::uapi::msm::MSM_BO_FLAGS;
use crate::uapi::msm::MSM_BO_NO_SHARE;
use crate::uapi::msm::MSM_BO_WC;
use crate::uapi::msm::MSM_INFO_GET_FLAGS;
use crate::uapi::msm::MSM_INFO_GET_IOVA;
use crate::uapi::msm::MSM_INFO_GET_METADATA;
use crate::uapi::msm::MSM_INFO_GET_NAME;
use crate::uapi::msm::MSM_INFO_GET_OFFSET;
use crate::uapi::msm::MSM_INFO_SET_IOVA;
use crate::uapi::msm::MSM_INFO_SET_METADATA;
use crate::uapi::msm::MSM_INFO_SET_NAME;
use crate::uapi::msm::MSM_PARAM_CHIP_ID;
use crate::uapi::msm::MSM_PARAM_EN_VM_BIND;
use crate::uapi::msm::MSM_PARAM_FAULTS;
use crate::uapi::msm::MSM_PARAM_GMEM_BASE;
use crate::uapi::msm::MSM_PARAM_GMEM_SIZE;
use crate::uapi::msm::MSM_PARAM_GPU_ID;
use crate::uapi::msm::MSM_PARAM_HAS_PRR;
use crate::uapi::msm::MSM_PARAM_HIGHEST_BANK_BIT;
use crate::uapi::msm::MSM_PARAM_MACROTILE_MODE;
use crate::uapi::msm::MSM_PARAM_MAX_FREQ;
use crate::uapi::msm::MSM_PARAM_PP_PGTABLE;
use crate::uapi::msm::MSM_PARAM_PRIORITIES;
use crate::uapi::msm::MSM_PARAM_RAYTRACING;
use crate::uapi::msm::MSM_PARAM_SUSPENDS;
use crate::uapi::msm::MSM_PARAM_TIMESTAMP;
use crate::uapi::msm::MSM_PARAM_UBWC_SWIZZLE;
use crate::uapi::msm::MSM_PARAM_UCHE_TRAP_BASE;
use crate::uapi::msm::MSM_PARAM_VA_SIZE;
use crate::uapi::msm::MSM_PARAM_VA_START;
use crate::uapi::msm::MSM_PERFCNTR_FLAGS;
use crate::uapi::msm::MSM_PERFCNTR_STREAM;
use crate::uapi::msm::MSM_PIPE_3D0;
use crate::uapi::msm::MSM_SUBMITQUEUE_ALLOW_PREEMPT;
use crate::uapi::msm::MSM_SUBMITQUEUE_FLAGS;
use crate::uapi::msm::MSM_SUBMITQUEUE_VM_BIND;
use crate::uapi::msm::MSM_VM_BIND_FENCE_FD_IN;
use crate::uapi::msm::MSM_VM_BIND_FENCE_FD_OUT;
use crate::uapi::msm::MSM_VM_BIND_FLAGS;
use crate::uapi::msm::MSM_VM_BIND_OP_FLAGS;
use crate::uapi::msm::MSM_VM_BIND_OP_MAP;
use crate::uapi::msm::MSM_VM_BIND_OP_MAP_NULL;
use crate::uapi::msm::MSM_VM_BIND_OP_UNMAP;
use crate::uapi::msm::drm_msm_gem_info;
use crate::uapi::msm::drm_msm_gem_new;
use crate::uapi::msm::drm_msm_param;
use crate::uapi::msm::drm_msm_perfcntr_config;
use crate::uapi::msm::drm_msm_perfcntr_group;
use crate::uapi::msm::drm_msm_submitqueue;
use crate::uapi::msm::drm_msm_vm_bind;
use crate::uapi::msm::drm_msm_vm_bind_op;
use crate::user::from_bytes;
use libc::EBUSY;
use libc::EFAULT;
use libc::EINVAL;
use libc::ENOENT;
use libc::ENOSPC;
use libc::ENXIO;
use libc::EOVERFLOW;
use libc::EPERM;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::ffi::c_char;

const NAME_SIZE: u32 = 32;
const METADATA_MAX_SIZE: u32 = 128;
const NR_SCHED_PRIORITIES: u64 = 3;
const ADRENO_VM_START: u64 = 0x100000000;
const UCHE_TRAP_BASE: u64 = 0x1fffffffff000;
const SPEEDBIN_UNFUSED: u64 = 0xffff;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Queue {
    Ring,
    VmBind,
}

struct Vm {
    mm: DrmMm,
    iovas: BTreeMap<u32, u64>,
}

struct File {
    userspace_managed_vm: bool,
    vm: Option<Vm>,
    va_size: u64,
    queues: BTreeMap<u32, Queue>,
    next_queue: u32,
}

impl File {
    fn create_vm(&mut self) -> &mut Vm {
        self.vm.get_or_insert_with(|| Vm {
            mm: DrmMm::new(ADRENO_VM_START, self.va_size),
            iovas: BTreeMap::new(),
        })
    }
}

#[derive(Copy, Clone)]
struct Buffer {
    flags: u32,
}

pub struct Device {
    chip: &'static Chip,
    kernel: KernelVersion,
    files: Mutex<BTreeMap<u64, File>>,
    buffers: Buffers<Buffer>,
    syncobjs: Syncobjs,
    master: Master,
}

fn names_equal(name: &str, user: &[c_char; 16]) -> bool {
    for (i, &b) in user.iter().enumerate() {
        let a = name.as_bytes().get(i).copied().unwrap_or(0);
        if a != b as u8 {
            return false;
        }
        if a == 0 {
            return true;
        }
    }
    true
}

impl Device {
    pub fn new(name: &str, kernel: KernelVersion) -> Self {
        if kernel < KernelVersion::V6_17 {
            unimplemented!("msm without a separate GPU device before 6.17");
        }
        Self {
            chip: chip(name),
            kernel,
            files: Mutex::new(BTreeMap::new()),
            buffers: Buffers::default(),
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

    fn with_file<R>(&self, file: u64, f: impl FnOnce(&mut File) -> R) -> R {
        let va_size = self.va_size();
        let mut files = self.files.lock();
        let file = files.entry(file).or_insert_with(|| File {
            userspace_managed_vm: false,
            vm: None,
            va_size,
            queues: BTreeMap::from([(0, Queue::Ring)]),
            next_queue: 1,
        });
        f(file)
    }

    fn has_prr(&self) -> bool {
        let compatible = |name| self.chip.smmu_compatible.contains(&name);
        compatible("qcom,smmu-500")
            && !compatible("qcom,sm8250-smmu-500")
            && compatible("qcom,adreno-smmu")
    }

    fn nr_rings(&self) -> u64 {
        if self.chip.preemption { 4 } else { 1 }
    }

    unsafe fn get_param(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_msm_param>() };
        if args.pipe != MSM_PIPE_3D0 || args.pad != 0 {
            return -(EINVAL as i64);
        }
        self.with_file(file, |file| {
            file.create_vm();
        });
        if args.len != 0 {
            return -(EINVAL as i64);
        }
        let chip = self.chip;
        let ubwc = |f: fn(&Ubwc) -> u64| match &chip.ubwc {
            Some(ubwc) => Ok(f(ubwc)),
            None => Err(-(ENOENT as i64)),
        };
        let value = match args.param {
            MSM_PARAM_GPU_ID => chip.revn as u64,
            MSM_PARAM_GMEM_SIZE => chip.gmem,
            MSM_PARAM_MAX_FREQ => chip.max_freq,
            MSM_PARAM_TIMESTAMP => 0,
            MSM_PARAM_GMEM_BASE => match chip.family >= Family::A6xxGen3 {
                true => 0,
                false => 0x100000,
            },
            MSM_PARAM_CHIP_ID => match chip.revn {
                0 => chip.chip_id as u64 | SPEEDBIN_UNFUSED << 32,
                _ => chip.chip_id as u64,
            },
            MSM_PARAM_PRIORITIES => self.nr_rings() * NR_SCHED_PRIORITIES,
            MSM_PARAM_PP_PGTABLE => 0,
            MSM_PARAM_FAULTS => 0,
            MSM_PARAM_SUSPENDS => 0,
            MSM_PARAM_HIGHEST_BANK_BIT => match ubwc(|u| u.highest_bank_bit as u64) {
                Ok(value) => value,
                Err(e) => return e,
            },
            MSM_PARAM_UBWC_SWIZZLE => match ubwc(|u| u.ubwc_swizzle as u64) {
                Ok(value) => value,
                Err(e) => return e,
            },
            MSM_PARAM_MACROTILE_MODE => match ubwc(|u| u.macrotile_mode as u64) {
                Ok(value) => value,
                Err(e) => return e,
            },
            MSM_PARAM_UCHE_TRAP_BASE => UCHE_TRAP_BASE,
            MSM_PARAM_RAYTRACING => chip.ray_tracing as u64,
            MSM_PARAM_HAS_PRR => self.has_prr() as u64,
            MSM_PARAM_VA_START => ADRENO_VM_START,
            MSM_PARAM_VA_SIZE => self.va_size(),
            param => unimplemented!("MSM_PARAM {param:#x}"),
        };
        args.value = value;
        0
    }

    unsafe fn set_param(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &*arg.cast::<drm_msm_param>() };
        if args.pipe != MSM_PIPE_3D0 || args.pad != 0 || args.len != 0 {
            return -(EINVAL as i64);
        }
        match args.param {
            MSM_PARAM_EN_VM_BIND => self.with_file(file, |file| {
                if file.vm.is_some() {
                    return -(EBUSY as i64);
                }
                file.userspace_managed_vm = args.value != 0;
                0
            }),
            param => unimplemented!("setting MSM_PARAM {param:#x}"),
        }
    }

    unsafe fn submitqueue_new(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_msm_submitqueue>() };
        if args.flags & !MSM_SUBMITQUEUE_FLAGS != 0 {
            return -(EINVAL as i64);
        }
        let nr_rings = self.nr_rings();
        self.with_file(file, |file| {
            let queue = if args.flags & MSM_SUBMITQUEUE_VM_BIND != 0 {
                if !file.userspace_managed_vm || args.prio != 0 {
                    return -(EINVAL as i64);
                }
                file.create_vm();
                Queue::VmBind
            } else {
                let preemption_supported = nr_rings == 1;
                if args.flags & MSM_SUBMITQUEUE_ALLOW_PREEMPT != 0 && preemption_supported {
                    return -(EINVAL as i64);
                }
                let ring = args.prio as u64 / NR_SCHED_PRIORITIES;
                if ring >= nr_rings {
                    return -(EINVAL as i64);
                }
                Queue::Ring
            };
            args.id = file.next_queue;
            file.next_queue += 1;
            file.queues.insert(args.id, queue);
            0
        })
    }

    unsafe fn submitqueue_close(&self, file: u64, arg: *mut u8) -> i64 {
        let id = unsafe { *arg.cast::<u32>() };
        if id == 0 {
            return -(ENOENT as i64);
        }
        self.with_file(file, |file| match file.queues.remove(&id) {
            Some(_) => 0,
            None => -(ENOENT as i64),
        })
    }

    fn va_size(&self) -> u64 {
        (1 << self.chip.smmu_input_bits) - ADRENO_VM_START
    }

    fn lookup_op(&self, op: &drm_msm_vm_bind_op) -> bool {
        let aligned = |value: u64| value.is_multiple_of(PAGE_SIZE);
        let in_range = op.iova.checked_add(op.range).is_some_and(|end| {
            op.iova >= ADRENO_VM_START && end <= ADRENO_VM_START + self.va_size()
        });
        let handle_ok = match op.op {
            MSM_VM_BIND_OP_MAP => op.handle != 0,
            _ => op.handle == 0,
        };
        op.flags & !MSM_VM_BIND_OP_FLAGS == 0
            && aligned(op.iova)
            && aligned(op.obj_offset)
            && aligned(op.range)
            && in_range
            && handle_ok
            && matches!(
                op.op,
                MSM_VM_BIND_OP_MAP | MSM_VM_BIND_OP_MAP_NULL | MSM_VM_BIND_OP_UNMAP
            )
            && (op.op != MSM_VM_BIND_OP_MAP_NULL || self.has_prr())
    }

    unsafe fn vm_bind(
        &self,
        file: u64,
        arg: *mut u8,
        user: &dyn UserMemory,
        prime: &mut dyn Prime,
    ) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_msm_vm_bind>() };
        let queue = self.with_file(file, |file| {
            file.create_vm();
            if !file.userspace_managed_vm || args.flags & !MSM_VM_BIND_FLAGS != 0 {
                return Err(-(EINVAL as i64));
            }
            match file.queues.get(&args.queue_id) {
                None => Err(-(ENOENT as i64)),
                Some(Queue::VmBind) => Ok(()),
                Some(Queue::Ring) => Err(-(EINVAL as i64)),
            }
        });
        if let Err(e) = queue {
            return e;
        }
        if args.flags & MSM_VM_BIND_FENCE_FD_IN != 0
            || args.in_syncobjs != 0
            || args.out_syncobjs != 0
        {
            unimplemented!("VM_BIND in-fences and syncobjs");
        }
        let mut ops = Vec::new();
        for index in 0..args.nr_ops as u64 {
            let op = match args.nr_ops {
                1 => unsafe { args.anon.op },
                _ => {
                    let stride = size_of::<drm_msm_vm_bind_op>() as u64;
                    let address = unsafe { args.anon.ops }.wrapping_add(index * stride);
                    match user.read_value::<drm_msm_vm_bind_op>(address) {
                        Ok(op) => op,
                        Err(_) => return -(EFAULT as i64),
                    }
                }
            };
            if !self.lookup_op(&op) {
                return -(EINVAL as i64);
            }
            ops.push(op);
        }
        let buffers = self.buffers.file(file);
        for op in ops.iter().filter(|op| op.handle != 0) {
            let Some(size) = buffers.size(op.handle) else {
                return -(EINVAL as i64);
            };
            if op.range.wrapping_add(op.obj_offset) > size {
                return -(EINVAL as i64);
            }
        }
        if args.flags & MSM_VM_BIND_FENCE_FD_OUT != 0 {
            match prime.sync_file() {
                Ok(fd) => args.fence_fd = fd,
                Err(e) => return -(e as i64),
            }
        }
        0
    }

    unsafe fn gem_info(&self, file: u64, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_msm_gem_info>() };
        if args.pad != 0 {
            return -(EINVAL as i64);
        }
        match args.info {
            MSM_INFO_GET_OFFSET | MSM_INFO_GET_IOVA | MSM_INFO_SET_IOVA | MSM_INFO_GET_FLAGS => {
                if args.len != 0 {
                    return -(EINVAL as i64);
                }
            }
            MSM_INFO_SET_NAME
            | MSM_INFO_GET_NAME
            | MSM_INFO_SET_METADATA
            | MSM_INFO_GET_METADATA => {}
            _ => return -(EINVAL as i64),
        }
        let buffers = self.buffers.file(file);
        let Some(flags) = buffers.with(args.handle, |buffer| buffer.flags) else {
            return -(ENOENT as i64);
        };
        let object = buffers.object(args.handle).unwrap();
        let size = buffers.size(args.handle).unwrap();
        match args.info {
            MSM_INFO_GET_IOVA => {
                let iova = self.with_file(file, |file| {
                    if file.userspace_managed_vm {
                        return Err(EINVAL);
                    }
                    let vm = file.create_vm();
                    if let Some(&iova) = vm.iovas.get(&object) {
                        return Ok(iova);
                    }
                    let request = Request {
                        size,
                        alignment: PAGE_SIZE,
                        color: 0,
                        range: 0..u64::MAX,
                    };
                    let iova = vm.mm.insert(&request).ok_or(ENOSPC)?;
                    vm.iovas.insert(object, iova);
                    Ok(iova)
                });
                match iova {
                    Ok(iova) => args.value = iova,
                    Err(e) => return -(e as i64),
                }
            }
            MSM_INFO_GET_OFFSET => args.value = buffers.offset(args.handle).unwrap(),
            MSM_INFO_GET_FLAGS => args.value = (flags & MSM_BO_FLAGS) as u64,
            MSM_INFO_SET_NAME => {
                if args.len >= NAME_SIZE {
                    return -(EINVAL as i64);
                }
                let mut name = vec![0; args.len as usize];
                if user.read(args.value, &mut name).is_err() {
                    return -(EFAULT as i64);
                }
            }
            MSM_INFO_SET_METADATA => {
                if args.len > METADATA_MAX_SIZE {
                    return -(EOVERFLOW as i64);
                }
                let mut metadata = vec![0; args.len as usize];
                if user.read(args.value, &mut metadata).is_err() {
                    return -(EFAULT as i64);
                }
            }
            info => unimplemented!("MSM_INFO {info:#x}"),
        }
        0
    }

    unsafe fn perfcntr_config(&self, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let args = unsafe { &*arg.cast::<drm_msm_perfcntr_config>() };
        let groups = self.chip.perfcntr_groups;
        if groups.is_empty() {
            return -(ENXIO as i64);
        }
        if args.nr_groups as usize > groups.len() {
            return -(EINVAL as i64);
        }
        if args.flags & !MSM_PERFCNTR_FLAGS != 0
            || (args.nr_groups != 0 && (args.group_stride == 0 || args.groups == 0))
        {
            return -(EINVAL as i64);
        }
        if args.flags & MSM_PERFCNTR_STREAM != 0 {
            unimplemented!("perfcntr streams");
        }
        if args.period != 0 || args.bufsz_shift != 0 {
            return -(EINVAL as i64);
        }
        let mut requested = vec![false; groups.len()];
        for i in 0..args.nr_groups {
            let offset = i.wrapping_mul(args.group_stride);
            let address = args.groups.wrapping_add(offset as u64);
            let size = (args.group_stride as usize).min(size_of::<drm_msm_perfcntr_group>());
            let mut bytes = vec![0u8; size];
            if user.read(address, &mut bytes).is_err() {
                return -(EFAULT as i64);
            }
            let group: drm_msm_perfcntr_group = from_bytes(&bytes);
            if group.pad != 0 {
                return -(EINVAL as i64);
            }
            let Some(index) = groups
                .iter()
                .position(|g| names_equal(g.name, &group.group_name))
            else {
                return -(EINVAL as i64);
            };
            if requested[index] {
                return -(EINVAL as i64);
            }
            if group.nr_countables > groups[index].num_counters || group.countables != 0 {
                return -(EINVAL as i64);
            }
            requested[index] = true;
        }
        0
    }

    unsafe fn gem_new(&self, file: u64, arg: *mut u8) -> i64 {
        let args = unsafe { &mut *arg.cast::<drm_msm_gem_new>() };
        if args.flags & !MSM_BO_FLAGS != 0 {
            return -(EINVAL as i64);
        }
        let size = args.size.next_multiple_of(PAGE_SIZE);
        if size == 0 {
            return -(EINVAL as i64);
        }
        match args.flags & MSM_BO_CACHE_MASK {
            MSM_BO_CACHED | MSM_BO_WC => {}
            MSM_BO_CACHED_COHERENT => unimplemented!("MSM_BO_CACHED_COHERENT"),
            _ => return -(EINVAL as i64),
        }
        if args.flags & MSM_BO_NO_SHARE != 0 && self.with_file(file, |file| file.vm.is_none()) {
            unimplemented!("MSM_BO_NO_SHARE without a VM");
        }
        let buffer = Buffer { flags: args.flags };
        let buffers = self.buffers.file(file);
        args.handle = buffers.create_with_offset(buffer, size).handle;
        0
    }
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "msm"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        format!("msm:chip={}:kernel={}", self.chip.name, self.kernel)
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn desc(&self) -> &'static str {
        "MSM Snapdragon DRM"
    }

    fn driver_version(&self) -> DriverVersion {
        self.kernel.msm()
    }

    fn uevent_driver(&self) -> &'static str {
        "adreno"
    }

    fn platform(&self) -> Option<&'static Platform> {
        Some(&self.chip.platform)
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        ioctls::MSM
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
        prime: &mut dyn Prime,
    ) -> Option<i64> {
        let ret = match func {
            Func::MsmIoctlVmBind => unsafe { self.vm_bind(file, arg, user, prime) },
            Func::MsmIoctlGetParam => unsafe { self.get_param(file, arg) },
            Func::MsmIoctlSetParam => unsafe { self.set_param(file, arg) },
            Func::MsmIoctlGemNew => unsafe { self.gem_new(file, arg) },
            Func::MsmIoctlGemInfo => unsafe { self.gem_info(file, arg, user) },
            Func::MsmIoctlPerfcntrConfig => unsafe { self.perfcntr_config(arg, user) },
            Func::MsmIoctlSubmitqueueNew => unsafe { self.submitqueue_new(file, arg) },
            Func::MsmIoctlSubmitqueueClose => unsafe { self.submitqueue_close(file, arg) },
            _ => return None,
        };
        Some(ret)
    }

    fn export(&self, file: u64, handle: u32) -> Result<Export, i32> {
        self.buffers
            .file(file)
            .export(handle, |buffer| match buffer.flags & MSM_BO_NO_SHARE {
                0 => Ok(()),
                _ => Err(EPERM),
            })
    }

    fn import(&self, file: u64, object: u32) -> Option<u32> {
        self.buffers.file(file).import(object)
    }

    fn import_foreign(&self, _file: u64, _export: &Export) -> NewHandle {
        unimplemented!("importing dma-bufs into msm");
    }

    fn close_buffer(&self, file: u64, handle: u32) -> bool {
        let buffers = self.buffers.file(file);
        let Some(object) = buffers.object(handle) else {
            return false;
        };
        buffers.close(handle);
        let mut files = self.files.lock();
        let managed = files
            .get_mut(&file)
            .filter(|file| !file.userspace_managed_vm);
        if let Some(vm) = managed.and_then(|file| file.vm.as_mut())
            && let Some(iova) = vm.iovas.remove(&object)
        {
            vm.mm.remove(iova);
        }
        true
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
        self.files.lock().remove(&file);
        self.buffers.release_file(file);
        self.syncobjs.release_file(file);
    }
}
