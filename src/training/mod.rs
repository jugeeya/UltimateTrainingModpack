use smash::app::{self, sv_system, sv_animcmd, lua_bind::*, FighterManager};
use smash::lib::{self, L2CAgent, L2CValue, lua_const::*};
use smash::phx::{Hash40, Vector3f};
use smash::hash40;
use skyline::{c_str, nn::ro::LookupSymbol, logging::hex_dump_ptr};
use crate::common::fighter_manager_addr;
use crate::common::*;
use crate::common::consts::*;
use crate::hitbox_visualizer;

mod DirectionalInfluence;
mod Shield;
mod Tech;
mod Mash;
mod Ledge;

#[allow(unused_unsafe)]
#[skyline::hook(replace = WorkModule::get_float)]
pub unsafe fn handle_get_float(module_accessor: &mut app::BattleObjectModuleAccessor, var: i32) -> f32 {
    DirectionalInfluence::get_float(module_accessor, var).unwrap_or_else( || {
        original!()(module_accessor, var)
    })
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = WorkModule::get_param_float)]
pub unsafe fn handle_get_param_float(module_accessor: &mut app::BattleObjectModuleAccessor, param_type: u64, param_hash: u64) -> f32 {
    Shield::get_param_float(module_accessor, param_type, param_hash).unwrap_or_else( || {
        original!()(module_accessor, param_type, param_hash)
    })
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = ControlModule::get_attack_air_kind)]
pub unsafe fn handle_get_attack_air_kind(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    // bool replace;
    // int kind = InputRecorder::get_attack_air_kind(module_accessor, replace);
    // if (replace) return kind;

    Mash::get_attack_air_kind(module_accessor).unwrap_or_else( || {
        original!()(module_accessor)
    })
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = ControlModule::get_command_flag_cat)]
pub unsafe fn handle_get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32) -> i32 
{
    //save_states(module_accessor);

    // apply only once per frame
    if category == 0 && is_training_mode() && menu.HITBOX_VIS {
        // Pause Effect AnimCMD if hitbox visualization is active
        let status_kind = StatusModule::status_kind(module_accessor) as i32;
        MotionAnimcmdModule::set_sleep_effect(module_accessor,
            !((*FIGHTER_STATUS_KIND_CATCH..=*FIGHTER_STATUS_KIND_TREAD_FALL).contains(&status_kind) ||
            (*FIGHTER_STATUS_KIND_WAIT..=*FIGHTER_STATUS_KIND_REBOUND_JUMP).contains(&status_kind)));

        if !(*FIGHTER_STATUS_KIND_CATCH..=*FIGHTER_STATUS_KIND_CATCH_TURN).contains(&status_kind) {
            EffectModule::set_visible_kind(module_accessor, Hash40{hash: hash40("sys_shield")}, false);
            EffectModule::kill_kind(module_accessor, Hash40{hash: hash40("sys_shield")}, false, true);
            for i in 0..8 {
                if AttackModule::is_attack(module_accessor, i, false) {
                    let attack_data = *AttackModule::attack_data(module_accessor, i, false);
                    let is_capsule = attack_data.x2 != 0.0 || attack_data.y2 != 0.0 || attack_data.z2 != 0.0;
                    let mut x2 = None;
                    let mut y2 = None;
                    let mut z2 = None;
                    if is_capsule {
                        x2 = Some(attack_data.x2);
                        y2 = Some(attack_data.y2);
                        z2 = Some(attack_data.z2);
                    }
                    hitbox_visualizer::generate_hitbox_effects(
                        module_accessor, 
                        attack_data.node_, // joint 
                        attack_data.size_, 
                        attack_data.x, attack_data.y, attack_data.z, 
                        x2, y2, z2, 
                        hitbox_visualizer::ID_COLORS[(i % 8) as usize]);
                }
            }
        }
    }

    let mut flag = original!()(module_accessor, category);

    // bool replace;
    // int ret = InputRecorder::get_command_flag_cat(module_accessor, category, flag, replace);
    // if (replace) return ret;

    Mash::get_command_flag_cat(module_accessor, category, &mut flag);
    Ledge::get_command_flag_cat(module_accessor, category, &mut flag);
    Tech::get_command_flag_cat(module_accessor, category, &mut flag);

    flag
}

