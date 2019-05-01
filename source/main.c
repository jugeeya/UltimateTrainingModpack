#include <switch.h>

#include <dirent.h>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <switch/kernel/ipc.h>
#include <sys/iosupport.h>
#include <sys/reent.h>

#include "crc32.h"
#include "useful.h"

#include "saltysd_core.h"
#include "saltysd_dynamic.h"
#include "saltysd_ipc.h"

#include "lua/lstate.h"
#include "lua/lua.h"

#include "acmd_imports.h"
#include "l2c.h"
#include "l2c_imports.h"
#include "lua_bind.h"
#include "lua_helper.h"
#include "raygun_printer.h"
#include "saltysd_helper.h"

#include "taunt_toggles.h"

u32 __nx_applet_type = AppletType_None;

static char g_heap[0x8000];

Handle orig_main_thread;
void *orig_ctx;
void *orig_saved_lr;

u64 effect_manager_addr;
u64 fighter_manager_addr;

u64 sv_set_function_hash_addr;

void (*AttackModule_set_attack_lua_state)(u64, u64);

void (*AttackModule_clear_all_orig)(u64);
void (*AttackModule_clear_orig)(u64, int);

#define PI 3.14159265358979323846

void __libnx_init(void *ctx, Handle main_thread, void *saved_lr) {
  extern char *fake_heap_start;
  extern char *fake_heap_end;

  fake_heap_start = &g_heap[0];
  fake_heap_end = &g_heap[sizeof g_heap];

  orig_ctx = ctx;
  orig_main_thread = main_thread;
  orig_saved_lr = saved_lr;

  // Call constructors.
  void __libc_init_array(void);
  __libc_init_array();
}

void __attribute__((weak)) NORETURN __libnx_exit(int rc) {
  // Call destructors.
  void __libc_fini_array(void);
  __libc_fini_array();

  SaltySD_printf("SaltySD Plugin: jumping to %p\n", orig_saved_lr);

  __nx_exit(0, orig_saved_lr);
  while (true)
    ;
}

Vector3f id_colors[8] = {
    {1.0f, 0.0f, 0.0f},       {0.7843f, 0.3529f, 1.0f},
    {1.0f, 0.7843f, 0.7843f}, {0.0f, 1.0f, 0.8431f},
    {1.0f, 0.4706f, 0.0f},    {0.7843f, 0.7059f, 0.0f},
    {0.7843f, 0.0f, 1.0f},    {0.3765f, 0.2863f, 0.5294f},
};

#define is_training_mode _ZN3app9smashball16is_training_modeEv
extern u64 _ZN3app9smashball16is_training_modeEv(void) LINKABLE;

void AttackModule_clear_all_replace(u64 attack_module) {
  AttackModule_clear_all_orig(attack_module);

  if (is_training_mode()) {
    u64 module_accessor = LOAD64(attack_module + 0x8);

    // Clear graphics every time we clear all hitboxes.
    Hash40 shieldEffectHash = {.hash = 0xAFAE75F05LL};
    EffectModule_kill_kind(module_accessor, shieldEffectHash.hash, 0, 1);
  }
}

void AttackModule_clear_replace(u64 attack_module, int id) {
  AttackModule_clear_orig(attack_module, id);

  // TODO: Kill effect based on hitbox ID
}

void lib_L2CAgent_push_color(L2CAgent *l2c_agent, Vector3f color) {
  L2CValue red = {.raw_float = color.x, .type = L2C_number};
  L2CValue green = {.raw_float = color.y, .type = L2C_number};
  L2CValue blue = {.raw_float = color.z, .type = L2C_number};

  lib_L2CAgent_push_lua_stack(l2c_agent, &red);
  lib_L2CAgent_push_lua_stack(l2c_agent, &green);
  lib_L2CAgent_push_lua_stack(l2c_agent, &blue);
}

