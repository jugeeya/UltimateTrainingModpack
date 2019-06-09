#pragma once

namespace app::lua_bind {
    namespace GrabModule {
        u64 clear_all(u64) asm("_ZN3app8lua_bind26GrabModule__clear_all_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        bool is_grab(u64,int) asm("_ZN3app8lua_bind24GrabModule__is_grab_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 node(u64,int) asm("_ZN3app8lua_bind21GrabModule__node_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_check_front(u64,bool) asm("_ZN3app8lua_bind32GrabModule__set_check_front_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_constraint(u64,int,bool) asm("_ZN3app8lua_bind31GrabModule__set_constraint_implEPNS_26BattleObjectModuleAccessorEib") LINKABLE;
        u64 set_rebound(u64,bool) asm("_ZN3app8lua_bind28GrabModule__set_rebound_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
    }
}