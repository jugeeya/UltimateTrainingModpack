#pragma once

namespace app::lua_bind {
    namespace FighterMotionModuleImpl {
        u64 add_body_type_hash(u64,u64,int) asm("_ZN3app8lua_bind48FighterMotionModuleImpl__add_body_type_hash_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei") LINKABLE;
        u64 add_motion_partial_kirby_copy(u64,int,u64,float,float,bool,bool,float,bool,bool,bool) asm("_ZN3app8lua_bind59FighterMotionModuleImpl__add_motion_partial_kirby_copy_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Effbbfbbb") LINKABLE;
        u64 change_motion_force_inherit_frame_kirby_copy(u64,u64,float,float,float) asm("_ZN3app8lua_bind74FighterMotionModuleImpl__change_motion_force_inherit_frame_kirby_copy_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efff") LINKABLE;
        u64 change_motion_inherit_frame_keep_rate_kirby_copy(u64,u64,float,float,float) asm("_ZN3app8lua_bind78FighterMotionModuleImpl__change_motion_inherit_frame_keep_rate_kirby_copy_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efff") LINKABLE;
        u64 change_motion_inherit_frame_kirby_copy(u64,u64,float,float,float,bool,bool) asm("_ZN3app8lua_bind68FighterMotionModuleImpl__change_motion_inherit_frame_kirby_copy_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efffbb") LINKABLE;
        u64 change_motion_kirby_copy(u64,u64,float,float,bool,float,bool,bool) asm("_ZN3app8lua_bind54FighterMotionModuleImpl__change_motion_kirby_copy_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Effbfbb") LINKABLE;
        u64 end_frame_from_hash_kirby_copy(u64,u64) asm("_ZN3app8lua_bind60FighterMotionModuleImpl__end_frame_from_hash_kirby_copy_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E") LINKABLE;
        u64 get_cancel_frame(u64,u64,bool) asm("_ZN3app8lua_bind46FighterMotionModuleImpl__get_cancel_frame_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb") LINKABLE;
        u64 get_cancel_frame_kirby_copy(u64,u64,bool) asm("_ZN3app8lua_bind57FighterMotionModuleImpl__get_cancel_frame_kirby_copy_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb") LINKABLE;
        bool is_valid_cancel_frame(u64,int,bool) asm("_ZN3app8lua_bind51FighterMotionModuleImpl__is_valid_cancel_frame_implEPNS_26BattleObjectModuleAccessorEib") LINKABLE;
        u64 motion_kind_kirby_copy_original(u64,u64) asm("_ZN3app8lua_bind61FighterMotionModuleImpl__motion_kind_kirby_copy_original_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E") LINKABLE;
        u64 set_blend_waist(u64,bool) asm("_ZN3app8lua_bind45FighterMotionModuleImpl__set_blend_waist_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_frame_sync_anim_cmd_kirby_copy(u64,float,bool) asm("_ZN3app8lua_bind64FighterMotionModuleImpl__set_frame_sync_anim_cmd_kirby_copy_implEPNS_26BattleObjectModuleAccessorEfb") LINKABLE;
        u64 set_pause_motion_interpolation_stop(u64) asm("_ZN3app8lua_bind65FighterMotionModuleImpl__set_pause_motion_interpolation_stop_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 start_damage_stop_interpolation(u64,float) asm("_ZN3app8lua_bind61FighterMotionModuleImpl__start_damage_stop_interpolation_implEPNS_26BattleObjectModuleAccessorEf") LINKABLE;
    }
}