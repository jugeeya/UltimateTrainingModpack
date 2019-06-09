#pragma once

namespace app::lua_bind {
    namespace AbsorberModule {
        u64 set_status(u64,int,int shieldStatus,int) asm("_ZN3app8lua_bind31AbsorberModule__set_status_implEPNS_26BattleObjectModuleAccessorEiNS_12ShieldStatusEi") LINKABLE;
    }
}