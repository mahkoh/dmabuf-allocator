use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::LowerHex;
use std::str::FromStr;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Region {
    pub memory_class: u16,
    pub memory_instance: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub key: &'static str,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Traced {
    pub handle: Option<u32>,
    pub call: Call,
}

trait Format<T> {
    fn text(value: &T) -> Option<String>;
    fn parse(text: &str) -> Result<T, String>;
    fn absent() -> Option<T> {
        None
    }
}

struct Dec;
struct Hex;
struct Words;
struct Regions;
struct Optional;

fn number<T: TryFrom<u64>>(value: &str) -> Result<T, String> {
    let parsed = match value.strip_prefix("0x") {
        Some(hex) => u64::from_str_radix(hex, 16),
        None => value.parse(),
    };
    parsed
        .ok()
        .and_then(|v| T::try_from(v).ok())
        .ok_or_else(|| format!("bad number {value}"))
}

impl<T: Display + TryFrom<u64>> Format<T> for Dec {
    fn text(value: &T) -> Option<String> {
        Some(value.to_string())
    }

    fn parse(text: &str) -> Result<T, String> {
        number(text)
    }
}

impl<T: LowerHex + TryFrom<u64>> Format<T> for Hex {
    fn text(value: &T) -> Option<String> {
        Some(format!("{value:#x}"))
    }

    fn parse(text: &str) -> Result<T, String> {
        number(text)
    }
}

impl Format<Vec<u32>> for Words {
    fn text(value: &Vec<u32>) -> Option<String> {
        if value.is_empty() {
            return None;
        }
        let words: Vec<String> = value.iter().map(|word| format!("{word:08x}")).collect();
        Some(words.join(","))
    }

    fn parse(text: &str) -> Result<Vec<u32>, String> {
        text.split(',')
            .map(|word| u32::from_str_radix(word, 16).map_err(|_| format!("bad word {word}")))
            .collect()
    }

    fn absent() -> Option<Vec<u32>> {
        Some(Vec::new())
    }
}

impl Format<Option<Vec<Region>>> for Regions {
    fn text(value: &Option<Vec<Region>>) -> Option<String> {
        let regions: Vec<String> = value
            .as_ref()?
            .iter()
            .map(|region| format!("{}:{}", region.memory_class, region.memory_instance))
            .collect();
        Some(regions.join(","))
    }

    fn parse(text: &str) -> Result<Option<Vec<Region>>, String> {
        let regions = text.split(',').map(|region| {
            let (class, instance) = region
                .split_once(':')
                .ok_or_else(|| format!("bad region {region}"))?;
            Ok(Region {
                memory_class: number(class)?,
                memory_instance: number(instance)?,
            })
        });
        regions.collect::<Result<_, String>>().map(Some)
    }

    fn absent() -> Option<Option<Vec<Region>>> {
        Some(None)
    }
}

impl<T: Display + TryFrom<u64>> Format<Option<T>> for Optional {
    fn text(value: &Option<T>) -> Option<String> {
        value.as_ref().map(T::to_string)
    }

    fn parse(text: &str) -> Result<Option<T>, String> {
        number(text).map(Some)
    }

    fn absent() -> Option<Option<T>> {
        Some(None)
    }
}

struct Word<'a> {
    key: &'a str,
    value: &'a str,
}

fn words(line: &str) -> Result<Vec<Word<'_>>, String> {
    let mut words: Vec<Word<'_>> = Vec::new();
    for word in line.split(' ') {
        let (key, value) = word
            .split_once('=')
            .ok_or_else(|| format!("bad field {word}"))?;
        if words.iter().any(|word| word.key == key) {
            return Err(format!("duplicate field {key}"));
        }
        words.push(Word { key, value });
    }
    Ok(words)
}

fn take<'a>(words: &mut Vec<Word<'a>>, key: &str) -> Option<&'a str> {
    let index = words.iter().position(|word| word.key == key)?;
    Some(words.remove(index).value)
}

