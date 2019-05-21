#ifndef ACMD_WRAPPER_H
#define ACMD_WRAPPER_H

#include <switch.h>

#include "acmd_imports.hpp"
#include "l2c_imports.hpp"
#include "lua_helper.hpp"

#include <initializer_list>

using namespace lib;

u64 load_module(u64 module_accessor, u64 module_offset) {
    return LOAD64(module_accessor + module_offset);
}

void* load_module_impl(u64 module, u64 function_offset) {
    u64 function_impl = LOAD64(module) + function_offset;
    return (void*) LOAD64(function_impl);
}

namespace app::sv_system
{
    u64 battle_object_module_accessor(u64) asm("_ZN3app9sv_system29battle_object_module_accessorEP9lua_State") LINKABLE;
}

namespace app::sv_math
{
    int rand(u64 hash, int endIndex) asm("_ZN3app7sv_math4randEN3phx6Hash40Ei") LINKABLE;
}

namespace app::lua_bind
{
    namespace AttackModule
    {
        void clear_all(u64) asm("_ZN3app8lua_bind28AttackModule__clear_all_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
    }

    namespace ControlModule
    {
        bool check_button_on(u64, int) asm("_ZN3app8lua_bind35ControlModule__check_button_on_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
    }  

    namespace EffectModule
    {
        // boma, effect, joint, pos, rot, size, random_pos, random_rot, NO_SCALE?, attr?, unkint1, unkint2
        uint req_on_joint(u64, u64, u64, const Vector3f*, const Vector3f*, float a6, const Vector3f*, const Vector3f*, bool, uint, int, int) 
            asm("_ZN3app8lua_bind31EffectModule__req_on_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_RKNS3_8Vector3fES7_fS7_S7_bjii") LINKABLE;

        void kill_kind(u64, u64, bool, bool) 
            asm("_ZN3app8lua_bind28EffectModule__kill_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ebb") LINKABLE;
    }

    namespace FighterManager
    {
        u64 get_fighter_information(u64, int) asm("_ZN3app8lua_bind44FighterManager__get_fighter_information_implEPNS_14FighterManagerENS_14FighterEntryIDE") LINKABLE;
    }

    namespace FighterInformation
    {
        bool is_operation_cpu(u64) asm("_ZN3app8lua_bind41FighterInformation__is_operation_cpu_implEPNS_18FighterInformationE") LINKABLE;
    }

    namespace MotionModule 
    {
        float frame(u64) asm("_ZN3app8lua_bind24MotionModule__frame_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 motion_kind(u64) asm("_ZN3app8lua_bind30MotionModule__motion_kind_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
    }

    namespace PostureModule
    {
        float lr(u64) asm("_ZN3app8lua_bind22PostureModule__lr_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        float pos_x(u64) asm("_ZN3app8lua_bind25PostureModule__pos_x_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        float pos_y(u64) asm("_ZN3app8lua_bind25PostureModule__pos_y_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        float set_pos(u64, const Vector3f*) asm("_ZN3app8lua_bind27PostureModule__set_pos_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE") LINKABLE;
    }

    namespace StatusModule
    {
        u64 change_status_request_from_script(u64, int, bool) asm("_ZN3app8lua_bind52StatusModule__change_status_request_from_script_implEPNS_26BattleObjectModuleAccessorEib") LINKABLE;
        int status_kind(u64) asm("_ZN3app8lua_bind30StatusModule__status_kind_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
    }

    namespace WorkModule
    {
        bool get_int(u64, int) asm("_ZN3app8lua_bind24WorkModule__get_int_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
    }
}

struct ACMD
{
    L2CAgent* l2c_agent;
    u64 module_accessor;

    ACMD(L2CAgent* agent) {
        l2c_agent = agent;
        module_accessor = app::sv_system::battle_object_module_accessor(l2c_agent->lua_state_agent);
    }

    void frame(float f) {
        app::sv_animcmd::frame(l2c_agent->lua_state_agent, f);
        l2c_agent->clear_lua_stack();
    }

    bool is_excute() {
        app::sv_animcmd::is_excute(l2c_agent->lua_state_agent);
        L2CValue is_excute;
        l2c_agent->get_lua_stack(1, &is_excute);
        bool excute = is_excute.raw;
        l2c_agent->clear_lua_stack();
        return excute;
    }

    void wrap( u64 (*acmd_func)(u64), std::initializer_list<L2CValue> list )
    {
        l2c_agent->clear_lua_stack(); 
        for( L2CValue elem : list )
            l2c_agent->push_lua_stack(&elem);

        acmd_func(l2c_agent->lua_state_agent);
        l2c_agent->clear_lua_stack();    
    }

    void ATTACK(
        u64 i1,
        u64 i2,
        u64 h1,
        float f1,
        u64 i3,
        u64 i4,
        u64 i5,
        u64 i6,
        float f2,
        float f3,
        float f4,
        float f5,
        //void,
        //void,
        //void,
        float f6,
        float f7,
        u64 i7,
        u64 i8,
        u64 i9,
        u64 i10,
        float f8,
        u64 i11,
        u64 i12,
        u64 i13,
        u64 i14,
        u64 i15,
        u64 i16,
        u64 i17,
        u64 i18,
        u64 i19,
        u64 i20,
        u64 h2,
        u64 i21,
        u64 i22,
        u64 i23
    ) {
        L2CValue hitbox_params[36] = {
            L2CValue(i1),    // ID
            L2CValue(i2),    // Unk
            L2CValue(h1),   // Joint
            L2CValue(f1), // Damage
            L2CValue(i3),   // Angle
            L2CValue(i4),   // KBG
            L2CValue(i5),    // WBKB
            L2CValue(i6),   // BKB
            L2CValue(f2), // Size
            L2CValue(f3),   // X
            L2CValue(f4), // Y
            L2CValue(f5),   // Z
            L2CValue("void"),   // X2
            L2CValue("void"),   // Y2
            L2CValue("void"),   // Z2
            L2CValue(f6),   // Hitlag
            L2CValue(f7),   // SDI
            L2CValue(i7),
            L2CValue(i8),
            L2CValue(i9),
            L2CValue(i10),
            L2CValue(f8),
            L2CValue(i11),
            L2CValue(i12),
            L2CValue(i13),
            L2CValue(i14),
            L2CValue(i15),
            L2CValue(i16),
            L2CValue(i17),
            L2CValue(i18),
            L2CValue(i19),
            L2CValue(i20),
            L2CValue(h2),
            L2CValue(i21),
            L2CValue(i22),
            L2CValue(i23)
        };

        for (size_t i = 0; i < 36; i++)
            l2c_agent->push_lua_stack(&hitbox_params[i]);

        app::sv_animcmd::ATTACK(l2c_agent->lua_state_agent);

        l2c_agent->clear_lua_stack();
    }
};

#endif // ACMD_WRAPPER_H