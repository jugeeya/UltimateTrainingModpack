use crate::common::menu::P1_CONTROLLER_STATE;
use crate::common::*;
use crate::input::{ControllerStyle::*, *};

use strum::IntoEnumIterator;
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
        _ => false,
    }
}

#[derive(Debug, EnumIter, PartialEq)]
pub enum ButtonCombo {
    OpenMenu,
    SaveState,
    LoadState,
    InputRecord,
    InputPlayback,
}

unsafe fn get_combo_keys(combo: ButtonCombo) -> ButtonConfig {
    match combo {
        ButtonCombo::OpenMenu => MENU.menu_open,
        ButtonCombo::SaveState => MENU.save_state_save,
        ButtonCombo::LoadState => MENU.save_state_load,
        ButtonCombo::InputRecord => MENU.input_record,
        ButtonCombo::InputPlayback => MENU.input_playback,
    }
}

fn combo_passes(combo: ButtonCombo) -> bool {
    unsafe {
        let combo_keys = get_combo_keys(combo).to_vec();
        let p1_controller_state = *P1_CONTROLLER_STATE.data_ptr();

        let mut this_combo_passes = false;

        for hold_button in &combo_keys[..] {
            if button_mapping(
                *hold_button,
                p1_controller_state.style,
                p1_controller_state.current_buttons,
            ) && combo_keys
                .iter()
                .filter(|press_button| **press_button != *hold_button)
                .all(|press_button| {
                    button_mapping(
                        *press_button,
                        p1_controller_state.style,
                        p1_controller_state.just_down,
                    )
                })
            {
                this_combo_passes = true;
            }
        }

        this_combo_passes
    }
}

pub fn combo_passes_exclusive(combo: ButtonCombo) -> bool {
    let other_combo_passes = ButtonCombo::iter()
        .filter(|other_combo| *other_combo != combo)
        .any(combo_passes);
    combo_passes(combo) && !other_combo_passes
}
