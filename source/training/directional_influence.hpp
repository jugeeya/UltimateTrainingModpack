#include "common.hpp"

namespace DirectionalInfluence {
float get_float(u64 module_accessor, int var, bool& replace) {
    if (var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X ||
        var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y) {
        if (is_training_mode() && is_operation_cpu(module_accessor) &&
            is_in_hitstun(module_accessor)) {
            if (menu.DI_STATE != NONE) {
                float stick_x = 0.0, stick_y = 0.0;
                if (menu.DI_STATE == SET_DI) {
                    stick_x = menu.DI_stick_x;
                    stick_y = menu.DI_stick_y;
                } else if (menu.DI_STATE == DI_RANDOM_IN_AWAY) {
                    // either 1.0 or -1.0
                    stick_x = (float)(app::sv_math::rand(hash40("fighter"), 2) * 2.0) - 1;
                    stick_y = 0.0;
                }

                // If facing left, reverse stick x
                if (var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X) {
                    replace = true;
                    return stick_x * -1 * PostureModule::lr(module_accessor);
                }
                if (var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y) {
                    replace = true;
                    return stick_y;
                }
            }
        }
    }

    replace = false;
    return 0;
}
}