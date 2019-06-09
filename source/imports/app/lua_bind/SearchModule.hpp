#pragma once

namespace app::lua_bind {
    namespace SearchModule {
        u64 clear(u64,int) asm("_ZN3app8lua_bind24SearchModule__clear_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 clear_all(u64) asm("_ZN3app8lua_bind28SearchModule__clear_all_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 enable_safe_pos(u64) asm("_ZN3app8lua_bind34SearchModule__enable_safe_pos_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 is_search(u64,int) asm("_ZN3app8lua_bind28SearchModule__is_search_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_offset(u64,int,const Vector3f*) asm("_ZN3app8lua_bind29SearchModule__set_offset_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fE") LINKABLE;
        u64 set_size(u64,int,float) asm("_ZN3app8lua_bind27SearchModule__set_size_implEPNS_26BattleObjectModuleAccessorEif") LINKABLE;
        u64 set_sync_lr(u64,bool) asm("_ZN3app8lua_bind30SearchModule__set_sync_lr_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_target_opponent(u64,int,int,int,u64) asm("_ZN3app8lua_bind38SearchModule__set_target_opponent_implEPNS_26BattleObjectModuleAccessorEiiij") LINKABLE;
        u64 sleep(u64,bool) asm("_ZN3app8lua_bind24SearchModule__sleep_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
    }
}