#include "common.h"

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
                int ledge_case = menu.LEDGE_STATE;

                if (menu.LEDGE_STATE == RANDOM_LEDGE)
                    ledge_case = app::sv_math::rand(hash40("fighter"), 4) + 2;

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

bool should_perform_defensive_option(u64 module_accessor, int prev_status, int status) {
    return (status == FIGHTER_STATUS_KIND_CLIFF_CLIMB || 
        status == FIGHTER_STATUS_KIND_CLIFF_ATTACK || 
        status == FIGHTER_STATUS_KIND_CLIFF_ESCAPE ||
        prev_status == FIGHTER_STATUS_KIND_CLIFF_CLIMB ||
        prev_status == FIGHTER_STATUS_KIND_CLIFF_ATTACK || 
        prev_status == FIGHTER_STATUS_KIND_CLIFF_ESCAPE) && 
        (WorkModule::is_enable_transition_term(module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) ||
        CancelModule::is_enable_cancel(module_accessor));
}

void defensive_option(u64 module_accessor, int category, int& flag) {
    int status = StatusModule::status_kind(module_accessor);
    int prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    if (status == FIGHTER_STATUS_KIND_CLIFF_JUMP3 ||
        status == FIGHTER_STATUS_KIND_CLIFF_JUMP2 ||
        status == FIGHTER_STATUS_KIND_CLIFF_JUMP1) {
        flag |= FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE;
    }

    if (should_perform_defensive_option(module_accessor, prev_status, status)) {
        perform_defensive_option(module_accessor, flag);
    }
}

bool check_button_on(u64 module_accessor, int button, bool& replace) {
    if (button == CONTROL_PAD_BUTTON_GUARD_HOLD || button == CONTROL_PAD_BUTTON_GUARD) {
        if (is_training_mode() && is_operation_cpu(module_accessor)) {
            if (menu.DEFENSIVE_STATE == DEFENSIVE_SHIELD && 
                should_perform_defensive_option(
                    module_accessor, 
                    StatusModule::prev_status_kind(module_accessor, 0), 
                    StatusModule::status_kind(module_accessor))) {
                replace = true;
                return true;
            }
        }
    }

    replace = false;
    return false;
}

void get_command_flag_cat(u64 module_accessor, int category, int& flag) {
    if (menu.LEDGE_STATE != NONE && is_training_mode() && is_operation_cpu(module_accessor)) {
        force_option(module_accessor);
        defensive_option(module_accessor, category, flag);
    }
}
}