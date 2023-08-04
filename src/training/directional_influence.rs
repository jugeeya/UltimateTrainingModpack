use core::f64::consts::PI;

use smash::app::{self, lua_bind::*, sv_system};
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

use crate::common::consts::*;
use crate::common::*;

static mut DI_CASE: Direction = Direction::empty();

pub fn roll_di_case() {
    unsafe {
        if DI_CASE != Direction::empty() {
            // DI direction already selected, don't pick a new one
            return;
        }

        DI_CASE = MENU.di_state.get_random();
    }
}

pub fn reset_di_case(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if is_in_hitstun(module_accessor) {
        // Don't reset the DI direction during hitstun
        return;
    }
    unsafe {
        if DI_CASE != Direction::empty() {
            DI_CASE = Direction::empty();
        }
    }
}

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

unsafe fn mod_handle_di(fighter: &L2CFighterCommon, _arg1: L2CValue) {
    if MENU.di_state == Direction::empty() {
        return;
    }

    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if !is_operation_cpu(module_accessor) {
        return;
    }

    roll_di_case();

    let angle_tuple = DI_CASE.into_angle().map_or((0.0, 0.0), |angle| {
        let a = if should_reverse_angle(&DI_CASE) {
            PI - angle
        } else {
            angle
        };

        (a.cos(), a.sin())
    });

    set_x_y(module_accessor, angle_tuple.0 as f32, angle_tuple.1 as f32);
}

pub fn should_reverse_angle(direction: &Direction) -> bool {
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    let player_module_accessor = get_module_accessor(FighterId::Player);
    unsafe {
        PostureModule::pos_x(player_module_accessor) > PostureModule::pos_x(cpu_module_accessor)
            && ![Direction::LEFT, Direction::RIGHT].contains(direction)
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

pub fn get_command_flag_cat(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) {
        return;
    }

    reset_di_case(module_accessor);
}
