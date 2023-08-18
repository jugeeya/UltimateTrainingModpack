use std::collections::VecDeque;

use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};
use training_mod_consts::{InputDisplay, MENU};

use crate::{
    common::menu::QUICK_MENU_ACTIVE,
    training::input_log::{DirectionStrength, InputLog, P1_INPUT_LOGS, WHITE, YELLOW},
};

macro_rules! log_parent_fmt {
    ($x:ident) => {
        format!("TrModInputLog{}", $x).as_str()
    };
}

fn get_input_icons(log: &InputLog) -> VecDeque<(&str, ResColor)> {
    let mut icons = log.button_icons();

    let (rstick_strength, rstick_angle) = log.binned_rstick();
    let rstick_icon = if rstick_strength != DirectionStrength::None {
        match rstick_angle as u32 {
            0 => ">",
            45 => "^>",
            90 => "^",
            135 => "<^",
            180 => "<",
            225 => "<v",
            270 => "v",
            315 => "v>",
            _ => "?",
        }
    } else {
        ""
    };

    if !rstick_icon.is_empty() {
        icons.push_front((rstick_icon, YELLOW));
    }

    let (lstick_strength, lstick_angle) = log.binned_lstick();
    let lstick_icon = if lstick_strength != DirectionStrength::None {
        match lstick_angle as u32 {
            0 => ">",
            45 => "^>",
            90 => "^",
            135 => "<^",
            180 => "<",
            225 => "<v",
            270 => "v",
            315 => "v>",
            _ => "?",
        }
    } else {
        ""
    };

    if !lstick_icon.is_empty() {
        icons.push_front((lstick_icon, WHITE));
    }

    icons
}

unsafe fn draw_log(root_pane: &Pane, log_idx: usize, log: &InputLog) {
    let log_pane = root_pane
        .find_pane_by_name_recursive(log_parent_fmt!(log_idx))
        .unwrap();

    log_pane.set_visible(!QUICK_MENU_ACTIVE && MENU.input_display != InputDisplay::None);
    if MENU.input_display == InputDisplay::None {
        return;
    }
    const FADE_FRAMES: u32 = 200;
    if log.ttl < FADE_FRAMES {
        // Logarithmic fade out
        let alpha =
            ((255.0 / (FADE_FRAMES as f32 + 1.0).log10()) * (log.ttl as f32 + 1.0).log10()) as u8;
        log_pane.alpha = alpha;
        log_pane.global_alpha = alpha;
    } else {
        log_pane.alpha = 255;
        log_pane.global_alpha = 255;
    }

    let icons = get_input_icons(log);

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

        input_pane.set_text_string(icon.0);
        input_pane.set_default_material_colors();
        input_pane.set_color(icon.1.r, icon.1.g, icon.1.b, icon.1.a);
    }

    let frame_text = format!("{}", log.frames);
    log_pane
        .find_pane_by_name_recursive("FrameTxt")
        .unwrap()
        .as_textbox()
        .set_text_string(frame_text.as_str());
}

pub unsafe fn draw(root_pane: &Pane) {
    let logs_ptr = P1_INPUT_LOGS.data_ptr();
    if logs_ptr.is_null() {
        return;
    }
    let logs = &*logs_ptr;

    for (log_idx, log) in logs.iter().enumerate() {
        draw_log(root_pane, log_idx, log);
    }
}
