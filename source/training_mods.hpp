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

u64 WorkModule_enable_transition_term_group_replace(u64 module_accessor, u64 transition_group);
float WorkModule_get_float_replace(u64 module_accessor, int var);
void MotionModule_change_motion_replace(u64 module_accessor, u64 hash,
                                        float start_frame,
                                        float frame_speed_mult, bool unk1,
                                        float unk2, bool unk3, bool unk4);

void training_mods_main() {
    fighter_manager_addr = SaltySDCore_FindSymbol("_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E");

    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind45WorkModule__enable_transition_term_group_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&WorkModule_enable_transition_term_group_replace);

    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind26WorkModule__get_float_implEPNS_26BattleObjectModuleAccessorEi",
        (u64)&WorkModule_get_float_replace);
}

bool is_operation_cpu(u64 module_accessor) {
    int entry_id = WorkModule::get_int(module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    u64 fighter_information = FighterManager::get_fighter_information(
        LOAD64(fighter_manager_addr), entry_id);

    return FighterInformation::is_operation_cpu(fighter_information);
}

// Force airdodge
u64 WorkModule_enable_transition_term_group_replace(u64 module_accessor,
                                                         u64 transition_group) {
  if (TOGGLE_STATE == MASH_AIRDODGE && is_training_mode()) {
    // 0x1F00000D for airdodge
    if (transition_group == 0x1F00000D) {
      if (is_operation_cpu(module_accessor)) {
        int status_kind = StatusModule::status_kind(module_accessor);
        // Damage -> DamageFall
        if (status_kind >= 0x48 && status_kind <= 0x50) {
          StatusModule::change_status_request_from_script(module_accessor, 0x22, 1);
        }
      }
    }
  }

  // call original WorkModule::enable_transition_term_group_impl
  u64 work_module = LOAD64(module_accessor + 0x50);
  u64 enable_transition_term_group_impl = LOAD64(work_module) + 0x140LL;

  u64 (*work_module_enable_transition_term_group_impl)(u64, u64) =
      (u64(*)(u64, u64))(LOAD64(enable_transition_term_group_impl));

  return work_module_enable_transition_term_group_impl(work_module,
                                                       transition_group);
}

// Force DI
float WorkModule_get_float_replace(u64 module_accessor, int var) {
  if (is_training_mode() && DI_STATE != NONE) {
    if (is_operation_cpu(module_accessor)) {
      int status_kind = StatusModule::status_kind(module_accessor);
      // Damage -> DamageFall
      if (status_kind >= 0x48 && status_kind <= 0x50) {
        float angle = (DI_STATE - 1) * M_PI / 4.0;

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
  u64 work_module = LOAD64(module_accessor + 0x50);
  u64 get_float_impl = LOAD64(work_module) + 0x58LL;

  float (*work_module_get_float_impl)(u64, int) =
      (float (*)(u64, int))(LOAD64(get_float_impl));

  return work_module_get_float_impl(work_module, var);
}