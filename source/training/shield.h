#include "common.h"
#include "useful/crc32.h"

namespace Shield {
float get_param_float(u64 module_accessor, u64 param_type, u64 param_hash, bool& replace) {
    if (is_training_mode()) {
        if (menu.SHIELD_STATE == SHIELD_INFINITE) {
            if (param_type == hash40("common")) {
                if (param_hash == hash40("shield_dec1")) {
                    replace = true;
                    return 0.0;
                }
                if (param_hash == hash40("shield_recovery1")) {
                    replace = true;
                    return 999.0;
                }
                // doesn't work, somehow. This parameter isn't checked?
                if (param_hash == hash40("shield_damage_mul")) {
                    replace = true;
                    return 0.0;
                }
            }
        }
    }

    replace = false;
    return 0.0;
}

bool should_hold_shield(u64 module_accessor) {
    // We should hold shield if the state requires it
    if (menu.SHIELD_STATE == SHIELD_HOLD || menu.SHIELD_STATE == SHIELD_INFINITE) {
        // If we are not mashing then we will always hold shield
        if(menu.MASH_STATE == NONE)
            return true;

        if(!is_in_shieldstun(module_accessor))
            return true;

        // We will only drop shield if we are in shieldstun and our attack can be performed OOS
        if(menu.MASH_STATE == MASH_ATTACK) {
            if(menu.ATTACK_STATE == MASH_NEUTRAL_B || menu.ATTACK_STATE == MASH_SIDE_B || menu.ATTACK_STATE == MASH_DOWN_B)
                return true;
        }
    }

    return false;
}

bool check_button_on(u64 module_accessor, int button, bool& replace) {
    if (button == CONTROL_PAD_BUTTON_GUARD_HOLD || button == CONTROL_PAD_BUTTON_GUARD) {
        if (is_training_mode() && is_operation_cpu(module_accessor)) {
            if (should_hold_shield(module_accessor)) {
                replace = true;
                return true;
            }
        }
    }

    replace = false;
    return false;
}

bool check_button_off(u64 module_accessor, int button, bool& replace) {
    if (button == CONTROL_PAD_BUTTON_GUARD_HOLD || button == CONTROL_PAD_BUTTON_GUARD) {
        if (is_training_mode() && is_operation_cpu(module_accessor)) {
            if (should_hold_shield(module_accessor)) {
                replace = true;
                return false;
            }
        }
    }

    replace = false;
    return false;
}
}