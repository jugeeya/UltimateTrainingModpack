use crate::common::consts::*;
use crate::common::*;
use smash::app;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::sv_system;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

pub unsafe fn get_param_float(
    _module_accessor: &mut app::BattleObjectModuleAccessor,
    param_type: u64,
    param_hash: u64,
) -> Option<f32> {
    if is_training_mode() {
        if MENU.shield_state == Shield::Infinite {
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
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if is_training_mode() && is_operation_cpu(module_accessor) {
        if MENU.mash_state == Mash::Attack && MENU.mash_attack_state == Attack::Grab {
            if StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_STATUS_KIND_GUARD_DAMAGE {
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
        }
        if MENU.mash_state == Mash::Spotdodge {
            if StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_STATUS_KIND_GUARD_DAMAGE {
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
        }

        if MENU.mash_state == Mash::Attack {
            if MENU.mash_attack_state == Attack::UpB {
                if StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_STATUS_KIND_GUARD_DAMAGE {
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
            }
            if MENU.mash_attack_state == Attack::UpSmash {
                if StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_STATUS_KIND_GUARD_DAMAGE {
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
    }

    original!()(fighter)
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
