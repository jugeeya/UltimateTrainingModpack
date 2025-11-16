use skyline::nn::ui2d::ResColor;
use smash::app::lua_bind::{AttackModule, CancelModule, StatusModule, WorkModule};
use smash::app::BattleObjectModuleAccessor;
use smash::lib::lua_const::*;

use crate::consts::Action;
use crate::info;
use crate::training::frame_counter;
use crate::training::ui::notifications;
use crate::try_get_module_accessor;

use training_mod_consts::{FighterId, OnOff, MENU};
use training_mod_sync::*;

static PLAYER_WAS_ACTIONABLE: RwLock<bool> = RwLock::new(false);
static CPU_WAS_ACTIONABLE: RwLock<bool> = RwLock::new(false);
static IS_COUNTING: RwLock<bool> = RwLock::new(false);

static PLAYER_FRAME_COUNTER_INDEX: LazyLock<usize> =
    LazyLock::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGame));
static CPU_FRAME_COUNTER_INDEX: LazyLock<usize> =
    LazyLock::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGame));

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
    if read(&MENU).frame_advantage == OnOff::ON {
        // Prioritize notifications for Frame Advantage
        notifications::clear_all_notifications();
        notifications::color_notification(
            "Frame Advantage".to_string(),
            format!("{frame_advantage:+}"),
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

pub unsafe fn once_per_frame(module_accessor: &mut BattleObjectModuleAccessor) {
    // Skip the CPU so we don't run twice per frame
    // Also skip if the CPU is set to mash since that interferes with the frame calculation
    let entry_id_int = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    if entry_id_int != (FighterId::Player as i32) || read(&MENU).mash_state != Action::empty() {
        return;
    }
    let player_module_accessor = try_get_module_accessor(FighterId::Player)
        .expect("Could not get player module accessor in once_per_frame");
    let cpu_module_accessor = try_get_module_accessor(FighterId::CPU)
        .expect("Could not get CPU module accessor in once_per_frame");
    let player_is_actionable = is_actionable(player_module_accessor);
    let player_was_actionable = read(&PLAYER_WAS_ACTIONABLE);
    let player_just_actionable = !player_was_actionable && player_is_actionable;
    let cpu_is_actionable = is_actionable(cpu_module_accessor);
    let cpu_was_actionable = read(&CPU_WAS_ACTIONABLE);
    let cpu_just_actionable = !cpu_was_actionable && cpu_is_actionable;

    // Lock in frames
    if cpu_just_actionable {
        frame_counter::stop_counting(*CPU_FRAME_COUNTER_INDEX);
    }

    if player_just_actionable {
        frame_counter::stop_counting(*PLAYER_FRAME_COUNTER_INDEX);
    }

    // DEBUG LOGGING
    // if read(&IS_COUNTING) {
    //     if player_is_actionable && cpu_is_actionable {
    //         info!("!");
    //     } else if !player_is_actionable && cpu_is_actionable {
    //         info!("-");
    //     } else if player_is_actionable && !cpu_is_actionable {
    //         info!("+");
    //     } else {
    //         info!(".");
    //     }
    // }

    if !player_is_actionable && !cpu_is_actionable {
        if AttackModule::is_infliction(
            player_module_accessor,
            *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD,
        ) || StatusModule::status_kind(player_module_accessor) == *FIGHTER_STATUS_KIND_THROW
        {
            if !read(&IS_COUNTING) {
                // Start counting when the player lands a hit
                info!("Starting frame counter");
            } else {
                // Note that we want the same behavior even if we are already counting!
                // This prevents multihit moves which aren't true combos from miscounting
                // from the first hit (e.g. Pikachu back air on shield)
                info!("Restarting frame counter");
            }

            frame_counter::reset_frame_count(*PLAYER_FRAME_COUNTER_INDEX);
            frame_counter::reset_frame_count(*CPU_FRAME_COUNTER_INDEX);
            frame_counter::start_counting(*PLAYER_FRAME_COUNTER_INDEX);
            frame_counter::start_counting(*CPU_FRAME_COUNTER_INDEX);
            assign(&IS_COUNTING, true);
        }
    } else if player_is_actionable && cpu_is_actionable {
        if read(&IS_COUNTING) {
            let frame_advantage = frame_counter::get_frame_count(*CPU_FRAME_COUNTER_INDEX) as i32
                - frame_counter::get_frame_count(*PLAYER_FRAME_COUNTER_INDEX) as i32;
            info!(
                "Stopping frame counter, frame advantage: {}",
                frame_advantage
            );
            update_frame_advantage(frame_advantage);
            frame_counter::reset_frame_count(*PLAYER_FRAME_COUNTER_INDEX);
            frame_counter::reset_frame_count(*CPU_FRAME_COUNTER_INDEX);
            assign(&IS_COUNTING, false);
        }
    } else {
        // No need to start or stop counting, one of the fighters is still not actionable
    }

    assign(&CPU_WAS_ACTIONABLE, cpu_is_actionable);
    assign(&PLAYER_WAS_ACTIONABLE, player_is_actionable);
}
