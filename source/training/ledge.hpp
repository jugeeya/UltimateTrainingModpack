#include "common.hpp"

namespace Ledge {
void force_option(u64 module_accessor) {
    if (StatusModule::status_kind(module_accessor) == FIGHTER_STATUS_KIND_CLIFF_WAIT) {
        if (WorkModule::is_enable_transition_term(module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB)) {
            
            int random_frame = app::sv_math::rand(
                hash40("fighter"), 
                (int) MotionModule::end_frame(module_accessor));

            float frame = MotionModule::frame(module_accessor);
                
            if (frame == random_frame || frame > 30.0) {
                int status = 0;
                int ledge_case = LEDGE_STATE - 1;

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

void defensive_option(u64 module_accessor, int category, int& flag) {
    int status = StatusModule::status_kind(module_accessor);
    if (status == FIGHTER_STATUS_KIND_CLIFF_JUMP3 ||
        status == FIGHTER_STATUS_KIND_CLIFF_JUMP2 ||
        status == FIGHTER_STATUS_KIND_CLIFF_JUMP1) {
        flag |= FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE;
    }

    if ((status == FIGHTER_STATUS_KIND_CLIFF_CLIMB || 
        status == FIGHTER_STATUS_KIND_CLIFF_ATTACK || 
        status == FIGHTER_STATUS_KIND_CLIFF_ESCAPE) && 
        WorkModule::is_enable_transition_term(module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE)) {
        const int NUM_GROUND_STATUSES = 3;
        int random_statuses[NUM_GROUND_STATUSES] = {
            FIGHTER_STATUS_KIND_ESCAPE, 
            FIGHTER_STATUS_KIND_ATTACK,
            FIGHTER_STATUS_KIND_GUARD_ON
        };

        int random_status_index = app::sv_math::rand(hash40("fighter"), NUM_GROUND_STATUSES);
        StatusModule::change_status_request_from_script(module_accessor, random_statuses[random_status_index], 1);
    }
}

void get_command_flag_cat(u64 module_accessor, int category, int& flag) {
    if (LEDGE_STATE != NONE && is_training_mode() && is_operation_cpu(module_accessor)) {
        force_option(module_accessor);
        defensive_option(module_accessor, category, flag);
    }
}
}