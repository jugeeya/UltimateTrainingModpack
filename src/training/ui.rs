use skyline::{hooks::InlineCtx, logging::hex_dump_ptr, logging::HexDump};

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr()
    };
}

#[derive(Debug)]
pub struct TValue {
    value: u64,
    tt: i32,
}

#[skyline::hook(offset = 0x38f3d60)]
pub unsafe fn handle_lua_setfield(
    lua_state: u64,
    lua_tvalue: *const TValue,
    field_name: *const skyline::libc::c_char,
) {
    if skyline::from_c_str(field_name) == "LayoutRootList" {
        println!("In LayoutRootList");
    }
    original!()(lua_state, lua_tvalue, field_name);
}

#[skyline::hook(offset = 0x3777130)]
pub unsafe fn handle_play_animation(
    layout_view: u64,
    animation_name: *const skyline::libc::c_char,
) -> u64 {
    println!("play_animation: {}", skyline::from_c_str(animation_name));
    original!()(layout_view, animation_name)
}

#[skyline::hook(offset = 0x3776cd0)]
pub unsafe fn handle_play_animation_at_speed(
    speed: f32,
    unk: u64,
    animation_name: *const skyline::libc::c_char,
) -> u64 {
    println!(
        "play_animation_at_speed: {}",
        skyline::from_c_str(animation_name)
    );
    original!()(speed, unk, animation_name)
}

#[skyline::hook(offset = 0x3777000)]
pub unsafe fn handle_play_animation_at_speed2(
    speed: f32,
    unk: u64,
    animation_name: *const skyline::libc::c_char,
) -> u64 {
    println!(
        "play_animation_at_speed2: {}",
        skyline::from_c_str(animation_name)
    );
    original!()(speed, unk, animation_name)
}

#[skyline::hook(offset = 0x3776ab0, inline)]
pub unsafe fn handle_get_pane_animation(ctx: &mut InlineCtx) {
    println!(
        "get_pane_animation: {}",
        skyline::from_c_str(*ctx.registers[1].x.as_ref() as *const u8)
    );
}

#[skyline::hook(offset = 0x4b120)]
pub unsafe fn handle_bind_animation(
    layout_view: u64,
    animation_name: *const skyline::libc::c_char,
) -> u64 {
    println!("bind_animation: {}", skyline::from_c_str(animation_name));
    original!()(layout_view, animation_name)
}

#[skyline::hook(offset = 0x0595d0)]
pub unsafe fn handle_bind_animation2(
    layout_view: u64,
    animation_name: *const skyline::libc::c_char,
    unk1: u32,
    unk2: u32,
) -> u64 {
    println!("bind_animation: {}", skyline::from_c_str(animation_name));
    original!()(layout_view, animation_name, unk1, unk2)
}

#[repr(C, align(8))]
#[derive(Debug)]
pub struct Pane {
    vtable: u64,
    unk_addresses: [u64; 2],
    parent: *mut Pane,
    children_list: PaneNode,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    scale_x: f32,
    scale_y: f32,
    size_x: f32,
    size_y: f32,
    flags: u8,
    alpha: u8,
    global_alpha: u8,
    base_position: u8,
    flag_ex: u8,
    // This is supposed to be 3 bytes padding + flags of 4 bytes + padding of 4 bytes
    pad: [u8; 3 + 4 + 4 + 8],
    global_matrix: [[f32; 3]; 4],
    user_matrix: *const u64,
    ext_user_data_list: *const u64,
    name: [skyline::libc::c_char; 25],
    user_data: [skyline::libc::c_char; 9],
}

#[repr(C)]
#[derive(Debug)]
pub struct Parts {
    pane: Pane,
    // Some IntrusiveList
    link: PaneNode,
    layout: *mut Layout,
}

#[repr(C)]
#[derive(Debug)]
pub struct LayoutPane {
    layout_pane_ui2d: *mut Pane,
    picture: u64,
    sub_layout_pane_user_data_unk: u64,
    sub_layout_pane: *mut LayoutPane,
}

