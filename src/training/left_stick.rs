use crate::common::consts::*;
use crate::common::*;
use core::f64::consts::PI;
use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;

static mut STICK_DIRECTION: Direction = Direction::None;

pub unsafe fn mod_get_stick_x(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> Option<f32> {
    let angle: f64 = get_angle(module_accessor);

    if angle == ANGLE_NONE {
        return None;
    }

    Some(angle.cos() as f32)
}

pub unsafe fn mod_get_stick_y(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> Option<f32> {
    let angle: f64 = get_angle(module_accessor);

    if angle == ANGLE_NONE {
        return None;
    }

    Some(angle.sin() as f32)
}

unsafe fn get_angle(module_accessor: &mut app::BattleObjectModuleAccessor) -> f64 {
    if !is_training_mode() {
        return ANGLE_NONE;
    }

    if !is_operation_cpu(module_accessor) {
        return ANGLE_NONE;
    }

    // Currently used for air dodge//Drift only
    if !(is_airborne(module_accessor) && is_in_hitstun(module_accessor)) {
        return ANGLE_NONE;
    }

    STICK_DIRECTION = MENU.di_state;
    let mut angle: f64 = pick_angle(STICK_DIRECTION);

    if angle == ANGLE_NONE {
        return ANGLE_NONE;
    }

    // If facing left, reverse angle
    if PostureModule::lr(module_accessor) != FIGHTER_FACING_RIGHT {
        angle -= PI;
    }

    angle
}

unsafe fn pick_angle(direction: Direction) -> f64 {
    if direction == Direction::Random {
        let rand_direction = get_random_direction();
        return direction_to_angle(rand_direction);
    }

    direction_to_angle(direction)
}

unsafe fn get_random_direction() -> Direction {
    let rand = app::sv_math::rand(hash40("fighter"), 8);
    Direction::from(rand)
}
