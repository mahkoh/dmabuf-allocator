pub struct Counter {
    pub category: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

pub static V42: [Counter; 87] = [
    Counter {
        category: "FEP",
        name: "FEP-valid-primitives-no-rendered-pixels",
        description: "[FEP] Valid primitives that result in no rendered pixels, for all rendered tiles",
    },
    Counter {
        category: "FEP",
        name: "FEP-valid-primitives-rendered-pixels",
        description: "[FEP] Valid primitives for all rendered tiles (primitives may be counted in more than one tile)",
    },
    Counter {
        category: "FEP",
        name: "FEP-clipped-quads",
        description: "[FEP] Early-Z/Near/Far clipped quads",
    },
    Counter {
        category: "FEP",
        name: "FEP-valid-quads",
        description: "[FEP] Valid quads",
    },
    Counter {
        category: "TLB",
        name: "TLB-quads-not-passing-stencil-test",
        description: "[TLB] Quads with no pixels passing the stencil test",
    },
    Counter {
        category: "TLB",
        name: "TLB-quads-not-passing-z-and-stencil-test",
        description: "[TLB] Quads with no pixels passing the Z and stencil tests",
    },
    Counter {
        category: "TLB",
        name: "TLB-quads-passing-z-and-stencil-test",
        description: "[TLB] Quads with any pixels passing the Z and stencil tests",
    },
    Counter {
        category: "TLB",
        name: "TLB-quads-with-zero-coverage",
        description: "[TLB] Quads with all pixels having zero coverage",
    },
    Counter {
        category: "TLB",
        name: "TLB-quads-with-non-zero-coverage",
        description: "[TLB] Quads with any pixels having non-zero coverage",
    },
    Counter {
        category: "TLB",
        name: "TLB-quads-written-to-color-buffer",
        description: "[TLB] Quads with valid pixels written to colour buffer",
    },
    Counter {
        category: "PTB",
        name: "PTB-primitives-discarded-outside-viewport",
        description: "[PTB] Primitives discarded by being outside the viewport",
    },
    Counter {
        category: "PTB",
        name: "PTB-primitives-need-clipping",
        description: "[PTB] Primitives that need clipping",
    },
    Counter {
        category: "PTB",
        name: "PTB-primitives-discarded-reversed",
        description: "[PTB] Primitives that are discarded because they are reversed",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-idle-clk-cycles",
        description: "[QPU] Total idle clock cycles for all QPUs",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-active-clk-cycles-vertex-coord-shading",
        description: "[QPU] Total active clock cycles for all QPUs doing vertex/coordinate/user shading (counts only when QPU is not stalled)",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-active-clk-cycles-fragment-shading",
        description: "[QPU] Total active clock cycles for all QPUs doing fragment shading (counts only when QPU is not stalled)",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-clk-cycles-executing-valid-instr",
        description: "[QPU] Total clock cycles for all QPUs executing valid instructions",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-clk-cycles-waiting-TMU",
        description: "[QPU] Total clock cycles for all QPUs stalled waiting for TMUs only (counter won't increment if QPU also stalling for another reason)",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-clk-cycles-waiting-scoreboard",
        description: "[QPU] Total clock cycles for all QPUs stalled waiting for Scoreboard only (counter won't increment if QPU also stalling for another reason)",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-clk-cycles-waiting-varyings",
        description: "[QPU] Total clock cycles for all QPUs stalled waiting for Varyings only (counter won't increment if QPU also stalling for another reason)",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-instr-cache-hit",
        description: "[QPU] Total instruction cache hits for all slices",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-instr-cache-miss",
        description: "[QPU] Total instruction cache misses for all slices",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-uniform-cache-hit",
        description: "[QPU] Total uniforms cache hits for all slices",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-uniform-cache-miss",
        description: "[QPU] Total uniforms cache misses for all slices",
    },
    Counter {
        category: "TMU",
        name: "TMU-total-text-quads-access",
        description: "[TMU] Total texture cache accesses",
    },
    Counter {
        category: "TMU",
        name: "TMU-total-text-cache-miss",
        description: "[TMU] Total texture cache misses (number of fetches from memory/L2cache)",
    },
    Counter {
        category: "VPM",
        name: "VPM-total-clk-cycles-VDW-stalled",
        description: "[VPM] Total clock cycles VDW is stalled waiting for VPM access",
    },
    Counter {
        category: "VPM",
        name: "VPM-total-clk-cycles-VCD-stalled",
        description: "[VPM] Total clock cycles VCD is stalled waiting for VPM access",
    },
    Counter {
        category: "CLE",
        name: "CLE-bin-thread-active-cycles",
        description: "[CLE] Bin thread active cycles",
    },
    Counter {
        category: "CLE",
        name: "CLE-render-thread-active-cycles",
        description: "[CLE] Render thread active cycles",
    },
    Counter {
        category: "L2T",
        name: "L2T-total-cache-hit",
        description: "[L2T] Total Level 2 cache hits",
    },
    Counter {
        category: "L2T",
        name: "L2T-total-cache-miss",
        description: "[L2T] Total Level 2 cache misses",
    },
    Counter {
        category: "CORE",
        name: "cycle-count",
        description: "[CORE] Cycle counter",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-clk-cycles-waiting-vertex-coord-shading",
        description: "[QPU] Total stalled clock cycles for all QPUs doing vertex/coordinate/user shading",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-clk-cycles-waiting-fragment-shading",
        description: "[QPU] Total stalled clock cycles for all QPUs doing fragment shading",
    },
    Counter {
        category: "PTB",
        name: "PTB-primitives-binned",
        description: "[PTB] Total primitives binned",
    },
    Counter {
        category: "AXI",
        name: "AXI-writes-seen-watch-0",
        description: "[AXI] Writes seen by watch 0",
    },
    Counter {
        category: "AXI",
        name: "AXI-reads-seen-watch-0",
        description: "[AXI] Reads seen by watch 0",
    },
    Counter {
        category: "AXI",
        name: "AXI-writes-stalled-seen-watch-0",
        description: "[AXI] Write stalls seen by watch 0",
    },
    Counter {
        category: "AXI",
        name: "AXI-reads-stalled-seen-watch-0",
        description: "[AXI] Read stalls seen by watch 0",
    },
    Counter {
        category: "AXI",
        name: "AXI-write-bytes-seen-watch-0",
        description: "[AXI] Total bytes written seen by watch 0",
    },
    Counter {
        category: "AXI",
        name: "AXI-read-bytes-seen-watch-0",
        description: "[AXI] Total bytes read seen by watch 0",
    },
    Counter {
        category: "AXI",
        name: "AXI-writes-seen-watch-1",
        description: "[AXI] Writes seen by watch 1",
    },
    Counter {
        category: "AXI",
        name: "AXI-reads-seen-watch-1",
        description: "[AXI] Reads seen by watch 1",
    },
    Counter {
        category: "AXI",
        name: "AXI-writes-stalled-seen-watch-1",
        description: "[AXI] Write stalls seen by watch 1",
    },
    Counter {
        category: "AXI",
        name: "AXI-reads-stalled-seen-watch-1",
        description: "[AXI] Read stalls seen by watch 1",
    },
    Counter {
        category: "AXI",
        name: "AXI-write-bytes-seen-watch-1",
        description: "[AXI] Total bytes written seen by watch 1",
    },
    Counter {
        category: "AXI",
        name: "AXI-read-bytes-seen-watch-1",
        description: "[AXI] Total bytes read seen by watch 1",
    },
    Counter {
        category: "TLB",
        name: "TLB-partial-quads-written-to-color-buffer",
        description: "[TLB] Partial quads written to the colour buffer",
    },
    Counter {
        category: "TMU",
        name: "TMU-total-config-access",
        description: "[TMU] Total config accesses",
    },
    Counter {
        category: "L2T",
        name: "L2T-no-id-stalled",
        description: "[L2T] No ID stall",
    },
    Counter {
        category: "L2T",
        name: "L2T-command-queue-stalled",
        description: "[L2T] Command queue full stall",
    },
    Counter {
        category: "L2T",
        name: "L2T-TMU-writes",
        description: "[L2T] TMU write accesses",
    },
    Counter {
        category: "TMU",
        name: "TMU-active-cycles",
        description: "[TMU] Active cycles",
    },
    Counter {
        category: "TMU",
        name: "TMU-stalled-cycles",
        description: "[TMU] Stalled cycles",
    },
    Counter {
        category: "CLE",
        name: "CLE-thread-active-cycles",
        description: "[CLE] Bin or render thread active cycles",
    },
    Counter {
        category: "L2T",
        name: "L2T-TMU-reads",
        description: "[L2T] TMU read accesses",
    },
    Counter {
        category: "L2T",
        name: "L2T-CLE-reads",
        description: "[L2T] CLE read accesses",
    },
    Counter {
        category: "L2T",
        name: "L2T-VCD-reads",
        description: "[L2T] VCD read accesses",
    },
    Counter {
        category: "L2T",
        name: "L2T-TMU-config-reads",
        description: "[L2T] TMU CFG read accesses",
    },
    Counter {
        category: "L2T",
        name: "L2T-SLC0-reads",
        description: "[L2T] SLC0 read accesses",
    },
    Counter {
        category: "L2T",
        name: "L2T-SLC1-reads",
        description: "[L2T] SLC1 read accesses",
    },
    Counter {
        category: "L2T",
        name: "L2T-SLC2-reads",
        description: "[L2T] SLC2 read accesses",
    },
    Counter {
        category: "L2T",
        name: "L2T-TMU-write-miss",
        description: "[L2T] TMU write misses",
    },
    Counter {
        category: "L2T",
        name: "L2T-TMU-read-miss",
        description: "[L2T] TMU read misses",
    },
    Counter {
        category: "L2T",
        name: "L2T-CLE-read-miss",
        description: "[L2T] CLE read misses",
    },
    Counter {
        category: "L2T",
        name: "L2T-VCD-read-miss",
        description: "[L2T] VCD read misses",
    },
    Counter {
        category: "L2T",
        name: "L2T-TMU-config-read-miss",
        description: "[L2T] TMU CFG read misses",
    },
    Counter {
        category: "L2T",
        name: "L2T-SLC0-read-miss",
        description: "[L2T] SLC0 read misses",
    },
    Counter {
        category: "L2T",
        name: "L2T-SLC1-read-miss",
        description: "[L2T] SLC1 read misses",
    },
    Counter {
        category: "L2T",
        name: "L2T-SLC2-read-miss",
        description: "[L2T] SLC2 read misses",
    },
    Counter {
        category: "CORE",
        name: "core-memory-writes",
        description: "[CORE] Total memory writes",
    },
    Counter {
        category: "L2T",
        name: "L2T-memory-writes",
        description: "[L2T] Total memory writes",
    },
    Counter {
        category: "PTB",
        name: "PTB-memory-writes",
        description: "[PTB] Total memory writes",
    },
    Counter {
        category: "TLB",
        name: "TLB-memory-writes",
        description: "[TLB] Total memory writes",
    },
    Counter {
        category: "CORE",
        name: "core-memory-reads",
        description: "[CORE] Total memory reads",
    },
    Counter {
        category: "L2T",
        name: "L2T-memory-reads",
        description: "[L2T] Total memory reads",
    },
    Counter {
        category: "PTB",
        name: "PTB-memory-reads",
        description: "[PTB] Total memory reads",
    },
    Counter {
        category: "PSE",
        name: "PSE-memory-reads",
        description: "[PSE] Total memory reads",
    },
    Counter {
        category: "TLB",
        name: "TLB-memory-reads",
        description: "[TLB] Total memory reads",
    },
    Counter {
        category: "GMP",
        name: "GMP-memory-reads",
        description: "[GMP] Total memory reads",
    },
    Counter {
        category: "PTB",
        name: "PTB-memory-words-writes",
        description: "[PTB] Total memory words written",
    },
    Counter {
        category: "TLB",
        name: "TLB-memory-words-writes",
        description: "[TLB] Total memory words written",
    },
    Counter {
        category: "PSE",
        name: "PSE-memory-words-reads",
        description: "[PSE] Total memory words read",
    },
    Counter {
        category: "TLB",
        name: "TLB-memory-words-reads",
        description: "[TLB] Total memory words read",
    },
    Counter {
        category: "TMU",
        name: "TMU-MRU-hits",
        description: "[TMU] Total MRU hits",
    },
    Counter {
        category: "CORE",
        name: "compute-active-cycles",
        description: "[CORE] Compute active cycles",
    },
];

pub static V71: [Counter; 93] = [
    Counter {
        category: "CORE",
        name: "cycle-count",
        description: "[CORE] Cycle counter",
    },
    Counter {
        category: "CORE",
        name: "core-active",
        description: "[CORE] Bin/Render/Compute active cycles",
    },
    Counter {
        category: "CLE",
        name: "CLE-bin-thread-active-cycles",
        description: "[CLE] Bin thread active cycles",
    },
    Counter {
        category: "CLE",
        name: "CLE-render-thread-active-cycles",
        description: "[CLE] Render thread active cycles",
    },
    Counter {
        category: "CORE",
        name: "compute-active-cycles",
        description: "[CORE] Compute active cycles",
    },
    Counter {
        category: "FEP",
        name: "FEP-valid-primitives-no-rendered-pixels",
        description: "[FEP] Valid primitives that result in no rendered pixels, for all rendered tiles",
    },
    Counter {
        category: "FEP",
        name: "FEP-valid-primitives-rendered-pixels",
        description: "[FEP] Valid primitives for all rendered tiles (primitives may be counted in more than one tile)",
    },
    Counter {
        category: "FEP",
        name: "FEP-clipped-quads",
        description: "[FEP] Early-Z/Near/Far clipped quads",
    },
    Counter {
        category: "FEP",
        name: "FEP-valid-quads",
        description: "[FEP] Valid quads",
    },
    Counter {
        category: "TLB",
        name: "TLB-quads-not-passing-stencil-test",
        description: "[TLB] Quads with no pixels passing the stencil test",
    },
    Counter {
        category: "TLB",
        name: "TLB-quads-not-passing-z-and-stencil-test",
        description: "[TLB] Quads with no pixels passing the Z and stencil tests",
    },
    Counter {
        category: "TLB",
        name: "TLB-quads-passing-z-and-stencil-test",
        description: "[TLB] Quads with any pixels passing the Z and stencil tests",
    },
    Counter {
        category: "TLB",
        name: "TLB-quads-written-to-color-buffer",
        description: "[TLB] Quads with valid pixels written to colour buffer",
    },
    Counter {
        category: "TLB",
        name: "TLB-partial-quads-written-to-color-buffer",
        description: "[TLB] Partial quads written to the colour buffer",
    },
    Counter {
        category: "PTB",
        name: "PTB-primitives-need-clipping",
        description: "[PTB] Primitives that need clipping",
    },
    Counter {
        category: "PTB",
        name: "PTB-primitives-discarded-outside-viewport",
        description: "[PTB] Primitives discarded by being outside the viewport",
    },
    Counter {
        category: "PTB",
        name: "PTB-primitives-binned",
        description: "[PTB] Total primitives binned",
    },
    Counter {
        category: "PTB",
        name: "PTB-primitives-discarded-reversed",
        description: "[PTB] Primitives that are discarded because they are reversed",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-instr-cache-hit",
        description: "[QPU] Total instruction cache hits for all slices",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-instr-cache-miss",
        description: "[QPU] Total instruction cache misses for all slices",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-uniform-cache-hit",
        description: "[QPU] Total uniforms cache hits for all slices",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-uniform-cache-miss",
        description: "[QPU] Total uniforms cache misses for all slices",
    },
    Counter {
        category: "TMU",
        name: "TMU-active-cycles",
        description: "[TMU] Active cycles",
    },
    Counter {
        category: "TMU",
        name: "TMU-stalled-cycles",
        description: "[TMU] Stalled cycles",
    },
    Counter {
        category: "TMU",
        name: "TMU-total-text-quads-access",
        description: "[TMU] Total texture cache accesses",
    },
    Counter {
        category: "TMU",
        name: "TMU-cache-x4-active-cycles",
        description: "[TMU] Cache active cycles for x4 access",
    },
    Counter {
        category: "TMU",
        name: "TMU-cache-x4-stalled-cycles",
        description: "[TMU] Cache stalled cycles for x4 access",
    },
    Counter {
        category: "TMU",
        name: "TMU-total-text-quads-x4-access",
        description: "[TMU] Total texture cache x4 access",
    },
    Counter {
        category: "L2T",
        name: "L2T-total-cache-hit",
        description: "[L2T] Total Level 2 cache hits",
    },
    Counter {
        category: "L2T",
        name: "L2T-total-cache-miss",
        description: "[L2T] Total Level 2 cache misses",
    },
    Counter {
        category: "L2T",
        name: "L2T-local",
        description: "[L2T] Local mode access",
    },
    Counter {
        category: "L2T",
        name: "L2T-writeback",
        description: "[L2T] Writeback",
    },
    Counter {
        category: "L2T",
        name: "L2T-zero",
        description: "[L2T] Zero",
    },
    Counter {
        category: "L2T",
        name: "L2T-merge",
        description: "[L2T] Merge",
    },
    Counter {
        category: "L2T",
        name: "L2T-fill",
        description: "[L2T] Fill",
    },
    Counter {
        category: "L2T",
        name: "L2T-stalls-no-wid",
        description: "[L2T] Stalls because no WID available",
    },
    Counter {
        category: "L2T",
        name: "L2T-stalls-no-rid",
        description: "[L2T] Stalls because no RID available",
    },
    Counter {
        category: "L2T",
        name: "L2T-stalls-queue-full",
        description: "[L2T] Stalls because internal queue full",
    },
    Counter {
        category: "L2T",
        name: "L2T-stalls-wrightback",
        description: "[L2T] Stalls because writeback in flight",
    },
    Counter {
        category: "L2T",
        name: "L2T-stalls-mem",
        description: "[L2T] Stalls because AXI blocks read",
    },
    Counter {
        category: "L2T",
        name: "L2T-stalls-fill",
        description: "[L2T] Stalls because fill pending for victim cache-line",
    },
    Counter {
        category: "L2T",
        name: "L2T-hitq",
        description: "[L2T] Sent request via hit queue",
    },
    Counter {
        category: "L2T",
        name: "L2T-hitq-full",
        description: "[L2T] Sent request via main queue because hit queue is full",
    },
    Counter {
        category: "L2T",
        name: "L2T-stalls-read-data",
        description: "[L2T] Stalls because waiting for data from SDRAM",
    },
    Counter {
        category: "L2T",
        name: "L2T-TMU-read-hits",
        description: "[L2T] TMU read hits",
    },
    Counter {
        category: "L2T",
        name: "L2T-TMU-read-miss",
        description: "[L2T] TMU read misses",
    },
    Counter {
        category: "L2T",
        name: "L2T-VCD-read-hits",
        description: "[L2T] VCD read hits",
    },
    Counter {
        category: "L2T",
        name: "L2T-VCD-read-miss",
        description: "[L2T] VCD read misses",
    },
    Counter {
        category: "L2T",
        name: "L2T-SLC-read-hits",
        description: "[L2T] SLC read hits (all slices)",
    },
    Counter {
        category: "L2T",
        name: "L2T-SLC-read-miss",
        description: "[L2T] SLC read misses (all slices)",
    },
    Counter {
        category: "AXI",
        name: "AXI-writes-seen-watch-0",
        description: "[AXI] Writes seen by watch 0",
    },
    Counter {
        category: "AXI",
        name: "AXI-reads-seen-watch-0",
        description: "[AXI] Reads seen by watch 0",
    },
    Counter {
        category: "AXI",
        name: "AXI-writes-stalled-seen-watch-0",
        description: "[AXI] Write stalls seen by watch 0",
    },
    Counter {
        category: "AXI",
        name: "AXI-reads-stalled-seen-watch-0",
        description: "[AXI] Read stalls seen by watch 0",
    },
    Counter {
        category: "AXI",
        name: "AXI-write-bytes-seen-watch-0",
        description: "[AXI] Total bytes written seen by watch 0",
    },
    Counter {
        category: "AXI",
        name: "AXI-read-bytes-seen-watch-0",
        description: "[AXI] Total bytes read seen by watch 0",
    },
    Counter {
        category: "AXI",
        name: "AXI-writes-seen-watch-1",
        description: "[AXI] Writes seen by watch 1",
    },
    Counter {
        category: "AXI",
        name: "AXI-reads-seen-watch-1",
        description: "[AXI] Reads seen by watch 1",
    },
    Counter {
        category: "AXI",
        name: "AXI-writes-stalled-seen-watch-1",
        description: "[AXI] Write stalls seen by watch 1",
    },
    Counter {
        category: "AXI",
        name: "AXI-reads-stalled-seen-watch-1",
        description: "[AXI] Read stalls seen by watch 1",
    },
    Counter {
        category: "AXI",
        name: "AXI-write-bytes-seen-watch-1",
        description: "[AXI] Total bytes written seen by watch 1",
    },
    Counter {
        category: "AXI",
        name: "AXI-read-bytes-seen-watch-1",
        description: "[AXI] Total bytes read seen by watch 1",
    },
    Counter {
        category: "CORE",
        name: "core-memory-writes",
        description: "[CORE] Total memory writes",
    },
    Counter {
        category: "L2T",
        name: "L2T-memory-writes",
        description: "[L2T] Total memory writes",
    },
    Counter {
        category: "PTB",
        name: "PTB-memory-writes",
        description: "[PTB] Total memory writes",
    },
    Counter {
        category: "TLB",
        name: "TLB-memory-writes",
        description: "[TLB] Total memory writes",
    },
    Counter {
        category: "CORE",
        name: "core-memory-reads",
        description: "[CORE] Total memory reads",
    },
    Counter {
        category: "L2T",
        name: "L2T-memory-reads",
        description: "[L2T] Total memory reads",
    },
    Counter {
        category: "PTB",
        name: "PTB-memory-reads",
        description: "[PTB] Total memory reads",
    },
    Counter {
        category: "PSE",
        name: "PSE-memory-reads",
        description: "[PSE] Total memory reads",
    },
    Counter {
        category: "TLB",
        name: "TLB-memory-reads",
        description: "[TLB] Total memory reads",
    },
    Counter {
        category: "PTB",
        name: "PTB-memory-words-writes",
        description: "[PTB] Total memory words written",
    },
    Counter {
        category: "TLB",
        name: "TLB-memory-words-writes",
        description: "[TLB] Total memory words written",
    },
    Counter {
        category: "PSE",
        name: "PSE-memory-words-reads",
        description: "[PSE] Total memory words read",
    },
    Counter {
        category: "TLB",
        name: "TLB-memory-words-reads",
        description: "[TLB] Total memory words read",
    },
    Counter {
        category: "AXI",
        name: "AXI-read-trans",
        description: "[AXI] Read transaction count",
    },
    Counter {
        category: "AXI",
        name: "AXI-write-trans",
        description: "[AXI] Write transaction count",
    },
    Counter {
        category: "AXI",
        name: "AXI-read-wait-cycles",
        description: "[AXI] Read total wait cycles",
    },
    Counter {
        category: "AXI",
        name: "AXI-write-wait-cycles",
        description: "[AXI] Write total wait cycles",
    },
    Counter {
        category: "AXI",
        name: "AXI-max-outstanding-reads",
        description: "[AXI] Maximum outstanding read transactions",
    },
    Counter {
        category: "AXI",
        name: "AXI-max-outstanding-writes",
        description: "[AXI] Maximum outstanding write transactions",
    },
    Counter {
        category: "QPU",
        name: "QPU-wait-bubble",
        description: "[QPU] Pipeline bubble in qcycles due all threads waiting",
    },
    Counter {
        category: "QPU",
        name: "QPU-ic-miss-bubble",
        description: "[QPU] Pipeline bubble in qcycles due instruction-cache miss",
    },
    Counter {
        category: "QPU",
        name: "QPU-active",
        description: "[QPU] Executed shader instruction",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-active-clk-cycles-fragment-shading",
        description: "[QPU] Total active clock cycles for all QPUs doing fragment shading (counts only when QPU is not stalled)",
    },
    Counter {
        category: "QPU",
        name: "QPU-stalls",
        description: "[QPU] Stalled qcycles executing shader instruction",
    },
    Counter {
        category: "QPU",
        name: "QPU-total-clk-cycles-waiting-fragment-shading",
        description: "[QPU] Total stalled clock cycles for all QPUs doing fragment shading",
    },
    Counter {
        category: "QPU",
        name: "QPU-stalls-TMU",
        description: "[QPU] Stalled qcycles waiting for TMU",
    },
    Counter {
        category: "QPU",
        name: "QPU-stalls-TLB",
        description: "[QPU] Stalled qcycles waiting for TLB",
    },
    Counter {
        category: "QPU",
        name: "QPU-stalls-VPM",
        description: "[QPU] Stalled qcycles waiting for VPM",
    },
    Counter {
        category: "QPU",
        name: "QPU-stalls-uniforms",
        description: "[QPU] Stalled qcycles waiting for uniforms",
    },
    Counter {
        category: "QPU",
        name: "QPU-stalls-SFU",
        description: "[QPU] Stalled qcycles waiting for SFU",
    },
    Counter {
        category: "QPU",
        name: "QPU-stalls-other",
        description: "[QPU] Stalled qcycles waiting for any other reason (vary/W/Z)",
    },
];
