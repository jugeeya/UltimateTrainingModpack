use smash::hash40;
use smash::app::BattleObjectModuleAccessor;
use smash::app::sv_animcmd::{self};
use smash::app::lua_bind::*;
use smash::lib::{self, L2CAgent, L2CValue};
use smash::phx::{Hash40, Vector3f};
use smash::lib::lua_const::{*};
use smash::app::sv_system::{self};
use smash::app::{self};
use skyline::logging::hex_dump_ptr;
use crate::common::*;

/**
 * Rounds a number to the nearest multiple of another number.
 */
 pub fn round_to(val: f32, align: f32) -> f32 { (val / align).round() * align }

 /**
  * Linearly interpolates between two numbers, without bounds checking.
  */
pub fn lerp(min: f32, max: f32, t: f32) -> f32 { min + (max - min) * t }
 
pub fn unlerp(min: f32, max: f32, val: f32) -> f32 { (val - min) / (max - min) }
 
 /**
  * Linearly interpolates between two numbers, with bounds checking.
  */
pub fn lerp_bounded(min: f32, max: f32, t: f32) -> f32 {
    if t <= 0.0 { min } else { if t >= 1.0 { max } else { lerp(min, max, t) } }
}
 
pub fn unlerp_bounded(min: f32, max: f32, val: f32) -> f32 {
    if val <= min { 0.0 } else { if val >= max { 1.0 } else { unlerp(min, max, val)} }
}
 
 /**
  * Linearly nterpolates between two colors, with bounds checking, accounting for
  * gamma. arguments:
  * - min_color (Vector3f) -- xyz maps to rgb, components are usually in the
  * range [0.0f, 1.0f] but can go beyond to account for super-bright or
  * super-dark colors
  * - max_Color (Vector3f) -- same as minColor
  * - t (float) -- how far to interpolate between the colors
  * - gamma (float = 2.0f) -- used for color correction, helps avoid ugly dark
  * colors when interpolating b/t bright colors
  */
 
pub fn color_lerp(min_color: Vector3f, max_color: Vector3f, t: f32,
                    gamma: f32) -> Vector3f {
     let gamma_inv = 1.0 / gamma;
     let align =
         1.0 / 255.0;  // color components must be a multiple of 1/255
    Vector3f{x: round_to(lerp_bounded(min_color.x.powf(gamma),
                                        max_color.x.powf(gamma), t).powf(
                           gamma_inv),
                      align),
             y: round_to(lerp_bounded(min_color.y.powf(gamma),
                                        max_color.y.powf(gamma), t).powf(
                           gamma_inv),
                      align),
             z: round_to(lerp_bounded(min_color.z.powf(gamma),
                                        max_color.z.powf(gamma), t).powf(
                           gamma_inv),
                      align)}
 }

const ID_COLORS: &[Vector3f] = &[
    // used to tint the hitbox effects -- make sure that at least one component
    // is equal to 1.0
    Vector3f{x: 1.0, y: 0.0, z: 0.0},  // #ff0000 (red)
    Vector3f{x: 1.0, y: 0.4, z: 0.0},  // #ff9900 (orange)
    Vector3f{x: 0.8, y: 1.0, z: 0.0},  // #ccff00 (yellow)
    Vector3f{x: 0.2, y: 1.0, z: 0.2},  // #00ff33 (green)
    Vector3f{x: 0.0, y: 0.8, z: 1.0},  // #00ccff (sky blue)
    Vector3f{x: 0.4, y: 0.4, z: 1.0},  // #6666ff (blue)
    Vector3f{x: 0.8, y: 0.0, z: 1.0},  // #cc00ff (purple)
    Vector3f{x: 1.0, y: 0.2, z: 0.8},  // #ff33cc (pink)
];
const MAX_EFFECTS_PER_HITBOX: i32 = 16; // max # of circles drawn for an extended hitbox

pub unsafe fn wrap(func: unsafe extern "C" fn(lua_state: u64), agent: &mut L2CAgent, vals: &mut Vec::<L2CValue>) {
    agent.clear_lua_stack();
    for val in vals {
            agent.push_lua_stack(val);
    }
    func(agent.lua_state_agent);
    agent.clear_lua_stack();
}

