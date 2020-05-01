pub mod consts;

use smash::lib::lua_const::{*};
use crate::common::consts::*;
use smash::app::{self};
use smash::app::lua_bind::{self, *};
use smash::app::{FighterManager, FighterInformation};
use smash::hash40;

pub static menu : consts::TrainingModpackMenu = consts::TrainingModpackMenu{
    HITBOX_VIS : true,
    DI_STATE : NONE,
    ATTACK_STATE : MASH_NAIR,
    LEDGE_STATE : RANDOM_LEDGE,
    TECH_STATE : RANDOM_TECH,
    MASH_STATE : NONE,
    SHIELD_STATE : NONE,
    DEFENSIVE_STATE : RANDOM_DEFENSIVE,
};

static mut fighter_manager: FighterManager = FighterManager{ _address: 0 };

extern "C" {
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;
}

pub fn get_category(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
	return (module_accessor.info >> 28) as u8 as i32;
}

pub unsafe fn is_operation_cpu(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if get_category(module_accessor) as i32 != BATTLE_OBJECT_CATEGORY_FIGHTER {
        return false
    }

    let entry_id = app::FighterEntryID(WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32);
    let fighter_information = lua_bind::FighterManager::get_fighter_information(&mut fighter_manager, entry_id) as *mut FighterInformation;

    lua_bind::FighterInformation::is_operation_cpu(fighter_information)
}

pub unsafe fn is_in_hitstun(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    return status_kind >= FIGHTER_STATUS_KIND_DAMAGE &&
           status_kind <= FIGHTER_STATUS_KIND_DAMAGE_FALL;
}

pub unsafe fn is_in_shieldstun(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0) as i32;
    // If we are taking shield damage or we are droping shield from taking shield damage we are in hitstun
    if status_kind == FIGHTER_STATUS_KIND_GUARD_DAMAGE || 
        (prev_status == FIGHTER_STATUS_KIND_GUARD_DAMAGE && status_kind == FIGHTER_STATUS_KIND_GUARD_OFF) {
        return true
    }

    false
}


pub unsafe fn is_in_landing(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    (FIGHTER_STATUS_KIND_LANDING..FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT)
        .contains(&status_kind)
}


pub unsafe fn perform_defensive_option(module_accessor: &mut app::BattleObjectModuleAccessor, flag: &mut i32) {
    if menu.DEFENSIVE_STATE == RANDOM_DEFENSIVE {
        let NUM_DEFENSIVE_CMDS = 4;
        let random_cmds = vec![
            *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N
        ];

        let random_cmd_index = app::sv_math::rand(hash40("fighter"), random_cmds.len() as i32) as usize;
        *flag |= random_cmds[random_cmd_index];
    } else if menu.DEFENSIVE_STATE == DEFENSIVE_ROLL {
        if app::sv_math::rand(hash40("fighter"), 2) == 0 {
            *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F;
        } else {
            *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B;
        }
    } else if menu.DEFENSIVE_STATE == DEFENSIVE_SPOTDODGE {
        *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE;
    } else if menu.DEFENSIVE_STATE == DEFENSIVE_JAB {
        *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N;
    }
}