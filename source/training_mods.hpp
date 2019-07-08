#ifndef TRAINING_MODS_H
#define TRAINING_MODS_H

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif
#include <stdarg.h>
#include "useful/const_value_table.h"
#include "useful/crc32.h"
#include "useful/useful.h"

#include "useful/raygun_printer.hpp"

#include "acmd_wrapper.hpp"
#include "imports/lib/l2c.hpp"
#include "saltysd/saltysd_dynamic.h"
#include "saltysd/saltysd_helper.hpp"
#include "taunt_toggles.h"

#include "training/common.hpp"
#include "training/save_states.hpp"
#include "training/directional_influence.hpp"
#include "training/ledge.hpp"
#include "training/mash.hpp"
#include "training/selection.hpp"
#include "training/shield.hpp"

using namespace lib;
using namespace app::lua_bind;
using namespace app::sv_animcmd;

namespace app::lua_bind {
namespace WorkModule {
// Force DI
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

void enable_transition_term_replace(u64 module_accessor, int transition_id) {
    Ledge::enable_transition_term(module_accessor, transition_id);

    u64 work_module = load_module(module_accessor, 0x50);
    void (*enable_transition_term)(u64, int) = (void (*) (u64, int)) load_module_impl(work_module, 0x188);
    enable_transition_term(work_module, transition_id);
}
}  // namespace WorkModule

namespace ControlModule {
int get_attack_air_kind_replace(u64 module_accessor) {
    bool replace;
    int kind = Mash::get_attack_air_kind(module_accessor, replace);
    if (replace) return kind;

    u64 control_module = load_module(module_accessor, 0x48);
    int (*get_attack_air_kind)(u64) = (int (*)(u64)) load_module_impl(control_module, 0x3B0);
    return get_attack_air_kind(control_module);
}

int get_command_flag_cat_replace(u64 module_accessor, int category) {
    //save_states(module_accessor);

    // Pause Effect AnimCMD if hitbox visualization is active
    int status_kind = StatusModule::status_kind(module_accessor);
    MotionAnimcmdModule::set_sleep_effect(module_accessor, 
        is_training_mode() &&
        HITBOX_VIS &&
        !(status_kind >= FIGHTER_STATUS_KIND_CATCH && status_kind <= FIGHTER_STATUS_KIND_TREAD_FALL));

    u64 control_module = load_module(module_accessor, 0x48);
    int (*get_command_flag_cat)(u64, int) = (int (*)(u64, int)) load_module_impl(control_module, 0x350);
    int flag = get_command_flag_cat(control_module, category);

    Mash::get_command_flag_cat(module_accessor, category, flag);
    Ledge::get_command_flag_cat(module_accessor, category, flag);

    return flag;
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
}  // namespace app::lua_bind

namespace app::lua_bind::MotionModule {
void change_motion_replace(u64 module_accessor, u64 motion_kind, float start_frame, float frame_speed_mult, bool unk1, float unk2, bool unk3, bool unk4) {
    Selection::change_motion(module_accessor, motion_kind);

    u64 motion_module = load_module(module_accessor, 0x88);
    void (*change_motion)(u64, u64, float, float, bool, float, bool, bool) =
        (void (*)(u64, u64, float, float, bool, float, bool, bool)) load_module_impl(motion_module, 0xD8);

    change_motion(motion_module, motion_kind, start_frame, frame_speed_mult, unk1, unk2, unk3, unk4);
}
}  // namespace app::lua_bind::MotionModule

void training_mods_main() {
    fighter_manager_addr = SaltySDCore_FindSymbol(
        "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E");
	SaltySD_function_replace_sym(
        "_ZN3app8lua_bind32MotionModule__change_motion_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Effbfbb",
        (u64)&MotionModule::change_motion_replace);

    // Mash airdodge/jump
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind40ControlModule__get_command_flag_cat_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&ControlModule::get_command_flag_cat_replace);

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

    // Ledge options
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind39WorkModule__enable_transition_term_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&WorkModule::enable_transition_term_replace);

    // Mash attack
    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind39ControlModule__get_attack_air_kind_implEPNS_26BattleObjectModuleAccessorE",
        (u64)&ControlModule::get_attack_air_kind_replace);

    Selection::menu_replace();
}

#endif  // TRAINING_MODS_H
