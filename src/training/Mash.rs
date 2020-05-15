use crate::common::consts::*;
use crate::common::*;
use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;

pub unsafe fn get_attack_air_kind(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> Option<i32> {
    if is_training_mode() && is_operation_cpu(module_accessor) {
        if menu.MASH_STATE == MASH_ATTACK {
            match menu.ATTACK_STATE {
                MASH_NAIR => return Some(*FIGHTER_COMMAND_ATTACK_AIR_KIND_N),
                MASH_FAIR => return Some(*FIGHTER_COMMAND_ATTACK_AIR_KIND_F),
                MASH_BAIR => return Some(*FIGHTER_COMMAND_ATTACK_AIR_KIND_B),
                MASH_UPAIR => return Some(*FIGHTER_COMMAND_ATTACK_AIR_KIND_HI),
                MASH_DAIR => return Some(*FIGHTER_COMMAND_ATTACK_AIR_KIND_LW),
                _ => (),
            }
        }

        if menu.MASH_STATE == MASH_RANDOM {
            return Some(app::sv_math::rand(hash40("fighter"), 5) + 1);
        }
    }

    None
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
    flag: &mut i32,
) {
    if is_training_mode() && is_operation_cpu(module_accessor) {
        if is_in_hitstun(module_accessor)
            || is_in_landing(module_accessor)
            || is_in_shieldstun(module_accessor)
        {
            match menu.MASH_STATE {
                MASH_AIRDODGE => {
                    if category == FIGHTER_PAD_COMMAND_CATEGORY1 {
                        *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE;
                    }
                }
                MASH_JUMP => {
                    if !is_in_landing(module_accessor) && category == FIGHTER_PAD_COMMAND_CATEGORY1
                    {
                        *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON;
                    }
                }
                MASH_SPOTDODGE => {
                    if category == FIGHTER_PAD_COMMAND_CATEGORY1 {
                        *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE;
                    }
                }
                MASH_ATTACK => {
                    if category == FIGHTER_PAD_COMMAND_CATEGORY1 {
                        match menu.ATTACK_STATE {
                            MASH_NAIR | MASH_FAIR | MASH_BAIR | MASH_UPAIR | MASH_DAIR => {
                                *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N;
                                // If we are performing the attack OOS we also need to jump
                                if is_in_shieldstun(module_accessor) {
                                    *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON;
                                }
                            }
                            MASH_NEUTRAL_B => *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N,
                            MASH_SIDE_B => *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S,
                            MASH_UP_B => *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI,
                            MASH_DOWN_B => *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW,
                            MASH_UP_SMASH => *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4,
                            MASH_GRAB => *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH,
                            _ => (),
                        }
                    }
                }
                MASH_RANDOM => {
                    if category == FIGHTER_PAD_COMMAND_CATEGORY1 {
                        let situation_kind = StatusModule::situation_kind(module_accessor) as i32;

                        if situation_kind == SITUATION_KIND_AIR {
                            let random_commands = vec![
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

                            let random_cmd_index =
                                app::sv_math::rand(hash40("fighter"), random_commands.len() as i32)
                                    as usize;

                            *flag |= random_commands[random_cmd_index];
                        } else if situation_kind == SITUATION_KIND_GROUND {
                            let random_commands = vec![
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

                            let random_cmd_index =
                                app::sv_math::rand(hash40("fighter"), random_commands.len() as i32)
                                    as usize;

                            *flag |= random_commands[random_cmd_index];
                        }
                    }
                }
                _ => (),
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
            if menu.MASH_STATE == MASH_AIRDODGE
                && (is_in_hitstun(module_accessor) || is_in_landing(module_accessor))
            {
                return Some(true);
            }
        }
    }

    None
}
