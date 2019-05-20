#include "l2c.hpp"
#include "saltysd_helper.hpp"
#include "l2c_imports.hpp"
#include "acmd_imports.hpp"
#include "taunt_toggles.h"
#include "raygun_printer.hpp"

using namespace lib;
using namespace app::lua_bind;
using namespace app::sv_animcmd;

u64 fighter_manager_addr;

bool is_operation_cpu(u64 module_accessor) {
    int entry_id = WorkModule::get_int(module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    u64 fighter_information = FighterManager::get_fighter_information(LOAD64(fighter_manager_addr), entry_id);

    return FighterInformation::is_operation_cpu(fighter_information);
}

bool is_in_hitstun(u64 module_accessor) {
  int status_kind = StatusModule::status_kind(module_accessor);
  return status_kind >= FIGHTER_STATUS_KIND_DAMAGE && status_kind <= FIGHTER_STATUS_KIND_DAMAGE_FALL;
}

namespace app::lua_bind::WorkModule {
  // Force option out of hitstun
  u64 enable_transition_term_group_replace(u64 module_accessor, int transition_group) {
    if (is_training_mode() && is_operation_cpu(module_accessor)) {
      if (is_in_hitstun(module_accessor)) {
        // Airdodge
        if (TOGGLE_STATE == MASH_AIRDODGE) { 
          if (transition_group == FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE)
            StatusModule::change_status_request_from_script(module_accessor, FIGHTER_STATUS_KIND_ESCAPE_AIR, 1);
        } 
        // Jump
        else if (TOGGLE_STATE == MASH_JUMP) {
          if  (transition_group == FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL)
            StatusModule::change_status_request_from_script(module_accessor, FIGHTER_STATUS_KIND_JUMP_AERIAL, 1);
          else if (transition_group == FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP)
            StatusModule::change_status_request_from_script(module_accessor, FIGHTER_STATUS_KIND_JUMP_SQUAT, 1);
        }
      }
    }

    // call original WorkModule::enable_transition_term_group_impl
    u64 work_module = load_module(module_accessor, 0x50);
    u64 (*enable_transition_term_group)(u64, u64) = (u64(*)(u64, u64))(load_module_impl(work_module, 0x140));

    return enable_transition_term_group(work_module, transition_group);
  }
  
  // Force DI
  float get_float_replace(u64 module_accessor, int var) {
    if (is_training_mode() && is_operation_cpu(module_accessor)) {
      if (is_in_hitstun(module_accessor)) {
        if (DI_STATE != NONE) {
          float angle = (DI_STATE - 1) * M_PI / 4.0;

          // Either 0 (right) or PI (left)
          if (DI_STATE = DI_RANDOM_IN_AWAY) {
            angle = (rand() % 2) * M_PI;
          }

          // If facing left, reverse angle
          if (PostureModule::lr(module_accessor) != -1.0)
            angle -= M_PI;

          if (var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X)
            return cos(angle);

          if (var == FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y)
            return sin(angle);
        }
      }
    }

    // call original WorkModule::get_float_impl
    u64 work_module = load_module(module_accessor, 0x50);
    float (*get_float)(u64, int) = (float (*)(u64, int))(load_module_impl(work_module, 0x58));

    return get_float(work_module, var);
  }
}

void training_mods_main() {
    fighter_manager_addr = SaltySDCore_FindSymbol("_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E");

    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind45WorkModule__enable_transition_term_group_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&WorkModule::enable_transition_term_group_replace);

    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind26WorkModule__get_float_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&WorkModule::get_float_replace);
}