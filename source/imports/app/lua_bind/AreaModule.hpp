#pragma once

namespace app::lua_bind {
    namespace AreaModule {
        u64 enable_area(u64,int,bool,int) asm("_ZN3app8lua_bind28AreaModule__enable_area_implEPNS_26BattleObjectModuleAccessorEibi") LINKABLE;
        u64 erase_wind(u64,int) asm("_ZN3app8lua_bind27AreaModule__erase_wind_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 get_area_contact_count(u64,int) asm("_ZN3app8lua_bind39AreaModule__get_area_contact_count_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 get_area_contact_log(u64,int,int) asm("_ZN3app8lua_bind37AreaModule__get_area_contact_log_implEPNS_26BattleObjectModuleAccessorEii") LINKABLE;
        u64 get_area_contact_target_id(u64,int,int) asm("_ZN3app8lua_bind43AreaModule__get_area_contact_target_id_implEPNS_26BattleObjectModuleAccessorEii") LINKABLE;
        u64 get_water_surface_y(u64) asm("_ZN3app8lua_bind36AreaModule__get_water_surface_y_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_water_task_id(u64) asm("_ZN3app8lua_bind34AreaModule__get_water_task_id_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        bool is_enable_area(u64,int) asm("_ZN3app8lua_bind31AreaModule__is_enable_area_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        bool is_exist_area_instance(u64,int) asm("_ZN3app8lua_bind39AreaModule__is_exist_area_instance_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        bool is_water(u64) asm("_ZN3app8lua_bind25AreaModule__is_water_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 layer(u64,int) asm("_ZN3app8lua_bind22AreaModule__layer_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 reset_area(u64,int) asm("_ZN3app8lua_bind27AreaModule__reset_area_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_area_shape_aabb(u64,int,const Vector2f*,const Vector2f*) asm("_ZN3app8lua_bind36AreaModule__set_area_shape_aabb_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector2fES6_") LINKABLE;
        u64 set_area_shape_circle(u64,int,const Vector2f*,float) asm("_ZN3app8lua_bind38AreaModule__set_area_shape_circle_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector2fEf") LINKABLE;
        u64 set_area_shape_type(u64,int,unsigned char) asm("_ZN3app8lua_bind36AreaModule__set_area_shape_type_implEPNS_26BattleObjectModuleAccessorEih") LINKABLE;
        u64 set_auto_layer_update(u64,bool) asm("_ZN3app8lua_bind38AreaModule__set_auto_layer_update_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_center_x0(u64,int,bool) asm("_ZN3app8lua_bind30AreaModule__set_center_x0_implEPNS_26BattleObjectModuleAccessorEib") LINKABLE;
        u64 set_layer(u64,int,int) asm("_ZN3app8lua_bind26AreaModule__set_layer_implEPNS_26BattleObjectModuleAccessorEii") LINKABLE;
        u64 set_whole(u64,bool) asm("_ZN3app8lua_bind26AreaModule__set_whole_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 sleep(u64,bool) asm("_ZN3app8lua_bind22AreaModule__sleep_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
    }
}