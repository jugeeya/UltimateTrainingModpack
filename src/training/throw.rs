use crate::common::consts::*;
use crate::common::*;
use crate::training::frame_counter;
use crate::training::mash;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

const NOT_SET: u32 = 9001;
static mut THROW_DELAY: u32 = NOT_SET;
static mut THROW_DELAY_COUNTER: usize = 0;
static mut THROW_CASE: ThrowOption = ThrowOption::empty();

pub fn init() {
    unsafe {
        THROW_DELAY_COUNTER = frame_counter::register_counter();
    }
}

pub fn reset_throw_delay() {
    unsafe {
        if THROW_DELAY != NOT_SET {
            THROW_DELAY = NOT_SET;
            frame_counter::full_reset(THROW_DELAY_COUNTER);
        }
    }
}

pub fn reset_throw_case() {
    unsafe {
        if THROW_CASE != ThrowOption::empty() {
            // Don't roll another throw option if one is already selected
            THROW_CASE = ThrowOption::empty();
        }
    }
}

fn roll_throw_delay() {
    unsafe {
        if THROW_DELAY != NOT_SET {
            // Don't roll another throw delay if one is already selected
            return;
        }

        THROW_DELAY = MENU.throw_delay.get_random.into_delay(); // NEW! removed into long delay, 
													 // assuming it's why ledge options 
													 // are increments of 10 instead of 1.into_longdelay();
    }
}

fn roll_throw_case() {
    unsafe {
        // Don't re-roll if there is already a throw option selected
        // This prevents choosing a different throw option during ThrowOption::WAIT
        if THROW_CASE != ThrowOption::empty() {
            return;
        }

        THROW_CASE = MENU.throw_state.get_random();
    }
}

pub unsafe fn force_option(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if StatusModule::status_kind(module_accessor) as i32 != *FIGHTER_STATUS_KIND_CATCH_WAIT {
        // No longer holding character, so re-roll the throw case and reset the delay counter for next time
        reset_throw_case();
        reset_throw_delay();
        return;
    }

    if !WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_ATTACK,
		// NEW! Can you add all 4 "FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW"s?
		// Can you just use one of them? What does this code snippet actually do?
    ) {
        // Not able to take any action yet
        return;
    }

    roll_throw_delay();
    roll_throw_case();

    if THROW_CASE == ThrowOption::WAIT {
        // Do nothing, but don't reset the throw case.
        return;
    }

    if frame_counter::should_delay(THROW_DELAY, THROW_DELAY_COUNTER) {
        // Not yet time to perform the throw action
        return;
    }

    let status = THROW_CASE.into_status().unwrap_or(0);
    match THROW_CASE { // NEW! Should I change ThrowOption JUMP to always mash here? Or always use a defensive option?
						// Because a throw means that grab is a mash or followup. Always do a defensive option?
        ThrowOption::JUMP => {
            mash::buffer_menu_mash();
        }
        _ => mash::perform_defensive_option(),
    }

    StatusModule::change_status_request_from_script(module_accessor, status, true);
}

pub unsafe fn is_enable_transition_term(
    _module_accessor: *mut app::BattleObjectModuleAccessor,
    term: i32,
) -> Option<bool> {
    if !is_operation_cpu(&mut *_module_accessor) {
        return None;
    }
    // NEW! What does any of this function do? Not entirely sure. Switched to CATCH from CLIFF for now.
	// Only handle throw scenarios from menu
    if StatusModule::status_kind(_module_accessor) as i32 != *FIGHTER_STATUS_KIND_CATCH_WAIT
        || MENU.throw_state == ThrowOption::empty()
    {
        return None;
    }

    // NEW! There is no default throw option, outside of grab release. Most likely should remove,
	// but could be used to override regular mashing/percent windows to force X pummels and a throw or something
	// Disallow the default cliff-climb if we are waiting
    if (THROW_CASE == ThrowOption::WAIT
        || frame_counter::get_frame_count(THROW_DELAY_COUNTER) < THROW_DELAY)
        && term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB
		// NEW! Maybe make this a transition ESCAPE ?
    {
        return Some(false);
    }
    None
}

pub fn get_command_flag_cat(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) {
        return;
    }

    unsafe {
        if MENU.throw_state == ThrowOption::empty() {
            return;
        }

        force_option(module_accessor);
    }
}

