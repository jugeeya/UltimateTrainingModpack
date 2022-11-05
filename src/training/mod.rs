use crate::common::{
    is_training_mode, menu, FIGHTER_MANAGER_ADDR, ITEM_MANAGER_ADDR, STAGE_MANAGER_ADDR,
};
use crate::hitbox_visualizer;
use crate::training::character_specific::items;
use skyline::hooks::{getRegionAddress, InlineCtx, Region};
use skyline::nn::hid::*;
use skyline::nn::ro::LookupSymbol;
use smash::app::{self, enSEType, lua_bind::*};
use smash::lib::lua_const::*;
use smash::params::*;
use smash::phx::{Hash40, Vector3f};

pub mod buff;
pub mod charge;
pub mod clatter;
pub mod combo;
pub mod crouch;
pub mod directional_influence;
pub mod frame_counter;
pub mod ledge;
pub mod sdi;
pub mod shield;
pub mod tech;
pub mod throw;

mod air_dodge_direction;
mod attack_angle;
mod character_specific;
mod input_recording;
mod fast_fall;
mod full_hop;
pub(crate) mod input_delay;
mod input_record;
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

    if input_record::is_playback() {
        return ori;
    }
    mash::get_attack_air_kind(module_accessor).unwrap_or(ori)
}

#[skyline::hook(replace = ControlModule::get_command_flag_cat)]
pub unsafe fn handle_get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) -> i32 {
    let mut flag = original!()(module_accessor, category);

    if category == FIGHTER_PAD_COMMAND_CATEGORY1 {
        shield::param_installer();
    }

    if !is_training_mode() {
        return flag;
    }

    flag |= mash::get_command_flag_cat(module_accessor, category);
    // Get throw directions
    flag |= throw::get_command_flag_throw_direction(module_accessor);

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
        if menu::menu_condition(module_accessor) {
            menu::spawn_menu();
        }

        input_record::get_command_flag_cat(module_accessor);
        combo::get_command_flag_cat(module_accessor);
        hitbox_visualizer::get_command_flag_cat(module_accessor);
        save_states::save_states(module_accessor);
        tech::get_command_flag_cat(module_accessor);
        clatter::handle_clatter(module_accessor);
    }

    fast_fall::get_command_flag_cat(module_accessor);
    frame_counter::get_command_flag_cat(module_accessor);
    ledge::get_command_flag_cat(module_accessor);
    shield::get_command_flag_cat(module_accessor);
    directional_influence::get_command_flag_cat(module_accessor);
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
 * Called when:
 * angled ftilt/fsmash
 */
#[skyline::hook(replace = ControlModule::get_stick_dir)]
pub unsafe fn get_stick_dir(module_accessor: &mut app::BattleObjectModuleAccessor) -> f32 {
    let ori = original!()(module_accessor);
    if !is_training_mode() {
        return ori;
    }

    let situation_kind = StatusModule::situation_kind(module_accessor);
    if situation_kind == *SITUATION_KIND_CLIFF {
        return ori;
    }

    if input_record::is_playback() {
        return ori;
    }

    attack_angle::mod_get_stick_dir(module_accessor).unwrap_or(ori)
}

/**
 * Called when:
 * Directional airdodge
 * Crouching
 */
