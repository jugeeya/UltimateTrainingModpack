#include <switch.h>

#include <stdint.h>

#include "useful/crc32.h"
#include "useful/useful.h"

#include "useful/const_value_table.h"
#include "useful/raygun_printer.hpp"

#include "acmd_wrapper.hpp"
#include "imports/lib/l2c.hpp"

#include "taunt_toggles.h"

using namespace lib;
using namespace app::lua_bind;

void sv_replace_status_func(u64 l2c_agentbase, int status_kind, u64 key,
                            void* func);

u64 appeal_lw_replace(L2CAgent* l2c_agent, void* variadic);
u64 appeal_hi_replace(L2CAgent* l2c_agent, void* variadic);
u64 appeal_s_replace(L2CAgent* l2c_agent, void* variadic);
u64 pre_GuardDamage_replace(L2CAgent* l2c_fighter, L2CAgent* l2c_agent);

void replace_scripts(L2CAgent* l2c_agent, u8 category, int kind) {
    // fighter
    if (category == BATTLE_OBJECT_CATEGORY_FIGHTER) {
        // taunt toggles
        l2c_agent->sv_set_function_hash(&appeal_lw_replace,
                                        hash40("effect_appeallwl"));
        l2c_agent->sv_set_function_hash(&appeal_lw_replace,
                                        hash40("effect_appeallwr"));
        l2c_agent->sv_set_function_hash(&appeal_hi_replace,
                                        hash40("effect_appealhil"));
        l2c_agent->sv_set_function_hash(&appeal_hi_replace,
                                        hash40("effect_appealhir"));
        l2c_agent->sv_set_function_hash(&appeal_s_replace,
                                        hash40("effect_appealsl"));
        l2c_agent->sv_set_function_hash(&appeal_s_replace,
                                        hash40("effect_appealsr"));
    }
}

u64 appeal_lw_replace(L2CAgent* l2c_agent, void* variadic) {
    ACMD acmd = ACMD(l2c_agent);

    acmd.frame(1);
    if (acmd.is_excute()) {
        if (is_training_mode()) {
            if (TOGGLE_STATE == MASH_TOGGLES) {
                MASH_STATE = (MASH_STATE + 1) % NUM_MASH_STATES;
                const char* toggle_strings[NUM_MASH_STATES] = {
                    "NONE", "AIRDODGE", "JUMP", "RANDOM"};

                print_string(acmd.module_accessor, toggle_strings[MASH_STATE]);
            }

            if (TOGGLE_STATE == ESCAPE_TOGGLES) {
                ESCAPE_STATE = (ESCAPE_STATE + 1) % NUM_ESCAPE_STATES;
                const char* toggle_strings[NUM_ESCAPE_STATES] = {
                    "NONE", "LEDGE"};

                print_string(acmd.module_accessor, toggle_strings[ESCAPE_STATE]);
            }

            if (TOGGLE_STATE == SHIELD_TOGGLES) {
                SHIELD_STATE = (SHIELD_STATE + 1) % NUM_SHIELD_STATES;
                const char* toggle_strings[NUM_SHIELD_STATES] = {
                    "NONE", "INFINITE", "HOLD"};

                print_string(acmd.module_accessor, toggle_strings[SHIELD_STATE]);
            }


        }
    }

    return 0;
}

u64 appeal_hi_replace(L2CAgent* l2c_agent, void* variadic) {
    ACMD acmd = ACMD(l2c_agent);

    acmd.frame(1);
    if (acmd.is_excute()) {
        if (is_training_mode()) {
            HITBOX_VIS = !HITBOX_VIS;
            if (HITBOX_VIS)
                print_string(acmd.module_accessor, "HITBOX\nVIS");
            else
                print_string(acmd.module_accessor, "NO\nHITBOX");
        }
    }

    return 0;
}

