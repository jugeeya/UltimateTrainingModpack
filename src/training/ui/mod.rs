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
            vec![
                (false, menu::BUILD_CONTAINER_PANE),
                (false, display::BUILD_PIC_BASE)
            ]
        ),
        (
            (String::from("info_training"), String::from("pic_help_bg_00")),
            vec![(false, menu::BUILD_FOOTER_BG)]
        ),
        (
            (String::from("info_training"), String::from("set_txt_help_00")),
            vec![(false, menu::BUILD_FOOTER_TXT)]
        ),
        (
            (String::from("info_training"), String::from("set_txt_num_01")),
            vec![
                (false, menu::BUILD_TAB_TXTS),
                (false, menu::BUILD_OPT_TXTS),
                (false, menu::BUILD_SLIDER_TXTS),
                (false, display::BUILD_PANE_TXT),
            ]
        ),
        (
            (String::from("info_training"), String::from("txt_cap_01")),
            vec![(false, display::BUILD_HEADER_TXT)]
        ),
        (
            (String::from("info_training_btn0_00_item"), String::from("icn_bg_main")),
            vec![(false, menu::BUILD_BG_LEFTS)]
        ),
        (
            (String::from("info_training_btn0_00_item"), String::from("btn_bg")),
            vec![(false, menu::BUILD_BG_BACKS)]
        ),
    ]));
}

pub unsafe fn reset_creation() {
    let pane_created = &mut *PANE_CREATED.data_ptr();
    pane_created.iter_mut().for_each(|(_identifier, creators)| {
        creators.iter_mut().for_each(|(created, _callback)| {
            *created = false;
        })
    })
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

    let block_name = (*block).get_name();
    let identifier = (layout_name.to_string(), block_name);
    let pane_created = &mut *PANE_CREATED.data_ptr();
    let panes = pane_created.get_mut(&identifier);
    if let Some(panes) = panes {
        panes.iter_mut().for_each(|(has_created, callback)| {
            if !*has_created {
                callback(layout_name,
                         root_pane,
                         original!(),
                         layout,
                         out_build_result_information,
                         device,
                         block,
                         parts_build_data_set,
                         build_arg_set,
                         build_res_set,
                         kind
                );

                // Special case: Menu init should always occur
                if ("info_training".to_string(), "pic_numbase_01".to_string()) != identifier {
                    *has_created = true;
                }
            }
        });
    }

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
