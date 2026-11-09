use crate::DriverFeature;
use crate::Drm;
use crate::UserMemory;
use crate::handles::HandleTable;
use crate::ioctls::Func;
use crate::time::ktime_get_ns;
use crate::uapi::drm::DRM_SYNCOBJ_CREATE_SIGNALED;
use crate::uapi::drm::DRM_SYNCOBJ_WAIT_FLAGS_WAIT_ALL;
use crate::uapi::drm::DRM_SYNCOBJ_WAIT_FLAGS_WAIT_AVAILABLE;
use crate::uapi::drm::DRM_SYNCOBJ_WAIT_FLAGS_WAIT_DEADLINE;
use crate::uapi::drm::DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT;
use crate::uapi::drm::drm_syncobj_array;
use crate::uapi::drm::drm_syncobj_create;
use crate::uapi::drm::drm_syncobj_destroy;
use crate::uapi::drm::drm_syncobj_timeline_wait;
use crate::uapi::drm::drm_syncobj_wait;
use libc::EFAULT;
use libc::EINVAL;
use libc::ENOENT;
use libc::ENOMEM;
use libc::EOPNOTSUPP;
use libc::ETIME;
use parking_lot::Mutex;
use std::collections::BTreeMap;

#[derive(Copy, Clone)]
pub enum Fence {
    Plain,
    Chain { seqno: u64, prev_seqno: u64 },
}

impl Fence {
    pub fn find_seqno(self, point: u64) -> Option<Fence> {
        if point == 0 {
            return Some(self);
        }
        match self {
            Fence::Chain { seqno, prev_seqno } if seqno >= point => match prev_seqno < point {
                true => Some(self),
                false => Some(Fence::Plain),
            },
            _ => None,
        }
    }
}

#[derive(Default)]
struct SyncobjTable {
    handles: HandleTable<u64>,
    fences: BTreeMap<u64, Option<Fence>>,
    next_object: u64,
}

impl SyncobjTable {
    fn fence(&self, file: u64, handle: u32) -> Result<Option<Fence>, i32> {
        let object = self.handles.get(file, handle).ok_or(ENOENT)?;
        Ok(self.fences[&object])
    }

    fn release_if_unused(&mut self, object: u64) {
        if !self.handles.referenced(object) {
            self.fences.remove(&object);
        }
    }

    fn slot(&mut self, object: u64) -> &mut Option<Fence> {
        self.fences
            .get_mut(&object)
            .unwrap_or_else(|| panic!("syncobj {object} does not exist"))
    }
}

#[derive(Default)]
pub struct Syncobjs {
    table: Mutex<SyncobjTable>,
}

impl Syncobjs {
    fn create(&self, file: u64, fence: Option<Fence>) -> u32 {
        let mut table = self.table.lock();
        table.next_object += 1;
        let object = table.next_object;
        table.fences.insert(object, fence);
        table.handles.insert(file, object)
    }

    fn destroy(&self, file: u64, handle: u32) -> bool {
        let mut table = self.table.lock();
        let Some(object) = table.handles.remove(file, handle) else {
            return false;
        };
        table.release_if_unused(object);
        true
    }

    pub fn release_file(&self, file: u64) {
        let mut table = self.table.lock();
        for object in table.handles.remove_file(file) {
            table.release_if_unused(object);
        }
    }

    pub fn object(&self, file: u64, handle: u32) -> Option<u64> {
        self.table.lock().handles.get(file, handle)
    }

    fn fences(&self, file: u64, handles: &[u32]) -> Result<Vec<Option<Fence>>, i32> {
        let table = self.table.lock();
        handles
            .iter()
            .map(|&handle| table.fence(file, handle))
            .collect()
    }

    pub fn current(&self, file: u64, handle: u32) -> Result<Option<Fence>, i32> {
        self.table.lock().fence(file, handle)
    }

    pub fn find_fence(&self, file: u64, handle: u32, point: u64, flags: u64) -> Result<Fence, i32> {
        let fence = self.current(file, handle)?;
        if flags & !(DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT as u64) != 0 {
            return Err(EINVAL);
        }
        if let Some(fence) = fence.and_then(|fence| fence.find_seqno(point)) {
            return Ok(fence);
        }
        match flags & DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT as u64 != 0 {
            true => Err(ETIME),
            false => Err(EINVAL),
        }
    }

    pub fn replace(&self, object: u64, fence: Option<Fence>) {
        *self.table.lock().slot(object) = fence;
    }

    pub fn add_point(&self, object: u64, point: u64) {
        let mut table = self.table.lock();
        let fence = table.slot(object);
        *fence = Some(match *fence {
            Some(Fence::Chain { seqno, .. }) if point > seqno => Fence::Chain {
                seqno: point,
                prev_seqno: seqno,
            },
            Some(Fence::Chain { seqno, .. }) => Fence::Chain {
                seqno: seqno.max(point),
                prev_seqno: 0,
            },
            _ => Fence::Chain {
                seqno: point,
                prev_seqno: 0,
            },
        });
    }
}

struct Wait {
    handles: u64,
    points: Option<u64>,
    count: u32,
    flags: u32,
    timeout_nsec: i64,
}

