#pragma once

namespace app::lua_bind {
    namespace PhysicsModule {
        u64 get_2nd_active_length(u64) asm("_ZN3app8lua_bind41PhysicsModule__get_2nd_active_length_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_2nd_active_node_num(u64) asm("_ZN3app8lua_bind43PhysicsModule__get_2nd_active_node_num_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_2nd_joint_num(u64) asm("_ZN3app8lua_bind37PhysicsModule__get_2nd_joint_num_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_2nd_node_num(u64) asm("_ZN3app8lua_bind36PhysicsModule__get_2nd_node_num_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_2nd_pos(u64,int) asm("_ZN3app8lua_bind31PhysicsModule__get_2nd_pos_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 get_2nd_speed(u64,int) asm("_ZN3app8lua_bind33PhysicsModule__get_2nd_speed_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 get_2nd_touch_ground_line_num(u64,int) asm("_ZN3app8lua_bind49PhysicsModule__get_2nd_touch_ground_line_num_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 get_ik_end_joint_id(u64,int) asm("_ZN3app8lua_bind39PhysicsModule__get_ik_end_joint_id_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 reset_swing(u64) asm("_ZN3app8lua_bind31PhysicsModule__reset_swing_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set_2nd_active_node_num(u64,float) asm("_ZN3app8lua_bind43PhysicsModule__set_2nd_active_node_num_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_2nd_air_resistance(u64,float) asm("_ZN3app8lua_bind42PhysicsModule__set_2nd_air_resistance_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_2nd_back_speed(u64,float) asm("_ZN3app8lua_bind38PhysicsModule__set_2nd_back_speed_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_2nd_collision_size(u64,int,float) asm("_ZN3app8lua_bind42PhysicsModule__set_2nd_collision_size_implEPNS_26BattleObjectModuleAccessorEif") LINKABLE;
        u64 set_2nd_disable_collision(u64,u64,bool) asm("_ZN3app8lua_bind45PhysicsModule__set_2nd_disable_collision_implEPNS_26BattleObjectModuleAccessorEjb") LINKABLE;
        u64 set_2nd_end_pos(u64,const Vector3f*,const Vector3f*) asm("_ZN3app8lua_bind35PhysicsModule__set_2nd_end_pos_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fES6_") LINKABLE;
        u64 set_2nd_flip(u64,bool) asm("_ZN3app8lua_bind32PhysicsModule__set_2nd_flip_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_2nd_gravity(u64,float) asm("_ZN3app8lua_bind35PhysicsModule__set_2nd_gravity_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_2nd_length(u64,int,float) asm("_ZN3app8lua_bind34PhysicsModule__set_2nd_length_implEPNS_26BattleObjectModuleAccessorEif") LINKABLE;
        u64 set_2nd_node_num_max(u64,int) asm("_ZN3app8lua_bind40PhysicsModule__set_2nd_node_num_max_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_2nd_pos(u64,int,const Vector3f*) asm("_ZN3app8lua_bind31PhysicsModule__set_2nd_pos_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fE") LINKABLE;
        u64 set_2nd_restitution_range(u64,float) asm("_ZN3app8lua_bind45PhysicsModule__set_2nd_restitution_range_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_2nd_restitution_rate(u64,float,float) asm("_ZN3app8lua_bind44PhysicsModule__set_2nd_restitution_rate_implEPNS_26BattleObjectModuleAccessorEff") LINKABLE;
        u64 set_2nd_restitution_rate_2(u64,int,float) asm("_ZN3app8lua_bind46PhysicsModule__set_2nd_restitution_rate_2_implEPNS_26BattleObjectModuleAccessorEif") LINKABLE;
        u64 set_2nd_status(u64,int) asm("_ZN3app8lua_bind34PhysicsModule__set_2nd_status_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_2nd_water_resistance(u64,float) asm("_ZN3app8lua_bind44PhysicsModule__set_2nd_water_resistance_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_2nd_z_range(u64,float,float) asm("_ZN3app8lua_bind35PhysicsModule__set_2nd_z_range_implEPNS_26BattleObjectModuleAccessorEff") LINKABLE;
        u64 set_enable_floor_collision_line(u64, void *) asm("_ZN3app8lua_bind51PhysicsModule__set_enable_floor_collision_line_implEPNS_26BattleObjectModuleAccessorEPNS_19GroundCollisionLineE") LINKABLE;
        u64 set_ik(u64,int,float) asm("_ZN3app8lua_bind26PhysicsModule__set_ik_implEPNS_26BattleObjectModuleAccessorEif") LINKABLE;
        u64 set_ik_target_pos(u64,int,const Vector3f*,const Vector3f*) asm("_ZN3app8lua_bind37PhysicsModule__set_ik_target_pos_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fES6_") LINKABLE;
        u64 set_reflect_param_ceil(u64,float,float,float) asm("_ZN3app8lua_bind42PhysicsModule__set_reflect_param_ceil_implEPNS_26BattleObjectModuleAccessorEfff") LINKABLE;
        u64 set_reflect_param_floor(u64,float,float,float) asm("_ZN3app8lua_bind43PhysicsModule__set_reflect_param_floor_implEPNS_26BattleObjectModuleAccessorEfff") LINKABLE;
        u64 set_reflect_param_wall(u64,float,float,float) asm("_ZN3app8lua_bind42PhysicsModule__set_reflect_param_wall_implEPNS_26BattleObjectModuleAccessorEfff") LINKABLE;
        u64 set_swing_ground_collision_all(u64,bool) asm("_ZN3app8lua_bind50PhysicsModule__set_swing_ground_collision_all_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_swing_joint_name(u64,bool,u64,bool) asm("_ZN3app8lua_bind40PhysicsModule__set_swing_joint_name_implEPNS_26BattleObjectModuleAccessorEbN3phx6Hash40Eb") LINKABLE;
        u64 set_swing_only_anim(u64,bool) asm("_ZN3app8lua_bind39PhysicsModule__set_swing_only_anim_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_swing_rate(u64,float) asm("_ZN3app8lua_bind34PhysicsModule__set_swing_rate_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_swing_rebirth(u64,bool) asm("_ZN3app8lua_bind37PhysicsModule__set_swing_rebirth_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_swing_special_state(u64,int) asm("_ZN3app8lua_bind43PhysicsModule__set_swing_special_state_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 stop_charge(u64) asm("_ZN3app8lua_bind31PhysicsModule__stop_charge_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
    }
}
