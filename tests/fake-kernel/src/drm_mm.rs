use std::collections::BTreeMap;
use std::ops::Range;

pub type ColorAdjust = fn(color: u64, hole: Range<u64>) -> Range<u64>;

struct Hole {
    end: u64,
    sequence: u64,
}

struct Candidate {
    hole: Range<u64>,
    sequence: u64,
}

struct Placed {
    hole: Range<u64>,
    start: u64,
}

pub struct Request {
    pub size: u64,
    pub alignment: u64,
    pub color: u64,
    pub range: Range<u64>,
}

impl Request {
    pub fn new(size: u64) -> Self {
        Self {
            size,
            alignment: 0,
            color: 0,
            range: 0..u64::MAX,
        }
    }
}

pub struct DrmMm {
    range: Range<u64>,
    nodes: BTreeMap<u64, u64>,
    holes: BTreeMap<u64, Hole>,
    color_adjust: Option<ColorAdjust>,
    next_sequence: u64,
}

impl DrmMm {
    pub fn new(start: u64, size: u64) -> Self {
        let mut mm = Self {
            range: start..start + size,
            nodes: BTreeMap::new(),
            holes: BTreeMap::new(),
            color_adjust: None,
            next_sequence: 0,
        };
        mm.add_hole(start, start + size);
        mm
    }

    pub fn with_color_adjust(mut self, color_adjust: ColorAdjust) -> Self {
        self.color_adjust = Some(color_adjust);
        self
    }

    fn add_hole(&mut self, start: u64, end: u64) {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        self.holes.insert(start, Hole { end, sequence });
    }

    fn place(&self, request: &Request, hole: Range<u64>) -> Option<u64> {
        let colored = match self.color_adjust {
            Some(adjust) => adjust(request.color, hole),
            None => hole,
        };
        let start = colored.start.max(request.range.start);
        let end = colored.end.min(request.range.end);
        if end <= start || end - start < request.size {
            return None;
        }
        if request.alignment <= 1 || start.is_multiple_of(request.alignment) {
            return Some(start);
        }
        let aligned = start.checked_next_multiple_of(request.alignment)?;
        if end <= aligned || end - aligned < request.size {
            return None;
        }
        Some(aligned)
    }

    pub fn insert(&mut self, request: &Request) -> Option<u64> {
        if request.size == 0 || request.range.end - request.range.start < request.size {
            return None;
        }
        let mut candidates: Vec<Candidate> = self
            .holes
            .iter()
            .filter(|(start, hole)| hole.end - **start >= request.size)
            .map(|(&start, hole)| Candidate {
                hole: start..hole.end,
                sequence: hole.sequence,
            })
            .collect();
        candidates.sort_by(|a, b| {
            let size = |candidate: &Candidate| candidate.hole.end - candidate.hole.start;
            size(a).cmp(&size(b)).then(b.sequence.cmp(&a.sequence))
        });
        let placed = candidates.into_iter().find_map(|candidate| {
            let start = self.place(request, candidate.hole.clone())?;
            Some(Placed {
                hole: candidate.hole,
                start,
            })
        })?;
        let Placed { hole, start } = placed;
        self.holes.remove(&hole.start);
        if start > hole.start {
            self.add_hole(hole.start, start);
        }
        if start + request.size < hole.end {
            self.add_hole(start + request.size, hole.end);
        }
        self.nodes.insert(start, start + request.size);
        Some(start)
    }

    pub fn remove(&mut self, start: u64) {
        let end = self
            .nodes
            .remove(&start)
            .unwrap_or_else(|| panic!("no drm_mm node starts at {start:#x}"));
        self.holes.remove(&end);
        let prev_end = self
            .nodes
            .range(..start)
            .next_back()
            .map_or(self.range.start, |(_, &end)| end);
        self.holes.remove(&prev_end);
        let next_start = self
            .nodes
            .range(start..)
            .next()
            .map_or(self.range.end, |(&start, _)| start);
        self.add_hole(prev_end, next_start);
    }
}
