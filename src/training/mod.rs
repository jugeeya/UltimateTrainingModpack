use crate::common::button_config;
use crate::common::{
    consts::BuffOption, consts::FighterId, consts::MENU, dev_config, get_module_accessor,
    is_operation_cpu, is_training_mode, menu, FIGHTER_MANAGER_ADDR, ITEM_MANAGER_ADDR,
    STAGE_MANAGER_ADDR,
};
use crate::hitbox_visualizer;
use crate::input::*;
use crate::logging::*;
use crate::training::character_specific::{items, kirby, pikmin, ptrainer};
use skyline::hooks::{getRegionAddress, InlineCtx, Region};
use skyline::nn::ro::LookupSymbol;
use smash::app::{self, enSEType, lua_bind::*, utility};
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
pub mod ui;

mod air_dodge_direction;
mod attack_angle;
pub mod character_specific;
mod fast_fall;
mod full_hop;
pub mod input_delay;
mod input_log;
mod input_record;
mod mash;
mod reset;
pub mod save_states;
mod shield_tilt;

#[cfg(debug_assertions)]
mod debug;

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

    // this must be run even outside of training mode
    // because otherwise it won't reset the shield_damage_mul
    // back to "normal" once you leave training mode.
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
        if menu::menu_condition() {
            menu::spawn_menu();
        }

        if is_operation_cpu(module_accessor) {
            // Handle dodge staling here b/c input recording or mash can cause dodging
            WorkModule::set_flag(
                module_accessor,
                !(MENU.stale_dodges.as_bool()),
                *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_PENALTY,
            );
        }

        input_record::get_command_flag_cat(module_accessor);
        combo::get_command_flag_cat(module_accessor);
        hitbox_visualizer::get_command_flag_cat(module_accessor);
        save_states::save_states(module_accessor);
        tech::get_command_flag_cat(module_accessor);
        clatter::handle_clatter(module_accessor);
    }

    fast_fall::get_command_flag_cat(module_accessor);
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

    let ori = original!()(
        module_accessor,
        mod_motion_kind,
        unk1,
        unk2,
        unk3,
        unk4,
        unk5,
        unk6,
    );
    // After we've changed motion, speed up if necessary
    if is_training_mode() {
        ptrainer::change_motion(module_accessor, motion_kind);
    }
    ori
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

static CLOUD_ADD_LIMIT_OFFSET: usize = 0x008dc140;

// this function is used to add limit to Cloud's limit gauge. Hooking it here so we can call it in buff.rs
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
    if ori == *FIGHTER_JACK_STATUS_KIND_SUMMON as u64 && buff::is_buffing(module_accessor) {
        return 4294967295;
    }
    ori
}

#[skyline::hook(offset = 0x03ff9a0)]
pub unsafe fn handle_add_damage(
    damage_module: *mut u64, // DamageModule
    mut damage_to_add: f32,
    param_2: i32,
) -> u64 {
    if !is_training_mode() {
        return original!()(damage_module, damage_to_add, param_2);
    }
    let module_accessor =
        &mut **(damage_module.byte_add(0x8) as *mut *mut app::BattleObjectModuleAccessor);
    // Prevent Wii Fit Deep Breathing from Healing on Save State Load
    if utility::get_kind(module_accessor) == *FIGHTER_KIND_WIIFIT
        && buff::is_buffing(module_accessor)
    {
        damage_to_add = 0.0;
    }
    original!()(damage_module, damage_to_add, param_2)
}

// Set Stale Moves to On
static STALE_OFFSET: usize = 0x013e88a4;

// One instruction after stale moves toggle register is set to 0
#[skyline::hook(offset = STALE_OFFSET, inline)]
unsafe fn stale_handle(ctx: &mut InlineCtx) {
    let x22 = ctx.registers[22].x.as_mut();
    let training_structure_address = (*x22 + 0xb60) as *mut u8;
    *training_structure_address = 1;
}

// Set Stale Moves to On in the menu text
static STALE_MENU_OFFSET: usize = 0x013e88a0;

