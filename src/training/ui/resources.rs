// Maybe needs a vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResVec2 {
    x: f32,
    y: f32,
}

impl ResVec2 {
    pub fn default() -> ResVec2 {
        ResVec2 {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn new(x: f32, y: f32) -> ResVec2 {
        ResVec2 { x, y}
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ResVec3 {
    pub fn default() -> ResVec3 {
        ResVec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
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
    a: u8,
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
            pos: ResVec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
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
        assert!(
            name.len() <= 24,
            "Name of pane must be at most 24 characters"
        );
        unsafe {
            std::ptr::copy_nonoverlapping(name.as_ptr(), self.name.as_mut_ptr(), name.len());
        }
    }

    pub fn set_pos(&mut self, pos: ResVec3) {
        self.pos = pos;
    }

    pub fn set_size(&mut self, size: ResVec2) {
        self.size_x = size.x;
        self.size_y = size.y;
    }

    pub fn name_matches(&self, other: &str) -> bool {
        self.name
            .iter()
            .take_while(|b| **b != 0)
            .map(|b| *b as char)
            .collect::<String>()
            == other
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
enum TextBoxFlag
{
    ShadowEnabled,
    ForceAssignTextLength,
    InvisibleBorderEnabled,
    DoubleDrawnBorderEnabled,
    PerCharacterTransformEnabled,
    CenterCeilingEnabled,
    LineWidthOffsetEnabled,
    ExtendedTagEnabled,
    PerCharacterTransformSplitByCharWidth,
    PerCharacterTransformAutoShadowAlpha,
    DrawFromRightToLeft,
    PerCharacterTransformOriginToCenter,
    KeepingFontScaleEnabled,
    PerCharacterTransformFixSpace,
    PerCharacterTransformSplitByCharWidthInsertSpaceEnabled,
    MaxTextBoxFlag,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum TextAlignment {
    Synchronous,
    Left,
    Center,
    Right,
    MaxTextAlignment
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResTextBox {
    pub pane: ResPane,
    text_buf_bytes: u16,
    text_str_bytes: u16,
    material_idx: u16,
    pub font_idx: u16,
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

impl ResTextBox {
    pub fn enable_shadow(&mut self) {
        self.text_box_flag |= 0x1 << TextBoxFlag::ShadowEnabled as u8;
    }

    pub fn text_alignment(&mut self, align: TextAlignment) {
        self.text_alignment = align as u8;
    }
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
pub struct ResPictureWithTex<const TEX_COORD_COUNT: usize> {
    pub picture: ResPicture,
    tex_coords: [[ResVec2; TEX_COORD_COUNT]; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResParts {
    pub pane: ResPane,
    pub property_count: u32,
    magnify: ResVec2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct ResPartsProperty
{
    name: [skyline::libc::c_char; 24],
    usage_flag: u8,
    basic_usage_flag: u8,
    material_usage_flag: u8,
    system_ext_user_data_override_flag: u8,
    property_offset: u32,
    ext_user_data_offset: u32,
    pane_basic_info_offset: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResPartsWithProperty<const PROPERTY_COUNT: usize> {
    pub parts: ResParts,
    property_table: [ResPartsProperty; PROPERTY_COUNT],
}