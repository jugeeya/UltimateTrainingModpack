use std::collections::HashMap;

use crate::common::menu::{MENU_CLOSE_FRAME_COUNTER, QUICK_MENU_ACTIVE};
use crate::common::ButtonConfig;
use crate::input::{ControllerStyle::*, *};
use crate::training::frame_counter;
use crate::training::ui::menu::VANILLA_MENU_ACTIVE;

use training_mod_consts::{OnOff, MENU};
use training_mod_sync::*;

use strum_macros::EnumIter;

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
        _ => panic!("Invalid value in button_mapping: {}", button_config),
    }
}

pub fn name_to_font_glyph(button: ButtonConfig, style: ControllerStyle) -> Option<u16> {
    let is_gcc = style == ControllerStyle::GCController;
    Some(match button {
        ButtonConfig::A => 0xE0E0,
        // TODO: Find one that works in training...
        ButtonConfig::B => 0xE0E0,
        ButtonConfig::X => {
            if is_gcc {
                0xE206
            } else {
                0xE0E2
            }
        }
        ButtonConfig::Y => {
            if is_gcc {
                0xE207
            } else {
                0xE0E3
            }
        }
        ButtonConfig::L => {
            if is_gcc {
                return None;
            } else {
                0xE0E4
            }
        }
        ButtonConfig::R => {
            if is_gcc {
                0xE205
            } else {
                0xE0E5
            }
        }
        ButtonConfig::ZL => {
            if is_gcc {
                0xE204
            } else {
                0xE0E6
            }
        }
        ButtonConfig::ZR => {
            if is_gcc {
                0xE208
            } else {
                0xE0E7
            }
        }
        ButtonConfig::DPAD_UP => {
            if is_gcc {
                0xE209
            } else {
                0xE079
            }
        }
        ButtonConfig::DPAD_DOWN => {
            if is_gcc {
                0xE20A
            } else {
                0xE07A
            }
        }
        ButtonConfig::DPAD_LEFT => {
            if is_gcc {
                0xE20B
            } else {
                0xE07B
            }
        }
        ButtonConfig::DPAD_RIGHT => {
            if is_gcc {
                0xE20C
            } else {
                0xE07C
            }
        }
        ButtonConfig::PLUS => {
            if is_gcc {
                0xE20D
            } else {
                0xE0EF
            }
        }
        ButtonConfig::MINUS => {
            if is_gcc {
                return None;
            } else {
                0xE0F0
            }
        }
        ButtonConfig::LSTICK => {
            if is_gcc {
                return None;
            } else {
                0xE104
            }
        }
        ButtonConfig::RSTICK => {
            if is_gcc {
                return None;
            } else {
                0xE105
            }
        }
        _ => return None,
    })
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ButtonCombo {
    OpenMenu,
    SaveState,
    LoadState,
    InputRecord,
    InputPlayback,
}

pub const DEFAULT_OPEN_MENU_CONFIG: ButtonConfig = ButtonConfig {
    B: 1,
    DPAD_UP: 1,
    ..ButtonConfig::empty()
};

unsafe fn get_combo_keys(combo: ButtonCombo) -> ButtonConfig {
    match combo {
        // For OpenMenu, have a default in addition to accepting start press
        ButtonCombo::OpenMenu => DEFAULT_OPEN_MENU_CONFIG,
        ButtonCombo::SaveState => get(&MENU).save_state_save,
        ButtonCombo::LoadState => get(&MENU).save_state_load,
        ButtonCombo::InputRecord => get(&MENU).input_record,
        ButtonCombo::InputPlayback => get(&MENU).input_playback,
    }
}

// Note: in addition to RwLock we also need a LazyLock initializer because HashMap::from() is not const
static BUTTON_COMBO_REQUESTS: LazyLock<RwLock<HashMap<ButtonCombo, bool>>> = LazyLock::new(|| {
    RwLock::new(HashMap::from([
        (ButtonCombo::OpenMenu, false),
        (ButtonCombo::SaveState, false),
        (ButtonCombo::LoadState, false),
        (ButtonCombo::InputRecord, false),
        (ButtonCombo::InputPlayback, false),
    ]))
});
static START_HOLD_FRAMES: RwLock<u32> = RwLock::new(0);

fn _combo_passes(p1_controller: Controller, combo: ButtonCombo) -> bool {
    unsafe {
        // Prevent button combos from passing if either the vanilla or mod menu is open
        if read_rwlock(&VANILLA_MENU_ACTIVE) || read_rwlock(&QUICK_MENU_ACTIVE) {
            return false;
        }

        let combo_keys = get_combo_keys(combo).to_vec();
        let mut this_combo_passes = false;

        for hold_button in combo_keys.iter() {
            if button_mapping(
                *hold_button,
                p1_controller.style,
                p1_controller.current_buttons,
            ) && combo_keys
                .iter()
                .filter(|press_button| press_button != &hold_button)
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

pub fn combo_passes(combo: ButtonCombo) -> bool {
    let mut button_combo_requests_guard = lock_write_rwlock(&BUTTON_COMBO_REQUESTS);
    let passes = (*button_combo_requests_guard).get_mut(&combo);
    let mut did_pass = false;
    if let Some(passes) = passes {
        if *passes {
            did_pass = true;
        }
        *passes = false;
    }
    drop(button_combo_requests_guard);
    did_pass
}

pub fn handle_final_input_mapping(player_idx: i32, controller_struct: &mut SomeControllerStruct) {
    if player_idx == 0 {
        let p1_controller = &mut *controller_struct.controller;
        let mut start_menu_request = false;

        let menu_close_wait_frame = frame_counter::get_frame_count(*MENU_CLOSE_FRAME_COUNTER);
        if get(&MENU).menu_open_start_press == OnOff::ON {
            let mut start_hold_frames = read_rwlock(&START_HOLD_FRAMES);
            if p1_controller.current_buttons.plus() {
                start_hold_frames += 1;
                p1_controller.previous_buttons.set_plus(false);
                p1_controller.current_buttons.set_plus(false);
                p1_controller.just_down.set_plus(false);
                p1_controller.just_release.set_plus(false);
                if start_hold_frames >= 10 && !read_rwlock(&VANILLA_MENU_ACTIVE) {
                    // If we've held for more than 10 frames,
                    // let's open the training mod menu
                    start_menu_request = true;
                }
            } else {
                // Here, we just finished holding start
                if start_hold_frames > 0
                    && start_hold_frames < 10
                    && !read_rwlock(&QUICK_MENU_ACTIVE)
                    && menu_close_wait_frame == 0
                {
                    // If we held for fewer than 10 frames, let's let the game know that
                    // we had pressed start
                    p1_controller.current_buttons.set_plus(true);
                    p1_controller.just_down.set_plus(true);
                    assign_rwlock(&VANILLA_MENU_ACTIVE, true);
                }
                start_hold_frames = 0;
            }
            assign_rwlock(&START_HOLD_FRAMES, start_hold_frames);

            // If we ever press minus, open the mod menu
            if p1_controller.current_buttons.minus() {
                start_menu_request = true;
            }
        }

        let mut button_combo_requests_guard = lock_write_rwlock(&BUTTON_COMBO_REQUESTS);
        (*button_combo_requests_guard)
            .iter_mut()
            .for_each(|(combo, is_request)| {
                if !*is_request {
                    *is_request = _combo_passes(*p1_controller, *combo);
                    if *combo == ButtonCombo::OpenMenu && start_menu_request {
                        *is_request = true;
                    }
                }
            })
    }
}
