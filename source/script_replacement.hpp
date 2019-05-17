#include <switch.h>

#include <stdint.h>

#include "crc32.h"

#include "l2c.hpp"
#include "l2c_imports.hpp"
#include "acmd_wrapper.hpp"
#include "lua_helper.hpp"

#define LOAD64 *(u64 *)

using namespace lib;
using namespace app::lua_bind;

u64 shine_replace(L2CAgent* l2c_agent, void* variadic);

void replace_scripts(L2CAgent* l2c_agent, u8 category, uint kind) {
    // fighter
    if (category == CONST_VALUE("BATTLE_OBJECT_CATEGORY_FIGHTER")) {
        // fox
        if (kind == CONST_VALUE("FIGHTER_KIND_FOX")) {
            l2c_agent->sv_set_function_hash(&shine_replace, hash40("game_speciallwstart"));
            l2c_agent->sv_set_function_hash(&shine_replace, hash40("game_specialairlwstart"));
        }

        // peach
        if (kind == CONST_VALUE("FIGHTER_KIND_PEACH")) {
        }
    }
}

// AnimCMD replacement function
u64 shine_replace(L2CAgent* l2c_agent, void* variadic) {
  ACMD acmd = ACMD{.l2c_agent = l2c_agent};

  acmd.frame(1);
  if (acmd.is_excute()) {
    acmd.ATTACK(0, 0, hash40("top"), 10.0, 10, 32, 0, 66, 7.5, 0, 6.5,
                // 0, 0, 0, //L2C_voids: no X2, Y2, Z2
                0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 63, 31, 0,
                0x13462FCFE4LL, 2, 7, 25);
  }

  return 0;
}

void* sv_get_status_func(u64 l2c_agentbase, int status_kind, u64 key) {
  u64 unk48 = LOAD64(l2c_agentbase + 0x48);
  u64 unk50 = LOAD64(l2c_agentbase + 0x50);
  if ( 0x2E8BA2E8BA2E8BA3LL * ((unk50 - unk48) >> 4) > (u64)status_kind)
    return *(void **)(unk48 + 0xB0LL * status_kind + (key << 32 >> 29));
  
  return 0;
}

void sv_replace_status_func(u64 l2c_agentbase, int status_kind, u64 key, void* func) {
  u64 unk48 = LOAD64(l2c_agentbase + 0x48);
  u64 unk50 = LOAD64(l2c_agentbase + 0x50);
  if ( 0x2E8BA2E8BA2E8BA3LL * ((unk50 - unk48) >> 4) > (u64)status_kind) {
    *(void **)(unk48 + 0xB0LL * status_kind + (key << 32 >> 29)) = func;
  }
}

u64 clear_lua_stack_replace(u64 l2c_agent) {
  u64 lua_state = LOAD64(l2c_agent + 8);
  if (lua_state-8 && LOAD64(lua_state-8) && LOAD64(LOAD64(lua_state - 8) + 416LL)) {
    u8 battle_object_category = *(u8 *)(LOAD64(lua_state - 8) + 404LL);
    uint battle_object_kind = *(uint *)(LOAD64(lua_state - 8) + 408LL);
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
