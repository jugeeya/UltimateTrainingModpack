#pragma once

namespace app::lua_bind {
    namespace SlowModule {
        u64 clear(u64) asm("_ZN3app8lua_bind22SlowModule__clear_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 clear_whole(u64) asm("_ZN3app8lua_bind28SlowModule__clear_whole_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 frame(u64,int) asm("_ZN3app8lua_bind22SlowModule__frame_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 is_prev_skip(u64) asm("_ZN3app8lua_bind29SlowModule__is_prev_skip_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 is_skip(u64) asm("_ZN3app8lua_bind24SlowModule__is_skip_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 is_slow(u64) asm("_ZN3app8lua_bind24SlowModule__is_slow_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 rate(u64) asm("_ZN3app8lua_bind21SlowModule__rate_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 rate_ignore_effect(u64) asm("_ZN3app8lua_bind35SlowModule__rate_ignore_effect_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set(u64,int,int,int,bool,uint) asm("_ZN3app8lua_bind20SlowModule__set_implEPNS_26BattleObjectModuleAccessorEiiibj") LINKABLE;
        u64 set_whole(u64,unsigned char,int) asm("_ZN3app8lua_bind26SlowModule__set_whole_implEPNS_26BattleObjectModuleAccessorEhi") LINKABLE;
        u64 whole_frame(u64) asm("_ZN3app8lua_bind28SlowModule__whole_frame_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 whole_mag(u64) asm("_ZN3app8lua_bind26SlowModule__whole_mag_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        }
    }
