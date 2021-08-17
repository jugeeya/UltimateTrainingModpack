use smash::phx::{Vector3f, Hash40};
use smash::app;

pub static RAYGUN_LENGTH : f32 = 8.0;
pub static RAYGUN_HEIGHT : f32 = 6.0;
pub static RAYGUN_HORIZ_OFFSET : f32 = 2.0;

/*
    segment data list : {Z, Y, X, ZRot, Size}
    segment labels :
         _
        |_| from top to top left, clockwise: a->f + g mid +  \|/ from top mid to top left, clockwise: h->m + --two half g's: n, o
        |_|                                                  /|\
*/

pub static SEGMENT_DICT: [[f32; 5]; 15] = [
        [0.0, RAYGUN_HEIGHT*2.0,   0.0,                    0.0, 0.25], // a
        [0.0, RAYGUN_HEIGHT,     RAYGUN_LENGTH,       90.0, 0.25], // b
        [0.0, 0.0,                 RAYGUN_LENGTH,       90.0, 0.25], // c
        [0.0, 0.0,                 0.0,                    0.0, 0.25], // d
        [0.0, 0.0,                 0.0,                   90.0, 0.25], // e
        [0.0, RAYGUN_HEIGHT,     0.0,                   90.0, 0.25], // f
        [0.0, RAYGUN_HEIGHT,     0.0,                    0.0, 0.25], // g mid
        [0.0, RAYGUN_HEIGHT,     RAYGUN_LENGTH/2.0,     90.0, 0.25], // h
        [0.0, RAYGUN_HEIGHT,     RAYGUN_LENGTH/2.0,     52.0, 0.2],  // i
        [0.0, RAYGUN_HEIGHT,     RAYGUN_LENGTH/2.0,    -52.0, 0.2],  // j
        [0.0, 0.0,                 RAYGUN_LENGTH/2.0,     90.0, 0.25], // k
        [0.0, RAYGUN_HEIGHT/2.0,   RAYGUN_LENGTH*3.0/16.0,  52.0, 0.2],  // l
        [0.0, RAYGUN_HEIGHT*3.0/2.0, RAYGUN_LENGTH*3.0/16.0, -52.0, 0.2],  // m
        [0.0, RAYGUN_HEIGHT,     0.0,                    0.0, 0.15], // n
        [0.0, RAYGUN_HEIGHT,     RAYGUN_LENGTH/2.0,      0.0, 0.15], // o
    ];

/* 
    Segments making up each character, each index corresponding to:
    'A' through 'Z', '0' through '9', ' ', '-', '+', '#' (where '#' is all segments)
*/
pub static ALPHABET: [&str; 40] = [   
    "abcefg",
    "adefijn",
    "adef",
    "eflm",
    "adefn",
    "aefn",
    "acdefo",
    "bcefg",
    "adhk",
    "bcd",
    "efnij",
    "def",
    "bcefim",
    "bcefjm",
    "abcdef",
    "abefg",
    "abcdefj",
    "aefijn",
    "acdfg",
    "ahk",
    "bcdef",
    "efil",
    "bcefjl",
    "ijlm",
    "ikm",
    "adil",
    "abcdef",
    "ef",
    "abdeg",
    "abcdg",
    "bcfg",
    "acdfg",
    "acdefg",
    "abc",
    "abcdefg",
    "abcdfg",
    "",
    "g",
    "ghk",
    "abcdefhijklmno",
];

// Each index is a segment's corresponding flipped segment, for when facing left
pub static SEGMENT_REV: [char; 15] = [
    'a',
    'f',
    'e',
    'd',
    'c',
    'b',
    'g',
    'h',
    'm',
    'l',
    'k',
    'j',
    'i',
    'o',
    'n',
];

fn show_segment(module_accessor: &mut app::BattleObjectModuleAccessor, z: f32, y: f32, x: f32, zrot: f32, size: f32) {
    let pos = Vector3f{x, y, z};
    let rot = Vector3f{x : 0.0, y : 90.0, z : zrot};
    let random = Vector3f{x : 0.0, y : 0.0, z : 0.0};

    unsafe {
        app::lua_bind::EffectModule::req_on_joint(module_accessor, 
            Hash40::new("sys_raygun_bullet"), Hash40::new("top"), 
            &pos, &rot, size, &random, &random, 
            false, 0, 0, 0);
    }
}

fn alphabet_index(to_print: char) -> i32 {
    match to_print {
        'A'..='Z' => to_print as i32 - 'A' as i32,
        '0'..='9' => to_print as i32 - '0' as i32 + 'Z' as i32 - 'A' as i32 + 1,
        ' ' => 36,
        '-' => 37,
        '+' => 38,
        '#' => 39,
        _ => -1
    }
}

fn print_char(module_accessor: &mut app::BattleObjectModuleAccessor,
    to_print: char, 
    line_num: i32, 
    horiz_offset: f32,
    facing_left: i32) 
{
    let alph_index = alphabet_index(to_print);
    if !(0..40).contains(&alph_index) {
        return;
    }
    let segment_str = ALPHABET[alph_index as usize];

    let line_offset = 40.0 - ((line_num as f32) * 16.0);

    for segment_char in  segment_str.chars() {
        let mut index = segment_char as i32 - 'a' as i32;

        let segment: [f32; 5];
        if facing_left == -1 {
            index = SEGMENT_REV[index as usize] as i32 - 'a' as i32;
        }
        segment = SEGMENT_DICT[index as usize];

        let size_mult : f32 = 0.5;

        let mut z = segment[0];
        let mut y = segment[1] + line_offset;
        let mut x = segment[2] + horiz_offset;
        let mut zrot = segment[3];

        if facing_left == -1 {
            zrot *= -1.0;
        }

        let mut size = segment[4];

        x *= size_mult;
        x += facing_left as f32 * 5.0;
        y *= size_mult;
        y += 5.0;
        z *= size_mult;
        size *= size_mult;
        show_segment(module_accessor, z, y, x, zrot, size);
    }
}

pub fn print_string(module_accessor: &mut app::BattleObjectModuleAccessor, to_write: &str) {
    // Delete any previous strings
    unsafe {
        app::lua_bind::EffectModule::kill_kind(module_accessor, Hash40::new("sys_raygun_bullet"), false, true);
    }

    let mut line_num = 0;
    let mut horiz_offset = 0.0;
    let mut char_num = 0;

    let facing_left: i32;
    unsafe {
        facing_left = app::lua_bind::PostureModule::lr(module_accessor) as i32;
    }

    if to_write.len() <= 8 && !to_write.contains('\n') {
        line_num = 1;
    }
    for curr_char in to_write.chars() {
        if curr_char == '\n' {
            horiz_offset = 0.0;
            char_num = 0;
            line_num += 1;
            continue;
        }

        print_char(module_accessor, curr_char.to_uppercase().collect::<Vec<_>>()[0], line_num, horiz_offset, facing_left);

        char_num += 1;
        // short characters
        if curr_char == 'D' || curr_char == '1' {
            horiz_offset += facing_left as f32 * (RAYGUN_LENGTH/2.0 + 3.0);
        } else {
            horiz_offset += facing_left as f32 * (RAYGUN_LENGTH + 3.0);
        }

        if char_num > 8 {
            horiz_offset = 0.0;
            char_num = 0;
            line_num += 1;
        }
    }
}
