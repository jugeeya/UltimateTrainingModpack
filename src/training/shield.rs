use crate::common::consts::*;
use crate::common::*;
use smash::app;
use smash::app::lua_bind::*;
use smash::app::sv_system;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

// How many hits to hold shield until picking an Out Of Shield option
static mut MULTI_HIT_OFFSET: u8 = MENU.oos_offset;
// Used to only decrease once per shieldstun change
static mut WAS_IN_SHIELDSTUN: bool = false;

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
    if !is_operation_cpu(module_accessor) {
        return None;
    }

    if is_training_mode() {
        if MENU.shield_state != Shield::None {
            handle_oos_offset(module_accessor);
        }

        if MENU.shield_state == Shield::Infinite || should_pause_shield_decay() {
            if param_type == hash40("common") {
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
        }
    }

    None
}

pub unsafe fn should_hold_shield(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    // We should hold shield if the state requires it
    if [Shield::Hold, Shield::Infinite].contains(&MENU.shield_state) {
        // If we are not mashing attack then we will always hold shield
        if MENU.mash_state != Mash::Attack {
            return true;
        }

        // Hold shield while OOS is not allowed
        if !allow_oos() {
            return true;
        }

        if !is_in_shieldstun(module_accessor) {
            return true;
        }

        // We will only drop shield if we are in shieldstun and our attack can be performed OOS
        if MENU.mash_state == Mash::Attack {
            if [Attack::NeutralB, Attack::SideB, Attack::DownB].contains(&MENU.mash_attack_state) {
                return false;
            }

            if MENU.mash_attack_state == Attack::Grab {
                return true;
            }
        }
    }

    false
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_guard_cont)]
pub unsafe fn handle_sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    mod_handle_sub_guard_cont(fighter);
    original!()(fighter)
}

unsafe fn mod_handle_sub_guard_cont(fighter: &mut L2CFighterCommon) {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);

    // Enable shield decay
    if !is_training_mode()
        || !is_operation_cpu(module_accessor)
        || !StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_STATUS_KIND_GUARD_DAMAGE
    {
        return;
    }

    set_shield_decay(true);

    // Check for OOS delay
    if !allow_oos() {
        return;
    }

    if MENU.mash_state == Mash::Attack && MENU.mash_attack_state == Attack::Grab {
        if WorkModule::get_int(
            module_accessor,
            *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME,
        ) == 0
        {
            if WorkModule::is_enable_transition_term(
                module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
            ) {
                fighter.fighter_base.change_status(
                    L2CValue::new_int(*FIGHTER_STATUS_KIND_CATCH as u64),
                    L2CValue::new_bool(true),
                );
            }
        }
    }
    if MENU.mash_state == Mash::Spotdodge {
        if WorkModule::is_enable_transition_term(
            module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        ) {
            fighter.fighter_base.change_status(
                L2CValue::new_int(*FIGHTER_STATUS_KIND_ESCAPE as u64),
                L2CValue::new_bool(true),
            );
        }
    }
    if MENU.mash_state == Mash::RollForward {
        if WorkModule::is_enable_transition_term(
            module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        ) {
            fighter.fighter_base.change_status(
                L2CValue::new_int(*FIGHTER_STATUS_KIND_ESCAPE_F as u64),
                L2CValue::new_bool(true),
            );
        }
    }
    if MENU.mash_state == Mash::RollBack {
        if WorkModule::is_enable_transition_term(
            module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        ) {
            fighter.fighter_base.change_status(
                L2CValue::new_int(*FIGHTER_STATUS_KIND_ESCAPE_B as u64),
                L2CValue::new_bool(true),
            );
        }
    }
    if MENU.mash_state == Mash::Attack {
        if MENU.mash_attack_state == Attack::UpB {
            if WorkModule::is_enable_transition_term(
                module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
            ) {
                fighter.fighter_base.change_status(
                    L2CValue::new_int(*FIGHTER_STATUS_KIND_SPECIAL_HI as u64),
                    L2CValue::new_bool(false),
                );
            }
        }
        if MENU.mash_attack_state == Attack::UpSmash {
            if WorkModule::is_enable_transition_term(
                module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
            ) {
                fighter.fighter_base.change_status(
                    L2CValue::new_int(*FIGHTER_STATUS_KIND_ATTACK_HI4_START as u64),
                    L2CValue::new_bool(false),
                );
            }
        }
    }
}

pub unsafe fn check_button_on(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> Option<bool> {
    if [*CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_GUARD].contains(&button) {
        if is_training_mode() && is_operation_cpu(module_accessor) {
            if should_hold_shield(module_accessor) {
                return Some(true);
            }
        }
    }

    None
}

pub unsafe fn check_button_off(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> Option<bool> {
    if [*CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_GUARD].contains(&button) {
        if is_training_mode() && is_operation_cpu(module_accessor) {
            if should_hold_shield(module_accessor) {
                return Some(false);
            }
        }
    }

    None
}
