use crate::common::{is_training_mode, FIGHTER_MANAGER_ADDR, STAGE_MANAGER_ADDR};
use crate::hitbox_visualizer;
use skyline::nn::ro::LookupSymbol;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use smash::params::*;

pub mod combo;
pub mod directional_influence;
pub mod sdi;
pub mod shield;
pub mod tech;

mod air_dodge_direction;
mod character_specific;
mod fast_fall;
mod frame_counter;
mod full_hop;
mod ledge;
mod mash;
mod reset;
mod save_states;
mod shield_tilt;

#[skyline::hook(replace = WorkModule::get_param_float)]
pub unsafe fn handle_get_param_float(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    param_type: u64,
    param_hash: u64,
) -> f32 {
    let ori = original!()(module_accessor, param_type, param_hash);
    if !is_training_mode() {
        return ori;
    }

    shield::get_param_float(module_accessor, param_type, param_hash).unwrap_or(ori)
}

#[skyline::hook(replace = WorkModule::get_param_int)]
pub unsafe fn handle_get_param_int(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    param_type: u64,
    param_hash: u64,
) -> i32 {
    let ori = original!()(module_accessor, param_type, param_hash);

    if !is_training_mode() {
        return ori;
    }

    save_states::get_param_int(module_accessor, param_type, param_hash).unwrap_or(ori)
}

#[skyline::hook(replace = ControlModule::get_attack_air_kind)]
pub unsafe fn handle_get_attack_air_kind(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> i32 {
    let ori = original!()(module_accessor);
    if !is_training_mode() {
        return ori;
    }

    // bool replace;
    // int kind = InputRecorder::get_attack_air_kind(module_accessor, replace);
    // if (replace) return kind;

    mash::get_attack_air_kind(module_accessor).unwrap_or(ori)
}

#[skyline::hook(replace = ControlModule::get_command_flag_cat)]
pub unsafe fn handle_get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) -> i32 {
    let mut flag = original!()(module_accessor, category);

    shield::param_installer();

    if !is_training_mode() {
        return flag;
    }

    // bool replace;
    // int ret = InputRecorder::get_command_flag_cat(module_accessor, category, flag, replace);
    // if (replace) return ret;

    flag |= mash::get_command_flag_cat(module_accessor, category);

    once_per_frame_per_fighter(module_accessor, category);

    flag
}

fn once_per_frame_per_fighter(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) {
    if category != FIGHTER_PAD_COMMAND_CATEGORY1 {
        return;
    }

    unsafe {
        combo::get_command_flag_cat(module_accessor);
        hitbox_visualizer::get_command_flag_cat(module_accessor);
        save_states::save_states(module_accessor);
        tech::get_command_flag_cat(module_accessor);
    }

    fast_fall::get_command_flag_cat(module_accessor);
    frame_counter::get_command_flag_cat(module_accessor);
    ledge::get_command_flag_cat(module_accessor);
    shield::get_command_flag_cat(module_accessor);

    reset::check_reset(module_accessor);
}

/**
 * This is called to get the stick position when
 * shielding (shield tilt)
 * 1 is fully right, -1 is fully left
 */
#[skyline::hook(replace = ControlModule::get_stick_x_no_clamp)]
pub unsafe fn get_stick_x_no_clamp(module_accessor: &mut app::BattleObjectModuleAccessor) -> f32 {
    let ori = original!()(module_accessor);
    if !is_training_mode() {
        return ori;
    }

    shield_tilt::mod_get_stick_x(module_accessor).unwrap_or(ori)
}

/**
 * This is called to get the stick position when
 * shielding (shield tilt)
 * 1 is fully up, -1 is fully down
 */
#[skyline::hook(replace = ControlModule::get_stick_y_no_clamp)]
pub unsafe fn get_stick_y_no_clamp(module_accessor: &mut app::BattleObjectModuleAccessor) -> f32 {
    let ori = original!()(module_accessor);
    if !is_training_mode() {
        return ori;
    }

    shield_tilt::mod_get_stick_y(module_accessor).unwrap_or(ori)
}

/**
 * Called when:
 * Walking in the facing direction
 * Air Dodging
 */
#[skyline::hook(replace = ControlModule::get_stick_x)]
pub unsafe fn get_stick_x(module_accessor: &mut app::BattleObjectModuleAccessor) -> f32 {
    let ori = original!()(module_accessor);
    if !is_training_mode() {
        return ori;
    }

    air_dodge_direction::mod_get_stick_x(module_accessor).unwrap_or(ori)
}

/**
 *
 */
#[skyline::hook(replace = ControlModule::get_stick_y)]
pub unsafe fn get_stick_y(module_accessor: &mut app::BattleObjectModuleAccessor) -> f32 {
    let ori = original!()(module_accessor);
    if !is_training_mode() {
        return ori;
    }

    air_dodge_direction::mod_get_stick_y(module_accessor).unwrap_or(ori)
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
    let ori = original!()(module_accessor, button);
    if !is_training_mode() {
        return ori;
    }

    shield::check_button_on(module_accessor, button)
        .unwrap_or_else(|| full_hop::check_button_on(module_accessor, button).unwrap_or(ori))
}

