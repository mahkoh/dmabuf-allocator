pub const fn shift(offset: u32, width: u32) -> u32 {
    match cfg!(target_endian = "big") {
        true => u32::BITS - offset - width,
        false => offset,
    }
}

pub const fn mask(width: u32) -> u32 {
    u32::MAX >> (u32::BITS - width)
}

pub const fn get(unit: u32, offset: u32, width: u32) -> u32 {
    unit >> shift(offset, width) & mask(width)
}

pub const fn set(unit: u32, offset: u32, width: u32, value: u32) -> u32 {
    let mask = mask(width) << shift(offset, width);
    unit & !mask | value << shift(offset, width) & mask
}

macro_rules! bitfields {
    ($($unit:ident { $($get:ident, $set:ident: $offset:expr, $width:expr;)* })*) => {
        $($(
            pub const fn $get(&self) -> u32 {
                $crate::bitfield::get(self.$unit, $offset, $width)
            }

            pub const fn $set(&mut self, value: u32) {
                self.$unit = $crate::bitfield::set(self.$unit, $offset, $width, value);
            }
        )*)*
    };
}

pub(crate) use bitfields;
