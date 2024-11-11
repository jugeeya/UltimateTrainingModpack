use smash::app;
use smash::app::lua_bind::*;
use smash::app::sv_system;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

use crate::common::consts::*;
use crate::common::*;
use training_mod_sync::*;
use crate::training::{frame_counter, input_record, mash, save_states};

// TODO!() We only reset this on save state load or LRA reset
// How many hits to hold shield until picking an Out Of Shield option
static MULTI_HIT_OFFSET: RwLock<u32> = RwLock::new(0);

// The current set delay
static SHIELD_DELAY: RwLock<u32> = RwLock::new(0);

// Used to only decrease once per shieldstun change
static WAS_IN_SHIELDSTUN: RwLock<bool> = RwLock::new(false);

// For how many frames should the shield hold be overwritten
static SUSPEND_SHIELD: RwLock<bool> = RwLock::new(false);

// Toggle for shield decay
static SHIELD_DECAY: RwLock<bool> = RwLock::new(false);

/// This is the cached shield damage multiplier.
/// Vanilla is 1.19, but mods can change this.
static CACHED_SHIELD_DAMAGE_MUL: RwLock<Option<f32>> = RwLock::new(None);

static REACTION_COUNTER_INDEX: LazyLock<usize> =
    LazyLock::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGame));

fn set_shield_decay(value: bool) {
    assign_rwlock(&SHIELD_DECAY, value);
}

fn should_pause_shield_decay() -> bool {
    !read_rwlock(&SHIELD_DECAY)
}

fn reset_oos_offset() {
    unsafe {
        /*
         * Need to offset by 1, since we decrease as soon as shield gets hit
         * but only check later if we can OOS
         */
        assign_rwlock(
            &MULTI_HIT_OFFSET,
            MENU.oos_offset.get_random().into_delay() + 1,
        );
    }
}

fn handle_oos_offset(module_accessor: &mut app::BattleObjectModuleAccessor) {
    // Check if we are currently in shield stun
    let mut was_in_shieldstun_guard = lock_write_rwlock(&WAS_IN_SHIELDSTUN);
    if !is_in_shieldstun(module_accessor) {
        // Make sure we don't forget and wait until we get hit on shield
        *was_in_shieldstun_guard = false;
        return;
    }

    // Make sure we just freshly entered shield stun
    if *was_in_shieldstun_guard {
        return;
    }

    // Roll shield delay
    unsafe {
        assign_rwlock(&SHIELD_DELAY, MENU.reaction_time.get_random().into_delay());
    }

    // Decrease offset once if needed
    let mut multi_hit_offset_guard = lock_write_rwlock(&MULTI_HIT_OFFSET);
    *multi_hit_offset_guard = (*multi_hit_offset_guard).saturating_sub(1);

    // Mark that we were in shield stun, so we don't decrease again
    *was_in_shieldstun_guard = true;
}

pub fn allow_oos() -> bool {
    // Delay OOS until offset hits 0
    read_rwlock(&MULTI_HIT_OFFSET) == 0
}

pub fn get_command_flag_cat(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) {
        return;
    }

    // Reset oos offset when standing
    if is_idle(module_accessor) || is_in_hitstun(module_accessor) {
        reset_oos_offset();
    }

    // Reset when not shielding
    unsafe {
        let status_kind = StatusModule::status_kind(module_accessor);
        if status_kind != FIGHTER_STATUS_KIND_GUARD {
            set_shield_decay(false);
        }
    }
}

pub unsafe fn get_param_float(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    param_type: u64,
    param_hash: u64,
) -> Option<f32> {
    if !is_operation_cpu(module_accessor) || input_record::is_playback() {
        // shield normally during playback
        return None;
    }

    if MENU.shield_state != Shield::NONE {
        handle_oos_offset(module_accessor);
    }

    handle_shield_decay(param_type, param_hash)
}

