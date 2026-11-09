use crate::allocator::Config;
use crate::allocator::DeviceLayout;
use crate::allocator::Extra;
use crate::drivers::amd::amdgpu::Bo;
use crate::drivers::amd::amdgpu::Device;
use crate::drivers::amd::amdgpu::GFX9;
use crate::drivers::amd::amdgpu::GFX10;
use crate::drivers::amd::amdgpu::GFX11_5;
use crate::drivers::amd::amdgpu::GFX12;
use crate::drivers::amd::amdgpu::GfxVersion;
use crate::drivers::amd::gfx6;
use crate::drivers::amd::gfx6::GFX6_1D_TILED_THIN1;
use crate::drivers::amd::gfx6::GFX6_2D_TILED_THIN1;
use crate::drivers::amd::gfx6::GFX6_CHIP_FIELDS;
use crate::drivers::amd::gfx6::THIN;
use crate::drivers::amd::uapi::drm_amdgpu_gem_metadata__data;
use crate::format::Fmt;
use crate::kernel::Kernel;
use crate::layout::DRM_FORMAT_MOD_LINEAR;
use crate::layout::DRM_FORMAT_MOD_VENDOR_AMD;
use crate::layout::Field;
use crate::layout::Layout;
use crate::layout::PlaneLayout;
use crate::layout::PlaneVec;
use crate::layout::VENDOR;
use crate::layout::align;
use crate::layout::blocks;
use crate::layout::end;
use crate::layout::field;

pub(crate) const TILE_VERSION: Field = Field { shift: 0, bits: 8 };
pub(crate) const TILE: Field = Field { shift: 8, bits: 5 };
const DCC: Field = Field { shift: 13, bits: 1 };
const DCC_RETILE: Field = Field { shift: 14, bits: 1 };
const DCC_PIPE_ALIGN: Field = Field { shift: 15, bits: 1 };
const DCC_INDEPENDENT_64B: Field = Field { shift: 16, bits: 1 };
const DCC_INDEPENDENT_128B: Field = Field { shift: 17, bits: 1 };
const DCC_MAX_COMPRESSED_BLOCK: Field = Field { shift: 18, bits: 2 };
const PIPE_XOR_BITS: Field = Field { shift: 21, bits: 3 };
const PACKERS: Field = Field { shift: 27, bits: 3 };
const RB: Field = Field { shift: 30, bits: 3 };
const BANK_XOR_BITS: Field = Field { shift: 24, bits: 3 };
const PIPE: Field = Field { shift: 33, bits: 3 };
const DCC_CONSTANT_ENCODE: Field = Field { shift: 20, bits: 1 };

pub(crate) const VER_GFX6: u64 = 0;
const VER_GFX9: u64 = 1;
const VER_GFX10: u64 = 2;
const VER_GFX10_RBPLUS: u64 = 3;
const VER_GFX11: u64 = 4;
const VER_GFX12: u64 = 5;

const GFX9_64K_S: u64 = 9;
const GFX9_64K_D: u64 = 10;
const GFX9_4K_D_X: u64 = 22;
const GFX9_64K_S_X: u64 = 25;
const GFX9_64K_D_X: u64 = 26;
const GFX9_64K_R_X: u64 = 27;
const GFX11_256K_R_X: u64 = 31;
const GFX12_256B_2D: u64 = 1;
const GFX12_4K_2D: u64 = 2;
const GFX12_64K_2D: u64 = 3;
const GFX12_256K_2D: u64 = 4;

const DCC_BLOCK_128B: u64 = 1;
const DCC_BLOCK_256B: u64 = 2;

const CHIP_FIELDS: u64 = mask(DCC_CONSTANT_ENCODE)
    | mask(PIPE_XOR_BITS)
    | mask(BANK_XOR_BITS)
    | mask(PACKERS)
    | mask(RB)
    | mask(PIPE);

const MAX_TILED_BITS: u32 = 64;
const MAX_NON_INDEPENDENT_64B_EXTENT: u32 = 2560;
const MAX_64K_R_X_PIPES: u64 = 16;

const ADDR_NUM_PIPES: Field = Field { shift: 0, bits: 3 };

pub(crate) const fn set(field: Field, value: u64) -> u64 {
    (value & ((1 << field.bits) - 1)) << field.shift
}

