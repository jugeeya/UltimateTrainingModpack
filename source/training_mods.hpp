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

    SaltySD_function_replace_sym(
        "_ZN3app8lua_bind32MotionModule__change_motion_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Effbfbb",
        (u64)&MotionModule_change_motion_replace);
}

bool is_operation_cpu(u64 module_accessor) {
    int entry_id = WorkModule::get_int(module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    u64 fighter_information = FighterManager::get_fighter_information(
        LOAD64(fighter_manager_addr), entry_id);

    return FighterInformation::is_operation_cpu(fighter_information);
}

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

void show_angle(u64 module_accessor, float y, float x, float zrot) {
  Hash40 raygunShot = {.hash = 0x11e470b07fLL};
  Hash40 top = {.hash = 0x031ed91fcaLL};

  Vector3f pos = {.x = x, .y = y, .z = 0};
  Vector3f rot = {.x = 0, .y = 90, .z = zrot};
  Vector3f random = {.x = 0, .y = 0, .z = 0};

  float size = 0.5;

  EffectModule::req_on_joint(module_accessor, raygunShot.hash, top.hash, &pos,
                            &rot, size, &random, &random, 0, 0, 0, 0);
}

float WorkModule_get_float_replace(u64 module_accessor, int var) {
  if (is_training_mode()) {
    if (is_operation_cpu(module_accessor)) {
      int status_kind = StatusModule::status_kind(module_accessor);
      // Damage -> DamageFall
      if (status_kind >= 0x48 && status_kind <= 0x50) {
        float angle = 0;//(DI_STATE - 1) * PI / 4.0;

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

void MotionModule_change_motion_replace(u64 module_accessor, u64 hash,
                                        float start_frame,
                                        float frame_speed_mult, bool unk1,
                                        float unk2, bool unk3, bool unk4) {
  const char *down_taunt_l = "appeal_lw_l";
  const char *down_taunt_r = "appeal_lw_r";
  const char *up_taunt_l = "appeal_hi_l";
  const char *up_taunt_r = "appeal_hi_r";
  const char *side_taunt_l = "appeal_s_l";
  const char *side_taunt_r = "appeal_s_r";

  char buffer[16];
  // Down Taunt
  if (hash == hash40(down_taunt_l) || hash == hash40(down_taunt_r)) {
    TOGGLE_STATE = (TOGGLE_STATE + 1) % NUM_TOGGLE_STATES;
    if (TOGGLE_STATE)
      print_string(module_accessor, "MASH\nAIRDODGE");
    else
      print_string(module_accessor, "NONE");
  }
  // Up Taunt
  else if (hash == hash40(up_taunt_l) || hash == hash40(up_taunt_r)) {
    HITBOX_VIS = !HITBOX_VIS;
    if (HITBOX_VIS)
      print_string(module_accessor, "HITBOX\nVIS");
    else
      print_string(module_accessor, "NO\nHITBOX");
  }
  // Side Taunt
  else if (hash == hash40(side_taunt_l) || hash == hash40(side_taunt_r)) {
  }

  // call original WorkModule::enable_transition_term_group_impl
  u64 motion_module = LOAD64(module_accessor + 0x88);
  u64 change_motion_impl = LOAD64(motion_module) + 0xD8LL;

  void (*motion_module_change_motion_impl)(u64, u64, float, float, bool, float, bool, bool) =
      (void (*)(u64, u64, float, float, bool, float, bool, bool))(
          LOAD64(change_motion_impl));

  motion_module_change_motion_impl(motion_module, hash, start_frame, frame_speed_mult, unk1, unk2, unk3, unk4);
}