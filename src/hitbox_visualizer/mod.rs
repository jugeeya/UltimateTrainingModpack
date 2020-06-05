use crate::common::{*};
use smash::app::{self, lua_bind::*, sv_animcmd, sv_system};
use smash::lib::{lua_const::*, L2CAgent};
use smash::phx::{Hash40, Vector3f};

pub const ID_COLORS: &[Vector3f] = &[
    // used to tint the hitbox effects -- make sure that at least one component
    // is equal to 1.0
    Vector3f {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    }, // #ff0000 (red)
    Vector3f {
        x: 1.0,
        y: 0.4,
        z: 0.0,
    }, // #ff9900 (orange)
    Vector3f {
        x: 0.8,
        y: 1.0,
        z: 0.0,
    }, // #ccff00 (yellow)
    Vector3f {
        x: 0.2,
        y: 1.0,
        z: 0.2,
    }, // #00ff33 (green)
    Vector3f {
        x: 0.0,
        y: 0.8,
        z: 1.0,
    }, // #00ccff (sky blue)
    Vector3f {
        x: 0.4,
        y: 0.4,
        z: 1.0,
    }, // #6666ff (blue)
    Vector3f {
        x: 0.8,
        y: 0.0,
        z: 1.0,
    }, // #cc00ff (purple)
    Vector3f {
        x: 1.0,
        y: 0.2,
        z: 0.8,
    }, // #ff33cc (pink)
];
const MAX_EFFECTS_PER_HITBOX: i32 = 16; // max # of circles drawn for an extended hitbox

pub unsafe fn generate_hitbox_effects(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    bone: u64,
    size: f32,
    x: f32,
    y: f32,
    z: f32,
    x2: Option<f32>,
    y2: Option<f32>,
    z2: Option<f32>,
    color: Vector3f,
) {
    let size_mult = 19.0 / 200.0;

    let x_dist: f32;
    let y_dist: f32;
    let z_dist: f32;
    let mut n_effects: i32;
    if x2 == None && y2 == None && z2 == None {
        // && let lib::L2CValueType::Void = y2.val_type && let lib::L2CValueType::Void = z2.val_type {  // extended hitbox
        x_dist = 0.0;
        y_dist = 0.0;
        z_dist = 0.0;
        n_effects = 1;
    } else {
        // non-extended hitbox
        x_dist = x2.unwrap_or(0.0) - x;
        y_dist = y2.unwrap_or(0.0) - y;
        z_dist = z2.unwrap_or(0.0) - z;
        let dist_sq: f32 = x_dist * x_dist + y_dist * y_dist + z_dist * z_dist;
        let dist = dist_sq.sqrt();
        n_effects = ((dist / (size * 1.75)) + 1.0).ceil() as i32; // just enough effects to form a continuous line
        if n_effects < 2 {
            n_effects = 2;
        } else if n_effects > MAX_EFFECTS_PER_HITBOX {
            n_effects = MAX_EFFECTS_PER_HITBOX;
        }
    }

    for i in 0..n_effects {
        let mut t = 0.0;
        if n_effects > 1 {
            t = (i as f32) / ((n_effects - 1) as f32);
        }
        let x_curr = x + x_dist * t;
        let y_curr = y + y_dist * t;
        let z_curr = z + z_dist * t;

        let pos = Vector3f {
            x: x_curr,
            y: y_curr,
            z: z_curr,
        };
        let zeros = Vector3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        if is_fighter(module_accessor) {
            EffectModule::req_on_joint(
                module_accessor,
                Hash40::new("sys_shield"),
                Hash40::new_raw(bone),
                &pos,
                &zeros,
                size * size_mult,
                &zeros,
                &zeros,
                true,
                *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32
                    | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32
                    | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32,
                0,
                0,
            );
        } else {
            EffectModule::req_follow(
                module_accessor,
                Hash40::new("sys_shield"),
                Hash40::new_raw(bone),
                &pos,
                &zeros,
                size * size_mult,
                true,
                *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32
                    | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32
                    | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32,
                0,
                0,
                0,
                0,
                true,
                true,
            );
        }

        // set to hitbox ID color
        EffectModule::set_rgb_partial_last(module_accessor, color.x, color.y, color.z);

        // speed up animation by rate to remove pulsing effect
        EffectModule::set_rate_last(module_accessor, 8.0);
    }
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) {
    if is_training_mode() {
        // Pause Effect AnimCMD if hitbox visualization is active
        MotionAnimcmdModule::set_sleep_effect(module_accessor, MENU.hitbox_vis
        && !((*FIGHTER_STATUS_KIND_CATCH..=*FIGHTER_STATUS_KIND_TREAD_FALL).contains(&StatusModule::status_kind(module_accessor))));
    }

    // apply only once per frame
    if category == 0 && is_training_mode() && MENU.hitbox_vis {
        let status_kind = StatusModule::status_kind(module_accessor) as i32;
        if !(*FIGHTER_STATUS_KIND_CATCH..=*FIGHTER_STATUS_KIND_CATCH_TURN).contains(&status_kind)
            && !is_shielding(module_accessor)
        {
            EffectModule::set_visible_kind(
                module_accessor,
                Hash40::new("sys_shield"),
                false,
            );
            EffectModule::kill_kind(
                module_accessor,
                Hash40::new("sys_shield"),
                false,
                true,
            );
            for i in 0..8 {
                if AttackModule::is_attack(module_accessor, i, false) {
                    let attack_data = *AttackModule::attack_data(module_accessor, i, false);
                    let is_capsule =
                        attack_data.x2 != 0.0 || attack_data.y2 != 0.0 || attack_data.z2 != 0.0;
                    let mut x2 = None;
                    let mut y2 = None;
                    let mut z2 = None;
                    if is_capsule {
                        x2 = Some(attack_data.x2);
                        y2 = Some(attack_data.y2);
                        z2 = Some(attack_data.z2);
                    }
                    generate_hitbox_effects(
                        module_accessor,
                        attack_data.node, // joint
                        attack_data.size,
                        attack_data.x,
                        attack_data.y,
                        attack_data.z,
                        x2,
                        y2,
                        z2,
                        ID_COLORS[(i % 8) as usize],
                    );
                }
            }
        }
    }
}

