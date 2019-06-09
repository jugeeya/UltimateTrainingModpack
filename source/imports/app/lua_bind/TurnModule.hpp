#pragma once

namespace app::lua_bind {
    namespace TurnModule {
        u64 end_turn(u64) asm("_ZN3app8lua_bind25TurnModule__end_turn_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 is_extern(u64) asm("_ZN3app8lua_bind26TurnModule__is_extern_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 is_turn(u64) asm("_ZN3app8lua_bind24TurnModule__is_turn_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 is_turn_after90(u64) asm("_ZN3app8lua_bind32TurnModule__is_turn_after90_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 ry_reverse(u64) asm("_ZN3app8lua_bind27TurnModule__ry_reverse_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set_omit_intermediate(u64,bool) asm("_ZN3app8lua_bind38TurnModule__set_omit_intermediate_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_turn(u64,u64,float,bool,bool,bool) asm("_ZN3app8lua_bind25TurnModule__set_turn_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efbbb") LINKABLE;
    }
}