pub(crate) const fn mask(field: Field) -> u64 {
    set(field, u64::MAX)
}

const fn on(field: Field) -> u64 {
    set(field, 1)
}

const fn tile(tile: u64) -> u64 {
    set(TILE, tile)
}

const BLOCK_128B: u64 = set(DCC_MAX_COMPRESSED_BLOCK, DCC_BLOCK_128B);
const BLOCK_256B: u64 = set(DCC_MAX_COMPRESSED_BLOCK, DCC_BLOCK_256B);
const GFX9_DCC: u64 = on(DCC) | on(DCC_INDEPENDENT_64B);
const R_X_DCC_256B: u64 = on(DCC) | on(DCC_PIPE_ALIGN) | on(DCC_INDEPENDENT_128B) | BLOCK_256B;
const R_X_DCC_128B: u64 = on(DCC) | on(DCC_PIPE_ALIGN) | on(DCC_INDEPENDENT_128B) | BLOCK_128B;
const R_X_DCC_RETILE_128B: u64 = on(DCC) | on(DCC_RETILE) | on(DCC_INDEPENDENT_128B) | BLOCK_128B;
const R_X_DCC_RETILE_64B: u64 =
    on(DCC) | on(DCC_RETILE) | on(DCC_INDEPENDENT_64B) | on(DCC_INDEPENDENT_128B);

const RANKS: &[u64] = &[
    VER_GFX6 | tile(GFX6_1D_TILED_THIN1),
    VER_GFX6 | tile(GFX6_1D_TILED_THIN1) | THIN,
    VER_GFX6 | tile(GFX6_2D_TILED_THIN1),
    VER_GFX6 | tile(GFX6_2D_TILED_THIN1) | THIN,
    VER_GFX6 | tile(GFX6_2D_TILED_THIN1) | THIN | on(DCC),
    VER_GFX9 | tile(GFX9_64K_S),
    VER_GFX9 | tile(GFX9_64K_D),
    VER_GFX9 | tile(GFX9_4K_D_X),
    VER_GFX9 | tile(GFX9_64K_S_X),
    VER_GFX9 | tile(GFX9_64K_D_X),
    VER_GFX9 | tile(GFX9_4K_D_X) | on(DCC_PIPE_ALIGN) | GFX9_DCC,
    VER_GFX9 | tile(GFX9_64K_S_X) | on(DCC_RETILE) | GFX9_DCC,
    VER_GFX9 | tile(GFX9_64K_S_X) | GFX9_DCC,
    VER_GFX9 | tile(GFX9_64K_S_X) | on(DCC_PIPE_ALIGN) | GFX9_DCC,
    VER_GFX9 | tile(GFX9_64K_D_X) | on(DCC_PIPE_ALIGN) | GFX9_DCC,
    VER_GFX10 | tile(GFX9_4K_D_X),
    VER_GFX10 | tile(GFX9_64K_R_X),
    VER_GFX10 | tile(GFX9_64K_R_X) | R_X_DCC_RETILE_64B,
    VER_GFX10 | tile(GFX9_64K_R_X) | R_X_DCC_RETILE_128B,
    VER_GFX10 | tile(GFX9_64K_R_X) | R_X_DCC_128B,
    VER_GFX12 | tile(GFX12_256B_2D),
    VER_GFX11 | tile(GFX9_64K_D),
    VER_GFX12 | tile(GFX12_64K_2D),
    VER_GFX12 | tile(GFX12_256B_2D) | on(DCC) | BLOCK_256B,
    VER_GFX12 | tile(GFX12_4K_2D) | on(DCC) | BLOCK_256B,
    VER_GFX12 | tile(GFX12_256K_2D) | on(DCC) | BLOCK_256B,
    VER_GFX12 | tile(GFX12_64K_2D) | on(DCC) | BLOCK_128B,
    VER_GFX12 | tile(GFX12_64K_2D) | on(DCC) | BLOCK_256B,
    VER_GFX11 | tile(GFX9_4K_D_X),
    VER_GFX11 | tile(GFX11_256K_R_X),
    VER_GFX11 | tile(GFX11_256K_R_X) | R_X_DCC_RETILE_64B,
    VER_GFX11 | tile(GFX11_256K_R_X) | R_X_DCC_RETILE_128B,
    VER_GFX11 | tile(GFX11_256K_R_X) | R_X_DCC_128B,
    VER_GFX11 | tile(GFX11_256K_R_X) | R_X_DCC_256B,
    VER_GFX11 | tile(GFX9_64K_R_X),
    VER_GFX11 | tile(GFX9_64K_R_X) | R_X_DCC_RETILE_64B,
    VER_GFX11 | tile(GFX9_64K_R_X) | R_X_DCC_RETILE_128B,
    VER_GFX11 | tile(GFX9_64K_R_X) | R_X_DCC_128B,
    VER_GFX11 | tile(GFX9_64K_R_X) | R_X_DCC_256B,
];

