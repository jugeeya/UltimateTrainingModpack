use crate::common::consts::*;
use crate::common::*;
use crate::training::frame_counter;
use crate::training::mash;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

/*const NOT_SET: u32 = 9001;
static mut THROW_DELAY: u32 = NOT_SET;
static mut THROW_DELAY_COUNTER: usize = 0;
static mut THROW_CASE: ThrowOption = ThrowOption::empty();

pub fn init() {
    unsafe {
        THROW_DELAY_COUNTER = frame_counter::register_counter();
    }
}*/

/*
// Rolling Throw Delays and Pummel Delays separately

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

        THROW_DELAY = MENU.throw_delay.get_random().into_meddelay();
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
*/

/*
pub unsafe fn get_command_flag_throw_direction(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {

}
*/

pub unsafe fn handle_buffs(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, status: i32) -> bool {
    if fighter_kind == *FIGHTER_KIND_BRAVE {
        return buff_hero(module_accessor,status);
    }
    return true;
}

unsafe fn buff_hero(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    println!("Status: {}, Prev: {}", status, prev_status_kind);
    if prev_status_kind == FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START { //&& buffs_remaining = 0 // If finished applying buffs, need to have some kind of struct responsible
        return true;
    }
    if status != FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START {
        WorkModule::set_int(module_accessor, 10, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_DECIDE_COMMAND);
        StatusModule::change_status_force( // _request_from_script?
            module_accessor,
            *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START,
            false,
        );
    } else {
        MotionModule::set_rate(module_accessor, 40.0);
    }
    return false;
}

unsafe fn _buff_cloud(module_accessor: &mut app::BattleObjectModuleAccessor) {

}

unsafe fn _buff_joker(module_accessor: &mut app::BattleObjectModuleAccessor) {

}

unsafe fn _buff_mac(module_accessor: &mut app::BattleObjectModuleAccessor) {

}

unsafe fn _buff_sepiroth(module_accessor: &mut app::BattleObjectModuleAccessor) {

}

unsafe fn _buff_wiifit(module_accessor: &mut app::BattleObjectModuleAccessor) {

}
