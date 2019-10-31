#include "common.hpp"

namespace Tech {
void init_settings(u64 module_accessor, int status_kind, bool& replace) {
    if (is_training_mode() && is_operation_cpu(module_accessor)) {
        if (status_kind == FIGHTER_STATUS_KIND_DOWN) {
            if (menu.TECH_STATE == RANDOM_TECH) {
                const int NUM_TECH_STATUSES = 3;
                int random_statuses[NUM_TECH_STATUSES] = {
                    FIGHTER_STATUS_KIND_DOWN,
                    FIGHTER_STATUS_KIND_PASSIVE,
                    FIGHTER_STATUS_KIND_PASSIVE_FB
                };

                int random_status_index = app::sv_math::rand(hash40("fighter"), NUM_TECH_STATUSES);
                if (random_statuses[random_status_index] != FIGHTER_STATUS_KIND_DOWN) {
                    StatusModule::change_status_request_from_script(module_accessor, random_statuses[random_status_index], 1);
                    replace = true;
                    return;
                }
            } else if (menu.TECH_STATE == TECH_IN_PLACE) {
                StatusModule::change_status_request_from_script(module_accessor, FIGHTER_STATUS_KIND_PASSIVE, 1);
                replace = true;
                return;
            } else if (menu.TECH_STATE == TECH_ROLL) {
                StatusModule::change_status_request_from_script(module_accessor, FIGHTER_STATUS_KIND_PASSIVE_FB, 1);
                replace = true;
                return;
            } 
        }

        // else if (status_kind == FIGHTER_STATUS_KIND_PASSIVE) {
        //     const int NUM_TECH_STATUSES = 2;
        //     int random_statuses[NUM_TECH_STATUSES] = {
        //         FIGHTER_STATUS_KIND_PASSIVE,
        //         FIGHTER_STATUS_KIND_PASSIVE_FB
        //     };

        //     int random_status_index = app::sv_math::rand(hash40("fighter"), NUM_TECH_STATUSES);
        //     if (random_statuses[random_status_index] != FIGHTER_STATUS_KIND_PASSIVE)
        //         StatusModule::change_status_request_from_script(module_accessor, random_statuses[random_status_index], 1);
        // }
    }

    replace = false;
    return;
}

void get_command_flag_cat(u64 module_accessor, int category, int& flag) {
    if (menu.TECH_STATE != NONE && is_training_mode() && is_operation_cpu(module_accessor)) {
        int prev_status = StatusModule::prev_status_kind(module_accessor, 0);
        int status = StatusModule::status_kind(module_accessor);
        if (status == FIGHTER_STATUS_KIND_DOWN_WAIT || status == FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE) {
            const int NUM_GETUP_STATUSES = 3;
            int random_statuses[NUM_GETUP_STATUSES] = {
                FIGHTER_STATUS_KIND_DOWN_STAND,
                FIGHTER_STATUS_KIND_DOWN_STAND_FB, 
                FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK
            };

            int random_status_index = app::sv_math::rand(hash40("fighter"), NUM_GETUP_STATUSES);
            StatusModule::change_status_request_from_script(module_accessor, random_statuses[random_status_index], 1);
        }
        else if ((prev_status == FIGHTER_STATUS_KIND_PASSIVE || 
            prev_status == FIGHTER_STATUS_KIND_PASSIVE_FB ||
            prev_status == FIGHTER_STATUS_KIND_DOWN_STAND ||
            prev_status == FIGHTER_STATUS_KIND_DOWN_STAND_FB ||
            prev_status == FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK ||
            status == FIGHTER_STATUS_KIND_DOWN_STAND ||
            status == FIGHTER_STATUS_KIND_DOWN_STAND_FB ||
            status == FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK) && 
            WorkModule::is_enable_transition_term(module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON)) {
            perform_defensive_option(module_accessor);
        }
    }
}
}