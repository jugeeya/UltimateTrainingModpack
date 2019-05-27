#include <switch.h>

#include <stdint.h>

#include "crc32.h"

#include "l2c.hpp"
#include "l2c_imports.hpp"
#include "acmd_wrapper.hpp"

#include "raygun_printer.hpp"

#include "const_value_table.h"
#include "taunt_toggles.h"

#include <vector>

using namespace lib;
using namespace app::lua_bind;

u64 appeal_lw_replace(L2CAgent* l2c_agent, void* variadic);
u64 appeal_hi_replace(L2CAgent* l2c_agent, void* variadic);
u64 appeal_s_replace(L2CAgent* l2c_agent, void* variadic);

void replace_scripts(L2CAgent* l2c_agent, u8 category, int kind) {
    // fighter
    if (category == BATTLE_OBJECT_CATEGORY_FIGHTER) {
        l2c_agent->sv_set_function_hash(&appeal_lw_replace, hash40("effect_appeallwl"));
        l2c_agent->sv_set_function_hash(&appeal_lw_replace, hash40("effect_appeallwr"));
        l2c_agent->sv_set_function_hash(&appeal_hi_replace, hash40("effect_appealhil"));
        l2c_agent->sv_set_function_hash(&appeal_hi_replace, hash40("effect_appealhir"));
        l2c_agent->sv_set_function_hash(&appeal_s_replace, hash40("effect_appealsl"));
        l2c_agent->sv_set_function_hash(&appeal_s_replace, hash40("effect_appealsr"));
    }
}

u64 appeal_lw_replace(L2CAgent* l2c_agent, void* variadic) {
    ACMD acmd = ACMD(l2c_agent);
  
    acmd.frame(1);
    if (acmd.is_excute()) {
		TOGGLE_STATE = (TOGGLE_STATE + 1) % NUM_TOGGLE_STATES;
		if (TOGGLE_STATE == MASH_AIRDODGE)
		    print_string(acmd.module_accessor, "MASH\nAIRDODGE");
		else if (TOGGLE_STATE == MASH_JUMP)
		    print_string(acmd.module_accessor, "MASH\nJUMP");
		else
		    print_string(acmd.module_accessor, "NONE");
    }

    return 0;
}

u64 appeal_hi_replace(L2CAgent* l2c_agent, void* variadic) {
	ACMD acmd = ACMD(l2c_agent);

	acmd.frame(1);
	if (acmd.is_excute()) {
		HITBOX_VIS = !HITBOX_VIS;
		if (HITBOX_VIS)
			print_string(acmd.module_accessor, "HITBOX\nVIS");
		else
			print_string(acmd.module_accessor, "NO\nHITBOX");
	}

	return 0;
}

u64 appeal_s_replace(L2CAgent* l2c_agent, void* variadic) {
	ACMD acmd = ACMD(l2c_agent);

	acmd.frame(1);
	if (acmd.is_excute()) {
		DI_STATE = (DI_STATE + 1) % NUM_DI_STATES;
		const char* DI_strings[NUM_DI_STATES] = {"NONE", "AWAY", "DOWN AWAY", "DOWN", "DOWN IN",
			"IN", "UP IN", "UP", "UP AWAY", "RANDOM\nIN AWAY"};
		print_string(acmd.module_accessor, DI_strings[DI_STATE]);
	}

	return 0;
}

void* sv_get_status_func(u64 l2c_agentbase, int status_kind, u64 key) {
	u64 unk48 = LOAD64(l2c_agentbase + 0x48);
	u64 unk50 = LOAD64(l2c_agentbase + 0x50);
	if (0x2E8BA2E8BA2E8BA3LL * ((unk50 - unk48) >> 4) > (u64)status_kind)
		return *(void **)(unk48 + 0xB0LL * status_kind + (key << 32 >> 29));

	return 0;
}

void sv_replace_status_func(u64 l2c_agentbase, int status_kind, u64 key, void* func) {
	u64 unk48 = LOAD64(l2c_agentbase + 0x48);
	u64 unk50 = LOAD64(l2c_agentbase + 0x50);
	if (0x2E8BA2E8BA2E8BA3LL * ((unk50 - unk48) >> 4) > (u64)status_kind) {
		*(void **)(unk48 + 0xB0LL * status_kind + (key << 32 >> 29)) = func;
	}
}

u64 clear_lua_stack_replace(u64 l2c_agent) {
	u64 lua_state = LOAD64(l2c_agent + 8);
	if ((lua_state - 8) && LOAD64(lua_state - 8) && (LOAD64(LOAD64(lua_state - 8) + 416LL))) {
		u8 battle_object_category = *(u8 *)(LOAD64(lua_state - 8) + 404LL);
		int battle_object_kind = *(int *)(LOAD64(lua_state - 8) + 408LL);
		replace_scripts((L2CAgent*)l2c_agent, battle_object_category, battle_object_kind);
	}

	// Original clear_lua_stack:
	u64 v1 = LOAD64(l2c_agent + 8);
	u64 v2 = LOAD64(v1 + 16);
	u64 i = LOAD64(LOAD64(v1 + 32)) + 16LL;
	for (; v2 < i; v2 = LOAD64(v1 + 16)) {
		LOAD64(v1 + 16) = v2 + 16;
		*(u32 *)(v2 + 8) = 0;
	}
	LOAD64(v1 + 16) = i;
	return l2c_agent;
}
