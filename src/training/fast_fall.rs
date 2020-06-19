use crate::common::consts::FastFall;
use crate::common::*;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};

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

    if MENU.fast_fall != FastFall::On {
        return;
    }

    if !is_operation_cpu(module_accessor) {
        return;
    }

    if !is_airborne(module_accessor) {
        return;
    }

    let y_speed =
        KineticModule::get_sum_speed_y(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    // Need to be falling
    if y_speed >= 0.0 {
        return;
    }

    // Can't fastfall in hitstun
    if is_in_hitstun(module_accessor) {
        return;
    }

    // Already in fast fall, nothing to do
    if WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
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
