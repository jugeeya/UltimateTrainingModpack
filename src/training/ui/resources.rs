// Maybe needs a vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResVec2 {
    x: f32,
    y: f32
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
    flagEx: u8,
    pub name: [skyline::libc::c_char; 24],
    pub user_data: [skyline::libc::c_char; 8],
    pub pos_x: f32,
    pub pos_y: f32,
    pos_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub size_x: f32,
    pub size_y: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResTextBox {
    pub pane: ResPane,
    textBufBytes: u16,
    textStrBytes: u16,
    materialIdx: u16,
    fontIdx: u16,
    textPosition: u8,
    textAlignment: u8,
    pub textBoxFlag: u16,
    italicRatio: f32,
    textStrOffset: u32,
    textCols: [ResColor; 2],
    fontSize: ResVec2,
    charSpace: f32,
    lineSpace: f32,
    textIdOffset: u32,
    shadowOffset: ResVec2,
    shadowScale: ResVec2,
    shadowCols: [ResColor; 2],
    shadowItalicRatio: f32,
    lineWidthOffsetOffset: u32,
    perCharacterTransformOffset: u32,

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