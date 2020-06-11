use crate::common::consts::*;
use crate::common::*;
use core::f64::consts::PI;
use smash::app::{self, lua_bind::*, sv_system};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

pub static mut DI_ANGLE: f64 = 0.0;
pub static NO_DI: f64 = -69.0;

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

    if MENU.di_state == DirectionalInfluence::None {
        return;
    }

    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if !is_operation_cpu(module_accessor)() {
        return;
    }

    DI_ANGLE = (MENU.di_state as i32 - 1) as f64 * PI / 4.0;

    // Either left, right, or none
    if MENU.di_state == DirectionalInfluence::RandomInAway {
        let rand = app::sv_math::rand(hash40("fighter"), 3);
        // Either 0 (right) or PI (left)
        if [0, 1].contains(&rand) {
            DI_ANGLE = rand as f64 * PI;
        } else {
            DI_ANGLE = NO_DI;
        }
    }
    // If facing left, reverse angle
    if DI_ANGLE != NO_DI && PostureModule::lr(module_accessor) != -1.0 {
        DI_ANGLE -= PI;
    }

    if DI_ANGLE == NO_DI {
        return;
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