macro_rules! calls {
    ($($variant:ident $name:literal { $($field:ident: $ty:ty = $format:ident,)* })*) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum Call {
            $($variant { $($field: $ty,)* },)*
        }

        impl Call {
            pub fn name(&self) -> &'static str {
                match self {
                    $(Call::$variant { .. } => $name,)*
                }
            }

            pub fn fields(&self) -> Vec<Field> {
                let mut fields = Vec::new();
                match self {
                    $(Call::$variant { $($field,)* } => {
                        $(
                            if let Some(value) = <$format as Format<$ty>>::text($field) {
                                fields.push(Field {
                                    key: stringify!($field),
                                    value,
                                });
                            }
                        )*
                    })*
                }
                fields
            }

            pub fn set(&mut self, key: &str, value: &str) -> Result<bool, String> {
                match self {
                    $(Call::$variant { $($field,)* } => {
                        $(
                            if key == stringify!($field) {
                                if <$format as Format<$ty>>::text($field).is_none() {
                                    return Ok(false);
                                }
                                *$field = <$format as Format<$ty>>::parse(value)?;
                                return Ok(true);
                            }
                        )*
                    })*
                }
                Ok(false)
            }

            fn from_words(name: &str, words: &mut Vec<Word<'_>>) -> Result<Call, String> {
                let call = match name {
                    $($name => Call::$variant {
                        $($field: match take(words, stringify!($field)) {
                            Some(text) => <$format as Format<$ty>>::parse(text)?,
                            None => <$format as Format<$ty>>::absent().ok_or_else(|| {
                                format!("{name}: missing {}", stringify!($field))
                            })?,
                        },)*
                    },)*
                    _ => return Err(format!("unknown call {name}")),
                };
                Ok(call)
            }
        }
    }
}