// int get_pad_flag(u64 module_accessor) {
//     u64 control_module = load_module(module_accessor, 0x48);
//     int (*get_pad_flag)(u64) = (int (*)(u64)) load_module_impl(control_module, 0x348);
//     int pad_flag = get_pad_flag(control_module);

//     bool replace;
//     int ret = InputRecorder::get_pad_flag(module_accessor, replace);
//     if (replace) return ret;

//     return pad_flag;
// }

// float get_stick_x_replace(u64 module_accessor) {
//     u64 control_module = load_module(module_accessor, 0x48);
//     float (*get_stick_x)(u64) = (float (*)(u64)) load_module_impl(control_module, 0x178);
//     float stick_x = get_stick_x(control_module);

//     bool replace;
//     float ret = InputRecorder::get_stick_x(module_accessor, replace);
//     if (replace) return ret;

//     return stick_x;
// }

// float get_stick_y_replace(u64 module_accessor) {
//     u64 control_module = load_module(module_accessor, 0x48);
//     float (*get_stick_y)(u64) = (float (*)(u64)) load_module_impl(control_module, 0x188);
//     float stick_y = get_stick_y(control_module);

//     bool replace;
//     float ret = InputRecorder::get_stick_y(module_accessor, replace);
//     if (replace) return ret;

//     return stick_y;
// }

#[allow(unused_unsafe)]
#[skyline::hook(replace = ControlModule::check_button_on)]
pub unsafe fn handle_check_button_on(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32) -> bool
{
    Shield::check_button_on(module_accessor, button).unwrap_or_else( || {
        Mash::check_button_on(module_accessor, button).unwrap_or_else( || {
            Tech::check_button_on(module_accessor, button).unwrap_or_else( || {
                Ledge::check_button_on(module_accessor, button).unwrap_or_else( || {
                    original!()(module_accessor, button)
                })
            })
        })
    })
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = ControlModule::check_button_off)]
pub unsafe fn handle_check_button_off(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32) -> bool
{
    Shield::check_button_off(module_accessor, button).unwrap_or_else( || {
        original!()(module_accessor, button)
    })
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = StatusModule::init_settings)]
pub unsafe fn handle_init_settings(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    situationKind: i32, 
    unk1: i32, 
    unk2: u32, 
    groundCliffCheckKind: i32,
    unk3: bool, 
    unk4: i32, 
    unk5: i32, 
    unk6: i32, 
    unk7: i32)
{
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    Tech::init_settings(module_accessor, status_kind).unwrap_or_else( || {
        original!()(module_accessor, situationKind, unk1, unk2, groundCliffCheckKind, unk3, unk4, unk5, unk6, unk7)
    })
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = MotionModule::change_motion)]
pub unsafe fn handle_change_motion(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    motion_kind: u64, 
    unk1: f32, 
    unk2: f32, 
    unk3: bool, 
    unk4: f32, 
    unk5: bool, 
    unk6: bool) -> u64
{
    Tech::change_motion(module_accessor, motion_kind).unwrap_or_else( || {
        original!()(module_accessor, motion_kind, unk1, unk2, unk3, unk4, unk5, unk6)
    })
}


pub fn training_mods() {
    println!("Applying training mods.");
    unsafe {
        LookupSymbol(&mut fighter_manager_addr, c_str("_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"));
        println!("Lookup symbol output: {:#?}", fighter_manager_addr);
    }

    // Mash airdodge/jump
    skyline::install_hook!(handle_get_command_flag_cat);

    // Set DI
    skyline::install_hook!(handle_get_float);

    // Hold/Infinite shield
    skyline::install_hook!(handle_check_button_on);
    skyline::install_hook!(handle_check_button_off);

    skyline::install_hook!(handle_get_param_float);

    // Mash attack
    skyline::install_hook!(handle_get_attack_air_kind);

    // // Input recorder
    // SaltySD_function_replace_sym(
    //     "_ZN3app8lua_bind31ControlModule__get_stick_x_implEPNS_26BattleObjectModuleAccessorE",
    //     (u64)&ControlModule::get_stick_x_replace);
    // SaltySD_function_replace_sym(
    //     "_ZN3app8lua_bind31ControlModule__get_stick_y_implEPNS_26BattleObjectModuleAccessorE",
    //     (u64)&ControlModule::get_stick_y_replace);

    // Tech options
    skyline::install_hook!(handle_init_settings);
    skyline::install_hook!(handle_change_motion);
}
