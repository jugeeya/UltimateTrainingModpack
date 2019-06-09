#pragma once

namespace app::lua_bind {
    namespace EffectModule {
        void clear_screen(u64,int,int) asm("_ZN3app8lua_bind31EffectModule__clear_screen_implEPNS_26BattleObjectModuleAccessorEii") LINKABLE;
        void detach(u64,u64,int) asm("_ZN3app8lua_bind25EffectModule__detach_implEPNS_26BattleObjectModuleAccessorEji") LINKABLE;
        void detach_all(u64,u64) asm("_ZN3app8lua_bind29EffectModule__detach_all_implEPNS_26BattleObjectModuleAccessorEj") LINKABLE;
        void detach_kind(u64,u64,int) asm("_ZN3app8lua_bind30EffectModule__detach_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei") LINKABLE;
        u64 enable_stencil(u64,bool) asm("_ZN3app8lua_bind33EffectModule__enable_stencil_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 enable_sync_init_pos_last(u64) asm("_ZN3app8lua_bind44EffectModule__enable_sync_init_pos_last_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 end_kind(u64,u64,int) asm("_ZN3app8lua_bind27EffectModule__end_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei") LINKABLE;
        u64 force_update_common_effect(u64) asm("_ZN3app8lua_bind45EffectModule__force_update_common_effect_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_dead_effect_rot_z(u64,const Vector3f*,float,bool) asm("_ZN3app8lua_bind40EffectModule__get_dead_effect_rot_z_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEfb") LINKABLE;
        u64 get_dead_effect_scale(u64,const Vector3f*,float,bool) asm("_ZN3app8lua_bind40EffectModule__get_dead_effect_scale_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEfb") LINKABLE;
        u64 get_last_handle(u64) asm("_ZN3app8lua_bind34EffectModule__get_last_handle_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_variation_effect_kind(u64,u64,int) asm("_ZN3app8lua_bind44EffectModule__get_variation_effect_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei") LINKABLE;
        bool is_enable_ground_effect(u64) asm("_ZN3app8lua_bind42EffectModule__is_enable_ground_effect_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        bool is_exist_common(u64,u64) asm("_ZN3app8lua_bind34EffectModule__is_exist_common_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E") LINKABLE;
        bool is_exist_effect(u64,u64) asm("_ZN3app8lua_bind34EffectModule__is_exist_effect_implEPNS_26BattleObjectModuleAccessorEj") LINKABLE;
        bool is_sync_visibility(u64) asm("_ZN3app8lua_bind37EffectModule__is_sync_visibility_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        void kill(u64,u64,bool,bool) asm("_ZN3app8lua_bind23EffectModule__kill_implEPNS_26BattleObjectModuleAccessorEjbb") LINKABLE;
        void kill_all(u64,u64,bool,bool) asm("_ZN3app8lua_bind27EffectModule__kill_all_implEPNS_26BattleObjectModuleAccessorEjbb") LINKABLE;
        void kill_joint_id(u64,u64,bool,bool) asm("_ZN3app8lua_bind32EffectModule__kill_joint_id_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ebb") LINKABLE;
        void kill_kind(u64,u64,bool,bool) asm("_ZN3app8lua_bind28EffectModule__kill_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ebb") LINKABLE;
        u64 preset_lifetime_rate_partial(u64,float) asm("_ZN3app8lua_bind47EffectModule__preset_lifetime_rate_partial_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 preset_limit_num(u64,int) asm("_ZN3app8lua_bind35EffectModule__preset_limit_num_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 remove(u64,u64,u64) asm("_ZN3app8lua_bind25EffectModule__remove_implEPNS_26BattleObjectModuleAccessorEjj") LINKABLE;
        u64 remove_all(u64,u64,u64) asm("_ZN3app8lua_bind29EffectModule__remove_all_implEPNS_26BattleObjectModuleAccessorEjj") LINKABLE;
        u64 remove_all_after_image(u64,u64,u64) asm("_ZN3app8lua_bind41EffectModule__remove_all_after_image_implEPNS_26BattleObjectModuleAccessorEjj") LINKABLE;
        u64 remove_common(u64,u64) asm("_ZN3app8lua_bind32EffectModule__remove_common_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E") LINKABLE;
        u64 remove_post_effect_line(u64,int,bool) asm("_ZN3app8lua_bind42EffectModule__remove_post_effect_line_implEPNS_26BattleObjectModuleAccessorEib") LINKABLE;
        u64 remove_screen(u64,u64,int) asm("_ZN3app8lua_bind32EffectModule__remove_screen_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei") LINKABLE;
        u64 remove_time(u64,u64) asm("_ZN3app8lua_bind30EffectModule__remove_time_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E") LINKABLE;
        u64 req(u64,u64,const Vector3f*,const Vector3f*,float,u64,int,bool,int) asm("_ZN3app8lua_bind22EffectModule__req_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fES7_fjibi") LINKABLE;
        u64 req_2d(u64,u64,const Vector3f*,const Vector3f*,float,u64) asm("_ZN3app8lua_bind25EffectModule__req_2d_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fES7_fj") LINKABLE;
        u64 req_common(u64,u64,float) asm("_ZN3app8lua_bind29EffectModule__req_common_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ef") LINKABLE;
        u64 req_emit(u64,u64,u64) asm("_ZN3app8lua_bind27EffectModule__req_emit_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ej") LINKABLE;
        u64 req_follow(u64,u64,u64,const Vector3f*,const Vector3f*,float,bool,u64,int,int,int,int,bool,bool) asm("_ZN3app8lua_bind29EffectModule__req_follow_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_RKNS3_8Vector3fES7_fbjiiiibb") LINKABLE;
        u64 req_on_joint(u64,u64,u64,const Vector3f*,const Vector3f*,float,const Vector3f*,const Vector3f*,bool,u64,int,int) asm("_ZN3app8lua_bind31EffectModule__req_on_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_RKNS3_8Vector3fES7_fS7_S7_bjii") LINKABLE;
        u64 req_screen(u64,u64,bool,bool,bool) asm("_ZN3app8lua_bind29EffectModule__req_screen_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ebbb") LINKABLE;
        u64 req_time(u64,u64,int,const Vector3f*,const Vector3f*,float,u64,bool,bool) asm("_ZN3app8lua_bind27EffectModule__req_time_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40EiRKNS3_8Vector3fES7_fjbb") LINKABLE;
        u64 request_post_effect_line_circle(u64,u64,u64,Vector2f,Vector3f,bool,float,float) asm("_ZN3app8lua_bind50EffectModule__request_post_effect_line_circle_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_NS3_8Vector2fENS3_8Vector3fEbff") LINKABLE;
        u64 request_post_effect_line_parallel_2d(u64,u64,Vector2f,Vector2f,Vector2f,Vector2f,bool,float,float) asm("_ZN3app8lua_bind55EffectModule__request_post_effect_line_parallel_2d_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ENS3_8Vector2fES5_S5_S5_bff") LINKABLE;
        u64 reset_screen(u64,int) asm("_ZN3app8lua_bind31EffectModule__reset_screen_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_alpha(u64,u64,float) asm("_ZN3app8lua_bind28EffectModule__set_alpha_implEPNS_26BattleObjectModuleAccessorEjf") LINKABLE;
        u64 set_custom_uv_offset(u64,u64,const Vector2f*,int) asm("_ZN3app8lua_bind39EffectModule__set_custom_uv_offset_implEPNS_26BattleObjectModuleAccessorEjRKN3phx8Vector2fEi") LINKABLE;
        u64 set_disable_render_offset_last(u64) asm("_ZN3app8lua_bind49EffectModule__set_disable_render_offset_last_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set_disable_system_slow(u64,bool) asm("_ZN3app8lua_bind42EffectModule__set_disable_system_slow_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_frame(u64,u64,float) asm("_ZN3app8lua_bind28EffectModule__set_frame_implEPNS_26BattleObjectModuleAccessorEjf") LINKABLE;
        u64 set_offset_to_next(u64,int) asm("_ZN3app8lua_bind37EffectModule__set_offset_to_next_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_pos(u64,u64,const Vector3f*) asm("_ZN3app8lua_bind26EffectModule__set_pos_implEPNS_26BattleObjectModuleAccessorEjRKN3phx8Vector3fE") LINKABLE;
        u64 set_post_effect_line_circle_target(u64,u64,Vector2f,Vector3f,bool) asm("_ZN3app8lua_bind53EffectModule__set_post_effect_line_circle_target_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ENS3_8Vector2fENS3_8Vector3fEb") LINKABLE;
        u64 set_rate(u64,u64,float) asm("_ZN3app8lua_bind27EffectModule__set_rate_implEPNS_26BattleObjectModuleAccessorEjf") LINKABLE;
        u64 set_rate_last(u64,float) asm("_ZN3app8lua_bind32EffectModule__set_rate_last_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
        u64 set_rgb(u64,u64,float,float,float) asm("_ZN3app8lua_bind26EffectModule__set_rgb_implEPNS_26BattleObjectModuleAccessorEjfff") LINKABLE;
        u64 set_rgb_partial_last(u64,float,float,float) asm("_ZN3app8lua_bind39EffectModule__set_rgb_partial_last_implEPNS_26BattleObjectModuleAccessorEfff") LINKABLE;
        u64 set_rot(u64,u64,const Vector3f*) asm("_ZN3app8lua_bind26EffectModule__set_rot_implEPNS_26BattleObjectModuleAccessorEjRKN3phx8Vector3fE") LINKABLE;
        u64 set_scale(u64,u64,const Vector3f*) asm("_ZN3app8lua_bind28EffectModule__set_scale_implEPNS_26BattleObjectModuleAccessorEjRKN3phx8Vector3fE") LINKABLE;
        u64 set_sync_scale(u64,bool) asm("_ZN3app8lua_bind33EffectModule__set_sync_scale_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_sync_visibility(u64,bool) asm("_ZN3app8lua_bind38EffectModule__set_sync_visibility_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_visible(u64,u64,bool) asm("_ZN3app8lua_bind30EffectModule__set_visible_implEPNS_26BattleObjectModuleAccessorEjb") LINKABLE;
        u64 set_visible_kind(u64,u64,bool) asm("_ZN3app8lua_bind35EffectModule__set_visible_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb") LINKABLE;
        u64 set_whole(u64,bool) asm("_ZN3app8lua_bind28EffectModule__set_whole_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_whole_attr(u64,bool,u64) asm("_ZN3app8lua_bind33EffectModule__set_whole_attr_implEPNS_26BattleObjectModuleAccessorEbj") LINKABLE;
    }
}