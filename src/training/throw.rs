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

static mut PUMMEL_DELAY: u32 = NOT_SET;
static mut PUMMEL_DELAY_COUNTER: usize = 0;

pub fn init() {
    unsafe {
        THROW_DELAY_COUNTER = frame_counter::register_counter();
        PUMMEL_DELAY_COUNTER = frame_counter::register_counter();
    }
}

// Rolling Throw Delays and Pummel Delays separately

pub fn reset_throw_delay() {
    unsafe {
        if THROW_DELAY != NOT_SET {
            THROW_DELAY = NOT_SET;
            frame_counter::full_reset(THROW_DELAY_COUNTER);
        }
    }
}

pub fn reset_pummel_delay() {
    unsafe {
        if PUMMEL_DELAY != NOT_SET {
            PUMMEL_DELAY = NOT_SET;
            frame_counter::full_reset(PUMMEL_DELAY_COUNTER);
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

        THROW_DELAY = MENU.throw_delay.get_random().into_meddelay();
    }
}

fn roll_pummel_delay() {
    unsafe {
        if PUMMEL_DELAY != NOT_SET {
            // Don't roll another pummel delay if one is already selected
            return;
        }

        PUMMEL_DELAY = MENU.pummel_delay.get_random().into_meddelay();
    }
}

fn roll_throw_case() {
    unsafe {
        // Don't re-roll if there is already a throw option selected
        if THROW_CASE != ThrowOption::empty() {
            return;
        }

        THROW_CASE = MENU.throw_state.get_random();
    }
}

pub unsafe fn get_command_flag_throw_direction(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> i32 {
    if !is_operation_cpu(module_accessor) {
        return 0;
    }

    if StatusModule::status_kind(module_accessor) as i32 != *FIGHTER_STATUS_KIND_CATCH_WAIT
        && StatusModule::status_kind(module_accessor) as i32 != *FIGHTER_STATUS_KIND_CATCH_PULL
        && StatusModule::status_kind(module_accessor) as i32 != *FIGHTER_STATUS_KIND_CATCH_ATTACK
    {
        // No longer holding character, so re-roll the throw case and reset the delay counter for next time
        reset_throw_case();
        reset_throw_delay();

        reset_pummel_delay();
        return 0;
    }

    if !WorkModule::is_enable_transition_term(
        // If you can't throw right now, don't bother
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI,
    ) {
        return 0;
    }

    roll_throw_delay();
    roll_throw_case();

    roll_pummel_delay();

    if THROW_CASE == ThrowOption::NONE {
        // Do nothing, but don't reroll the throw case.
        return 0;
    }

    if frame_counter::should_delay(THROW_DELAY, THROW_DELAY_COUNTER) {
        // Not yet time to perform the throw action
        if frame_counter::should_delay(PUMMEL_DELAY, PUMMEL_DELAY_COUNTER) {
            // And not yet time to pummel either, so don't do anything
            return 0;
        }

        // If no pummel delay is selected (default), then don't pummel
        if MENU.pummel_delay == MedDelay::empty() {
            return 0;
        }

        // (this conditional would need to be changed to speed up pummelling)
        if StatusModule::status_kind(module_accessor) as i32 == *FIGHTER_STATUS_KIND_CATCH_WAIT {
            let status = *FIGHTER_STATUS_KIND_CATCH_ATTACK; //.unwrap_or(0);
            StatusModule::change_status_request_from_script(module_accessor, status, true);
        }

        return 0;
    }

    // If you can uthrow, then throw (since all throws should be possible at the same times)
    if WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI,
    ) {
        let cmd = THROW_CASE.into_cmd().unwrap_or(0);
        mash::buffer_menu_mash();
        return cmd;
    }

    return 0;
}
