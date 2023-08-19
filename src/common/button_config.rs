use std::collections::HashMap;

use crate::common::*;
use crate::input::{ControllerStyle::*, *};
use crate::training::ui::menu::VANILLA_MENU_ACTIVE;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::menu::QUICK_MENU_ACTIVE;

pub fn button_mapping(
    button_config: ButtonConfig,
    style: ControllerStyle,
    b: ButtonBitfield,
) -> bool {
    match button_config {
        ButtonConfig::A => b.a(),
        ButtonConfig::B => b.b(),
        ButtonConfig::X => b.x(),
        ButtonConfig::Y => b.y(),
        ButtonConfig::L => match style {
            GCController => false,
            _ => b.l(),
        },
        ButtonConfig::R => match style {
            GCController => b.zr(),
            _ => b.r(),
        },
        ButtonConfig::ZL => match style {
            GCController => b.l() || b.real_digital_l(),
            _ => b.zl() || b.left_sl() || b.right_sl(),
        },
        ButtonConfig::ZR => match style {
            GCController => b.r() || b.real_digital_r(),
            _ => b.zr() || b.left_sr() || b.right_sr(),
        },
        ButtonConfig::DPAD_UP => b.dpad_up(),
        ButtonConfig::DPAD_DOWN => b.dpad_down(),
        ButtonConfig::DPAD_LEFT => b.dpad_left(),
        ButtonConfig::DPAD_RIGHT => b.dpad_right(),
        ButtonConfig::PLUS => b.plus(),
        ButtonConfig::MINUS => b.minus(),
        ButtonConfig::LSTICK => b.stick_l(),
        ButtonConfig::RSTICK => b.stick_r(),
        _ => false,
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ButtonCombo {
    OpenMenu,
    SaveState,
    LoadState,
    InputRecord,
    InputPlayback,
}

unsafe fn get_combo_keys(combo: ButtonCombo) -> ButtonConfig {
    match combo {
        // For OpenMenu, make it impossible to press so we can just use start press
        ButtonCombo::OpenMenu => ButtonConfig::all(),
        ButtonCombo::SaveState => MENU.save_state_save,
        ButtonCombo::LoadState => MENU.save_state_load,
        ButtonCombo::InputRecord => MENU.input_record,
        ButtonCombo::InputPlayback => MENU.input_playback,
    }
}

lazy_static! {
    static ref BUTTON_COMBO_REQUESTS: Mutex<HashMap<ButtonCombo, bool>> =
        Mutex::new(HashMap::from([
            (ButtonCombo::OpenMenu, false),
            (ButtonCombo::SaveState, false),
            (ButtonCombo::LoadState, false),
            (ButtonCombo::InputRecord, false),
            (ButtonCombo::InputPlayback, false),
        ]));
    static ref START_HOLD_FRAMES: Mutex<u32> = Mutex::new(0);
}

fn combo_passes(p1_controller: Controller, combo: ButtonCombo) -> bool {
    unsafe {
        let combo_keys = get_combo_keys(combo).to_vec();
        let mut this_combo_passes = false;

        for hold_button in &combo_keys[..] {
            if button_mapping(
                *hold_button,
                p1_controller.style,
                p1_controller.current_buttons,
            ) && combo_keys
                .iter()
                .filter(|press_button| **press_button != *hold_button)
                .all(|press_button| {
                    button_mapping(*press_button, p1_controller.style, p1_controller.just_down)
                })
            {
                this_combo_passes = true;
            }
        }

        this_combo_passes
    }
}

pub fn _combo_passes_exclusive(p1_controller: Controller, combo: ButtonCombo) -> bool {
    let other_combo_passes = ButtonCombo::iter()
        .filter(|other_combo| *other_combo != combo)
        .any(|other_combo| combo_passes(p1_controller, other_combo));
    combo_passes(p1_controller, combo) && !other_combo_passes
}

pub fn combo_passes_exclusive(combo: ButtonCombo) -> bool {
    unsafe {
        let button_combo_requests = &mut *BUTTON_COMBO_REQUESTS.data_ptr();
        let passes = button_combo_requests.get_mut(&combo);
        let mut did_pass = false;
        if let Some(passes) = passes {
            if *passes {
                did_pass = true;
            }
            *passes = false;
        }

        did_pass
    }
}

pub fn handle_final_input_mapping(player_idx: i32, controller_struct: &mut SomeControllerStruct) {
    if player_idx == 0 {
        let p1_controller = &mut *controller_struct.controller;
        let mut start_menu_request = false;

        let start_hold_frames = &mut *START_HOLD_FRAMES.lock();
        if p1_controller.current_buttons.plus() {
            *start_hold_frames += 1;
            p1_controller.previous_buttons.set_plus(false);
            p1_controller.current_buttons.set_plus(false);
            p1_controller.just_down.set_plus(false);
            p1_controller.just_release.set_plus(false);
        } else {
            if *start_hold_frames > 0 {
                // Here, we just finished holding start
                if *start_hold_frames < 10 && unsafe { !VANILLA_MENU_ACTIVE } {
                    // If we held for fewer than 10 frames, let's open the training mod menu
                    start_menu_request = true;
                } else if unsafe { !QUICK_MENU_ACTIVE } {
                    // Otherwise, let's let the game know that we had pressed start
                    // So long as our menu isn't active
                    p1_controller.current_buttons.set_plus(true);
                    p1_controller.just_down.set_plus(true);
                    unsafe {
                        VANILLA_MENU_ACTIVE = true;
                    }
                }
            }
            *start_hold_frames = 0;
        }

        let button_combo_requests = &mut *BUTTON_COMBO_REQUESTS.lock();
        button_combo_requests
            .iter_mut()
            .for_each(|(combo, is_request)| {
                if !*is_request {
                    *is_request = _combo_passes_exclusive(*p1_controller, *combo);
                    if *combo == button_config::ButtonCombo::OpenMenu && start_menu_request {
                        *is_request = true;
                    }
                }
            })
    }
}
