pub mod consts;

use crate::common::consts::*;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use smash::hash40;

pub static mut MENU_STRUCT: consts::TrainingModpackMenu = consts::TrainingModpackMenu {
    hitbox_vis: true,
    di_state: NONE,
    mash_attack_state: Attack::Nair,
    ledge_state: RANDOM_LEDGE,
    tech_state: RANDOM_TECH,
    mash_state: Mash::None,
    shield_state: NONE,
    defensive_state: RANDOM_DEFENSIVE,
};

pub static MENU: &'static mut consts::TrainingModpackMenu = unsafe { &mut MENU_STRUCT };

pub static mut FIGHTER_MANAGER_ADDR: usize = 0;

extern "C" {
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;
    
    //#[link_name = "\u{1}_ZN3app7utility8get_kindEPKNS_26BattleObjectModuleAccessorE"]
    //pub fn get_kind(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32;
}

pub fn get_category(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    return (module_accessor.info >> 28) as u8 as i32;
}

pub unsafe fn is_operation_cpu(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if get_category(module_accessor) as i32 != BATTLE_OBJECT_CATEGORY_FIGHTER {
        return false;
    }

    let entry_id_int =
        WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
    let _entry_id = app::FighterEntryID(entry_id_int);
    // let mut mgr = FighterManager{_address : fighter_manager_addr as u64};
    // let fighter_information = lua_bind::FighterManager::get_fighter_information(&mut mgr, entry_id) as *mut FighterInformation;
    // println!("FighterInformation: {:#?}", fighter_information);

    // lua_bind::FighterInformation::is_operation_cpu(fighter_information)
    entry_id_int > 0
}

pub unsafe fn is_in_hitstun(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    (*FIGHTER_STATUS_KIND_DAMAGE..=*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind)
}

pub unsafe fn is_in_shieldstun(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0) as i32;
    // If we are taking shield damage or we are droping shield from taking shield damage we are in hitstun
    if status_kind == FIGHTER_STATUS_KIND_GUARD_DAMAGE
        || (prev_status == FIGHTER_STATUS_KIND_GUARD_DAMAGE
            && status_kind == FIGHTER_STATUS_KIND_GUARD_OFF)
    {
        return true;
    }

    false
}

pub unsafe fn is_in_landing(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    (*FIGHTER_STATUS_KIND_LANDING..=*FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT).contains(&status_kind)
}

pub unsafe fn perform_defensive_option(
    _module_accessor: &mut app::BattleObjectModuleAccessor,
    flag: &mut i32,
) {
    match MENU.defensive_state {
        RANDOM_DEFENSIVE => {
            let random_cmds = vec![
                *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
            ];

            let random_cmd_index =
                app::sv_math::rand(hash40("fighter"), random_cmds.len() as i32) as usize;
            *flag |= random_cmds[random_cmd_index];
        }
        DEFENSIVE_ROLL => {
            if app::sv_math::rand(hash40("fighter"), 2) == 0 {
                *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F;
            } else {
                *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B;
            }
        }
        DEFENSIVE_SPOTDODGE => *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE,
        DEFENSIVE_JAB => *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
        _ => (),
    }
}
