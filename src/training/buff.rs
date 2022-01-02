use crate::common::consts::*;
use crate::common::*;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::training::cloud_func_hook;

static mut BUFF_REMAINING: i32 = 0;
static mut NUM_OPERATIONS: i32 = 0;
static mut IS_BUFFING: bool = false;

pub fn restart_buff() {
    unsafe {
        IS_BUFFING = false;
    }
}

fn get_spell_vec() -> Vec<BuffOption> { // prob unneeded
    unsafe {
        //let spell_buff = vec![BuffOption::OOMPH,BuffOption::PSYCHE,BuffOption::BOUNCE,BuffOption::ACCELERATLE];
        let menu_buff = MENU.buff_state.to_vec();
        let menu_iter = menu_buff.iter();
        let mut spell_buff: Vec<BuffOption> = Vec::new();
        for buff in menu_iter {
            if buff.into_int().unwrap_or(1) != 1 { // all non-spells into_int as 1. Maybe should be 0 instead?
                spell_buff.push(*buff); 
            }
        }
        return spell_buff;
    }
}

pub unsafe fn handle_buffs(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, status: i32, percent: f32) -> bool {
    SoundModule::stop_all_sound(module_accessor); // should silence voice lines etc. need to test on every buff
    //MotionAnimcmdModule::set_sleep(module_accessor, false); // does this prevent all the anims?
    SoundModule::pause_se_all(module_accessor, false);
    ControlModule::stop_rumble(module_accessor, false);
    //KineticModule::clear_speed_all(module_accessor);
    //ShakeModule::stop(module_accessor); // doesn't work?
    //CameraModule::stop_quake(module_accessor, 60); // doesn't work
    //app::sv_animcmd::QUAKE_STOP(60); crashes game very cool

    //fix psyche up camera shake?
    // This cannot be a match statement because of the pointer derefrencing, 
    // though you may be able to write something smarter than this like iter over a tuple of your pointer values and use find() or position()
    // unsure if the above idea has any merit though

    let menu_vec = MENU.buff_state.to_vec();

    if fighter_kind == *FIGHTER_KIND_BRAVE {
        return buff_hero(module_accessor,status);
    } else if fighter_kind == *FIGHTER_KIND_JACK && menu_vec.contains(&BuffOption::ARSENE) {
        return buff_joker(module_accessor,status);
    } else if fighter_kind == *FIGHTER_KIND_WIIFIT && menu_vec.contains(&BuffOption::BREATHING) {
        return buff_wiifit(module_accessor,status);
    } else if fighter_kind == *FIGHTER_KIND_CLOUD && menu_vec.contains(&BuffOption::LIMIT) {
        return buff_cloud(module_accessor, status);
    } else if fighter_kind == *FIGHTER_KIND_LITTLEMAC && menu_vec.contains(&BuffOption::KO) {
        return buff_mac(module_accessor, status);
    } else if fighter_kind == *FIGHTER_KIND_EDGE && menu_vec.contains(&BuffOption::WING) {
        return buff_sepiroth(module_accessor, percent);
    }

    return true;
}

// Probably should have some vector of the statuses selected on the Menu, and for each status you
// have the framecounter delay be its index (0 for first, 1 frame/index for second, etc.), this probably goes backwards? Unsure
// probably better to just always call the first if its not empty, but won't cause idk how

unsafe fn buff_hero(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    let buff_vec = get_spell_vec();
    if !IS_BUFFING { // should I do 0 or 1? Initial set up for spells
        IS_BUFFING = true; // This should be fine, as starting it multiple times per frame shouldn't be an issue. Start counting
        NUM_OPERATIONS = 0;
        BUFF_REMAINING = buff_vec.len() as i32; // since its the first frame, we need to set up how many buffs there are
    } // else { // commands to use if we're buffing, does this else need to be here?
    if BUFF_REMAINING <= 0 { // If there are no buffs selected/left, get out of here
        return true; // this may be needed since we're casting a potential -1 to usize?
    }
    buff_hero_single(module_accessor, status, buff_vec);
    return false;
}

