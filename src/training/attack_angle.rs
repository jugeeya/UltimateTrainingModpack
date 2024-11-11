use smash::app::{self};

use crate::common::consts::*;
use crate::common::*;
use training_mod_sync::*;

static ATTACK_ANGLE_DIRECTION: RwLock<AttackAngle> = RwLock::new(AttackAngle::NEUTRAL);

pub fn roll_direction() {
    unsafe {
        assign_rwlock(&ATTACK_ANGLE_DIRECTION, MENU.attack_angle.get_random());
    }
}

pub unsafe fn mod_get_stick_dir(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> Option<f32> {
    if !is_operation_cpu(module_accessor) {
        return None;
    }

    match read_rwlock(&ATTACK_ANGLE_DIRECTION) {
        AttackAngle::UP => Some(1.0),
        AttackAngle::DOWN => Some(-1.0),
        _ => None,
    }
}
