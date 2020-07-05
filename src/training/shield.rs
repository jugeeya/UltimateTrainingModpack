use crate::common::consts::*;
use crate::common::*;
use crate::training::frame_counter;
use crate::training::mash;
use smash::app;
use smash::app::lua_bind::*;
use smash::app::sv_system;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

// How many hits to hold shield until picking an Out Of Shield option
static mut MULTI_HIT_OFFSET: u32 = unsafe { MENU.oos_offset };
// Used to only decrease once per shieldstun change
static mut WAS_IN_SHIELDSTUN: bool = false;

static mut FRAME_COUNTER_INDEX: usize = 0;
static mut REACTION_INDEX: usize = 0;

// For how many frames should the shield hold be overwritten
static mut SHIELD_SUSPEND_FRAMES: u32 = 0;

pub fn init() {
    unsafe {
        FRAME_COUNTER_INDEX = frame_counter::register_counter();
        REACTION_INDEX = frame_counter::register_counter();
    }
}

// Toggle for shield decay
static mut SHIELD_DECAY: bool = false;

unsafe fn set_shield_decay(value: bool) {
    SHIELD_DECAY = value;
}

unsafe fn should_pause_shield_decay() -> bool {
    !SHIELD_DECAY
}

unsafe fn reset_oos_offset() {
    /*
     * Need to offset by 1, since we decrease as soon as shield gets hit
     * but only check later if we can OOS
     */
    MULTI_HIT_OFFSET = MENU.oos_offset + 1;
}

unsafe fn handle_oos_offset(module_accessor: &mut app::BattleObjectModuleAccessor) {
    // Check if we are currently in shield stun
    if !is_in_shieldstun(module_accessor) {
        // Make sure we don't forget and wait until we get hit on shield
        WAS_IN_SHIELDSTUN = false;
        return;
    }

    // Make sure we just freshly entered shield stun
    if WAS_IN_SHIELDSTUN {
        return;
    }

    // Decrease offset once if needed
    if MULTI_HIT_OFFSET > 0 {
        MULTI_HIT_OFFSET -= 1;
    }

    // Mark that we were in shield stun, so we don't decrease again
    WAS_IN_SHIELDSTUN = true;
}

pub unsafe fn allow_oos() -> bool {
    // Delay OOS until offset hits 0
    MULTI_HIT_OFFSET == 0
}

pub unsafe fn get_command_flag_cat(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_training_mode() {
        return;
    }

    if !is_operation_cpu(module_accessor) {
        return;
    }

    // Reset oos offset when standing
    if is_idle(module_accessor) || is_in_hitstun(module_accessor) {
        reset_oos_offset();
    }

    // Reset when not shielding
    let status_kind = StatusModule::status_kind(module_accessor);
    if !(status_kind == FIGHTER_STATUS_KIND_GUARD) {
        set_shield_decay(false);
    }
}

pub unsafe fn get_param_float(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    param_type: u64,
    param_hash: u64,
) -> Option<f32> {
    if !is_training_mode() {
        return None;
    }

    if !is_operation_cpu(module_accessor) {
        return None;
    }

    if MENU.shield_state != Shield::None {
        handle_oos_offset(module_accessor);
    }

    // Shield Decay//Recovery
    if MENU.shield_state == Shield::Infinite || should_pause_shield_decay() {
        if param_type != hash40("common") {
            return None;
        }
        if param_hash == hash40("shield_dec1") {
            return Some(0.0);
        }
        if param_hash == hash40("shield_recovery1") {
            return Some(999.0);
        }
        // doesn't work, somehow. This parameter isn't checked?
        if param_hash == hash40("shield_damage_mul") {
            return Some(0.0);
        }
    }

    None
}

pub unsafe fn should_hold_shield() -> bool {
    // Mash shield
    if mash::get_current_buffer() == Mash::Shield {
        return true;
    }

    // We should hold shield if the state requires it
    if ![Shield::Hold, Shield::Infinite].contains(&MENU.shield_state) {
        return false;
    }

    true
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_guard_cont)]
pub unsafe fn handle_sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    mod_handle_sub_guard_cont(fighter);
    original!()(fighter)
}