void generate_hitbox_effects(L2CAgent *l2c_agent, L2CValue *id, L2CValue *bone,
                             L2CValue *size, L2CValue *x, L2CValue *y,
                             L2CValue *z, L2CValue *x2, L2CValue *y2,
                             L2CValue *z2) {
  float sizeMult = 19.0 / 200.0;
  Hash40 shieldEffectHash = {.hash = 0xAFAE75F05LL};

  L2CValue shieldEffect = {.raw = shieldEffectHash.hash, .type = L2C_hash};
  L2CValue xRot = {.raw_float = 0.0, .type = L2C_number};
  L2CValue yRot = {.raw_float = 0.0, .type = L2C_number};
  L2CValue zRot = {.raw_float = 0.0, .type = L2C_number};
  L2CValue terminate = {.raw = 1, .type = L2C_bool};
  L2CValue effectSize = {.raw_float = (float)size->raw_float * sizeMult,
                         .type = L2C_number};

  L2CValue rate = {.raw_float = 8.0f, .type = L2C_number};

  // Extended Hitboxes if x2, y2, z2 are not L2CValue::nil
  int num_effects;
  if (x2->type != L2C_void && y2->type != L2C_void && z2->type != L2C_void) {
    num_effects = 4;
  } else {
    *x2 = *x;
    *y2 = *y;
    *z2 = *z;
    num_effects = 1;
  }

  for (int i = 0; i < num_effects; i++) {
    // EFFECT_FOLLOW_NO_SCALE(graphic, bone, x, y, z, xrot, yrot, zrot, size,
    // terminate)
    L2CValue currX = {
        .raw_float = x->raw_float + ((x2->raw_float - x->raw_float) / 3 * i),
        .type = L2C_number};
    L2CValue currY = {
        .raw_float = y->raw_float + ((y2->raw_float - y->raw_float) / 3 * i),
        .type = L2C_number};
    L2CValue currZ = {
        .raw_float = z->raw_float + ((z2->raw_float - z->raw_float) / 3 * i),
        .type = L2C_number};

    lib_L2CAgent_clear_lua_stack(l2c_agent);
    lib_L2CAgent_push_lua_stack(l2c_agent, &shieldEffect);
    lib_L2CAgent_push_lua_stack(l2c_agent, bone);
    lib_L2CAgent_push_lua_stack(l2c_agent, &currX);
    lib_L2CAgent_push_lua_stack(l2c_agent, &currY);
    lib_L2CAgent_push_lua_stack(l2c_agent, &currZ);
    lib_L2CAgent_push_lua_stack(l2c_agent, &xRot);
    lib_L2CAgent_push_lua_stack(l2c_agent, &yRot);
    lib_L2CAgent_push_lua_stack(l2c_agent, &zRot);
    lib_L2CAgent_push_lua_stack(l2c_agent, &effectSize);
    lib_L2CAgent_push_lua_stack(l2c_agent, &terminate);
    app_sv_animcmd_EFFECT_FOLLOW_NO_SCALE(l2c_agent->lua_state_agent);

    // Set to hitbox ID color
    // LAST_EFFECT_SET_COLOR(Red, Green, Blue)
    lib_L2CAgent_clear_lua_stack(l2c_agent);
    lib_L2CAgent_push_color(l2c_agent, id_colors[id->raw % 8]);
    app_sv_animcmd_LAST_EFFECT_SET_COLOR(l2c_agent->lua_state_agent);

    // Speed up animation by rate to remove pulsing effect
    // LAST_EFFECT_SET_RATE(Rate)
    lib_L2CAgent_clear_lua_stack(l2c_agent);
    lib_L2CAgent_push_lua_stack(l2c_agent, &rate);
    app_sv_animcmd_LAST_EFFECT_SET_RATE(l2c_agent->lua_state_agent);
  }
}

