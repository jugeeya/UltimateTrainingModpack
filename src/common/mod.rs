pub mod consts;
pub mod events;
pub mod menu;
pub mod raygun_printer;
pub mod release;

use crate::common::consts::*;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

pub static BASE_MENU: consts::TrainingModpackMenu = consts::TrainingModpackMenu {
    // Mash Options
    mash_state: Action::empty(),
    follow_up: Action::empty(),
    attack_angle: AttackAngle::empty(),
    throw_state: ThrowOption::NONE,
    throw_delay: MedDelay::empty(),
    pummel_delay: MedDelay::empty(),
    full_hop: BoolFlag::empty(),
    falling_aerials: BoolFlag::empty(),
    aerial_delay: Delay::empty(),
    fast_fall: BoolFlag::empty(),
    fast_fall_delay: Delay::empty(),
    oos_offset: Delay::empty(),
    mash_in_neutral: OnOff::Off,
    reaction_time: Delay::empty(),
    // Defensive Options
    air_dodge_dir: Direction::empty(),
    di_state: Direction::empty(),
    sdi_state: Direction::empty(),
    sdi_strength: SdiStrength::Normal,
    ledge_state: LedgeOption::all(),
    ledge_delay: LongDelay::empty(),
    tech_state: TechFlags::all(),
    miss_tech_state: MissTechFlags::all(),
    shield_state: Shield::None,
    defensive_state: Defensive::all(),
    shield_tilt: Direction::empty(),
    buff_state: BuffOption::empty(),
    // Miscellaneous Options
    save_state_mirroring: SaveStateMirroring::None,
    save_damage: OnOff::On,
    save_state_enable: OnOff::On,
    frame_advantage: OnOff::Off,
    hitbox_vis: OnOff::On,
    input_delay: 0,
    stage_hazards: OnOff::Off,
};

pub static mut DEFAULT_MENU: TrainingModpackMenu = BASE_MENU;
pub static mut MENU: TrainingModpackMenu = BASE_MENU;
pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub static mut STAGE_MANAGER_ADDR: usize = 0;

#[cfg(not(feature = "outside_training_mode"))]
extern "C" {
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;
}

#[cfg(feature = "outside_training_mode")]
pub fn is_training_mode() -> bool {
    return true;
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

        if entry_id_int == 0 {
            return false;
        }

        let entry_id = app::FighterEntryID(entry_id_int);
        let mgr = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
        let fighter_information =
            FighterManager::get_fighter_information(mgr, entry_id) as *mut app::FighterInformation;

        FighterInformation::is_operation_cpu(fighter_information)
    }
}

pub fn is_grounded(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind = unsafe { StatusModule::situation_kind(module_accessor) as i32 };

    situation_kind == SITUATION_KIND_GROUND
}

pub fn is_airborne(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind = unsafe { StatusModule::situation_kind(module_accessor) as i32 };

    situation_kind == SITUATION_KIND_AIR
}

pub fn is_idle(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = unsafe { StatusModule::status_kind(module_accessor) };

    status_kind == FIGHTER_STATUS_KIND_WAIT
}

pub fn is_in_hitstun(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = unsafe { StatusModule::status_kind(module_accessor) };

    (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind)
}
pub fn is_in_footstool(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = unsafe { StatusModule::status_kind(module_accessor) };

    (*FIGHTER_STATUS_KIND_TREAD_DAMAGE..=*FIGHTER_STATUS_KIND_TREAD_FALL).contains(&status_kind)
}

pub fn is_shielding(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = unsafe { StatusModule::status_kind(module_accessor) as i32 };

    (*FIGHTER_STATUS_KIND_GUARD_ON..=*FIGHTER_STATUS_KIND_GUARD_DAMAGE).contains(&status_kind)
}

pub fn is_in_shieldstun(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = unsafe { StatusModule::status_kind(module_accessor) };
    let prev_status = unsafe { StatusModule::prev_status_kind(module_accessor, 0) };

    // If we are taking shield damage or we are droping shield from taking shield damage we are in hitstun
    status_kind == FIGHTER_STATUS_KIND_GUARD_DAMAGE
        || (prev_status == FIGHTER_STATUS_KIND_GUARD_DAMAGE
            && status_kind == FIGHTER_STATUS_KIND_GUARD_OFF)
}

pub unsafe fn is_dead(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let fighter_kind = app::utility::get_kind(module_accessor);
    let fighter_is_ptrainer = [
        *FIGHTER_KIND_PZENIGAME,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_PLIZARDON,
    ]
    .contains(&fighter_kind);
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    // Pokemon trainer enters FIGHTER_STATUS_KIND_WAIT for one frame during their respawn animation
    // And the previous status is FIGHTER_STATUS_NONE
    if fighter_is_ptrainer {
        [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY].contains(&status_kind)
            || (status_kind == FIGHTER_STATUS_KIND_WAIT
                && prev_status_kind == FIGHTER_STATUS_KIND_NONE)
    } else {
        [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY].contains(&status_kind)
    }
}
