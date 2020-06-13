use crate::common::consts::*;
use crate::common::*;
use core::f64::consts::PI;
use smash::app::{self, lua_bind::*, sv_system};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

pub static mut DI_ANGLE: f64 = 0.0;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FighterStatusDamage__correctDamageVectorCommon)]
pub unsafe fn handle_correct_damage_vector_common(
    fighter: &mut L2CFighterCommon,
    arg1: L2CValue,
) -> L2CValue {
    mod_handle_di(fighter, arg1);
    original!()(fighter, arg1)
}

unsafe fn mod_handle_di(fighter: &mut L2CFighterCommon, _arg1: L2CValue) {
    if !is_training_mode() {
        return;
    }

    if MENU.di_state == Direction::None {
        return;
    }

    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if !is_operation_cpu(module_accessor) {
        return;
    }

    // Either left, right, or none
    if MENU.di_state == Direction::Random {
        DI_ANGLE = get_random_di();
    } else {
        DI_ANGLE = direction_to_angle(MENU.di_state)
    }

    // Nothig to do on no DI
    if DI_ANGLE == ANGLE_NONE {
        return;
    }

    // If facing left, reverse angle
    if PostureModule::lr(module_accessor) != FIGHTER_FACING_RIGHT {
        DI_ANGLE -= PI;
    }

    WorkModule::set_float(
        module_accessor,
        DI_ANGLE.cos() as f32,
        *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X,
    );
    WorkModule::set_float(
        module_accessor,
        DI_ANGLE.sin() as f32,
        *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y,
    );
}

unsafe fn get_random_di() -> f64 {
    let rand = app::sv_math::rand(hash40("fighter"), 3);
    if [0, 1].contains(&rand) {
        // Either 0 (right) or PI (left)
        rand as f64 * PI
    } else {
        ANGLE_NONE
    }
}