const fn is_amd(modifier: u64) -> bool {
    field(modifier, VENDOR) == DRM_FORMAT_MOD_VENDOR_AMD
}

pub(crate) fn has_dcc(modifier: u64) -> bool {
    field(modifier, DCC) != 0
}

fn supports<K: Kernel>(device: &Device<K>, modifier: u64, config: &Config<'_>) -> bool {
    if device.gfx_version < GFX9 {
        return device.gfx6_supports(modifier, config);
    }
    if !has_dcc(modifier) {
        return true;
    }
    if device.gfx_version >= GFX12 {
        return true;
    }
    if config.implicit_interop {
        return false;
    }
    if field(modifier, DCC_INDEPENDENT_64B) == 1
        || config.width.max(config.height) <= MAX_NON_INDEPENDENT_64B_EXTENT
    {
        return true;
    }
    false
}

fn replace(key: u64, f: Field, value: u64) -> u64 {
    (key & !mask(f)) | set(f, value)
}

fn rank_key(pipes: u64, key: u64) -> u64 {
    match field(key, TILE_VERSION) {
        VER_GFX10_RBPLUS => replace(key, TILE_VERSION, VER_GFX10),
        VER_GFX11 if 1 << pipes > MAX_64K_R_X_PIPES => match field(key, TILE) {
            GFX11_256K_R_X => replace(key, TILE, GFX9_64K_R_X),
            GFX9_64K_R_X => replace(key, TILE, GFX11_256K_R_X),
            _ => key,
        },
        _ => key,
    }
}

pub(crate) fn rank<K: Kernel>(
    device: &Device<K>,
    modifier: u64,
    config: &Config<'_>,
) -> Option<u32> {
    if !is_amd(modifier)
        || config.format.0.planes[0].bits > MAX_TILED_BITS
        || !supports(device, modifier, config)
    {
        return None;
    }
    let chip_fields = match field(modifier, TILE_VERSION) {
        VER_GFX6 => GFX6_CHIP_FIELDS,
        _ => CHIP_FIELDS,
    };
    let key = rank_key(
        field(device.addr_config as u64, ADDR_NUM_PIPES),
        modifier & !(chip_fields | mask(VENDOR)),
    );
    let position = RANKS.iter().position(|&entry| entry == key)?;
    Some(position as u32)
}

pub(crate) struct Planes {
    pub(crate) planes: PlaneVec<PlaneLayout>,
    pub(crate) alignment: u64,
}

pub(crate) fn compute_layout(fmt: &Fmt, modifier: u64, width: u32, height: u32) -> Option<Planes> {
    if !is_amd(modifier) {
        return None;
    }
    match field(modifier, TILE_VERSION) {
        VER_GFX6 => gfx6::modifier_planes(fmt, modifier, width, height),
        VER_GFX9..=VER_GFX12 => gfx9(fmt, modifier, width, height),
        _ => None,
    }
}

pub(crate) fn push_padded(
    planes: &mut PlaneVec<PlaneLayout>,
    padded_x: u64,
    padded_y: u64,
    bytes: u64,
) {
    let stride = padded_x * bytes;
    planes.push(PlaneLayout {
        offset: end(planes),
        stride,
        size: stride * padded_y,
    });
}

fn block_dimensions(block_size_log2: u32, elem_log2: u32) -> (u64, u64) {
    let elems_per_block_log2 = block_size_log2 - elem_log2;
    let width = 1 << elems_per_block_log2.div_ceil(2);
    let height = (1 << elems_per_block_log2) / width;
    (width, height)
}

