use std::collections::VecDeque;

use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};
use training_mod_consts::{InputDisplay, MENU};

use crate::{
    common::{consts::status_display_name, menu::QUICK_MENU_ACTIVE},
    training::{
        input_log::{DirectionStrength, InputLog, BLACK, P1_INPUT_LOGS, YELLOW},
        ui::{fade_out, menu::VANILLA_MENU_ACTIVE},
    },
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
            0 => "right",
            45 => "up_right",
            90 => "up",
            135 => "up_left",
            180 => "left",
            225 => "down_left",
            270 => "down",
            315 => "down_right",
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
            0 => "right",
            45 => "up_right",
            90 => "up",
            135 => "up_left",
            180 => "left",
            225 => "down_left",
            270 => "down",
            315 => "down_right",
            _ => "?",
        }
    } else {
        ""
    };

    if !lstick_icon.is_empty() {
        icons.push_front((lstick_icon, BLACK));
    }

    icons
}

unsafe fn draw_log(root_pane: &Pane, log_idx: usize, log: &InputLog) {
    let log_pane = root_pane
        .find_pane_by_name_recursive(log_parent_fmt!(log_idx))
        .unwrap();

    log_pane.set_visible(
        !QUICK_MENU_ACTIVE && !VANILLA_MENU_ACTIVE && MENU.input_display != InputDisplay::None,
    );
    if MENU.input_display == InputDisplay::None {
        return;
    }
    const FADE_FRAMES: u32 = 200;
    fade_out(log_pane, log.ttl, FADE_FRAMES);

    let icons = get_input_icons(log);

    // Empty them first
    const NUM_ICON_SLOTS: usize = 5;
    let available_icons = vec![
        "a",
        "b",
        "x",
        "y",
        "lb",
        "rb",
        "zl",
        "zr",
        "up",
        "down",
        "left",
        "right",
        "up_left",
        "up_right",
        "down_left",
        "down_right",
        "gcc_l",
        "gcc_r",
        "gcc_z",
        "plus",
        "minus",
        "l_stick",
        "r_stick",
        "gcc_c_stick",
    ];

    for idx in 0..NUM_ICON_SLOTS {
        let input_pane = log_pane
            .find_pane_by_name_recursive(format!("Input{}", idx).as_str())
            .unwrap();

        available_icons
            .iter()
            .map(|name| input_pane.find_pane_by_name_recursive(name).unwrap())
            .for_each(|icon_pane| {
                icon_pane.set_visible(false);
            });
    }

    for (index, icon) in icons.iter().enumerate() {
        // Temporarily comparing to the list of available icons until they are all in
        // Just in case we run into an icon name that isn't present
        let (icon_name, icon_color) = icon;
        if index >= NUM_ICON_SLOTS || !available_icons.contains(icon_name) {
            continue;
        }

        let input_pane = log_pane
            .find_pane_by_name_recursive(format!("Input{}", index).as_str())
            .unwrap();

        let icon_pane = input_pane
            .find_pane_by_name_recursive(icon_name)
            .unwrap()
            .as_picture();

        icon_pane.set_visible(true);
        (*icon_pane.material).set_black_res_color(*icon_color);
        icon_pane.flags |= PaneFlag::IsGlobalMatrixDirty as u8;
    }

    let frame_text = format!("{}", log.frames);
    log_pane
        .find_pane_by_name_recursive("Frame")
        .unwrap()
        .as_textbox()
        .set_text_string(frame_text.as_str());

    let status_text = if MENU.input_display_status.as_bool() {
        status_display_name(log.fighter_kind, log.status)
    } else {
        "".to_string()
    };
    log_pane
        .find_pane_by_name_recursive("Status")
        .unwrap()
        .as_textbox()
        .set_text_string(status_text.as_str());
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
