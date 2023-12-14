use itertools::Itertools;
use std::collections::VecDeque;

use skyline::nn::ui2d::*;
use smash::app::{lua_bind::*, utility};
use smash::lib::lua_const::*;
use smash::ui2d::{SmashPane, SmashTextBox};
use training_mod_consts::{FighterId, InputDisplay, MENU};

use crate::{
    common::{consts::status_display_name, menu::QUICK_MENU_ACTIVE, try_get_module_accessor},
    training::{
        input_log::{
            DirectionStrength, InputLog, DRAW_LOG_BASE_IDX, NUM_LOGS, P1_INPUT_LOGS, WHITE, YELLOW,
        },
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
        icons.push_front((lstick_icon, WHITE));
    }

    icons
}

unsafe fn draw_log(root_pane: &Pane, log_idx: usize, log: &InputLog) {
    let draw_log_idx = (log_idx + (NUM_LOGS - *DRAW_LOG_BASE_IDX.data_ptr())) % NUM_LOGS;
    let log_pane = root_pane
        .find_pane_by_name_recursive(log_parent_fmt!(draw_log_idx))
        .unwrap();

    // Handle visibility and alpha
    log_pane.set_visible(true);
    const FADE_FRAMES: u32 = 200;
    fade_out(log_pane, log.ttl, FADE_FRAMES);

    // Handle positioning
    let new_pos_y = -52.5 * log_idx as f32;
    if new_pos_y != log_pane.pos_y {
        log_pane.pos_y = new_pos_y;
        log_pane.flags |= 1 << PaneFlag::IsGlobalMatrixDirty as u8;
    }

    // Only redraw first log!
    if log_idx != 0 {
        return;
    }

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
        "dpad_up",
        "dpad_down",
        "dpad_left",
        "dpad_right",
        "dpad_left_right",
        "up_strong",
        "down_strong",
        "left_strong",
        "right_strong",
        "up_left_strong",
        "down_left_strong",
        "up_right_strong",
        "down_right_strong",
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
    let logs_pane = root_pane
        .find_pane_by_name_recursive("TrModInputLog")
        .unwrap();
    logs_pane.set_visible(
        !QUICK_MENU_ACTIVE
            && !VANILLA_MENU_ACTIVE
            && (MENU.input_display != InputDisplay::NONE
                && MENU.input_display != InputDisplay::DEBUG),
    );

    let debug_pane = root_pane.find_pane_by_name_recursive("TrModDebug").unwrap();
    debug_pane.set_visible(
        !QUICK_MENU_ACTIVE && !VANILLA_MENU_ACTIVE && MENU.input_display == InputDisplay::DEBUG,
    );
    if MENU.input_display == InputDisplay::NONE {
        return;
    }

    let logs_ptr = P1_INPUT_LOGS.data_ptr();
    if logs_ptr.is_null() {
        return;
    }
    let logs = &*logs_ptr;

    if MENU.input_display == InputDisplay::DEBUG {
        let module_accessor = try_get_module_accessor(FighterId::Player);
        if module_accessor.is_none() {
            return;
        }
        let module_accessor = module_accessor.unwrap();

        let fighter_kind = utility::get_kind(&mut *module_accessor);
        let status = StatusModule::status_kind(module_accessor);

        let status_line = format!(
            "Status: {status}",
            status = status_display_name(fighter_kind, status)
        );

        let pos_x = PostureModule::pos_x(module_accessor);
        let pos_y = PostureModule::pos_y(module_accessor);
        let lr = PostureModule::lr(module_accessor);

        let position_line = format!("Position: ({pos_x:.2}, {pos_y:.2}); LR: {lr}");

        let x_vel =
            KineticModule::get_sum_speed_x(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let y_vel =
            KineticModule::get_sum_speed_y(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

        let speed_line = format!("Speed: ({x_vel:.2}, {y_vel:.2})");

        let first_log = logs.first().unwrap();
        let raw_left_stick_x = first_log.raw_inputs.left_stick_x;
        let raw_left_stick_y = first_log.raw_inputs.left_stick_y;
        let raw_buttons = first_log.raw_button_icons();

        let smash_left_stick_x = first_log.smash_inputs.lstick_x;
        let smash_left_stick_y = first_log.smash_inputs.lstick_y;
        let smash_buttons = first_log.smash_button_icons();

        let raw_line = format!(
            "Raw | Left Stick: ({left_x:.5}, {left_y:.5})\nButtons: {buttons}",
            left_x = raw_left_stick_x,
            left_y = raw_left_stick_y,
            buttons = raw_buttons
                .iter()
                .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
                .map(|(name, _color)| name)
                .join("|")
        );

        let smash_line = format!(
            "Smash | Left Stick: ({left_x}, {left_y})\nButtons: {buttons}",
            left_x = smash_left_stick_x,
            left_y = smash_left_stick_y,
            buttons = smash_buttons
                .iter()
                .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
                .map(|(name, _color)| name)
                .join("|")
        );

        debug_pane
            .find_pane_by_name_recursive("DebugTxt")
            .unwrap()
            .as_textbox()
            .set_text_string(&format!(
                "{status_line}\n{position_line}\n{speed_line}\n{raw_line}\n{smash_line}"
            ));
    } else {
        for (log_idx, log) in logs.iter().enumerate() {
            draw_log(root_pane, log_idx, log);
        }
    }
}
