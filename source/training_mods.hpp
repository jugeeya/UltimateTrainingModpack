#ifndef TRAINING_MODS_H
#define TRAINING_MODS_H

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif
#include <stdarg.h>
#include "useful/const_value_table.h"
#include "useful/crc32.h"
#include "useful/useful.h"

#include "useful/raygun_printer.hpp"

#include "acmd_wrapper.hpp"
#include "imports/lib/l2c.hpp"
#include "saltysd/saltysd_dynamic.h"
#include "saltysd/saltysd_helper.hpp"
#include "taunt_toggles.h"

using namespace lib;
using namespace app::lua_bind;
using namespace app::sv_animcmd;

u64 fighter_manager_addr;

bool is_operation_cpu(u64 module_accessor) {
    int entry_id = WorkModule::get_int(module_accessor,
                                       FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    u64 fighter_information = FighterManager::get_fighter_information(
        LOAD64(fighter_manager_addr), entry_id);

    return FighterInformation::is_operation_cpu(fighter_information);
}

bool is_in_hitstun(u64 module_accessor) {
    int status_kind = StatusModule::status_kind(module_accessor);
    return status_kind >= FIGHTER_STATUS_KIND_DAMAGE &&
           status_kind <= FIGHTER_STATUS_KIND_DAMAGE_FALL;
}

bool is_in_landing(u64 module_accessor) {
    int status_kind = StatusModule::status_kind(module_accessor);
    return status_kind >= FIGHTER_STATUS_KIND_LANDING &&
           status_kind <= FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT;
}

namespace app::lua_bind {
namespace WorkModule {
// Force DI
float get_float_replace(u64 module_accessor, int var) {
    // call original WorkModule::get_float_impl
    u64 work_module = load_module(module_accessor, 0x50);
    float (*get_float)(u64, int) =
        (float (*)(u64, int))(load_module_impl(work_module, 0x58));

    float ret_val = get_float(work_module, var);

    if (var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X ||
        var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y) {
        if (is_training_mode() && is_operation_cpu(module_accessor) &&
            is_in_hitstun(module_accessor)) {
            if (DI_STATE != NONE) {
                float stick_x = 0.0, stick_y = 0.0;
                if (DI_STATE == SET_DI) {
                    stick_x = DI_stick_x;
                    stick_y = DI_stick_y;
                } else if (DI_STATE == DI_RANDOM_IN_AWAY) {
                    // either 1.0 or -1.0
                    stick_x = (float)(app::sv_math::rand(hash40("fighter"), 2) * 2.0) - 1;
                    stick_y = 0.0;
                }

                // If facing left, reverse stick x
                if (var ==
                    FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X)
                    return stick_x * -1 * PostureModule::lr(module_accessor);
                if (var ==
                    FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y)
                    return stick_y;
            }
        }
    }

    return ret_val;
}

float get_param_float_replace(u64 module_accessor, u64 param_type,
                              u64 param_hash) {
    if (is_training_mode()) {
        if (SHIELD_STATE == SHIELD_INFINITE) {
            if (param_type == hash40("common")) {
                if (param_hash == hash40("shield_dec1")) return 0.0;
                if (param_hash == hash40("shield_recovery1")) return 999.0;
                // doesn't work, somehow. This parameter isn't checked?
                if (param_hash == hash40("shield_damage_mul")) return 0.0;
            }
        }
    }

    // call original
    u64 work_module = load_module(module_accessor, 0x50);
    float (*get_param_float)(u64, u64, u64) =
        (float (*)(u64, u64, u64))(load_module_impl(work_module, 0x240));

    return get_param_float(work_module, param_type, param_hash);
}

// Force ledge option
u64 enable_transition_term_replace(u64 module_accessor, int transition_id) {
    if (ESCAPE_STATE == ESCAPE_LEDGE && is_training_mode() &&
        is_operation_cpu(module_accessor)) {
        if (StatusModule::status_kind(module_accessor) ==
            FIGHTER_STATUS_KIND_CLIFF_WAIT) {
            if (transition_id ==
                FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB) {
                int status = 0;
                int ledge_case = LEDGE_STATE;

                if (LEDGE_STATE == RANDOM_LEDGE)
                    ledge_case = app::sv_math::rand(hash40("fighter"), 4) + 1;

                switch (ledge_case) {
                    case NEUTRAL_LEDGE:
                        status = FIGHTER_STATUS_KIND_CLIFF_CLIMB;
                        break;
                    case ROLL_LEDGE:
                        status = FIGHTER_STATUS_KIND_CLIFF_ESCAPE;
                        break;
                    case JUMP_LEDGE:
                        status = FIGHTER_STATUS_KIND_CLIFF_JUMP1;
                        break;
                    case ATTACK_LEDGE:
                        status = FIGHTER_STATUS_KIND_CLIFF_ATTACK;
                        break;
                }

                StatusModule::change_status_request_from_script(module_accessor,
                                                                status, 1);
            }
        }
    }

    // call original WorkModule::enable_transition_term_group_impl
    u64 work_module = load_module(module_accessor, 0x50);
    u64 (*enable_transition_term)(u64, int) =
        (u64(*)(u64, int))(load_module_impl(work_module, 0x188));

    return enable_transition_term(work_module, transition_id);
}
}  // namespace WorkModule

namespace ControlModule {
int get_attack_air_kind_replace(u64 module_accessor) {
    // call original
    u64 control_module = load_module(module_accessor, 0x48);
    int (*get_attack_air_kind)(u64) =
        (int (*)(u64))load_module_impl(control_module, 0x3B0);
    int kind = get_attack_air_kind(control_module);

    if (is_training_mode() && is_operation_cpu(module_accessor)) {
        if (MASH_STATE == MASH_ATTACK) {
            switch (ATTACK_STATE) {
                case MASH_NAIR:
                    kind = FIGHTER_COMMAND_ATTACK_AIR_KIND_N;
                    break;
                case MASH_FAIR:
                    kind = FIGHTER_COMMAND_ATTACK_AIR_KIND_F;
                    break;
                case MASH_BAIR:
                    kind = FIGHTER_COMMAND_ATTACK_AIR_KIND_B;
                    break;
                case MASH_UPAIR:
                    kind = FIGHTER_COMMAND_ATTACK_AIR_KIND_HI;
                    break;
                case MASH_DAIR:
                    kind = FIGHTER_COMMAND_ATTACK_AIR_KIND_LW;
                    break;
            }
        }

        if (MASH_STATE == MASH_RANDOM) {
            kind = app::sv_math::rand(hash40("fighter"), 5) + 1;
        }
    }

    return kind;
}

int get_command_flag_cat_replace(u64 module_accessor, int category) {
    // call original
    u64 control_module = load_module(module_accessor, 0x48);
    int (*get_command_flag_cat)(u64, int) =
        (int (*)(u64, int))load_module_impl(control_module, 0x350);
    int flag = get_command_flag_cat(control_module, category);

    if (is_training_mode() && is_operation_cpu(module_accessor)) {
        if (is_in_hitstun(module_accessor) || is_in_landing(module_accessor)) {
            if (MASH_STATE == MASH_AIRDODGE)
                if (category == FIGHTER_PAD_COMMAND_CATEGORY1)
                    flag |= FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE;

            if (MASH_STATE == MASH_JUMP)
                if (category == FIGHTER_PAD_COMMAND_CATEGORY1)
                    flag |= FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON;

            if (MASH_STATE == MASH_ATTACK)
                if (category == FIGHTER_PAD_COMMAND_CATEGORY1) {
                    switch (ATTACK_STATE) {
                        case MASH_NAIR:
                        case MASH_FAIR:
                        case MASH_BAIR:
                        case MASH_UPAIR:
                        case MASH_DAIR:
                            flag |= FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N;
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
                            break;
                    }
                }

            if (MASH_STATE == MASH_RANDOM)
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

    return flag;
}

bool check_button_on_replace(u64 module_accessor, int button) {
    if (button == CONTROL_PAD_BUTTON_GUARD_HOLD ||
        button == CONTROL_PAD_BUTTON_GUARD) {
        if (is_training_mode() && is_operation_cpu(module_accessor)) {
            if (SHIELD_STATE == SHIELD_HOLD || SHIELD_STATE == SHIELD_INFINITE)
                return true;
            if (MASH_STATE == MASH_AIRDODGE &&
                (is_in_hitstun(module_accessor) ||
                 is_in_landing(module_accessor)))
                return true;
        }
    }

    // call original
    u64 control_module = load_module(module_accessor, 0x48);
    bool (*check_button_on)(u64, int) =
        (bool (*)(u64, int))load_module_impl(control_module, 0x260);
    return check_button_on(control_module, button);
}

bool check_button_off_replace(u64 module_accessor, int button) {
    if (button == CONTROL_PAD_BUTTON_GUARD_HOLD ||
        button == CONTROL_PAD_BUTTON_GUARD) {
        if (is_training_mode() && is_operation_cpu(module_accessor)) {
            if (SHIELD_STATE == SHIELD_HOLD || SHIELD_STATE == SHIELD_INFINITE)
                return false;
        }
    }

    // call original
    u64 control_module = load_module(module_accessor, 0x48);
    bool (*check_button_off)(u64, int) =
        (bool (*)(u64, int))load_module_impl(control_module, 0x268);
    return check_button_off(control_module, button);
}

#define SAVE_STATE 1
#define DEFAULT 2
#define CAMERA_MOVE 3
#define POS_MOVE 4

int save_state_player_state = DEFAULT;
int save_state_cpu_state = DEFAULT;
bool save_state_move_alert = false;

float save_state_x_player = 0;
float save_state_y_player = 0;
float save_state_percent_player = 0;
float save_state_lr_player = 1.0;
int save_state_situation_kind_player = 0;

float save_state_x_cpu = 0;
float save_state_y_cpu = 0;
float save_state_percent_cpu = 0;
float save_state_lr_cpu = 1.0;
int save_state_situation_kind_cpu = 0;

int get_pad_flag_replace(u64 module_accessor) {
    if (is_training_mode()) {
        float* save_state_x;
        float* save_state_y;
        float* save_state_percent;
        float* save_state_lr;
        int* save_state_situation_kind;
        int* save_state;
        if (is_operation_cpu(module_accessor)) {
            save_state_x = &save_state_x_cpu;
            save_state_y = &save_state_y_cpu;
            save_state_percent = &save_state_percent_cpu;
            save_state_lr = &save_state_lr_cpu;
            save_state_situation_kind = &save_state_situation_kind_cpu;
            save_state = &save_state_cpu_state;
        } else {
            save_state_x = &save_state_x_player;
            save_state_y = &save_state_y_player;
            save_state_percent = &save_state_percent_player;
            save_state_lr = &save_state_lr_player;
            save_state_situation_kind = &save_state_situation_kind_player;
            save_state = &save_state_player_state;
        }

        // Grab + Dpad up: reset state
        if (ControlModule::check_button_on(module_accessor,
                                           CONTROL_PAD_BUTTON_CATCH) &&
            ControlModule::check_button_trigger(module_accessor,
                                                CONTROL_PAD_BUTTON_APPEAL_HI)) {
            if (*save_state == DEFAULT) {
                save_state_player_state = CAMERA_MOVE;
                save_state_cpu_state = CAMERA_MOVE;
            }
        }

        // move to camera bounds
        if (*save_state == CAMERA_MOVE) {
            *save_state = POS_MOVE;

            float left_right = (*save_state_x > 0) - (*save_state_x < 0);
            float y_pos = 0;
            if (*save_state_situation_kind == SITUATION_KIND_GROUND)
                y_pos = -50;

            Vector3f pos = {.x = left_right * 50, .y = y_pos, .z = 0};
            PostureModule::set_pos(module_accessor, &pos);
            StatusModule::set_situation_kind(module_accessor,
                                             SITUATION_KIND_AIR, 0);
        }

        // move to correct pos
        if (*save_state == POS_MOVE) {
            *save_state = DEFAULT;

            Vector3f pos = {.x = *save_state_x, .y = *save_state_y, .z = 0};
            PostureModule::set_pos(module_accessor, &pos);
            PostureModule::set_lr(module_accessor, *save_state_lr);
            DamageModule::add_damage(
                module_accessor,
                -1.0 * DamageModule::damage(module_accessor, 0), 0);
            DamageModule::add_damage(module_accessor, *save_state_percent, 0);
            StatusModule::set_situation_kind(module_accessor,
                                             *save_state_situation_kind, 0);

            // Doesn't work, and I don't know why yet.
            /*if (*save_state_situation_kind == SITUATION_KIND_GROUND)
                    StatusModule::change_status_request(module_accessor,
            FIGHTER_STATUS_KIND_WAIT, 0); else if (*save_state_situation_kind ==
            SITUATION_KIND_AIR)
                    StatusModule::change_status_request(module_accessor,
            FIGHTER_STATUS_KIND_FALL, 0); else if (*save_state_situation_kind ==
            SITUATION_KIND_CLIFF)
                    StatusModule::change_status_request(module_accessor,
            FIGHTER_STATUS_KIND_CLIFF_CATCH, 0);
            */
        }

        // Grab + Dpad down: Save state
        if (ControlModule::check_button_on(module_accessor,
                                           CONTROL_PAD_BUTTON_CATCH) &&
            ControlModule::check_button_trigger(module_accessor,
                                                CONTROL_PAD_BUTTON_APPEAL_LW)) {
            save_state_player_state = SAVE_STATE;
            save_state_cpu_state = SAVE_STATE;
        }

        if (*save_state == SAVE_STATE) {
            *save_state = DEFAULT;

            *save_state_x = PostureModule::pos_x(module_accessor);
            *save_state_y = PostureModule::pos_y(module_accessor);
            *save_state_lr = PostureModule::lr(module_accessor);
            *save_state_percent = DamageModule::damage(module_accessor, 0);
            *save_state_situation_kind =
                StatusModule::situation_kind(module_accessor);
        }
    }

    // call original
    u64 control_module = load_module(module_accessor, 0x48);
    int (*get_pad_flag)(u64) =
        (int (*)(u64))load_module_impl(control_module, 0x348);
    return get_pad_flag(control_module);
}
}  // namespace ControlModule
}  // namespace app::lua_bind

extern int vsnprintf(char* s, size_t maxlen, const char* format, va_list arg) LINKABLE;

int vsnprintf_intercept(char* s, size_t maxlen, const char* format, va_list arg) {
    if (strcmp(format, "mel_training_help_invincible") == 0) {
        HITBOX_VIS = true;
    } else if (strcmp(format, "mel_training_help_invincible_off") == 0) {
        HITBOX_VIS = false;
    } else if (strcmp(format, "mel_training_help_shift0") == 0) {
        TOGGLE_STATE = MASH_TOGGLES;
        if (MASH_STATE == NONE)
            format = "mel_shortmsg_1";
        if (MASH_STATE == MASH_AIRDODGE)
            format = "mel_shortmsg_2";
        if (MASH_STATE == MASH_JUMP)
            format = "mel_shortmsg_3";
        if (MASH_STATE == MASH_RANDOM)
            format = "mel_shortmsg_4";
    } else if (strcmp(format, "mel_training_help_shift1") == 0) {
        TOGGLE_STATE = ESCAPE_TOGGLES;
        if (ESCAPE_STATE == NONE)
            format = "mel_shortmsg_5";
        if (ESCAPE_STATE == ESCAPE_LEDGE)
            format = "mel_shortmsg_6";
    } else if (strcmp(format, "mel_training_help_shift2") == 0) {
        TOGGLE_STATE = SHIELD_TOGGLES;
        if (SHIELD_STATE == NONE)
            format = "mel_shortmsg_7";
        if (SHIELD_STATE == SHIELD_INFINITE)
            format = "mel_shortmsg_8";
        if (SHIELD_STATE == SHIELD_HOLD)
            format = "mel_shortmsg_9";
    }

    int ret = vsnprintf(s, maxlen, format, arg);
    return ret;
}

void training_mods_main() {
    fighter_manager_addr = SaltySDCore_FindSymbol(
        "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E");
    // Mash airdodge/jump
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind40ControlModule__get_command_flag_cat_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&ControlModule::get_command_flag_cat_replace);

    // Set DI
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind26WorkModule__get_float_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&WorkModule::get_float_replace);

    // Hold/Infinite shield
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind35ControlModule__check_button_on_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&ControlModule::check_button_on_replace);
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind36ControlModule__check_button_off_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&ControlModule::check_button_off_replace);
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind32WorkModule__get_param_float_implEPNS_26BattleObjectModuleAccessorEmm",
        (u64)&WorkModule::get_param_float_replace);

    // Ledge options
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind39WorkModule__enable_transition_term_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&WorkModule::enable_transition_term_replace);

    // Mash attack
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind39ControlModule__get_attack_air_kind_implEPNS_26BattleObjectModuleAccessorE",
        (u64)&ControlModule::get_attack_air_kind_replace);

    // Save states: in beta
    /*SaltySD_function_replace_sym(
            "_ZN3app8lua_bind32ControlModule__get_pad_flag_implEPNS_26BattleObjectModuleAccessorE",
            (u64)&ControlModule::get_pad_flag_replace);*/

    // Menu replacements
    SaltySDCore_ReplaceImport("vsnprintf", (void*)vsnprintf_intercept);
}

#endif  // TRAINING_MODS_H
