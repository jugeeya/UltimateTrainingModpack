use crate::common::consts::*;
use crate::common::*;
use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;

static mut BUFFERED_ACTION: Mash = Mash::None;
static mut BUFFERED_ATTACK: Attack = Attack::Nair;

pub fn buffer_action(action: Mash) {
    unsafe {
        BUFFERED_ACTION = action;
    }
    println!("Buffering {}", action as i32);
}

pub fn get_current_buffer() -> Mash {
    unsafe { BUFFERED_ACTION }
}

pub fn set_attack(attack: Attack) {
    unsafe {
        BUFFERED_ATTACK = attack;
    }
    println!("Attack {}", attack as i32);
}
pub fn get_current_attack() -> Attack {
    unsafe { BUFFERED_ATTACK }
}

pub unsafe fn get_attack_air_kind(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> Option<i32> {
    if !is_training_mode() {
        return None;
    }

    if !is_operation_cpu(module_accessor) {
        return None;
    }

    BUFFERED_ATTACK.into_attack_air_kind()
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
    flag: &mut i32,
) {
    // Only do once per frame
    if category != FIGHTER_PAD_COMMAND_CATEGORY1 {
        return;
    }

    if !is_training_mode() {
        return;
    }

    if !is_operation_cpu(module_accessor) {
        return;
    }

    check_buffer(module_accessor);

    *flag |= perform_action(module_accessor);
}

unsafe fn check_buffer(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if BUFFERED_ACTION != Mash::None {
        return;
    }

    if !is_in_hitstun(module_accessor) && MENU.mash_in_neutral != MashInNeutral::On {
        return;
    }

    buffer_action(MENU.mash_state);
    set_attack(MENU.mash_attack_state);
}

unsafe fn perform_action(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    match BUFFERED_ACTION {
        Mash::Airdodge => {
            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR,
                *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE,
            );
        }
        Mash::Jump => {
            return update_jump_flag(module_accessor);
        }
        Mash::Spotdodge => {
            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE,
            );
        }
        Mash::RollForward => {
            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F,
            );
        }
        Mash::RollBack => {
            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B,
            );
        }
        Mash::Attack => {
            return get_attack_flag(module_accessor);
        }
        _ => return 0,
    }
}

unsafe fn update_jump_flag(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    let check_flag: i32;

    if is_grounded(module_accessor) {
        check_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON;
    } else if is_airborne(module_accessor) {
        check_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON;
    } else {
        check_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_JUMP_BUTTON;
    }

    return get_flag(
        module_accessor,
        check_flag,
        *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON,
    );
}

unsafe fn get_attack_flag(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    use Attack::*;

    let action_flag: i32;
    let transition_flag: i32;

    match BUFFERED_ATTACK {
        Nair | Fair | Bair | UpAir | Dair => {
            return get_aerial_flag(module_accessor, BUFFERED_ATTACK);
        }
        NeutralB => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N;
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N;
        }
        SideB => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S;
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S;
        }
        UpB => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI;
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI;
        }
        DownB => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW;
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW;
        }
        UpSmash => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4;
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4;
        }
        Grab => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH;
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH;
        }
        Jab => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N;
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK;
        }
        _ => return 0,
    }

    return get_flag(module_accessor, transition_flag, action_flag);
}

unsafe fn get_aerial_flag(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    attack: Attack,
) -> i32 {
    let mut flag: i32 = 0;
    // If we are grounded we also need to jump
    if is_grounded(module_accessor) {
        flag += update_jump_flag(module_accessor);
        // Keep buffer until jump squat is over
        BUFFERED_ACTION = Mash::Attack;
    }

    let action_flag: i32;
    let transition_flag: i32;

    transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR;

    match attack {
        Attack::Nair => {
            action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_N;
        }
        Attack::Fair => {
            // For some reason the game doesn't trigger the fair correctly
            // action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_F;
            action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_N;
        }
        Attack::Bair => {
            action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_B;
        }
        Attack::UpAir => {
            // For some reason the game doesn't trigger the uair correctly
            // action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI;
            action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_N;
        }
        Attack::Dair => {
            action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW;
        }
        _ => {
            action_flag = 0;
        }
    }

    flag |= get_flag(module_accessor, transition_flag, action_flag);

    flag
}

/**
 * Updates the flag if the transition is valid
 *
 */
unsafe fn get_flag(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    transition_flag: i32,
    action_flag: i32,
) -> i32 {
    if !WorkModule::is_enable_transition_term(module_accessor, transition_flag) {
        return 0;
    }

    // Reset Buffer
    println!("Consuming Buffer Action: {}", action_flag);
    BUFFERED_ACTION = Mash::None;

    return action_flag;
}

unsafe fn get_random_command_list(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> Vec<i32> {
    let situation_kind = StatusModule::situation_kind(module_accessor) as i32;

    if situation_kind == SITUATION_KIND_AIR {
        return vec![
            *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE,
            *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON,
            // one for each aerial
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
            *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N,
            *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S,
            *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI,
            *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW,
        ];
    }

    if situation_kind == SITUATION_KIND_GROUND {
        return vec![
            *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4,
            *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI,
            *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S,
            *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI,
            *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW,
            *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B,
        ];
    }

    return vec![];
}

pub unsafe fn perform_defensive_option(
    _module_accessor: &mut app::BattleObjectModuleAccessor,
    _flag: &mut i32,
) {
    match MENU.defensive_state {
        Defensive::Random => {
            let random_cmds = vec![
                Mash::Spotdodge,
                Mash::RollForward,
                Mash::RollBack,
                Mash::Attack,
            ];

            let random_cmd_index =
                app::sv_math::rand(hash40("fighter"), random_cmds.len() as i32) as usize;

            buffer_action(random_cmds[random_cmd_index]);
            set_attack(Attack::Jab);
        }
        Defensive::Roll => {
            if app::sv_math::rand(hash40("fighter"), 2) == 0 {
                buffer_action(Mash::RollForward);
            } else {
                buffer_action(Mash::RollBack);
            }
        }
        Defensive::Spotdodge => buffer_action(Mash::Spotdodge),
        Defensive::Jab => {
            buffer_action(Mash::Attack);
            set_attack(Attack::Jab);
        }
        _ => (),
    }
}