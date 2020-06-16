use crate::common::consts::FighterId;
use crate::common::FIGHTER_MANAGER_ADDR;
use crate::common::*;
use crate::training::*;

pub static mut FRAME_ADVANTAGE: i32 = 0;
static mut PLAYER_ACTIONABLE: bool = true;
static mut CPU_ACTIONABLE: bool = true;
static mut PLAYER_ACTIVE_FRAME: u32 = 0;
static mut CPU_ACTIVE_FRAME: u32 = 0;
static mut FRAME_ADVANTAGE_CHECK: bool = false;
static mut SAVED_FRAME: u32 = 0;

unsafe fn get_module_accessor(fighter_id: FighterId) -> *mut app::BattleObjectModuleAccessor {
    let entry_id_int = fighter_id as i32;
    let entry_id = app::FighterEntryID(entry_id_int);
    let mgr = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
    let fighter_entry = FighterManager::get_fighter_entry(mgr, entry_id) as *mut app::FighterEntry;
    let current_fighter_id = FighterEntry::current_fighter_id(fighter_entry);
    app::sv_battle_object::module_accessor(current_fighter_id as u32)
}

unsafe fn is_actionable(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    // Can Air Dodge
    WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR,
    )
    // Can perform an Aerial
    || WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR,
    )
    // Can Shield
     || WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON,
    )
    // Can Roll/Spot Dodge
    || WorkModule::is_enable_transition_term(
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

    if !once_per_frame(module_accessor, category) {
        return;
    }

    let player_module_accessor = get_module_accessor(FighterId::Player);

    // Use to factor in that we should only update frame advantage if
    // there's been a hit that connects
    // if AttackModule::is_infliction(
    //     player_module_accessor,
    //     *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {

    // }

    // Start frame counter when the player becomes inactionable
    if PLAYER_ACTIONABLE && !is_actionable(player_module_accessor) {
        // Some strings lock the CPU while the player can move, so we need to save the advantage
        if PLAYER_ACTIVE_FRAME > 0 {
            SAVED_FRAME = frame_counter::get_frame_count();

            println!("Saving Frame {}", SAVED_FRAME);
        }

        frame_counter::reset_frame_count();
        frame_counter::start_counting();
        PLAYER_ACTIONABLE = false;
        FRAME_ADVANTAGE_CHECK = true;

        println!("Starting Counter");
    }

    // Nothing to do until we want to start checking
    if !FRAME_ADVANTAGE_CHECK {
        if !check_start_counter(player_module_accessor) {
            return;
        }
    }

    check_cpu_frames();

    // The frame the player becomes actionable again
    if !PLAYER_ACTIONABLE && is_actionable(player_module_accessor) {
        PLAYER_ACTIVE_FRAME = frame_counter::get_frame_count();
        PLAYER_ACTIONABLE = true;
        println!("Player FAF {}", PLAYER_ACTIVE_FRAME);
    }

    // When both are actionable again
    if PLAYER_ACTIONABLE && CPU_ACTIONABLE {
        calculate_frame_advantage(CPU_ACTIVE_FRAME, PLAYER_ACTIVE_FRAME);
        reset();
    }
}

/**
 * Returns true if we just started counting
 */
unsafe fn check_start_counter(
    player_module_accessor: *mut app::BattleObjectModuleAccessor,
) -> bool {
    // Nothing to do if we were inactionable from before
    if !PLAYER_ACTIONABLE {
        return false;
    }

    // Nothing to do if we are still actionable
    if is_actionable(player_module_accessor) {
        return false;
    }

    // Some strings lock the CPU while the player can move, so we need to save the advantage
    if PLAYER_ACTIVE_FRAME > 0 {
        SAVED_FRAME = frame_counter::get_frame_count();
        println!("Saving Frame {}", SAVED_FRAME);
    }

    frame_counter::reset_frame_count();
    frame_counter::start_counting();
    PLAYER_ACTIONABLE = false;
    FRAME_ADVANTAGE_CHECK = true;

    println!("Starting Counter");
    return true;
}

unsafe fn check_cpu_frames() {
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);

    if is_actionable(cpu_module_accessor) {
        // Nothing changed if we were already actionable
        if CPU_ACTIONABLE {
            return;
        }

        // Save The CPU Frame
        CPU_ACTIVE_FRAME = frame_counter::get_frame_count();
        CPU_ACTIONABLE = true;
        println!("CPU FAF {}", CPU_ACTIVE_FRAME);

        // Check combo advantage
        if SAVED_FRAME != 0 {
            calculate_frame_advantage(CPU_ACTIVE_FRAME, SAVED_FRAME);
        }
    } else {
        CPU_ACTIONABLE = false;
    }
}

// Calculate advantage and reset
unsafe fn calculate_frame_advantage(cpu_frame: u32, player_frame: u32) {
    // Don't count neutral frames
    if player_frame <= 1 {
        return;
    }

    let mut diff = (cpu_frame as i32) - (player_frame as i32);

    // Fix diff for on whiff options
    if cpu_frame <= 1{
        diff +=2;
    }

    /*
     * Convert to i32 to allow negative values
     */
    FRAME_ADVANTAGE = diff;

    println!("{} - {} = {}", cpu_frame, player_frame, FRAME_ADVANTAGE);
}

unsafe fn reset() {
    frame_counter::stop_counting();
    CPU_ACTIVE_FRAME = 0;
    PLAYER_ACTIVE_FRAME = 0;
    SAVED_FRAME = 0;
    FRAME_ADVANTAGE_CHECK = false;
    println!("Resetting");
}
