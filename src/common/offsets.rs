

// #[cfg(not(feature = "no_offset_search"))]
// mod offsets_inner {
//     pub static OFFSET_GET_BATTLE_OBJECT_FROM_ID: usize = 0x3ac540;
//     pub static OFFSET_COPY_SETUP: usize = 0xba0e60;
//     pub static OFFSET_IS_VISIBLE_BACKSHIELD: usize = 0x1655400;
//     pub static OFFSET_SET_CPU_CONTROLS: usize = 0x2da180;
//     pub static OFFSET_ADD_DAMAGE: usize = 0x3ff9a0;
//     pub static OFFSET_FIGHTER_REQ_QUAKE_POS: usize = 0x3ec820;
//     pub static OFFSET_CHANGE_ACTIVE_CAMERA: usize = 0x4ee460;
//     pub static OFFSET_SET_TRAINING_FIXED_CAMERA_VALUES: usize = 0x3157bb0;
//     pub static OFFSET_DRAW: usize = 0x4b620;

//     pub static OFFSET_ON_FLAG: usize = 0x4e4910;
//     pub static OFFSET_SET_INT: usize = 0x4e4600;
//     pub static OFFSET_SET_INT64: usize = 0x4e4680;
//     pub static OFFSET_SET_FLOAT: usize = 0x4e4420;
//     pub static OFFSET_IS_FLAG: usize = 0x4e48e0;
//     pub static OFFSET_GET_INT: usize = 0x4e45e0;
//     pub static OFFSET_CLOUD_ADD_LIMIT: usize = 0x008dc140;
//     pub static OFFSET_STALE: usize = 0x013e88a4;
//     pub static OFFSET_STALE_MENU: usize = 0x013e88a0;
//     pub static OFFSET_PLAY_SE: usize = 0x04cf6a0;
//     pub static OFFSET_FOLLOW_REQ: usize = 0x044f860;
//     pub static OFFSET_EFFECT_REQ: usize = 0x44de50;
//     pub static OFFSET_JOINT_EFFECT_REQ: usize = 0x44e1e0;
//     pub static OFFSET_CAN_FUTTOBI_BACK: usize = 0x0260f950;
//     pub static OFFSET_REUSED_UI: usize = 0x068cd80;
//     pub static OFFSET_ARTICLE_GET_INT: usize = 0x3d5920;
//     pub static OFFSET_OPCF: usize = 0x06b7fdc;
//     pub static OFFSET_FIM: usize = 0x17504a0;
//     pub static OFFSET_SSS_TRAINING: usize = 0x184d1d8;
//     pub static OFFSET_GAFT: usize = 0x03d40a0;
//     pub static OFFSET_KIRBY_OPFF: usize = 0xb971b0;
//     pub static OFFSET_ACTIVATE_AUTONOMY: usize = 0x034b5cf0;
//     pub static OFFSET_POKEMON_DECIDE: usize = 0x34cdc64;
//     pub static OFFSET_LAYOUT_ARC_MALLOC: usize = 0x37730d4;

