use crate::common::consts::*;
use crate::common::*;
use core::f64::consts::PI;
use smash::app::{lua_bind::*, sv_system};
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

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
    let mut angle = get_angle(MENU.di_state);
    // Nothing to do on no DI
    if angle == ANGLE_NONE {
        return;
    }

    let launch_speed_x = KineticEnergy::get_speed_x(
        KineticModule::get_energy(
            module_accessor, 
            *FIGHTER_KINETIC_ENERGY_ID_DAMAGE
        ) as *mut smash::app::KineticEnergy);

    // If we're launched left, reverse stick X
    if launch_speed_x < 0.0 {
        angle = PI - angle;
    }

    WorkModule::set_float(
        module_accessor,
        angle.cos() as f32,
        *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X,
    );
    WorkModule::set_float(
        module_accessor,
        angle.sin() as f32,
        *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y,
    );
}

unsafe fn get_angle(direction: Direction) -> f64 {
    if direction == Direction::Random {
        let rand_direction = get_random_direction();
        return direction_to_angle(rand_direction);
    }

    direction_to_angle(direction)
}

unsafe fn get_random_direction() -> Direction {
    // Choose Left/Right/None
    let rand = get_random_int(3);
    if rand == 0 {
        Direction::Left
    } else if rand == 1 {
        Direction::Right
    } else {
        Direction::None
    }
}
