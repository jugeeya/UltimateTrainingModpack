#include <switch.h>

#include "acmd_imports.hpp"
#include "l2c_imports.hpp"
#include "lua_helper.hpp"

using namespace lib;

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
    void frame(float f) {
        app::sv_animcmd::frame(l2c_agent->lua_state_agent, f);
        l2c_agent->clear_lua_stack();
    }

    bool is_excute() {
        app::sv_animcmd::is_excute(l2c_agent->lua_state_agent);
        L2CValue is_excute;
        get_lua_stack(l2c_agent, 1, &is_excute);
        bool excute = is_excute.raw;
        l2c_agent->clear_lua_stack();
        return excute;
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
            {.type = L2C_integer, .raw = i1},    // ID
            {.type = L2C_integer, .raw = i2},    // Unk
            {.type = L2C_hash, .raw = h1},   // Joint
            {.type = L2C_number, .raw_float = f1}, // Damage
            {.type = L2C_integer, .raw = i3},   // Angle
            {.type = L2C_integer, .raw = i4},   // KBG
            {.type = L2C_integer, .raw = i5},    // WBKB
            {.type = L2C_integer, .raw = i6},   // BKB
            {.type = L2C_number, .raw_float = f2}, // Size
            {.type = L2C_number, .raw_float = f3},   // X
            {.type = L2C_number, .raw_float = f4}, // Y
            {.type = L2C_number, .raw_float = f5},   // Z
            {.type = L2C_void, .raw = 0},   // X2
            {.type = L2C_void, .raw = 0},   // Y2
            {.type = L2C_void, .raw = 0},   // Z2
            {.type = L2C_number, .raw_float = f6},   // Hitlag
            {.type = L2C_number, .raw_float = f7},   // SDI
            {.type = L2C_integer, .raw = i7},
            {.type = L2C_integer, .raw = i8},
            {.type = L2C_integer, .raw = i9},
            {.type = L2C_integer, .raw = i10},
            {.type = L2C_number, .raw_float = f8},
            {.type = L2C_integer, .raw = i11},
            {.type = L2C_integer, .raw = i12},
            {.type = L2C_integer, .raw = i13},
            {.type = L2C_integer, .raw = i14},
            {.type = L2C_integer, .raw = i15},
            {.type = L2C_integer, .raw = i16},
            {.type = L2C_integer, .raw = i17},
            {.type = L2C_integer, .raw = i18},
            {.type = L2C_integer, .raw = i19},
            {.type = L2C_integer, .raw = i20},
            {.type = L2C_hash, .raw = h2},
            {.type = L2C_integer, .raw = i21},
            {.type = L2C_integer, .raw = i22},
            {.type = L2C_integer, .raw = i23},
        };

        for (size_t i = 0; i < 36; i++)
            l2c_agent->push_lua_stack(&hitbox_params[i]);

        app::sv_animcmd::ATTACK(l2c_agent->lua_state_agent);

        l2c_agent->clear_lua_stack();
    }
};