#pragma once

namespace app::lua_bind {
    namespace ArticleModule {
        u64 add_motion_2nd(u64,int,u64,float,float,bool,float) asm("_ZN3app8lua_bind34ArticleModule__add_motion_2nd_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Effbf") LINKABLE;
        u64 add_motion_partial(u64,int,int,u64,float,float,bool,bool,float,bool,bool,bool) asm("_ZN3app8lua_bind38ArticleModule__add_motion_partial_implEPNS_26BattleObjectModuleAccessorEiiN3phx6Hash40Effbbfbbb") LINKABLE;
        u64 change_motion(u64,int,u64,bool,float) asm("_ZN3app8lua_bind33ArticleModule__change_motion_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Ebf") LINKABLE;
        u64 change_status(u64,int,int,int articleOperationTarget) asm("_ZN3app8lua_bind33ArticleModule__change_status_implEPNS_26BattleObjectModuleAccessorEiiNS_22ArticleOperationTargetE") LINKABLE;
        u64 change_status_exist(u64,int,int) asm("_ZN3app8lua_bind39ArticleModule__change_status_exist_implEPNS_26BattleObjectModuleAccessorEii") LINKABLE;
        u64 generate_article(u64,int,bool,int) asm("_ZN3app8lua_bind36ArticleModule__generate_article_implEPNS_26BattleObjectModuleAccessorEibi") LINKABLE;
        u64 generate_article_enable(u64,int,bool,int) asm("_ZN3app8lua_bind43ArticleModule__generate_article_enable_implEPNS_26BattleObjectModuleAccessorEibi") LINKABLE;
        u64 generate_article_have_item(u64,int,int,u64) asm("_ZN3app8lua_bind46ArticleModule__generate_article_have_item_implEPNS_26BattleObjectModuleAccessorEiiN3phx6Hash40E") LINKABLE;
        u64 get_active_num(u64,int) asm("_ZN3app8lua_bind34ArticleModule__get_active_num_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 get_article(u64,int) asm("_ZN3app8lua_bind31ArticleModule__get_article_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 get_joint_pos(u64,int,u64,int articleOperationTarget) asm("_ZN3app8lua_bind33ArticleModule__get_joint_pos_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40ENS_22ArticleOperationTargetE") LINKABLE;
        u64 get_joint_rotate(u64,int,u64,int articleOperationTarget) asm("_ZN3app8lua_bind36ArticleModule__get_joint_rotate_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40ENS_22ArticleOperationTargetE") LINKABLE;
        u64 have(u64,int,u64,int articleOperationTarget,uint,bool) asm("_ZN3app8lua_bind24ArticleModule__have_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40ENS_22ArticleOperationTargetEjb") LINKABLE;
        bool is_exist(u64,int) asm("_ZN3app8lua_bind28ArticleModule__is_exist_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        bool is_generatable(u64,int) asm("_ZN3app8lua_bind34ArticleModule__is_generatable_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 motion_kind(u64,int,int articleOperationTarget) asm("_ZN3app8lua_bind31ArticleModule__motion_kind_implEPNS_26BattleObjectModuleAccessorEiNS_22ArticleOperationTargetE") LINKABLE;
        u64 motion_kind_2nd(u64,int,int articleOperationTarget) asm("_ZN3app8lua_bind35ArticleModule__motion_kind_2nd_implEPNS_26BattleObjectModuleAccessorEiNS_22ArticleOperationTargetE") LINKABLE;
        u64 remove(u64,int,int articleOperationTarget) asm("_ZN3app8lua_bind26ArticleModule__remove_implEPNS_26BattleObjectModuleAccessorEiNS_22ArticleOperationTargetE") LINKABLE;
        u64 remove_exist(u64,int,int articleOperationTarget) asm("_ZN3app8lua_bind32ArticleModule__remove_exist_implEPNS_26BattleObjectModuleAccessorEiNS_22ArticleOperationTargetE") LINKABLE;
        u64 remove_exist_object_id(u64,uint) asm("_ZN3app8lua_bind42ArticleModule__remove_exist_object_id_implEPNS_26BattleObjectModuleAccessorEj") LINKABLE;
        u64 set_flag(u64,int,bool,int) asm("_ZN3app8lua_bind28ArticleModule__set_flag_implEPNS_26BattleObjectModuleAccessorEibi") LINKABLE;
        u64 set_float(u64,int,float,int) asm("_ZN3app8lua_bind29ArticleModule__set_float_implEPNS_26BattleObjectModuleAccessorEifi") LINKABLE;
        u64 set_frame(u64,int,float) asm("_ZN3app8lua_bind29ArticleModule__set_frame_implEPNS_26BattleObjectModuleAccessorEif") LINKABLE;
        u64 set_frame_2nd(u64,int,float,bool) asm("_ZN3app8lua_bind33ArticleModule__set_frame_2nd_implEPNS_26BattleObjectModuleAccessorEifb") LINKABLE;
        u64 set_item_action(u64,int,int,float) asm("_ZN3app8lua_bind35ArticleModule__set_item_action_implEPNS_26BattleObjectModuleAccessorEiif") LINKABLE;
        u64 set_pos(u64,int,Vector3f) asm("_ZN3app8lua_bind27ArticleModule__set_pos_implEPNS_26BattleObjectModuleAccessorEiN3phx8Vector3fE") LINKABLE;
        u64 set_rate(u64,int,float) asm("_ZN3app8lua_bind28ArticleModule__set_rate_implEPNS_26BattleObjectModuleAccessorEif") LINKABLE;
        u64 set_visibility(u64,int,u64,u64,int articleOperationTarget) asm("_ZN3app8lua_bind34ArticleModule__set_visibility_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40ES4_NS_22ArticleOperationTargetE") LINKABLE;
        u64 set_visibility_whole(u64,int,bool,int articleOperationTarget) asm("_ZN3app8lua_bind40ArticleModule__set_visibility_whole_implEPNS_26BattleObjectModuleAccessorEibNS_22ArticleOperationTargetE") LINKABLE;
        u64 set_weight(u64,int,float,bool) asm("_ZN3app8lua_bind30ArticleModule__set_weight_implEPNS_26BattleObjectModuleAccessorEifb") LINKABLE;
        u64 shoot(u64,int,int articleOperationTarget,bool) asm("_ZN3app8lua_bind25ArticleModule__shoot_implEPNS_26BattleObjectModuleAccessorEiNS_22ArticleOperationTargetEb") LINKABLE;
        u64 shoot_exist(u64,int,int articleOperationTarget,bool) asm("_ZN3app8lua_bind31ArticleModule__shoot_exist_implEPNS_26BattleObjectModuleAccessorEiNS_22ArticleOperationTargetEb") LINKABLE;
    }
}