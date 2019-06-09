#pragma once

namespace app::lua_bind {
    namespace FighterWorkModuleImpl {
        u64 calc_escape_air_slide_param(u64,float) asm("_ZN3app8lua_bind55FighterWorkModuleImpl__calc_escape_air_slide_param_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 calc_param(u64,bool,bool) asm("_ZN3app8lua_bind38FighterWorkModuleImpl__calc_param_implEPNS_26BattleObjectModuleAccessorEbb") LINKABLE;
    }
}
