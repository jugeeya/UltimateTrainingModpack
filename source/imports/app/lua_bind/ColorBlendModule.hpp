#pragma once

namespace app::lua_bind {
    namespace ColorBlendModule {
        u64 cancel_main_color(u64,int) asm("_ZN3app8lua_bind40ColorBlendModule__cancel_main_color_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_disable_camera_depth_influence(u64,bool) asm("_ZN3app8lua_bind57ColorBlendModule__set_disable_camera_depth_influence_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_enable_flash(u64,bool) asm("_ZN3app8lua_bind39ColorBlendModule__set_enable_flash_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_last_attack_direction(u64,const Vector3f*) asm("_ZN3app8lua_bind48ColorBlendModule__set_last_attack_direction_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE") LINKABLE;
        u64 set_main_color(u64,const Vector4f*, const Vector4f *, float,float,int,bool) asm("_ZN3app8lua_bind37ColorBlendModule__set_main_color_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector4fES6_ffib") LINKABLE;
        u64 set_shadow_bloom(u64,float,bool) asm("_ZN3app8lua_bind39ColorBlendModule__set_shadow_bloom_implEPNS_26BattleObjectModuleAccessorEfb") LINKABLE;
    }
}