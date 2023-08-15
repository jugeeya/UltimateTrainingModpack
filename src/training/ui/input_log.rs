use itertools::Itertools;
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

use super::set_colored_icon_text;

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

    let icons = first_log
        .buttons
        .to_vec()
        .iter()
        .filter_map(|button| {
            Some(match *button {
                Buttons::ATTACK | Buttons::ATTACK_RAW => (
                    name_to_font_glyph(ButtonConfig::A, *p1_style_ptr),
                    ResColor {
                        r: 0,
                        g: 255,
                        b: 0,
                        a: 255,
                    },
                ),
                Buttons::SPECIAL | Buttons::SPECIAL_RAW | Buttons::SPECIAL_RAW2 => (
                    name_to_font_glyph(ButtonConfig::B, *p1_style_ptr),
                    ResColor {
                        r: 255,
                        g: 0,
                        b: 0,
                        a: 255,
                    },
                ),
                Buttons::JUMP | Buttons::FLICK_JUMP => (
                    name_to_font_glyph(ButtonConfig::X, *p1_style_ptr),
                    ResColor {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                ),
                Buttons::GUARD | Buttons::GUARD_HOLD => (
                    name_to_font_glyph(ButtonConfig::L, *p1_style_ptr),
                    ResColor {
                        r: 0,
                        g: 0,
                        b: 255,
                        a: 255,
                    },
                ),
                Buttons::CATCH => (
                    name_to_font_glyph(ButtonConfig::ZR, *p1_style_ptr),
                    ResColor {
                        r: 255,
                        g: 0,
                        b: 255,
                        a: 255,
                    },
                ),
                Buttons::STOCK_SHARE => (
                    name_to_font_glyph(ButtonConfig::PLUS, *p1_style_ptr),
                    ResColor {
                        r: 0,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                ),
                Buttons::APPEAL_HI => (
                    name_to_font_glyph(ButtonConfig::DPAD_UP, *p1_style_ptr),
                    ResColor {
                        r: 0,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                ),
                Buttons::APPEAL_LW => (
                    name_to_font_glyph(ButtonConfig::DPAD_DOWN, *p1_style_ptr),
                    ResColor {
                        r: 0,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                ),
                Buttons::APPEAL_SL => (
                    name_to_font_glyph(ButtonConfig::DPAD_LEFT, *p1_style_ptr),
                    ResColor {
                        r: 0,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                ),
                Buttons::APPEAL_SR => (
                    name_to_font_glyph(ButtonConfig::DPAD_RIGHT, *p1_style_ptr),
                    ResColor {
                        r: 0,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                ),
                _ => return None,
            })
        })
        .filter_map(|(icon_opt, color)| {
            if let Some(icon) = icon_opt {
                return Some((icon, color));
            }

            None
        })
        .unique_by(|(icon, _)| *icon)
        .collect::<Vec<(u16, ResColor)>>();

    // Empty them first
    const NUM_ICON_SLOTS: usize = 5;
    for idx in 0..NUM_ICON_SLOTS {
        let input_pane = log_pane
            .find_pane_by_name_recursive(format!("InputTxt{}", idx).as_str())
            .unwrap()
            .as_textbox();

        input_pane.set_text_string("");
    }

    for (idx, icon) in icons.iter().enumerate() {
        // todo: handle this better
        if idx >= NUM_ICON_SLOTS {
            continue;
        }

        let input_pane = log_pane
            .find_pane_by_name_recursive(format!("InputTxt{}", idx).as_str())
            .unwrap()
            .as_textbox();

        set_colored_icon_text(input_pane, &vec![icon.0], icon.1);
    }

    log_pane
        .find_pane_by_name_recursive("FrameTxt")
        .unwrap()
        .as_textbox()
        .set_text_string(format!("{}", logs.len()).as_str());
}