u64 appeal_s_replace(L2CAgent* l2c_agent, void* variadic) {
    ACMD acmd = ACMD(l2c_agent);

    acmd.frame(1);
    if (acmd.is_excute()) {
        if (is_training_mode()) {
            if (TOGGLE_STATE == ESCAPE_TOGGLES && ESCAPE_STATE == ESCAPE_LEDGE) {
                LEDGE_STATE = (LEDGE_STATE + 1) % NUM_LEDGE_STATES;
                const char* LEDGE_strings[NUM_LEDGE_STATES] = {
                    "RANDOM", "NORMAL", "ROLL", "JUMP", "ATTACK"};

                print_string(acmd.module_accessor, LEDGE_strings[LEDGE_STATE]);
            } else if (MASH_STATE == MASH_ATTACK) {
                ATTACK_STATE = (ATTACK_STATE + 1) % NUM_ATTACK_STATES;
                const char* ATTACK_strings[NUM_ATTACK_STATES] = {
                    "NAIR",      "FAIR",   "BAIR", "UPAIR", "DAIR",
                    "NEUTRAL B", "SIDE B", "UP B", "DOWN B"};

                print_string(acmd.module_accessor,
                             ATTACK_strings[ATTACK_STATE]);
            } else {
                if (ControlModule::check_button_on(acmd.module_accessor, CONTROL_PAD_BUTTON_APPEAL_S_L)) {
                    DI_STATE = DI_STATE == NONE ? DI_RANDOM_IN_AWAY : NONE; 
                } else {
                    DI_STATE = DI_STATE == NONE ? SET_DI : NONE; 
                }

                const char* DI_strings[NUM_DI_STATES] = {
                    "NONE", "SET_DI", "RANDOM\nIN AWAY"};

                print_string(acmd.module_accessor, DI_strings[DI_STATE]);
                if (DI_STATE == SET_DI) {
                    DI_stick_x = ControlModule::get_stick_x(acmd.module_accessor);
                    DI_stick_y = ControlModule::get_stick_y(acmd.module_accessor);
                }
            }
        }
    }

    return 0;
}

void* sv_get_status_func(u64 l2c_agentbase, int status_kind, u64 key) {
    u64 unk48 = LOAD64(l2c_agentbase + 0x48);
    u64 unk50 = LOAD64(l2c_agentbase + 0x50);
    if (0x2E8BA2E8BA2E8BA3LL * ((unk50 - unk48) >> 4) > (u64)status_kind)
        return *(void**)(unk48 + 0xB0LL * status_kind + (key << 32 >> 29));

    return 0;
}

void sv_replace_status_func(u64 l2c_agentbase, int status_kind, u64 key,
                            void* func) {
    u64 unk48 = LOAD64(l2c_agentbase + 0x48);
    u64 unk50 = LOAD64(l2c_agentbase + 0x50);
    if (0x2E8BA2E8BA2E8BA3LL * ((unk50 - unk48) >> 4) > (u64)status_kind) {
        *(void**)(unk48 + 0xB0LL * status_kind + (key << 32 >> 29)) = func;
    }
}

u64 clear_lua_stack_replace(u64 l2c_agent) {
    u64 lua_state = LOAD64(l2c_agent + 8);
    if ((lua_state - 8) && LOAD64(lua_state - 8) &&
        (LOAD64(LOAD64(lua_state - 8) + 416LL))) {
        u8 battle_object_category = *(u8*)(LOAD64(lua_state - 8) + 404LL);
        int battle_object_kind = *(int*)(LOAD64(lua_state - 8) + 408LL);
        replace_scripts((L2CAgent*)l2c_agent, battle_object_category,
                        battle_object_kind);
    }

    // Original clear_lua_stack:
    u64 v1 = LOAD64(l2c_agent + 8);
    u64 v2 = LOAD64(v1 + 16);
    u64 i = LOAD64(LOAD64(v1 + 32)) + 16LL;
    for (; v2 < i; v2 = LOAD64(v1 + 16)) {
        LOAD64(v1 + 16) = v2 + 16;
        *(u32*)(v2 + 8) = 0;
    }
    LOAD64(v1 + 16) = i;
    return l2c_agent;
}
