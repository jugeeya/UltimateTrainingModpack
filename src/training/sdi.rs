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

    let res;

    let new_x: L2CValue;
    let new_y: L2CValue;
    let vector = mod_sdi_direction(fighter);
    if vector.is_some() {
        new_x = vector.unwrap().x.into();
        new_y = vector.unwrap().y.into();
    }
    else {
        new_x = x;
        new_y = y;
    }

    res = original!()(fighter, arg1, hit_stop_delay_flick_mul, new_x, new_y);

    res
}

fn mod_sdi_direction(fighter: &mut L2CFighterCommon) -> Option<Vector2f> {
    unsafe {
        let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);

        if !is_training_mode() {
            return None;
        }

        if !is_operation_cpu(module_accessor) {
            return None;
        }
    }
    let mut angle: f64;

    unsafe {
        angle = DIRECTION.into_angle();
    }

    if angle == ANGLE_NONE {
        return None;
    }

    if directional_influence::should_reverse_angle() {
        angle = PI - angle;
    }

    return Some(Vector2f {
        x: angle.cos() as f32,
        y: angle.sin() as f32,
    });
}

#[skyline::hook(replace = FighterControlModuleImpl::check_hit_stop_delay_command)]
pub unsafe fn check_hit_stop_delay_command(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    arg1: *mut Vector2f,
) -> u64 {
    let ori = original!()(module_accessor, arg1);
    let res = mod_check_hit_stop_delay_command(module_accessor, arg1).unwrap_or_else(|| ori);

    res
}

/**
 * Returning Some(1) here has the effect of enabling the call to process_hit_stop_delay()
 */
fn mod_check_hit_stop_delay_command(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    _arg1: *mut Vector2f,
) -> Option<u64> {
    unsafe {
        if !is_training_mode() {
            return None;
        }
    }

    if !is_operation_cpu(module_accessor) {
        return None;
    }
    unsafe {
        if DIRECTION == Direction::empty() {
            return None;
        }
    }

    unsafe {
        COUNTER = (COUNTER + 1) % 4;
    }

    unsafe {
        if COUNTER != 1 {
            return None;
        }
    }

    return Some(1);
}