#[skyline::hook(offset = 0x3775480, inline)]
pub unsafe fn handle_get_pane_by_name(ctx: &mut InlineCtx) {
    // Grabbing stuff off the stack is interesting.
    let pane_name =
        skyline::from_c_str((ctx as *const InlineCtx as *const u8).add(0x100).add(0xD8));
    println!("get_pane_by_name: {}", pane_name);
    if pane_name == "set_dmg_p" || true {
        let layout_pane = (*ctx.registers[0].x.as_ref()) as *mut LayoutPane;
        if !layout_pane.is_null() {
            println!("pane: {:#?}", *layout_pane);
            // pane_set_text_string(layout_pane, c_str!("Test!"));
            let sublayout_pane = (*layout_pane).sub_layout_pane;
            if !sublayout_pane.is_null() {
                println!("sublayout_pane: {:#?}", *sublayout_pane);
                // pane_set_text_string(layout_pane, c_str!("Test!"));
            }
            let layout_pane_ui2d = (*layout_pane).layout_pane_ui2d;
            if !layout_pane_ui2d.is_null() {
                println!("pane_ui2d: {:#?}", *layout_pane_ui2d);
                // Turn invisible
                (*layout_pane_ui2d).scale_x = (*layout_pane_ui2d).scale_x * 2.0;
                (*layout_pane_ui2d).scale_y = (*layout_pane_ui2d).scale_y * 2.0;
                (*layout_pane_ui2d).flags = (*layout_pane_ui2d).flags | 0x10;
            }
        }
    }
}

