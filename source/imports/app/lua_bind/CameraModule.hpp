#pragma once

namespace app::lua_bind {
    namespace CameraModule {
        u64 add_camera_range_offset(u64,const Vector3f*,int) asm("_ZN3app8lua_bind42CameraModule__add_camera_range_offset_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEi") LINKABLE;
        // u64 add_camera_range_rect(u64,lib::Rect const&,int) asm("_ZN3app8lua_bind40CameraModule__add_camera_range_rect_implEPNS_26BattleObjectModuleAccessorERKN3lib4RectEi") LINKABLE;
        u64 end_final_zoom_out(u64) asm("_ZN3app8lua_bind37CameraModule__end_final_zoom_out_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 exit_being_carried(u64,int) asm("_ZN3app8lua_bind37CameraModule__exit_being_carried_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 exit_damage_fly(u64,int) asm("_ZN3app8lua_bind34CameraModule__exit_damage_fly_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 exit_run(u64,int) asm("_ZN3app8lua_bind27CameraModule__exit_run_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 get_camera_type_for_save(u64) asm("_ZN3app8lua_bind43CameraModule__get_camera_type_for_save_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_camera_view_volume_z0_rect(u64) asm("_ZN3app8lua_bind49CameraModule__get_camera_view_volume_z0_rect_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_internal_camera_target_pos(u64) asm("_ZN3app8lua_bind49CameraModule__get_internal_camera_target_pos_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_main_camera_range(u64) asm("_ZN3app8lua_bind40CameraModule__get_main_camera_range_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 is_clip_in(u64,bool) asm("_ZN3app8lua_bind29CameraModule__is_clip_in_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 req_quake(u64,int) asm("_ZN3app8lua_bind28CameraModule__req_quake_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 req_quake_pos(u64,int,const Vector3f*) asm("_ZN3app8lua_bind32CameraModule__req_quake_pos_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fE") LINKABLE;
        u64 reset_all(u64) asm("_ZN3app8lua_bind28CameraModule__reset_all_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 reset_camera_range(u64,int) asm("_ZN3app8lua_bind37CameraModule__reset_camera_range_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 reset_main_camera_fov(u64) asm("_ZN3app8lua_bind40CameraModule__reset_main_camera_fov_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set_being_carried(u64,int) asm("_ZN3app8lua_bind36CameraModule__set_being_carried_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_camera_range_offset(u64,const Vector3f*,int) asm("_ZN3app8lua_bind42CameraModule__set_camera_range_offset_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEi") LINKABLE;
        u64 set_camera_type(u64,int) asm("_ZN3app8lua_bind34CameraModule__set_camera_type_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_damage_fly(u64,int) asm("_ZN3app8lua_bind33CameraModule__set_damage_fly_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_enable_camera(u64,bool,int) asm("_ZN3app8lua_bind36CameraModule__set_enable_camera_implEPNS_26BattleObjectModuleAccessorEbi") LINKABLE;
        u64 set_enable_update_pos(u64,unsigned char,int) asm("_ZN3app8lua_bind40CameraModule__set_enable_update_pos_implEPNS_26BattleObjectModuleAccessorEhi") LINKABLE;
        u64 set_priority(u64,unsigned char,int) asm("_ZN3app8lua_bind31CameraModule__set_priority_implEPNS_26BattleObjectModuleAccessorEhi") LINKABLE;
        u64 set_run(u64,int) asm("_ZN3app8lua_bind26CameraModule__set_run_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_status(u64,int cameraStatus,int) asm("_ZN3app8lua_bind29CameraModule__set_status_implEPNS_26BattleObjectModuleAccessorENS_12CameraStatusEi") LINKABLE;
        u64 set_whole(u64,bool) asm("_ZN3app8lua_bind28CameraModule__set_whole_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        // u64 start_final_zoom_out(u64,int,lib::Rect const&,const Vector3f*) asm("_ZN3app8lua_bind39CameraModule__start_final_zoom_out_implEPNS_26BattleObjectModuleAccessorEiRKN3lib4RectERKN3phx8Vector3fE") LINKABLE;
        u64 stop_quake(u64,int) asm("_ZN3app8lua_bind29CameraModule__stop_quake_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 update_force(u64,int,const Vector3f*,const Vector3f*) asm("_ZN3app8lua_bind31CameraModule__update_force_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fES6_") LINKABLE;
        u64 zoom_in(u64,int,int,float,const Vector2f*,bool) asm("_ZN3app8lua_bind26CameraModule__zoom_in_implEPNS_26BattleObjectModuleAccessorEiifRKN3phx8Vector2fEb") LINKABLE;
        u64 zoom_out(u64,int) asm("_ZN3app8lua_bind27CameraModule__zoom_out_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
    }
}