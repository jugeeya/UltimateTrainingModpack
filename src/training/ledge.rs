use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

use crate::common::consts::*;
use crate::common::*;
use crate::training::{frame_counter, input_record, mash};

use training_mod_sync::*;

const NOT_SET: u32 = 9001;
static LEDGE_DELAY: RwLock<u32> = RwLock::new(NOT_SET);
static LEDGE_CASE: RwLock<LedgeOption> = RwLock::new(LedgeOption::empty());

static LEDGE_DELAY_COUNTER: LazyLock<usize> =
    LazyLock::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGame));

pub fn reset_ledge_delay() {
    let mut ledge_delay_lock = lock_write(&LEDGE_DELAY);
    if *ledge_delay_lock != NOT_SET {
        *ledge_delay_lock = NOT_SET;
        frame_counter::full_reset(*LEDGE_DELAY_COUNTER);
    }
}

pub fn reset_ledge_case() {
    let mut ledge_case_lock = lock_write(&LEDGE_CASE);
    if *ledge_case_lock != LedgeOption::empty() {
        // Don't roll another ledge option if one is already selected
        *ledge_case_lock = LedgeOption::empty();
    }
}

fn roll_ledge_delay() {
    let mut ledge_delay_lock = lock_write(&LEDGE_DELAY);
    if *ledge_delay_lock != NOT_SET {
        // Don't roll another ledge delay if one is already selected
        return;
    }
    *ledge_delay_lock = read(&MENU).ledge_delay.get_random().into_longdelay();
}

fn roll_ledge_case() {
    // Don't re-roll if there is already a ledge option selected
    // This prevents choosing a different ledge option during LedgeOption::WAIT
    let mut ledge_case_lock = lock_write(&LEDGE_CASE);
    if *ledge_case_lock != LedgeOption::empty() {
        return;
    }
    *ledge_case_lock = read(&MENU).ledge_state.get_random();
}

fn get_ledge_option() -> Option<Action> {
    let mut override_action: Option<Action> = None;
    let regular_action = if read(&MENU).mash_triggers.contains(&MashTrigger::LEDGE) {
        Some(read(&MENU).mash_state.get_random())
    } else {
        None
    };

    match read(&LEDGE_CASE) {
        LedgeOption::NEUTRAL => {
            if read(&MENU).ledge_neutral_override != Action::empty() {
                override_action = Some(read(&MENU).ledge_neutral_override.get_random());
            }
        }
        LedgeOption::ROLL => {
            if read(&MENU).ledge_roll_override != Action::empty() {
                override_action = Some(read(&MENU).ledge_roll_override.get_random());
            }
        }
        LedgeOption::JUMP => {
            if read(&MENU).ledge_jump_override != Action::empty() {
                override_action = Some(read(&MENU).ledge_jump_override.get_random());
            }
        }
        LedgeOption::ATTACK => {
            if read(&MENU).ledge_attack_override != Action::empty() {
                override_action = Some(read(&MENU).ledge_attack_override.get_random());
            }
        }
        _ => {
            override_action = None;
        }
    }
    override_action.or(regular_action)
}

pub unsafe fn force_option(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if StatusModule::situation_kind(module_accessor) != *SITUATION_KIND_CLIFF {
        // No longer on ledge, so re-roll the ledge case and reset the delay counter for next time
        reset_ledge_case();
        reset_ledge_delay();
        return;
    }
    // Need to roll ledge delay so we know if getup needs to be buffered
    roll_ledge_delay();
    roll_ledge_case();

    // This flag is false when needing to buffer, and true when getting up
    let flag_cliff =
        WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF);
    let current_frame = MotionModule::frame(module_accessor) as i32;
    let ledge_delay = read(&LEDGE_DELAY);
    let ledge_case = read(&LEDGE_CASE);
    // Allow this because sometimes we want to make sure our NNSDK doesn't have
    // an erroneous definition
    #[allow(clippy::unnecessary_cast)]
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    let should_buffer_playback = (ledge_delay == 0) && (current_frame == 13); // 18 - 5 of buffer
    let should_buffer;
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);

    if status_kind == *FIGHTER_STATUS_KIND_CLIFF_WAIT
        && prev_status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH
    {
        // For regular ledge grabs, we were just in catch and want to buffer on this frame
        should_buffer = (ledge_delay == 0) && (current_frame == 19) && (!flag_cliff);
    } else if status_kind == *FIGHTER_STATUS_KIND_CLIFF_WAIT {
        // otherwise we're in "wait" from grabbing with lasso, so we want to buffer on frame
        should_buffer = (ledge_delay == 0) && (current_frame == 18) && (flag_cliff);
    } else {
        should_buffer = false;
    }

    if !WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_ATTACK,
    ) {
        // Not able to take any action yet
        // We buffer playback on frame 18 because we don't change status this frame from inputting on next frame; do we need to do one earlier for lasso?
        if should_buffer_playback
            && ledge_case.is_playback()
            && read(&MENU).ledge_delay != LongDelay::empty()
        {
            input_record::playback_ledge(ledge_case.playback_slot());
            return;
        }
        // This check isn't reliable for buffered options in time, so don't return if we need to buffer an option this frame
        if !should_buffer {
            return;
        }
    }

    if ledge_case == LedgeOption::WAIT {
        // Do nothing, but don't reset the ledge case.
        return;
    }

    if frame_counter::should_delay(ledge_delay, *LEDGE_DELAY_COUNTER) {
        // Not yet time to perform the ledge action
        return;
    }

    let status = ledge_case.into_status().unwrap_or(0);
    if ledge_case.is_playback() {
        input_record::playback(ledge_case.playback_slot());
    } else {
        StatusModule::change_status_request_from_script(module_accessor, status, true);
    }

    if let Some(ledge_option) = get_ledge_option() {
        mash::external_buffer_menu_mash(ledge_option);
    }
}

pub unsafe fn is_enable_transition_term(
    _module_accessor: *mut app::BattleObjectModuleAccessor,
    term: i32,
) -> Option<bool> {
    if !is_operation_cpu(&mut *_module_accessor) {
        return None;
    }

    // Only handle ledge scenarios from menu
    if StatusModule::status_kind(_module_accessor) != *FIGHTER_STATUS_KIND_CLIFF_WAIT
        || read(&MENU).ledge_state == LedgeOption::empty()
    {
        return None;
    }

    // Disallow the default cliff-climb if we are waiting or we didn't get up during a recording
    let ledge_case = read(&LEDGE_CASE);
    let ledge_delay = read(&LEDGE_DELAY);
    if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB
        && ((ledge_case == LedgeOption::WAIT
            || frame_counter::get_frame_count(*LEDGE_DELAY_COUNTER) < ledge_delay)
            || (ledge_case.is_playback() && !input_record::is_playback()))
    {
        return Some(false);
    }
    None
}

pub fn get_command_flag_cat(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) {
        return;
    }

    // Set up check for beginning of ledge grab
    unsafe {
        // Behave normally if we're playing back recorded inputs or controlling the cpu
        if input_record::is_playback() {
            return;
        }

        if read(&MENU).ledge_state == LedgeOption::empty() {
            return;
        }

        force_option(module_accessor);
    }
}
