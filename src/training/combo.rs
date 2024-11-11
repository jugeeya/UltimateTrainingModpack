use skyline::nn::ui2d::ResColor;
use training_mod_consts::OnOff;

use crate::common::*;
use crate::consts::Action;
use training_mod_sync::*;
use crate::training::ui::notifications;
use crate::training::*;

static PLAYER_WAS_ACTIONABLE: RwLock<bool> = RwLock::new(false);
static CPU_WAS_ACTIONABLE: RwLock<bool> = RwLock::new(false);

static PLAYER_FRAME_COUNTER_INDEX: LazyLock<usize> =
    LazyLock::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGame));
static CPU_FRAME_COUNTER_INDEX: LazyLock<usize> =
    LazyLock::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGame));

unsafe fn was_in_hitstun(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&prev_status)
}

unsafe fn is_in_hitstun(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL)
        .contains(&StatusModule::status_kind(module_accessor))
}

unsafe fn was_in_shieldstun(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    prev_status == FIGHTER_STATUS_KIND_GUARD_DAMAGE
}

unsafe fn is_in_shieldstun(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    StatusModule::status_kind(module_accessor) == FIGHTER_STATUS_KIND_GUARD_DAMAGE
}

unsafe fn is_actionable(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    [
        FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR, // Airdodge
        FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR, // Aerial
        FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON,   // Shield
        FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,     // Spotdodge/Roll
        FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN_STAND,      // Neutral Getup from Tech/Slip
    ]
    .iter()
    .any(|actionable_transition| {
        WorkModule::is_enable_transition_term(module_accessor, **actionable_transition)
    }) || CancelModule::is_enable_cancel(module_accessor)
}

fn update_frame_advantage(frame_advantage: i32) {
    unsafe {
        if MENU.frame_advantage == OnOff::ON {
            // Prioritize Frame Advantage over Input Recording Playback
            notifications::clear_notifications_except("Input Recording");
            notifications::clear_notifications_except("Frame Advantage");
            notifications::color_notification(
                "Frame Advantage".to_string(),
                format!("{frame_advantage}"),
                60,
                match frame_advantage {
                    x if x < 0 => ResColor {
                        r: 200,
                        g: 8,
                        b: 8,
                        a: 255,
                    },
                    0 => ResColor {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 255,
                    },
                    _ => ResColor {
                        r: 31,
                        g: 198,
                        b: 0,
                        a: 255,
                    },
                },
            );
        }
    }
}

pub unsafe fn once_per_frame(module_accessor: &mut BattleObjectModuleAccessor) {
    // Skip the CPU so we don't run twice per frame
    let entry_id_int = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    if entry_id_int != (FighterId::Player as i32) {
        return;
    }
    let player_module_accessor = get_module_accessor(FighterId::Player);
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    let player_is_actionable = is_actionable(player_module_accessor);
    let player_was_actionable = read_rwlock(&PLAYER_WAS_ACTIONABLE);
    let player_just_actionable = !player_was_actionable && player_is_actionable;
    let cpu_is_actionable = is_actionable(cpu_module_accessor);
    let cpu_was_actionable = read_rwlock(&CPU_WAS_ACTIONABLE);
    let cpu_just_actionable = !cpu_was_actionable && cpu_is_actionable;
    let is_counting = frame_counter::is_counting(*PLAYER_FRAME_COUNTER_INDEX)
        || frame_counter::is_counting(*CPU_FRAME_COUNTER_INDEX);

    if !is_counting {
        if MENU.mash_state == Action::empty()
            && !player_is_actionable
            && !cpu_is_actionable
            && (!was_in_shieldstun(cpu_module_accessor) && is_in_shieldstun(cpu_module_accessor)
                || (!was_in_hitstun(cpu_module_accessor) && is_in_hitstun(cpu_module_accessor)))
        {
            // Start counting when:
            // 1. We have no mash option selected AND
            // 2. Neither fighter is currently actionable AND
            // 3. Either
            //  a.     the CPU has just entered shieldstun
            //  b.     the CPU has just entered hitstun
            //
            // If a mash option is selected, this can interfere with our ability to determine when
            // a character becomes actionable. So don't ever start counting if we can't reliably stop.
            //
            // Since our "just_actionable" checks assume that neither character is already actionable,
            // we need to guard against instances where the player is already actionable by the time that
            // the CPU get hit, such as if the player threw a projectile from far away.
            // Otherwise our "just_actionable" checks are not valid.
            //
            // We also need to guard against instances where the CPU's status is in hitstun but they are actually actionable.
            // I dunno, makes no sense to me either. Can trigger this edge case with PAC-MAN jab 1 against Lucas at 0%.
            // This shows up as the count restarting immediately after the last one ended.
            frame_counter::reset_frame_count(*PLAYER_FRAME_COUNTER_INDEX);
            frame_counter::reset_frame_count(*CPU_FRAME_COUNTER_INDEX);
            frame_counter::start_counting(*PLAYER_FRAME_COUNTER_INDEX);
            frame_counter::start_counting(*CPU_FRAME_COUNTER_INDEX);
        }
    } else {
        // Uncomment this if you want some frame logging
        // if (player_is_actionable && cpu_is_actionable) {
        //     info!("!");
        // } else if (!player_is_actionable && cpu_is_actionable) {
        //     info!("-");
        // } else if (player_is_actionable && !cpu_is_actionable) {
        //     info!("+");
        // } else {
        //     info!(".");
        // }

        // Stop counting as soon as each fighter becomes actionable
        if player_just_actionable {
            frame_counter::stop_counting(*PLAYER_FRAME_COUNTER_INDEX);
        }

        if cpu_just_actionable {
            frame_counter::stop_counting(*CPU_FRAME_COUNTER_INDEX);
        }

        // If we just finished counting for the second fighter, then display frame advantage
        if !frame_counter::is_counting(*PLAYER_FRAME_COUNTER_INDEX)
            && !frame_counter::is_counting(*CPU_FRAME_COUNTER_INDEX)
            && (player_just_actionable || cpu_just_actionable)
        {
            update_frame_advantage(
                frame_counter::get_frame_count(*CPU_FRAME_COUNTER_INDEX) as i32
                    - frame_counter::get_frame_count(*PLAYER_FRAME_COUNTER_INDEX) as i32,
            );
            // Frame counters should reset before we start again, but reset them just to be safe
            frame_counter::reset_frame_count(*PLAYER_FRAME_COUNTER_INDEX);
            frame_counter::reset_frame_count(*CPU_FRAME_COUNTER_INDEX);
        };

        // Store the current actionability state for next frame
        assign_rwlock(&PLAYER_WAS_ACTIONABLE, player_is_actionable);
        assign_rwlock(&CPU_WAS_ACTIONABLE, cpu_is_actionable);
    }
}