// Shield Decay//Recovery
fn handle_shield_decay(param_type: u64, param_hash: u64) -> Option<f32> {
    let menu_state;
    unsafe {
        menu_state = MENU.shield_state;
    }

    if menu_state != Shield::INFINITE
        && menu_state != Shield::CONSTANT
        && !should_pause_shield_decay()
    {
        return None;
    }

    if param_type != hash40("common") {
        return None;
    }

    if param_hash == hash40("shield_dec1") {
        return Some(0.0);
    }

    if param_hash == hash40("shield_recovery1") {
        return Some(999.0);
    }

    None
}

/// sets/resets the shield_damage_mul within
/// the game's internal structure.
///
/// `common_params` is effectively a mutable reference
/// to the game's own internal data structure for params.
pub unsafe fn param_installer() {
    if crate::training::COMMON_PARAMS as usize != 0 {
        let common_params = &mut *crate::training::COMMON_PARAMS;
        let mut cached_shield_damage_mul_guard = lock_write_rwlock(&CACHED_SHIELD_DAMAGE_MUL);
        // cache the original shield damage multiplier once
        if (*cached_shield_damage_mul_guard).is_none() {
            *cached_shield_damage_mul_guard = Some(common_params.shield_damage_mul);
        }

        if is_training_mode() && (MENU.shield_state == Shield::INFINITE) {
            // if you are in training mode and have infinite shield enabled,
            // set the game's shield_damage_mul to 0.0
            common_params.shield_damage_mul = 0.0;
        } else {
            // reset the game's shield_damage_mul back to what
            // it originally was at game boot.
            common_params.shield_damage_mul = (*cached_shield_damage_mul_guard).unwrap();
        }
    }
}

pub fn should_hold_shield(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    // Don't let shield override input recording playback
    unsafe {
        if input_record::is_playback() || input_record::is_standby() {
            return false;
        }
    }
    // Mash shield
    if mash::request_shield(module_accessor) {
        return true;
    }

    let shield_state;
    unsafe {
        shield_state = &MENU.shield_state;
    }

    // We should hold shield if the state requires it
    if unsafe { save_states::is_loading() }
        || ![Shield::HOLD, Shield::INFINITE, Shield::CONSTANT].contains(shield_state)
    {
        return false;
    }

    true
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_guard_cont)]
pub unsafe fn handle_sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ori = original!()(fighter);

    if !is_training_mode() {
        return ori;
    }

    mod_handle_sub_guard_cont(fighter);
    ori
}

unsafe fn mod_handle_sub_guard_cont(fighter: &mut L2CFighterCommon) {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if !is_operation_cpu(module_accessor) {
        return;
    }

    if !was_in_shieldstun(module_accessor) {
        return;
    }

    // Enable shield decay
    if MENU.shield_state == Shield::HOLD {
        set_shield_decay(true);
    }

    // Check for OOS delay
    if !allow_oos() {
        return;
    }

    if !is_shielding(module_accessor) {
        frame_counter::full_reset(*REACTION_COUNTER_INDEX);
        return;
    }

    if frame_counter::should_delay(read_rwlock(&SHIELD_DELAY), *REACTION_COUNTER_INDEX) {
        return;
    }

    if is_in_parry(module_accessor) {
        return;
    }

    if MENU.mash_triggers.contains(&MashTrigger::SHIELDSTUN) {
        if MENU.shieldstun_override == Action::empty() {
            mash::external_buffer_menu_mash(MENU.mash_state.get_random())
        } else {
            mash::external_buffer_menu_mash(MENU.shieldstun_override.get_random())
        }
    }

    let action = mash::get_current_buffer();

    if handle_escape_option(fighter, module_accessor) {
        return;
    }

    if needs_oos_handling_drop_shield() {
        return;
    }

    // Set shield suspension
    suspend_shield(action);
}

/**
 * This is needed to have the CPU put up shield
 */