calls! {
    AmdgpuGemCreate "amdgpu_gem_create" {
        size: u64 = Dec,
        alignment: u64 = Dec,
        domains: u64 = Hex,
        flags: u64 = Hex,
    }
    AmdgpuGemMetadata "amdgpu_gem_metadata" {
        flags: u64 = Hex,
        tiling_info: u64 = Hex,
        data: Vec<u32> = Words,
    }
    I915GemCreate "i915_gem_create" {
        size: u64 = Dec,
    }
    AsahiGemCreate "asahi_gem_create" {
        size: u64 = Dec,
        flags: u32 = Hex,
    }
    PanfrostCreateBo "panfrost_create_bo" {
        size: u32 = Dec,
        flags: u32 = Hex,
    }
    PanthorBoCreate "panthor_bo_create" {
        size: u64 = Dec,
        flags: u32 = Hex,
    }
    V3dCreateBo "v3d_create_bo" {
        size: u32 = Dec,
        flags: u32 = Hex,
    }
    Vc4CreateBo "vc4_create_bo" {
        size: u32 = Dec,
        flags: u32 = Hex,
    }
    Vc4SetTiling "vc4_set_tiling" {
        flags: u32 = Hex,
        modifier: u64 = Hex,
    }
    LimaGemCreate "lima_gem_create" {
        size: u32 = Dec,
        flags: u32 = Hex,
    }
    EtnavivGemNew "etnaviv_gem_new" {
        size: u64 = Dec,
        flags: u32 = Hex,
    }
    VirtgpuResourceCreate "virtgpu_resource_create" {
        target: u32 = Dec,
        format: u32 = Dec,
        bind: u32 = Hex,
        width: u32 = Dec,
        height: u32 = Dec,
        depth: u32 = Dec,
        array_size: u32 = Dec,
        last_level: u32 = Dec,
        nr_samples: u32 = Dec,
        flags: u32 = Hex,
        size: u32 = Dec,
        stride: u32 = Dec,
    }
    VirtgpuAmdgpuGemNew "virtgpu_amdgpu_gem_new" {
        blob_mem: u32 = Dec,
        blob_flags: u32 = Hex,
        blob_hints: u32 = Hex,
        size: u64 = Dec,
        alloc_size: u64 = Dec,
        phys_alignment: u64 = Dec,
        preferred_heap: u32 = Hex,
        flags: u64 = Hex,
    }
    VirtgpuAmdgpuSetMetadata "virtgpu_amdgpu_set_metadata" {
        flags: u64 = Hex,
        tiling_info: u64 = Hex,
    }
    VirtgpuPanfrostCreateBo "virtgpu_panfrost_create_bo" {
        blob_mem: u32 = Dec,
        blob_flags: u32 = Hex,
        blob_hints: u32 = Hex,
        size: u64 = Dec,
        gem_size: u64 = Dec,
        flags: u32 = Hex,
    }
    VirtgpuAsahiGemNew "virtgpu_asahi_gem_new" {
        blob_mem: u32 = Dec,
        blob_flags: u32 = Hex,
        blob_hints: u32 = Hex,
        size: u64 = Dec,
        gem_size: u64 = Dec,
        flags: u32 = Hex,
    }
    VirtgpuI915GemCreate "virtgpu_i915_gem_create" {
        blob_mem: u32 = Dec,
        blob_flags: u32 = Hex,
        blob_hints: u32 = Hex,
        size: u64 = Dec,
        gem_size: u64 = Dec,
    }
    VirtgpuI915GemCreateExt "virtgpu_i915_gem_create_ext" {
        blob_mem: u32 = Dec,
        blob_flags: u32 = Hex,
        blob_hints: u32 = Hex,
        size: u64 = Dec,
        gem_size: u64 = Dec,
        flags: u32 = Hex,
        regions: Option<Vec<Region>> = Regions,
        pat_index: Option<u32> = Optional,
    }
    ModeCreateDumb "mode_create_dumb" {
        width: u32 = Dec,
        height: u32 = Dec,
        bpp: u32 = Dec,
    }
    I915GemCreateExt "i915_gem_create_ext" {
        size: u64 = Dec,
        flags: u32 = Hex,
        regions: Option<Vec<Region>> = Regions,
        pat_index: Option<u32> = Optional,
    }
    I915GemSetTiling "i915_gem_set_tiling" {
        tiling_mode: u32 = Dec,
        stride: u32 = Dec,
    }
    I915GemSetDomain "i915_gem_set_domain" {
        read_domains: u32 = Hex,
        write_domain: u32 = Hex,
    }
    XeGemCreate "xe_gem_create" {
        size: u64 = Dec,
        placement: u32 = Hex,
        flags: u32 = Hex,
        vm_id: u32 = Dec,
        cpu_caching: u16 = Dec,
    }
    PrimeHandleToFd "prime_handle_to_fd" {
        flags: u32 = Hex,
    }
    NvidiaGemAllocNvkmsMemory "nvidia_gem_alloc_nvkms_memory" {
        block_linear: u8 = Dec,
        compressible: u8 = Dec,
        memory_size: u64 = Dec,
        flags: u32 = Hex,
    }
    NouveauGemNew "nouveau_gem_new" {
        domain: u32 = Hex,
        size: u64 = Dec,
        align: u32 = Dec,
        tile_mode: u32 = Hex,
        tile_flags: u32 = Hex,
    }
    MsmGemNew "msm_gem_new" {
        size: u64 = Dec,
        flags: u32 = Hex,
    }
    UdmabufCreate "udmabuf_create" {
        flags: u32 = Hex,
        offset: u64 = Dec,
        size: u64 = Dec,
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())?;
        for field in self.fields() {
            write!(f, " {}={}", field.key, field.value)?;
        }
        Ok(())
    }
}

impl FromStr for Call {
    type Err = String;

    fn from_str(line: &str) -> Result<Call, String> {
        let (name, mut words) = match line.split_once(' ') {
            Some((name, rest)) => (name, words(rest)?),
            None => (line, Vec::new()),
        };
        let call = Call::from_words(name, &mut words)?;
        match words.first() {
            Some(word) => Err(format!("{name}: unknown field {}", word.key)),
            None => Ok(call),
        }
    }
}
