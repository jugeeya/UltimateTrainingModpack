#include "common.h"

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

namespace DirectionalInfluence {
float get_float(u64 module_accessor, int var, bool& replace) {
    if (var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X ||
        var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y) {
        if (is_training_mode() && is_operation_cpu(module_accessor) &&
            is_in_hitstun(module_accessor)) {
            if (menu.DI_STATE != NONE) {
                float angle = (menu.DI_STATE - 1) * M_PI / 4.0;

                // Either 0 (right) or PI (left)
                if (menu.DI_STATE == DI_RANDOM_IN_AWAY) {
                    angle = app::sv_math::rand(hash40("fighter"), 2) * M_PI;
                }
                // If facing left, reverse angle
                if (PostureModule::lr(module_accessor) != -1.0) angle -= M_PI;

                if (var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X) {
                    replace = true;
                    return cos(angle);
                }

                if (var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y) {
                    replace = true;
                    return sin(angle);
                }
            }
        }
    }

    replace = false;
    return 0;
}
}