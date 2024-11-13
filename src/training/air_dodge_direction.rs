use core::f64::consts::PI;

use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

use crate::common::consts::*;
use crate::common::*;
use crate::training::directional_influence::should_reverse_angle;
use training_mod_sync::*;

static AIRDODGE_STICK_DIRECTION: RwLock<Direction> = RwLock::new(Direction::empty());

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

    // Currently used for air dodge//Drift only
    if !is_correct_status(module_accessor) {
        return None;
    }

    assign_rwlock(
        &AIRDODGE_STICK_DIRECTION,
        get(&MENU).air_dodge_dir.get_random(),
    );
    let direction = read_rwlock(&AIRDODGE_STICK_DIRECTION);
    direction.into_angle().map(|angle| {
        if !should_reverse_angle(direction) {
            // Direction is LEFT/RIGHT, so don't perform any adjustment
            angle
        } else {
            let launch_speed_x = KineticEnergy::get_speed_x(KineticModule::get_energy(
                module_accessor,
                *FIGHTER_KINETIC_ENERGY_ID_DAMAGE,
            )
                as *mut app::KineticEnergy);
            // If we're launched left, reverse stick X
            if launch_speed_x < 0.0 {
                PI - angle
            } else {
                angle
            }
        }
    })
}

fn is_correct_status(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    is_airborne(module_accessor) && is_in_hitstun(module_accessor)
}
