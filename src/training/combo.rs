use crate::common::consts::FighterId;
use crate::common::*;
use crate::training::*;

pub static mut FRAME_ADVANTAGE: i32 = 0;
static mut PLAYER_ACTIONABLE: bool = false;
static mut CPU_ACTIONABLE: bool = false;
static mut PLAYER_ACTIVE_FRAME: u32 = 0;
static mut CPU_ACTIVE_FRAME: u32 = 0;
static mut FRAME_ADVANTAGE_CHECK: bool = false;

lazy_static::lazy_static! {
    static ref FRAME_ADVANTAGE_COUNTER: frame_counter::FrameCounter = frame_counter::FrameCounter::new();
}

unsafe fn was_in_hitstun(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&prev_status)
}

unsafe fn was_in_shieldstun(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    prev_status == FIGHTER_STATUS_KIND_GUARD_DAMAGE
}

macro_rules! actionable_statuses {
    () => {
            vec![
                FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR,
                FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR,
                FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON,
                FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE
            ];
    };
}

unsafe fn is_actionable(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    actionable_statuses!().iter().any(
        |actionable_transition|
        WorkModule::is_enable_transition_term(module_accessor, **actionable_transition))
    || CancelModule::is_enable_cancel(module_accessor)
}

fn update_frame_advantage(module_accessor: *mut app::BattleObjectModuleAccessor, new_frame_adv: i32) {
    unsafe {
        FRAME_ADVANTAGE = new_frame_adv;
        if MENU.frame_advantage == consts::OnOff::On {
            raygun_printer::print_string(&mut *module_accessor, &format!("{}", FRAME_ADVANTAGE));
        }
    }
}

pub unsafe fn is_enable_transition_term(
    module_accessor: *mut app::BattleObjectModuleAccessor,
    transition_term: i32,
    is: bool
) {
    let entry_id_int =
        WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;

    if entry_id_int != (FighterId::Player as i32) {
        return;
    }

    // Extra check later in the frame.
    // This is in the case that the transition term becomes enabled after our initial check
    // and the user buffers that action on that frame.

    if !PLAYER_ACTIONABLE &&
        (
            (is && actionable_statuses!().iter().any(|actionable_transition| *actionable_transition == transition_term))
            ||
            (CancelModule::is_enable_cancel(module_accessor))
        ) {
        PLAYER_ACTIVE_FRAME = FRAME_ADVANTAGE_COUNTER.get_frame_count();
        PLAYER_ACTIONABLE = true;

        // if both are now active
        if PLAYER_ACTIONABLE && CPU_ACTIONABLE && FRAME_ADVANTAGE_CHECK {
            let cpu_module_accessor = get_module_accessor(FighterId::CPU);
            if was_in_hitstun(cpu_module_accessor) || was_in_shieldstun(cpu_module_accessor) {
                update_frame_advantage(module_accessor,
                    (CPU_ACTIVE_FRAME as i64 - PLAYER_ACTIVE_FRAME as i64) as i32);
            }

            FRAME_ADVANTAGE_COUNTER.stop_counting();
            FRAME_ADVANTAGE_CHECK = false;
        }
    }
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) {
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

    // the frame the fighter *becomes* actionable
    if !CPU_ACTIONABLE && is_actionable(cpu_module_accessor) {
        CPU_ACTIVE_FRAME = FRAME_ADVANTAGE_COUNTER.get_frame_count();
    }

    if !PLAYER_ACTIONABLE && is_actionable(player_module_accessor) {
        PLAYER_ACTIVE_FRAME = FRAME_ADVANTAGE_COUNTER.get_frame_count();
    }

    CPU_ACTIONABLE = is_actionable(cpu_module_accessor);
    PLAYER_ACTIONABLE = is_actionable(player_module_accessor);

    // if neither are active
    if !CPU_ACTIONABLE && !PLAYER_ACTIONABLE {
        if !FRAME_ADVANTAGE_CHECK {
            FRAME_ADVANTAGE_COUNTER.reset_frame_count();
            FRAME_ADVANTAGE_COUNTER.start_counting();
        }
        FRAME_ADVANTAGE_CHECK = true;
    }

    // if both are now active
    if PLAYER_ACTIONABLE && CPU_ACTIONABLE && FRAME_ADVANTAGE_CHECK {
        if was_in_hitstun(cpu_module_accessor) || was_in_shieldstun(cpu_module_accessor) {
            update_frame_advantage(player_module_accessor, 
                (CPU_ACTIVE_FRAME as i64 - PLAYER_ACTIVE_FRAME as i64) as i32);
        }

        FRAME_ADVANTAGE_COUNTER.stop_counting();
        FRAME_ADVANTAGE_CHECK = false;
    }
}
