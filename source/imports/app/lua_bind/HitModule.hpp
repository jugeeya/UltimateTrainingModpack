#pragma once

namespace app::lua_bind {
    namespace HitModule {
        u64 cancel_xlu_global(u64,int) asm("_ZN3app8lua_bind33HitModule__cancel_xlu_global_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 clean(u64) asm("_ZN3app8lua_bind21HitModule__clean_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 exist_log(u64,uint,int) asm("_ZN3app8lua_bind25HitModule__exist_log_implEPNS_26BattleObjectModuleAccessorEji") LINKABLE;
        u64 get_center_pos(u64,int,int) asm("_ZN3app8lua_bind30HitModule__get_center_pos_implEPNS_26BattleObjectModuleAccessorEii") LINKABLE;
        u64 get_status(u64,int,int) asm("_ZN3app8lua_bind26HitModule__get_status_implEPNS_26BattleObjectModuleAccessorEii") LINKABLE;
        u64 get_total_status(u64,int) asm("_ZN3app8lua_bind32HitModule__get_total_status_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 get_whole(u64,int) asm("_ZN3app8lua_bind25HitModule__get_whole_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_check_catch(u64,bool,int) asm("_ZN3app8lua_bind31HitModule__set_check_catch_implEPNS_26BattleObjectModuleAccessorEbi") LINKABLE;
        u64 set_defense_mul_status(u64,float) asm("_ZN3app8lua_bind38HitModule__set_defense_mul_status_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_hit_stop_mul(u64,float,int hitStopMulTarget,float) asm("_ZN3app8lua_bind32HitModule__set_hit_stop_mul_implEPNS_26BattleObjectModuleAccessorEfNS_16HitStopMulTargetEf") LINKABLE;
        u64 set_invincible_frame_global(u64,int,bool,int) asm("_ZN3app8lua_bind43HitModule__set_invincible_frame_global_implEPNS_26BattleObjectModuleAccessorEibi") LINKABLE;
        u64 set_no_team(u64,bool) asm("_ZN3app8lua_bind27HitModule__set_no_team_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_status(u64,int,int hitStatus,int) asm("_ZN3app8lua_bind26HitModule__set_status_implEPNS_26BattleObjectModuleAccessorEiNS_9HitStatusEi") LINKABLE;
        u64 set_status_all(u64,int hitStatus,int) asm("_ZN3app8lua_bind30HitModule__set_status_all_implEPNS_26BattleObjectModuleAccessorENS_9HitStatusEi") LINKABLE;
        u64 set_status_joint(u64,u64,int hitStatus,int) asm("_ZN3app8lua_bind32HitModule__set_status_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ENS_9HitStatusEi") LINKABLE;
        u64 set_whole(u64,int hitStatus,int) asm("_ZN3app8lua_bind25HitModule__set_whole_implEPNS_26BattleObjectModuleAccessorENS_9HitStatusEi") LINKABLE;
        u64 set_xlu_frame_global(u64,int,int) asm("_ZN3app8lua_bind36HitModule__set_xlu_frame_global_implEPNS_26BattleObjectModuleAccessorEii") LINKABLE;
        u64 set_xlu_global(u64,int) asm("_ZN3app8lua_bind30HitModule__set_xlu_global_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 sleep(u64,bool) asm("_ZN3app8lua_bind21HitModule__sleep_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
    }
}