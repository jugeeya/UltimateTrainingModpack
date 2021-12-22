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

        THROW_DELAY = MENU.throw_delay.get_random().into_delay(); // NEW! removed into long delay, 
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
/*
pub unsafe fn force_option(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if StatusModule::status_kind(module_accessor) as i32 != *FIGHTER_STATUS_KIND_CATCH_WAIT {
        // No longer holding character, so re-roll the throw case and reset the delay counter for next time
        return;
    }

    if !WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI,
    ) {
        // NEW! Can you add all 4 "FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW"s?
		// Can you just use one of them? What does this code snippet actually do?
        // I assume that it just checks get up attack originally because if you can't get up attack,
        // you can't do any ledge option. So I'll make it up throw for now.
        // Not able to take any action yet
        return;
    }

    let status = *FIGHTER_STATUS_KIND_CATCH_WAIT;

    match THROW_CASE { // Perform mash after throwing
        _ => mash::buffer_menu_mash(),
    };

    //WorkModule::set_flag(module_accessor, true, *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B);
    StatusModule::change_status_request_from_script(module_accessor, status, true);

}
*/
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
    /*
    if (THROW_CASE == ThrowOption::WAIT
        || frame_counter::get_frame_count(THROW_DELAY_COUNTER) < THROW_DELAY)
        && term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB
		// NEW! Maybe make this a transition ESCAPE ?
    {
        return Some(false);
    }
    */
    None
}

/*pub fn get_command_flag_cat(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) {
        return;
    }

    unsafe {
        if MENU.throw_state == ThrowOption::empty() {
            return;
        }

        force_option(module_accessor);
        //WorkModule::set_flag(module_accessor, true, *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B);
        // *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B // Need to fix!!! Maybe this function needs to return?
    }
}*/

pub unsafe fn get_command_flag_throw_direction(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) -> i32 {
    // Only do once per frame
    /*if category != FIGHTER_PAD_COMMAND_CATEGORY1 {
        return 0;
    }*/

    if !is_operation_cpu(module_accessor) {
        return 0;
    }
    
    if StatusModule::status_kind(module_accessor) as i32 != *FIGHTER_STATUS_KIND_CATCH_WAIT {
        // No longer holding character, so re-roll the throw case and reset the delay counter for next time
        // Does this really need to be checked every frame?
        reset_throw_case();
        reset_throw_delay();
        return 0;
    }
    
    if !WorkModule::is_enable_transition_term( // If you can't throw right now, don't bother
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI,
    ) {
        return 0;
    }

    roll_throw_delay();
    roll_throw_case();

    if THROW_CASE == ThrowOption::NONE {
        // Do nothing, but don't reset the throw case.
        return 0;
    }

    if frame_counter::should_delay(THROW_DELAY, THROW_DELAY_COUNTER) {
        // Not yet time to perform the throw action

        /*
        // Add Pummel Handling here?
        if StatusModule::status_kind(module_accessor) as i32 == *FIGHTER_STATUS_KIND_CATCH_WAIT {
            
            //if PUMMEL_CASE == True/PummelOption::PUMMEL { // - Pummel options so you can have cpu mix up between pummeling and not pummeling
                let pummelCmd = PUMMEL_CASE.into_CMD().unwrap_or(0); //THROW_CASE.into_CMD().unwrap_or(0);
                return pummelCmd;
            //}

        }
        // Need to buffer the throw mash?
        */
        return 0;
    }


    // Need to deal with NONE as well?
    if StatusModule::status_kind(module_accessor) as i32 == *FIGHTER_STATUS_KIND_CATCH_WAIT {
        let cmd = THROW_CASE.into_CMD().unwrap_or(0);
        mash::buffer_menu_mash();
        return cmd; // if throwing, with CATCH_WAIT and doesn't catch delay
    }
    
    return 0;
}