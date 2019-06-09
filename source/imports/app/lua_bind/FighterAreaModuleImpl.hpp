#pragma once

namespace app::lua_bind {
    namespace FighterAreaModuleImpl {
        u64 enable_fix_jostle_area(u64,float,float) asm("_ZN3app8lua_bind50FighterAreaModuleImpl__enable_fix_jostle_area_implEPNS_26BattleObjectModuleAccessorEff") LINKABLE;
        u64 enable_fix_jostle_area_xy(u64,float,float,float,float) asm("_ZN3app8lua_bind53FighterAreaModuleImpl__enable_fix_jostle_area_xy_implEPNS_26BattleObjectModuleAccessorEffff") LINKABLE;
    }
}