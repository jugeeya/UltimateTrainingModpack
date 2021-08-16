use crate::common::{consts::*, *};
use smash::app::{self, lua_bind::*, sv_animcmd, sv_system};
use smash::lib::{lua_const::*, L2CAgent, L2CValue};
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
    center: Vector3f,
    capsule_center: Option<Vector3f>,
    color: Vector3f,
) {
    let size_mult = 19.0 / 200.0;

    let (x, y, z) = (center.x, center.y, center.z);

    let x_dist: f32;
    let y_dist: f32;
    let z_dist: f32;
    let mut n_effects: i32;
    if let Some(capsule_center) = capsule_center {
        let (x2, y2, z2) = (capsule_center.x, capsule_center.y, capsule_center.z);
        x_dist = x2 - x;
        y_dist = y2 - y;
        z_dist = z2 - z;
        let dist_sq: f32 = x_dist * x_dist + y_dist * y_dist + z_dist * z_dist;
        let dist = dist_sq.sqrt();
        n_effects = ((dist / (size * 1.75)) + 1.0).ceil() as i32; // just enough effects to form a continuous line
        if n_effects < 2 {
            n_effects = 2;
        } else if n_effects > MAX_EFFECTS_PER_HITBOX {
            n_effects = MAX_EFFECTS_PER_HITBOX;
        }
    } else {
        x_dist = 0.0;
        y_dist = 0.0;
        z_dist = 0.0;
        n_effects = 1;
    }

    for i in 0..n_effects {
        let t = if n_effects > 1 {
            (i as f32) / ((n_effects - 1) as f32)
        } else {
            0.0
        };
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

        if false {
            // is_fighter(module_accessor) {
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

pub unsafe fn get_command_flag_cat(module_accessor: &mut app::BattleObjectModuleAccessor) {
    // Resume Effect AnimCMD incase we don't display hitboxes
    MotionAnimcmdModule::set_sleep_effect(module_accessor, false);

    if MENU.hitbox_vis == OnOff::Off {
        return;
    }

    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    if (*FIGHTER_STATUS_KIND_CATCH..=*FIGHTER_STATUS_KIND_CATCH_TURN).contains(&status_kind) {
        return;
    }

    if is_shielding(module_accessor) {
        return;
    }

    // Pause Effect AnimCMD if hitbox visualization is active
    // Keep effects on for missed tech effect
    MotionAnimcmdModule::set_sleep_effect(module_accessor, status_kind != FIGHTER_STATUS_KIND_DOWN);

    EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_shield"), false);
    EffectModule::kill_kind(module_accessor, Hash40::new("sys_shield"), false, true);
    for i in 0..8 {
        if !AttackModule::is_attack(module_accessor, i, false) {
            continue;
        }

        let attack_data = *AttackModule::attack_data(module_accessor, i, false);
        let center = Vector3f {
            x: attack_data.x,
            y: attack_data.y,
            z: attack_data.z,
        };
        let is_capsule = attack_data.x2 != 0.0 || attack_data.y2 != 0.0 || attack_data.z2 != 0.0;
        let capsule_center = if is_capsule {
            Some(Vector3f {
                x: attack_data.x2,
                y: attack_data.y2,
                z: attack_data.z2,
            })
        } else {
            None
        };
        generate_hitbox_effects(
            module_accessor,
            attack_data.node, // joint
            attack_data.size,
            center,
            capsule_center,
            ID_COLORS[(i % 8) as usize],
        );
    }
}

// Necessary to ensure we visualize on the first frame of the hitbox
#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn handle_attack(lua_state: u64) {
    if is_training_mode() {
        mod_handle_attack(lua_state);
    }

    original!()(lua_state);
}

unsafe fn mod_handle_attack(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);

    // necessary if param object fails
    // hacky way of forcing no shield damage on all hitboxes
    if MENU.shield_state == Shield::Infinite {
        let mut hitbox_params: Vec<L2CValue> =
            (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        l2c_agent.clear_lua_stack();
        for (i, mut x) in hitbox_params.iter_mut().enumerate().take(36) {
            if i == 20 {
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(-999.0));
            } else {
                l2c_agent.push_lua_stack(&mut x);
            }
        }
    }

    // Hitbox Visualization
    if MENU.hitbox_vis == OnOff::On {
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

        let center = Vector3f {
            x: x.get_num(),
            y: y.get_num(),
            z: z.get_num(),
        };
        let capsule_center = if let (Some(x2), Some(y2), Some(z2)) =
            (x2.try_get_num(), y2.try_get_num(), z2.try_get_num())
        {
            Some(Vector3f {
                x: x2,
                y: y2,
                z: z2,
            })
        } else {
            None
        };

        generate_hitbox_effects(
            sv_system::battle_object_module_accessor(lua_state),
            joint.get_int(),
            size.get_num(),
            center,
            capsule_center,
            ID_COLORS[(id.get_int() % 8) as usize],
        );
    }
}

#[skyline::hook(replace = sv_animcmd::CATCH)]
unsafe fn handle_catch(lua_state: u64) {
    if is_training_mode() {
        mod_handle_catch(lua_state);
    }

    original!()(lua_state);
}

unsafe fn mod_handle_catch(lua_state: u64) {
    if MENU.hitbox_vis == OnOff::Off {
        return;
    }

    // get all necessary grabbox params
    let mut l2c_agent = L2CAgent::new(lua_state);
    let id = l2c_agent.pop_lua_stack(1); // int
    let joint = l2c_agent.pop_lua_stack(2); // hash40
    let size = l2c_agent.pop_lua_stack(3); // float
    let x = l2c_agent.pop_lua_stack(4); // float
    let y = l2c_agent.pop_lua_stack(5); // float
    let z = l2c_agent.pop_lua_stack(6); // float
    let x2 = l2c_agent.pop_lua_stack(7); // float or void
    let y2 = l2c_agent.pop_lua_stack(8); // float or void
    let z2 = l2c_agent.pop_lua_stack(9); // float or void

    let center = Vector3f {
        x: x.get_num(),
        y: y.get_num(),
        z: z.get_num(),
    };
    let capsule_center = if let (Some(x2), Some(y2), Some(z2)) =
        (x2.try_get_num(), y2.try_get_num(), z2.try_get_num())
    {
        Some(Vector3f {
            x: x2,
            y: y2,
            z: z2,
        })
    } else {
        None
    };

    generate_hitbox_effects(
        sv_system::battle_object_module_accessor(lua_state),
        joint.get_int(),
        size.get_num(),
        center,
        capsule_center,
        ID_COLORS[(id.get_int() + 3 % 8) as usize],
    );
}

#[skyline::hook(replace = GrabModule::set_rebound)]
pub unsafe fn handle_set_rebound(
    module_accessor: *mut app::BattleObjectModuleAccessor,
    rebound: bool,
) {
    if is_training_mode() {
        mod_handle_handle_set_rebound(module_accessor, rebound);
    }

    original!()(module_accessor, rebound);
}

unsafe fn mod_handle_handle_set_rebound(
    module_accessor: *mut app::BattleObjectModuleAccessor,
    rebound: bool,
) {
    if rebound {
        return;
    }

    // only if we're not shielding
    if is_shielding(module_accessor) {
        return;
    }

    EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_shield"), false);
    EffectModule::kill_kind(module_accessor, Hash40::new("sys_shield"), false, true);
}

pub fn hitbox_visualization() {
    println!("[Training Modpack] Applying hitbox visualization mods.");
    skyline::install_hooks!(handle_attack, handle_catch, handle_set_rebound);
}
