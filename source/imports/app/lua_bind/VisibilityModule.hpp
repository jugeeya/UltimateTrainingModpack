#pragma once
namespace app::lua_bind {
    namespace VisibilityModule {
        u64 get_whole(u64) asm("_ZN3app8lua_bind32VisibilityModule__get_whole_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 is_visible(u64) asm("_ZN3app8lua_bind33VisibilityModule__is_visible_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 reset_status_default_int64(u64,s64) asm("_ZN3app8lua_bind49VisibilityModule__reset_status_default_int64_implEPNS_26BattleObjectModuleAccessorEl") LINKABLE;
        u64 set(u64,u64,u64) asm("_ZN3app8lua_bind26VisibilityModule__set_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_") LINKABLE;
        u64 set_default(u64,u64) asm("_ZN3app8lua_bind34VisibilityModule__set_default_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E") LINKABLE;
        u64 set_default_all(u64) asm("_ZN3app8lua_bind38VisibilityModule__set_default_all_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set_default_int64(u64,s64) asm("_ZN3app8lua_bind40VisibilityModule__set_default_int64_implEPNS_26BattleObjectModuleAccessorEl") LINKABLE;
        u64 set_int64(u64,s64,s64) asm("_ZN3app8lua_bind32VisibilityModule__set_int64_implEPNS_26BattleObjectModuleAccessorEll") LINKABLE;
        u64 set_material_anim_priority(u64,u64,bool) asm("_ZN3app8lua_bind49VisibilityModule__set_material_anim_priority_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb") LINKABLE;
        u64 set_mesh_visibility(u64,u64,int,bool) asm("_ZN3app8lua_bind42VisibilityModule__set_mesh_visibility_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eib") LINKABLE;
        u64 set_mesh_visibility_int64(u64,s64,int,bool) asm("_ZN3app8lua_bind48VisibilityModule__set_mesh_visibility_int64_implEPNS_26BattleObjectModuleAccessorElib") LINKABLE;
        u64 set_status_default(u64,u64,u64) asm("_ZN3app8lua_bind41VisibilityModule__set_status_default_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_") LINKABLE;
        u64 set_status_default_int64(u64,s64,s64) asm("_ZN3app8lua_bind47VisibilityModule__set_status_default_int64_implEPNS_26BattleObjectModuleAccessorEll") LINKABLE;
        u64 set_whole(u64,bool) asm("_ZN3app8lua_bind32VisibilityModule__set_whole_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
    }
}
