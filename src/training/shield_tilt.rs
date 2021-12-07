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
    get_angle(module_accessor).map(|a| a.cos() as f32)
}

pub unsafe fn mod_get_stick_y(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> Option<f32> {
    get_angle(module_accessor).map(|a| a.sin() as f32)
}

unsafe fn get_angle(module_accessor: &mut app::BattleObjectModuleAccessor) -> Option<f64> {
    if !is_operation_cpu(module_accessor) {
        return None;
    }

    STICK_DIRECTION.into_angle()
}
