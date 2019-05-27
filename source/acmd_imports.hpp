#ifndef ACMD_IMPORTS_H
#define ACMD_IMPORTS_H

#include <switch.h>

namespace app::sv_animcmd {
	extern void wait(u64, float) asm("_ZN3app10sv_animcmd4waitEP9lua_Statef") LINKABLE;
	extern void frame(u64, float) asm("_ZN3app10sv_animcmd5frameEP9lua_Statef") LINKABLE;
	extern void is_excute(u64) asm("_ZN3app10sv_animcmd9is_excuteEP9lua_State") LINKABLE;
	extern u64 ATTACK(u64) asm("_ZN3app10sv_animcmd6ATTACKEP9lua_State") LINKABLE;
	extern u64 CATCH(u64) asm("_ZN3app10sv_animcmd5CATCHEP9lua_State") LINKABLE;

	extern u64 EFFECT(u64) asm("_ZN3app10sv_animcmd6EFFECTEP9lua_State") LINKABLE;
	extern u64 EFFECT_FOLLOW_NO_SCALE(u64) asm("_ZN3app10sv_animcmd22EFFECT_FOLLOW_NO_SCALEEP9lua_State") LINKABLE;
	extern u64 LAST_EFFECT_SET_COLOR(u64) asm("_ZN3app10sv_animcmd21LAST_EFFECT_SET_COLOREP9lua_State") LINKABLE;
	extern u64 LAST_EFFECT_SET_RATE(u64) asm("_ZN3app10sv_animcmd20LAST_EFFECT_SET_RATEEP9lua_State") LINKABLE;
}

#endif // ACMD_IMPORTS_H