fn array_wait(
    syncobjs: &Syncobjs,
    file: u64,
    wait: &Wait,
    user: &dyn UserMemory,
) -> Result<u32, i32> {
    let handles: Vec<u32> = user
        .read_array(wait.handles, wait.count as usize)
        .map_err(copy_error)?;
    let fences = syncobjs.fences(file, &handles)?;
    if wait.flags & DRM_SYNCOBJ_WAIT_FLAGS_WAIT_DEADLINE != 0 {
        unimplemented!("syncobj wait deadlines");
    }
    let points: Vec<u64> = match wait.points {
        Some(points) => user
            .read_array(points, wait.count as usize)
            .map_err(copy_error)?,
        None => vec![0; handles.len()],
    };
    let may_wait = DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT | DRM_SYNCOBJ_WAIT_FLAGS_WAIT_AVAILABLE;
    let mut first = u32::MAX;
    let mut signaled = 0;
    for (i, (&fence, &point)) in fences.iter().zip(&points).enumerate() {
        if fence.and_then(|fence| fence.find_seqno(point)).is_none() {
            match wait.flags & may_wait != 0 {
                true => continue,
                false => return Err(EINVAL),
            }
        }
        if signaled == 0 {
            first = i as u32;
        }
        signaled += 1;
    }
    let wait_all = wait.flags & DRM_SYNCOBJ_WAIT_FLAGS_WAIT_ALL != 0;
    if signaled == wait.count || signaled > 0 && !wait_all {
        return Ok(first);
    }
    if wait.timeout_nsec > ktime_get_ns() as i64 {
        panic!("waiting for a fence that is never submitted is not modelled");
    }
    Err(ETIME)
}

fn copy_error(errno: i32) -> i32 {
    match errno {
        ENOMEM => ENOMEM,
        _ => EFAULT,
    }
}

pub unsafe fn dispatch(
    kernel: &dyn Drm,
    file: u64,
    func: Func,
    arg: *mut u8,
    user: &dyn UserMemory,
) -> Option<i64> {
    let timeline = match func {
        Func::DrmSyncobjCreateIoctl
        | Func::DrmSyncobjDestroyIoctl
        | Func::DrmSyncobjWaitIoctl
        | Func::DrmSyncobjResetIoctl => false,
        Func::DrmSyncobjTimelineWaitIoctl => true,
        _ => return None,
    };
    let errno = |errno: i32| -(errno as i64);
    let Some(syncobjs) = kernel.syncobjs() else {
        return Some(errno(EOPNOTSUPP));
    };
    if timeline && !kernel.check_feature(DriverFeature::SyncobjTimeline) {
        return Some(errno(EOPNOTSUPP));
    }
    let ret = unsafe {
        match func {
            Func::DrmSyncobjCreateIoctl => {
                let args = &mut *arg.cast::<drm_syncobj_create>();
                if args.flags & !DRM_SYNCOBJ_CREATE_SIGNALED != 0 {
                    return Some(errno(EINVAL));
                }
                let fence = match args.flags & DRM_SYNCOBJ_CREATE_SIGNALED != 0 {
                    true => Some(Fence::Plain),
                    false => None,
                };
                args.handle = syncobjs.create(file, fence);
                0
            }
            Func::DrmSyncobjDestroyIoctl => {
                let args = &*arg.cast::<drm_syncobj_destroy>();
                if args.pad != 0 {
                    return Some(errno(EINVAL));
                }
                match syncobjs.destroy(file, args.handle) {
                    true => 0,
                    false => errno(EINVAL),
                }
            }
            Func::DrmSyncobjWaitIoctl => {
                let args = &mut *arg.cast::<drm_syncobj_wait>();
                let possible = DRM_SYNCOBJ_WAIT_FLAGS_WAIT_ALL
                    | DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT
                    | DRM_SYNCOBJ_WAIT_FLAGS_WAIT_DEADLINE;
                if args.flags & !possible != 0 {
                    return Some(errno(EINVAL));
                }
                if args.count_handles == 0 {
                    return Some(0);
                }
                let wait = Wait {
                    handles: args.handles,
                    points: None,
                    count: args.count_handles,
                    flags: args.flags,
                    timeout_nsec: args.timeout_nsec,
                };
                match array_wait(syncobjs, file, &wait, user) {
                    Ok(first) => {
                        args.first_signaled = first;
                        0
                    }
                    Err(e) => errno(e),
                }
            }
            Func::DrmSyncobjTimelineWaitIoctl => {
                let args = &mut *arg.cast::<drm_syncobj_timeline_wait>();
                let possible = DRM_SYNCOBJ_WAIT_FLAGS_WAIT_ALL
                    | DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT
                    | DRM_SYNCOBJ_WAIT_FLAGS_WAIT_AVAILABLE
                    | DRM_SYNCOBJ_WAIT_FLAGS_WAIT_DEADLINE;
                if args.flags & !possible != 0 {
                    return Some(errno(EINVAL));
                }
                if args.count_handles == 0 {
                    return Some(0);
                }
                let wait = Wait {
                    handles: args.handles,
                    points: match args.points {
                        0 => None,
                        points => Some(points),
                    },
                    count: args.count_handles,
                    flags: args.flags,
                    timeout_nsec: args.timeout_nsec,
                };
                match array_wait(syncobjs, file, &wait, user) {
                    Ok(first) => {
                        args.first_signaled = first;
                        0
                    }
                    Err(e) => errno(e),
                }
            }
            Func::DrmSyncobjResetIoctl => {
                let args = &*arg.cast::<drm_syncobj_array>();
                if args.pad != 0 || args.count_handles == 0 {
                    return Some(errno(EINVAL));
                }
                let handles: Vec<u32> =
                    match user.read_array(args.handles, args.count_handles as usize) {
                        Ok(handles) => handles,
                        Err(e) => return Some(errno(copy_error(e))),
                    };
                let objects: Option<Vec<u64>> = handles
                    .iter()
                    .map(|&handle| syncobjs.object(file, handle))
                    .collect();
                let Some(objects) = objects else {
                    return Some(errno(ENOENT));
                };
                for object in objects {
                    syncobjs.replace(object, None);
                }
                0
            }
            _ => return None,
        }
    };
    Some(ret)
}
