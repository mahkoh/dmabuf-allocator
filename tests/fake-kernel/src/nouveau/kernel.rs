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
use crate::PCI_VENDOR_ID_NVIDIA;
use crate::Pci;
use crate::Prime;
use crate::Syncobjs;
use crate::UserMemory;
use crate::errno;
use crate::id_fields;
use crate::ioctls;
use crate::ioctls::Func;
use crate::ioctls::IoctlDesc;
use crate::nouveau::chips::Chip;
use crate::nouveau::chips::Hardware;
use crate::nouveau::chips::chip;
use crate::syncobj::Fence;
use crate::uapi::drm::DRM_COMMAND_BASE;
use crate::uapi::ioctl::_IOC_NR;
use crate::uapi::ioctl::_IOC_SIZE;
use crate::uapi::nouveau::*;
use crate::uapi::nvctrl::NV2080_ENGINE_TYPE_GRAPHICS;
use crate::uapi::nvctrl::NV2080_ENGINE_TYPE_IS_COPY;
use libc::EEXIST;
use libc::EFAULT;
use libc::EINVAL;
use libc::ENOENT;
use libc::ENOSYS;
use libc::EOPNOTSUPP;
use libc::EPERM;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ffi::c_char;
use std::mem::size_of;

const PAGE_SHIFT: u32 = PAGE_SIZE.trailing_zeros();
const NOUVEAU_VA_SPACE_END: u64 = 1 << 47;
const NV50_DMA_PUSH_MAX_LENGTH: u32 = 0x7fffff;
const NV50_DMA_IB_MAX: u64 = 0x02000 / 8 - 1;
const R535_RSVD_CHIDS: i32 = 0;
const R570_RSVD_CHIDS: i32 = 1;
const KERNEL_CHANNELS: i32 = 2;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Uvmm {
    Undecided,
    Disabled,
    Enabled { kernel_start: u64, kernel_size: u64 },
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Runlist {
    Gr,
    Ce,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Object {
    Device,
    Engine,
}

struct File {
    uvmm: Uvmm,
    objects: BTreeMap<u64, Object>,
    channels: BTreeMap<i32, Runlist>,
}

impl Default for File {
    fn default() -> Self {
        File {
            uvmm: Uvmm::Undecided,
            objects: BTreeMap::new(),
            channels: BTreeMap::new(),
        }
    }
}

#[derive(Copy, Clone)]
struct Buffer {
    size: u64,
    kind: u8,
    mode: u32,
    contig: bool,
    valid_domains: u32,
    no_share: bool,
}

struct Signal {
    object: u64,
    timeline: bool,
    point: u64,
}

impl Buffer {
    fn placement(&self) -> u32 {
        if self.valid_domains & NOUVEAU_GEM_DOMAIN_VRAM != 0 {
            NOUVEAU_GEM_DOMAIN_VRAM
        } else if self.valid_domains & NOUVEAU_GEM_DOMAIN_GART != 0 {
            NOUVEAU_GEM_DOMAIN_GART
        } else {
            unimplemented!("buffers in system memory");
        }
    }
}

pub struct Device {
    chip: &'static Chip,
    kernel: KernelVersion,
    files: Mutex<BTreeMap<u64, File>>,
    chids: Mutex<BTreeSet<i32>>,
    buffers: Buffers<Buffer>,
    syncobjs: Syncobjs,
    master: Master,
}

impl Device {
    pub fn new(name: &str, kernel: KernelVersion) -> Self {
        let chip = chip(name);
        if let Some(first) = chip.first_kernel {
            assert!(kernel >= first, "nouveau chip {name} needs kernel {first}");
        }
        Self {
            chip,
            kernel,
            files: Mutex::new(BTreeMap::new()),
            chids: Mutex::new(BTreeSet::new()),
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
        f(self.files.lock().entry(file).or_default())
    }

    fn disable_uvmm_noinit(&self, file: u64) {
        self.with_file(file, |file| {
            if file.uvmm == Uvmm::Undecided {
                file.uvmm = Uvmm::Disabled;
            }
        });
    }

    fn getparam(&self, arg: *mut u8) -> i64 {
        let getparam = unsafe { &mut *arg.cast::<drm_nouveau_getparam>() };
        getparam.value = match getparam.param {
            NOUVEAU_GETPARAM_CHIPSET_ID => self.chipset() as u64,
            NOUVEAU_GETPARAM_PCI_DEVICE => self.chip.pci_id as u64,
            NOUVEAU_GETPARAM_EXEC_PUSH_MAX => NV50_DMA_IB_MAX / 2 - 1,
            NOUVEAU_GETPARAM_VRAM_BAR_SIZE => self.hardware().gpu.bar1_size,
            NOUVEAU_GETPARAM_HAS_VMA_TILEMODE => 1,
            NOUVEAU_GETPARAM_VRAM_USED => self.buffers.sum(|buffer| match buffer.placement() {
                NOUVEAU_GEM_DOMAIN_VRAM => buffer.size,
                _ => 0,
            }),
            NOUVEAU_GETPARAM_GRAPH_UNITS => {
                let tpcs = self.hardware().gpu.tpcs_per_gpc;
                let gpcs = tpcs.iter().filter(|&&tpcs| tpcs != 0).count() as u64;
                let tpcs: u64 = tpcs.iter().map(|&tpcs| tpcs as u64).sum();
                tpcs << 8 | gpcs
            }
            param => unimplemented!("NOUVEAU_GETPARAM {param}"),
        };
        0
    }

    fn vm_init(&self, file: u64, arg: *mut u8) -> i64 {
        let init = unsafe { &*arg.cast::<drm_nouveau_vm_init>() };
        let Some(end) = init
            .kernel_managed_addr
            .checked_add(init.kernel_managed_size)
        else {
            return errno(EINVAL);
        };
        if end > NOUVEAU_VA_SPACE_END {
            return errno(EINVAL);
        }
        self.with_file(file, |file| match file.uvmm {
            Uvmm::Disabled => errno(ENOSYS),
            Uvmm::Enabled { .. } => unimplemented!("a second VM_INIT"),
            Uvmm::Undecided => {
                file.uvmm = Uvmm::Enabled {
                    kernel_start: init.kernel_managed_addr,
                    kernel_size: init.kernel_managed_size,
                };
                0
            }
        })
    }

    fn vm_bind(&self, file: u64, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let req = unsafe { &*arg.cast::<drm_nouveau_vm_bind>() };
        let Uvmm::Enabled {
            kernel_start,
            kernel_size,
        } = self.with_file(file, |file| file.uvmm)
        else {
            return errno(ENOSYS);
        };
        if req.flags != 0 {
            unimplemented!("VM_BIND flags {:#x}", req.flags);
        }
        if req.wait_count != 0 || req.sig_count != 0 {
            unimplemented!(
                "VM_BIND with {} waits and {} signals",
                req.wait_count,
                req.sig_count
            );
        }
        let ops: Result<Vec<drm_nouveau_vm_bind_op>, i32> =
            user.read_array(req.op_ptr, req.op_count as usize);
        let Ok(ops) = ops else {
            return errno(EFAULT);
        };
        let range_valid = |addr: u64, range: u64| {
            let Some(end) = addr.checked_add(range) else {
                return false;
            };
            let kernel_end = kernel_start + kernel_size;
            addr & (PAGE_SIZE - 1) == 0
                && range & (PAGE_SIZE - 1) == 0
                && end <= NOUVEAU_VA_SPACE_END
                && !(kernel_size != 0 && addr < kernel_end && kernel_start < end)
        };
        let buffers = self.buffers.file(file);
        for op in &ops {
            if op.flags & DRM_NOUVEAU_VM_BIND_SPARSE != 0 {
                unimplemented!("sparse VM_BIND operations");
            }
            if op.op == DRM_NOUVEAU_VM_BIND_OP_MAP {
                let Some(size) = buffers.with(op.handle, |buffer| buffer.size) else {
                    return errno(ENOENT);
                };
                if op.bo_offset & (PAGE_SIZE - 1) != 0
                    || size <= op.bo_offset
                    || op.range > size - op.bo_offset
                {
                    return errno(EINVAL);
                }
            }
            if !range_valid(op.addr, op.range) {
                return errno(EINVAL);
            }
        }
        let known = |op: &drm_nouveau_vm_bind_op| {
            op.op == DRM_NOUVEAU_VM_BIND_OP_MAP || op.op == DRM_NOUVEAU_VM_BIND_OP_UNMAP
        };
        match ops.iter().all(known) {
            true => 0,
            false => errno(EINVAL),
        }
    }

    fn exec(&self, file: u64, arg: *mut u8, user: &dyn UserMemory) -> i64 {
        let req = unsafe { &*arg.cast::<drm_nouveau_exec>() };
        let (uvmm, channel) = self.with_file(file, |file| {
            let uvmm = matches!(file.uvmm, Uvmm::Enabled { .. });
            (uvmm, file.channels.contains_key(&(req.channel as i32)))
        });
        if !uvmm {
            return errno(ENOSYS);
        }
        if !channel {
            return errno(ENOENT);
        }
        if req.push_count as u64 > NV50_DMA_IB_MAX / 2 - 1 {
            return errno(EINVAL);
        }
        let pushes: Result<Vec<drm_nouveau_exec_push>, i32> =
            user.read_array(req.push_ptr, req.push_count as usize);
        let Ok(pushes) = pushes else {
            return errno(EFAULT);
        };
        let waits: Result<Vec<drm_nouveau_sync>, i32> =
            user.read_array(req.wait_ptr, req.wait_count as usize);
        let Ok(waits) = waits else {
            return errno(EFAULT);
        };
        let signals: Result<Vec<drm_nouveau_sync>, i32> =
            user.read_array(req.sig_ptr, req.sig_count as usize);
        let Ok(signals) = signals else {
            return errno(EFAULT);
        };
        if pushes
            .iter()
            .any(|push| push.va_len > NV50_DMA_PUSH_MAX_LENGTH)
        {
            return errno(EINVAL);
        }
        for wait in &waits {
            let point = match wait.flags & DRM_NOUVEAU_SYNC_TYPE_MASK {
                DRM_NOUVEAU_SYNC_SYNCOBJ => 0,
                DRM_NOUVEAU_SYNC_TIMELINE_SYNCOBJ => wait.timeline_value,
                _ => return errno(EOPNOTSUPP),
            };
            if let Err(e) = self.syncobjs.find_fence(file, wait.handle, point, 0) {
                return errno(e);
            }
        }
        let mut objects = Vec::new();
        for signal in &signals {
            let timeline = match signal.flags & DRM_NOUVEAU_SYNC_TYPE_MASK {
                DRM_NOUVEAU_SYNC_SYNCOBJ => false,
                DRM_NOUVEAU_SYNC_TIMELINE_SYNCOBJ => true,
                _ => return errno(EINVAL),
            };
            let Some(object) = self.syncobjs.object(file, signal.handle) else {
                return errno(ENOENT);
            };
            objects.push(Signal {
                object,
                timeline,
                point: signal.timeline_value,
            });
        }
        for signal in objects {
            match signal.timeline {
                true => self.syncobjs.add_point(signal.object, signal.point),
                false => self.syncobjs.replace(signal.object, Some(Fence::Plain)),
            }
        }
        0
    }

    fn gem_new(&self, file: u64, arg: *mut u8) -> i64 {
        let req = unsafe { &mut *arg.cast::<drm_nouveau_gem_new>() };
        self.disable_uvmm_noinit(file);
        let uvmm = self.with_file(file, |file| matches!(file.uvmm, Uvmm::Enabled { .. }));
        let mut domain = req.info.domain;
        let no_share = domain & NOUVEAU_GEM_DOMAIN_NO_SHARE != 0;
        if no_share && !uvmm {
            return errno(EINVAL);
        }
        if domain & (NOUVEAU_GEM_DOMAIN_VRAM | NOUVEAU_GEM_DOMAIN_GART) == 0 {
            domain |= NOUVEAU_GEM_DOMAIN_CPU;
        }
        if domain & NOUVEAU_GEM_DOMAIN_COHERENT != 0 {
            unimplemented!("NOUVEAU_GEM_DOMAIN_COHERENT");
        }
        let mut size = req.info.size;
        if size == 0 {
            return errno(EINVAL);
        }
        let mmu = self.hardware().mmu;
        let kind = ((req.info.tile_flags & 0x0000ff00) >> 8) as u8;
        if !mmu.kind_valid(kind) {
            return errno(EINVAL);
        }
        let comp = mmu.kinds[kind as usize] != kind;
        if comp {
            unimplemented!("compressed kind {kind:#x}");
        }
        let mut page = None;
        for candidate in mmu.pages {
            if domain & NOUVEAU_GEM_DOMAIN_VRAM != 0 && !candidate.vram {
                continue;
            }
            if domain & NOUVEAU_GEM_DOMAIN_GART != 0
                && (!candidate.host || candidate.shift > PAGE_SHIFT)
            {
                continue;
            }
            page = Some(candidate.shift);
            if size >= 1 << candidate.shift {
                break;
            }
        }
        let page = page.expect("no page size supports the domains");
        size = size.next_multiple_of(1 << page).next_multiple_of(PAGE_SIZE);
        let buffer = Buffer {
            size,
            kind,
            mode: req.info.tile_mode,
            contig: req.info.tile_flags & NOUVEAU_GEM_TILE_NONCONTIG == 0,
            valid_domains: (NOUVEAU_GEM_DOMAIN_VRAM | NOUVEAU_GEM_DOMAIN_GART) & domain,
            no_share,
        };
        if !uvmm {
            unimplemented!("GEM_NEW with a kernel-managed VM");
        }
        let buffers = self.buffers.file(file);
        let handle = buffers.create(buffer, size).handle;
        req.info.handle = handle;
        req.info.map_handle = buffers.offset(handle).unwrap();
        self.gem_info(&buffer, &mut req.info);
        0
    }

    fn gem_info(&self, buffer: &Buffer, info: &mut drm_nouveau_gem_info) {
        info.domain = match buffer.valid_domains.is_power_of_two() {
            true => buffer.valid_domains,
            false => buffer.placement(),
        };
        info.offset = 0;
        info.size = buffer.size;
        info.tile_mode = buffer.mode;
        let contig = match buffer.contig {
            true => 0,
            false => NOUVEAU_GEM_TILE_NONCONTIG,
        };
        info.tile_flags = contig | (buffer.kind as u32) << 8;
    }

    fn hardware(&self) -> &'static Hardware {
        match &self.chip.hardware {
            Some(hardware) => hardware,
            None => unimplemented!("the hardware of {}", self.chip.name),
        }
    }

    fn chipset(&self) -> u16 {
        self.chip.chipset
    }

    fn family(&self) -> u8 {
        match self.chipset() & 0x1f0 {
            0x190 => NV_DEVICE_INFO_V0_ADA,
            0x1b0 => NV_DEVICE_INFO_V0_BLACKWELL,
            chipset => unimplemented!("the family of chipset {chipset:#x}"),
        }
    }

    fn has_engine(&self, engine: impl Fn(u32) -> bool) -> bool {
        self.hardware().gpu.engines.iter().any(|&e| engine(e))
    }

    fn classes(&self, runlist: Runlist) -> Vec<u32> {
        let gr = &self.hardware().gr;
        let mut classes = Vec::new();
        if runlist == Runlist::Gr {
            classes.extend([gr.i2m, gr.twod, gr.threed, gr.compute]);
        }
        if self.has_engine(NV2080_ENGINE_TYPE_IS_COPY) {
            classes.push(self.hardware().ce);
        }
        classes
    }

    fn first_user_chid(&self) -> i32 {
        let reserved = match self.kernel >= KernelVersion::V6_16 {
            true => R570_RSVD_CHIDS,
            false => R535_RSVD_CHIDS,
        };
        reserved + KERNEL_CHANNELS
    }

    fn zcull_info(&self, arg: *mut u8) -> i64 {
        let out = unsafe { &mut *arg.cast::<drm_nouveau_get_zcull_info>() };
        let gpu = self.hardware().gpu;
        let zcull = &gpu.zcull;
        out.width_align_pixels = zcull.widthAlignPixels;
        out.height_align_pixels = zcull.heightAlignPixels;
        out.pixel_squares_by_aliquots = zcull.pixelSquaresByAliquots;
        out.aliquot_total = zcull.aliquotTotal;
        out.zcull_region_byte_multiplier = zcull.zcullRegionByteMultiplier;
        out.zcull_region_header_size = zcull.zcullRegionHeaderSize;
        out.zcull_subregion_header_size = zcull.zcullSubregionHeaderSize;
        out.subregion_count = zcull.subregionCount;
        out.subregion_width_align_pixels = zcull.subregionWidthAlignPixels;
        out.subregion_height_align_pixels = zcull.subregionHeightAlignPixels;
        out.ctxsw_size = gpu.zcull_ctxsw_size;
        out.ctxsw_align = gpu.zcull_ctxsw_align;
        0
    }

    fn channel_alloc(&self, file: u64, arg: *mut u8) -> i64 {
        let init = unsafe { &mut *arg.cast::<drm_nouveau_channel_alloc>() };
        self.disable_uvmm_noinit(file);
        let mut runlist = Runlist::Gr;
        if self.family() >= NV_DEVICE_INFO_V0_KEPLER && init.fb_ctxdma_handle == !0 {
            runlist = match init.tt_ctxdma_handle {
                NOUVEAU_FIFO_ENGINE_GR => Runlist::Gr,
                NOUVEAU_FIFO_ENGINE_CE => Runlist::Ce,
                NOUVEAU_FIFO_ENGINE_VP | NOUVEAU_FIFO_ENGINE_PPP | NOUVEAU_FIFO_ENGINE_BSP => {
                    unimplemented!("channels on engine {:#x}", init.tt_ctxdma_handle);
                }
                _ => return errno(ENOSYS),
            };
            init.fb_ctxdma_handle = 0;
            init.tt_ctxdma_handle = 0;
        }
        let exists = match runlist {
            Runlist::Gr => self.has_engine(|e| e == NV2080_ENGINE_TYPE_GRAPHICS),
            Runlist::Ce => self.has_engine(NV2080_ENGINE_TYPE_IS_COPY),
        };
        if !exists || init.fb_ctxdma_handle == !0 || init.tt_ctxdma_handle == !0 {
            return errno(EINVAL);
        }
        let chid = {
            let mut chids = self.chids.lock();
            let chid = (self.first_user_chid()..)
                .find(|chid| !chids.contains(chid))
                .unwrap();
            chids.insert(chid);
            chid
        };
        self.with_file(file, |file| file.channels.insert(chid, runlist));
        init.channel = chid;
        init.pushbuf_domains = NOUVEAU_GEM_DOMAIN_VRAM | NOUVEAU_GEM_DOMAIN_GART;
        let notifier = Buffer {
            size: PAGE_SIZE,
            kind: 0,
            mode: 0,
            contig: true,
            valid_domains: NOUVEAU_GEM_DOMAIN_GART,
            no_share: false,
        };
        init.notifier_handle = self.buffers.file(file).create(notifier, PAGE_SIZE).handle;
        0
    }

    fn channel_free(&self, file: u64, arg: *mut u8) -> i64 {
        let req = unsafe { &*arg.cast::<drm_nouveau_channel_free>() };
        if self
            .with_file(file, |file| file.channels.remove(&req.channel))
            .is_none()
        {
            return errno(ENOENT);
        }
        self.chids.lock().remove(&req.channel);
        0
    }

    fn device_info(&self) -> nv_device_info_v0 {
        let gpu = self.hardware().gpu;
        let ram_size = (gpu.total_fb_kib as u64) << 10;
        let mut info: nv_device_info_v0 = unsafe { std::mem::zeroed() };
        info.platform = NV_DEVICE_INFO_V0_PCIE;
        info.chipset = self.chipset();
        info.revision = gpu.revision;
        info.family = self.family();
        info.ram_size = ram_size;
        info.ram_user = ram_size;
        let name = self.chip.name.as_bytes();
        for (i, &byte) in name.iter().enumerate() {
            info.chip[i] = byte as c_char;
            info.name[i] = byte as c_char;
        }
        info
    }

    fn nvif(&self, file: u64, data: &mut [u8]) -> i64 {
        let header = size_of::<nvif_ioctl_v0>();
        if data.len() < header {
            return errno(EINVAL);
        }
        let ioctl = unsafe { data.as_ptr().cast::<nvif_ioctl_v0>().read_unaligned() };
        if ioctl.version != 0 || (ioctl.route != 0 && ioctl.route != 0xff) {
            return errno(EINVAL);
        }
        let args = &mut data[header..];
        match ioctl.type_ {
            NVIF_IOCTL_V0_NEW => self.nvif_new(file, &ioctl, args),
            NVIF_IOCTL_V0_MTHD => self.nvif_mthd(file, &ioctl, args),
            NVIF_IOCTL_V0_SCLASS => self.nvif_sclass(file, &ioctl, args),
            NVIF_IOCTL_V0_DEL => self.nvif_del(file, &ioctl, args),
            type_ => unimplemented!("NVIF ioctl type {type_}"),
        }
    }

    fn nvif_new(&self, file: u64, ioctl: &nvif_ioctl_v0, args: &mut [u8]) -> i64 {
        if args.len() < size_of::<nvif_ioctl_new_v0>() {
            return errno(EINVAL);
        }
        let new = unsafe { args.as_ptr().cast::<nvif_ioctl_new_v0>().read_unaligned() };
        if new.version != 0 {
            return errno(EINVAL);
        }
        if ioctl.route != 0 {
            let classes = match self.channel(file, ioctl.token) {
                Some(runlist) => self.classes(runlist),
                None => return errno(EINVAL),
            };
            return self.with_file(file, |file| {
                if file.objects.contains_key(&new.object) {
                    return errno(EEXIST);
                }
                if !classes.contains(&(new.oclass as u32)) {
                    return errno(EINVAL);
                }
                file.objects.insert(new.object, Object::Engine);
                0
            });
        }
        if ioctl.object != 0 || new.oclass != NV_DEVICE {
            return errno(EINVAL);
        }
        self.with_file(file, |file| {
            if file.objects.contains_key(&new.object) {
                return errno(EEXIST);
            }
            file.objects.insert(new.object, Object::Device);
            0
        })
    }

    fn channel(&self, file: u64, token: u64) -> Option<Runlist> {
        self.with_file(file, |file| file.channels.get(&(token as i32)).copied())
    }

    fn nvif_sclass(&self, file: u64, ioctl: &nvif_ioctl_v0, args: &mut [u8]) -> i64 {
        let header = size_of::<nvif_ioctl_sclass_v0>();
        let entry = size_of::<nvif_ioctl_sclass_oclass_v0>();
        if ioctl.route == 0 || args.len() < header {
            return errno(EINVAL);
        }
        let count = args[1] as usize;
        if args.len() - header != count * entry {
            return errno(EINVAL);
        }
        let Some(runlist) = self.channel(file, ioctl.token) else {
            return errno(EINVAL);
        };
        let classes = self.classes(runlist);
        for (i, &class) in classes.iter().enumerate().take(count) {
            let oclass = nvif_ioctl_sclass_oclass_v0 {
                oclass: class as i32,
                minver: -1,
                maxver: 0,
            };
            unsafe {
                args[header + i * entry..]
                    .as_mut_ptr()
                    .cast::<nvif_ioctl_sclass_oclass_v0>()
                    .write_unaligned(oclass);
            }
        }
        args[1] = classes.len() as u8;
        0
    }

    fn nvif_del(&self, file: u64, ioctl: &nvif_ioctl_v0, args: &mut [u8]) -> i64 {
        if ioctl.route != 0 || !args.is_empty() {
            return errno(EINVAL);
        }
        self.with_file(file, |file| file.objects.remove(&ioctl.object));
        0
    }

    fn nvif_mthd(&self, file: u64, ioctl: &nvif_ioctl_v0, args: &mut [u8]) -> i64 {
        let header = size_of::<nvif_ioctl_mthd_v0>();
        if ioctl.route != 0 || args.len() < header {
            return errno(EINVAL);
        }
        let mthd = unsafe { args.as_ptr().cast::<nvif_ioctl_mthd_v0>().read_unaligned() };
        let object = self.with_file(file, |file| file.objects.get(&ioctl.object).copied());
        if object != Some(Object::Device) {
            return errno(EINVAL);
        }
        let data = &mut args[header..];
        if mthd.method != NV_DEVICE_V0_INFO || data.len() != size_of::<nv_device_info_v0>() {
            return errno(EINVAL);
        }
        if data[0] != 0 {
            return errno(EINVAL);
        }
        let info = self.device_info();
        unsafe {
            data.as_mut_ptr()
                .cast::<nv_device_info_v0>()
                .write_unaligned(info);
        }
        0
    }
}

impl Kernel for Device {
    fn name(&self) -> &'static str {
        "nouveau"
    }

    fn kernel_version(&self) -> KernelVersion {
        self.kernel
    }

    fn id(&self) -> String {
        format!("nouveau:chip={}:kernel={}", self.chip.name, self.kernel)
    }

    fn drm(&self) -> Option<&dyn Drm> {
        Some(self)
    }
}

impl Drm for Device {
    fn desc(&self) -> &'static str {
        "nVidia Riva/TNT/GeForce/Quadro/Tesla/Tegra K1+"
    }

    fn driver_version(&self) -> DriverVersion {
        self.kernel.nouveau()
    }

    fn uevent_driver(&self) -> &'static str {
        "nouveau"
    }

    fn pci(&self) -> Option<Pci> {
        Some(Pci {
            vendor: PCI_VENDOR_ID_NVIDIA,
            device: self.chip.pci_id,
        })
    }

    fn ioctls(&self) -> &'static [IoctlDesc] {
        ioctls::NOUVEAU
    }

    fn syncobjs(&self) -> Option<&Syncobjs> {
        Some(&self.syncobjs)
    }

    fn master(&self) -> &Master {
        &self.master
    }

    fn check_feature(&self, feature: DriverFeature) -> bool {
        match feature {
            DriverFeature::Modeset => true,
            DriverFeature::SyncobjTimeline => true,
        }
    }

    fn unlocked_ioctl(&self, file: u64, cmd: u32, arg: u64, user: &dyn UserMemory) -> Option<i64> {
        if _IOC_NR(cmd).wrapping_sub(DRM_COMMAND_BASE) != DRM_NOUVEAU_NVIF {
            return None;
        }
        let mut data = vec![0u8; _IOC_SIZE(cmd) as usize];
        if user.read(arg, &mut data).is_err() {
            return Some(errno(EFAULT));
        }
        let ret = self.nvif(file, &mut data);
        if ret == 0 && user.write(arg, &data).is_err() {
            return Some(errno(EFAULT));
        }
        Some(ret)
    }

    unsafe fn ioctl(
        &self,
        file: u64,
        func: Func,
        arg: *mut u8,
        user: &dyn UserMemory,
        _prime: &mut dyn Prime,
    ) -> Option<i64> {
        let ret = match func {
            Func::NouveauUvmmIoctlVmBind => self.vm_bind(file, arg, user),
            Func::NouveauExecIoctlExec => self.exec(file, arg, user),
            Func::NouveauAbi16IoctlGetparam => self.getparam(arg),
            Func::NouveauUvmmIoctlVmInit => self.vm_init(file, arg),
            Func::NouveauGemIoctlNew => self.gem_new(file, arg),
            Func::NouveauAbi16IoctlGetZcullInfo => self.zcull_info(arg),
            Func::NouveauAbi16IoctlChannelAlloc => self.channel_alloc(file, arg),
            Func::NouveauAbi16IoctlChannelFree => self.channel_free(file, arg),
            _ => return None,
        };
        Some(ret)
    }

    fn export(&self, file: u64, handle: u32) -> Result<Export, i32> {
        self.buffers
            .file(file)
            .export(handle, |buffer| match buffer.no_share {
                true => Err(EPERM),
                false => Ok(()),
            })
    }

    fn import(&self, file: u64, object: u32) -> Option<u32> {
        self.buffers.file(file).import(object)
    }

    fn import_foreign(&self, _file: u64, _export: &Export) -> NewHandle {
        unimplemented!("importing dma-bufs into nouveau");
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
        if let Some(file) = self.files.lock().remove(&file) {
            let mut chids = self.chids.lock();
            for chid in file.channels.keys() {
                chids.remove(chid);
            }
        }
        self.buffers.release_file(file);
        self.syncobjs.release_file(file);
    }
}
