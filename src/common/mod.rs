pub mod consts;

use crate::common::consts::*;
use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;

pub static mut MENU_STRUCT: consts::TrainingModpackMenu = consts::TrainingModpackMenu {
    hitbox_vis: OnOff::On,
    stage_hazards: OnOff::Off,
    di_state: Direction::empty(),
    sdi_state: Direction::empty(),
    sdi_strength: SdiStrength::Normal,
    air_dodge_dir: Direction::empty(),
    mash_state: Action::empty(),
    follow_up: Action::empty(),
    attack_angle: AttackAngle::empty(),
    ledge_state: LedgeOption::all(),
    ledge_delay: Delay::empty(),
    tech_state: TechFlags::all(),
    miss_tech_state: MissTechFlags::all(),
    shield_state: Shield::None,
    player_shield: Shield::None,
    defensive_state: Defensive::all(),
    oos_offset: Delay::empty(),
    shield_tilt: Direction::empty(),
    reaction_time: Delay::empty(),
    mash_in_neutral: OnOff::Off,
    fast_fall: BoolFlag::empty(),
    fast_fall_delay: Delay::empty(),
    falling_aerials: BoolFlag::empty(),
    aerial_delay: Delay::empty(),
    full_hop: BoolFlag::empty(),
    input_delay: 0,
    save_damage: OnOff::On,
};

pub static mut MENU: &consts::TrainingModpackMenu = unsafe { &mut MENU_STRUCT };

pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub static mut STAGE_MANAGER_ADDR: usize = 0;

extern "C" {
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;

//#[link_name = "\u{1}_ZN3app7utility8get_kindEPKNS_26BattleObjectModuleAccessorE"]
//pub fn get_kind(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32;
}

pub fn get_category(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    (module_accessor.info >> 28) as u8 as i32
}

pub fn get_module_accessor(fighter_id: FighterId) -> *mut app::BattleObjectModuleAccessor {
    let entry_id_int = fighter_id as i32;
    let entry_id = app::FighterEntryID(entry_id_int);
    unsafe {
        let mgr = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
        let fighter_entry =
            FighterManager::get_fighter_entry(mgr, entry_id) as *mut app::FighterEntry;
        let current_fighter_id = FighterEntry::current_fighter_id(fighter_entry);
        app::sv_battle_object::module_accessor(current_fighter_id as u32)
    }
}

pub fn is_fighter(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    get_category(module_accessor) == BATTLE_OBJECT_CATEGORY_FIGHTER
}

pub fn is_operation_cpu(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        if !is_fighter(module_accessor) {
            return false;
        }

        let entry_id_int =
            WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
        let entry_id = app::FighterEntryID(entry_id_int);
        let mgr = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
        let fighter_information =
            FighterManager::get_fighter_information(mgr, entry_id) as *mut app::FighterInformation;

        FighterInformation::is_operation_cpu(fighter_information)
    }
}

pub fn is_grounded(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind;
    unsafe {
        situation_kind = StatusModule::situation_kind(module_accessor) as i32;
    }
    situation_kind == SITUATION_KIND_GROUND
}

pub fn is_airborne(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind;
    unsafe {
        situation_kind = StatusModule::situation_kind(module_accessor) as i32;
    }
    situation_kind == SITUATION_KIND_AIR
}

pub fn is_idle(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind;
    unsafe {
        status_kind = StatusModule::status_kind(module_accessor);
    }
    status_kind == FIGHTER_STATUS_KIND_WAIT
}

pub fn is_in_hitstun(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind;
    unsafe {
        status_kind = StatusModule::status_kind(module_accessor);
    }
    (*FIGHTER_STATUS_KIND_DAMAGE..=*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind)
}
pub fn is_in_footstool(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind;
    unsafe {
        status_kind = StatusModule::status_kind(module_accessor);
    }
    (*FIGHTER_STATUS_KIND_TREAD_DAMAGE..=*FIGHTER_STATUS_KIND_TREAD_FALL).contains(&status_kind)
}

pub fn is_shielding(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        let status_kind = StatusModule::status_kind(module_accessor) as i32;
        (*FIGHTER_STATUS_KIND_GUARD_ON..=*FIGHTER_STATUS_KIND_GUARD_DAMAGE).contains(&status_kind)
    }
}

pub fn is_in_shieldstun(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind;
    let prev_status;

    unsafe {
        status_kind = StatusModule::status_kind(module_accessor);
        prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    }

    // If we are taking shield damage or we are droping shield from taking shield damage we are in hitstun
    status_kind == FIGHTER_STATUS_KIND_GUARD_DAMAGE
        || (prev_status == FIGHTER_STATUS_KIND_GUARD_DAMAGE
            && status_kind == FIGHTER_STATUS_KIND_GUARD_OFF)
}

pub fn get_random_int(max: i32) -> i32 {
    unsafe { app::sv_math::rand(hash40("fighter"), max) }
}