#[skyline::hook(replace = ControlModule::check_button_off)]
pub unsafe fn handle_check_button_off(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> bool {
    let ori = original!()(module_accessor, button);
    if !is_training_mode() {
        return ori;
    }

    shield::check_button_off(module_accessor, button)
        .unwrap_or_else(|| full_hop::check_button_off(module_accessor, button).unwrap_or(ori))
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
    let mut mod_motion_kind = motion_kind;

    if is_training_mode() {
        mod_motion_kind = tech::change_motion(module_accessor, motion_kind).unwrap_or(motion_kind);
    }

    original!()(
        module_accessor,
        mod_motion_kind,
        unk1,
        unk2,
        unk3,
        unk4,
        unk5,
        unk6,
    )
}

#[skyline::hook(replace = WorkModule::is_enable_transition_term)]
pub unsafe fn handle_is_enable_transition_term(
    module_accessor: *mut app::BattleObjectModuleAccessor,
    transition_term: i32,
) -> bool {
    let ori = original!()(module_accessor, transition_term);

    if !is_training_mode() {
        return ori;
    }

    combo::is_enable_transition_term(module_accessor, transition_term, ori);

    ori
}

extern "C" {
    #[link_name = "\u{1}_ZN3app15sv_fighter_util15set_dead_rumbleEP9lua_State"]
    pub fn set_dead_rumble(lua_state: u64) -> u64;
}

#[skyline::hook(replace = set_dead_rumble)]
pub unsafe fn handle_set_dead_rumble(lua_state: u64) -> u64 {
    if is_training_mode() {
        return 0;
    }

    original!()(lua_state)
}

pub static mut COMMON_PARAMS: *mut CommonParams = 0 as *mut _;

// 8.1.0 Offset
static mut LOAD_PRC_FILE_OFFSET: usize = 0x34369d0;

static LOAD_PRC_FILE_SEARCH_CODE: &[u8] = &[
    0xff, 0xc3, 0x01, 0xd1, 0xff, 0xdf, 0x02, 0xa9, 0xf6, 0x57, 0x04, 0xa9, 0xf4, 0x4f, 0x05, 0xa9,
    0xfd, 0x7b, 0x06, 0xa9, 0xfd, 0x83, 0x01, 0x91, 0xf6, 0x5f, 0x00, 0x32, 0x88, 0x79, 0x00, 0xb0,
    0x08, 0xed, 0x0f, 0x91, 0xf3, 0x03, 0x00, 0xaa, 0xe8, 0x07, 0x00, 0xf9, 0xe8, 0x03, 0x00, 0x91,
    0x3f, 0x00, 0x16, 0x6b,
];

#[skyline::hook(offset = LOAD_PRC_FILE_OFFSET)]
unsafe fn load_once_common_params(common_obj: u64, table1_idx: u32) {
    let loaded_tables = smash::resource::LoadedTables::get_instance();
    let hash = loaded_tables.get_hash_from_t1_index(table1_idx).as_u64();

    if hash == smash::phx::Hash40::new("fighter/common/param/common.prc").hash {
        COMMON_PARAMS = CommonParams::from_u64_mut(common_obj).unwrap() as *mut _;
    }
    original!()(common_obj, table1_idx)
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
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

        LookupSymbol(
            &mut STAGE_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app12StageManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
        );

        use skyline::hooks::{getRegionAddress, Region};

        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, LOAD_PRC_FILE_SEARCH_CODE) {
            LOAD_PRC_FILE_OFFSET = offset;
        } else {
            println!("Error: no offset found for function 'load_prc_file'. Defaulting to 8.1.0 offset. This likely won't work.");
        }
    }

    skyline::install_hooks!(
        // Mash airdodge/jump
        handle_get_command_flag_cat,
        // Hold/Infinite shield
        handle_check_button_on,
        handle_check_button_off,
        handle_get_param_float,
        // Save states
        handle_get_param_int,
        handle_set_dead_rumble,
        // Mash attack
        handle_get_attack_air_kind,
        // Tech options
        handle_change_motion,
        // Directional AirDodge,
        get_stick_x,
        get_stick_y,
        // Shield Tilt
        get_stick_x_no_clamp,
        get_stick_y_no_clamp,
        // Combo
        handle_is_enable_transition_term,
        // SDI
        crate::training::sdi::check_hit_stop_delay_command,
        // Generic params
        load_once_common_params
    );

    combo::init();
    shield::init();
    fast_fall::init();
    mash::init();
    ledge::init();

    // // Input recorder
    // SaltySD_function_replace_sym(
    //     "_ZN3app8lua_bind31ControlModule__get_stick_x_implEPNS_26BattleObjectModuleAccessorE",
    //     (u64)&ControlModule::get_stick_x_replace);
    // SaltySD_function_replace_sym(
    //     "_ZN3app8lua_bind31ControlModule__get_attack_air_stick_x_implEPNS_26BattleObjectModuleAccessorE",
    //     (u64)&ControlModule::get_attack_air_stick_x_replace);
}
