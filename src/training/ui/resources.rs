// Maybe needs a vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResVec2 {
    x: f32,
    y: f32
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResVec3 {
    x: f32,
    y: f32,
    z: f32
}

impl ResVec3 {
    pub fn default() -> ResVec3 {
        ResVec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new(x: f32, y: f32, z: f32) -> ResVec3 {
        ResVec3 { x, y, z }
    }
}

// Maybe needs a vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResPane {
    block_header_kind: u32,
    block_header_size: u32,
    flag: u8,
    base_position: u8,
    alpha: u8,
    flag_ex: u8,
    pub name: [skyline::libc::c_char; 24],
    pub user_data: [skyline::libc::c_char; 8],
    pub pos: ResVec3,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub size_x: f32,
    pub size_y: f32,
}

impl ResPane {
    // For null pane
    pub fn new(name: &str) -> ResPane {
        let mut pane = ResPane {
            block_header_kind: u32::from_le_bytes([b'p', b'a', b'n', b'1']),
            block_header_size: 84,
            /// Visible | InfluencedAlpha
            flag: 0x3,
            base_position: 0,
            alpha: 0xFF,
            flag_ex: 0,
            name: [0; 24],
            user_data: [0; 8],
            pos: ResVec3{x: 0.0, y: 0.0, z: 0.0},
            rot_x: 0.0,
            rot_y: 0.0,
            rot_z: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            size_x: 30.0,
            size_y: 40.0,
        };
        pane.set_name(name);
        pane
    }

    pub fn set_name(&mut self, name: &str) {
        assert!(name.len() <= 24, "Name of pane must be at most 24 characters");
        unsafe {
            std::ptr::copy_nonoverlapping(name.as_ptr(), self.name.as_mut_ptr(), name.len());
        }
    }

    pub fn set_pos(&mut self, pos: ResVec3) {
        self.pos = pos;
    }

    pub fn name_matches(&self, other: &str) -> bool {
        self.name.iter().take_while(|b| **b != 0).map(|b| *b as char).collect::<String>() == other
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResTextBox {
    pub pane: ResPane,
    text_buf_bytes: u16,
    text_str_bytes: u16,
    material_idx: u16,
    font_idx: u16,
    text_position: u8,
    text_alignment: u8,
    text_box_flag: u16,
    italic_ratio: f32,
    text_str_offset: u32,
    text_cols: [ResColor; 2],
    font_size: ResVec2,
    char_space: f32,
    line_space: f32,
    text_id_offset: u32,
    shadow_offset: ResVec2,
    shadow_scale: ResVec2,
    shadow_cols: [ResColor; 2],
    shadow_italic_ratio: f32,
    line_width_offset_offset: u32,
    per_character_transform_offset: u32,

/* Additional Info
    uint16_t           text[];                     // Text.
    char                textId[];                   // The text ID.
    u8 lineWidthOffsetCount; // The quantity of widths and offsets for each line.
    float lineOffset[]; // The offset for each line.
    float lineWidth[]; // The width of each line.
    ResPerCharacterTransform perCharacterTransform     // Information for per-character animation.
    ResAnimationInfo       perCharacterTransformAnimationInfo;     // Animation information for per-character animation.
*/
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResPicture {
    pub pane: ResPane,
    vtx_cols: [ResColor; 4],
    material_idx: u16,
    tex_coord_count: u8,
    flags: u8,
/* Additional Info
    ResVec2 texCoords[texCoordCount][VERTEX_MAX];
    uint32_t shapeBinaryIndex;
*/
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResPictureWithTex<const TEX_COORD_COUNT : usize> {
    pub picture: ResPicture,
    tex_coords:[[ResVec2; TEX_COORD_COUNT]; 4]
}