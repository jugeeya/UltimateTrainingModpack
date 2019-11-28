#ifndef TRAINING_MODS_H
#define TRAINING_MODS_H

#include <stdarg.h>
#include "useful/const_value_table.h"
#include "useful/crc32.h"
#include "useful/useful.h"

#include "useful/raygun_printer.h"

#include "acmd_wrapper.h"
#include "lib/l2c_imports.h"
#include "saltysd/saltysd_dynamic.h"
#include "saltysd/saltysd_helper.h"
#include "taunt_toggles.h"

#include "training/common.h"
#include "training/save_states.h"
#include "training/directional_influence.h"
#include "training/ledge.h"
#include "training/mash.h"
#include "training/shield.h"
#include "training/tech.h"
#include "training/input_recorder.h"

using namespace lib;
using namespace app::lua_bind;
using namespace app::sv_animcmd;

u64 prev_get_command_flag_cat = 0;

namespace app::lua_bind {
namespace WorkModule {
float get_float_replace(u64 module_accessor, int var) {
    bool replace;
    float ret = DirectionalInfluence::get_float(module_accessor, var, replace);
    if (replace) return ret;

    u64 work_module = load_module(module_accessor, 0x50);
    float (*get_float)(u64, int) = (float (*)(u64, int)) load_module_impl(work_module, 0x58);
    return get_float(work_module, var);
}

float get_param_float_replace(u64 module_accessor, u64 param_type, u64 param_hash) {
    bool replace;
    float ret = Shield::get_param_float(module_accessor, param_type, param_hash, replace);
    if (replace) return ret;

    u64 work_module = load_module(module_accessor, 0x50);
    float (*get_param_float)(u64, u64, u64) = (float (*)(u64, u64, u64)) load_module_impl(work_module, 0x240);
    return get_param_float(work_module, param_type, param_hash);
}
}  // namespace WorkModule

namespace ControlModule {
int get_attack_air_kind_replace(u64 module_accessor) {
    bool replace;
    int kind = InputRecorder::get_attack_air_kind(module_accessor, replace);
    if (replace) return kind;

    kind = Mash::get_attack_air_kind(module_accessor, replace);
    if (replace) return kind;

    u64 control_module = load_module(module_accessor, 0x48);
    int (*get_attack_air_kind)(u64) = (int (*)(u64)) load_module_impl(control_module, 0x3B0);
    return get_attack_air_kind(control_module);
}

int get_command_flag_cat_replace(u64 module_accessor, int category) {
    int (*prev_replace)(u64, int) = (int (*)(u64, int)) prev_get_command_flag_cat;
    if (prev_replace)
        prev_replace(module_accessor, category);
    //save_states(module_accessor);

    // Pause Effect AnimCMD if hitbox visualization is active
    int status_kind = StatusModule::status_kind(module_accessor);
    MotionAnimcmdModule::set_sleep_effect(module_accessor, 
        is_training_mode() &&
        menu.HITBOX_VIS &&
        !(status_kind >= FIGHTER_STATUS_KIND_CATCH && status_kind <= FIGHTER_STATUS_KIND_TREAD_FALL));

    u64 control_module = load_module(module_accessor, 0x48);
    int (*get_command_flag_cat)(u64, int) = (int (*)(u64, int)) load_module_impl(control_module, 0x350);
    int flag = get_command_flag_cat(control_module, category);

    // bool replace;
    // int ret = InputRecorder::get_command_flag_cat(module_accessor, category, flag, replace);
    // if (replace) return ret;

    Mash::get_command_flag_cat(module_accessor, category, flag);
    Ledge::get_command_flag_cat(module_accessor, category, flag);
    Tech::get_command_flag_cat(module_accessor, category, flag);

    return flag;
}

int get_pad_flag(u64 module_accessor) {
    u64 control_module = load_module(module_accessor, 0x48);
    int (*get_pad_flag)(u64) = (int (*)(u64)) load_module_impl(control_module, 0x348);
    int pad_flag = get_pad_flag(control_module);

    bool replace;
    int ret = InputRecorder::get_pad_flag(module_accessor, replace);
    if (replace) return ret;

    return pad_flag;
}

float get_stick_x_replace(u64 module_accessor) {
    u64 control_module = load_module(module_accessor, 0x48);
    float (*get_stick_x)(u64) = (float (*)(u64)) load_module_impl(control_module, 0x178);
    float stick_x = get_stick_x(control_module);

    bool replace;
    float ret = InputRecorder::get_stick_x(module_accessor, replace);
    if (replace) return ret;

    return stick_x;
}

float get_stick_y_replace(u64 module_accessor) {
    u64 control_module = load_module(module_accessor, 0x48);
    float (*get_stick_y)(u64) = (float (*)(u64)) load_module_impl(control_module, 0x188);
    float stick_y = get_stick_y(control_module);

    bool replace;
    float ret = InputRecorder::get_stick_y(module_accessor, replace);
    if (replace) return ret;

    return stick_y;
}

bool check_button_on_replace(u64 module_accessor, int button) {
    bool replace;
    bool ret = Shield::check_button_on(module_accessor, button, replace);
    if (replace) return ret;
    ret = Mash::check_button_on(module_accessor, button, replace);
    if (replace) return ret;

    u64 control_module = load_module(module_accessor, 0x48);
    bool (*check_button_on)(u64, int) = (bool (*)(u64, int)) load_module_impl(control_module, 0x260);
    return check_button_on(control_module, button);
}

bool check_button_off_replace(u64 module_accessor, int button) {
    bool replace;
    bool ret = Shield::check_button_off(module_accessor, button, replace);
    if (replace) return ret;

    u64 control_module = load_module(module_accessor, 0x48);
    bool (*check_button_off)(u64, int) = (bool (*)(u64, int)) load_module_impl(control_module, 0x268);
    return check_button_off(control_module, button);
}
}  // namespace ControlModule

namespace StatusModule {
void init_settings_replace(u64 module_accessor, int situationKind, int unk1, uint unk2, int groundCliffCheckKind, bool unk3, int unk4, int unk5, int unk6, int unk7) {
    bool replace;
    Tech::init_settings(module_accessor, StatusModule::status_kind(module_accessor), replace);
    if (replace) return;

    u64 status_module = load_module(module_accessor, 0x40);
    void (*init_settings)(u64,int,int,uint,int,bool,int,int,int,int) =
        (void (*)(u64,int,int,uint,int,bool,int,int,int,int)) load_module_impl(status_module, 0x1C8);

    init_settings(status_module, situationKind, unk1, unk2, groundCliffCheckKind, unk3, unk4, unk5, unk6, unk7);
}
}  // namespace StatusModule

namespace MotionModule {
u64 change_motion_replace(u64 module_accessor, u64 motion_kind, float unk1, float unk2, bool unk3, float unk4, bool unk5, bool unk6) {
    bool replace;
    u64 motion_kind_ret = Tech::change_motion(module_accessor, motion_kind, replace);
    if (replace) motion_kind = motion_kind_ret;

    u64 motion_module = load_module(module_accessor, 0x88);
    u64 change_motion_offset = 0;
    if (major < 4) change_motion_offset = 0xD8;
    else change_motion_offset = 0xE0;
    
    u64 (*change_motion)(u64,u64,float,float,bool,float,bool,bool) = 
        (u64 (*)(u64,u64,float,float,bool,float,bool,bool)) load_module_impl(motion_module, change_motion_offset);

    return change_motion(motion_module, motion_kind, unk1, unk2, unk3, unk4, unk5, unk6);
}
}  // namespace MotionModule
}  // namespace app::lua_bind

