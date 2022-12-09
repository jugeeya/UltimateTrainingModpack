#![allow(dead_code)]

mod resources;
pub use resources::*;

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr()
    };
}

#[repr(C)]
#[derive(Debug)]
pub struct ResAnimationContent {
    name: [skyline::libc::c_char; 28],
    count: u8,
    anim_content_type: u8,
    padding: [skyline::libc::c_char; 2],
}

/**
 * Block Header Kind
 *
 * ANIM_TAG: pat1
 * ANIM_SHARE: pah1
 * ANIM_INFO: pai1
 */

#[repr(C)]
#[derive(Debug)]
pub struct ResAnimationBlock {
    block_header_kind: u32,
    block_header_size: u32,
    num_frames: u16,
    is_loop: bool,
    pad: [skyline::libc::c_char; 1],
    file_count: u16,
    anim_cont_count: u16,
    anim_cont_offsets_offset: u32,
}

#[repr(C)]
pub struct AnimTransform {
    res_animation_block: *mut ResAnimationBlock,
    frame: f32,
    enabled: bool,
}

impl AnimTransform {
    pub unsafe fn parse_anim_transform(&mut self) {
        let res_animation_block_data_start = (*self).res_animation_block as u64;
        let res_animation_block = &*(*self).res_animation_block;
        let mut anim_cont_offsets = (res_animation_block_data_start
            + res_animation_block.anim_cont_offsets_offset as u64)
            as *const u32;
        for anim_cont_idx in 0..res_animation_block.anim_cont_count {
            let anim_cont_offset = *anim_cont_offsets;
            let res_animation_cont = (res_animation_block_data_start + anim_cont_offset as u64)
                as *const ResAnimationContent;

            let name = skyline::try_from_c_str((*res_animation_cont).name.as_ptr())
                .unwrap_or("UNKNOWN".to_string());
            let anim_type = (*res_animation_cont).anim_content_type;
            let frame = (*self).frame;
            println!(
                "animTransform/resAnimationContent_{anim_cont_idx}: {name} of type {anim_type} on frame {frame}",
            );
            // AnimContentType 1 == MATERIAL
            if (name == "dig_3_anim" || name == "set_dmg_num_3") && anim_type == 1 {
                (*self).frame = 4.0;
            }
            if (name == "dig_2_anim" || name == "set_dmg_num_2") && anim_type == 1 {
                (*self).frame = 2.0;
            }
            if (name == "dig_1_anim" || name == "set_dmg_num_1") && anim_type == 1 {
                (*self).frame = 8.0;
            }

            anim_cont_offsets = anim_cont_offsets.add(1);
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct AnimTransformNode {
    prev: *mut AnimTransformNode,
    next: *mut AnimTransformNode,
}

impl AnimTransformNode {
    pub unsafe fn iterate_anim_list(&mut self) {
        let mut curr = self as *mut AnimTransformNode;
        let mut _anim_idx = 0;
        while !curr.is_null() {
            // Only if valid
            if curr != (*curr).next {
                let anim_transform = (curr as *mut u64).add(2) as *mut AnimTransform;
                anim_transform.as_mut().unwrap().parse_anim_transform();
            }
    
            curr = (*curr).next;
            _anim_idx += 1;
            if curr == self as *mut AnimTransformNode || curr == (*curr).next {
                break;
            }
        }
    }
}

#[repr(C)]
pub struct AnimTransformList {
    root: AnimTransformNode,
}


#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Pane {
    vtable: u64,
    pub link: PaneNode,
    pub parent: *mut Pane,
    pub children_list: PaneNode,
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
    pub flags: u8,
    pub alpha: u8,
    pub global_alpha: u8,
    base_position: u8,
    flag_ex: u8,
    // This is supposed to be 3 bytes padding + flags of 4 bytes + padding of 4 bytes
    pad: [u8; 3 + 4 + 4 + 8],
    global_matrix: [[f32; 3]; 4],
    user_matrix: *const u64,
    ext_user_data_list: *const u64,
    pub name: [skyline::libc::c_char; 25],
    user_data: [skyline::libc::c_char; 9],
}

impl Pane {
    pub unsafe fn find_pane_by_name_recursive(
        &self,
        s: &str,
    ) -> Option<&mut Pane> {
        find_pane_by_name_recursive(self, c_str!(s)).as_mut()
    }

    pub unsafe fn find_pane_by_name(
        &self,
        s: &str,
        recursive: bool,
    ) -> Option<&mut Pane> {
        find_pane_by_name(self, c_str!(s), recursive).as_mut()
    }

    pub unsafe fn set_text_string(&self, s: &str) {
        pane_set_text_string(self, c_str!(s));
    }

    pub unsafe fn remove_child(&self, child: &Pane) {
        pane_remove_child(self, child as *const Pane);
    }

    pub unsafe fn append_child(&self, child: &Pane) {
        pane_append_child(self, child as *const Pane);
    }

    pub unsafe fn as_parts(&mut self) -> *mut Parts {
        self as *mut Pane as *mut Parts
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Parts {
    pub pane: Pane,
    // Some IntrusiveList
    link: PaneNode,
    pub layout: *mut Layout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Picture {
    pub pane: Pane,
    material: *mut u8,
    vertex_colors: [[u8; 4]; 4],
    shared_memory: *mut u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct LayoutPane {
    layout_pane_ui2d: *mut Pane,
    picture: u64,
    sub_layout_pane_user_data_unk: u64,
    sub_layout_pane: *mut LayoutPane,
}

#[repr(C)]
#[derive(Debug)]
pub struct RawLayout {
    pub anim_trans_list: AnimTransformNode,
    pub root_pane: *const Pane,
    group_container: u64,
    layout_size: f64,
    pub layout_name: *const skyline::libc::c_char,
}

#[derive(Debug, Copy, Clone)]
pub struct PaneNode {
    pub prev: *mut PaneNode,
    pub next: *mut PaneNode,
}

#[repr(C)]
pub struct Group {
    pane_list: PaneNode,
    name: *const skyline::libc::c_char,
}

#[repr(C)]
pub struct GroupContainer {}

#[repr(C)]
#[derive(Debug)]
pub struct Layout {
    vtable: u64,
    pub raw_layout: RawLayout,
}

#[skyline::from_offset(0x59970)]
pub unsafe fn find_pane_by_name_recursive(
    pane: *const Pane,
    s: *const skyline::libc::c_char,
) -> *mut Pane;

#[skyline::from_offset(0x583c0)]
pub unsafe fn find_pane_by_name(
    pane: *const Pane,
    s: *const skyline::libc::c_char,
    recursive: bool,
) -> *mut Pane;

#[skyline::from_offset(0x37a1270)]
pub unsafe fn pane_set_text_string(pane: *const Pane, s: *const skyline::libc::c_char);

#[skyline::from_offset(0x58290)]
pub unsafe fn pane_remove_child(pane: *const Pane, child: *const Pane);

#[skyline::from_offset(0x58250)]
pub unsafe fn pane_append_child(pane: *const Pane, child: *const Pane);

pub unsafe fn get_typeinfo_name(cls_vtable: u64) -> String {
    let typeinfo_ptr_addr = (cls_vtable - 8) as *const u64;
    let typeinfo_addr = *typeinfo_ptr_addr;
    let typeinfo_name_ptr_addr = (typeinfo_addr + 8) as *const u64;
    let type_info_name_addr = (*typeinfo_name_ptr_addr) as *const skyline::libc::c_char;
    skyline::from_c_str(type_info_name_addr)
}