#ifndef APP_SV_ANIMCMD_H
#define APP_SV_ANIMCMD_H

#include <switch.h>

namespace app::sv_animcmd {
	extern void wait(u64, float) asm("_ZN3app10sv_animcmd4waitEP9lua_Statef") LINKABLE;
	extern void frame(u64, float) asm("_ZN3app10sv_animcmd5frameEP9lua_Statef") LINKABLE;
	extern void is_excute(u64) asm("_ZN3app10sv_animcmd9is_excuteEP9lua_State") LINKABLE;
	
	extern u64 CATCH(u64) asm("_ZN3app10sv_animcmd5CATCHEP9lua_State") LINKABLE;
	extern u64 ATTACK(u64) asm("_ZN3app10sv_animcmd6ATTACKEP9lua_State") LINKABLE;
	extern u64 EFFECT(u64) asm("_ZN3app10sv_animcmd6EFFECTEP9lua_State") LINKABLE;
	extern u64 EFFECT_FOLLOW_NO_SCALE(u64) asm("_ZN3app10sv_animcmd22EFFECT_FOLLOW_NO_SCALEEP9lua_State") LINKABLE;
	extern u64 LAST_EFFECT_SET_COLOR(u64) asm("_ZN3app10sv_animcmd21LAST_EFFECT_SET_COLOREP9lua_State") LINKABLE;
	extern u64 LAST_EFFECT_SET_RATE(u64) asm("_ZN3app10sv_animcmd20LAST_EFFECT_SET_RATEEP9lua_State") LINKABLE;
	extern u64 SEARCH(u64) asm("_ZN3app10sv_animcmd6SEARCHEP9lua_State") LINKABLE;
	extern u64 HIT_NODE(u64) asm("_ZN3app10sv_animcmd8HIT_NODEEP9lua_State") LINKABLE;
	extern u64 ATK_POWER(u64) asm("_ZN3app10sv_animcmd9ATK_POWEREP9lua_State") LINKABLE;
	extern u64 REVERSE_LR(u64) asm("_ZN3app10sv_animcmd10REVERSE_LREP9lua_State") LINKABLE;
	extern u64 FT_MOTION_RATE(u64) asm("_ZN3app10sv_animcmd14FT_MOTION_RATEEP9lua_State") LINKABLE;
}

#endif // APP_SV_ANIMCMD_H
