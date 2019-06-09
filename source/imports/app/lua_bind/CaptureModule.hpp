#pragma once

namespace app::lua_bind {
    namespace CaptureModule {
        u64 capture(u64,u64,int,bool,int) asm("_ZN3app8lua_bind27CaptureModule__capture_implEPNS_26BattleObjectModuleAccessorEjibi") LINKABLE;
        u64 capture_cut(u64,bool,bool,bool) asm("_ZN3app8lua_bind31CaptureModule__capture_cut_implEPNS_26BattleObjectModuleAccessorEbbb") LINKABLE;
        u64 capture_to_catch_node_pos_diff(u64) asm("_ZN3app8lua_bind50CaptureModule__capture_to_catch_node_pos_diff_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 catch_node_pos_y(u64) asm("_ZN3app8lua_bind36CaptureModule__catch_node_pos_y_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 check_damage(u64,int,float,float) asm("_ZN3app8lua_bind32CaptureModule__check_damage_implEPNS_26BattleObjectModuleAccessorEiff") LINKABLE;
        u64 check_damage_thrown(u64) asm("_ZN3app8lua_bind39CaptureModule__check_damage_thrown_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        bool is_capture(u64) asm("_ZN3app8lua_bind30CaptureModule__is_capture_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        bool is_catch_hit_stop(u64) asm("_ZN3app8lua_bind37CaptureModule__is_catch_hit_stop_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        bool is_ignore_distance(u64) asm("_ZN3app8lua_bind38CaptureModule__is_ignore_distance_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        bool is_motion_hi_lw(u64) asm("_ZN3app8lua_bind35CaptureModule__is_motion_hi_lw_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        bool is_thrown(u64) asm("_ZN3app8lua_bind29CaptureModule__is_thrown_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        bool is_thrown_finish(u64) asm("_ZN3app8lua_bind36CaptureModule__is_thrown_finish_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 motion(u64,u64,int) asm("_ZN3app8lua_bind26CaptureModule__motion_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei") LINKABLE;
        u64 motion_lw(u64,u64,int) asm("_ZN3app8lua_bind29CaptureModule__motion_lw_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei") LINKABLE;
        u64 motion_offset(u64) asm("_ZN3app8lua_bind33CaptureModule__motion_offset_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 motion_offset_lw(u64) asm("_ZN3app8lua_bind36CaptureModule__motion_offset_lw_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 node_offset(u64,bool) asm("_ZN3app8lua_bind31CaptureModule__node_offset_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_nodes(u64,u64,u64,float) asm("_ZN3app8lua_bind29CaptureModule__set_nodes_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_f") LINKABLE;
        u64 set_send_cut_event(u64,bool) asm("_ZN3app8lua_bind38CaptureModule__set_send_cut_event_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 thrown(u64) asm("_ZN3app8lua_bind26CaptureModule__thrown_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 thrown_cut(u64,bool,bool) asm("_ZN3app8lua_bind30CaptureModule__thrown_cut_implEPNS_26BattleObjectModuleAccessorEbb") LINKABLE;
        u64 update_node_pos(u64) asm("_ZN3app8lua_bind35CaptureModule__update_node_pos_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 update_pos_thrown(u64) asm("_ZN3app8lua_bind37CaptureModule__update_pos_thrown_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
    }
}