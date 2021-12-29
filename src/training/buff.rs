use crate::common::consts::*;
use smash::hash40;
//use crate::common::consts::FighterId;
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
    SoundModule::stop_all_sound(module_accessor); // should silence voice lines etc. need to test on every buff
    if fighter_kind == *FIGHTER_KIND_BRAVE {
        return buff_hero(module_accessor,status);
    } else if fighter_kind == *FIGHTER_KIND_JACK {
        return buff_joker(module_accessor,status);
    } else if fighter_kind == *FIGHTER_KIND_WIIFIT {
        return buff_wiifit(module_accessor,status);
    } else if fighter_kind == *FIGHTER_KIND_CLOUD {
        return buff_cloud(module_accessor, status);
    }
    return true;
}

unsafe fn buff_hero(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    if prev_status_kind == FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START { //&& buffs_remaining = 0 // If finished applying buffs, need to have some kind of struct responsible
        return true;
    }
    if status != FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START {
        WorkModule::set_int(module_accessor, 10, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_DECIDE_COMMAND);
        StatusModule::change_status_force( // _request_from_script? - no, because you need to override shield buffer
            module_accessor,
            *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START,
            false,
        );
    } else {
        MotionModule::set_rate(module_accessor, 40.0);
    }
    return false;
}

unsafe fn buff_cloud(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    //WorkModule::set_flag(module_accessor, true, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_GAUGE_CHARGE);
    //WorkModule::set_flag(module_accessor, true, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
    //return true; // change to false when implemented
    //FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_GAUGE_CHARGE
    //FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK
    //FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM
    //FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL
    //FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE
    //FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_CATCH_CUT_DAMAGE
    //FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE_NOTICE

    //Prevent beginning from being shield cancellable? The limit end animation is a problem too. Maybe params aren't the best way to
    //  go about initially setting the status here (but should probably still be used to allow for instant limit charge? Not sure)

    println!("limit_gauge_add = {}", hash40("limit_gauge_add"));
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    if prev_status_kind == FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE {
        return true;
    }
    if status != FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE {
        StatusModule::change_status_force(
            module_accessor,
            *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE,
            false,
        );
    } else {
        MotionModule::set_rate(module_accessor, 300.0); // frame count for limit? Too high?
    }
    return false;

}

unsafe fn buff_joker(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    // call function to add/set rebel gauge
    // if none exists, edit param over time to give you exact gauge needed for arsene (or more) when loading save state
    // if can't figure that out, super speed up rebel's guard attack and make it give insane meter or something
    // Also, maybe try to make the arsene voice call not happen?
    let entry_id = app::FighterEntryID(FighterId::Player as i32); // may need to be 0? // May want to apply to CPU? For 2 framing?
    app::FighterSpecializer_Jack::add_rebel_gauge(module_accessor, entry_id, 120.0); // Why do I need to use app:: when I don't need to for other Modules?
    // prevent cutscene, may want to do in enable_transition_term or something
    
    //MotionModule::set_frame(48.0); // try these
    //CancelModule::enable_cancel(boma);
    
    /*if transition_term == *FIGHTER_JACK_STATUS_KIND_SUMMON {
        return false;
    }*/ // in mod.rs /training didn't work

    //WorkModule::unable_transition_term(module_accessor, *FIGHTER_JACK_STATUS_KIND_SUMMON);
    //WorkModule::enable_transition_term_forbid(module_accessor, *FIGHTER_JACK_STATUS_KIND_SUMMON);
   
    /*let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    if status == FIGHTER_JACK_STATUS_KIND_SUMMON { //&& buffs_remaining = 0 // If finished applying buffs, need to have some kind of struct responsible
        StatusModule::change_status_force( // _request_from_script?
            module_accessor,
            prev_status_kind,
            false,
        );
    }*/
    
    // add some check for if Arsene is actually out here?
    return true; // change to false when implemented
}

unsafe fn _buff_mac(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    return true; // change to false when implemented
    //FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE
}

unsafe fn _buff_sepiroth(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    return true; // change to false when implemented
}

unsafe fn buff_wiifit(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    if prev_status_kind == FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS {
        return true;
    }
    if status != FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS {
        StatusModule::change_status_force(
            module_accessor,
            *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS,
            false,
        );
    } else {
        MotionModule::set_rate(module_accessor, 40.0); // frame count for deep breathing???
    }
    return false;
}
