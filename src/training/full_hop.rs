use crate::common::*;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

// the current full hop status
static mut FULL_HOP: bool = false;

pub fn should_full_hop() -> bool {
    unsafe{
        FULL_HOP
    }
}

pub fn roll_full_hop() {
    unsafe {
        FULL_HOP = MENU.full_hop.get_random().into_bool();
    }
}

/**
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
    if !is_operation_cpu(module_accessor) {
        return true;
    }

    // We only care about the jump button
    if ![*CONTROL_PAD_BUTTON_JUMP, *CONTROL_PAD_BUTTON_FLICK_JUMP].contains(&button) {
        return true;
    }

    // Nothing to do if not toggled
    if !should_full_hop() {
        return true;
    }

    // Only need to hold during jump squat
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    if status_kind != FIGHTER_STATUS_KIND_JUMP_SQUAT {
        return true;
    }

    false
}
