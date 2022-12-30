pub mod button_config;
pub mod consts;
pub mod events;
pub mod menu;
pub mod raygun_printer;
pub mod release;

use crate::common::consts::*;
use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;

pub use crate::common::consts::MENU;
pub static mut DEFAULTS_MENU: TrainingModpackMenu = crate::common::consts::DEFAULTS_MENU;
pub static mut BASE_MENU: TrainingModpackMenu = unsafe { DEFAULTS_MENU };
pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub static mut ITEM_MANAGER_ADDR: usize = 0;
pub static mut STAGE_MANAGER_ADDR: usize = 0;

#[cfg(not(feature = "outside_training_mode"))]
extern "C" {
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;
}

#[cfg(feature = "outside_training_mode")]
pub fn is_training_mode() -> bool {
    true
}

pub fn get_category(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    (module_accessor.info >> 28) as u8 as i32
}

pub fn is_emulator() -> bool {
    unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 == 0x8004000 }
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
            WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

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
    let situation_kind = unsafe { StatusModule::situation_kind(module_accessor) };

    situation_kind == SITUATION_KIND_GROUND
}

pub fn is_airborne(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind = unsafe { StatusModule::situation_kind(module_accessor) };

    situation_kind == SITUATION_KIND_AIR
}

pub fn is_idle(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = unsafe { StatusModule::status_kind(module_accessor) };

    status_kind == FIGHTER_STATUS_KIND_WAIT
}

pub fn is_in_hitstun(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = unsafe { StatusModule::status_kind(module_accessor) };
    // TODO: Should this be *FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_AIR ?
    (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind)
}
pub fn is_in_footstool(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = unsafe { StatusModule::status_kind(module_accessor) };

    (*FIGHTER_STATUS_KIND_TREAD_DAMAGE..=*FIGHTER_STATUS_KIND_TREAD_FALL).contains(&status_kind)
}

pub fn is_shielding(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = unsafe { StatusModule::status_kind(module_accessor) };

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

pub unsafe fn is_ptrainer(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    [
        *FIGHTER_KIND_PZENIGAME,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_PLIZARDON,
    ]
    .contains(&app::utility::get_kind(module_accessor))
}

pub unsafe fn is_dead(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor);
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    // Pokemon trainer enters FIGHTER_STATUS_KIND_WAIT for one frame during their respawn animation
    // And the previous status is FIGHTER_STATUS_NONE
    if is_ptrainer(module_accessor) {
        [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY].contains(&status_kind)
            || (status_kind == FIGHTER_STATUS_KIND_WAIT
                && prev_status_kind == FIGHTER_STATUS_KIND_NONE)
    } else {
        [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY].contains(&status_kind)
    }
}

pub unsafe fn is_in_clatter(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    [
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_START,
        *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY,
        *FIGHTER_STATUS_KIND_BURY,
        *FIGHTER_STATUS_KIND_BURY_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_YOSHI,
        *FIGHTER_STATUS_KIND_YOSHI_EGG,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI,
        *FIGHTER_STATUS_KIND_SWALLOWED,
        *FIGHTER_STATUS_KIND_SWALLOWED_CAPTURE,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON,
        *FIGHTER_STATUS_KIND_BIND,        // Mewtwo disable
        *FIGHTER_STATUS_KIND_DAMAGE_SONG, // Jigglypuff sing
        // FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL, // Jigglypuff sing. Not sure when this is called?
        *FIGHTER_STATUS_KIND_BITTEN_WARIO,
        *FIGHTER_STATUS_KIND_CLUNG_DIDDY,
        *FIGHTER_STATUS_KIND_CLUNG_DAMAGE_DIDDY,
        *FIGHTER_STATUS_KIND_ICE,
        *FIGHTER_STATUS_KIND_CATCHED_REFLET,        // Nosferatu
        *FIGHTER_STATUS_KIND_KAMUI_PIERCE,          // Corrin pin
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED, // Incin sideb
        *FIGHTER_STATUS_KIND_DRAGGED_RIDLEY,
        *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY, // Steve minecart
        *FIGHTER_STATUS_KIND_SLEEP,
        *FIGHTER_STATUS_KIND_FURAFURA,
        *FIGHTER_STATUS_KIND_GIMMICK_FISH_CAPTURE,
        *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND,
        *FIGHTER_STATUS_KIND_CAPTURE_ITEM,
        *FIGHTER_STATUS_KIND_CAPTURE_BEETLE,
        *FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE,
        *FIGHTER_STATUS_KIND_CAPTURE_BEITCRANE,
        *FIGHTER_STATUS_KIND_CAPTURE_KAWASAKI,
        *FIGHTER_STATUS_KIND_CAPTURE_DRIVER,
        *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU,
        *FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP,
        *FIGHTER_STATUS_KIND_CAPTURE_BOSSGALAGA,
        *FIGHTER_STATUS_KIND_CAPTURE_NABBIT,
        *FIGHTER_STATUS_KIND_CAPTURE_MASTERCORE,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS,
    ]
    .contains(&StatusModule::status_kind(module_accessor))
}

pub unsafe fn is_in_ledgetrump(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor);

    status_kind == FIGHTER_STATUS_KIND_CLIFF_ROBBED
}

pub unsafe fn is_in_parry(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let motion_kind = MotionModule::motion_kind(module_accessor);

    motion_kind == hash40("just_shield_off")
}

pub unsafe fn is_in_tumble(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor);

    (*FIGHTER_STATUS_KIND_DAMAGE_FLY..=*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind)
}

pub unsafe fn is_in_landing(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor);

    (*FIGHTER_STATUS_KIND_LANDING..=*FIGHTER_STATUS_KIND_LANDING_LIGHT).contains(&status_kind)
}

// Returns true if a match is currently active
pub unsafe fn is_ready_go() -> bool {
    let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
    FighterManager::is_ready_go(fighter_manager)
}

// Returns true if a match is currently active
pub unsafe fn entry_count() -> i32 {
    let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
    FighterManager::entry_count(fighter_manager)
}

pub unsafe fn get_player_dmg_digits(p: FighterId) -> (u8, u8, u8, u8) {
    let module_accessor = get_module_accessor(p);
    let dmg = DamageModule::damage(module_accessor, 0);
    let hundreds = dmg as u16 / 100;
    let tens = (dmg as u16 - hundreds * 100) / 10;
    let ones = (dmg as u16) - (hundreds * 100) - (tens * 10);
    let dec = ((dmg * 10.0) as u16) - (hundreds * 1000) - (tens * 100) - ones * 10;
    (hundreds as u8, tens as u8, ones as u8, dec as u8)
}

pub unsafe fn get_fighter_distance() -> f32 {
    let player_module_accessor = get_module_accessor(FighterId::Player);
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    let player_pos = *PostureModule::pos(player_module_accessor);
    let cpu_pos = *PostureModule::pos(cpu_module_accessor);
    app::sv_math::vec3_distance(
        player_pos.x,
        player_pos.y,
        player_pos.z,
        cpu_pos.x,
        cpu_pos.y,
        cpu_pos.z,
    )
}

// From https://github.com/chrispo-git/ult-s/blob/cc1c3060ed83f6d33f39964e84f9c32c07a17bae/src/controls/util.rs#L106
pub unsafe fn get_fighter_common_from_accessor(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> &mut L2CFighterCommon {
    let lua_module =
        *(module_accessor as *mut app::BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
    &mut *(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
}
