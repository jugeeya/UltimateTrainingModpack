use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};
use training_mod_consts::ButtonConfig;

use crate::{
    common::{
        button_config::name_to_font_glyph,
        input::Buttons,
        menu::{P1_CONTROLLER_STYLE, QUICK_MENU_ACTIVE},
    },
    training::input_log::P1_INPUT_MAPPINGS,
};

use super::set_icon_text;

macro_rules! log_parent_fmt {
    ($x:ident) => {
        format!("TrModInputLog{}", $x).as_str()
    };
}

pub unsafe fn draw(root_pane: &Pane) {
    let log_number = 0;
    let log_pane = root_pane
        .find_pane_by_name_recursive(log_parent_fmt!(log_number))
        .unwrap();

    // TODO: And menu option for input log is on
    log_pane.set_visible(!QUICK_MENU_ACTIVE);

    let logs_ptr = P1_INPUT_MAPPINGS.data_ptr();
    if logs_ptr.is_null() {
        return;
    }
    let logs = &*logs_ptr;
    let first_log = logs.front();
    if first_log.is_none() {
        return;
    }
    let first_log = first_log.unwrap();

    let p1_style_ptr = P1_CONTROLLER_STYLE.data_ptr();
    if p1_style_ptr.is_null() {
        return;
    }

    let input_pane = log_pane
        .find_pane_by_name_recursive("InputTxt")
        .unwrap()
        .as_textbox();

    input_pane.set_text_string("NONE");

    let mut glyphs = vec![];
    if first_log.buttons.contains(Buttons::ATTACK) {
        let potential_font_glyph = name_to_font_glyph(ButtonConfig::A, *p1_style_ptr);
        if let Some(font_glyph) = potential_font_glyph {
            glyphs.push(font_glyph);
        }
    }

    if first_log.buttons.contains(Buttons::SPECIAL) {
        let potential_font_glyph = name_to_font_glyph(ButtonConfig::B, *p1_style_ptr);
        if let Some(font_glyph) = potential_font_glyph {
            glyphs.push(font_glyph);
        }
    }

    if first_log.buttons.contains(Buttons::JUMP) {
        let potential_font_glyph = name_to_font_glyph(ButtonConfig::X, *p1_style_ptr);
        if let Some(font_glyph) = potential_font_glyph {
            glyphs.push(font_glyph);
        }
    }

    if !glyphs.is_empty() {
        set_icon_text(input_pane, glyphs);
    }

    log_pane
        .find_pane_by_name_recursive("FrameTxt")
        .unwrap()
        .as_textbox()
        .set_text_string(format!("{}", logs.len()).as_str());
}
