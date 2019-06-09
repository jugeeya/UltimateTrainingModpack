#pragma once

namespace app::lua_bind {
    namespace ShadowModule {
        u64 set_draw(u64,int shadowDrawFlag,bool) asm("_ZN3app8lua_bind27ShadowModule__set_draw_implEPNS_26BattleObjectModuleAccessorENS_14ShadowDrawFlagEb") LINKABLE;
        u64 set_draw_status(u64,bool) asm("_ZN3app8lua_bind34ShadowModule__set_draw_status_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_offset_y(u64,float) asm("_ZN3app8lua_bind31ShadowModule__set_offset_y_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
    }
}