// One instruction after menu text register is set to off
#[skyline::hook(offset = STALE_MENU_OFFSET, inline)]
unsafe fn stale_menu_handle(ctx: &mut InlineCtx) {
    // Set the text pointer to where "mel_training_on" is located
    let on_text_ptr = (getRegionAddress(Region::Text) as u64) + 0x42b215e;
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
    if !is_training_mode() {
        return original!()(
            module_accessor,
            my_hash,
            bool1,
            bool2,
            bool3,
            bool4,
            se_type,
        );
    }
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

#[repr(C)]
pub struct FighterSoundModule {
    vtable: u64,
    owner: *mut app::BattleObjectModuleAccessor,
}

static PLAY_SE_OFFSET: usize = 0x04cf6a0;
// fighters don't use the symbol and go straight through their vtable to this function
#[skyline::hook(offset = PLAY_SE_OFFSET)]
pub unsafe fn handle_fighter_play_se(
    sound_module: *mut FighterSoundModule, // pointer to fighter's SoundModule
    mut my_hash: Hash40,
    bool1: bool,
    bool2: bool,
    bool3: bool,
    bool4: bool,
    se_type: enSEType,
) -> u64 {
    if !is_training_mode() {
        return original!()(sound_module, my_hash, bool1, bool2, bool3, bool4, se_type);
    }
    // Supress Buff Sound Effects while buffing
    if buff::is_buffing_any() {
        my_hash = Hash40::new("se_silent");
    }
    // Supress Kirby Copy Ability SFX when loading Save State
    if my_hash.hash == 0x1453dd86e4 || my_hash.hash == 0x14bdd3e7c8 {
        let module_accessor = (*sound_module).owner;
        if StatusModule::status_kind(module_accessor) != FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_DRINK {
            my_hash = Hash40::new("se_silent");
        }
    }
    my_hash = ptrainer::handle_pokemon_sound_effect(my_hash);
    original!()(sound_module, my_hash, bool1, bool2, bool3, bool4, se_type)
}

pub struct FighterEffectModule {
    _vtable: u64,
    owner: *mut app::BattleObjectModuleAccessor,
}

static FOLLOW_REQ_OFFSET: usize = 0x044f860;
#[skyline::hook(offset = FOLLOW_REQ_OFFSET)] // hooked to prevent score gfx from playing when loading save states
pub unsafe fn handle_effect_follow(
    effect_module: &mut FighterEffectModule,
    eff_hash: Hash40,
    joint_hash: Hash40,
    pos: *const Vector3f,
    rot: *const Vector3f,
    mut size: f32,
    arg5: bool,
    arg6: u32,
    arg7: i32,
    arg8: i32,
    arg9: i32,
    arg10: i32,
    arg11: bool,
    arg12: bool,
) -> u64 {
    if !is_training_mode() {
        return original!()(
            effect_module,
            eff_hash,
            joint_hash,
            pos,
            rot,
            size,
            arg5,
            arg6,
            arg7,
            arg8,
            arg9,
            arg10,
            arg11,
            arg12,
        );
    }
    if utility::get_kind(&mut *(effect_module.owner)) == *FIGHTER_KIND_GAOGAEN {
        // Debug: Print info so I can populate Kirby's effects
        println!("let eff_hash = Hash40{{hash:0x{:x}}};", eff_hash.hash);
        println!("let joint_hash = Hash40{{hash:0x{:x}}};", joint_hash.hash);
        println!("let pos = Vector3f {{");
        println!("	x: {:.},", (*pos).x);
        println!("	y: {:.},", (*pos).y);
        println!("	z: {:.},", (*pos).z);
        println!("}};");
        println!("let rot = Vector3f {{");
        println!("	x: {:.},", (*rot).x);
        println!("	y: {:.},", (*rot).y);
        println!("	z: {:.},", (*rot).z);
        println!("}};");
        println!("let efh = EffectModule::req_follow(");
        println!("	module_accessor,");
        println!("	charge_hash,");
        println!("	joint_hash,");
        println!("	&pos,");
        println!("	&rot,");
        println!("	{:.},", size);
        println!("	{},", arg5);
        println!("	{},", arg6);
        println!("	{},", arg7);
        println!("	{},", arg8);
        println!("	{},", arg9);
        println!("	{},", arg10);
        println!("	{},", arg11);
        println!("	{},", arg12);
        println!(");");
        // End Debug
    }
    // Prevent the score GFX from playing on the CPU when loading save state during hitstop
    if eff_hash == Hash40::new("sys_score_aura") && save_states::is_loading() {
        size = 0.0
    }
    original!()(
        effect_module,
        eff_hash,
        joint_hash,
        pos,
        rot,
        size,
        arg5,
        arg6,
        arg7,
        arg8,
        arg9,
        arg10,
        arg11,
        arg12,
    )
}

static EFFECT_REQ_OFFSET: usize = 0x44de50;
#[skyline::hook(offset = EFFECT_REQ_OFFSET)] // hooked to prevent death gfx from playing when loading save states
pub unsafe fn handle_fighter_effect(
    effect_module: *mut FighterEffectModule, // pointer to effect module
    eff_hash: Hash40,
    pos: *const Vector3f,
    rot: *const Vector3f,
    mut size: f32,
    arg6: u32,
    arg7: i32,
    arg8: bool,
    arg9: i32,
) -> u64 {
    if !is_training_mode() {
        return original!()(
            effect_module,
            eff_hash,
            pos,
            rot,
            size,
            arg6,
            arg7,
            arg8,
            arg9,
        );
    }
    size = ptrainer::handle_pokemon_effect(&mut *(*effect_module).owner, eff_hash, size);
    original!()(
        effect_module,
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

static JOINT_EFFECT_REQ_OFFSET: usize = 0x44e1e0;
#[skyline::hook(offset = JOINT_EFFECT_REQ_OFFSET)] // hooked to prevent death gfx from playing when loading save states
pub unsafe fn handle_fighter_joint_effect(
    effect_module: *mut FighterEffectModule, // pointer to effect module
    eff_hash: Hash40,
    joint_hash: Hash40,
    pos: *const Vector3f,
    rot: *const Vector3f,
    mut size: f32,
    pos2: *const Vector3f, //unk, maybe displacement and not pos/rot
    rot2: *const Vector3f, //unk, ^
    arg5: bool,
    arg6: u32,
    arg7: i32,
    arg9: i32,
) -> u64 {
    if !is_training_mode() {
        return original!()(
            effect_module,
            eff_hash,
            joint_hash,
            pos,
            rot,
            size,
            pos2,
            rot2,
            arg5,
            arg6,
            arg7,
            arg9,
        );
    }
    size = ptrainer::handle_pokemon_effect(&mut *(*effect_module).owner, eff_hash, size);
    original!()(
        effect_module,
        eff_hash,
        joint_hash,
        pos,
        rot,
        size,
        pos2,
        rot2,
        arg5,
        arg6,
        arg7,
        arg9,
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
    if !is_training_mode() {
        return original!()(
            module_accessor,
            eff_hash,
            pos,
            rot,
            size,
            arg6,
            arg7,
            arg8,
            arg9,
        );
    }
    if save_states::is_loading() && !buff::is_buffing(module_accessor) {
        // Making the size 0 prevents these effects from being displayed. Fixes throw explosions, ICs squall, etc.
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

static CAN_FUTTOBI_BACK_OFFSET: usize = 0x0260f950;

// can_futtobi_back, checks if stage allows for star KOs
#[skyline::hook(offset = CAN_FUTTOBI_BACK_OFFSET)]
pub unsafe fn handle_star_ko(my_long_ptr: &mut u64) -> bool {
    let ori = original!()(my_long_ptr);
    if !is_training_mode() {
        ori
    } else {
        false
    }
}

static REUSED_UI_OFFSET: usize = 0x068cd80;
// A function reused by many functions to update UI. Called to update at least Little Mac's meter.
#[skyline::hook(offset = REUSED_UI_OFFSET)]
pub unsafe fn handle_reused_ui(
    fighter_data: *mut u32, // a pointer to length 4 data in the Fighter's FighterEntry in the FighterManager
    mut param_2: u32,       // In Little Mac's case, the meter value as an integer
) {
    if !is_training_mode() {
        original!()(fighter_data, param_2);
    }

    if save_states::is_loading() {
        let player_module_accessor = &mut *get_module_accessor(FighterId::Player);
        let cpu_module_accessor = &mut *get_module_accessor(FighterId::CPU);
        let player_fighter_kind = utility::get_kind(player_module_accessor);
        let cpu_fighter_kind = utility::get_kind(cpu_module_accessor);
        // If Little Mac is in the game and we're buffing him, set the meter to 100
        if (player_fighter_kind == *FIGHTER_KIND_LITTLEMAC
            || cpu_fighter_kind == *FIGHTER_KIND_LITTLEMAC)
            && MENU.buff_state.to_vec().contains(&BuffOption::KO)
        {
            param_2 = 100;
        }
    }

    original!()(fighter_data, param_2)
}

static ARTICLE_GET_INT_OFFSET: usize = 0x3d5920;

#[skyline::hook(offset = ARTICLE_GET_INT_OFFSET)]
pub unsafe fn handle_article_get_int(
    article_module: *mut app::BattleObjectModuleAccessor, // *mut ArticleModule
    generate_article: i32,
    address: i32,
) -> i32 {
    original!()(article_module, generate_article, address)
}

// Instruction run on the completion of the CPU Control function
static OPCF_OFFSET: usize = 0x06b7fdc;

// One instruction after the CPU Control function completes
#[skyline::hook(offset = OPCF_OFFSET, inline)]
unsafe fn handle_once_per_cpu_frame(_ctx: &mut InlineCtx) {
    frame_counter::tick_ingame();
    tech::hide_tech();
    // Tick notifications
    let queue = &mut ui::notifications::QUEUE;
    let notification = queue.first();
    if notification.is_some() {
        let notification = queue.first_mut().unwrap();
        notification.tick();
    }
}

static FIM_OFFSET: usize = 0x17504a0;
// TODO: Should we define all of our offsets in one file? Should at least be a good start for changing to be based on ASM instructions
#[skyline::hook(offset = FIM_OFFSET)]
unsafe fn handle_final_input_mapping(
    mappings: *mut ControllerMapping,
    player_idx: i32, // Is this the player index, or plugged in controller index? Need to check, assuming player for now - is this 0 indexed or 1?
    out: *mut MappedInputs,
    controller_struct: &mut SomeControllerStruct,
    arg: bool,
) {
    // Order of hooks here REALLY matters. Tread lightly

    // Go through the original mapping function first
    original!()(mappings, player_idx, out, controller_struct, arg);
    if !is_training_mode() {
        return;
    }

    // Check if we should apply hot reload configs
    // Non-mutable pull
    dev_config::handle_final_input_mapping(player_idx, controller_struct);

    // Grab menu inputs from player
    // MUTATES controller state to kill inputs when in or closing menu
    menu::handle_final_input_mapping(player_idx, controller_struct, out);

    // Grab button input requests from player
    // MUTATES controller state to kill start presses for menu
    button_config::handle_final_input_mapping(player_idx, controller_struct);

    // Potentially apply input delay
    // MUTATES controller state to delay inputs
    input_delay::handle_final_input_mapping(player_idx, out);

    // Read potentially delayed state for loggers
    input_log::handle_final_input_mapping(player_idx, controller_struct, out);

    // Potentially apply input recording, thus with delay
    // MUTATES controller state to apply recording or playback
    input_record::handle_final_input_mapping(player_idx, out);
}

pub fn training_mods() {
    info!("Applying training mods.");

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

        add_hook(params_main).unwrap();
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
        handle_reused_ui,
        handle_req_screen,
        handle_add_damage,
        // Buff SFX
        handle_fighter_play_se,
        // Stale Moves
        stale_handle,
        stale_menu_handle,
        // Death SFX
        handle_se,
        // Death GFX
        handle_effect,
        handle_effect_follow,
        // Star KO turn off
        handle_star_ko,
        // Clatter
        clatter::hook_start_clatter,
        // Notifications
        handle_once_per_cpu_frame,
        // Input
        handle_final_input_mapping,
        // Charge
        handle_article_get_int,
        handle_fighter_effect,
        handle_fighter_joint_effect,
    );

    items::init();
    input_record::init();
    ui::init();
    pikmin::init();
    ptrainer::init();
    kirby::init();
    tech::init();

    #[cfg(debug_assertions)]
    debug::init();
}
