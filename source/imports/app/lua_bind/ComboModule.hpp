#pragma once

namespace app::lua_bind {
    namespace ComboModule {
        u64 count(u64) asm("_ZN3app8lua_bind23ComboModule__count_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 reset(u64) asm("_ZN3app8lua_bind23ComboModule__reset_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set(u64,int) asm("_ZN3app8lua_bind21ComboModule__set_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
    }
}