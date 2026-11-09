pub struct AsicId {
    pub device_id: u16,
    pub revision_id: u8,
    pub marketing_name: &'static str,
}

pub static ASIC_IDS: &[AsicId] = &[
    AsicId {
        device_id: 0x1114,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon 860M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xd2,
        marketing_name: "AMD Radeon 860M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xd3,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xe2,
        marketing_name: "AMD Radeon 860M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xe4,
        marketing_name: "AMD Radeon 860M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xe5,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xe9,
        marketing_name: "AMD Radeon 860M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xea,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xed,
        marketing_name: "AMD Radeon 860M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xee,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xf2,
        marketing_name: "AMD Radeon 860M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xf3,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xf9,
        marketing_name: "AMD Radeon 860M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xfa,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xfc,
        marketing_name: "AMD Radeon 860M Graphics",
    },
    AsicId {
        device_id: 0x1114,
        revision_id: 0xfd,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1309,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x130a,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R6 Graphics",
    },
    AsicId {
        device_id: 0x130b,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R4 Graphics",
    },
    AsicId {
        device_id: 0x130c,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x130d,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R6 Graphics",
    },
    AsicId {
        device_id: 0x130e,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x130f,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x130f,
        revision_id: 0xd4,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x130f,
        revision_id: 0xd5,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x130f,
        revision_id: 0xd6,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x130f,
        revision_id: 0xd7,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x1313,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x1313,
        revision_id: 0xd4,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x1313,
        revision_id: 0xd5,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x1313,
        revision_id: 0xd6,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x1315,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x1315,
        revision_id: 0xd4,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x1315,
        revision_id: 0xd5,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x1315,
        revision_id: 0xd6,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x1315,
        revision_id: 0xd7,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x1316,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x1318,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x131b,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R4 Graphics",
    },
    AsicId {
        device_id: 0x131c,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x131d,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R6 Graphics",
    },
    AsicId {
        device_id: 0x1435,
        revision_id: 0xae,
        marketing_name: "AMD Custom GPU 0932",
    },
    AsicId {
        device_id: 0x1506,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon 610M",
    },
    AsicId {
        device_id: 0x1506,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon 610M",
    },
    AsicId {
        device_id: 0x1506,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon 610M",
    },
    AsicId {
        device_id: 0x1506,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon 610M",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon 890M Graphics",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon 880M Graphics",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xc5,
        marketing_name: "AMD Radeon 890M Graphics",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xc6,
        marketing_name: "AMD Radeon 890M Graphics",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon 890M Graphics",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xd1,
        marketing_name: "AMD Radeon 890M Graphics",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xd2,
        marketing_name: "AMD Radeon 880M Graphics",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xd3,
        marketing_name: "AMD Radeon 890M Graphics",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xe1,
        marketing_name: "AMD Radeon 890M Graphics",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xe3,
        marketing_name: "AMD Radeon 890M Graphics",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xe4,
        marketing_name: "AMD Radeon 890M Graphics",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xf1,
        marketing_name: "AMD Radeon 890M Graphics",
    },
    AsicId {
        device_id: 0x150e,
        revision_id: 0xf3,
        marketing_name: "AMD Radeon 890M Graphics",
    },
    AsicId {
        device_id: 0x1586,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon 8060S Graphics",
    },
    AsicId {
        device_id: 0x1586,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon 8050S Graphics",
    },
    AsicId {
        device_id: 0x1586,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon 8060S Graphics",
    },
    AsicId {
        device_id: 0x1586,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon 8050S Graphics",
    },
    AsicId {
        device_id: 0x1586,
        revision_id: 0xc6,
        marketing_name: "AMD Radeon 8060S Graphics",
    },
    AsicId {
        device_id: 0x1586,
        revision_id: 0xd1,
        marketing_name: "AMD Radeon 8060S Graphics",
    },
    AsicId {
        device_id: 0x1586,
        revision_id: 0xd2,
        marketing_name: "AMD Radeon 8050S Graphics",
    },
    AsicId {
        device_id: 0x1586,
        revision_id: 0xd4,
        marketing_name: "AMD Radeon 8050S Graphics",
    },
    AsicId {
        device_id: 0x1586,
        revision_id: 0xd5,
        marketing_name: "AMD Radeon 8040S Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0x00,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0x01,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0x02,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0x03,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0x05,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0x06,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0x07,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0x08,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xc5,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xc6,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xc9,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xca,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xcb,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xcc,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xcd,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xce,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xd0,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xd1,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xd2,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xd3,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xd4,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xd5,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xd6,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xd7,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xd8,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xd9,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xda,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xdb,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xdc,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xdd,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xde,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xdf,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15bf,
        revision_id: 0xf0,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xc5,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xc6,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xd1,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xd2,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xd3,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xd4,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xd5,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xd6,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xd7,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15c8,
        revision_id: 0xd8,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0x00,
        marketing_name: "AMD Radeon RX Vega 8 Graphics WS",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0x91,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0x91,
        marketing_name: "AMD Ryzen Embedded R1606G with Radeon Vega Gfx",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0x92,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0x92,
        marketing_name: "AMD Ryzen Embedded R1505G with Radeon Vega Gfx",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0x93,
        marketing_name: "AMD Radeon Vega 1 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xa1,
        marketing_name: "AMD Radeon Vega 10 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xa2,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xa3,
        marketing_name: "AMD Radeon Vega 6 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xa4,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xb1,
        marketing_name: "AMD Radeon Vega 10 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xb2,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xb3,
        marketing_name: "AMD Radeon Vega 6 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xb4,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon Vega 10 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon Vega 6 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xc5,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon Vega 11 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xc9,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xca,
        marketing_name: "AMD Radeon Vega 11 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xcb,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xcc,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xce,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xcf,
        marketing_name: "AMD Ryzen Embedded R1305G with Radeon Vega Gfx",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xd1,
        marketing_name: "AMD Radeon Vega 10 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xd2,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xd3,
        marketing_name: "AMD Radeon Vega 6 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xd4,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xd8,
        marketing_name: "AMD Radeon Vega 11 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xd9,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xda,
        marketing_name: "AMD Radeon Vega 11 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xdb,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xdb,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xdc,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xdd,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xde,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xdf,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xe3,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15d8,
        revision_id: 0xe4,
        marketing_name: "AMD Ryzen Embedded R1102G with Radeon Vega Gfx",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0x81,
        marketing_name: "AMD Ryzen Embedded V1807B with Radeon Vega Gfx",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0x82,
        marketing_name: "AMD Ryzen Embedded V1756B with Radeon Vega Gfx",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0x83,
        marketing_name: "AMD Ryzen Embedded V1605B with Radeon Vega Gfx",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0x84,
        marketing_name: "AMD Radeon Vega 6 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0x85,
        marketing_name: "AMD Ryzen Embedded V1202B with Radeon Vega Gfx",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0x86,
        marketing_name: "AMD Radeon Vega 11 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0x88,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon Vega 11 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon Vega 3 / 10 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xc5,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xc6,
        marketing_name: "AMD Radeon Vega 11 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xc9,
        marketing_name: "AMD Radeon Vega 11 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xca,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xcb,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xcc,
        marketing_name: "AMD Radeon Vega 6 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xce,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xd0,
        marketing_name: "AMD Radeon Vega 10 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xd1,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xd3,
        marketing_name: "AMD Radeon Vega 11 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xd5,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xd6,
        marketing_name: "AMD Radeon Vega 11 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xd7,
        marketing_name: "AMD Radeon Vega 8 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xd8,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xd9,
        marketing_name: "AMD Radeon Vega 6 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xe1,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x15dd,
        revision_id: 0xe2,
        marketing_name: "AMD Radeon Vega 3 Graphics",
    },
    AsicId {
        device_id: 0x163f,
        revision_id: 0xae,
        marketing_name: "AMD Custom GPU 0405",
    },
    AsicId {
        device_id: 0x163f,
        revision_id: 0xe1,
        marketing_name: "AMD Custom GPU 0405",
    },
    AsicId {
        device_id: 0x164e,
        revision_id: 0xd8,
        marketing_name: "AMD Radeon 610M",
    },
    AsicId {
        device_id: 0x164e,
        revision_id: 0xd9,
        marketing_name: "AMD Radeon 610M",
    },
    AsicId {
        device_id: 0x164e,
        revision_id: 0xda,
        marketing_name: "AMD Radeon 610M",
    },
    AsicId {
        device_id: 0x164e,
        revision_id: 0xdb,
        marketing_name: "AMD Radeon 610M",
    },
    AsicId {
        device_id: 0x164e,
        revision_id: 0xdc,
        marketing_name: "AMD Radeon 610M",
    },
    AsicId {
        device_id: 0x1681,
        revision_id: 0x06,
        marketing_name: "AMD Radeon 680M",
    },
    AsicId {
        device_id: 0x1681,
        revision_id: 0x07,
        marketing_name: "AMD Radeon 660M",
    },
    AsicId {
        device_id: 0x1681,
        revision_id: 0x0a,
        marketing_name: "AMD Radeon 680M",
    },
    AsicId {
        device_id: 0x1681,
        revision_id: 0x0b,
        marketing_name: "AMD Radeon 660M",
    },
    AsicId {
        device_id: 0x1681,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon 680M",
    },
    AsicId {
        device_id: 0x1681,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon 680M",
    },
    AsicId {
        device_id: 0x1681,
        revision_id: 0xc9,
        marketing_name: "AMD Radeon 660M",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0x01,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0x02,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0x03,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0x04,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0x05,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0x06,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0x07,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xb0,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xb1,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xb2,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xb3,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xb4,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xb5,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xb6,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xb7,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xb8,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xb9,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xba,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xbb,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xc5,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xc6,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xc9,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xca,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xcb,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xcc,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xcd,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xce,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xd0,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xd1,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xd2,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xd3,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xd4,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xd5,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xd6,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xd7,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xd8,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xd9,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xda,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xdb,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xdc,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xdd,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xde,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xdf,
        marketing_name: "AMD Radeon 760M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xf0,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xf1,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1900,
        revision_id: 0xf2,
        marketing_name: "AMD Radeon 780M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xc6,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xc9,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xca,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xd1,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xd2,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xd3,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xd4,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xd5,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xd6,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xd7,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1901,
        revision_id: 0xd8,
        marketing_name: "AMD Radeon 740M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon 820M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xc6,
        marketing_name: "AMD Radeon 820M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xc9,
        marketing_name: "AMD Radeon 820M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xca,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xd1,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xd3,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xd7,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x1902,
        revision_id: 0xd8,
        marketing_name: "AMD Radeon 840M Graphics",
    },
    AsicId {
        device_id: 0x6600,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8600 / 8700M",
    },
    AsicId {
        device_id: 0x6600,
        revision_id: 0x81,
        marketing_name: "AMD Radeon R7 M370",
    },
    AsicId {
        device_id: 0x6601,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8500M / 8700M",
    },
    AsicId {
        device_id: 0x6604,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 M265 Series",
    },
    AsicId {
        device_id: 0x6604,
        revision_id: 0x81,
        marketing_name: "AMD Radeon R7 M350",
    },
    AsicId {
        device_id: 0x6605,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 M260 Series",
    },
    AsicId {
        device_id: 0x6605,
        revision_id: 0x81,
        marketing_name: "AMD Radeon R7 M340",
    },
    AsicId {
        device_id: 0x6606,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8790M",
    },
    AsicId {
        device_id: 0x6607,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R5 M240",
    },
    AsicId {
        device_id: 0x6608,
        revision_id: 0x00,
        marketing_name: "AMD FirePro W2100",
    },
    AsicId {
        device_id: 0x6610,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 200 Series",
    },
    AsicId {
        device_id: 0x6610,
        revision_id: 0x81,
        marketing_name: "AMD Radeon R7 350",
    },
    AsicId {
        device_id: 0x6610,
        revision_id: 0x83,
        marketing_name: "AMD Radeon R5 340",
    },
    AsicId {
        device_id: 0x6610,
        revision_id: 0x87,
        marketing_name: "AMD Radeon R7 200 Series",
    },
    AsicId {
        device_id: 0x6611,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 200 Series",
    },
    AsicId {
        device_id: 0x6611,
        revision_id: 0x87,
        marketing_name: "AMD Radeon R7 200 Series",
    },
    AsicId {
        device_id: 0x6613,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 200 Series",
    },
    AsicId {
        device_id: 0x6617,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 240 Series",
    },
    AsicId {
        device_id: 0x6617,
        revision_id: 0x87,
        marketing_name: "AMD Radeon R7 200 Series",
    },
    AsicId {
        device_id: 0x6617,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon R7 240 Series",
    },
    AsicId {
        device_id: 0x6640,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8950",
    },
    AsicId {
        device_id: 0x6640,
        revision_id: 0x80,
        marketing_name: "AMD Radeon R9 M380",
    },
    AsicId {
        device_id: 0x6646,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 M280X",
    },
    AsicId {
        device_id: 0x6646,
        revision_id: 0x80,
        marketing_name: "AMD Radeon R9 M385",
    },
    AsicId {
        device_id: 0x6646,
        revision_id: 0x80,
        marketing_name: "AMD Radeon R9 M470X",
    },
    AsicId {
        device_id: 0x6647,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 M200X Series",
    },
    AsicId {
        device_id: 0x6647,
        revision_id: 0x80,
        marketing_name: "AMD Radeon R9 M380",
    },
    AsicId {
        device_id: 0x6649,
        revision_id: 0x00,
        marketing_name: "AMD FirePro W5100",
    },
    AsicId {
        device_id: 0x6658,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 200 Series",
    },
    AsicId {
        device_id: 0x665c,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7700 Series",
    },
    AsicId {
        device_id: 0x665d,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 200 Series",
    },
    AsicId {
        device_id: 0x665f,
        revision_id: 0x81,
        marketing_name: "AMD Radeon R7 360 Series",
    },
    AsicId {
        device_id: 0x6660,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8600M Series",
    },
    AsicId {
        device_id: 0x6660,
        revision_id: 0x81,
        marketing_name: "AMD Radeon R5 M335",
    },
    AsicId {
        device_id: 0x6660,
        revision_id: 0x83,
        marketing_name: "AMD Radeon R5 M330",
    },
    AsicId {
        device_id: 0x6663,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8500M Series",
    },
    AsicId {
        device_id: 0x6663,
        revision_id: 0x83,
        marketing_name: "AMD Radeon R5 M320",
    },
    AsicId {
        device_id: 0x6664,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R5 M200 Series",
    },
    AsicId {
        device_id: 0x6665,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R5 M230 Series",
    },
    AsicId {
        device_id: 0x6665,
        revision_id: 0x83,
        marketing_name: "AMD Radeon R5 M320",
    },
    AsicId {
        device_id: 0x6665,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon R5 M435",
    },
    AsicId {
        device_id: 0x6666,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R5 M200 Series",
    },
    AsicId {
        device_id: 0x6667,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R5 M200 Series",
    },
    AsicId {
        device_id: 0x666f,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8500M",
    },
    AsicId {
        device_id: 0x66a1,
        revision_id: 0x02,
        marketing_name: "AMD Instinct MI60 / MI50",
    },
    AsicId {
        device_id: 0x66a1,
        revision_id: 0x06,
        marketing_name: "AMD Radeon Pro VII",
    },
    AsicId {
        device_id: 0x66af,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon VII",
    },
    AsicId {
        device_id: 0x6780,
        revision_id: 0x00,
        marketing_name: "AMD FirePro W9000",
    },
    AsicId {
        device_id: 0x6784,
        revision_id: 0x00,
        marketing_name: "ATI FirePro V (FireGL V) Graphics Adapter",
    },
    AsicId {
        device_id: 0x6788,
        revision_id: 0x00,
        marketing_name: "ATI FirePro V (FireGL V) Graphics Adapter",
    },
    AsicId {
        device_id: 0x678a,
        revision_id: 0x00,
        marketing_name: "AMD FirePro W8000",
    },
    AsicId {
        device_id: 0x6798,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 200 / HD 7900 Series",
    },
    AsicId {
        device_id: 0x6799,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7900 Series",
    },
    AsicId {
        device_id: 0x679a,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7900 Series",
    },
    AsicId {
        device_id: 0x679b,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7900 Series",
    },
    AsicId {
        device_id: 0x679e,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7800 Series",
    },
    AsicId {
        device_id: 0x67a0,
        revision_id: 0x00,
        marketing_name: "AMD Radeon FirePro W9100",
    },
    AsicId {
        device_id: 0x67a1,
        revision_id: 0x00,
        marketing_name: "AMD Radeon FirePro W8100",
    },
    AsicId {
        device_id: 0x67b0,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 200 Series",
    },
    AsicId {
        device_id: 0x67b0,
        revision_id: 0x80,
        marketing_name: "AMD Radeon R9 390 Series",
    },
    AsicId {
        device_id: 0x67b1,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 200 Series",
    },
    AsicId {
        device_id: 0x67b1,
        revision_id: 0x80,
        marketing_name: "AMD Radeon R9 390 Series",
    },
    AsicId {
        device_id: 0x67b9,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 200 Series",
    },
    AsicId {
        device_id: 0x67c0,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX 7100 Graphics",
    },
    AsicId {
        device_id: 0x67c0,
        revision_id: 0x80,
        marketing_name: "AMD Radeon E9550",
    },
    AsicId {
        device_id: 0x67c2,
        revision_id: 0x01,
        marketing_name: "AMD Radeon Pro V7350x2",
    },
    AsicId {
        device_id: 0x67c2,
        revision_id: 0x02,
        marketing_name: "AMD Radeon Pro V7300X",
    },
    AsicId {
        device_id: 0x67c4,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX 7100 Graphics",
    },
    AsicId {
        device_id: 0x67c4,
        revision_id: 0x80,
        marketing_name: "AMD Radeon E9560 / E9565 Graphics",
    },
    AsicId {
        device_id: 0x67c7,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX 5100 Graphics",
    },
    AsicId {
        device_id: 0x67c7,
        revision_id: 0x80,
        marketing_name: "AMD Radeon E9390 Graphics",
    },
    AsicId {
        device_id: 0x67d0,
        revision_id: 0x01,
        marketing_name: "AMD Radeon Pro V7350x2",
    },
    AsicId {
        device_id: 0x67d0,
        revision_id: 0x02,
        marketing_name: "AMD Radeon Pro V7300X",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon Pro 580X",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 580 Series",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon RX 570 Series",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX 580 Series",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon RX 480 Graphics",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xc5,
        marketing_name: "AMD Radeon RX 470 Graphics",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xc6,
        marketing_name: "AMD Radeon RX 570 Series",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon RX 480 Graphics",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon RX 470 Graphics",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xd7,
        marketing_name: "AMD Radeon RX 470 Graphics",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xe0,
        marketing_name: "AMD Radeon RX 470 Series",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xe1,
        marketing_name: "AMD Radeon RX 590 Series",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xe3,
        marketing_name: "AMD Radeon RX Series",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xe7,
        marketing_name: "AMD Radeon RX 580 Series",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xeb,
        marketing_name: "AMD Radeon Pro 580X",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xef,
        marketing_name: "AMD Radeon RX 570 Series",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xf7,
        marketing_name: "AMD Radeon RX P30PH",
    },
    AsicId {
        device_id: 0x67df,
        revision_id: 0xff,
        marketing_name: "AMD Radeon RX 470 Series",
    },
    AsicId {
        device_id: 0x67e0,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX Series",
    },
    AsicId {
        device_id: 0x67e3,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX 4100",
    },
    AsicId {
        device_id: 0x67e8,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX Series",
    },
    AsicId {
        device_id: 0x67e8,
        revision_id: 0x01,
        marketing_name: "AMD Radeon Pro WX Series",
    },
    AsicId {
        device_id: 0x67e8,
        revision_id: 0x80,
        marketing_name: "AMD Radeon E9260 Graphics",
    },
    AsicId {
        device_id: 0x67eb,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro V5300X",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX Graphics",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 460 Graphics",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon Pro Series",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX Series",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xc5,
        marketing_name: "AMD Radeon RX 460 Graphics",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon RX Graphics",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon RX 460 Graphics",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xe0,
        marketing_name: "AMD Radeon RX 560 Series",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xe1,
        marketing_name: "AMD Radeon RX Series",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xe2,
        marketing_name: "AMD Radeon RX 560X",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xe3,
        marketing_name: "AMD Radeon RX Series",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xe5,
        marketing_name: "AMD Radeon RX 560 Series",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xe7,
        marketing_name: "AMD Radeon RX 560 Series",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xef,
        marketing_name: "AMD Radeon 550 Series",
    },
    AsicId {
        device_id: 0x67ef,
        revision_id: 0xff,
        marketing_name: "AMD Radeon RX 460 Graphics",
    },
    AsicId {
        device_id: 0x67ff,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon Pro 465",
    },
    AsicId {
        device_id: 0x67ff,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 560 Series",
    },
    AsicId {
        device_id: 0x67ff,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon RX 560 Series",
    },
    AsicId {
        device_id: 0x67ff,
        revision_id: 0xef,
        marketing_name: "AMD Radeon RX 560 Series",
    },
    AsicId {
        device_id: 0x67ff,
        revision_id: 0xff,
        marketing_name: "AMD Radeon RX 550 Series",
    },
    AsicId {
        device_id: 0x6800,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7970M",
    },
    AsicId {
        device_id: 0x6801,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8970M",
    },
    AsicId {
        device_id: 0x6806,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 M290X",
    },
    AsicId {
        device_id: 0x6808,
        revision_id: 0x00,
        marketing_name: "AMD FirePro W7000",
    },
    AsicId {
        device_id: 0x6808,
        revision_id: 0x00,
        marketing_name: "ATI FirePro V (FireGL V) Graphics Adapter",
    },
    AsicId {
        device_id: 0x6809,
        revision_id: 0x00,
        marketing_name: "ATI FirePro W5000",
    },
    AsicId {
        device_id: 0x6810,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 200 Series",
    },
    AsicId {
        device_id: 0x6810,
        revision_id: 0x81,
        marketing_name: "AMD Radeon R9 370 Series",
    },
    AsicId {
        device_id: 0x6811,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 200 Series",
    },
    AsicId {
        device_id: 0x6811,
        revision_id: 0x81,
        marketing_name: "AMD Radeon R7 370 Series",
    },
    AsicId {
        device_id: 0x6818,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7800 Series",
    },
    AsicId {
        device_id: 0x6819,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7800 Series",
    },
    AsicId {
        device_id: 0x6820,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 M275X",
    },
    AsicId {
        device_id: 0x6820,
        revision_id: 0x81,
        marketing_name: "AMD Radeon R9 M375",
    },
    AsicId {
        device_id: 0x6820,
        revision_id: 0x83,
        marketing_name: "AMD Radeon R9 M375X",
    },
    AsicId {
        device_id: 0x6821,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 M200X Series",
    },
    AsicId {
        device_id: 0x6821,
        revision_id: 0x83,
        marketing_name: "AMD Radeon R9 M370X",
    },
    AsicId {
        device_id: 0x6821,
        revision_id: 0x87,
        marketing_name: "AMD Radeon R7 M380",
    },
    AsicId {
        device_id: 0x6822,
        revision_id: 0x00,
        marketing_name: "AMD Radeon E8860",
    },
    AsicId {
        device_id: 0x6823,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 M200X Series",
    },
    AsicId {
        device_id: 0x6825,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7800M Series",
    },
    AsicId {
        device_id: 0x6826,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7700M Series",
    },
    AsicId {
        device_id: 0x6827,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7800M Series",
    },
    AsicId {
        device_id: 0x6828,
        revision_id: 0x00,
        marketing_name: "AMD FirePro W600",
    },
    AsicId {
        device_id: 0x682b,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8800M Series",
    },
    AsicId {
        device_id: 0x682b,
        revision_id: 0x87,
        marketing_name: "AMD Radeon R9 M360",
    },
    AsicId {
        device_id: 0x682c,
        revision_id: 0x00,
        marketing_name: "AMD FirePro W4100",
    },
    AsicId {
        device_id: 0x682d,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7700M Series",
    },
    AsicId {
        device_id: 0x682f,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7700M Series",
    },
    AsicId {
        device_id: 0x6830,
        revision_id: 0x00,
        marketing_name: "AMD Radeon 7800M Series",
    },
    AsicId {
        device_id: 0x6831,
        revision_id: 0x00,
        marketing_name: "AMD Radeon 7700M Series",
    },
    AsicId {
        device_id: 0x6835,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 Series / HD 9000 Series",
    },
    AsicId {
        device_id: 0x6837,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7700 Series",
    },
    AsicId {
        device_id: 0x683d,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7700 Series",
    },
    AsicId {
        device_id: 0x683f,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 7700 Series",
    },
    AsicId {
        device_id: 0x684c,
        revision_id: 0x00,
        marketing_name: "ATI FirePro V (FireGL V) Graphics Adapter",
    },
    AsicId {
        device_id: 0x6860,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Instinct MI25",
    },
    AsicId {
        device_id: 0x6860,
        revision_id: 0x01,
        marketing_name: "AMD Radeon Instinct MI25",
    },
    AsicId {
        device_id: 0x6860,
        revision_id: 0x02,
        marketing_name: "AMD Radeon Instinct MI25",
    },
    AsicId {
        device_id: 0x6860,
        revision_id: 0x03,
        marketing_name: "AMD Radeon Pro V340",
    },
    AsicId {
        device_id: 0x6860,
        revision_id: 0x04,
        marketing_name: "AMD Radeon Instinct MI25x2",
    },
    AsicId {
        device_id: 0x6860,
        revision_id: 0x07,
        marketing_name: "AMD Radeon Pro V320",
    },
    AsicId {
        device_id: 0x6861,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX 9100",
    },
    AsicId {
        device_id: 0x6862,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro SSG",
    },
    AsicId {
        device_id: 0x6863,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Vega Frontier Edition",
    },
    AsicId {
        device_id: 0x6864,
        revision_id: 0x03,
        marketing_name: "AMD Radeon Pro V340",
    },
    AsicId {
        device_id: 0x6864,
        revision_id: 0x04,
        marketing_name: "AMD Radeon Instinct MI25x2",
    },
    AsicId {
        device_id: 0x6864,
        revision_id: 0x05,
        marketing_name: "AMD Radeon Pro V340",
    },
    AsicId {
        device_id: 0x6868,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX 8200",
    },
    AsicId {
        device_id: 0x686c,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Instinct MI25 MxGPU",
    },
    AsicId {
        device_id: 0x686c,
        revision_id: 0x01,
        marketing_name: "AMD Radeon Instinct MI25 MxGPU",
    },
    AsicId {
        device_id: 0x686c,
        revision_id: 0x02,
        marketing_name: "AMD Radeon Instinct MI25 MxGPU",
    },
    AsicId {
        device_id: 0x686c,
        revision_id: 0x03,
        marketing_name: "AMD Radeon Pro V340 MxGPU",
    },
    AsicId {
        device_id: 0x686c,
        revision_id: 0x04,
        marketing_name: "AMD Radeon Instinct MI25x2 MxGPU",
    },
    AsicId {
        device_id: 0x686c,
        revision_id: 0x05,
        marketing_name: "AMD Radeon Pro V340L MxGPU",
    },
    AsicId {
        device_id: 0x686c,
        revision_id: 0x06,
        marketing_name: "AMD Radeon Instinct MI25 MxGPU",
    },
    AsicId {
        device_id: 0x687f,
        revision_id: 0x01,
        marketing_name: "AMD Radeon RX Vega",
    },
    AsicId {
        device_id: 0x687f,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX Vega",
    },
    AsicId {
        device_id: 0x687f,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX Vega",
    },
    AsicId {
        device_id: 0x687f,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX Vega",
    },
    AsicId {
        device_id: 0x687f,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon RX Vega",
    },
    AsicId {
        device_id: 0x6900,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R7 M260",
    },
    AsicId {
        device_id: 0x6900,
        revision_id: 0x81,
        marketing_name: "AMD Radeon R7 M360",
    },
    AsicId {
        device_id: 0x6900,
        revision_id: 0x83,
        marketing_name: "AMD Radeon R7 M340",
    },
    AsicId {
        device_id: 0x6900,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon R5 M465 Series",
    },
    AsicId {
        device_id: 0x6900,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon R5 M445 Series",
    },
    AsicId {
        device_id: 0x6900,
        revision_id: 0xd1,
        marketing_name: "AMD Radeon 530 Series",
    },
    AsicId {
        device_id: 0x6900,
        revision_id: 0xd3,
        marketing_name: "AMD Radeon 530 Series",
    },
    AsicId {
        device_id: 0x6901,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R5 M255",
    },
    AsicId {
        device_id: 0x6902,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Series",
    },
    AsicId {
        device_id: 0x6907,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R5 M255",
    },
    AsicId {
        device_id: 0x6907,
        revision_id: 0x87,
        marketing_name: "AMD Radeon R5 M315",
    },
    AsicId {
        device_id: 0x6920,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 M395X",
    },
    AsicId {
        device_id: 0x6920,
        revision_id: 0x01,
        marketing_name: "AMD Radeon R9 M390X",
    },
    AsicId {
        device_id: 0x6921,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 M390X",
    },
    AsicId {
        device_id: 0x6929,
        revision_id: 0x00,
        marketing_name: "AMD FirePro S7150",
    },
    AsicId {
        device_id: 0x6929,
        revision_id: 0x01,
        marketing_name: "AMD FirePro S7100X",
    },
    AsicId {
        device_id: 0x692b,
        revision_id: 0x00,
        marketing_name: "AMD FirePro W7100",
    },
    AsicId {
        device_id: 0x6938,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 200 Series",
    },
    AsicId {
        device_id: 0x6938,
        revision_id: 0xf0,
        marketing_name: "AMD Radeon R9 200 Series",
    },
    AsicId {
        device_id: 0x6938,
        revision_id: 0xf1,
        marketing_name: "AMD Radeon R9 380 Series",
    },
    AsicId {
        device_id: 0x6939,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R9 200 Series",
    },
    AsicId {
        device_id: 0x6939,
        revision_id: 0xf0,
        marketing_name: "AMD Radeon R9 200 Series",
    },
    AsicId {
        device_id: 0x6939,
        revision_id: 0xf1,
        marketing_name: "AMD Radeon R9 380 Series",
    },
    AsicId {
        device_id: 0x694c,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX Vega M GH Graphics",
    },
    AsicId {
        device_id: 0x694e,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX Vega M GL Graphics",
    },
    AsicId {
        device_id: 0x6980,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX 3100",
    },
    AsicId {
        device_id: 0x6981,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX 3200 Series",
    },
    AsicId {
        device_id: 0x6981,
        revision_id: 0x01,
        marketing_name: "AMD Radeon Pro WX 3200 Series",
    },
    AsicId {
        device_id: 0x6981,
        revision_id: 0x10,
        marketing_name: "AMD Radeon Pro WX 3200 Series",
    },
    AsicId {
        device_id: 0x6985,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX 3100",
    },
    AsicId {
        device_id: 0x6986,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX 2100",
    },
    AsicId {
        device_id: 0x6987,
        revision_id: 0x80,
        marketing_name: "AMD Embedded Radeon E9171",
    },
    AsicId {
        device_id: 0x6987,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon 550X Series",
    },
    AsicId {
        device_id: 0x6987,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 640",
    },
    AsicId {
        device_id: 0x6987,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon 540X Series",
    },
    AsicId {
        device_id: 0x6987,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon 540",
    },
    AsicId {
        device_id: 0x6995,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX 2100",
    },
    AsicId {
        device_id: 0x6997,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro WX 2100",
    },
    AsicId {
        device_id: 0x699f,
        revision_id: 0x81,
        marketing_name: "AMD Embedded Radeon E9170 Series",
    },
    AsicId {
        device_id: 0x699f,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon 500 Series",
    },
    AsicId {
        device_id: 0x699f,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon 540 Series",
    },
    AsicId {
        device_id: 0x699f,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon 500 Series",
    },
    AsicId {
        device_id: 0x699f,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon RX 550 / 550 Series",
    },
    AsicId {
        device_id: 0x699f,
        revision_id: 0xc9,
        marketing_name: "AMD Radeon 540",
    },
    AsicId {
        device_id: 0x6fdf,
        revision_id: 0xe7,
        marketing_name: "AMD Radeon RX 590 GME",
    },
    AsicId {
        device_id: 0x6fdf,
        revision_id: 0xef,
        marketing_name: "AMD Radeon RX 580 2048SP",
    },
    AsicId {
        device_id: 0x7300,
        revision_id: 0xc1,
        marketing_name: "AMD FirePro S9300 x2",
    },
    AsicId {
        device_id: 0x7300,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon R9 Fury Series",
    },
    AsicId {
        device_id: 0x7300,
        revision_id: 0xc9,
        marketing_name: "AMD Radeon Pro Duo",
    },
    AsicId {
        device_id: 0x7300,
        revision_id: 0xca,
        marketing_name: "AMD Radeon R9 Fury Series",
    },
    AsicId {
        device_id: 0x7300,
        revision_id: 0xcb,
        marketing_name: "AMD Radeon R9 Fury Series",
    },
    AsicId {
        device_id: 0x7312,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W5700",
    },
    AsicId {
        device_id: 0x731e,
        revision_id: 0xc6,
        marketing_name: "AMD Radeon RX 5700XTB",
    },
    AsicId {
        device_id: 0x731e,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon RX 5700B",
    },
    AsicId {
        device_id: 0x731f,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX 5700 XT 50th Anniversary",
    },
    AsicId {
        device_id: 0x731f,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 5700 XT",
    },
    AsicId {
        device_id: 0x731f,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon RX 5600M",
    },
    AsicId {
        device_id: 0x731f,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX 5700M",
    },
    AsicId {
        device_id: 0x731f,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon RX 5700",
    },
    AsicId {
        device_id: 0x731f,
        revision_id: 0xc5,
        marketing_name: "AMD Radeon RX 5700 XT",
    },
    AsicId {
        device_id: 0x731f,
        revision_id: 0xca,
        marketing_name: "AMD Radeon RX 5600 XT",
    },
    AsicId {
        device_id: 0x731f,
        revision_id: 0xcb,
        marketing_name: "AMD Radeon RX 5600 OEM",
    },
    AsicId {
        device_id: 0x7340,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 5500M",
    },
    AsicId {
        device_id: 0x7340,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX 5300M",
    },
    AsicId {
        device_id: 0x7340,
        revision_id: 0xc5,
        marketing_name: "AMD Radeon RX 5500 XT",
    },
    AsicId {
        device_id: 0x7340,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon RX 5500",
    },
    AsicId {
        device_id: 0x7340,
        revision_id: 0xc9,
        marketing_name: "AMD Radeon RX 5500XTB",
    },
    AsicId {
        device_id: 0x7340,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon RX 5300",
    },
    AsicId {
        device_id: 0x7341,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W5500",
    },
    AsicId {
        device_id: 0x7347,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W5500M",
    },
    AsicId {
        device_id: 0x7360,
        revision_id: 0x41,
        marketing_name: "AMD Radeon Pro 5600M",
    },
    AsicId {
        device_id: 0x7360,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon Pro V520",
    },
    AsicId {
        device_id: 0x7362,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon Pro V540",
    },
    AsicId {
        device_id: 0x7362,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon Pro V520",
    },
    AsicId {
        device_id: 0x738c,
        revision_id: 0x01,
        marketing_name: "AMD Instinct MI100",
    },
    AsicId {
        device_id: 0x73a1,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro V620",
    },
    AsicId {
        device_id: 0x73a3,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W6800",
    },
    AsicId {
        device_id: 0x73a5,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX 6950 XT",
    },
    AsicId {
        device_id: 0x73ae,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro V620 MxGPU",
    },
    AsicId {
        device_id: 0x73af,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX 6900 XT",
    },
    AsicId {
        device_id: 0x73bf,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX 6900 XT",
    },
    AsicId {
        device_id: 0x73bf,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 6800 XT",
    },
    AsicId {
        device_id: 0x73bf,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX 6800",
    },
    AsicId {
        device_id: 0x73df,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX 6750 XT",
    },
    AsicId {
        device_id: 0x73df,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 6700 XT",
    },
    AsicId {
        device_id: 0x73df,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon RX 6800M",
    },
    AsicId {
        device_id: 0x73df,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX 6800M",
    },
    AsicId {
        device_id: 0x73df,
        revision_id: 0xc5,
        marketing_name: "AMD Radeon RX 6700 XT",
    },
    AsicId {
        device_id: 0x73df,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon RX 6700M",
    },
    AsicId {
        device_id: 0x73df,
        revision_id: 0xd5,
        marketing_name: "AMD Radeon RX 6750 GRE 12GB",
    },
    AsicId {
        device_id: 0x73df,
        revision_id: 0xd7,
        marketing_name: "AMD TDC-235",
    },
    AsicId {
        device_id: 0x73df,
        revision_id: 0xdf,
        marketing_name: "AMD Radeon RX 6700",
    },
    AsicId {
        device_id: 0x73df,
        revision_id: 0xe5,
        marketing_name: "AMD Radeon RX 6750 GRE 12GB",
    },
    AsicId {
        device_id: 0x73df,
        revision_id: 0xff,
        marketing_name: "AMD Radeon RX 6700",
    },
    AsicId {
        device_id: 0x73e0,
        revision_id: 0x00,
        marketing_name: "AMD Radeon RX 6600M",
    },
    AsicId {
        device_id: 0x73e1,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W6600M",
    },
    AsicId {
        device_id: 0x73e3,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W6600",
    },
    AsicId {
        device_id: 0x73ef,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX 6800S",
    },
    AsicId {
        device_id: 0x73ef,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 6650 XT",
    },
    AsicId {
        device_id: 0x73ef,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon RX 6700S",
    },
    AsicId {
        device_id: 0x73ef,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX 6650M",
    },
    AsicId {
        device_id: 0x73ef,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon RX 6650M XT",
    },
    AsicId {
        device_id: 0x73ff,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 6600 XT",
    },
    AsicId {
        device_id: 0x73ff,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX 6600M",
    },
    AsicId {
        device_id: 0x73ff,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon RX 6600",
    },
    AsicId {
        device_id: 0x73ff,
        revision_id: 0xcb,
        marketing_name: "AMD Radeon RX 6600S",
    },
    AsicId {
        device_id: 0x73ff,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon RX 6600 LE",
    },
    AsicId {
        device_id: 0x73ff,
        revision_id: 0xdf,
        marketing_name: "AMD Radeon RX 6750 GRE 10GB",
    },
    AsicId {
        device_id: 0x7408,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI250X",
    },
    AsicId {
        device_id: 0x740c,
        revision_id: 0x01,
        marketing_name: "AMD Instinct MI250X / MI250",
    },
    AsicId {
        device_id: 0x740f,
        revision_id: 0x02,
        marketing_name: "AMD Instinct MI210",
    },
    AsicId {
        device_id: 0x7421,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W6500M",
    },
    AsicId {
        device_id: 0x7422,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W6400",
    },
    AsicId {
        device_id: 0x7423,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W6300M",
    },
    AsicId {
        device_id: 0x7423,
        revision_id: 0x01,
        marketing_name: "AMD Radeon Pro W6300",
    },
    AsicId {
        device_id: 0x7424,
        revision_id: 0x00,
        marketing_name: "AMD Radeon RX 6300",
    },
    AsicId {
        device_id: 0x743f,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 6500 XT",
    },
    AsicId {
        device_id: 0x743f,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX 6500",
    },
    AsicId {
        device_id: 0x743f,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX 6500M",
    },
    AsicId {
        device_id: 0x743f,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon RX 6400",
    },
    AsicId {
        device_id: 0x743f,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon RX 6500M",
    },
    AsicId {
        device_id: 0x743f,
        revision_id: 0xcc,
        marketing_name: "AMD Radeon 6550S",
    },
    AsicId {
        device_id: 0x743f,
        revision_id: 0xce,
        marketing_name: "AMD Radeon RX 6450M",
    },
    AsicId {
        device_id: 0x743f,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon RX 6300M",
    },
    AsicId {
        device_id: 0x743f,
        revision_id: 0xd3,
        marketing_name: "AMD Radeon RX 6550M",
    },
    AsicId {
        device_id: 0x743f,
        revision_id: 0xd7,
        marketing_name: "AMD Radeon RX 6400",
    },
    AsicId {
        device_id: 0x7448,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W7900",
    },
    AsicId {
        device_id: 0x7449,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W7800 48GB",
    },
    AsicId {
        device_id: 0x744a,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W7900 Dual Slot",
    },
    AsicId {
        device_id: 0x744b,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W7900D",
    },
    AsicId {
        device_id: 0x744c,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon RX 7900 XTX",
    },
    AsicId {
        device_id: 0x744c,
        revision_id: 0xcc,
        marketing_name: "AMD Radeon RX 7900 XT",
    },
    AsicId {
        device_id: 0x744c,
        revision_id: 0xce,
        marketing_name: "AMD Radeon RX 7900 GRE",
    },
    AsicId {
        device_id: 0x744c,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon RX 7900M",
    },
    AsicId {
        device_id: 0x745e,
        revision_id: 0xcc,
        marketing_name: "AMD Radeon Pro W7800",
    },
    AsicId {
        device_id: 0x7460,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro V710",
    },
    AsicId {
        device_id: 0x7461,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro V710 MxGPU",
    },
    AsicId {
        device_id: 0x7470,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W7700",
    },
    AsicId {
        device_id: 0x747e,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon RX 7800 XT",
    },
    AsicId {
        device_id: 0x747e,
        revision_id: 0xd8,
        marketing_name: "AMD Radeon RX 7800M",
    },
    AsicId {
        device_id: 0x747e,
        revision_id: 0xdb,
        marketing_name: "AMD Radeon RX 7700",
    },
    AsicId {
        device_id: 0x747e,
        revision_id: 0xff,
        marketing_name: "AMD Radeon RX 7700 XT",
    },
    AsicId {
        device_id: 0x7480,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W7600",
    },
    AsicId {
        device_id: 0x7480,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX 7600 XT",
    },
    AsicId {
        device_id: 0x7480,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 7700S",
    },
    AsicId {
        device_id: 0x7480,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon RX 7650 GRE",
    },
    AsicId {
        device_id: 0x7480,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX 7600S",
    },
    AsicId {
        device_id: 0x7480,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon RX 7600M XT",
    },
    AsicId {
        device_id: 0x7480,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon RX 7600",
    },
    AsicId {
        device_id: 0x7481,
        revision_id: 0xc7,
        marketing_name: "AMD Steam Machine",
    },
    AsicId {
        device_id: 0x7483,
        revision_id: 0xcf,
        marketing_name: "AMD Radeon RX 7600M",
    },
    AsicId {
        device_id: 0x7489,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W7500",
    },
    AsicId {
        device_id: 0x7499,
        revision_id: 0x00,
        marketing_name: "AMD Radeon Pro W7400",
    },
    AsicId {
        device_id: 0x7499,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX 7400",
    },
    AsicId {
        device_id: 0x7499,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 7300",
    },
    AsicId {
        device_id: 0x74a0,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI300A",
    },
    AsicId {
        device_id: 0x74a1,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI300X",
    },
    AsicId {
        device_id: 0x74a2,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI308X",
    },
    AsicId {
        device_id: 0x74a5,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI325X",
    },
    AsicId {
        device_id: 0x74a8,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI308X HF",
    },
    AsicId {
        device_id: 0x74a9,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI300X HF",
    },
    AsicId {
        device_id: 0x74b5,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI300X VF",
    },
    AsicId {
        device_id: 0x74b6,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI308X",
    },
    AsicId {
        device_id: 0x74bd,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI300X HF",
    },
    AsicId {
        device_id: 0x7550,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX 9070 XT",
    },
    AsicId {
        device_id: 0x7550,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon RX 9070 GRE",
    },
    AsicId {
        device_id: 0x7550,
        revision_id: 0xc3,
        marketing_name: "AMD Radeon RX 9070",
    },
    AsicId {
        device_id: 0x7551,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon AI PRO R9700",
    },
    AsicId {
        device_id: 0x7551,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon AI PRO R9600D",
    },
    AsicId {
        device_id: 0x7590,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon RX 9060 XT",
    },
    AsicId {
        device_id: 0x7590,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon RX 9060 XT LP",
    },
    AsicId {
        device_id: 0x7590,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon RX 9060",
    },
    AsicId {
        device_id: 0x75a0,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI350X",
    },
    AsicId {
        device_id: 0x75a3,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI355X",
    },
    AsicId {
        device_id: 0x75b0,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI350X VF",
    },
    AsicId {
        device_id: 0x75b3,
        revision_id: 0x00,
        marketing_name: "AMD Instinct MI355X VF",
    },
    AsicId {
        device_id: 0x9830,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8400 / R3 Series",
    },
    AsicId {
        device_id: 0x9831,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8400E",
    },
    AsicId {
        device_id: 0x9832,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8330",
    },
    AsicId {
        device_id: 0x9833,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8330E",
    },
    AsicId {
        device_id: 0x9834,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8210",
    },
    AsicId {
        device_id: 0x9835,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8210E",
    },
    AsicId {
        device_id: 0x9836,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8200 / R3 Series",
    },
    AsicId {
        device_id: 0x9837,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8280E",
    },
    AsicId {
        device_id: 0x9838,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8200 / R3 series",
    },
    AsicId {
        device_id: 0x9839,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8180",
    },
    AsicId {
        device_id: 0x983d,
        revision_id: 0x00,
        marketing_name: "AMD Radeon HD 8250",
    },
    AsicId {
        device_id: 0x9850,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R3 Graphics",
    },
    AsicId {
        device_id: 0x9850,
        revision_id: 0x03,
        marketing_name: "AMD Radeon R3 Graphics",
    },
    AsicId {
        device_id: 0x9850,
        revision_id: 0x40,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x9850,
        revision_id: 0x45,
        marketing_name: "AMD Radeon R3 Graphics",
    },
    AsicId {
        device_id: 0x9851,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R4 Graphics",
    },
    AsicId {
        device_id: 0x9851,
        revision_id: 0x01,
        marketing_name: "AMD Radeon R5E Graphics",
    },
    AsicId {
        device_id: 0x9851,
        revision_id: 0x05,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x9851,
        revision_id: 0x06,
        marketing_name: "AMD Radeon R5E Graphics",
    },
    AsicId {
        device_id: 0x9851,
        revision_id: 0x40,
        marketing_name: "AMD Radeon R4 Graphics",
    },
    AsicId {
        device_id: 0x9851,
        revision_id: 0x45,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x9852,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x9852,
        revision_id: 0x40,
        marketing_name: "AMD Radeon E1 Graphics",
    },
    AsicId {
        device_id: 0x9853,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x9853,
        revision_id: 0x01,
        marketing_name: "AMD Radeon R4E Graphics",
    },
    AsicId {
        device_id: 0x9853,
        revision_id: 0x03,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x9853,
        revision_id: 0x05,
        marketing_name: "AMD Radeon R1E Graphics",
    },
    AsicId {
        device_id: 0x9853,
        revision_id: 0x06,
        marketing_name: "AMD Radeon R1E Graphics",
    },
    AsicId {
        device_id: 0x9853,
        revision_id: 0x07,
        marketing_name: "AMD Radeon R1E Graphics",
    },
    AsicId {
        device_id: 0x9853,
        revision_id: 0x08,
        marketing_name: "AMD Radeon R1E Graphics",
    },
    AsicId {
        device_id: 0x9853,
        revision_id: 0x40,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x9854,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R3 Graphics",
    },
    AsicId {
        device_id: 0x9854,
        revision_id: 0x01,
        marketing_name: "AMD Radeon R3E Graphics",
    },
    AsicId {
        device_id: 0x9854,
        revision_id: 0x02,
        marketing_name: "AMD Radeon R3 Graphics",
    },
    AsicId {
        device_id: 0x9854,
        revision_id: 0x05,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x9854,
        revision_id: 0x06,
        marketing_name: "AMD Radeon R4 Graphics",
    },
    AsicId {
        device_id: 0x9854,
        revision_id: 0x07,
        marketing_name: "AMD Radeon R3 Graphics",
    },
    AsicId {
        device_id: 0x9855,
        revision_id: 0x02,
        marketing_name: "AMD Radeon R6 Graphics",
    },
    AsicId {
        device_id: 0x9855,
        revision_id: 0x05,
        marketing_name: "AMD Radeon R4 Graphics",
    },
    AsicId {
        device_id: 0x9856,
        revision_id: 0x00,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x9856,
        revision_id: 0x01,
        marketing_name: "AMD Radeon R2E Graphics",
    },
    AsicId {
        device_id: 0x9856,
        revision_id: 0x02,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x9856,
        revision_id: 0x05,
        marketing_name: "AMD Radeon R1E Graphics",
    },
    AsicId {
        device_id: 0x9856,
        revision_id: 0x06,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x9856,
        revision_id: 0x07,
        marketing_name: "AMD Radeon R1E Graphics",
    },
    AsicId {
        device_id: 0x9856,
        revision_id: 0x08,
        marketing_name: "AMD Radeon R1E Graphics",
    },
    AsicId {
        device_id: 0x9856,
        revision_id: 0x13,
        marketing_name: "AMD Radeon R1E Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0x81,
        marketing_name: "AMD Radeon R6 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0x84,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0x85,
        marketing_name: "AMD Radeon R6 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0x87,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0x88,
        marketing_name: "AMD Radeon R7E Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0x89,
        marketing_name: "AMD Radeon R6E Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xc5,
        marketing_name: "AMD Radeon R6 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xc6,
        marketing_name: "AMD Radeon R6 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xc7,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xc9,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xca,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xcb,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xcc,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xcd,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xce,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xe1,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xe2,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xe3,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xe4,
        marketing_name: "AMD Radeon R7 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xe5,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x9874,
        revision_id: 0xe6,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0x80,
        marketing_name: "AMD Radeon R5E Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0x81,
        marketing_name: "AMD Radeon R4E Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0x83,
        marketing_name: "AMD Radeon R2E Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0x84,
        marketing_name: "AMD Radeon R2E Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0x86,
        marketing_name: "AMD Radeon R1E Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xc0,
        marketing_name: "AMD Radeon R4 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xc1,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xc2,
        marketing_name: "AMD Radeon R4 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xc4,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xc6,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xc8,
        marketing_name: "AMD Radeon R4 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xc9,
        marketing_name: "AMD Radeon R4 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xca,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xd0,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xd1,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xd2,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xd4,
        marketing_name: "AMD Radeon R2 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xd9,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xda,
        marketing_name: "AMD Radeon R5 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xdb,
        marketing_name: "AMD Radeon R3 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xe1,
        marketing_name: "AMD Radeon R3 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xe2,
        marketing_name: "AMD Radeon R3 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xe9,
        marketing_name: "AMD Radeon R4 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xea,
        marketing_name: "AMD Radeon R4 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xeb,
        marketing_name: "AMD Radeon R3 Graphics",
    },
    AsicId {
        device_id: 0x98e4,
        revision_id: 0xeb,
        marketing_name: "AMD Radeon R4 Graphics",
    },
];
