use skyline::{hooks::InlineCtx};
use crate::training::ui::*;

#[skyline::hook(offset = 0x4b620)]
pub unsafe fn handle_draw(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    let layout_name = skyline::from_c_str((*layout).raw_layout.layout_name);
    let layout_root_pane = &*(*layout).raw_layout.root_pane;
    let _anim_list = &mut (*layout).raw_layout.anim_trans_list;
    // anim_list.iterate_anim_list();

    if layout_name == "info_training" {
        for s in ["txt_cap_00", "set_txt_num_00", "set_txt_num_01"] {
            let txt_pane = layout_root_pane.find_pane_by_name_recursive(s).unwrap();
            txt_pane.set_text_string("Hello!");
        }

        if let Some(parent_pane) = layout_root_pane.find_pane_by_name_recursive("N_null") {
            parent_pane.pos_x -= 300.0;
        }

        if let Some(text_box) = layout_root_pane.find_pane_by_name_recursive("T_text") {
            text_box.set_text_string("Hello?");
        }

        let _picture_pane = layout_root_pane.find_pane_by_name_recursive("P_pict");
    }

    // if layout_name == "info_melee" {
    //     let mut dmg_pane = &mut *layout_root_pane.find_pane_by_name_recursive("p1").unwrap().as_parts();
    //     dmg_pane.pane.pos_y += 300.0;

    //     let _p1_layout_name = skyline::from_c_str((*dmg_pane.layout).raw_layout.layout_name);
    //     let _anim_list = &mut (*dmg_pane.layout).raw_layout.anim_trans_list;
    //     // anim_list.iterate_anim_list();

    //     for anim_search_name in vec!["set_fxui_dead1", "set_fxui_dead2", "set_fxui_dead3"] {
    //         let dmg_pane_p1 = dmg_pane.pane.find_pane_by_name_recursive(anim_search_name);
    //         if dmg_pane_p1.is_some() {
    //             let dmg_pane_p1 = dmg_pane_p1.unwrap();
    //             println!(
    //                 "Found pane by {}::find_pane_by_name({}): {:X?}",
    //                 layout_name, anim_search_name, dmg_pane_p1
    //             );
    //             dmg_pane.pane.remove_child(&dmg_pane_p1);
    //         }
    //     }
    //     for anim_search_name in vec![
    //         "set_dmg_num_1",
    //         "set_dmg_num_2",
    //         "set_dmg_num_3",
    //         "set_dmg_num_p",
    //         "set_dmg_num_dec",
    //     ] {
    //         let dmg_pane_p1 = dmg_pane.pane.find_pane_by_name_recursive(anim_search_name);
    //         if dmg_pane_p1.is_some() {
    //             let dmg_pane_p1 = dmg_pane_p1.unwrap();
    //             println!(
    //                 "Found pane by {}::find_pane_by_name({}): {:X?}",
    //                 layout_name, anim_search_name, dmg_pane_p1
    //             );
    //             dmg_pane.pane.remove_child(&dmg_pane_p1);
    //         }
    //     }
    // }

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
pub unsafe fn general_number_formatter(_ctx: &mut InlineCtx) {}

#[skyline::hook(offset = 0x493a0)]
pub unsafe fn layout_build_parts_impl(
    layout: *mut Layout,
    out_build_result_information: *mut u8,
    device: *const u8,
    data: *mut u8,
    parts_build_data_set: *const u8,
    build_arg_set: *const u8,
    build_res_set: *const u8,
    kind: u32
) -> *mut Pane {
    let layout_name = skyline::from_c_str((*layout).raw_layout.layout_name);
    
    let kind_str : String = kind.to_le_bytes().map(|b| b as char).iter().collect();
    let block = data as *mut ResTextBox;
    if (*block).pane.name[0..=13].eq("set_txt_num_01".as_bytes()) {
        println!("Found set_txt_num_01!");
        let mut new_block = (*block).clone();
        new_block.pane.name[0] = b'Q' as u8;
        new_block.pane.pos_x -= 600.0;
        let new_pane = original!()(
            layout, out_build_result_information, device, &mut new_block as *mut ResTextBox as *mut u8, parts_build_data_set, build_arg_set, build_res_set, kind);    
        println!("New Pane: {:#?}", *new_pane);
        (*new_pane).set_text_string("New Pane?");
    }
    if kind_str == "txt1" {
        (*block).textBoxFlag = 0x4;
    }
    let pane = original!()(
        layout, out_build_result_information, device, data, parts_build_data_set, build_arg_set, build_res_set, kind);

    if layout_name == "info_training" {
        let pane_name = skyline::from_c_str(&(*pane).name as *const u8);

        if kind_str == "txt1" && ["set_txt_num_01", "T_text"].contains(&pane_name.as_str()) {
            println!("Layout BuildPartsImpl(Layout: {layout_name}, Kind: {kind_str}) -> Pane: {pane_name}\n");
            println!("{:#X?}", *block);
        }
    }

    pane
}

#[skyline::hook(offset = 0x47db0, inline)]
pub unsafe fn layout_build_pane_obj(ctx: &mut InlineCtx) {
    println!("Layout BuildPaneObj:\n{}", ctx);
}



pub fn install_hooks() {
    skyline::install_hooks!(
        handle_draw,
        handle_find_animation_by_name,
        general_number_formatter,
        layout_build_parts_impl,
        layout_build_pane_obj
    );
}