unsafe fn buff_hero_single(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32, buff_vec: Vec<BuffOption>) {
    NUM_OPERATIONS += 1;
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    if prev_status_kind == FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START { //&& buffs_remaining = 0 // If finished applying buffs, need to have some kind of struct responsible
        BUFF_REMAINING -= 1;
    }
    // need to handle finding the buff in here due to the above if statement, probably should do in a function
    let spell_index = BUFF_REMAINING - 1; // as usize here? var used to get spell from our vector
    let spell_option = buff_vec.get(spell_index as usize);
    if spell_option.is_none() { // there are no spells selected, or something went wrong with making the vector
        return;
    }
    let real_spell_value = spell_option.unwrap().into_int().unwrap();
    if status != FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START && BUFF_REMAINING != 0 { // probably needed
        WorkModule::set_int(module_accessor, real_spell_value, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_DECIDE_COMMAND); // not being set at right time
        StatusModule::change_status_force( // does this crash if forcing while already in the status?
            module_accessor,
            *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START,
            true, // true to prevent shielding over
        );
    } 
    if status == FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START {
        MotionModule::set_rate(module_accessor, 50.0); //needs to be at least 46 for psyche up?
    }
}
/*
unsafe fn _buff_cloud(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    println!("Next Status: {}", StatusModule::status_kind_next(module_accessor));
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    if prev_status_kind == FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END {
        //KineticModule::clear_speed_all(module_accessor);
        return true;
    }
    if !IS_BUFFING {
        IS_BUFFING = true;
        WorkModule::set_float(module_accessor, 100.0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
        StatusModule::change_status_request_from_script( // not doing from_script crashes the game here
            module_accessor,
            *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE,
            true, // originally false, probably should be true though so inputs aren't interfered with as we go through multiple buffs
        );
    } 
    MotionModule::set_rate(module_accessor, 50.0);
    return false;
}
*/
unsafe fn buff_cloud(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    //WorkModule::set_float(module_accessor, 99.0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
    cloud_func_hook(100.0,module_accessor,0);
    return true;
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

unsafe fn buff_mac(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    WorkModule::set_float(module_accessor, 100.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE); // Sets meter to proper amount
    // Need to figure out how to update the KO meter. Probably a fighter specializer function? Maybe can just put him in hitstop though, unsure
    return true; // change to false when implemented
}

unsafe fn buff_sepiroth(module_accessor: &mut app::BattleObjectModuleAccessor, percent: f32) -> bool {
    //WorkModule::on_flag(module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED);
    //~~~WorkModule::on_flag(module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_END_ACTIVATE);
    //WorkModule::set_float(module_accessor, 0.0, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_THRESHOLD_ACTIVATE_POINT);
    //WorkModule::set_float(module_accessor, 100.0, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_ACTIVATE_POINT);
    //WorkModule::set_float(module_accessor, 0.0, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_DAMAGE_DIFF_MIN);
    //WorkModule::set_int(module_accessor, 1, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE);
    //WorkModule::set_int(module_accessor, 2, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_PROCESS);
    //DamageModule::add_damage(module_accessor,-100.0 ,0);
    //if damage > min(999.00,damage + 100) - 100
    //fix damage
    if WorkModule::get_int(module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE) == 1 { // use flag instead? or find a faster way to do this
                                                                                                                        // since this comes out like frame 3
        DamageModule::heal(
            module_accessor,
            -1.0 * DamageModule::damage(module_accessor, 0),
            0,
        );
        DamageModule::add_damage(module_accessor, percent, 0);
        return true;
    } else {
        DamageModule::add_damage(module_accessor, 1000.0, 0);
    }
    return false; // change to false when implemented
    //FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE
    //FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_PROCESS
    //FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_THRESHOLD_ACTIVATE_POINT
    //FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_ACTIVATE_POINT
    //FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_DAMAGE_DIFF_MIN
    //FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED
    //FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_END_ACTIVATE


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
