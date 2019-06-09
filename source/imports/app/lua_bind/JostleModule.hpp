#pragma once

namespace app::lua_bind {
    namespace JostleModule {
        bool is_sleep(u64) asm("_ZN3app8lua_bind27JostleModule__is_sleep_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 overlap_x(u64,bool) asm("_ZN3app8lua_bind28JostleModule__overlap_x_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_fix(u64,bool) asm("_ZN3app8lua_bind26JostleModule__set_fix_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_force_gap(u64,int) asm("_ZN3app8lua_bind32JostleModule__set_force_gap_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_ignore_owner_id(u64,int) asm("_ZN3app8lua_bind38JostleModule__set_ignore_owner_id_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_ignore_speed_x(u64,bool) asm("_ZN3app8lua_bind37JostleModule__set_ignore_speed_x_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_influence_opponent_weight(u64,bool) asm("_ZN3app8lua_bind48JostleModule__set_influence_opponent_weight_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_layer(u64,int) asm("_ZN3app8lua_bind28JostleModule__set_layer_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_overlap_rate_mul(u64,float) asm("_ZN3app8lua_bind39JostleModule__set_overlap_rate_mul_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_priority(u64,int) asm("_ZN3app8lua_bind31JostleModule__set_priority_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_propagate_push_speed(u64,bool) asm("_ZN3app8lua_bind43JostleModule__set_propagate_push_speed_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_push_speed_x(u64,float,bool) asm("_ZN3app8lua_bind35JostleModule__set_push_speed_x_implEPNS_26BattleObjectModuleAccessorEfb") LINKABLE;
        u64 set_push_speed_x_overlap_rate(u64,float) asm("_ZN3app8lua_bind48JostleModule__set_push_speed_x_overlap_rate_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_refer(u64,bool) asm("_ZN3app8lua_bind28JostleModule__set_refer_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_status(u64,bool) asm("_ZN3app8lua_bind29JostleModule__set_status_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_team(u64,int) asm("_ZN3app8lua_bind27JostleModule__set_team_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_weight(u64,float) asm("_ZN3app8lua_bind29JostleModule__set_weight_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 sleep(u64,bool) asm("_ZN3app8lua_bind24JostleModule__sleep_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 target_weight(u64) asm("_ZN3app8lua_bind32JostleModule__target_weight_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 weight(u64) asm("_ZN3app8lua_bind25JostleModule__weight_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
    }
}