#[skyline::hook(replace = ControlModule::get_stick_y)]
pub unsafe fn get_stick_y(module_accessor: &mut app::BattleObjectModuleAccessor) -> f32 {
    let ori = original!()(module_accessor);
    if !is_training_mode() {
        return ori;
    }

    air_dodge_direction::mod_get_stick_y(module_accessor)
        .unwrap_or_else(|| crouch::mod_get_stick_y(module_accessor).unwrap_or(ori))
}

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
    let mod_motion_kind = if is_training_mode() {
        tech::change_motion(module_accessor, motion_kind).unwrap_or(motion_kind)
    } else {
        motion_kind
    };

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
    module_accessor: &mut app::BattleObjectModuleAccessor,
    transition_term: i32,
) -> bool {
    let ori = original!()(module_accessor, transition_term);

    if !is_training_mode() {
        return ori;
    }

    combo::is_enable_transition_term(module_accessor, transition_term, ori);
    match ledge::is_enable_transition_term(module_accessor, transition_term) {
        Some(r) => r,
        None => ori,
    }
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

#[skyline::hook(replace = CameraModule::req_quake)]
pub unsafe fn handle_req_quake(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    my_int: i32,
) -> u64 {
    if !is_training_mode() {
        return original!()(module_accessor, my_int);
    }
    if save_states::is_killing() {
        return original!()(module_accessor, *CAMERA_QUAKE_KIND_NONE);
    }
    original!()(module_accessor, my_int)
}

pub static mut COMMON_PARAMS: *mut CommonParams = 0 as *mut _;

fn params_main(params_info: &ParamsInfo<'_>) {
    if let Ok(common) = params_info.get::<CommonParams>() {
        unsafe {
            COMMON_PARAMS = common as *mut _;
        }
    }
}

static CLOUD_ADD_LIMIT_OFFSET: usize = 0x008dc140; // this function is used to add limit to Cloud's limit gauge. Hooking it here so we can call it in buff.rs
#[skyline::hook(offset = CLOUD_ADD_LIMIT_OFFSET)]
pub unsafe fn handle_add_limit(
    add_limit: f32,
    module_accessor: &mut app::BattleObjectModuleAccessor,
    is_special_lw: u64,
) {
    original!()(add_limit, module_accessor, is_special_lw)
}

#[skyline::hook(replace = EffectModule::req_screen)] // hooked to prevent the screen from darkening when loading a save state with One-Winged Angel
pub unsafe fn handle_req_screen(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    my_hash: Hash40,
    bool_1: bool,
    bool_2: bool,
    bool_3: bool,
) -> u64 {
    if !is_training_mode() {
        return original!()(module_accessor, my_hash, bool_1, bool_2, bool_3);
    }
    let new_hash = my_hash.hash;
    if new_hash == 72422354958 && buff::is_buffing(module_accessor) {
        // Wing bg hash
        let replace_hash = Hash40::new("bg");
        return original!()(module_accessor, replace_hash, bool_1, bool_2, bool_3);
    }
    original!()(module_accessor, my_hash, bool_1, bool_2, bool_3)
}

#[skyline::hook(replace = app::FighterSpecializer_Jack::check_doyle_summon_dispatch)] // returns status of summon dispatch if triggered, -1 as u64 otherwise
pub unsafe fn handle_check_doyle_summon_dispatch(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    bool_1: bool,
    bool_2: bool,
) -> u64 {
    let ori = original!()(module_accessor, bool_1, bool_2);
    if !is_training_mode() {
        return ori;
    }
    if ori == *FIGHTER_JACK_STATUS_KIND_SUMMON as u64 {
        if buff::is_buffing(module_accessor) {
            return 4294967295;
        }
    }
    ori
}

// Set Stale Moves to On
static STALE_OFFSET: usize = 0x013e88a4;
// One instruction after stale moves toggle register is set to 0
#[skyline::hook(offset=STALE_OFFSET, inline)]
unsafe fn stale_handle(ctx: &mut InlineCtx) {
    let x22 = ctx.registers[22].x.as_mut();
    let training_structure_address = (*x22 + 0xb60) as *mut u8;
    *training_structure_address = 1;
}

// Set Stale Moves to On in the menu text
static STALE_MENU_OFFSET: usize = 0x013e88a0;
// One instruction after menu text register is set to off
#[skyline::hook(offset=STALE_MENU_OFFSET, inline)]
unsafe fn stale_menu_handle(ctx: &mut InlineCtx) {
    // Set the text pointer to where "mel_training_on" is located
    let on_text_ptr = ((getRegionAddress(Region::Text) as u64) + (0x42b215e)) as u64;
    let x1 = ctx.registers[1].x.as_mut();
    *x1 = on_text_ptr;
}

#[skyline::hook(replace = SoundModule::play_se)] // hooked to prevent death sfx from playing when loading save states
pub unsafe fn handle_se(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    my_hash: Hash40,
    bool1: bool,
    bool2: bool,
    bool3: bool,
    bool4: bool,
    se_type: enSEType,
) -> u64 {
    // Make effects silent while we're killing fighters. Stops death explosion and fighter misfoot.
    if save_states::is_killing() {
        let silent_hash = Hash40::new("se_silent");
        return original!()(
            module_accessor,
            silent_hash,
            bool1,
            bool2,
            bool3,
            bool4,
            se_type,
        );
    }
    original!()(
        module_accessor,
        my_hash,
        bool1,
        bool2,
        bool3,
        bool4,
        se_type,
    )
}

#[skyline::hook(replace = EffectModule::req)] // hooked to prevent death gfx from playing when loading save states
pub unsafe fn handle_effect(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    eff_hash: Hash40,
    pos: *const Vector3f,
    rot: *const Vector3f,
    size: f32,
    arg6: u32,
    arg7: i32,
    arg8: bool,
    arg9: i32,
) -> u64 {
    if save_states::is_killing() {
        // Making the size 0 prevents these effects from being displayed. Fixs throw explosions, ICs squall, etc.
        return original!()(
            module_accessor,
            eff_hash,
            pos,
            rot,
            0.0,
            arg6,
            arg7,
            arg8,
            arg9,
        );
    }
    original!()(
        module_accessor,
        eff_hash,
        pos,
        rot,
        size,
        arg6,
        arg7,
        arg8,
        arg9,
    )
}

static CAN_FUTTOBI_BACK_OFFSET: usize = 0x0260f950; // can_futtobi_back, checks if stage allows for star KOs
#[skyline::hook(offset = CAN_FUTTOBI_BACK_OFFSET)]
pub unsafe fn handle_star_ko(my_long_ptr: &mut u64) -> bool {
    let ori = original!()(my_long_ptr);
    if !is_training_mode() {
        return ori;
    } else {
        return false;
    }
}

// TODO: Delete hook when neutral getup sorted
#[skyline::hook(replace = WorkModule::count_down_int)]
pub unsafe fn handle_cd_int(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    int_to_cd: i32,
    dummy: i32,
) -> u64 {
    let ori = original!()(module_accessor,int_to_cd,dummy);

    if int_to_cd == *FIGHTER_STATUS_CLIFF_WORK_INT_WAIT_FRAME {
        println!("CDing! Cliff Wait Frame: {}", WorkModule::get_int(module_accessor,*FIGHTER_STATUS_CLIFF_WORK_INT_WAIT_FRAME));
    }
    
    ori
}

#[allow(improper_ctypes)]
extern "C" {
    fn add_nn_hid_hook(callback: fn(*mut NpadGcState, *const u32));
}

pub fn training_mods() {
    println!("[Training Modpack] Applying training mods.");

    // Input Recording/Delay
    unsafe {
        if (add_nn_hid_hook as *const ()).is_null() {
            panic!("The NN-HID hook plugin could not be found and is required to add NRO hooks. Make sure libnn_hid_hook.nro is installed.");
        }
        add_nn_hid_hook(input_delay::handle_get_npad_state);
        add_nn_hid_hook(menu::handle_get_npad_state);
    }

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

        LookupSymbol(
            &mut ITEM_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app11ItemManagerEE9instance_E\0"
                .as_bytes()
                .as_ptr(),
        );

        smash::params::add_hook(params_main).unwrap();
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
        handle_req_quake,
        // Mash attack
        handle_get_attack_air_kind,
        // Attack angle
        get_stick_dir,
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
        // Buffs
        handle_add_limit,
        handle_check_doyle_summon_dispatch,
        handle_req_screen,
        // Stale Moves
        stale_handle,
        stale_menu_handle,
        // Death SFX
        handle_se,
        // Death GFX
        handle_effect,
        // Star KO turn off
        handle_star_ko,

        //debug
        handle_cd_int,
    );

    combo::init();
    shield::init();
    fast_fall::init();
    mash::init();
    ledge::init();
    throw::init();
    menu::init();
    buff::init();
    items::init();
    input_record::init();
}
