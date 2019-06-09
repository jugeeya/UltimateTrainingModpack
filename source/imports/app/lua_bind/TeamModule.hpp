#pragma once
namespace app::lua_bind {
    namespace TeamModule {
        u64 hit_team_no(u64) asm("_ZN3app8lua_bind28TeamModule__hit_team_no_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 metamon_owner_fighter_entry_id(u64) asm("_ZN3app8lua_bind47TeamModule__metamon_owner_fighter_entry_id_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set_hit_team(u64,int) asm("_ZN3app8lua_bind29TeamModule__set_hit_team_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_team(u64,int,bool) asm("_ZN3app8lua_bind25TeamModule__set_team_implEPNS_26BattleObjectModuleAccessorEib") LINKABLE;
        u64 set_team_owner_id(u64,uint) asm("_ZN3app8lua_bind34TeamModule__set_team_owner_id_implEPNS_26BattleObjectModuleAccessorEj") LINKABLE;
        u64 set_team_second(u64,int) asm("_ZN3app8lua_bind32TeamModule__set_team_second_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 team_no(u64) asm("_ZN3app8lua_bind24TeamModule__team_no_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 team_owner_id(u64) asm("_ZN3app8lua_bind30TeamModule__team_owner_id_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 team_second_no(u64) asm("_ZN3app8lua_bind31TeamModule__team_second_no_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
    }
}