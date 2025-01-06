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
        ButtonCombo::SaveState => read(&MENU).save_state_save,
        ButtonCombo::LoadState => read(&MENU).save_state_load,
        ButtonCombo::InputRecord => read(&MENU).input_record,
        ButtonCombo::InputPlayback => read(&MENU).input_playback,
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
static START_RELEASE_FRAMES: RwLock<u32> = RwLock::new(0);

fn _combo_passes(p1_controller: Controller, combo: ButtonCombo) -> bool {
    unsafe {
        // Prevent button combos from passing if either the vanilla or mod menu is open
        if read(&VANILLA_MENU_ACTIVE) || read(&QUICK_MENU_ACTIVE) {
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
    let mut button_combo_requests_lock = lock_write(&BUTTON_COMBO_REQUESTS);
    let passes = (*button_combo_requests_lock).get_mut(&combo);
    let mut did_pass = false;
    if let Some(passes) = passes {
        if *passes {
            did_pass = true;
        }
        *passes = false;
    }
    did_pass
}

fn handle_menu_open_start_press(controller: &mut Controller) -> bool {
    // If we press (-), open the modpack menu.
    // Exception: If the vanilla menu is open, don't open the modpack menu.
    if controller.current_buttons.minus() && !read(&VANILLA_MENU_ACTIVE) {
        return true;
    }

    // If we aren't currently holding start on this frame...
    // and we weren't holding start on the previous frame...
    if !controller.current_buttons.plus() && !controller.previous_buttons.plus() {
        // If we've only been releasing the start button for < 2 frames,
        // end here. This is done in order to ensure that the user has
        // actually released the start button, by waiting for a few more frames.
        if read(&START_RELEASE_FRAMES) < 2 {
            assign(&START_RELEASE_FRAMES, read(&START_RELEASE_FRAMES) + 1);
            return false;
        }
    }

    if controller.current_buttons.plus() {
        assign(&START_RELEASE_FRAMES, 0);

        // If the vanilla menu is open, we don't want to open the modpack menu.
        // Instead, we should just close the vanilla menu.
        if read(&VANILLA_MENU_ACTIVE) {
            assign(&START_HOLD_FRAMES, 0);
            return false;
        }

        assign(&START_HOLD_FRAMES, read(&START_HOLD_FRAMES) + 1);

        // Reset the (+) button state, so that the game doesn't
        // process the (+) button input until we're sure which action to take.
        // (i.e.: open the vanilla menu, open the modpack menu, or do nothing)
        controller.previous_buttons.set_plus(false);
        controller.current_buttons.set_plus(false);
        controller.just_down.set_plus(false);
        controller.just_release.set_plus(false);

        // If we've held the (+) button for more than 10 frames,
        // open the modpack menu.
        if read(&START_HOLD_FRAMES) >= 10 {
            assign(&START_HOLD_FRAMES, 0);
            return true;
        }

        // Don't open the modpack menu (at least, not on this frame).
        return false;
    }

    // If the (+) button was held for 1-10 frames, then released,
    // we should simulate a normal (+) button input, opening the vanilla menu.
    if read(&START_HOLD_FRAMES) > 0 && read(&START_HOLD_FRAMES) < 10 {
        if !read(&QUICK_MENU_ACTIVE)
            && frame_counter::get_frame_count(*MENU_CLOSE_FRAME_COUNTER) == 0
        {
            // If we held for fewer than 10 frames, let's let the game know that
            // we had pressed start
            controller.current_buttons.set_plus(true);
            controller.just_down.set_plus(true);
            assign(&VANILLA_MENU_ACTIVE, true);
        }
        assign(&START_HOLD_FRAMES, 0);
    }

    // Don't open the modpack menu (at least, not on this frame).
    false
}

pub fn handle_final_input_mapping(player_idx: i32, controller_struct: &mut SomeControllerStruct) {
    if player_idx != 0 {
        return;
    }
    let p1_controller = &mut *controller_struct.controller;
    let mut start_menu_request = false;

    if read(&MENU).menu_open_start_press == OnOff::ON {
        start_menu_request = handle_menu_open_start_press(p1_controller);
    }

    let mut button_combo_requests_lock = lock_write(&BUTTON_COMBO_REQUESTS);
    (*button_combo_requests_lock)
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
