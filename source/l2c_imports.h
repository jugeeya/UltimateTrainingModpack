#include <switch.h>
#include "l2c.h"

// lib::L2CAgent::L2CAgent(L2CAgent*, lua_State *)
//u64 (*lib_L2CAgent)(u64*, u64);
#define lib_L2CAgent _ZN3lib8L2CAgentC2EP9lua_State
extern u64 _ZN3lib8L2CAgentC2EP9lua_State(L2CAgent* unk1, u64 unk2) LINKABLE;

// L2CAgent *__fastcall lib::L2CAgent::push_lua_stack(L2CAgent *result, const lib::L2CValue *a2)
//u64 (*lib_L2CAgent_push_lua_stack)(u64, const u64*);
#define lib_L2CAgent_push_lua_stack _ZN3lib8L2CAgent14push_lua_stackERKNS_8L2CValueE
extern u64 _ZN3lib8L2CAgent14push_lua_stackERKNS_8L2CValueE(L2CAgent* unk1, const L2CValue* unk2) LINKABLE;

// pop_lua_stack
// Notes: 
// Actually takes three arguments, but the third is given through X8 due to how 
// AArch64 treats struct pointers that are used as pass by reference to get the value.
// Thus, my current solution is to use inline ASM before using this to pass the 
// last arg. This is done using asm("mov x8, %x0" : : "r"(&popped) : "x8" );, where
// popped is an L2CValue that will be populated by the function.
// FURTHERMORE, this function does NOT actually pop the stack, it only returns the value at the
// position indicated by the second argument.
// This index is either positive, meaning absolute position in the stack, or negative,
// which is more traditional, i.e. -1 is the top of the stack.
//u64 (*lib_L2CAgent_pop_lua_stack)(u64, int);
#define lib_L2CAgent_pop_lua_stack _ZN3lib8L2CAgent13pop_lua_stackEi
extern u64 _ZN3lib8L2CAgent13pop_lua_stackEi(L2CAgent* unk1, int unk2) LINKABLE;

// L2CAgent *__fastcall lib::L2CAgent::clear_lua_stack(L2CAgent *result)
// u64 (*lib_L2CAgent_clear_lua_stack)(u64);
#define lib_L2CAgent_clear_lua_stack _ZN3lib8L2CAgent15clear_lua_stackEv
extern u64 _ZN3lib8L2CAgent15clear_lua_stackEv(L2CAgent* unk1) LINKABLE;

#define lib_utility_Variadic_get_format _ZN3lib7utility8VariadicC1Ev
extern u64 _ZN3lib7utility8VariadicC1Ev(u64 unk1) LINKABLE;

#define lib_L2CValue_push_variadic _ZN3lib8L2CValue13push_variadicEmPKcRNS_7utility8VariadicE
extern u64 _ZN3lib8L2CValue13push_variadicEmPKcRNS_7utility8VariadicE(u64 unk1, u64 unk2, u64 unk3, u64 unk4) LINKABLE;

#define lib_L2CValue_del_L2CValue _ZN3lib8L2CValueD1Ev
extern u64 _ZN3lib8L2CValueD1Ev(u64 unk1) LINKABLE;
