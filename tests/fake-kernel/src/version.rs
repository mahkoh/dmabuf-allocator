use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

macro_rules! kernel_versions {
    ($($name:ident = $text:literal),* $(,)?) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub enum KernelVersion {
            $($name),*
        }

        impl KernelVersion {
            pub const EMULATED: &[Self] = &[$(Self::$name),*];

            pub const fn text(self) -> &'static str {
                match self {
                    $(Self::$name => $text),*
                }
            }
        }
    };
}

kernel_versions! {
    V6_13 = "6.13",
    V6_14 = "6.14",
    V6_15 = "6.15",
    V6_16 = "6.16",
    V6_17 = "6.17",
    V6_18 = "6.18",
    V6_19 = "6.19",
    V7_0 = "7.0",
    V7_1 = "7.1",
    V7_2 = "7.2",
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DriverVersion {
    pub major: i32,
    pub minor: i32,
    pub patchlevel: i32,
}

impl DriverVersion {
    pub const ASAHI: Self = Self::new(0, 0, 0);
    pub const ETNAVIV: Self = Self::new(1, 4, 0);
    pub const I915: Self = Self::new(1, 6, 0);
    pub const LIMA: Self = Self::new(1, 1, 0);
    pub const NVIDIA_DRM: Self = Self::new(0, 0, 0);
    pub const V3D: Self = Self::new(1, 0, 0);
    pub const VC4: Self = Self::new(0, 0, 0);
    pub const VIRTIO_GPU: Self = Self::new(0, 1, 0);
    pub const VKMS: Self = Self::new(1, 0, 0);
    pub const XE: Self = Self::new(1, 1, 0);

    pub const fn new(major: i32, minor: i32, patchlevel: i32) -> Self {
        Self {
            major,
            minor,
            patchlevel,
        }
    }
}

impl KernelVersion {
    pub const NEWEST: Self = Self::EMULATED[Self::EMULATED.len() - 1];

    pub fn parse(text: &str) -> Self {
        Self::EMULATED
            .iter()
            .copied()
            .find(|version| version.text() == text)
            .unwrap_or_else(|| panic!("kernel version {text} is not emulated"))
    }

    pub fn amdgpu(self) -> DriverVersion {
        let minor = match self {
            Self::V6_13 => 59,
            Self::V6_14 => 61,
            Self::V6_15 => 63,
            Self::V6_16
            | Self::V6_17
            | Self::V6_18
            | Self::V6_19
            | Self::V7_0
            | Self::V7_1
            | Self::V7_2 => 64,
        };
        DriverVersion::new(3, minor, 0)
    }

    pub fn nouveau(self) -> DriverVersion {
        let patchlevel = match self {
            Self::V6_13 | Self::V6_14 | Self::V6_15 | Self::V6_16 | Self::V6_17 | Self::V6_18 => 0,
            Self::V6_19 => 1,
            Self::V7_0 | Self::V7_1 | Self::V7_2 => 2,
        };
        DriverVersion::new(1, 4, patchlevel)
    }

    pub fn panfrost(self) -> DriverVersion {
        let minor = match self {
            Self::V6_13 | Self::V6_14 | Self::V6_15 | Self::V6_16 => 3,
            Self::V6_17 | Self::V6_18 => 4,
            Self::V6_19 => 5,
            Self::V7_0 | Self::V7_1 | Self::V7_2 => 6,
        };
        DriverVersion::new(1, minor, 0)
    }

    pub fn panthor(self) -> DriverVersion {
        let minor = match self {
            Self::V6_13 => 2,
            Self::V6_14 | Self::V6_15 => 3,
            Self::V6_16 => 4,
            Self::V6_17 | Self::V6_18 | Self::V6_19 => 5,
            Self::V7_0 => 7,
            Self::V7_1 | Self::V7_2 => 8,
        };
        DriverVersion::new(1, minor, 0)
    }

    pub fn msm(self) -> DriverVersion {
        let minor = match self {
            Self::V6_13 | Self::V6_14 | Self::V6_15 | Self::V6_16 => 12,
            Self::V6_17 | Self::V6_18 | Self::V6_19 | Self::V7_0 | Self::V7_1 | Self::V7_2 => 13,
        };
        DriverVersion::new(1, minor, 0)
    }
}

impl Display for KernelVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.text())
    }
}
