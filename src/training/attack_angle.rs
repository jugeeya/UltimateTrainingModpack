use crate::common::consts::*;
use crate::common::*;
use smash::app::{self};

static mut DIRECTION: AttackAngle = AttackAngle::NEUTRAL;

pub fn roll_direction() {
    unsafe {
        DIRECTION = MENU.attack_angle.get_random();
    }
}

pub unsafe fn mod_get_stick_dir(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> Option<f32> {
    if !is_operation_cpu(module_accessor) {
        return None;
    }

    match DIRECTION {
        AttackAngle::UP => Some(1.0),
        AttackAngle::DOWN => Some(-1.0),
        _ => None,
    }
}
