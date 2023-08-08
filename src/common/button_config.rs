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
            _ => b.zl(),
        },
        ButtonConfig::ZR => match style {
            GCController => b.r() || b.real_digital_r(),
            _ => b.zr(),
        },
        ButtonConfig::DpadUp => b.dpad_up(),
        ButtonConfig::DpadDown => b.dpad_down(),
        ButtonConfig::DpadLeft => b.dpad_left(),
        ButtonConfig::DpadRight => b.dpad_right(),
        ButtonConfig::Plus => b.plus(),
        ButtonConfig::Minus => b.minus(),
        ButtonConfig::LStick => b.stick_l(),
        ButtonConfig::RStick => b.stick_r(),
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

unsafe fn get_combo_keys(combo: ButtonCombo) -> (ButtonConfig, ButtonConfig) {
    match combo {
        ButtonCombo::OpenMenu => (MENU.menu_open_hold, MENU.menu_open_press),
        ButtonCombo::SaveState => (MENU.save_state_save_hold, MENU.save_state_save_press),
        ButtonCombo::LoadState => (MENU.save_state_load_hold, MENU.save_state_load_press),
        ButtonCombo::InputRecord => (MENU.input_record_hold, MENU.input_record_press),
        ButtonCombo::InputPlayback => (MENU.input_playback_hold, MENU.input_playback_press),
    }
}

fn combo_passes(combo: ButtonCombo) -> bool {
    unsafe {
        let (hold, press) = get_combo_keys(combo);
        let p1_controller_state = *P1_CONTROLLER_STATE.data_ptr();

        button_mapping(
            hold,
            p1_controller_state.style,
            p1_controller_state.current_buttons,
        ) && button_mapping(
            press,
            p1_controller_state.style,
            p1_controller_state.just_down,
        )
    }
}

pub fn combo_passes_exclusive(combo: ButtonCombo) -> bool {
    let other_combo_passes = ButtonCombo::iter()
        .filter(|other_combo| *other_combo != combo)
        .any(combo_passes);
    combo_passes(combo) && !other_combo_passes
}