// Necessary to ensure we visualize on the first frame of the hitbox
#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn handle_attack(lua_state: u64) {
    if !is_training_mode() {
        return;
    }

    let mut l2c_agent = L2CAgent::new(lua_state);

    // get all necessary grabbox params
    let id = l2c_agent.pop_lua_stack(1); // int
    let joint = l2c_agent.pop_lua_stack(3); // hash40
    let _damage = l2c_agent.pop_lua_stack(4); // float
    let _angle = l2c_agent.pop_lua_stack(5); // int
    let _kbg = l2c_agent.pop_lua_stack(6); // int
    let _fkb = l2c_agent.pop_lua_stack(7); // int
    let _bkb = l2c_agent.pop_lua_stack(8); // int
    let size = l2c_agent.pop_lua_stack(9); // float
    let x = l2c_agent.pop_lua_stack(10); // float
    let y = l2c_agent.pop_lua_stack(11); // float
    let z = l2c_agent.pop_lua_stack(12); // float
    let x2 = l2c_agent.pop_lua_stack(13); // float or void
    let y2 = l2c_agent.pop_lua_stack(14); // float or void
    let z2 = l2c_agent.pop_lua_stack(15); // float or void

    original!()(lua_state);

    if MENU.hitbox_vis {
        generate_hitbox_effects(
            sv_system::battle_object_module_accessor(lua_state),
            joint.get_int(),
            size.get_num(),
            x.get_num(),
            y.get_num(),
            z.get_num(),
            x2.try_get_num(),
            y2.try_get_num(),
            z2.try_get_num(),
            ID_COLORS[(id.get_int() % 8) as usize],
        );
    }
}

#[skyline::hook(replace = sv_animcmd::CATCH)]
unsafe fn handle_catch(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);

    // get all necessary grabbox params
    let id = l2c_agent.pop_lua_stack(1); // int
    let joint = l2c_agent.pop_lua_stack(2); // hash40
    let size = l2c_agent.pop_lua_stack(3); // float
    let x = l2c_agent.pop_lua_stack(4); // float
    let y = l2c_agent.pop_lua_stack(5); // float
    let z = l2c_agent.pop_lua_stack(6); // float
    let x2 = l2c_agent.pop_lua_stack(7); // float or void
    let y2 = l2c_agent.pop_lua_stack(8); // float or void
    let z2 = l2c_agent.pop_lua_stack(9); // float or void

    original!()(lua_state);

    if MENU.hitbox_vis && is_training_mode() {
        generate_hitbox_effects(
            sv_system::battle_object_module_accessor(lua_state),
            joint.get_int(),
            size.get_num(),
            x.get_num(),
            y.get_num(),
            z.get_num(),
            x2.try_get_num(),
            y2.try_get_num(),
            z2.try_get_num(),
            ID_COLORS[(id.get_int() + 3 % 8) as usize],
        );
    }
}

pub unsafe fn is_shielding(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    (*FIGHTER_STATUS_KIND_GUARD_ON..=*FIGHTER_STATUS_KIND_GUARD_DAMAGE).contains(&status_kind)
}

#[skyline::hook(replace = GrabModule::set_rebound)]
pub unsafe fn handle_set_rebound(
    module_accessor: *mut app::BattleObjectModuleAccessor,
    rebound: bool,
) {
    if !is_training_mode() {
        return original!()(module_accessor, rebound);
    }

    if rebound == true {
        return original!()(module_accessor, rebound);
    }
    
    // only if we're not shielding
    if is_shielding(module_accessor) {
        return original!()(module_accessor, rebound);
    }
    
    EffectModule::set_visible_kind(
        module_accessor,
        Hash40::new("sys_shield"),
        false,
    );
    EffectModule::kill_kind(
        module_accessor,
        Hash40::new("sys_shield"),
        false,
        true,
    );

    original!()(module_accessor, rebound);
}

pub fn hitbox_visualization() {
    println!("[Training Modpack] Applying hitbox visualization mods.");
    skyline::install_hooks!(handle_attack, handle_catch, handle_set_rebound);
}
