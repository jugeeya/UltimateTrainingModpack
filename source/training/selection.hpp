#include "common.hpp"

extern int vsnprintf(char* s, size_t maxlen, const char* format, va_list arg) LINKABLE;

int vsnprintf_intercept(char* s, size_t maxlen, const char* format, va_list arg) {
    if (strcmp(format, "mel_training_help_shift0") == 0) {
        TOGGLE_STATE = MASH_TOGGLES;
        if (MASH_STATE == NONE)
            format = "mel_shortmsg_1";
        if (MASH_STATE == MASH_AIRDODGE)
            format = "mel_shortmsg_2";
        if (MASH_STATE == MASH_JUMP)
            format = "mel_shortmsg_3";
        if (MASH_STATE == MASH_RANDOM)
            format = "mel_shortmsg_4";
        if (MASH_STATE == MASH_ATTACK)
            format = "mel_shortmsg_5";
    } else if (strcmp(format, "mel_training_help_shift1") == 0) {
        TOGGLE_STATE = ESCAPE_TOGGLES;
        if (ESCAPE_STATE == NONE)
            format = "mel_shortmsg_6";
        if (ESCAPE_STATE == ESCAPE_LEDGE)
            format = "mel_shortmsg_7";
    } else if (strcmp(format, "mel_training_help_shift2") == 0) {
        TOGGLE_STATE = SHIELD_TOGGLES;
        if (SHIELD_STATE == NONE)
            format = "mel_shortmsg_8";
        if (SHIELD_STATE == SHIELD_INFINITE)
            format = "mel_shortmsg_9";
        if (SHIELD_STATE == SHIELD_HOLD)
            format = "mel_shortmsg_10";
    }

    // For Shulk
    if (strcmp(format, "mel_info_fighter_shulk_special_00") == 0)
        format = "mel_shortmsg_101"; // SMASH
    else if (strcmp(format, "mel_info_fighter_shulk_special_03") == 0)
        format = "mel_shortmsg_102"; // SPEED
    else if (strcmp(format, "mel_info_fighter_shulk_special_02") == 0)
        format = "mel_shortmsg_103"; // SHIELD

    if (strcmp(format, "mel_training_shift0") == 0)
        format = "mel_info_fighter_shulk_special_00"; // SMASH
    else if (strcmp(format, "mel_training_shift1") == 0)
        format = "mel_info_fighter_shulk_special_03"; // SPEED
    else if (strcmp(format, "mel_training_shift2") == 0)
        format = "mel_info_fighter_shulk_special_02"; // SHIELD

    return vsnprintf(s, maxlen, format, arg);
}

namespace Selection {
void menu_replace() {
    SaltySDCore_ReplaceImport("vsnprintf", (void*)vsnprintf_intercept);
}

void change_motion(u64 module_accessor, u64 motion_kind) {
if (motion_kind == hash40("appeal_lw_l") || motion_kind == hash40("appeal_lw_r")) {
        if (is_training_mode()) {
            if (TOGGLE_STATE == MASH_TOGGLES) {
                MASH_STATE = (MASH_STATE + 1) % NUM_MASH_STATES;
                const char* toggle_strings[NUM_MASH_STATES] =
                    {"NONE", "AIRDODGE", "JUMP", "ATTACK", "RANDOM"};

                print_string(module_accessor, toggle_strings[MASH_STATE]);
            }

            if (TOGGLE_STATE == ESCAPE_TOGGLES) {
                ESCAPE_STATE = (ESCAPE_STATE + 1) % NUM_ESCAPE_STATES;
                const char* toggle_strings[NUM_ESCAPE_STATES] = 
                    {"NONE", "LEDGE"};

                print_string(module_accessor, toggle_strings[ESCAPE_STATE]);
            }

            if (TOGGLE_STATE == SHIELD_TOGGLES) {
                SHIELD_STATE = (SHIELD_STATE + 1) % NUM_SHIELD_STATES;
                const char* toggle_strings[NUM_SHIELD_STATES] =
                    {"NONE", "INFINITE", "HOLD"};

                print_string(module_accessor, toggle_strings[SHIELD_STATE]);
            }
        }
    } else if (motion_kind == hash40("appeal_s_l") || motion_kind == hash40("appeal_s_r")) {
		if (is_training_mode()) {
            if (TOGGLE_STATE == ESCAPE_TOGGLES &&
                ESCAPE_STATE == ESCAPE_LEDGE) {
                LEDGE_STATE = (LEDGE_STATE + 1) % NUM_LEDGE_STATES;
                const char* LEDGE_strings[NUM_LEDGE_STATES] =
                    {"RANDOM", "NORMAL", "ROLL", "JUMP", "ATTACK"};

                print_string(module_accessor, LEDGE_strings[LEDGE_STATE]);
            } else if (MASH_STATE == MASH_ATTACK) {
                ATTACK_STATE = (ATTACK_STATE + 1) % NUM_ATTACK_STATES;
                const char* ATTACK_strings[NUM_ATTACK_STATES] =
                    {"NAIR",      "FAIR",   "BAIR", "UPAIR", "DAIR",
                    "NEUTRAL B", "SIDE B", "UP B", "DOWN B"};

                print_string(module_accessor,
                             ATTACK_strings[ATTACK_STATE]);
            } else {
                if (ControlModule::check_button_on(module_accessor, CONTROL_PAD_BUTTON_APPEAL_S_L)) {
                    DI_STATE = DI_STATE == NONE ? DI_RANDOM_IN_AWAY : NONE;
                } else {
                    DI_STATE = DI_STATE == NONE ? SET_DI : NONE;
                }

                const char* DI_strings[NUM_DI_STATES] = 
                    {"NONE", "SET_DI", "RANDOM\nIN AWAY"};

                print_string(module_accessor, DI_strings[DI_STATE]);
                if (DI_STATE == SET_DI) {
                    DI_stick_x = ControlModule::get_stick_x(module_accessor);
                    DI_stick_y = ControlModule::get_stick_y(module_accessor);
                }
            }
        }
	} else if (motion_kind == hash40("appeal_hi_l") || motion_kind == hash40("appeal_hi_r")) {
		if (is_training_mode()) {
            HITBOX_VIS = !HITBOX_VIS;
            if (HITBOX_VIS)
                print_string(module_accessor, "HITBOX\nVIS");
            else
                print_string(module_accessor, "NO\nHITBOX");
        }
	}
}
}