// }
// #[cfg(not(feature = "no_offset_search"))]
mod offsets_inner {
    use crate::logging::*;
    static NEEDLE_GET_BATTLE_OBJECT_FROM_ID: &[u8] = &[
        0xff, 0x03, 0x06, 0xd1,
        0xee, 0x73, 0x00, 0xfd,
        0xed, 0x33, 0x0f, 0x6d,
        0xeb, 0x2b, 0x10, 0x6d,
        0xe9, 0x23, 0x11, 0x6d,
        0xfc, 0x6f, 0x12, 0xa9,
        0xfa, 0x67, 0x13, 0xa9,
        0xf8, 0x5f, 0x14, 0xa9,
    ];
    static NEEDLE_COPY_SETUP: &[u8] = &[
        0xe8, 0x0f, 0x19, 0xfc,
        0xfc, 0x6f, 0x01, 0xa9,
        0xfa, 0x67, 0x02, 0xa9,
        0xf8, 0x5f, 0x03, 0xa9,
        0xf6, 0x57, 0x04, 0xa9,
        0xf4, 0x4f, 0x05, 0xa9,
        0xfd, 0x7b, 0x06, 0xa9,
        0xfd, 0x83, 0x01, 0x91,
    ];
    static NEEDLE_IS_VISIBLE_BACKSHIELD: &[u8] = &[
        0xfd, 0x7b, 0xbf, 0xa9,
        0xfd, 0x03, 0x00, 0x91,
        0x00, 0x28, 0x40, 0xf9,
        0x08, 0x00, 0x40, 0xf9,
        0x08, 0x85, 0x40, 0xf9,
        0xa1, 0x0f, 0x80, 0x52,
        0x01, 0x00, 0xa4, 0x72,
        0x00, 0x01, 0x3f, 0xd6,
    ];
    static NEEDLE_SET_CPU_CONTROLS: &[u8] = &[
        0xff, 0x03, 0x06, 0xd1,
        0xee, 0x73, 0x00, 0xfd,
        0xed, 0x33, 0x0f, 0x6d,
        0xeb, 0x2b, 0x10, 0x6d,
        0xe9, 0x23, 0x11, 0x6d,
        0xfc, 0x6f, 0x12, 0xa9,
        0xfa, 0x67, 0x13, 0xa9,
        0xf8, 0x5f, 0x14, 0xa9,
    ];
    static NEEDLE_ADD_DAMAGE: &[u8] = &[
        0x08, 0x20, 0x20, 0x1e,
        0x8d, 0x00, 0x00, 0x54,
        0x08, 0x14, 0x4e, 0x39,
        0x48, 0x00, 0x00, 0x34,
        0xc0, 0x03, 0x5f, 0xd6,
    ];
    static NEEDLE_FIGHTER_REQ_QUAKE_POS: &[u8] = &[
        0x08, 0x64, 0x40, 0x39,
        0xe8, 0x03, 0x00, 0x34,
        0x28, 0x04, 0x00, 0x51,
        0x1f, 0x1d, 0x00, 0x71,
        0x68, 0x01, 0x00, 0x54,
        0x49, 0x06, 0x02, 0xd0,
        0x29, 0x31, 0x1f, 0x91,
        0x28, 0x79, 0xa8, 0xb8,
    ];
    static NEEDLE_CHANGE_ACTIVE_CAMERA: &[u8] = &[
        0xff, 0x03, 0x02, 0xd1,
        0xf8, 0x5f, 0x04, 0xa9,
        0xf6, 0x57, 0x05, 0xa9,
        0xf4, 0x4f, 0x06, 0xa9,
        0xfd, 0x7b, 0x07, 0xa9,
        0xfd, 0xc3, 0x01, 0x91,
        0x08, 0x04, 0x40, 0xb9,
        0x1f, 0x01, 0x01, 0x6b,
    ];
    static NEEDLE_SET_TRAINING_FIXED_CAMERA_VALUES: &[u8] = &[
        0x01, 0xe4, 0x00, 0x2f,
        0x20, 0x00, 0xc0, 0x3d,
        0x22, 0x1c, 0xa1, 0x4e,
        0x02, 0x44, 0x04, 0x6e,
        0xe8, 0x0a, 0x01, 0xf0,
        0x08, 0x81, 0x47, 0xf9,
        0x08, 0x01, 0x40, 0xf9,
        0x40, 0x04, 0x18, 0x6e,
        0x00, 0xf5, 0x82, 0x3d,
    ];
    static NEEDLE_DRAW: &[u8] = &[
        0x08, 0x0c, 0x40, 0xf9,
        0xc8, 0x03, 0x00, 0xb4,
        0xff, 0x83, 0x01, 0xd1,
        0xf5, 0x1b, 0x00, 0xf9,
        0xf4, 0x4f, 0x04, 0xa9,
        0xfd, 0x7b, 0x05, 0xa9,
        0xfd, 0x43, 0x01, 0x91,
        0xf4, 0x03, 0x00, 0xaa,
    ];

    // Stolen from HDR who stole it from Arcropolis
    // https://github.com/HDR-Development/HewDraw-Remix/blob/dev/dynamic/src/util.rs
    pub fn byte_search<T: Eq>(needle: &[T]) -> Option<usize> {
        let text = unsafe {
            let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const T;
            let end = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const T;
            let length = end.offset_from(start) as usize;
            std::slice::from_raw_parts(start, length)
        };
    
        text.windows(needle.len()).position(|window| window == needle)
    }

