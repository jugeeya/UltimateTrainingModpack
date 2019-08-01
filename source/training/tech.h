#include "common.hpp"

namespace Tech {
void init_settings(u64 module_accessor, int status_kind) {
    if (ESCAPE_STATE == ESCAPE_TECH && is_training_mode() && is_operation_cpu(module_accessor)) {
        if (status_kind == FIGHTER_STATUS_KIND_DOWN) {
            const int NUM_TECH_STATUSES = 4;
            int random_statuses[NUM_TECH_STATUSES] = {
                FIGHTER_STATUS_KIND_DOWN,
                FIGHTER_STATUS_KIND_PASSIVE,
                FIGHTER_STATUS_KIND_PASSIVE_FB,
                FIGHTER_STATUS_KIND_PASSIVE_FB
            };

            int random_status_index = app::sv_math::rand(hash40("fighter"), NUM_TECH_STATUSES);
            StatusModule::change_status_request_from_script(module_accessor, random_statuses[random_status_index], 1);
        }

        else if (status_kind == FIGHTER_STATUS_KIND_PASSIVE) {
            const int NUM_TECH_STATUSES = 2;
            int random_statuses[NUM_TECH_STATUSES] = {
                FIGHTER_STATUS_KIND_PASSIVE,
                FIGHTER_STATUS_KIND_PASSIVE_FB
            };

            int random_status_index = app::sv_math::rand(hash40("fighter"), NUM_TECH_STATUSES);
            StatusModule::change_status_request_from_script(module_accessor, random_statuses[random_status_index], 1);
        }
    }
}
}