void app_sv_animcmd_ATTACK_replace(u64 a1) {
  u64 v1;  // x19
  u64 v2;  // x9
  u64 i;   // x8

  // Instantiate our own L2CAgent with the given lua_State
  L2CAgent l2c_agent;
  lib_L2CAgent(&l2c_agent, a1);

  // Get all necessary hitbox params
  L2CValue id, bone, damage, angle, kbg, wkb, bkb, size, x, y, z, x2, y2, z2;
  get_lua_stack(&l2c_agent, 1, &id);
  get_lua_stack(&l2c_agent, 3, &bone);
  get_lua_stack(&l2c_agent, 4, &damage);
  get_lua_stack(&l2c_agent, 5, &angle);
  get_lua_stack(&l2c_agent, 6, &kbg);
  get_lua_stack(&l2c_agent, 7, &wkb);
  get_lua_stack(&l2c_agent, 8, &bkb);
  get_lua_stack(&l2c_agent, 9, &size);
  get_lua_stack(&l2c_agent, 10, &x);
  get_lua_stack(&l2c_agent, 11, &y);
  get_lua_stack(&l2c_agent, 12, &z);
  get_lua_stack(&l2c_agent, 13, &x2);
  get_lua_stack(&l2c_agent, 14, &y2);
  get_lua_stack(&l2c_agent, 15, &z2);

  // original code: parse lua stack and call AttackModule::set_attack()
  v1 = a1;
  AttackModule_set_attack_lua_state(LOAD64(LOAD64(a1 - 8) + 416LL), a1);

  if (HITBOX_VIS && is_training_mode()) {
    // Replace AttackModule::clear_all() and AttackModule::clear(int)
    u64 module_accessor = LOAD64(LOAD64(a1 - 8) + 416LL);
    u64 attack_module = LOAD64(module_accessor + 0xA0);
    u64 attack_module_clear_all = LOAD64(attack_module) + 0x50LL;
    if (AttackModule_clear_all_orig == 0) {
      AttackModule_clear_all_orig =
          (void (*)(u64))(LOAD64(attack_module_clear_all));
    }
    LOAD64(attack_module_clear_all) = (u64)AttackModule_clear_all_replace;

    u64 attack_module_clear = LOAD64(attack_module) + 0x58LL;
    if (AttackModule_clear_orig == 0) {
      AttackModule_clear_orig =
          (void (*)(u64, int))(LOAD64(attack_module_clear));
    }
    LOAD64(attack_module_clear) = (u64)AttackModule_clear_replace;

    // Generate hitbox effect(s)
    generate_hitbox_effects(&l2c_agent, &id, &bone, &size, &x, &y, &z, &x2, &y2,
                            &z2);
  }

  // original code: clear_lua_stack section
  v2 = LOAD64(v1 + 16);
  for (i = **(u64 **)(v1 + 32) + 16LL; v2 < i; v2 = LOAD64(v1 + 16)) {
    LOAD64(v1 + 16) = v2 + 16;
    *(__int32_t *)(v2 + 8) = 0;
  }
  LOAD64(v1 + 16) = i;
}

bool is_operation_cpu(u64 module_accessor) {
  // entry_id_var: 0x10000000;
  int entry_id_var;
  if (lib_lua_bind_get_value(0xF370199DB936C5ED, &entry_id_var) & 1) {
    int entry_id = WorkModule_get_int(module_accessor, entry_id_var);
    u64 fighter_information = FighterManager_get_fighter_information(
        LOAD64(fighter_manager_addr), entry_id);
    return FighterInformation_is_operation_cpu(fighter_information);
  }

  return false;
}

