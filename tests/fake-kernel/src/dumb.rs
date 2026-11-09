use crate::KernelVersion;
use crate::PAGE_SIZE;
use crate::errno;
use crate::uapi::drm::drm_mode_create_dumb;
use libc::EINVAL;

struct ColorMode {
    chars_per_block: u64,
    pixels_per_block: u64,
}

fn legacy_color_mode(bpp: u32) -> Option<ColorMode> {
    let mode = |chars_per_block, pixels_per_block| ColorMode {
        chars_per_block,
        pixels_per_block,
    };
    match bpp {
        1 => Some(mode(1, 8)),
        2 => Some(mode(1, 4)),
        4 => Some(mode(1, 2)),
        8 => Some(mode(1, 1)),
        15 | 16 => Some(mode(2, 1)),
        24 => Some(mode(3, 1)),
        32 => Some(mode(4, 1)),
        _ => None,
    }
}

pub(crate) fn create_dumb(
    kernel: KernelVersion,
    args: &mut drm_mode_create_dumb,
    dumb_create: impl FnOnce(&mut drm_mode_create_dumb) -> Result<(), i32>,
) -> i64 {
    let created = check(args).and_then(|()| {
        args.handle = 0;
        args.pitch = 0;
        args.size = 0;
        dumb_create(args)
    });
    match created {
        Ok(()) => 0,
        Err(e) => {
            if kernel >= KernelVersion::V6_19 {
                args.handle = 0;
                args.pitch = 0;
                args.size = 0;
            }
            errno(e)
        }
    }
}

fn check(args: &drm_mode_create_dumb) -> Result<(), i32> {
    if args.width == 0 || args.height == 0 || args.bpp == 0 {
        return Err(EINVAL);
    }
    if args.bpp > u32::MAX - 8 {
        return Err(EINVAL);
    }
    let cpp = args.bpp.div_ceil(8);
    if cpp > u32::MAX / args.width {
        return Err(EINVAL);
    }
    let stride = cpp * args.width;
    if args.height > u32::MAX / stride {
        return Err(EINVAL);
    }
    let size = args.height * stride;
    if (size as u64).next_multiple_of(PAGE_SIZE) == 0 {
        return Err(EINVAL);
    }
    Ok(())
}

pub(crate) fn shmem_dumb_size(
    kernel: KernelVersion,
    args: &mut drm_mode_create_dumb,
) -> Result<(), i32> {
    if kernel < KernelVersion::V6_19 {
        let bits = args.width.checked_mul(args.bpp);
        let Some(rounded) = bits.and_then(|bits| bits.checked_add(7)) else {
            unimplemented!("dumb buffers whose width * bpp overflows before 6.19");
        };
        let pitch = rounded / 8;
        let Some(size) = pitch.checked_mul(args.height) else {
            unimplemented!("dumb buffers whose pitch * height overflows before 6.19");
        };
        args.pitch = pitch;
        args.size = (size as u64).next_multiple_of(PAGE_SIZE);
        return Ok(());
    }
    let pitch = match legacy_color_mode(args.bpp) {
        Some(mode) => (args.width as u64 * mode.chars_per_block).div_ceil(mode.pixels_per_block),
        None => args.width as u64 * args.bpp.div_ceil(8) as u64,
    };
    let pitch = pitch as u32;
    let Some(size) = (args.height * pitch).checked_next_multiple_of(PAGE_SIZE as u32) else {
        return Err(EINVAL);
    };
    args.pitch = pitch;
    args.size = size as u64;
    Ok(())
}
