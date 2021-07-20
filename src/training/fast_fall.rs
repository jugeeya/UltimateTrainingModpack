use crate::common::*;
use crate::training::frame_counter;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};

static mut FRAME_COUNTER: usize = 0;

// The current fastfall delay
static mut DELAY: u32 = 0;

static mut FAST_FALL: bool = false;

fn should_fast_fall() -> bool {
    unsafe {
        FAST_FALL
    }
}

pub fn roll_fast_fall() {
    unsafe {
        FAST_FALL = MENU.fast_fall.get_random().into_bool();
    }
}

pub fn init() {
    unsafe {
        FRAME_COUNTER = frame_counter::register_counter();
    }
}

pub fn get_command_flag_cat(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !should_fast_fall() {
        return;
    }

    if !is_operation_cpu(module_accessor) {
        return;
    }

    if !is_airborne(module_accessor) {
        return;
    }

    // Need to be falling
    unsafe {
        if !is_falling(module_accessor) {
            // Roll FF delay
            DELAY = MENU.fast_fall_delay.get_random().into_delay();
            frame_counter::full_reset(FRAME_COUNTER);
            return;
        }

        if !is_correct_status(module_accessor) {
            return;
        }

        // Already in fast fall, nothing to do
        if WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            return;
        }

        // Check delay
        if frame_counter::should_delay(DELAY, FRAME_COUNTER) {
            return;
        }

        // Set Fast Fall Flag
        WorkModule::set_flag(
            module_accessor,
            true,
            *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE,
        );

        add_spark_effect(module_accessor);
    }
}

/**
 * Returns true for viable fast fall status
 */
fn is_correct_status(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status;

    unsafe {
        status = StatusModule::status_kind(module_accessor);
    }

    // Allow fast fall when falling
    if status == FIGHTER_STATUS_KIND_FALL {
        return true;
    }

    // Allow fast fall during aerials
    if status == FIGHTER_STATUS_KIND_ATTACK_AIR {
        return true;
    }

    false
}

/**
 * Returns true if the character is moving downwards
 */
pub fn is_falling(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let y_speed;
    unsafe {
        y_speed =
            KineticModule::get_sum_speed_y(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    y_speed < 0.0
}

unsafe fn add_spark_effect(module_accessor: &mut app::BattleObjectModuleAccessor) {
    // Mock Spark effect
    let pos = Vector3f {
        x: PostureModule::pos_x(module_accessor),
        y: PostureModule::pos_y(module_accessor),
        z: 0.0,
    };

    let rotation = Vector3f {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let size = 2.0;

    EffectModule::req(
        module_accessor,
        Hash40::new("sys_smash_flash_s"),
        &pos,
        &rotation,
        size,
        0,
        0,
        true,
        *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS,
    );
}
