use skyline::{hooks::InlineCtx, logging::hex_dump_ptr, logging::HexDump};
use crate::training::ui::*;

#[skyline::hook(offset = 0x4b620)]
pub unsafe fn handle_draw(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    let layout_name = skyline::from_c_str((*layout).raw_layout.layout_name);
    let layout_root_pane = &*(*layout).raw_layout.root_pane;
    let mut anim_list = &mut (*layout).raw_layout.anim_trans_list;
    // anim_list.iterate_anim_list();

    if layout_name == "info_training" {
        for s in ["txt_cap_00", "set_txt_num_00", "set_txt_num_01"] {
            let txt_pane = layout_root_pane.find_pane_by_name_recursive(s).unwrap();
            txt_pane.set_text_string("Hello!");
        }
    }

    if layout_name == "info_melee" {
        let mut dmg_pane = &mut *layout_root_pane.find_pane_by_name_recursive("p1").unwrap().as_parts();
        dmg_pane.pane.pos_y += 300.0;

        let p1_layout_name = skyline::from_c_str((*dmg_pane.layout).raw_layout.layout_name);
        let mut anim_list = &mut (*dmg_pane.layout).raw_layout.anim_trans_list;
        // anim_list.iterate_anim_list();

        for anim_search_name in vec!["set_fxui_dead1", "set_fxui_dead2", "set_fxui_dead3"] {
            let dmg_pane_p1 = dmg_pane.pane.find_pane_by_name_recursive(anim_search_name);
            if dmg_pane_p1.is_some() {
                let dmg_pane_p1 = dmg_pane_p1.unwrap();
                println!(
                    "Found pane by {}::find_pane_by_name({}): {:X?}",
                    layout_name, anim_search_name, dmg_pane_p1
                );
                dmg_pane.pane.remove_child(&dmg_pane_p1);
            }
        }
        for anim_search_name in vec![
            "set_dmg_num_1",
            "set_dmg_num_2",
            "set_dmg_num_3",
            "set_dmg_num_p",
            "set_dmg_num_dec",
        ] {
            let dmg_pane_p1 = dmg_pane.pane.find_pane_by_name_recursive(anim_search_name);
            if dmg_pane_p1.is_some() {
                let dmg_pane_p1 = dmg_pane_p1.unwrap();
                println!(
                    "Found pane by {}::find_pane_by_name({}): {:X?}",
                    layout_name, anim_search_name, dmg_pane_p1
                );
                dmg_pane.pane.remove_child(&dmg_pane_p1);
            }
        }
    }

    original!()(layout, draw_info, cmd_buffer);
}

#[skyline::hook(offset = 0x3794e80)]
pub unsafe fn handle_find_animation_by_name(
    layout_view: *const u64,
    s: *const skyline::libc::c_char,
) -> u64 {
    let ret = original!()(layout_view, s);
    if skyline::from_c_str(s) == "changedig" {
        let ret = ret as *mut AnimTransform;
        if !ret.is_null() {
            ret.as_mut().unwrap().parse_anim_transform();
        }

        println!("get_pane_animation(changedig) -> {:#x?}", ret);
    }

    ret
}

#[skyline::hook(offset = 0x37ac310, inline)]
pub unsafe fn general_number_formatter(ctx: &mut InlineCtx) {}

pub fn install_hooks() {
    skyline::install_hooks!(
        handle_draw,
        handle_find_animation_by_name,
        general_number_formatter
    );
}
