use crate::common::{is_ready_go, is_training_mode};
use sarc::SarcFile;
use skyline::nn::ui2d::*;
use training_mod_consts::{OnOff, MENU};
use std::collections::HashMap;
use parking_lot::Mutex;

mod damage;
mod display;
mod menu;
pub mod notifications;

type PaneCreationCallback = for<'a, 'b> unsafe fn(&'a str, &'b mut Pane,
                                                  extern "C" fn(*mut Layout, *mut u8, *const u8, *mut ResPane, *const u8, *const u8, *const u8, u32) -> *mut Pane,
                                                  *mut Layout, *mut u8, *const u8, *mut ResPane,
                                                  *const u8, *const u8, *const u8, u32);
type PaneCreationMap =  HashMap<
    (String, String), Vec<(bool, PaneCreationCallback)>
>;

lazy_static::lazy_static! {
    static ref PANE_CREATED: Mutex<PaneCreationMap> = Mutex::new(HashMap::from([
        // (
        //     (String::from("info_training"), String::from("pic_numbase_01")),
        //     vec![
        //         (false, menu::BUILD_CONTAINER_PANE),
        //         (false, menu::BUILD_SLIDER_CONTAINER_PANE),
        //     ]
        // ),
        // (
        //     (String::from("info_training"), String::from("pic_help_bg_00")),
        //     vec![(false, menu::BUILD_FOOTER_BG)]
        // ),
        // (
        //     (String::from("info_training"), String::from("set_txt_help_00")),
        //     vec![(false, menu::BUILD_FOOTER_TXT)]
        // ),
        // (
        //     (String::from("info_training"), String::from("set_txt_num_01")),
        //     vec![
        //         (false, menu::BUILD_TAB_TXTS),
        //         (false, menu::BUILD_OPT_TXTS),
        //         (false, menu::BUILD_SLIDER_TXTS),
        //     ]
        // ),
        // (
        //     (String::from("info_training"), String::from("txt_cap_01")),
        //     vec![
        //         (false, menu::BUILD_SLIDER_HEADER_TXT),
        //     ]
        // ),
        // (
        //     (String::from("info_training_btn0_00_item"), String::from("icn_bg_main")),
        //     vec![(false, menu::BUILD_BG_LEFTS)]
        // ),
        // (
        //     (String::from("info_training_btn0_00_item"), String::from("btn_bg")),
        //     vec![(false, menu::BUILD_BG_BACKS)]
        // ),
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
        // menu::draw(root_pane);
    }

    original!()(layout, draw_info, cmd_buffer);
}

#[skyline::hook(offset = 0x493a0)]
pub unsafe fn handle_build_parts_impl(
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

// We'll keep some sane max size here; we shouldn't reach above 500KiB is the idea,
// but we can try higher if we need to.
#[cfg(feature = "layout_arc_from_file")]
static mut LAYOUT_ARC : & mut [u8; 5000000] = &mut [0u8; 5000000];

// Insert actual byte size of the file below here!
const LAYOUT_ARC_SIZE: usize = 40000;
#[cfg(not(feature = "layout_arc_from_file"))]
static mut LAYOUT_ARC: &[u8; LAYOUT_ARC_SIZE] = include_bytes!("../../static/training_layout.arc");

#[skyline::hook(offset = 0x37730d4, inline)]
unsafe fn handle_pre_attach_malloc(
    ctx: &mut skyline::hooks::InlineCtx
) {
    let decompressed_file = *ctx.registers[21].x.as_ref() as *const u8;
    let decompressed_size = *ctx.registers[1].x.as_ref() as usize;

    let training_layout = String::from("blyt/info_training.bflyt");
    let is_training_layout = |file| file.name.is_some() && file.name.as_ref().unwrap() == &training_layout;
    if is_training_mode() && SarcFile::read(
            std::slice::from_raw_parts(decompressed_file,decompressed_size)
        ).unwrap().files.iter().any(is_training_layout) {

        let inject_arc_size : u64;

        #[cfg(feature = "layout_arc_from_file")] {
            let inject_arc = std::fs::read("sd:/TrainingModpack/layout.arc").unwrap();
            inject_arc_size = inject_arc.len() as u64;

            // Copy read file to global
            inject_arc
                .iter()
                .enumerate()
                .for_each(|(idx, byte)| LAYOUT_ARC[idx] = *byte);
        }

        #[cfg(not(feature = "layout_arc_from_file"))] {
            inject_arc_size = LAYOUT_ARC_SIZE as u64;
        }

        // Decompressed file pointer
        let decompressed_file = ctx.registers[21].x.as_mut();
        *decompressed_file = LAYOUT_ARC.as_ptr() as u64;

        // Decompressed size is in each of these registers
        *ctx.registers[1].x.as_mut() = inject_arc_size;
        *ctx.registers[23].x.as_mut() = inject_arc_size;
        *ctx.registers[24].x.as_mut() = inject_arc_size;
    }
}

pub fn init() {
    skyline::install_hooks!(
        handle_draw,
        handle_build_parts_impl,
        handle_layout_arc_malloc
    );
}
