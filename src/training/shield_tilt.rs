use crate::common::consts::*;
use crate::common::*;
use smash::app::{self};

static mut STICK_DIRECTION: Direction = Direction::OUT;

pub fn roll_direction() {
    unsafe {
        STICK_DIRECTION = MENU.shield_tilt.get_random();
    }
}

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
    if !is_operation_cpu(module_accessor) {
        return ANGLE_NONE;
    }

    let angle: f64 = STICK_DIRECTION.into_angle();

    angle
}