fn log2_bytes(bits: u32) -> Option<u32> {
    let bytes = bits / 8;
    (bits.is_multiple_of(8) && bytes.is_power_of_two()).then(|| bytes.ilog2())
}

fn gfx9(fmt: &Fmt, modifier: u64, width: u32, height: u32) -> Option<Planes> {
    let version = field(modifier, TILE_VERSION);
    let tile = field(modifier, TILE);
    let block_size_log2 = match version {
        VER_GFX12 => match tile {
            GFX12_256B_2D => 8,
            GFX12_4K_2D => 12,
            GFX12_64K_2D => 16,
            GFX12_256K_2D => 18,
            _ => return None,
        },
        _ => match tile / 4 {
            0 => 8,
            1 | 5 => 12,
            2 | 4 | 6 => 16,
            7 => 18,
            _ => return None,
        },
    };
    let mut planes = PlaneVec::new();
    for plane in fmt.planes.iter() {
        let elem_log2 = log2_bytes(plane.bits)?;
        let (tile_width, tile_height) = block_dimensions(block_size_log2, elem_log2);
        let padded_width = align(blocks(plane.block_width, width), tile_width);
        let padded_height = align(blocks(plane.block_height, height), tile_height);
        push_padded(&mut planes, padded_width, padded_height, 1 << elem_log2);
    }
    let mut alignment = 1 << block_size_log2;
    if !has_dcc(modifier) || version == VER_GFX12 {
        return Some(Planes { planes, alignment });
    }
    let plane0 = &fmt.planes[0];
    let elem_log2 = log2_bytes(plane0.bits)?;
    let x0 = blocks(plane0.block_width, width);
    let y0 = blocks(plane0.block_height, height);
    let is_gfx9 = version == VER_GFX9;
    let retile = field(modifier, DCC_RETILE) == 1;
    if planes.remaining_capacity() < 1 + retile as usize {
        return None;
    }
    let pipe_align = field(modifier, DCC_PIPE_ALIGN) == 1;
    let pipe_xor_bits = field(modifier, PIPE_XOR_BITS) as u32;
    let rb = field(modifier, RB) as u32;
    let pipes = if !retile && !pipe_align {
        0
    } else if is_gfx9 {
        pipe_xor_bits.min(5)
    } else if version >= VER_GFX10_RBPLUS
        && pipe_xor_bits > 1
        && field(modifier, PACKERS) as u32 == pipe_xor_bits
    {
        pipe_xor_bits + 1
    } else {
        pipe_xor_bits
    };
    let mut push_meta = |pipes_log2: u32, rbs_log2: u32| {
        let meta_blk_size_log2 = match is_gfx9 {
            true => 10 + rbs_log2,
            false => 8 + pipes_log2,
        }
        .max(12);
        let (meta_width, meta_height) = block_dimensions(meta_blk_size_log2 + 8, elem_log2);
        let count_x = x0.div_ceil(meta_width);
        let count_y = y0.div_ceil(meta_height);
        let mut size = count_x * count_y * (1 << meta_blk_size_log2);
        let mut meta_alignment = 1 << meta_blk_size_log2;
        if is_gfx9 {
            let meta_size_align = (1u64 << block_size_log2).max(1 << (8 + pipes_log2 + rbs_log2));
            size = align(size, meta_size_align);
            meta_alignment = meta_alignment.max(meta_size_align);
        }
        planes.push(PlaneLayout {
            offset: align(end(&planes), meta_alignment),
            stride: count_x * meta_width,
            size,
        });
        alignment = alignment.max(meta_alignment);
    };
    if retile {
        push_meta(0, 0);
    }
    push_meta(pipes, rb);
    Some(Planes { planes, alignment })
}

impl<K: Kernel> Device<K> {
    pub(super) fn explicit_layout(
        &self,
        layout: Layout,
        alignment: u64,
        config: &Config<'_>,
    ) -> DeviceLayout {
        let scanout = config.scanout && !(config.implicit_interop && self.gfx_version < GFX12);
        let metadata = match self.gfx_version < GFX9 {
            true => self.gfx6_metadata(&layout, config, scanout),
            false => drm_amdgpu_gem_metadata__data {
                flags: 0,
                tiling_info: match self.gfx_version >= GFX12 {
                    true => gfx12_tiling_info(&layout, scanout),
                    false => gfx9_tiling_info(&layout, scanout, self.gfx_version),
                },
                data_size_bytes: 0,
                data: [0; 64],
            },
        };
        let bo = Bo {
            size: end(&layout.planes),
            alignment,
            linear: layout.modifier == DRM_FORMAT_MOD_LINEAR,
            metadata,
        };
        DeviceLayout {
            layout,
            extra: Extra::Amd(bo),
        }
    }
}