void training_mods_main() {
    fighter_manager_addr = SaltySDCore_FindSymbol(
        "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E");

    // Mash airdodge/jump
    SaltySD_function_replace_sym_check_prev(
        "_ZN3app8lua_bind40ControlModule__get_command_flag_cat_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&ControlModule::get_command_flag_cat_replace,
        prev_get_command_flag_cat);

    // Set DI
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind26WorkModule__get_float_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&WorkModule::get_float_replace);

    // Hold/Infinite shield
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind35ControlModule__check_button_on_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&ControlModule::check_button_on_replace);
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind36ControlModule__check_button_off_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&ControlModule::check_button_off_replace);
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind32WorkModule__get_param_float_implEPNS_26BattleObjectModuleAccessorEmm",
        (u64)&WorkModule::get_param_float_replace);

    // Mash attack
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind39ControlModule__get_attack_air_kind_implEPNS_26BattleObjectModuleAccessorE",
        (u64)&ControlModule::get_attack_air_kind_replace);

    // Input recorder
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind31ControlModule__get_stick_x_implEPNS_26BattleObjectModuleAccessorE",
        (u64)&ControlModule::get_stick_x_replace);
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind31ControlModule__get_stick_y_implEPNS_26BattleObjectModuleAccessorE",
        (u64)&ControlModule::get_stick_y_replace);

    // Tech options
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind32StatusModule__init_settings_implEPNS_26BattleObjectModuleAccessorENS_13SituationKindEijNS_20GroundCliffCheckKindEbiiii",
        (u64)&StatusModule::init_settings_replace);
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind32MotionModule__change_motion_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Effbfbb",
        (u64)&MotionModule::change_motion_replace);
}

#endif  // TRAINING_MODS_H
