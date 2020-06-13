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
    if !is_training_mode() {
        return None;
    }

    if !is_operation_cpu(module_accessor) {
        return None;
    }

    let status_kind = StatusModule::status_kind(module_accessor);
    if !status_kind == FIGHTER_STATUS_KIND_ESCAPE_AIR {
        return None;
    }

    STICK_DIRECTION = MENU.di_state;
    let mut angle: f64 = get_angle(STICK_DIRECTION);

    if angle == ANGLE_NONE {
        return None;
    }

    // If facing left, reverse angle
    if PostureModule::lr(module_accessor) != FIGHTER_FACING_RIGHT {
        angle -= PI;
    }

    Some(angle.cos() as f32)
}

pub unsafe fn mod_get_stick_y(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> Option<f32> {
    if !is_training_mode() {
        return None;
    }

    if !is_operation_cpu(module_accessor) {
        return None;
    }

    let status_kind = StatusModule::status_kind(module_accessor);
    if !status_kind == FIGHTER_STATUS_KIND_ESCAPE_AIR {
        return None;
    }

    STICK_DIRECTION = MENU.di_state;
    let mut angle: f64 = get_angle(STICK_DIRECTION);

    if angle == ANGLE_NONE {
        return None;
    }

    // If facing left, reverse angle
    if PostureModule::lr(module_accessor) != FIGHTER_FACING_RIGHT {
        angle -= PI;
    }

    Some(angle.sin() as f32)
}

unsafe fn get_angle(direction: Direction) -> f64 {
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
