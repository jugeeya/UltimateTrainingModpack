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

pub fn name_to_font_glyph(name: &str, style: ControllerStyle) -> Option<u16> {
    let is_gcc = style == ControllerStyle::GCController;
    let button = ButtonConfig::from_name(name)?;
    Some(match button {
        ButtonConfig::A => {
            if is_gcc {
                0xE202
            } else {
                0xE0E0
            }
        }
        ButtonConfig::B => {
            if is_gcc {
                0xE203
            } else {
                0xE0E1
            }
        }
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
                0xE0EB
            }
        }
        ButtonConfig::DPAD_DOWN => {
            if is_gcc {
                0xE20A
            } else {
                0xE0EC
            }
        }
        ButtonConfig::DPAD_LEFT => {
            if is_gcc {
                0xE20B
            } else {
                0xE0ED
            }
        }
        ButtonConfig::DPAD_RIGHT => {
            if is_gcc {
                0xE20C
            } else {
                0xE0EE
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
