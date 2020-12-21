use crate::common::consts::*;
use crate::common::*;
use crate::training::frame_counter;
use crate::training::mash;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

const NOT_SET: u32 = 9001;
static mut LEDGE_DELAY: u32 = NOT_SET;
static mut LEDGE_DELAY_COUNTER: usize = 0;
static mut LEDGE_CASE: LedgeOption = LedgeOption::empty();

pub fn init() {
    unsafe {
        LEDGE_DELAY_COUNTER = frame_counter::register_counter();
    }
}

pub fn reset_ledge_delay() {
    unsafe {
        LEDGE_DELAY = NOT_SET;
    }
}

pub fn reset_ledge_case() {
    unsafe {
        if LEDGE_CASE != LedgeOption::empty() {
            LEDGE_CASE = LedgeOption::empty();
        }
    }
}

fn roll_ledge_delay() {
    unsafe {
        if LEDGE_DELAY != NOT_SET {
            return;
        }

        LEDGE_DELAY = MENU.ledge_delay.get_random().to_index();
    }
}

fn roll_ledge_case() {
    unsafe {
        // Don't re-roll if there is already a ledge option selected
        // This prevents choosing a different ledge option during LedgeOption::WAIT
        if LEDGE_CASE != LedgeOption::empty() {
            return;
        }

        LEDGE_CASE = MENU.ledge_state.get_random();
    }
}

pub unsafe fn force_option(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if StatusModule::status_kind(module_accessor) as i32 != *FIGHTER_STATUS_KIND_CLIFF_WAIT {
        reset_ledge_case(); // No longer on ledge, so re-roll the ledge case next time
        return;
    }

    if !WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_ATTACK,
    ) {
        // Not able to take any action yet
        return;
    }

    roll_ledge_delay();
    roll_ledge_case();

    if LEDGE_CASE == LedgeOption::WAIT {
        // Do nothing, but don't reset the ledge case.
        return;
    }

    if frame_counter::should_delay(LEDGE_DELAY, LEDGE_DELAY_COUNTER) {
        return;
    }

    reset_ledge_delay();

    let status = LEDGE_CASE.into_status().unwrap_or(0);
    match LEDGE_CASE {
        LedgeOption::JUMP => {
            mash::buffer_menu_mash();
        }
        _ => mash::perform_defensive_option(),
    }

    StatusModule::change_status_request_from_script(module_accessor, status, true);
}

pub unsafe fn is_enable_transition_term(
    module_accessor: *mut app::BattleObjectModuleAccessor,
    term: i32,
) -> Option<bool> {
    // Disallow cliff-climb if waiting on ledge per the current menu selection
    if LEDGE_CASE != LedgeOption::WAIT {
        return None;
    }

    if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB {
        return Some(false);
    }

    return None;
}

pub fn get_command_flag_cat(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) {
        return;
    }

    unsafe {
        if MENU.ledge_state == LedgeOption::empty() {
            return;
        }

        force_option(module_accessor);
    }
}
