use crate::common::consts::*;
use crate::common::*;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

/**
 * This is needed to have the CPU put up shield
 */
pub unsafe fn check_button_on(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> Option<bool> {
    if should_return_none_in_check_button(module_accessor, button) {
        return None;
    }
    Some(true)
}

/**
 * This is needed to prevent dropping shield immediately
 */
pub unsafe fn check_button_off(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> Option<bool> {
    if should_return_none_in_check_button(module_accessor, button) {
        return None;
    }
    Some(false)
}

/**
 * AKA should the cpu hold the jump button
 */
unsafe fn should_return_none_in_check_button(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> bool {
    if !is_training_mode() {
        return true;
    }

    if !is_operation_cpu(module_accessor) {
        return true;
    }

    if ![*CONTROL_PAD_BUTTON_JUMP, *CONTROL_PAD_BUTTON_FLICK_JUMP].contains(&button) {
        return true;
    }

    if MENU.full_hop != OnOff::On{
        return true;
    }

    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    if status_kind != FIGHTER_STATUS_KIND_JUMP_SQUAT {
        return true;
    }

    false
}
