use crate::{training::combo::FRAME_ADVANTAGE};
use crate::training::ui;
use training_mod_consts::OnOff;
use skyline::nn::ui2d::*;
use smash::app::sv_animcmd::frame;
use smash::ui2d::{SmashPane, SmashTextBox};

pub static NUM_DISPLAY_PANES: usize = 1;

macro_rules! display_parent_fmt {
    ($x:ident) => {
        format!("trMod_disp_{}", $x).as_str()
    };
}

macro_rules! display_pic_fmt {
    ($x:ident) => {
        format!("trMod_disp_{}_base", $x).as_str()
    };
}

macro_rules! display_header_fmt {
    ($x:ident) => {
        format!("trMod_disp_{}_header", $x).as_str()
    };
}

macro_rules! display_txt_fmt {
    ($x:ident) => {
        format!("trMod_disp_{}_txt", $x).as_str()
    };
}

pub unsafe fn draw(root_pane: &mut Pane) {
    let frame_adv_idx = 0;
    // Update frame advantage
    if let Some(parent) = root_pane.find_pane_by_name_recursive(display_parent_fmt!(frame_adv_idx)) {
        parent.set_visible(crate::common::MENU.frame_advantage == OnOff::On);
    }

    if let Some(header) = root_pane.find_pane_by_name_recursive(display_header_fmt!(frame_adv_idx)) {
        header.as_textbox().set_text_string("Frame Advantage");
    }

    if let Some(text) = root_pane.find_pane_by_name_recursive(display_txt_fmt!(frame_adv_idx)) {
        let text = text.as_textbox();
        text.set_text_string(format!("{FRAME_ADVANTAGE}").as_str());
        match FRAME_ADVANTAGE {
            x if x < 0 => text.set_color(200, 8, 8, 255),
            x if x == 0 => text.set_color(0, 0, 0, 255),
            _ => text.set_color(31, 198, 0, 255),
        };
    }
}


pub static BUILD_PIC_BASE: ui::PaneCreationCallback = |_, root_pane, original_build, layout, out_build_result_information, device, block, parts_build_data_set, build_arg_set, build_res_set, kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    (0..NUM_DISPLAY_PANES).for_each(|idx| {
        let block = block as *mut ResPictureWithTex<1>;
        let mut pic_block = *block;
        pic_block.set_name(display_pic_fmt!(idx));
        pic_block.set_pos(ResVec3::default());
        let pic_pane = build!(pic_block, ResPictureWithTex<1>, kind, Picture);
        pic_pane.detach();

        // pic is loaded first, we can create our parent pane here.
        let disp_pane_kind = u32::from_le_bytes([b'p', b'a', b'n', b'1']);
        let mut disp_pane_block = ResPane::new(display_parent_fmt!(idx));
        disp_pane_block.set_pos(ResVec3::new(806.0, 390.0 - (idx as f32 * 110.0), 0.0));
        let disp_pane = build!(disp_pane_block, ResPane, disp_pane_kind, Pane);
        disp_pane.detach();
        root_pane.append_child(disp_pane);
        disp_pane.append_child(pic_pane);
    });
};

pub static BUILD_PANE_TXT: ui::PaneCreationCallback = |_, root_pane, original_build, layout, out_build_result_information, device, block, parts_build_data_set, build_arg_set, build_res_set, kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    (0..NUM_DISPLAY_PANES).for_each(|idx| {
        let disp_pane = root_pane
            .find_pane_by_name(display_parent_fmt!(idx), true)
            .unwrap();

        let block = block as *mut ResTextBox;
        let mut text_block = *block;
        text_block.set_name(display_txt_fmt!(idx));
        text_block.set_pos(ResVec3::new(-10.0, -25.0, 0.0));
        let text_pane = build!(text_block, ResTextBox, kind, TextBox);
        text_pane.set_text_string(format!("Pane {idx}!").as_str());
        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        text_pane.set_default_material_colors();
        text_pane.detach();
        disp_pane.append_child(text_pane);
    });
};

pub static BUILD_HEADER_TXT: ui::PaneCreationCallback = |_, root_pane, original_build, layout, out_build_result_information, device, block, parts_build_data_set, build_arg_set, build_res_set, kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    (0..NUM_DISPLAY_PANES).for_each(|idx| {
        let disp_pane = root_pane
            .find_pane_by_name(display_parent_fmt!(idx), true)
            .unwrap();

        let block = block as *mut ResTextBox;
        let mut header_block = *block;
        header_block.set_name(display_header_fmt!(idx));
        header_block.set_pos(ResVec3::new(0.0, 25.0, 0.0));
        let header_pane = build!(header_block, ResTextBox, kind, TextBox);
        header_pane.set_text_string(format!("Header {idx}").as_str());
        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        header_pane.set_default_material_colors();
        // Header should be white text
        header_pane.set_color(255, 255, 255, 255);
        header_pane.detach();
        disp_pane.append_child(header_pane);
    });
};