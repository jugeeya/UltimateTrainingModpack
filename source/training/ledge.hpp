#include "common.hpp"

namespace Ledge {
void enable_transition_term(u64 module_accessor, int transition_id) {
    if (ESCAPE_STATE == ESCAPE_LEDGE && is_training_mode() && is_operation_cpu(module_accessor)) {
        if (StatusModule::status_kind(module_accessor) == FIGHTER_STATUS_KIND_CLIFF_WAIT) {
            if (transition_id == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB) {
                int status = 0;
                int ledge_case = LEDGE_STATE;

                if (LEDGE_STATE == RANDOM_LEDGE)
                    ledge_case = app::sv_math::rand(hash40("fighter"), 4) + 1;

                switch (ledge_case) {
                    case NEUTRAL_LEDGE:
                        status = FIGHTER_STATUS_KIND_CLIFF_CLIMB; break;
                    case ROLL_LEDGE:
                        status = FIGHTER_STATUS_KIND_CLIFF_ESCAPE; break;
                    case JUMP_LEDGE:
                        status = FIGHTER_STATUS_KIND_CLIFF_JUMP1; break;
                    case ATTACK_LEDGE:
                        status = FIGHTER_STATUS_KIND_CLIFF_ATTACK; break;
                }

                StatusModule::change_status_request_from_script(module_accessor, status, 1);
            }
        }
    }
}

int get_command_flag_cat(u64 module_accessor, int category, int orig_flag) {
    int flag = 0;
    if (is_training_mode() && is_operation_cpu(module_accessor)) {
        if (ESCAPE_STATE == ESCAPE_LEDGE) {
            int prev_status = StatusModule::prev_status_kind(module_accessor, 1);
            if (prev_status == FIGHTER_STATUS_KIND_CLIFF_JUMP3 ||
                prev_status == FIGHTER_STATUS_KIND_CLIFF_JUMP2 ||
                prev_status == FIGHTER_STATUS_KIND_CLIFF_JUMP1) {
                flag |= FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE;
            } else if (prev_status == FIGHTER_STATUS_KIND_CLIFF_CLIMB ||
                        prev_status == FIGHTER_STATUS_KIND_CLIFF_ATTACK ||
                        prev_status == FIGHTER_STATUS_KIND_CLIFF_ESCAPE) {
                const int NUM_GROUND_COMMANDS = 2;
                int random_commands[NUM_GROUND_COMMANDS] = {
                    FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N, 
                    FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE,
                };

                int random_cmd_index = app::sv_math::rand(hash40("fighter"), NUM_GROUND_COMMANDS);

                flag |= random_commands[random_cmd_index];
            }
        }
    }

    return flag | orig_flag;
}
}