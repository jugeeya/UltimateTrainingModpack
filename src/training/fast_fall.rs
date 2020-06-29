use crate::common::consts::OnOff;
use crate::common::*;
use crate::training::frame_counter;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};

static mut FRAME_COUNTER: usize = 0;

pub fn init() {
    unsafe {
        FRAME_COUNTER = frame_counter::register_counter();
    }
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) {
    if !is_training_mode() {
        return;
    }

    // Once per frame
    if category != 0 {
        return;
    }

    if MENU.fast_fall != OnOff::On {
        return;
    }

    if !is_operation_cpu(module_accessor) {
        return;
    }

    if !is_airborne(module_accessor) {
        return;
    }

    // Need to be falling
    if !is_falling(module_accessor) {
        return;
    }

    // Can't fastfall in hitstun // tumble // meteor
    if is_in_hitstun(module_accessor) {
        return;
    }

    // Already in fast fall, nothing to do
    if WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
        return;
    }

    // Check delay
    if should_delay() {
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

fn should_delay() -> bool {
    unsafe {
        if MENU.fast_fall_delay == 0 {
            return false;
        }

        let current_frame = frame_counter::get_frame_count(FRAME_COUNTER);

        if current_frame == 0 {
            frame_counter::start_counting(FRAME_COUNTER);
        }

        if current_frame >= MENU.fast_fall_delay {
            frame_counter::reset_frame_count(FRAME_COUNTER);
            return false;
        }

        return true;
    }
}

pub fn is_falling(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        let y_speed =
            KineticModule::get_sum_speed_y(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        y_speed < 0.0
    }
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
