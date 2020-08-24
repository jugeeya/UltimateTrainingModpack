use crate::common::consts::*;
use crate::common::*;
use crate::training::frame_counter;
use crate::training::mash;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

const NOT_SET :u32 = 9001;
static mut LEDGE_DELAY: u32 = NOT_SET;
static mut LEDGE_DELAY_COUNTER: usize = 0;

pub fn init() {
    unsafe {
        LEDGE_DELAY_COUNTER = frame_counter::register_counter();
    }
}

pub fn reset_ledge_delay(){
    unsafe{
        LEDGE_DELAY = NOT_SET;
    }
}

fn roll_ledge_delay(){
    unsafe{
        if LEDGE_DELAY !=  NOT_SET {
            return;
        }

        LEDGE_DELAY = MENU.ledge_delay.get_random().to_index();
    }
}

pub unsafe fn force_option(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if StatusModule::status_kind(module_accessor) as i32 != *FIGHTER_STATUS_KIND_CLIFF_WAIT {
        return;
    }

    roll_ledge_delay();

    if !WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB,
    ) {
        return;
    }

    if frame_counter::should_delay(LEDGE_DELAY, LEDGE_DELAY_COUNTER) {
        return;
    }

    reset_ledge_delay();

    let ledge_case: LedgeOption = MENU.ledge_state.get_random();
    let status = ledge_case.into_status().unwrap_or(0);

    match ledge_case {
        LedgeOption::JUMP => {
            mash::buffer_menu_mash();
        }
        _ => mash::perform_defensive_option(),
    }

    StatusModule::change_status_request_from_script(module_accessor, status, true);
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) {
    // Only do once per frame
    if category != FIGHTER_PAD_COMMAND_CATEGORY1 {
        return;
    }

    if !is_training_mode() {
        return;
    }

    if !is_operation_cpu(module_accessor) {
        return;
    }

    if MENU.ledge_state == LedgeOption::empty() {
        return;
    }

    force_option(module_accessor);
}