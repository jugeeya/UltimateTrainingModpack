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
        if LEDGE_DELAY != NOT_SET {
            LEDGE_DELAY = NOT_SET;
            frame_counter::full_reset(LEDGE_DELAY_COUNTER);
        }
    }
}

pub fn reset_ledge_case() {
    unsafe {
        if LEDGE_CASE != LedgeOption::empty() {
            // Don't roll another ledge option if one is already selected
            LEDGE_CASE = LedgeOption::empty();
        }
    }
}

fn roll_ledge_delay() {
    unsafe {
        if LEDGE_DELAY != NOT_SET {
            // Don't roll another ledge delay if one is already selected
            return;
        }

        LEDGE_DELAY = MENU.ledge_delay.get_random().into_longdelay();
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
    if StatusModule::situation_kind(module_accessor) as i32 != *SITUATION_KIND_CLIFF {
        // No longer on ledge, so re-roll the ledge case and reset the delay counter for next time
        reset_ledge_case();
        reset_ledge_delay();
        return;
    }
    
    // Need to roll ledge delay so we know if getup needs to be buffered
    roll_ledge_delay();
    roll_ledge_case();

    // This flag is false when needing to buffer, and true when getting up
    let flag_cliff = WorkModule::is_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF);
    let current_frame = MotionModule::frame(module_accessor) as i32;
    let should_buffer = (LEDGE_DELAY == 0) && (current_frame == 19) && (!flag_cliff);

    if !WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_ATTACK,
    ) {
        // Not able to take any action yet
        // This check isn't reliable for buffered options in time, so don't return if we need to buffer an option this frame
        if !should_buffer {
            return;
        }
    }

    if LEDGE_CASE == LedgeOption::WAIT {
        // Do nothing, but don't reset the ledge case.
        return;
    }

    if frame_counter::should_delay(LEDGE_DELAY, LEDGE_DELAY_COUNTER) {
        // Not yet time to perform the ledge action
        return;
    }

    let status = LEDGE_CASE.into_status().unwrap_or(0);

    StatusModule::change_status_request_from_script(module_accessor, status, true);

    match LEDGE_CASE {
        LedgeOption::JUMP => {
            mash::buffer_menu_mash();
        }
        _ => mash::perform_defensive_option(),
    }

}

pub unsafe fn is_enable_transition_term(
    _module_accessor: *mut app::BattleObjectModuleAccessor,
    term: i32,
) -> Option<bool> {
    if !is_operation_cpu(&mut *_module_accessor) {
        return None;
    }
    // Only handle ledge scenarios from menu
    if StatusModule::status_kind(_module_accessor) as i32 != *FIGHTER_STATUS_KIND_CLIFF_WAIT
        || MENU.ledge_state == LedgeOption::empty()
    {
        return None;
    }

    // Disallow the default cliff-climb if we are waiting
    if (LEDGE_CASE == LedgeOption::WAIT
        || frame_counter::get_frame_count(LEDGE_DELAY_COUNTER) < LEDGE_DELAY)
        && term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB
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
        if MENU.ledge_state == LedgeOption::empty() {
            return;
        }

        force_option(module_accessor);
    }
}
