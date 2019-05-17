#include <switch.h>

namespace app::sv_animcmd
{
    extern void frame(u64, float) asm("_ZN3app10sv_animcmd5frameEP9lua_Statef") LINKABLE;
    extern void is_excute(u64) asm("_ZN3app10sv_animcmd9is_excuteEP9lua_State") LINKABLE;
    extern u64 ATTACK(u64) asm("_ZN3app10sv_animcmd6ATTACKEP9lua_State") LINKABLE;
    extern u64 EFFECT(u64) asm("_ZN3app10sv_animcmd6EFFECTEP9lua_State") LINKABLE;
}