unsafe fn mod_handle_sub_guard_cont(fighter: &mut L2CFighterCommon) {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if !is_training_mode() || !is_operation_cpu(module_accessor) {
        return;
    }

    if !was_in_shieldstun(module_accessor) {
        return;
    }

    // Enable shield decay
    set_shield_decay(true);

    // Check for OOS delay
    if !allow_oos() {
        return;
    }

    if !is_shielding(module_accessor) {
        frame_counter::full_reset(REACTION_INDEX);
        return;
    }

    if frame_counter::should_delay(MENU.reaction_time, REACTION_INDEX){
        return;
    }

    if handle_escape_option(fighter, module_accessor) {
        return;
    }

    mash::buffer_action(MENU.mash_state);
    mash::set_attack(MENU.mash_attack_state);

    if needs_oos_handling_drop_shield() {
        return;
    }

    // Set shield suspension frames
    match MENU.mash_state {
        Mash::Attack => match MENU.mash_attack_state {
            Attack::UpSmash => {}
            Attack::Grab => {}
            _ => {
                // Force shield drop
                suspend_shield(15);
            }
        },

        _ => {}
    }
}

// Needed for shield drop options
pub fn suspend_shield(frames: u32) {
    if frames <= 0 {
        return;
    }

    unsafe {
        SHIELD_SUSPEND_FRAMES = frames;
        frame_counter::reset_frame_count(FRAME_COUNTER_INDEX);
        frame_counter::start_counting(FRAME_COUNTER_INDEX);
    }
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

    match MENU.mash_state {
        Mash::Spotdodge => {
            fighter
                .fighter_base
                .change_status(FIGHTER_STATUS_KIND_ESCAPE.as_lua_int(), LUA_TRUE);
            return true;
        }
        Mash::RollForward => {
            fighter
                .fighter_base
                .change_status(FIGHTER_STATUS_KIND_ESCAPE_F.as_lua_int(), LUA_TRUE);
            return true;
        }
        Mash::RollBack => {
            fighter
                .fighter_base
                .change_status(FIGHTER_STATUS_KIND_ESCAPE_B.as_lua_int(), LUA_TRUE);
            return true;
        }
        _ => return false,
    }
}

/**
 * Needed to allow these attacks to work OOS
 */
fn needs_oos_handling_drop_shield() -> bool {
    match mash::get_current_buffer() {
        Mash::Jump => return true,
        Mash::Attack => {
            let attack = mash::get_current_attack();
            if is_aerial(attack) {
                return true;
            }

            if attack == Attack::UpB {
                return true;
            }
        }
        _ => {}
    }

    false
}

fn is_aerial(attack: Attack) -> bool {
    match attack {
        Attack::Nair => return true,
        Attack::Fair => return true,
        Attack::Bair => return true,
        Attack::UpAir => return true,
        Attack::Dair => return true,
        _ => return false,
    }
}

/**
 * Needed for these options to work OOS
 */
unsafe fn shield_is_suspended() -> bool {
    // Normal behavior when not mashing
    if SHIELD_SUSPEND_FRAMES == 0 {
        return false;
    }

    let resume_normal_behavior =
        frame_counter::get_frame_count(FRAME_COUNTER_INDEX) > SHIELD_SUSPEND_FRAMES;

    if resume_normal_behavior {
        SHIELD_SUSPEND_FRAMES = 0;
        frame_counter::stop_counting(FRAME_COUNTER_INDEX);

        return false;
    }

    true
}

/**
 * AKA should the cpu hold the shield button
 */
unsafe fn should_return_none_in_check_button(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> bool {
    if !is_training_mode() {
        return true;
    }

    if !is_operation_cpu(module_accessor) {
        return true;
    }

    if ![*CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_GUARD].contains(&button) {
        return true;
    }

    if !should_hold_shield() {
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
