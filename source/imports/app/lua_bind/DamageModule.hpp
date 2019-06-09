#pragma once

namespace app::lua_bind {
    namespace DamageModule {
        u64 add_damage(u64,float,int) asm("_ZN3app8lua_bind29DamageModule__add_damage_implEPNS_26BattleObjectModuleAccessorEfi") LINKABLE;
        u64 damage(u64,int) asm("_ZN3app8lua_bind25DamageModule__damage_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 end_damage_info_log(u64) asm("_ZN3app8lua_bind38DamageModule__end_damage_info_log_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 heal(u64,float,int) asm("_ZN3app8lua_bind23DamageModule__heal_implEPNS_26BattleObjectModuleAccessorEfi") LINKABLE;
        u64 init_damage(u64,float) asm("_ZN3app8lua_bind30DamageModule__init_damage_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 is_capture_cut(u64,float) asm("_ZN3app8lua_bind33DamageModule__is_capture_cut_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 is_damage_lock(u64) asm("_ZN3app8lua_bind33DamageModule__is_damage_lock_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 is_no_reaction_mode_perfect(u64) asm("_ZN3app8lua_bind46DamageModule__is_no_reaction_mode_perfect_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 is_paralyze(u64) asm("_ZN3app8lua_bind30DamageModule__is_paralyze_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 overwrite_log_reaction_frame(u64,float) asm("_ZN3app8lua_bind47DamageModule__overwrite_log_reaction_frame_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 reaction(u64,int) asm("_ZN3app8lua_bind27DamageModule__reaction_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 reset_no_reaction_mode_status(u64) asm("_ZN3app8lua_bind48DamageModule__reset_no_reaction_mode_status_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set_attacker_info(u64,u64) asm("_ZN3app8lua_bind36DamageModule__set_attacker_info_implEPNS_26BattleObjectModuleAccessorEj") LINKABLE;
        u64 set_critical_hit(u64,bool) asm("_ZN3app8lua_bind35DamageModule__set_critical_hit_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_damage_lock(u64,bool) asm("_ZN3app8lua_bind34DamageModule__set_damage_lock_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_damage_mul(u64,float) asm("_ZN3app8lua_bind33DamageModule__set_damage_mul_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_damage_mul_2nd(u64,float) asm("_ZN3app8lua_bind37DamageModule__set_damage_mul_2nd_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_force_damage(u64,u64,const Vector3f*,int,int,bool,bool,bool,bool) asm("_ZN3app8lua_bind35DamageModule__set_force_damage_implEPNS_26BattleObjectModuleAccessorEjRKN3phx8Vector3fEiibbbb") LINKABLE;
        u64 set_force_damage_from_last_damage(u64) asm("_ZN3app8lua_bind52DamageModule__set_force_damage_from_last_damage_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set_ignore_capture_cut_no(u64,signed char) asm("_ZN3app8lua_bind44DamageModule__set_ignore_capture_cut_no_implEPNS_26BattleObjectModuleAccessorEa") LINKABLE;
        u64 set_no_reaction_damage_power(u64,float) asm("_ZN3app8lua_bind47DamageModule__set_no_reaction_damage_power_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_no_reaction_mode_status(u64,int damageNoReactionMode,float,float,int) asm("_ZN3app8lua_bind46DamageModule__set_no_reaction_mode_status_implEPNS_26BattleObjectModuleAccessorENS_20DamageNoReactionModeEffi") LINKABLE;
        u64 set_no_reaction_no_effect(u64,bool) asm("_ZN3app8lua_bind44DamageModule__set_no_reaction_no_effect_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_reaction_mul_2nd(u64,float) asm("_ZN3app8lua_bind39DamageModule__set_reaction_mul_2nd_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 sleep(u64,bool) asm("_ZN3app8lua_bind24DamageModule__sleep_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 start_damage_info_log(u64) asm("_ZN3app8lua_bind40DamageModule__start_damage_info_log_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
    }
}