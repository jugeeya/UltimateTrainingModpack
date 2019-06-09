#ifndef ITEMMODULE_H
#define ITEMMODULE_H
namespace app::lua_bind {
    namespace ItemModule {
        u64 attach_item(u64,int itemKind,int,bool) asm("_ZN3app8lua_bind28ItemModule__attach_item_implEPNS_26BattleObjectModuleAccessorENS_8ItemKindEib") LINKABLE;
        u64 attach_item_2(u64,u64 item,bool) asm("_ZN3app8lua_bind30ItemModule__attach_item_2_implEPNS_26BattleObjectModuleAccessorEPNS_4ItemEb") LINKABLE;
        u64 attach_item_instance(u64,u64 item,bool) asm("_ZN3app8lua_bind37ItemModule__attach_item_instance_implEPNS_26BattleObjectModuleAccessorEPNS_4ItemEb") LINKABLE;
        u64 born_item(u64,int) asm("_ZN3app8lua_bind26ItemModule__born_item_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 drop_attach_group(u64,unsigned char,float,float) asm("_ZN3app8lua_bind34ItemModule__drop_attach_group_implEPNS_26BattleObjectModuleAccessorEhff") LINKABLE;
        u64 drop_item(u64,float,float,int) asm("_ZN3app8lua_bind26ItemModule__drop_item_implEPNS_26BattleObjectModuleAccessorEffi") LINKABLE;
        u64 eject_attach(u64,int itemKind,bool,bool) asm("_ZN3app8lua_bind29ItemModule__eject_attach_implEPNS_26BattleObjectModuleAccessorENS_8ItemKindEbb") LINKABLE;
        u64 eject_have_item(u64,int,bool,bool) asm("_ZN3app8lua_bind32ItemModule__eject_have_item_implEPNS_26BattleObjectModuleAccessorEibb") LINKABLE;
        u64 get_have_item_id(u64,int) asm("_ZN3app8lua_bind33ItemModule__get_have_item_id_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 get_have_item_kind(u64,int) asm("_ZN3app8lua_bind35ItemModule__get_have_item_kind_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 get_have_item_size(u64,int) asm("_ZN3app8lua_bind35ItemModule__get_have_item_size_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        //u64 get_have_item_status_param_bool(u64,app::ItemStatusParamBool,int) asm("_ZN3app8lua_bind48ItemModule__get_have_item_status_param_bool_implEPNS_26BattleObjectModuleAccessorENS_19ItemStatusParamBoolEi") LINKABLE;
        //u64 get_have_item_status_param_float(u64,app::ItemStatusParamFloat,int) asm("_ZN3app8lua_bind49ItemModule__get_have_item_status_param_float_implEPNS_26BattleObjectModuleAccessorENS_20ItemStatusParamFloatEi") LINKABLE;
        //u64 get_have_item_status_param_int(u64,app::ItemStatusParamInt,int) asm("_ZN3app8lua_bind47ItemModule__get_have_item_status_param_int_implEPNS_26BattleObjectModuleAccessorENS_18ItemStatusParamIntEi") LINKABLE;
        u64 get_have_item_trait(u64,int) asm("_ZN3app8lua_bind36ItemModule__get_have_item_trait_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 get_pickable_item_kind(u64) asm("_ZN3app8lua_bind39ItemModule__get_pickable_item_kind_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_pickable_item_object_id(u64) asm("_ZN3app8lua_bind44ItemModule__get_pickable_item_object_id_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_pickable_item_size(u64) asm("_ZN3app8lua_bind39ItemModule__get_pickable_item_size_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_shoot_item_bullet(u64,int) asm("_ZN3app8lua_bind38ItemModule__get_shoot_item_bullet_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 have_item(u64,int itemKind,int,int,bool,bool) asm("_ZN3app8lua_bind26ItemModule__have_item_implEPNS_26BattleObjectModuleAccessorENS_8ItemKindEiibb") LINKABLE;
        u64 have_item_instance(u64,u64 item,int,bool,bool,bool,bool) asm("_ZN3app8lua_bind35ItemModule__have_item_instance_implEPNS_26BattleObjectModuleAccessorEPNS_4ItemEibbbb") LINKABLE;
        u64 is_have_item(u64,int) asm("_ZN3app8lua_bind29ItemModule__is_have_item_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 is_success_auto_pickup_item(u64) asm("_ZN3app8lua_bind44ItemModule__is_success_auto_pickup_item_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 is_success_pickup_item(u64) asm("_ZN3app8lua_bind39ItemModule__is_success_pickup_item_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 pickup_item(u64,int itemSize,int,int,int itemQuickItemTreatType,int itemPickupSearchMode) asm("_ZN3app8lua_bind28ItemModule__pickup_item_implEPNS_26BattleObjectModuleAccessorENS_8ItemSizeEiiNS_18QuickItemTreatTypeENS_20ItemPickupSearchModeE") LINKABLE;
        u64 remove_all(u64) asm("_ZN3app8lua_bind27ItemModule__remove_all_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 remove_item(u64,int) asm("_ZN3app8lua_bind28ItemModule__remove_item_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 reset_have_item_constraint_node(u64,int) asm("_ZN3app8lua_bind48ItemModule__reset_have_item_constraint_node_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 set_attach_item_action(u64,int itemKind,int,float) asm("_ZN3app8lua_bind39ItemModule__set_attach_item_action_implEPNS_26BattleObjectModuleAccessorENS_8ItemKindEif") LINKABLE;
        u64 set_attach_item_visibility(u64,bool,unsigned char) asm("_ZN3app8lua_bind43ItemModule__set_attach_item_visibility_implEPNS_26BattleObjectModuleAccessorEbh") LINKABLE;
        u64 set_change_status_event(u64,bool) asm("_ZN3app8lua_bind40ItemModule__set_change_status_event_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_have_item_action(u64,int,float,int) asm("_ZN3app8lua_bind37ItemModule__set_have_item_action_implEPNS_26BattleObjectModuleAccessorEifi") LINKABLE;
        u64 set_have_item_constraint_joint(u64,u64,int) asm("_ZN3app8lua_bind47ItemModule__set_have_item_constraint_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei") LINKABLE;
        u64 set_have_item_hold_anim(u64,bool,int) asm("_ZN3app8lua_bind40ItemModule__set_have_item_hold_anim_implEPNS_26BattleObjectModuleAccessorEbi") LINKABLE;
        u64 set_have_item_scale_anim(u64,int,float,int) asm("_ZN3app8lua_bind41ItemModule__set_have_item_scale_anim_implEPNS_26BattleObjectModuleAccessorEifi") LINKABLE;
        u64 set_have_item_visibility(u64,bool,int) asm("_ZN3app8lua_bind41ItemModule__set_have_item_visibility_implEPNS_26BattleObjectModuleAccessorEbi") LINKABLE;
        u64 shoot_item_bullet(u64,int,float,int) asm("_ZN3app8lua_bind34ItemModule__shoot_item_bullet_implEPNS_26BattleObjectModuleAccessorEifi") LINKABLE;
        u64 shoot_item_bullet_blanks(u64,int,int) asm("_ZN3app8lua_bind41ItemModule__shoot_item_bullet_blanks_implEPNS_26BattleObjectModuleAccessorEii") LINKABLE;
        u64 success_auto_pickup_item(u64) asm("_ZN3app8lua_bind41ItemModule__success_auto_pickup_item_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 throw_item(u64,float,float,float,int,bool,float) asm("_ZN3app8lua_bind27ItemModule__throw_item_implEPNS_26BattleObjectModuleAccessorEfffibf") LINKABLE;
        u64 update_have_item_action_info(u64,int) asm("_ZN3app8lua_bind45ItemModule__update_have_item_action_info_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        u64 use_item(u64,int,bool) asm("_ZN3app8lua_bind25ItemModule__use_item_implEPNS_26BattleObjectModuleAccessorEib") LINKABLE;
        u64 use_item_instance(u64,u64 item,bool) asm("_ZN3app8lua_bind34ItemModule__use_item_instance_implEPNS_26BattleObjectModuleAccessorEPNS_4ItemEb") LINKABLE;
        }
    }
#endif // ITEMMODULE_H
