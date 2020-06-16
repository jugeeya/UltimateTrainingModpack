use crate::common::FIGHTER_MANAGER_ADDR;
use crate::hitbox_visualizer;
use skyline::nn::ro::LookupSymbol;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

pub mod directional_influence;
pub mod shield;
pub mod tech;

pub mod combo;
mod frame_counter;
mod ledge;
mod left_stick;
mod mash;
mod save_states;

#[skyline::hook(replace = WorkModule::get_param_float)]
pub unsafe fn handle_get_param_float(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    param_type: u64,
    param_hash: u64,
) -> f32 {
    shield::get_param_float(module_accessor, param_type, param_hash)
        .unwrap_or_else(|| original!()(module_accessor, param_type, param_hash))
}

#[skyline::hook(replace = ControlModule::get_attack_air_kind)]
pub unsafe fn handle_get_attack_air_kind(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> i32 {
    // bool replace;
    // int kind = InputRecorder::get_attack_air_kind(module_accessor, replace);
    // if (replace) return kind;

    mash::get_attack_air_kind(module_accessor).unwrap_or_else(|| original!()(module_accessor))
}

#[skyline::hook(replace = ControlModule::get_command_flag_cat)]
pub unsafe fn handle_get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) -> i32 {
    save_states::save_states(module_accessor);

    let mut flag = original!()(module_accessor, category);

    frame_counter::get_command_flag_cat(module_accessor, category);
    combo::get_command_flag_cat(module_accessor, category);

    // bool replace;
    // int ret = InputRecorder::get_command_flag_cat(module_accessor, category, flag, replace);
    // if (replace) return ret;

    shield::get_command_flag_cat(module_accessor);
    mash::get_command_flag_cat(module_accessor, category, &mut flag);
    ledge::get_command_flag_cat(module_accessor, category, &mut flag);
    tech::get_command_flag_cat(module_accessor, category, &mut flag);
    hitbox_visualizer::get_command_flag_cat(module_accessor, category);

    flag
}

/**
 * This is called to get the stick position when
 * shielding (shield tilt)
 * 1 is fully right, -1 is fully left
 */
#[skyline::hook(replace = ControlModule::get_stick_x_no_clamp)]
pub unsafe fn get_stick_x_no_clamp(module_accessor: &mut app::BattleObjectModuleAccessor) -> f32 {
    left_stick::mod_get_stick_x(module_accessor).unwrap_or_else(|| original!()(module_accessor))
}
/**
 * This is called to get the stick position when
 * shielding (shield tilt)
 * 1 is fully up, -1 is fully down
 */
#[skyline::hook(replace = ControlModule::get_stick_y_no_clamp)]
pub unsafe fn get_stick_y_no_clamp(module_accessor: &mut app::BattleObjectModuleAccessor) -> f32 {
    left_stick::mod_get_stick_y(module_accessor).unwrap_or_else(|| original!()(module_accessor))
}

/**
 * Called when:
 * Walking in the facing direction
 * Air Dodging
 */
#[skyline::hook(replace = ControlModule::get_stick_x)]
pub unsafe fn get_stick_x(module_accessor: &mut app::BattleObjectModuleAccessor) -> f32 {
    left_stick::mod_get_stick_x(module_accessor).unwrap_or_else(|| original!()(module_accessor))
}

/**
 *
 */
#[skyline::hook(replace = ControlModule::get_stick_y)]
pub unsafe fn get_stick_y(module_accessor: &mut app::BattleObjectModuleAccessor) -> f32 {
    left_stick::mod_get_stick_y(module_accessor).unwrap_or_else(|| original!()(module_accessor))
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

// float get_attack_air_stick_x_replace(u64 module_accessor) {
//     u64 control_module = load_module(module_accessor, 0x48);
//     float (*get_attack_air_stick_x)(u64) = (float (*)(u64)) load_module_impl(control_module, 0x188);
//     float stick_y = get_attack_air_stick_x(control_module);

//     bool replace;
//     float ret = InputRecorder::get_attack_air_stick_x(module_accessor, replace);
//     if (replace) return ret;

//     return stick_y;
// }

#[skyline::hook(replace = ControlModule::check_button_on)]
pub unsafe fn handle_check_button_on(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> bool {
    shield::check_button_on(module_accessor, button).unwrap_or_else(|| {
        mash::check_button_on(module_accessor, button).unwrap_or_else(|| {
            tech::check_button_on(module_accessor, button).unwrap_or_else(|| {
                ledge::check_button_on(module_accessor, button)
                    .unwrap_or_else(|| original!()(module_accessor, button))
            })
        })
    })
}

#[skyline::hook(replace = ControlModule::check_button_off)]
pub unsafe fn handle_check_button_off(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> bool {
    shield::check_button_off(module_accessor, button)
        .unwrap_or_else(|| original!()(module_accessor, button))
}

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
    let motion_kind = tech::change_motion(module_accessor, motion_kind).unwrap_or(motion_kind);

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
}

pub fn training_mods() {
    println!("[Training Modpack] Applying training mods.");
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
        );
    }

    skyline::install_hooks!(
        // Mash airdodge/jump
        handle_get_command_flag_cat,
        // Hold/Infinite shield
        handle_check_button_on,
        handle_check_button_off,
        handle_get_param_float,
        // Mash attack
        handle_get_attack_air_kind,
        // Tech options
        handle_change_motion,
        // Directional AirDodge,
        get_stick_x,
        get_stick_y,
    );

    // // Input recorder
    // SaltySD_function_replace_sym(
    //     "_ZN3app8lua_bind31ControlModule__get_stick_x_implEPNS_26BattleObjectModuleAccessorE",
    //     (u64)&ControlModule::get_stick_x_replace);
    // SaltySD_function_replace_sym(
    //     "_ZN3app8lua_bind31ControlModule__get_attack_air_stick_x_implEPNS_26BattleObjectModuleAccessorE",
    //     (u64)&ControlModule::get_attack_air_stick_x_replace);
}