const AMDGPU_TILING_SWIZZLE_MODE: Field = Field { shift: 0, bits: 5 };
const AMDGPU_TILING_DCC_OFFSET_256B: Field = Field { shift: 5, bits: 24 };
const AMDGPU_TILING_DCC_PITCH_MAX: Field = Field {
    shift: 29,
    bits: 14,
};
const AMDGPU_TILING_DCC_INDEPENDENT_64B: Field = Field { shift: 43, bits: 1 };
const AMDGPU_TILING_DCC_INDEPENDENT_128B: Field = Field { shift: 44, bits: 1 };
const AMDGPU_TILING_DCC_MAX_COMPRESSED_BLOCK_SIZE: Field = Field { shift: 45, bits: 2 };
const AMDGPU_TILING_SCANOUT: Field = Field { shift: 63, bits: 1 };
const AMDGPU_TILING_GFX12_SWIZZLE_MODE: Field = Field { shift: 0, bits: 3 };
const AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK: Field = Field { shift: 3, bits: 2 };
const AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE: Field = Field { shift: 5, bits: 3 };
const AMDGPU_TILING_GFX12_DCC_DATA_FORMAT: Field = Field { shift: 8, bits: 6 };
const AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE: Field = Field { shift: 14, bits: 1 };
const AMDGPU_TILING_GFX12_SCANOUT: Field = Field { shift: 63, bits: 1 };

fn gfx9_tiling_info(layout: &Layout, scanout: bool, gfx_version: GfxVersion) -> u64 {
    let modifier = layout.modifier;
    let mut tiling = set(AMDGPU_TILING_SWIZZLE_MODE, field(modifier, TILE))
        | set(AMDGPU_TILING_SCANOUT, scanout as u64);
    let dcc = if let Some(dcc) = layout.planes.get(layout.format.planes.len()) {
        tiling |= set(AMDGPU_TILING_DCC_OFFSET_256B, dcc.offset >> 8)
            | set(AMDGPU_TILING_DCC_PITCH_MAX, dcc.stride - 1);
        modifier
    } else if gfx_version >= GFX11_5 {
        on(DCC_INDEPENDENT_128B) | BLOCK_256B
    } else if gfx_version >= GFX10 {
        on(DCC_INDEPENDENT_128B) | BLOCK_128B
    } else {
        on(DCC_INDEPENDENT_64B)
    };
    tiling
        | set(
            AMDGPU_TILING_DCC_INDEPENDENT_64B,
            field(dcc, DCC_INDEPENDENT_64B),
        )
        | set(
            AMDGPU_TILING_DCC_INDEPENDENT_128B,
            field(dcc, DCC_INDEPENDENT_128B),
        )
        | set(
            AMDGPU_TILING_DCC_MAX_COMPRESSED_BLOCK_SIZE,
            field(dcc, DCC_MAX_COMPRESSED_BLOCK),
        )
}

fn gfx12_tiling_info(layout: &Layout, scanout: bool) -> u64 {
    let modifier = layout.modifier;
    let swizzle_mode = match field(modifier, TILE) {
        GFX9_64K_D if field(modifier, TILE_VERSION) == VER_GFX11 => GFX12_64K_2D,
        tile => tile,
    };
    let tiling = set(AMDGPU_TILING_GFX12_SWIZZLE_MODE, swizzle_mode)
        | set(AMDGPU_TILING_GFX12_SCANOUT, scanout as u64)
        | set(
            AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK,
            field(modifier, DCC_MAX_COMPRESSED_BLOCK),
        );
    match &layout.format.amd_gfx12 {
        Some(color) => {
            tiling
                | set(
                    AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE,
                    color.number_type as u64,
                )
                | set(
                    AMDGPU_TILING_GFX12_DCC_DATA_FORMAT,
                    color.data_format as u64,
                )
        }
        None => tiling | set(AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE, 1),
    }
}