pub unsafe fn generate_hitbox_effects(l2c_agent: &mut L2CAgent, bone: L2CValue,
        size: L2CValue, x: L2CValue, y: L2CValue,
        z: L2CValue, x2: L2CValue, y2: L2CValue,
        z2: L2CValue, color: Vector3f) {
    let red = L2CValue::new_num(color.x);
    let green = L2CValue::new_num(color.y);
    let blue = L2CValue::new_num(color.z);

    let size_mult = 19.0 / 200.0;

    let shield_effect = L2CValue::new_int(hash40("sys_shield"));
    let zero_rot = L2CValue::new_num(0.0);
    let terminate = L2CValue::new_bool(true);
    let effect_size = L2CValue::new_num(size.get_num() * size_mult);

    let rate = L2CValue::new_num(8.0);

    let x_dist : f32;
    let y_dist : f32;
    let z_dist : f32;
    let mut n_effects : i32;
    if let lib::L2CValueType::Void = x2.val_type{ // && let lib::L2CValueType::Void = y2.val_type && let lib::L2CValueType::Void = z2.val_type {  // extended hitbox
        x_dist = 0.0; y_dist = 0.0; z_dist = 0.0;
        n_effects = 1;
    } 
    else {  // non-extended hitbox
        x_dist = x2.get_num() - x.get_num();
        y_dist = y2.get_num() - y.get_num();
        z_dist = z2.get_num() - z.get_num();
        let dist_sq : f32 = x_dist * x_dist + y_dist * y_dist + z_dist * z_dist;
        let dist = dist_sq.sqrt();
        n_effects = ((dist / (size.get_num() * 1.75)) + 1.0).ceil() as i32;  // just enough effects to form a continuous line
        if n_effects < 2 {
            n_effects = 2;
        } else if n_effects > MAX_EFFECTS_PER_HITBOX {
            n_effects = MAX_EFFECTS_PER_HITBOX;
        }
    }

    for i in  0..n_effects {
        let mut t = 0.0;
        if n_effects > 1 {
            t = (i as f32) / ((n_effects - 1) as f32);
        }
        let x_curr = L2CValue::new_num(x.get_num() + x_dist * t);
        let y_curr = L2CValue::new_num(y.get_num() + y_dist * t);
        let z_curr = L2CValue::new_num(z.get_num() + z_dist * t);

        wrap(sv_animcmd::EFFECT_FOLLOW_NO_SCALE,
            l2c_agent, 
            &mut [
            shield_effect, bone, x_curr, 
            y_curr, z_curr, zero_rot, zero_rot,
            zero_rot, effect_size, terminate].to_vec());

        // set to hitbox ID color
        wrap(sv_animcmd::LAST_EFFECT_SET_COLOR, l2c_agent,
            &mut [red, green, blue].to_vec());

        // speed up animation by rate to remove pulsing effect
        wrap(sv_animcmd::LAST_EFFECT_SET_RATE, l2c_agent, 
            &mut [rate].to_vec());
    }
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn handle_attack(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);

    // get all necessary hitbox params
    let id = l2c_agent.pop_lua_stack(1);      // int
    let bone = l2c_agent.pop_lua_stack(3);    // hash40
    let damage = l2c_agent.pop_lua_stack(4);  // float
    let _angle = l2c_agent.pop_lua_stack(5);   // int
    let kbg = l2c_agent.pop_lua_stack(6);     // int
    let fkb = l2c_agent.pop_lua_stack(7);     // int
    let bkb = l2c_agent.pop_lua_stack(8);     // int
    let size = l2c_agent.pop_lua_stack(9);    // float
    let x = l2c_agent.pop_lua_stack(10);      // float
    let y = l2c_agent.pop_lua_stack(11);      // float
    let z = l2c_agent.pop_lua_stack(12);      // float
    let x2 = l2c_agent.pop_lua_stack(13);     // float or void
    let y2 = l2c_agent.pop_lua_stack(14);     // float or void
    let z2 = l2c_agent.pop_lua_stack(15);     // float or void

    original!()(lua_state);

    if menu.HITBOX_VIS && is_training_mode() {  // generate hitbox effect(s)
        let color_scale: f32;
        if false {  // color intensity scales with damage
            color_scale = unlerp_bounded(1.0, 18.0, damage.get_num());
        } else {  // color intensity scales with total KB
            // calculate the expected KB a character with 95 weight will receive
            // at 80% pre-hit
            let target_percent = 80.0;
            let target_weight = 95.0;
            let percent_component: f32;
            if fkb.get_int() > 0 {
                percent_component = (10.0 + fkb.get_int() as f32) * 0.1 * (1.0 + fkb.get_int() as f32 * 0.5);
            } else {
                percent_component =  (target_percent + damage.get_num()) * 0.1 *
                                    (1.0 + damage.get_num() * 0.5);
            }
            let weight_component: f32 = 200.0 / (target_weight + 100.0);
            let kb: f32 = (percent_component * weight_component * 1.4 + 18.0) *
                           (kbg.get_int() as f32 * 0.01) + bkb.get_int() as f32;
            color_scale = unlerp_bounded(50.0, 200.0, kb);
        }
        // non-linear scaling to magnify
        // differences at lower values
        let color_t: f32 = 0.8 + 0.2 * color_scale.powf(0.5);
        let color = color_lerp(
            Vector3f{x: 1.0, y: 1.0, z: 1.0},
            ID_COLORS[(id.get_int() % 8) as usize],
            color_t, 
            2.0
        );
        generate_hitbox_effects(&mut l2c_agent, bone, size, x, y, z, x2, y2, z2, color);
    }
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = sv_animcmd::CATCH)]
unsafe fn handle_catch(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);

    // get all necessary grabbox params
    let id = l2c_agent.pop_lua_stack(1);     // int
    let joint = l2c_agent.pop_lua_stack(2);  // hash40
    let size = l2c_agent.pop_lua_stack(3);   // float
    let x = l2c_agent.pop_lua_stack(4);      // float
    let y = l2c_agent.pop_lua_stack(5);      // float
    let z = l2c_agent.pop_lua_stack(6);      // float
    let x2 = l2c_agent.pop_lua_stack(7);     // float or void
    let y2 = l2c_agent.pop_lua_stack(8);     // float or void
    let z2 = l2c_agent.pop_lua_stack(9);     // float or void

    original!()(lua_state);

    if menu.HITBOX_VIS && is_training_mode() {
        generate_hitbox_effects(&mut l2c_agent, joint, size, x, y, z, x2, y2, z2, ID_COLORS[(id.get_int() + 3 % 8) as usize]);
    }
}

