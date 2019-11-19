#include "common.hpp"

namespace Mash {
int get_attack_air_kind(u64 module_accessor, bool& replace) {
    int kind = 0;
    if (is_training_mode() && is_operation_cpu(module_accessor)) {
        if (menu.MASH_STATE == MASH_ATTACK) {
            replace = true;
            switch (menu.ATTACK_STATE) {
                case MASH_NAIR:
                    kind = FIGHTER_COMMAND_ATTACK_AIR_KIND_N; break;
                case MASH_FAIR:
                    kind = FIGHTER_COMMAND_ATTACK_AIR_KIND_F; break;
                case MASH_BAIR:
                    kind = FIGHTER_COMMAND_ATTACK_AIR_KIND_B; break;
                case MASH_UPAIR:
                    kind = FIGHTER_COMMAND_ATTACK_AIR_KIND_HI; break;
                case MASH_DAIR:
                    kind = FIGHTER_COMMAND_ATTACK_AIR_KIND_LW; break;
            }
            return kind;
        }

        if (menu.MASH_STATE == MASH_RANDOM) {
            replace = true;
            return app::sv_math::rand(hash40("fighter"), 5) + 1;
        }
    }

    replace = false;
    return kind;
}

void get_command_flag_cat(u64 module_accessor, int category, int& flag) {

    if (is_training_mode() && is_operation_cpu(module_accessor)) {
        if (is_in_hitstun(module_accessor) || is_in_landing(module_accessor) || is_in_shieldstun(module_accessor)) {
            if (menu.MASH_STATE == MASH_AIRDODGE)
                if (category == FIGHTER_PAD_COMMAND_CATEGORY1)
                    flag |= FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE;

            if (menu.MASH_STATE == MASH_JUMP && !is_in_landing(module_accessor))
                if (category == FIGHTER_PAD_COMMAND_CATEGORY1)
                    flag |= FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON;

            if (menu.MASH_STATE == MASH_SPOTDODGE)
                if (category == FIGHTER_PAD_COMMAND_CATEGORY1)
                    flag |= FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE;

            if (menu.MASH_STATE == MASH_ATTACK)
                if (category == FIGHTER_PAD_COMMAND_CATEGORY1) {
                    switch (menu.ATTACK_STATE) {
                        case MASH_NAIR:
                        case MASH_FAIR:
                        case MASH_BAIR:
                        case MASH_UPAIR:
                        case MASH_DAIR:
                            flag |= FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N;
                            // If we are performing the attack OOS we also need to jump
                            if(is_in_shieldstun(module_accessor))
                                flag |= FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON;
                            break;
                        case MASH_NEUTRAL_B:
                            flag |= FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N;
                            break;
                        case MASH_SIDE_B:
                            flag |= FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S;
                            break;
                        case MASH_UP_B:
                            flag |= FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI;
                            break;
                        case MASH_DOWN_B:
                            flag |= FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW;
                        case MASH_UP_SMASH:
                            flag |= FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4;
                        case MASH_GRAB:
                            flag |= FIGHTER_PAD_CMD_CAT1_FLAG_CATCH;
                            break;
                    }
                }

            if (menu.MASH_STATE == MASH_RANDOM)
                if (category == FIGHTER_PAD_COMMAND_CATEGORY1) {
                    int situation_kind =
                        StatusModule::situation_kind(module_accessor);

                    if (situation_kind == SITUATION_KIND_AIR) {
                        const int NUM_AIR_COMMANDS = 11;
                        int random_commands[NUM_AIR_COMMANDS] = {
                            FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE,
                            FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON,
                            // one for each aerial
                            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
                            FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N,
                            FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S,
                            FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI,
                            FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW,
                        };

                        int random_cmd_index = app::sv_math::rand(
                            hash40("fighter"), NUM_AIR_COMMANDS);

                        flag |= random_commands[random_cmd_index];
                    } else if (situation_kind == SITUATION_KIND_GROUND) {
                        const int NUM_GROUND_COMMANDS = 16;
                        int random_commands[NUM_GROUND_COMMANDS] = {
                            FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4,
                            FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI,
                            FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S,
                            FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI,
                            FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW,
                            FIGHTER_PAD_CMD_CAT1_FLAG_CATCH,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F,
                            FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B,
                        };

                        int random_cmd_index = app::sv_math::rand(
                            hash40("fighter"), NUM_GROUND_COMMANDS);

                        flag |= random_commands[random_cmd_index];
                    }
                }
        }
    }
}

bool check_button_on(u64 module_accessor, int button, bool& replace) {
    if (button == CONTROL_PAD_BUTTON_GUARD_HOLD || button == CONTROL_PAD_BUTTON_GUARD) {
        if (is_training_mode() && is_operation_cpu(module_accessor)) {
            if (menu.MASH_STATE == MASH_AIRDODGE && (is_in_hitstun(module_accessor) || is_in_landing(module_accessor))) {
                replace = true;
                return true;
            }
        }
    }

    replace = false;
    return false;
}
}