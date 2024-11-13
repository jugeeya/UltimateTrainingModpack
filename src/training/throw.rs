use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

use crate::common::consts::*;
use crate::common::*;
use crate::training::frame_counter;
use crate::training::mash;
use training_mod_sync::*;

const NOT_SET: u32 = 9001;
static THROW_DELAY: RwLock<u32> = RwLock::new(NOT_SET);
static PUMMEL_DELAY: RwLock<u32> = RwLock::new(NOT_SET);
static THROW_CASE: RwLock<ThrowOption> = RwLock::new(ThrowOption::empty());

static THROW_DELAY_COUNTER: LazyLock<usize> =
    LazyLock::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGame));
static PUMMEL_DELAY_COUNTER: LazyLock<usize> =
    LazyLock::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGame));

// Rolling Throw Delays and Pummel Delays separately

pub fn reset_throw_delay() {
    if read_rwlock(&THROW_DELAY) != NOT_SET {
        assign_rwlock(&THROW_DELAY, NOT_SET);
        frame_counter::full_reset(*THROW_DELAY_COUNTER);
    }
}

pub fn reset_pummel_delay() {
    if read_rwlock(&PUMMEL_DELAY) != NOT_SET {
        assign_rwlock(&PUMMEL_DELAY, NOT_SET);
        frame_counter::full_reset(*PUMMEL_DELAY_COUNTER);
    }
}

pub fn reset_throw_case() {
    if read_rwlock(&THROW_CASE) != ThrowOption::empty() {
        // Don't roll another throw option if one is already selected
        assign_rwlock(&THROW_CASE, ThrowOption::empty());
    }
}

fn roll_throw_delay() {
    if read_rwlock(&THROW_DELAY) == NOT_SET {
        // Only roll another throw delay if one is not already selected
        assign_rwlock(
            &THROW_DELAY,
            get(&MENU).throw_delay.get_random().into_meddelay(),
        );
    }
}

fn roll_pummel_delay() {
    if read_rwlock(&PUMMEL_DELAY) == NOT_SET {
        // Don't roll another pummel delay if one is already selected
        assign_rwlock(
            &PUMMEL_DELAY,
            get(&MENU).pummel_delay.get_random().into_meddelay(),
        );
    }
}

fn roll_throw_case() {
    if read_rwlock(&THROW_CASE) == ThrowOption::empty() {
        // Only re-roll if there is not already a throw option selected
        assign_rwlock(&THROW_CASE, get(&MENU).throw_state.get_random());
    }
}

pub unsafe fn get_command_flag_throw_direction(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> i32 {
    if !is_operation_cpu(module_accessor) {
        return 0;
    }

    if StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_CATCH_WAIT
        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_CATCH_PULL
        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_CATCH_ATTACK
    {
        // No longer holding character, so re-roll the throw case and reset the delay counter for next time
        reset_throw_case();
        reset_throw_delay();

        reset_pummel_delay();
        return 0;
    }

    if !WorkModule::is_enable_transition_term(
        // If you can't throw right now, don't bother
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI,
    ) {
        return 0;
    }

    roll_throw_delay();
    roll_throw_case();

    roll_pummel_delay();

    if read_rwlock(&THROW_CASE) == ThrowOption::NONE {
        // Do nothing, but don't reroll the throw case.
        return 0;
    }

    if frame_counter::should_delay(read_rwlock(&THROW_DELAY), *THROW_DELAY_COUNTER) {
        // Not yet time to perform the throw action
        if frame_counter::should_delay(read_rwlock(&PUMMEL_DELAY), *PUMMEL_DELAY_COUNTER) {
            // And not yet time to pummel either, so don't do anything
            return 0;
        }

        // If no pummel delay is selected (default), then don't pummel
        if get(&MENU).pummel_delay == MedDelay::empty() {
            return 0;
        }

        // (this conditional would need to be changed to speed up pummelling)
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCH_WAIT {
            let status = *FIGHTER_STATUS_KIND_CATCH_ATTACK; //.unwrap_or(0);
            StatusModule::change_status_request_from_script(module_accessor, status, true);
        }

        return 0;
    }

    // If you can uthrow, then throw (since all throws should be possible at the same times)
    if WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI,
    ) {
        let cmd = read_rwlock(&THROW_CASE).into_cmd().unwrap_or(0);
        mash::external_buffer_menu_mash(get(&MENU).mash_state.get_random());
        return cmd;
    }

    0
}
