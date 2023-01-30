use crate::common::{is_ready_go, is_training_mode};
use skyline::nn::ui2d::*;
use training_mod_consts::{OnOff, MENU};
use std::collections::HashMap;
use parking_lot::Mutex;

mod damage;
mod display;
mod menu;

type PaneCreationCallback = for<'a, 'b> unsafe fn(&'a str, &'b mut Pane,
                                                  extern "C" fn(*mut Layout, *mut u8, *const u8, *mut ResPane, *const u8, *const u8, *const u8, u32) -> *mut Pane,
                                                  *mut Layout, *mut u8, *const u8, *mut ResPane,
                                                  *const u8, *const u8, *const u8, u32);

lazy_static::lazy_static! {
    static ref PANE_CREATED: Mutex<HashMap<
        (String, String), Vec<(bool, PaneCreationCallback)>
    >> = Mutex::new(HashMap::from([
        (
            (String::from("info_training"), String::from("pic_numbase_01")),
            vec![(false, menu::build_menu_display_pane)]
        ),
        (
            (String::from("info_training"), String::from("pic_help_bg_00")),
            vec![(false, menu::build_menu_footer_bg)]
        ),
        (
            (String::from("info_training"), String::from("set_txt_help_00")),
            vec![(false, menu::build_menu_footer_txt)]
        ),
        (
            (String::from("info_training"), String::from("set_txt_num_01")),
            vec![
                (false, menu::build_menu_tab_txts),
                (false, menu::build_menu_opt_txts),
                (false, menu::build_menu_slider_txts)
            ]
        ),
    ]));
}

#[skyline::hook(offset = 0x4b620)]
pub unsafe fn handle_draw(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    let layout_name = &skyline::from_c_str((*layout).layout_name);
    let root_pane = &mut *(*layout).root_pane;

    // Set HUD to invisible if HUD is toggled off
    if is_training_mode() && is_ready_go() && layout_name != "info_training" {
        // InfluencedAlpha means "Should my children panes' alpha be influenced by mine, as the parent?"
        root_pane.flags |= 1 << PaneFlag::InfluencedAlpha as u8;
        root_pane.set_visible(MENU.hud == OnOff::On);
    }

    damage::draw(root_pane, layout_name);

    if layout_name == "info_training" {
        display::draw(root_pane);
        menu::draw(root_pane);
    }

    original!()(layout, draw_info, cmd_buffer);
}

#[skyline::hook(offset = 0x493a0)]
pub unsafe fn layout_build_parts_impl(
    layout: *mut Layout,
    out_build_result_information: *mut u8,
    device: *const u8,
    block: *mut ResPane,
    parts_build_data_set: *const u8,
    build_arg_set: *const u8,
    build_res_set: *const u8,
    kind: u32,
) -> *mut Pane {
    let layout_name = &skyline::from_c_str((*layout).layout_name);
    let root_pane = &mut *(*layout).root_pane;

    menu::build(
        layout_name,
        root_pane,
        original!(),
        layout,
        out_build_result_information,
        device,
        block,
        parts_build_data_set,
        build_arg_set,
        build_res_set,
        kind,
    );

    display::build(
        layout_name,
        root_pane,
        original!(),
        layout,
        out_build_result_information,
        device,
        block,
        parts_build_data_set,
        build_arg_set,
        build_res_set,
        kind,
    );

    original!()(
        layout,
        out_build_result_information,
        device,
        block,
        parts_build_data_set,
        build_arg_set,
        build_res_set,
        kind,
    )
}

pub fn init() {
    skyline::install_hooks!(handle_draw, layout_build_parts_impl,);
}
