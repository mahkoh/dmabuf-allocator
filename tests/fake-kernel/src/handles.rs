use std::collections::BTreeMap;

pub(crate) fn lowest_free(first: u32, sorted: impl Iterator<Item = u32>) -> Option<u32> {
    let mut candidate = first;
    for id in sorted {
        if id < candidate {
            continue;
        }
        if id > candidate {
            break;
        }
        candidate = candidate.checked_add(1)?;
    }
    Some(candidate)
}

pub(crate) struct HandleTable<O> {
    files: BTreeMap<u64, BTreeMap<u32, O>>,
}

impl<O> Default for HandleTable<O> {
    fn default() -> Self {
        Self {
            files: BTreeMap::new(),
        }
    }
}

impl<O: Copy + PartialEq> HandleTable<O> {
    pub(crate) fn get(&self, file: u64, handle: u32) -> Option<O> {
        self.files.get(&file)?.get(&handle).copied()
    }

    pub(crate) fn insert(&mut self, file: u64, object: O) -> u32 {
        let handles = self.files.entry(file).or_default();
        let handle = lowest_free(1, handles.keys().copied()).expect("handles exhausted");
        handles.insert(handle, object);
        handle
    }

    pub(crate) fn remove(&mut self, file: u64, handle: u32) -> Option<O> {
        self.files.get_mut(&file)?.remove(&handle)
    }

    pub(crate) fn remove_file(&mut self, file: u64) -> Vec<O> {
        match self.files.remove(&file) {
            Some(handles) => handles.into_values().collect(),
            None => Vec::new(),
        }
    }

    pub(crate) fn referenced(&self, object: O) -> bool {
        self.files
            .values()
            .any(|handles| handles.values().any(|&o| o == object))
    }

    pub(crate) fn find(&self, file: u64, f: impl Fn(O) -> bool) -> Option<u32> {
        self.files
            .get(&file)?
            .iter()
            .find(|&(_, &object)| f(object))
            .map(|(&handle, _)| handle)
    }
}
