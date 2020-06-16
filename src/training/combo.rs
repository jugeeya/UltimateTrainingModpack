use crate::common::consts::FighterId;
use crate::common::FIGHTER_MANAGER_ADDR;
use crate::common::*;
use crate::training::*;

pub static mut FRAME_ADVANTAGE: i32 = 0;
static mut FRAME_COUNTER: u64 = 0;
static mut PLAYER_ACTIONABLE: bool = false;
static mut CPU_ACTIONABLE: bool = false;
static mut PLAYER_ACTIVE_FRAME: u64 = 0;
static mut CPU_ACTIVE_FRAME: u64 = 0;
static mut FRAME_ADVANTAGE_CHECK: bool = false;

unsafe fn was_in_hitstun(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    (*FIGHTER_STATUS_KIND_DAMAGE..=*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&prev_status)
}

unsafe fn was_in_shieldstun(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    prev_status == FIGHTER_STATUS_KIND_GUARD_DAMAGE
}

unsafe fn get_module_accessor(fighter_id: FighterId) -> *mut app::BattleObjectModuleAccessor {
    let entry_id_int = fighter_id as i32;
    let entry_id = app::FighterEntryID(entry_id_int);
    let mgr = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
    let fighter_entry = FighterManager::get_fighter_entry(mgr, entry_id) as *mut app::FighterEntry;
    let current_fighter_id = FighterEntry::current_fighter_id(fighter_entry);
    app::sv_battle_object::module_accessor(current_fighter_id as u32)
}

unsafe fn is_actionable(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR,
    ) || WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR,
    ) || WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON,
    ) || WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
    ) || CancelModule::is_enable_cancel(module_accessor)
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) {
    if !is_training_mode() {
        return;
    }

    if category != 0 {
        return;
    }

    let entry_id_int =
        WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
    // do only once.
    if entry_id_int != (FighterId::Player as i32) {
        return;
    }

    let player_module_accessor = get_module_accessor(FighterId::Player);
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);

    // Use to factor in that we should only update frame advantage if
    // there's been a hit that connects
    // if AttackModule::is_infliction(
    //     player_module_accessor,
    //     *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {

    // }

    // the frame the fighter *becomes* actionable
    if !CPU_ACTIONABLE && is_actionable(cpu_module_accessor) {
        CPU_ACTIVE_FRAME = FRAME_COUNTER;
    }

    if !PLAYER_ACTIONABLE && is_actionable(player_module_accessor) {
        PLAYER_ACTIVE_FRAME = FRAME_COUNTER;
    }

    CPU_ACTIONABLE = is_actionable(cpu_module_accessor);
    PLAYER_ACTIONABLE = is_actionable(player_module_accessor);

    // if neither are active
    if !CPU_ACTIONABLE && !PLAYER_ACTIONABLE {
        FRAME_ADVANTAGE_CHECK = true;
    }

    // if both are now active
    if PLAYER_ACTIONABLE && CPU_ACTIONABLE {
        if FRAME_ADVANTAGE_CHECK {
            if was_in_hitstun(cpu_module_accessor) || was_in_shieldstun(cpu_module_accessor) {
                let frame_advantage: i64;
                if PLAYER_ACTIVE_FRAME > CPU_ACTIVE_FRAME {
                    frame_advantage = (PLAYER_ACTIVE_FRAME - CPU_ACTIVE_FRAME) as i64 * -1;
                } else {
                    frame_advantage = (CPU_ACTIVE_FRAME - PLAYER_ACTIVE_FRAME) as i64;
                }

                FRAME_ADVANTAGE = frame_advantage as i32;
            }

            FRAME_ADVANTAGE_CHECK = false;
        }
    }

    FRAME_COUNTER += 1;
}