pub unsafe fn is_shielding(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    (FIGHTER_STATUS_KIND_GUARD_ON..=FIGHTER_STATUS_KIND_GUARD_OFF).contains(&status_kind)
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = AttackModule::clear_all)]
pub unsafe fn handle_clear_all(module_accessor: *mut BattleObjectModuleAccessor) {
    if is_training_mode() {
        // only if we're not shielding
        if !is_shielding(module_accessor) {
            EffectModule::kill_kind(module_accessor, Hash40{hash: hash40("sys_shield")}, false, true);
        }
    }

    original!()(module_accessor);
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = GrabModule::set_rebound)]
pub unsafe fn handle_set_rebound(module_accessor: *mut BattleObjectModuleAccessor, rebound: bool) {
    if is_training_mode() && rebound == false {
        // only if we're not shielding
        if !is_shielding(module_accessor) {
            EffectModule::kill_kind(module_accessor, Hash40{hash: hash40("sys_shield")}, false, true);
        }
    }

    original!()(module_accessor, rebound);
}

pub fn hitbox_visualization() {
    println!("Applying hitbox visualization mods.");
    skyline::install_hook!(handle_attack);
    skyline::install_hook!(handle_catch);
    skyline::install_hook!(handle_clear_all);
    skyline::install_hook!(handle_set_rebound);
}
