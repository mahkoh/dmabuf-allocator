use crate::FILE_SIZE;
use crate::PAGE_SHIFT;
use crate::PAGE_SIZE;
use crate::drm_mm::DrmMm;
use crate::drm_mm::Request;
use crate::handles::HandleTable;
use parking_lot::Mutex;
use std::any::Any;
use std::collections::BTreeMap;
use std::sync::Arc;

const DRM_FILE_PAGE_OFFSET_START: u64 = (0xFFFFFFFF >> PAGE_SHIFT) + 1;
const DRM_FILE_PAGE_OFFSET_SIZE: u64 = (0xFFFFFFFF >> PAGE_SHIFT) * 256;

const _: () = assert!(
    (DRM_FILE_PAGE_OFFSET_START + DRM_FILE_PAGE_OFFSET_SIZE) << PAGE_SHIFT <= FILE_SIZE as u64
);

#[derive(Clone)]
pub struct Export {
    pub object: u32,
    pub size: u64,
    pub exporter_data: Option<Arc<dyn Any + Send + Sync>>,
}

pub struct NewHandle {
    pub handle: u32,
    pub object: u32,
}

struct Object<T> {
    data: T,
    exported: bool,
    size: u64,
    offset: Option<u64>,
}

struct BufferTable<T> {
    objects: BTreeMap<u32, Object<T>>,
    handles: HandleTable<u32>,
    vma_offsets: DrmMm,
    next_object: u32,
}

impl<T> BufferTable<T> {
    fn create(&mut self, file: u64, data: T, size: u64) -> NewHandle {
        let object = self.next_object;
        self.next_object = object.checked_add(1).expect("object ids exhausted");
        self.objects.insert(
            object,
            Object {
                data,
                exported: false,
                size,
                offset: None,
            },
        );
        let handle = self.handles.insert(file, object);
        NewHandle { handle, object }
    }

    fn offset(&mut self, object: u32) -> u64 {
        let entry = self.objects.get_mut(&object).unwrap();
        if let Some(offset) = entry.offset {
            return offset;
        }
        let pages = entry.size.div_ceil(PAGE_SIZE);
        assert_ne!(pages, 0, "zero-sized objects have no mmap offset");
        let page = self
            .vma_offsets
            .insert(&Request::new(pages))
            .unwrap_or_else(|| panic!("the fake device file has no room for {pages} pages"));
        let offset = page << PAGE_SHIFT;
        entry.offset = Some(offset);
        offset
    }

    fn free_if_unused(&mut self, object: u32) {
        let Some(entry) = self.objects.get(&object) else {
            return;
        };
        if entry.exported || self.handles.referenced(object) {
            return;
        }
        if let Some(offset) = entry.offset {
            self.vma_offsets.remove(offset >> PAGE_SHIFT);
        }
        self.objects.remove(&object);
    }
}

pub struct Buffers<T> {
    table: Mutex<BufferTable<T>>,
}

impl<T> Default for Buffers<T> {
    fn default() -> Self {
        Self {
            table: Mutex::new(BufferTable {
                objects: BTreeMap::new(),
                handles: HandleTable::default(),
                vma_offsets: DrmMm::new(DRM_FILE_PAGE_OFFSET_START, DRM_FILE_PAGE_OFFSET_SIZE),
                next_object: 1,
            }),
        }
    }
}

impl<T> Buffers<T> {
    pub fn file(&self, file: u64) -> FileBuffers<'_, T> {
        FileBuffers {
            buffers: self,
            file,
        }
    }

    pub fn sum(&self, f: impl Fn(&T) -> u64) -> u64 {
        self.table
            .lock()
            .objects
            .values()
            .map(|object| f(&object.data))
            .sum()
    }

    pub fn for_each(&self, mut f: impl FnMut(&mut T)) {
        for object in self.table.lock().objects.values_mut() {
            f(&mut object.data);
        }
    }

    pub fn has_handles(&self, object: u32) -> bool {
        self.table.lock().handles.referenced(object)
    }

    pub fn release_export(&self, object: u32) {
        let mut table = self.table.lock();
        if let Some(entry) = table.objects.get_mut(&object) {
            entry.exported = false;
        }
        table.free_if_unused(object);
    }

    pub fn release_file(&self, file: u64) {
        let mut table = self.table.lock();
        for object in table.handles.remove_file(file) {
            table.free_if_unused(object);
        }
    }
}

pub struct FileBuffers<'a, T> {
    buffers: &'a Buffers<T>,
    file: u64,
}

impl<T> FileBuffers<'_, T> {
    pub fn create(&self, data: T, size: u64) -> NewHandle {
        self.buffers.table.lock().create(self.file, data, size)
    }

    pub fn create_with_offset(&self, data: T, size: u64) -> NewHandle {
        let mut table = self.buffers.table.lock();
        let created = table.create(self.file, data, size);
        table.offset(created.object);
        created
    }

    pub fn with<R>(&self, handle: u32, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        let mut table = self.buffers.table.lock();
        let object = table.handles.get(self.file, handle)?;
        Some(f(&mut table.objects.get_mut(&object).unwrap().data))
    }

    pub fn contains(&self, handle: u32) -> bool {
        self.object(handle).is_some()
    }

    pub fn object(&self, handle: u32) -> Option<u32> {
        self.buffers.table.lock().handles.get(self.file, handle)
    }

    pub fn size(&self, handle: u32) -> Option<u64> {
        let table = self.buffers.table.lock();
        let object = table.handles.get(self.file, handle)?;
        Some(table.objects[&object].size)
    }

    pub fn find(&self, f: impl Fn(&T) -> bool) -> Option<u32> {
        let table = self.buffers.table.lock();
        table
            .handles
            .find(self.file, |object| f(&table.objects[&object].data))
    }

    pub fn offset(&self, handle: u32) -> Option<u64> {
        let mut table = self.buffers.table.lock();
        let object = table.handles.get(self.file, handle)?;
        Some(table.offset(object))
    }

    pub fn export(
        &self,
        handle: u32,
        check: impl FnOnce(&T) -> Result<(), i32>,
    ) -> Result<Export, i32> {
        let mut table = self.buffers.table.lock();
        let object = table.handles.get(self.file, handle).ok_or(libc::ENOENT)?;
        let entry = table.objects.get_mut(&object).unwrap();
        check(&entry.data)?;
        entry.exported = true;
        Ok(Export {
            object,
            size: entry.size,
            exporter_data: None,
        })
    }

    pub fn import(&self, object: u32) -> Option<u32> {
        let mut table = self.buffers.table.lock();
        if !table.objects.contains_key(&object) {
            return None;
        }
        let existing = table.handles.find(self.file, |o| o == object);
        Some(existing.unwrap_or_else(|| table.handles.insert(self.file, object)))
    }

    pub fn close(&self, handle: u32) -> bool {
        let mut table = self.buffers.table.lock();
        let Some(object) = table.handles.remove(self.file, handle) else {
            return false;
        };
        table.free_if_unused(object);
        true
    }
}