u64 WorkModule_enable_transition_term_group_impl_replace(u64 module_accessor,
                                                         u64 transition_group) {
  if (TOGGLE_STATE == MASH_AIRDODGE && is_training_mode()) {
    // 0x1F00000D for airdodge
    if (transition_group == 0x1F00000D) {
      if (is_operation_cpu(module_accessor)) {
        int status_kind = StatusModule_status_kind(module_accessor);
        // Damage -> DamageFall
        if (status_kind >= 0x48 && status_kind <= 0x50)
          StatusModule_change_status_request_from_script(module_accessor, 0x22,
                                                         1);
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

  EffectModule_req_on_joint(module_accessor, raygunShot.hash, top.hash, &pos,
                            &rot, size, &random, &random, 0, 0, 0, 0);
}

float WorkModule_get_float_replace(u64 module_accessor, int var) {
  if (DI_STATE != NONE && is_training_mode()) {
    if (is_operation_cpu(module_accessor)) {
      int status_kind = StatusModule_status_kind(module_accessor);
      // Damage -> DamageFall
      if (status_kind >= 0x48 && status_kind <= 0x50) {
        int DI_stick_x_ID;
        int DI_stick_y_ID;
        float angle = (DI_STATE - 1) * PI / 4.0;

        if (lib_lua_bind_get_value(0xA4D50A730E36970E, &DI_stick_x_ID) &&
            var == DI_stick_x_ID)
          return cos(angle);

        if (lib_lua_bind_get_value(0xEED4095B229D825B, &DI_stick_y_ID) &&
            var == DI_stick_y_ID)
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
    // currently has issues: sometimes crashes. We'll use global parameters
    // instead.
    /*
    DI_STATE = (DI_STATE + 1) % NUM_DI_STATES;
    if (DI_STATE != NONE) {
        float angle = (DI_STATE - 1) * PI / 4.0;
        show_angle(module_accessor, -10*sin(angle)+10, 10*cos(angle), -1 * angle
    * 180.0 / PI );
    }
    */
  }

  // call original WorkModule::enable_transition_term_group_impl
  u64 motion_module = LOAD64(module_accessor + 0x88);
  u64 change_motion_impl = LOAD64(motion_module) + 0xD8LL;

  void (*motion_module_change_motion_impl)(u64, u64, float, float, bool, float,
                                           bool, bool) =
      (void (*)(u64, u64, float, float, bool, float, bool, bool))(
          LOAD64(change_motion_impl));

  motion_module_change_motion_impl(motion_module, hash, start_frame,
                                   frame_speed_mult, unk1, unk2, unk3, unk4);
}

int main(int argc, char *argv[]) {
  SaltySD_printf("SaltySD Plugin: alive\n");

  // Get anchor for imports
  ANCHOR_ABS = SaltySDCore_getCodeStart();

  const char *ver = "Ver. %d.%d.%d";
  u64 dst_3 = SaltySDCore_findCode((u8 *)ver, strlen(ver));
  if (dst_3) {
    SaltySD_Memcpy(dst_3, (u64) "Noice v%d%d%d", 13);
  }

  effect_manager_addr = SaltySDCore_FindSymbol("_ZN3lib9SingletonINS_13EffectManagerEE9instance_E");
  fighter_manager_addr = SaltySDCore_FindSymbol("_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E");

  AttackModule_set_attack_lua_state =
      (void (*)(u64, u64))SaltySDCore_FindSymbol("_ZN3app10sv_animcmd6ATTACKEP9lua_State") + 0xD0 - 0x70;

  // Install animCMD function replacement
  SaltySD_function_replace_sym("_ZN3app10sv_animcmd6ATTACKEP9lua_State",
                               (u64)&app_sv_animcmd_ATTACK_replace);

  SaltySD_function_replace_sym(
      "_ZN3app8lua_bind45WorkModule__enable_transition_term_group_implEPNS_26BattleObjectModuleAccessorEi",
      (u64)&WorkModule_enable_transition_term_group_impl_replace);

  SaltySD_function_replace_sym(
      "_ZN3app8lua_bind26WorkModule__get_float_implEPNS_26BattleObjectModuleAccessorEi",
      (u64)&WorkModule_get_float_replace);

  SaltySD_function_replace_sym(
      "_ZN3app8lua_bind32MotionModule__change_motion_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Effbfbb",
      (u64)&MotionModule_change_motion_replace);

  __libnx_exit(0);
}