pub unsafe fn check_button_on(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> Option<bool> {
    if should_return_none_in_check_button(module_accessor, button) {
        return None;
    }
    Some(true)
}

/**
 * This is needed to prevent dropping shield immediately
 */
pub unsafe fn check_button_off(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> Option<bool> {
    if should_return_none_in_check_button(module_accessor, button)
        || needs_oos_handling_drop_shield()
    {
        return None;
    }
    Some(false)
}

/**
 * Roll/Dodge doesn't work oos the normal way
 */
unsafe fn handle_escape_option(
    fighter: &mut L2CFighterCommon,
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> bool {
    if !WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
    ) {
        return false;
    }

    match mash::get_current_buffer() {
        Action::SPOT_DODGE => {
            fighter
                .fighter_base
                .change_status(FIGHTER_STATUS_KIND_ESCAPE.as_lua_int(), LUA_TRUE);
            true
        }
        Action::ROLL_F => {
            fighter
                .fighter_base
                .change_status(FIGHTER_STATUS_KIND_ESCAPE_F.as_lua_int(), LUA_TRUE);
            true
        }
        Action::ROLL_B => {
            fighter
                .fighter_base
                .change_status(FIGHTER_STATUS_KIND_ESCAPE_B.as_lua_int(), LUA_TRUE);
            true
        }
        _ => false,
    }
}

/**
 * Needed to allow these attacks to work OOS
 */
fn needs_oos_handling_drop_shield() -> bool {
    let action = mash::get_current_buffer();

    if action == Action::JUMP {
        return true;
    }

    if is_aerial(action) {
        return true;
    }

    if action == Action::UP_B {
        return true;
    }

    if action == Action::U_SMASH {
        return true;
    }
    // Make sure we only flicker shield when Airdodge and Shield mash options are selected
    if action == Action::AIR_DODGE {
        let shield_state;
        unsafe {
            shield_state = &MENU.shield_state;
        }
        // If we're supposed to be holding shield, let airdodge make us drop shield
        if [Shield::HOLD, Shield::INFINITE, Shield::CONSTANT].contains(shield_state) {
            suspend_shield(Action::AIR_DODGE);
        }
        return true;
    }

    // Make sure we only flicker shield when Airdodge and Shield mash options are selected
    if action == Action::AIR_DODGE {
        let shield_state;
        unsafe {
            shield_state = &MENU.shield_state;
        }
        // If we're supposed to be holding shield, let airdodge make us drop shield
        if [Shield::HOLD, Shield::INFINITE, Shield::CONSTANT].contains(shield_state) {
            suspend_shield(Action::AIR_DODGE);
        }
        return true;
    }

    if action == Action::SHIELD {
        let shield_state;
        unsafe {
            shield_state = &MENU.shield_state;
        }
        // Don't drop shield on shield hit if we're supposed to be holding shield
        if [Shield::HOLD, Shield::INFINITE, Shield::CONSTANT].contains(shield_state) {
            return false;
        }
        return true;
    }
    false
}

pub fn is_aerial(action: Action) -> bool {
    matches!(
        action,
        Action::NAIR | Action::FAIR | Action::BAIR | Action::UAIR | Action::DAIR
    )
}

// Needed for shield drop options
pub fn suspend_shield(action: Action) {
    assign_rwlock(&SUSPEND_SHIELD, need_suspend_shield(action));
}

fn need_suspend_shield(action: Action) -> bool {
    if action == Action::empty() {
        return false;
    }

    match action {
        Action::U_SMASH => false,
        Action::GRAB => false,
        Action::SHIELD => false,
        _ => {
            // Force shield drop
            true
        }
    }
}

/**
 * Needed for these options to work OOS
 */
fn shield_is_suspended() -> bool {
    read_rwlock(&SUSPEND_SHIELD)
}

/**
 * AKA should the cpu hold the shield button
 */
unsafe fn should_return_none_in_check_button(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> bool {
    if !is_operation_cpu(module_accessor) {
        return true;
    }

    if ![*CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_GUARD].contains(&button) {
        return true;
    }

    if !should_hold_shield(module_accessor) {
        return true;
    }

    if shield_is_suspended() {
        return true;
    }

    false
}

fn was_in_shieldstun(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_STATUS_KIND_GUARD_DAMAGE
    }
}
