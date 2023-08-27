#[cfg(feature = "layout_arc_from_file")]
use byte_unit::MEBIBYTE;
use sarc::SarcFile;
use skyline::nn::ui2d::*;
use training_mod_consts::{OnOff, MENU};

#[cfg(feature = "layout_arc_from_file")]
use crate::consts::LAYOUT_ARC_PATH;
use crate::{
    common::{is_ready_go, is_training_mode, menu::QUICK_MENU_ACTIVE},
    training::frame_counter,
};

mod damage;
mod display;
pub mod menu;
pub mod notifications;

pub fn fade_out(pane: &mut Pane, current_frame: u32, total_frames: u32) {
    if current_frame < total_frames {
        // Logarithmic fade out
        let alpha = ((255.0 / (total_frames as f32 + 1.0).log10())
            * (current_frame as f32 + 1.0).log10()) as u8;
        pane.alpha = alpha;
        pane.global_alpha = alpha;

        // Linear fade out
        // let alpha = ((current_frame as f32 / 100.0) * 255.0) as u8;
        // pane.alpha = alpha;
        // pane.global_alpha = alpha;
    } else {
        pane.alpha = 0;
        pane.global_alpha = 0;
    }
}

#[skyline::hook(offset = 0x4b620)]
pub unsafe fn handle_draw(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    let layout_name = skyline::from_c_str((*layout).layout_name);
    let root_pane = &mut *(*layout).root_pane;

    // Set HUD to invisible if HUD is toggled off
    if is_training_mode()
        && is_ready_go()
        && [
            "info_playercursor",
            "info_playercursor_item",
            "info_melee",
            "info_radar_a",
            "info_radar_b",
        ]
        .contains(&layout_name.as_str())
    {
        // InfluencedAlpha means "Should my children panes' alpha be influenced by mine, as the parent?"
        root_pane.flags |= 1 << PaneFlag::InfluencedAlpha as u8;
        root_pane.set_visible(MENU.hud == OnOff::On && !QUICK_MENU_ACTIVE);
    }

    damage::draw(root_pane, &layout_name);

    if layout_name == "info_training" {
        frame_counter::tick_real();
        display::draw(root_pane);
        menu::draw(root_pane);
    }

    original!()(layout, draw_info, cmd_buffer);
}

// Allocate a static amount of memory that Smash isn't allowed to deallocate,
// in order for us to be able to swap the 'layout.arc' with the current
// version of the file in between loads of training mode.
#[cfg(feature = "layout_arc_from_file")]
const LAYOUT_ARC_SIZE: usize = (3 * MEBIBYTE) as usize;
#[cfg(feature = "layout_arc_from_file")]
static mut LAYOUT_ARC: &mut [u8; LAYOUT_ARC_SIZE] = &mut [0u8; LAYOUT_ARC_SIZE];

/// We are editing the info_training/layout.arc and replacing the original file with our
/// modified version from `LAYOUT_ARC_PATH`
///
/// When we edit the layout we are doing two things.
///
/// 1. Creating a new BFLYT inside the layout.arc for whatever component we are making. For example,
/// the slider menu button.
///
/// 2. Adding a Parts pane to the info_training.bflyt with the "Part Name" matching the name of
/// our new BFLYT without the file extension (mimicking how it's done with native Parts panes)
///
/// # Warnings
/// When creating a BFLYT from an existing one we need to edit the names of the panes so that
/// the default animations no longer modify them.  Otherwise the game will override properties,
/// i.e. material colours, and we won't be able to control them properly.
///
/// Once we have the file edited and saved to the correct location we can access the pane
/// from the layout as we normally would in our Draw function
/// `(root_pane.find_pane_by_name_recursive("name_of_parts_pane")`.
///
/// # Usage
/// Now say I want to edit background colour of the button's label.
/// I would have to grab the parts pane and find the pane I want to modify on it, then I'd be able
/// to make the modifications as I normally would.
///
/// ```rust
/// let slider_button = root_pane.find_pane_by_name_recursive("name_of_parts_pane");
/// let label_bg = slider_button.find_pane_by_name_recursive("name_of_picture_pane");
///
/// let label_material = &mut *label_bg.as_picture().material;
///
/// label_material.set_white_res_color(LABEL_WHITE_SELECTED_COLOR);
/// label_material.set_black_res_color(LABEL_BLACK_SELECTED_COLOR);
/// ```
#[skyline::hook(offset = 0x37730d4, inline)]
unsafe fn handle_layout_arc_malloc(ctx: &mut skyline::hooks::InlineCtx) {
    if !is_training_mode() {
        return;
    }

    let decompressed_file = *ctx.registers[21].x.as_ref() as *const u8;
    let decompressed_size = *ctx.registers[1].x.as_ref() as usize;

    let layout_arc = SarcFile::read(std::slice::from_raw_parts(
        decompressed_file,
        decompressed_size,
    ))
    .unwrap();
    let training_layout = layout_arc.files.iter().find(|f| {
        f.name.is_some() && f.name.as_ref().unwrap() == &String::from("blyt/info_training.bflyt")
    });
    if training_layout.is_none() {
        return;
    }

    let inject_arc;
    let inject_arc_size: u64;

    #[cfg(feature = "layout_arc_from_file")]
    {
        let inject_arc_from_file = std::fs::read(LAYOUT_ARC_PATH).unwrap();
        inject_arc_size = inject_arc_from_file.len() as u64;

        // Copy read file to global
        inject_arc_from_file
            .iter()
            .enumerate()
            .for_each(|(idx, byte)| LAYOUT_ARC[idx] = *byte);
        inject_arc = LAYOUT_ARC.as_ptr();
    }

    #[cfg(not(feature = "layout_arc_from_file"))]
    {
        include_flate::flate!(static INJECT_ARC_FROM_FILE: [u8] from "src/static/layout.arc");

        inject_arc = INJECT_ARC_FROM_FILE.as_ptr();
        inject_arc_size = INJECT_ARC_FROM_FILE.len() as u64;
    }

    // Decompressed file pointer
    let decompressed_file = ctx.registers[21].x.as_mut();
    *decompressed_file = inject_arc as u64;

    // Decompressed size is in each of these registers
    *ctx.registers[1].x.as_mut() = inject_arc_size;
    *ctx.registers[23].x.as_mut() = inject_arc_size;
    *ctx.registers[24].x.as_mut() = inject_arc_size;
}

pub fn init() {
    skyline::install_hooks!(handle_draw, handle_layout_arc_malloc);
}
