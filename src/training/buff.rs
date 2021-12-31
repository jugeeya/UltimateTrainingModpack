use crate::common::consts::*;
use smash::hash40;
//use crate::common::consts::FighterId;
use crate::common::*;
use crate::training::frame_counter;
use crate::training::mash;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

const NOT_SET: u32 = 9001;
static mut BUFF_DELAY: u32 = NOT_SET;
static mut BUFF_DELAY_COUNTER: usize = 0;
static mut BUFF_CASE: BuffOption = BuffOption::empty();
static mut BUFF_REMAINING: i32 = 0;
static mut NUM_OPERATIONS: i32 = 0;

pub fn init() { // need to init!!!!
    unsafe {
        BUFF_DELAY_COUNTER = frame_counter::register_counter();
    }
}

pub fn reset_buff_delay() {
    unsafe {
        frame_counter::full_reset(BUFF_DELAY_COUNTER);
    }
}

pub fn count_buff_delay() {
    unsafe {
        frame_counter::start_counting(BUFF_DELAY_COUNTER);
    }
}

fn _get_buff_vec() { // prob unneeded
    unsafe {
        BUFF_CASE = MENU.buff_state.get_random();
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
    // This cannot be a match statement, though you may be able to write something smarter than this like iter over a tuple of your pointer values and use find() or position()
    // unsure if the above idea has any merit though
    if fighter_kind == *FIGHTER_KIND_BRAVE {
        return buff_hero(module_accessor,status);
    } else if fighter_kind == *FIGHTER_KIND_JACK {
        return buff_joker(module_accessor,status);
    } else if fighter_kind == *FIGHTER_KIND_WIIFIT {
        return buff_wiifit(module_accessor,status);
    } else if fighter_kind == *FIGHTER_KIND_CLOUD {
        return buff_cloud(module_accessor, status);
    } else if fighter_kind == *FIGHTER_KIND_LITTLEMAC {
        return buff_mac(module_accessor, status);
    } else if fighter_kind == *FIGHTER_KIND_EDGE {
        return buff_sepiroth(module_accessor, percent);
    }

    return true;
}

// Probably should have some vector of the statuses selected on the Menu, and for each status you
// have the framecounter delay be its index (0 for first, 1 frame/index for second, etc.), this probably goes backwards? Unsure
// probably better to just always call the first if its not empty, but won't cause idk how

unsafe fn buff_hero(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    // does frame_counter actually prevent crashes at all? Or does it never tick because of the infinite loop?
    // probably should implement another static var as a failsafe
    if NUM_OPERATIONS > 30 {
        println!("31 Ops reached");
        return true;
    }
    
    let buff_vec = vec![BuffOption::OOMPH,BuffOption::PSYCHE,BuffOption::BOUNCE,BuffOption::ACCELERATLE];
    //let buff_vec = vec![BuffOption::OOMPH,BuffOption::ACCELERATLE];

    //let mut buff_vec = MENU.buff_state.to_vec().intersect(spell_vec); // vector of currently selected hero buffs
    
    let current_frame = frame_counter::get_frame_count(BUFF_DELAY_COUNTER) as i32; // do I want to be grabbing this like that?
    
    println!("Frame: {}, Operations: {}",current_frame,NUM_OPERATIONS);
    
    if current_frame == 0 { // should I do 0 or 1? Initial set up for spells
        count_buff_delay(); // This should be fine, as starting it multiple times per frame shouldn't be an issue. Start counting
        NUM_OPERATIONS = 0;
        BUFF_REMAINING = buff_vec.len() as i32; // since its the first frame, we need to set up how many buffs there are
    } // else { // commands to use if we're buffing, does this else need to be here?
    if BUFF_REMAINING <= 0 { // If there are no buffs selected/left, get out of here
        return true; // this may be needed since we're casting a potential -1 to usize?
    }
    let spell_index = BUFF_REMAINING - 1; // as usize here? var used to get spell from our vector
    let spell_option = buff_vec.get(spell_index as usize);
    if spell_option.is_none() { // there are no spells selected, or something went wrong with making the vector
        return true;
    }
    let spell_value = spell_option.unwrap().into_int().unwrap(); // this just seems extremely wrong
    buff_hero_single(module_accessor, status, buff_vec);
    println!("Buff Hero Frame: {}, PostStatus: {}, Spell: {}",current_frame,status,spell_value);
    //}
    return false;
}

unsafe fn buff_hero_single(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32, buff_vec: Vec<BuffOption>) {
    NUM_OPERATIONS += 1;
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    if prev_status_kind == FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START { //&& buffs_remaining = 0 // If finished applying buffs, need to have some kind of struct responsible
        BUFF_REMAINING -= 1;
    }
    
    // need to handle finding the buff in here due to the above if statement
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
        if (BUFF_REMAINING > 1) { // not last buff
            MotionModule::set_rate(module_accessor, 50.0); //only needs to be 40 I think ??? 46 for psyche up?
        } else {
            MotionModule::set_rate(module_accessor, 50.0);
        } // works decently for 2 spells, but should honestly just forget frame counters and move on I think
        
        // try delaying for 2 frames before next spell here?
        // RESET, but don't start counting
    }
}

unsafe fn buff_cloud(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    // forcing status module crashes the game
    /*StatusModule::change_status_force( // _request_from_script? - no, because you need to override shield buffer
        module_accessor,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END,
        true, // originally false, probably should be true though so inputs aren't interfered with as we go through multiple buffs
    );*/
    WorkModule::set_float(module_accessor, 100.0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
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