#[skyline::hook(offset = 0x3774ac0)]
pub unsafe fn handle_set_enable_input(layout_root: u64, enable: bool) -> u64 {
    println!("set_enable_input");
    original!()(layout_root, enable)
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

#[repr(C)]
#[derive(Debug)]
pub struct AnimTransformNode {
    prev: *mut AnimTransformNode,
    next: *mut AnimTransformNode,
}

#[repr(C)]
pub struct AnimTransformList {
    root: AnimTransformNode,
}

#[repr(C)]
#[derive(Debug)]
pub struct RawLayout {
    anim_trans_list: AnimTransformNode,
    root_pane: *const Pane,
    group_container: u64,
    layout_size: f64,
    layout_name: *const skyline::libc::c_char,
}

#[derive(Debug)]
pub struct PaneNode {
    prev: *mut PaneNode,
    next: *mut PaneNode,
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
    raw_layout: RawLayout,
}

pub unsafe fn parse_anim_transform(layout_name: &str, anim_transform: *mut AnimTransform) {
    // Past the header kind bytes
    let res_animation_block_data_start = (*anim_transform).res_animation_block as u64;
    let res_animation_block = &*(*anim_transform).res_animation_block;
    let mut anim_cont_offsets = (res_animation_block_data_start
        + res_animation_block.anim_cont_offsets_offset as u64)
        as *const u32;
    for anim_cont_idx in 0..res_animation_block.anim_cont_count {
        let anim_cont_offset = *anim_cont_offsets;
        let res_animation_cont = (res_animation_block_data_start + anim_cont_offset as u64)
            as *const ResAnimationContent;

        let name = skyline::try_from_c_str((*res_animation_cont).name.as_ptr())
            .unwrap_or("UNKNOWN".to_string());
        println!(
            "{layout_name}/animTransform/resAnimationContent_{anim_cont_idx}: {}",
            name
        );
        if name == "UNKNOWN" {
            println!("Failed to get name of {layout_name}/animTransform/resAnimationContent_{anim_cont_idx}");
            println!(
                "Anim Transform:\n{}",
                HexDump(unsafe { &*(anim_transform as *const u8) })
            );
            println!(
                "Res Animation Block:\n{}",
                HexDump(unsafe { &*((*anim_transform).res_animation_block as *const u8) })
            );
            println!("Res Animation Block Values:\n{:#X?}", res_animation_block);
            println!(
                "Curr Res Animation Content:\n{}",
                HexDump(unsafe { &*(res_animation_cont as *const u8) })
            );

            skyline::error::show_error(
                0x70,
                "Failed to read animation block",
                "Read println logs for details",
            );
        }

        anim_cont_offsets = anim_cont_offsets.add(1);
    }
}

pub unsafe fn iterate_anim_list(layout_name: &str, anim_list: &mut AnimTransformNode) {
    let mut curr = anim_list as *mut AnimTransformNode;
    let mut anim_idx = 0;
    while !curr.is_null() {
        // Only if valid
        if curr != (*curr).next {
            let anim_transform = (curr as *mut u64).add(2) as *mut AnimTransform;
            parse_anim_transform(layout_name, anim_transform);
        }

        curr = (*curr).next;
        anim_idx += 1;
        if curr == anim_list as *mut AnimTransformNode || curr == (*curr).next {
            break;
        }
    }
}

pub unsafe fn get_typeinfo_name(cls_vtable: u64) -> String {
    let typeinfo_ptr_addr = (cls_vtable - 8) as *const u64;
    let typeinfo_addr = *typeinfo_ptr_addr;
    let typeinfo_name_ptr_addr = (typeinfo_addr + 8) as *const u64;
    let type_info_name_addr = (*typeinfo_name_ptr_addr) as *const skyline::libc::c_char;
    skyline::from_c_str(type_info_name_addr)
}

#[skyline::hook(offset = 0x4b620)]
pub unsafe fn handle_draw(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    let layout_name = skyline::from_c_str((*layout).raw_layout.layout_name);
    let layout_root_pane = (*layout).raw_layout.root_pane;
    let mut anim_list = &mut (*layout).raw_layout.anim_trans_list;
    // iterate_anim_list(&layout_name, anim_list);

    if layout_name == "info_training" {
        for s in ["txt_cap_00", "set_txt_num_00", "set_txt_num_01"] {
            let txt_pane = find_pane_by_name_recursive(layout_root_pane, c_str!(s));
            pane_set_text_string(txt_pane, c_str!("Hello!"));
        }
    }

    if layout_name == "info_melee" {
        for s in &["p1"] {
            let dmg_pane = find_pane_by_name_recursive(layout_root_pane, c_str!(s)) as *mut Parts;
            (*(dmg_pane as *mut Pane)).pos_y += 300.0;

            for anim_search_name in vec!["set_fxui_dead1", "set_fxui_dead2", "set_fxui_dead3"] {
                let dmg_pane_p1 =
                    find_pane_by_name_recursive(dmg_pane as *mut Pane, c_str!(anim_search_name))
                        as *mut Pane;
                if !dmg_pane_p1.is_null() {
                    println!(
                        "Found pane by {}::find_pane_by_name({}): {:X?}",
                        layout_name, anim_search_name, *dmg_pane_p1
                    );
                    pane_remove_child(dmg_pane as *mut Pane, dmg_pane_p1);
                }
            }
            for anim_search_name in vec![
                "set_dmg_num_1",
                "set_dmg_num_2",
                "set_dmg_num_3",
                "set_dmg_num_p",
                "set_dmg_num_dec",
            ] {
                let dmg_pane_p1 =
                    find_pane_by_name_recursive(dmg_pane as *mut Pane, c_str!(anim_search_name))
                        as *mut Pane;
                if !dmg_pane_p1.is_null() {
                    println!(
                        "p1 Layout Name: {}",
                        skyline::from_c_str((*(*dmg_pane).layout).raw_layout.layout_name)
                    );

                    println!(
                        "Found pane by {}::find_pane_by_name({}): {:X?}",
                        layout_name, anim_search_name, *dmg_pane_p1
                    );
                    pane_remove_child(dmg_pane as *mut Pane, dmg_pane_p1);
                }
            }
        }
    }

    original!()(layout, draw_info, cmd_buffer);
}

#[skyline::hook(offset = 0x4b120)]
pub unsafe fn handle_layout_bind_animation(layout: *mut Layout, anim: *const u64) {
    println!("Bind Animation");
    original!()(layout, anim)
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
    some_bool_maybe: bool,
) -> *mut Pane;

#[skyline::from_offset(0x37a1270)]
pub unsafe fn pane_set_text_string(pane: *const Pane, s: *const skyline::libc::c_char);

#[skyline::from_offset(0x58290)]
pub unsafe fn pane_remove_child(pane: *const Pane, child: *const Pane);

#[skyline::from_offset(0x4b780)]
pub unsafe fn layout_update_anim_frame(layout: *const Layout, frame: f32);

#[skyline::hook(offset = 0x3794e80)]
pub unsafe fn handle_find_animation_by_name(
    layout_view: *const u64,
    s: *const skyline::libc::c_char,
) -> u64 {
    let ret = original!()(layout_view, s);
    if skyline::from_c_str(s) == "changedig" {
        let ret = ret as *mut AnimTransform;
        if !ret.is_null() {
            parse_anim_transform("UNK", ret);
        }

        println!("get_pane_animation(changedig) -> {:#x?}", ret);
    }

    ret
}

#[skyline::hook(offset = 0x37ac310, inline)]
pub unsafe fn general_number_formatter(ctx: &mut InlineCtx) {}

pub fn install_hooks() {
    skyline::install_hooks!(
        // handle_lua_setfield,
        // handle_play_animation,
        // handle_play_animation_at_speed,
        // handle_get_pane_animation,
        // handle_play_animation_at_speed2,
        // handle_bind_animation,
        // handle_bind_animation2,
        // handle_set_enable_input,
        // handle_get_pane_by_name,
        handle_draw,
        handle_layout_bind_animation,
        handle_find_animation_by_name,
        general_number_formatter
    );
}
