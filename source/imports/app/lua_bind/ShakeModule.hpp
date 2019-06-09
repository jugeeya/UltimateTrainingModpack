#pragma once

namespace app::lua_bind {
    namespace ShakeModule {
        u64 disable_offset(u64) asm("_ZN3app8lua_bind32ShakeModule__disable_offset_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 enable_offset(u64,Vector3f *) asm("_ZN3app8lua_bind31ShakeModule__enable_offset_implEPNS_26BattleObjectModuleAccessorERN3phx8Vector3fE") LINKABLE;
        u64 extend(u64,u64,int) asm("_ZN3app8lua_bind24ShakeModule__extend_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei") LINKABLE;
        u64 is_shake(u64) asm("_ZN3app8lua_bind26ShakeModule__is_shake_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 req(u64,u64,int,bool,const Vector2f*,float,float,bool,bool) asm("_ZN3app8lua_bind21ShakeModule__req_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40EibRKNS3_8Vector2fEffbb") LINKABLE;
        u64 req_time_scale(u64,u64,int,bool,const Vector2f*,float,float,bool,bool,int,float) asm("_ZN3app8lua_bind32ShakeModule__req_time_scale_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40EibRKNS3_8Vector2fEffbbif") LINKABLE;
        u64 set_axis_xy_kind(u64,u64,bool) asm("_ZN3app8lua_bind34ShakeModule__set_axis_xy_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb") LINKABLE;
        u64 set_scale_kind(u64,u64,float) asm("_ZN3app8lua_bind32ShakeModule__set_scale_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ef") LINKABLE;
        u64 stop(u64) asm("_ZN3app8lua_bind22ShakeModule__stop_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 stop_kind(u64,u64) asm("_ZN3app8lua_bind27ShakeModule__stop_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E") LINKABLE;
    }
}