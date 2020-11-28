use crate::common::consts::*;
use crate::common::*;
use crate::training::directional_influence;
use core::f64::consts::PI;
use smash::app::{self, lua_bind::*, sv_system};
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use smash::Vector2f;

static mut COUNTER: u32 = 0;

static mut DIRECTION: Direction = Direction::NEUTRAL;

pub fn roll_direction() {
    unsafe {
        COUNTER = 0;
        DIRECTION = MENU.sdi_state.get_random();
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FighterStatusUniqProcessDamage_hit_stop_delay)]
pub unsafe fn process_hit_stop_delay(
    fighter: &mut L2CFighterCommon,
    arg1: L2CValue,
    hit_stop_delay_flick_mul: L2CValue,
    x: L2CValue,
    y: L2CValue,
) -> L2CValue {
    let mut new_x: L2CValue = x;
    let mut new_y: L2CValue = y;

    if is_training_mode() {
        let option = mod_sdi_direction(fighter);

        if let Some(angle) = option {
            new_x = (angle.cos() as f32).into();
            new_y = (angle.sin() as f32).into();
        }
    }

    original!()(fighter, arg1, hit_stop_delay_flick_mul, new_x, new_y)
}

fn mod_sdi_direction(fighter: &mut L2CFighterCommon) -> Option<f64> {
    unsafe {
        let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);

        if !is_operation_cpu(module_accessor) {
            return None;
        }

        DIRECTION.into_angle().map(|angle| {
            if directional_influence::should_reverse_angle() {
                PI - angle
            } else {
                angle
            }
        })
    }
}

#[skyline::hook(replace = FighterControlModuleImpl::check_hit_stop_delay_command)]
pub unsafe fn check_hit_stop_delay_command(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    arg1: *mut Vector2f,
) -> u64 {
    let ori = original!()(module_accessor, arg1);

    if !is_training_mode() {
        return ori;
    }

    mod_check_hit_stop_delay_command(module_accessor, arg1).unwrap_or(ori)
}

/**
 * Returning Some(1) here has the effect of enabling the call to process_hit_stop_delay()
 */
fn mod_check_hit_stop_delay_command(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    _arg1: *mut Vector2f,
) -> Option<u64> {
    if !is_operation_cpu(module_accessor) {
        return None;
    }
    unsafe {
        if DIRECTION == Direction::empty() {
            return None;
        }
    }

    unsafe {
        COUNTER = (COUNTER + 1) % MENU.sdi_strength.into_u32();
    }

    unsafe {
        if COUNTER != 1 {
            return None;
        }
    }

    Some(1)
}
