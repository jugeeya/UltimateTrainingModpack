use crate::common::consts::*;
use crate::common::*;
use core::f64::consts::PI;
use smash::app::{self, lua_bind::*, sv_system};
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FighterStatusDamage__correctDamageVectorCommon)]
pub unsafe fn handle_correct_damage_vector_common(
    fighter: &mut L2CFighterCommon,
    arg1: L2CValue,
) -> L2CValue {
    if is_training_mode() {
        mod_handle_di(fighter, arg1);
    }

    original!()(fighter, arg1)
}

unsafe fn mod_handle_di(fighter: &mut L2CFighterCommon, _arg1: L2CValue) {
    if MENU.di_state == Direction::empty() {
        return;
    }

    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if !is_operation_cpu(module_accessor) {
        return;
    }

    // Either left, right, or none
    let angle_tuple = MENU.di_state
        .get_random()
        .into_angle()
        .map_or((0.0, 0.0), |angle| {
        let a = if should_reverse_angle() {
            PI - angle
        } else {
            angle
        };

        (a.cos(), a.sin())
    });

    set_x_y(module_accessor, angle_tuple.0 as f32, angle_tuple.1 as f32);
}

pub fn should_reverse_angle() -> bool {
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    let player_module_accessor = get_module_accessor(FighterId::Player);
    unsafe {
        PostureModule::pos_x(player_module_accessor)
            > PostureModule::pos_x(cpu_module_accessor)
    }
}

fn set_x_y(module_accessor: &mut app::BattleObjectModuleAccessor, x: f32, y: f32) {
    unsafe {
        WorkModule::set_float(
            module_accessor,
            x,
            *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X,
        );
        WorkModule::set_float(
            module_accessor,
            y,
            *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y,
        );
    }
}
