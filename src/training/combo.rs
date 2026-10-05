use skyline::nn::ui2d::ResColor;
use smash::app::lua_bind::{CancelModule, WorkModule};
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
// Frame (relative to the start of counting) on which each fighter first became actionable
static PLAYER_ACTIONABLE_AT: RwLock<Option<u32>> = RwLock::new(None);
static CPU_ACTIONABLE_AT: RwLock<Option<u32>> = RwLock::new(None);

static ELAPSED_FRAME_COUNTER_INDEX: LazyLock<usize> =
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

pub fn reset() {
    frame_counter::full_reset(*ELAPSED_FRAME_COUNTER_INDEX);
    assign(&PLAYER_ACTIONABLE_AT, None);
    assign(&CPU_ACTIONABLE_AT, None);
    assign(&IS_COUNTING, false);
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

    if !read(&IS_COUNTING) {
        // Start counting as soon as either fighter stops being actionable
        if !player_is_actionable || !cpu_is_actionable {
            info!("Starting frame counter");
            frame_counter::reset_frame_count(*ELAPSED_FRAME_COUNTER_INDEX);
            frame_counter::start_counting(*ELAPSED_FRAME_COUNTER_INDEX);
            // A fighter who is still actionable has no frames to wait out
            assign(&PLAYER_ACTIONABLE_AT, player_is_actionable.then_some(0));
            assign(&CPU_ACTIONABLE_AT, cpu_is_actionable.then_some(0));
            assign(&IS_COUNTING, true);
        }
    } else {
        // Lock in frames
        let elapsed = frame_counter::get_frame_count(*ELAPSED_FRAME_COUNTER_INDEX);
        if player_just_actionable {
            println!("Locking in Player frame: {}", elapsed);
            assign(&PLAYER_ACTIONABLE_AT, Some(elapsed));
        }
        if cpu_just_actionable {
            println!("Locking in CPU frame: {}", elapsed);
            assign(&CPU_ACTIONABLE_AT, Some(elapsed));
        }

        if player_is_actionable && cpu_is_actionable {
            info!("Both players are actionable!");
            if let (Some(player_frames), Some(cpu_frames)) =
                (read(&PLAYER_ACTIONABLE_AT), read(&CPU_ACTIONABLE_AT))
            {
                let frame_advantage = cpu_frames as i32 - player_frames as i32;
                info!(
                    "Stopping frame counter, frame advantage: {}",
                    frame_advantage
                );
                update_frame_advantage(frame_advantage);
            }
            reset();
        }
    }

    assign(&CPU_WAS_ACTIONABLE, cpu_is_actionable);
    assign(&PLAYER_WAS_ACTIONABLE, player_is_actionable);
}
