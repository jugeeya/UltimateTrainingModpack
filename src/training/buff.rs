use crate::common::consts::*;
use crate::common::*;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::training::handle_add_limit;
use crate::training::frame_counter;

static mut BUFF_DELAY_COUNTER: usize = 0;

static mut BUFF_REMAINING: i32 = 0;
static mut IS_BUFFING: bool = false;

pub fn init() {
    unsafe {
        BUFF_DELAY_COUNTER = frame_counter::register_counter();
    }
}

pub unsafe fn restart_buff() {
    IS_BUFFING = false;
}

pub unsafe fn is_buffing() -> bool {
    return IS_BUFFING;
}


fn get_spell_vec() -> Vec<BuffOption> {
    unsafe {
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
    SoundModule::stop_all_sound(module_accessor); // silences buff sfx other than KO Punch
    ControlModule::stop_rumble(module_accessor, false);
    MotionAnimcmdModule::set_sleep(module_accessor, false); // does this prevent all the anims?
    //KineticModule::clear_speed_all(module_accessor);
    //CameraModule::stop_quake(module_accessor, 60); // doesn't work
    //app::sv_animcmd::QUAKE_STOP(60); crashes game very cool

    // This cannot be a match statement because of the pointer derefrencing, 
    // though you may be able to write something smarter than this like iter over a tuple of your pointer values and use find() or position()
    // unsure if the above idea has any merit though

    let menu_vec = MENU.buff_state.to_vec();

    if fighter_kind == *FIGHTER_KIND_BRAVE {
        return buff_hero(module_accessor,status);
    } else if fighter_kind == *FIGHTER_KIND_JACK && menu_vec.contains(&BuffOption::ARSENE) {
        return buff_joker(module_accessor);
    } else if fighter_kind == *FIGHTER_KIND_WIIFIT && menu_vec.contains(&BuffOption::BREATHING) {
        return buff_wiifit(module_accessor,status);
    } else if fighter_kind == *FIGHTER_KIND_CLOUD && menu_vec.contains(&BuffOption::LIMIT) {
        return buff_cloud(module_accessor);
    } else if fighter_kind == *FIGHTER_KIND_LITTLEMAC && menu_vec.contains(&BuffOption::KO) {
        return buff_mac(module_accessor);
    } else if fighter_kind == *FIGHTER_KIND_EDGE && menu_vec.contains(&BuffOption::WING) {
        return buff_sepiroth(module_accessor, percent);
    }

    return true;
}

unsafe fn buff_hero(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    let buff_vec = get_spell_vec();
    if !IS_BUFFING { // should I do 0 or 1? Initial set up for spells
        IS_BUFFING = true; // This should be fine, as starting it multiple times per frame shouldn't be an issue. Start counting
        BUFF_REMAINING = buff_vec.len() as i32; // since its the first step of buffing, we need to set up how many buffs there are
    } // else { // commands to use if we're buffing, does this else need to be here?
    if BUFF_REMAINING <= 0 { // If there are no buffs selected/left, get out of here
        return true; // this may be needed since we're casting a potential -1 to usize?
    }
    buff_hero_single(module_accessor, status, buff_vec);
    return false;
}

unsafe fn buff_hero_single(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32, buff_vec: Vec<BuffOption>) {
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
        WorkModule::set_int(module_accessor, real_spell_value, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_DECIDE_COMMAND);
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

unsafe fn buff_cloud(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if !IS_BUFFING { // only need to set limit gauge once
        IS_BUFFING = true;
        handle_add_limit(100.0,module_accessor,0);
    }
    if frame_counter::should_delay(2 as u32, BUFF_DELAY_COUNTER) { // need to wait 2 frames to make sure we stop the limit SFX, since it's a bit delayed
        return false;
    } // see if stop se stuff works for this, or if I should just stop the limit line 
    return true;
}

unsafe fn buff_joker(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if !IS_BUFFING { // only need to set rebel gauge once
        IS_BUFFING = true;
        let entry_id = app::FighterEntryID(FighterId::Player as i32); // may need to be 0? // May want to apply to CPU? For 2 framing?
        app::FighterSpecializer_Jack::add_rebel_gauge(module_accessor, entry_id, 120.0); // Why do I need to use app:: when I don't need to for other Modules?
    }

    if frame_counter::should_delay(5 as u32, BUFF_DELAY_COUNTER) { // need to wait 5 frames to make sure we stop the voice call, since it's a bit delayed
        return false;
    }
        
    return true; 
}

unsafe fn buff_mac(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if !IS_BUFFING { // only need to set rebel gauge once
        IS_BUFFING = true;
        WorkModule::set_float(module_accessor, 100.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE); // Sets meter to full
    }
    //WorkModule::on_flag(module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_REQUEST_KO_GAUGE_MAX_EFFECT); // doesn't work?
    SoundModule::stop_se(module_accessor,smash::phx::Hash40::new("se_littlemac_kogeuge_burst"), 0);
    /*if frame_counter::should_delay(10 as u32, BUFF_DELAY_COUNTER) { // need to wait 3 frames to stop the KO Punch SFX
        return false;
    }*/
    // Trying to stop KO Punch from playing seems to make it play multiple times in rapid succession. Look at 0x7100c44b60 for the func that handles this
    // Need to figure out how to update the KO meter. Probably a fighter specializer function? Maybe can just put him in hitstop though, unsure
    return true;
}

unsafe fn buff_sepiroth(module_accessor: &mut app::BattleObjectModuleAccessor, percent: f32) -> bool {
    if WorkModule::get_int(module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE) == 1 { // once we're in wing, heal to correct damage
        DamageModule::heal(
            module_accessor,
            -1.0 * DamageModule::damage(module_accessor, 0),
            0,
        );
        DamageModule::add_damage(module_accessor, percent, 0);
        return true;
    } else { // if we're not in wing, add damage
        DamageModule::add_damage(module_accessor, 1000.0, 0);
    }
    return false;
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
