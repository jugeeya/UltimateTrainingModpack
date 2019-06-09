#pragma once

namespace app::lua_bind {
    namespace ReflectModule {
        bool is_reflect(u64) asm("_ZN3app8lua_bind30ReflectModule__is_reflect_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 object_id(u64) asm("_ZN3app8lua_bind29ReflectModule__object_id_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 reset_info(u64) asm("_ZN3app8lua_bind30ReflectModule__reset_info_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 speed_mul(u64) asm("_ZN3app8lua_bind29ReflectModule__speed_mul_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 team_no(u64) asm("_ZN3app8lua_bind27ReflectModule__team_no_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
    }
}