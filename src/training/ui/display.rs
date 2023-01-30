use crate::{training::combo::FRAME_ADVANTAGE};
use training_mod_consts::OnOff;
use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};

pub static NUM_DISPLAY_PANES: usize = 1;

pub unsafe fn draw(root_pane: &mut Pane) {
    // Update frame advantage
    if let Some(parent) = root_pane.find_pane_by_name_recursive("trMod_disp_0") {
        parent.set_visible(crate::common::MENU.frame_advantage == OnOff::On);
    }

    if let Some(header) = root_pane.find_pane_by_name_recursive("trMod_disp_0_header") {
        header.as_textbox().set_text_string("Frame Advantage");
    }

    if let Some(text) = root_pane.find_pane_by_name_recursive("trMod_disp_0_txt") {
        let text = text.as_textbox();
        text.set_text_string(format!("{FRAME_ADVANTAGE}").as_str());
        match FRAME_ADVANTAGE {
            x if x < 0 => text.set_color(200, 8, 8, 255),
            x if x == 0 => text.set_color(0, 0, 0, 255),
            _ => text.set_color(31, 198, 0, 255),
        };
    }
}


#[allow(clippy::too_many_arguments)]
pub unsafe fn build(
    layout_name: &str,
    root_pane: &mut Pane,
    original_build: extern "C" fn(layout: *mut Layout,
        out_build_result_information: *mut u8,
        device: *const u8,
        block: *mut ResPane,
        parts_build_data_set: *const u8,
        build_arg_set: *const u8,
        build_res_set: *const u8,
        kind: u32,
    ) -> *mut Pane,
    layout: *mut Layout,
    out_build_result_information: *mut u8,
    device: *const u8,
    block: *mut ResPane,
    parts_build_data_set: *const u8,
    build_arg_set: *const u8,
    build_res_set: *const u8,
    kind: u32,
) {
    if layout_name != "info_training" {
        return;
    }

    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(
                    layout,
                    out_build_result_information,
                    device,
                    &mut $block as *mut $resTyp as *mut ResPane,
                    parts_build_data_set,
                    build_arg_set,
                    build_res_set,
                    $kind,
                ) as *mut $typ)
            }
        };
    }

    (0..NUM_DISPLAY_PANES).for_each(|idx| {
        let mod_prefix = "trMod_disp_";
        let parent_name = format!("{mod_prefix}{idx}");
        let pic_name = format!("{mod_prefix}{idx}_base");
        let header_name = format!("{mod_prefix}{idx}_header");
        let txt_name = format!("{mod_prefix}{idx}_txt");

        if (*block).name_matches("pic_numbase_01") {
            let block = block as *mut ResPictureWithTex<1>;
            let mut pic_block = *block;
            pic_block.set_name(pic_name.as_str());
            pic_block.set_pos(ResVec3::default());
            let pic_pane = build!(pic_block, ResPictureWithTex<1>, kind, Picture);
            pic_pane.detach();

            // pic is loaded first, we can create our parent pane here.
            let disp_pane_kind = u32::from_le_bytes([b'p', b'a', b'n', b'1']);
            let mut disp_pane_block = ResPane::new(parent_name.as_str());
            disp_pane_block.set_pos(ResVec3::new(806.0, 390.0 - (idx as f32 * 110.0), 0.0));
            let disp_pane = build!(disp_pane_block, ResPane, disp_pane_kind, Pane);
            disp_pane.detach();
            root_pane.append_child(disp_pane);
            disp_pane.append_child(pic_pane);
        }

        if (*block).name_matches("set_txt_num_01") {
            let disp_pane = root_pane
                .find_pane_by_name(parent_name.as_str(), true)
                .unwrap();

            let block = block as *mut ResTextBox;
            let mut text_block = *block;
            text_block.set_name(txt_name.as_str());
            text_block.set_pos(ResVec3::new(-10.0, -25.0, 0.0));
            let text_pane = build!(text_block, ResTextBox, kind, TextBox);
            text_pane.set_text_string(format!("Pane {idx}!").as_str());
            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            text_pane.set_default_material_colors();
            text_pane.detach();
            disp_pane.append_child(text_pane);
        }

        if (*block).name_matches("txt_cap_01") {
            let disp_pane = root_pane
                .find_pane_by_name(parent_name.as_str(), true)
                .unwrap();

            let block = block as *mut ResTextBox;
            let mut header_block = *block;
            header_block.set_name(header_name.as_str());
            header_block.set_pos(ResVec3::new(0.0, 25.0, 0.0));
            let header_pane = build!(header_block, ResTextBox, kind, TextBox);
            header_pane.set_text_string(format!("Header {idx}").as_str());
            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            header_pane.set_default_material_colors();
            // Header should be white text
            header_pane.set_color(255, 255, 255, 255);
            header_pane.detach();
            disp_pane.append_child(header_pane);
        }
    });
}