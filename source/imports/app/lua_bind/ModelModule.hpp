#pragma once

namespace app::lua_bind {
    namespace ModelModule {
        u64 clear_joint_srt(u64,u64) asm("_ZN3app8lua_bind33ModelModule__clear_joint_srt_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E") LINKABLE;
        u64 disable_gold_eye(u64) asm("_ZN3app8lua_bind34ModelModule__disable_gold_eye_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 enable_gold_eye(u64) asm("_ZN3app8lua_bind33ModelModule__enable_gold_eye_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 joint_global_offset_from_top(u64,u64,Vector3f *) asm("_ZN3app8lua_bind46ModelModule__joint_global_offset_from_top_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERNS3_8Vector3fE") LINKABLE;
        u64 joint_global_position(u64,u64,Vector3f *,bool) asm("_ZN3app8lua_bind39ModelModule__joint_global_position_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERNS3_8Vector3fEb") LINKABLE;
        u64 joint_global_position_with_offset(u64,u64,const Vector3f*,Vector3f *,bool) asm("_ZN3app8lua_bind51ModelModule__joint_global_position_with_offset_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fERS5_b") LINKABLE;
        u64 joint_global_rotation(u64,u64,Vector3f *,bool) asm("_ZN3app8lua_bind39ModelModule__joint_global_rotation_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERNS3_8Vector3fEb") LINKABLE;
        u64 joint_rotate(u64,u64,Vector3f *) asm("_ZN3app8lua_bind30ModelModule__joint_rotate_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERNS3_8Vector3fE") LINKABLE;
        u64 scale(u64) asm("_ZN3app8lua_bind23ModelModule__scale_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 scale_z(u64) asm("_ZN3app8lua_bind25ModelModule__scale_z_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set_alpha(u64,float) asm("_ZN3app8lua_bind27ModelModule__set_alpha_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_color_rgb(u64,float,float,float,int MODEL_COLOR_TYPE) asm("_ZN3app8lua_bind31ModelModule__set_color_rgb_implEPNS_26BattleObjectModuleAccessorEfffNS_16MODEL_COLOR_TYPEE") LINKABLE;
        u64 set_depth_offset(u64,float) asm("_ZN3app8lua_bind34ModelModule__set_depth_offset_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_depth_stencil(u64,int) asm("_ZN3app8lua_bind35ModelModule__set_depth_stencil_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_joint_rotate(u64,u64,const Vector3f*,int motionNodeRotateCompose,int motionNodeRotateOrder) asm("_ZN3app8lua_bind34ModelModule__set_joint_rotate_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fENS_23MotionNodeRotateComposeENS_21MotionNodeRotateOrderE") LINKABLE;
        u64 set_joint_scale(u64,u64,const Vector3f*) asm("_ZN3app8lua_bind33ModelModule__set_joint_scale_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fE") LINKABLE;
        u64 set_joint_srt(u64,u64,const Vector3f*,const Vector3f*,const Vector3f*) asm("_ZN3app8lua_bind31ModelModule__set_joint_srt_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fES7_S7_") LINKABLE;
        u64 set_joint_translate(u64,u64,const Vector3f*,bool,bool) asm("_ZN3app8lua_bind37ModelModule__set_joint_translate_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fEbb") LINKABLE;
        u64 set_mesh_visibility(u64,u64,bool) asm("_ZN3app8lua_bind37ModelModule__set_mesh_visibility_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb") LINKABLE;
        u64 set_render_offset_position(u64,const Vector3f*) asm("_ZN3app8lua_bind44ModelModule__set_render_offset_position_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE") LINKABLE;
        u64 set_rotation_order(u64,int motionNodeRotateOrder) asm("_ZN3app8lua_bind36ModelModule__set_rotation_order_implEPNS_26BattleObjectModuleAccessorENS_21MotionNodeRotateOrderE") LINKABLE;
        u64 set_scale(u64,float) asm("_ZN3app8lua_bind27ModelModule__set_scale_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_temporary_scale_z(u64,float) asm("_ZN3app8lua_bind39ModelModule__set_temporary_scale_z_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 top_joint_global_position_from_joint(u64,u64,const Vector3f*,Vector3f *) asm("_ZN3app8lua_bind54ModelModule__top_joint_global_position_from_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fERS5_") LINKABLE;
        u64 virtual_joint_tra(u64,u64) asm("_ZN3app8lua_bind35ModelModule__virtual_joint_tra_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E") LINKABLE;
    }
}