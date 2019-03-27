#include <switch.h>
#include "saltysd_core.h"
#include "saltysd_ipc.h"
#include "saltysd_dynamic.h"
#include "acmd_imports.h"

__int64_t (*lib_L2CAgent_pop_lua_stack)(__int64_t, int) = NULL;

void get_lua_stack(__int64_t* l2c_agent, int index, __int64_t* l2c_val) {
    if (lib_L2CAgent_pop_lua_stack == NULL) 
        lib_L2CAgent_pop_lua_stack = (__int64_t (*)(__int64_t, int))(SaltySDCore_FindSymbol("_ZN3lib8L2CAgent13pop_lua_stackEi"));
	
	asm("mov x8, %x0" : : "r"(l2c_val) : "x8" );
    lib_L2CAgent_pop_lua_stack(l2c_agent, index);
}
