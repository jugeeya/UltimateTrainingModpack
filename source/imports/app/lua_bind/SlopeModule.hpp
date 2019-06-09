#pragma once

namespace app::lua_bind {
    namespace SlopeModule {
        u64 floor_diff_l(u64) asm("_ZN3app8lua_bind30SlopeModule__floor_diff_l_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 floor_diff_r(u64) asm("_ZN3app8lua_bind30SlopeModule__floor_diff_r_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 update_model_top_angle(u64) asm("_ZN3app8lua_bind40SlopeModule__update_model_top_angle_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
    }
}