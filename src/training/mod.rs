use crate::common::fighter_manager_addr;
use crate::hitbox_visualizer;
use skyline::{c_str, nn::ro::LookupSymbol};
use smash::app::{self, lua_bind::*};

mod DirectionalInfluence;
mod Ledge;
mod Mash;
mod SaveStates;
mod Shield;
mod Tech;

#[allow(unused_unsafe)]
#[skyline::hook(replace = WorkModule::get_float)]
pub unsafe fn handle_get_float(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    var: i32,
) -> f32 {
    DirectionalInfluence::get_float(module_accessor, var)
        .unwrap_or_else(|| original!()(module_accessor, var))
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = WorkModule::get_param_float)]
pub unsafe fn handle_get_param_float(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    param_type: u64,
    param_hash: u64,
) -> f32 {
    Shield::get_param_float(module_accessor, param_type, param_hash)
        .unwrap_or_else(|| original!()(module_accessor, param_type, param_hash))
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = ControlModule::get_attack_air_kind)]
pub unsafe fn handle_get_attack_air_kind(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> i32 {
    // bool replace;
    // int kind = InputRecorder::get_attack_air_kind(module_accessor, replace);
    // if (replace) return kind;

    Mash::get_attack_air_kind(module_accessor).unwrap_or_else(|| original!()(module_accessor))
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = ControlModule::get_command_flag_cat)]
pub unsafe fn handle_get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) -> i32 {
    SaveStates::save_states(module_accessor);

    let mut flag = original!()(module_accessor, category);

    // bool replace;
    // int ret = InputRecorder::get_command_flag_cat(module_accessor, category, flag, replace);
    // if (replace) return ret;

    Mash::get_command_flag_cat(module_accessor, category, &mut flag);
    Ledge::get_command_flag_cat(module_accessor, category, &mut flag);
    Tech::get_command_flag_cat(module_accessor, category, &mut flag);
    hitbox_visualizer::get_command_flag_cat(module_accessor, category);

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
    button: i32,
) -> bool {
    Shield::check_button_on(module_accessor, button).unwrap_or_else(|| {
        Mash::check_button_on(module_accessor, button).unwrap_or_else(|| {
            Tech::check_button_on(module_accessor, button).unwrap_or_else(|| {
                Ledge::check_button_on(module_accessor, button)
                    .unwrap_or_else(|| original!()(module_accessor, button))
            })
        })
    })
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = ControlModule::check_button_off)]
pub unsafe fn handle_check_button_off(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> bool {
    Shield::check_button_off(module_accessor, button)
        .unwrap_or_else(|| original!()(module_accessor, button))
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
    unk7: i32,
) {
    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    Tech::init_settings(module_accessor, status_kind).unwrap_or_else(|| {
        original!()(
            module_accessor,
            situationKind,
            unk1,
            unk2,
            groundCliffCheckKind,
            unk3,
            unk4,
            unk5,
            unk6,
            unk7,
        )
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
    unk6: bool,
) -> u64 {
    Tech::change_motion(module_accessor, motion_kind).unwrap_or_else(|| {
        original!()(
            module_accessor,
            motion_kind,
            unk1,
            unk2,
            unk3,
            unk4,
            unk5,
            unk6,
        )
    })
}

pub fn training_mods() {
    println!("Applying training mods.");
    unsafe {
        LookupSymbol(
            &mut fighter_manager_addr,
            c_str("_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"),
        );
        println!("Lookup symbol output: {:#?}", fighter_manager_addr);
    }

    skyline::install_hooks!(
        // Mash airdodge/jump
        handle_get_command_flag_cat,

        // Set DI
        handle_get_float,

        // Hold/Infinite shield
        handle_check_button_on,
        handle_check_button_off,
    
        handle_get_param_float,
    
        // Mash attack
        handle_get_attack_air_kind,
    
        // Tech options
        handle_init_settings,
        handle_change_motion,
    );

    // // Input recorder
    // SaltySD_function_replace_sym(
    //     "_ZN3app8lua_bind31ControlModule__get_stick_x_implEPNS_26BattleObjectModuleAccessorE",
    //     (u64)&ControlModule::get_stick_x_replace);
    // SaltySD_function_replace_sym(
    //     "_ZN3app8lua_bind31ControlModule__get_stick_y_implEPNS_26BattleObjectModuleAccessorE",
    //     (u64)&ControlModule::get_stick_y_replace);
}