    fn find_offset(name: &str, needle: &[u8]) -> Option<usize> {
        info!("Searching for {}", name);
        let offset_opt = byte_search(needle);
        match offset_opt {
            Some(offset) => {
                info!("Found offset for {} at {:#x}", name, offset);
                Some(offset)
            },
            None => {
                error!("Cound not find offset for {}", name);
                None
            }
        }
    }

    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref OFFSET_GET_BATTLE_OBJECT_FROM_ID: usize = find_offset("GET_BATTLE_OBJECT_FROM_ID", NEEDLE_GET_BATTLE_OBJECT_FROM_ID).expect("Failed to find offset for GET_BATTLE_OBJECT_FROM_ID!");
        pub static ref OFFSET_COPY_SETUP: usize = find_offset("COPY_SETUP", NEEDLE_COPY_SETUP).expect("Failed to find offset for COPY_SETUP!");
        pub static ref OFFSET_IS_VISIBLE_BACKSHIELD: usize = find_offset("IS_VISIBLE_BACKSHIELD", NEEDLE_IS_VISIBLE_BACKSHIELD).expect("Failed to find offset for IS_VISIBLE_BACKSHIELD!");
        pub static ref OFFSET_SET_CPU_CONTROLS: usize = find_offset("SET_CPU_CONTROLS", NEEDLE_SET_CPU_CONTROLS).expect("Failed to find offset for SET_CPU_CONTROLS!");
        pub static ref OFFSET_ADD_DAMAGE: usize = find_offset("ADD_DAMAGE", NEEDLE_ADD_DAMAGE).expect("Failed to find offset for ADD_DAMAGE!");
        pub static ref OFFSET_FIGHTER_REQ_QUAKE_POS: usize = find_offset("REQ_QUAKE_POS", NEEDLE_FIGHTER_REQ_QUAKE_POS).expect("Failed to find offset for FIGHTER_REQ_QUAKE_POS!");
        pub static ref OFFSET_CHANGE_ACTIVE_CAMERA: usize = find_offset("CHANGE_ACTIVE_CAMERA", NEEDLE_CHANGE_ACTIVE_CAMERA).expect("Failed to find offset for CHANGE_ACTIVE_CAMERA:!");
        pub static ref OFFSET_SET_TRAINING_FIXED_CAMERA_VALUES: usize = find_offset("WSET_TRAINING_FIXED_CAMERA_VALUES", NEEDLE_SET_TRAINING_FIXED_CAMERA_VALUES).expect("Failed to find offset for SET_TRAINING_FIXED_CAMERA_VALUES:!");
        pub static ref OFFSET_DRAW: usize = find_offset("DRAW", NEEDLE_DRAW).expect("Failed to find offset for DRAW!");
    }

    pub static OFFSET_ON_FLAG: &usize = &0x4e4910;
    pub static OFFSET_SET_INT: &usize = &0x4e4600;
    pub static OFFSET_SET_INT64: &usize = &0x4e4680;
    pub static OFFSET_SET_FLOAT: &usize = &0x4e4420;
    pub static OFFSET_IS_FLAG: &usize = &0x4e48e0;
    pub static OFFSET_GET_INT: &usize = &0x4e45e0;
    pub static OFFSET_CLOUD_ADD_LIMIT: &usize = &0x008dc140;
    pub static OFFSET_STALE: &usize = &0x013e88a4;
    pub static OFFSET_STALE_MENU: &usize = &0x013e88a0;
    pub static OFFSET_PLAY_SE: &usize = &0x04cf6a0;
    pub static OFFSET_FOLLOW_REQ: &usize = &0x044f860;
    pub static OFFSET_EFFECT_REQ: &usize = &0x44de50;
    pub static OFFSET_JOINT_EFFECT_REQ: &usize = &0x44e1e0;
    pub static OFFSET_CAN_FUTTOBI_BACK: &usize = &0x0260f950;
    pub static OFFSET_REUSED_UI: &usize = &0x068cd80;
    pub static OFFSET_ARTICLE_GET_INT: &usize = &0x3d5920;
    pub static OFFSET_OPCF: &usize = &0x06b7fdc;
    pub static OFFSET_FIM: &usize = &0x17504a0;
    pub static OFFSET_SSS_TRAINING: &usize = &0x184d1d8;
    pub static OFFSET_GAFT: &usize = &0x03d40a0;
    pub static OFFSET_KIRBY_OPFF: &usize = &0xb971b0;
    pub static OFFSET_ACTIVATE_AUTONOMY: &usize = &0x034b5cf0;
    pub static OFFSET_POKEMON_DECIDE: &usize = &0x34cdc64;
    pub static OFFSET_LAYOUT_ARC_MALLOC: &usize = &0x37730d4;
}

pub use offsets_inner::*;
