#include "l2c.hpp"
#include "saltysd_helper.hpp"
#include "l2c_imports.hpp"
#include "acmd_imports.hpp"
#include "taunt_toggles.h"

using namespace lib;
using namespace app::lua_bind;
using namespace app::sv_animcmd;

void (*AttackModule_set_attack_lua_state)(u64, u64);

void (*AttackModule_clear_all_orig)(u64);
void (*AttackModule_clear_orig)(u64, int);

Vector3f id_colors[8] = {
    {1.0f, 0.0f, 0.0f},       {0.7843f, 0.3529f, 1.0f},
    {1.0f, 0.7843f, 0.7843f}, {0.0f, 1.0f, 0.8431f},
    {1.0f, 0.4706f, 0.0f},    {0.7843f, 0.7059f, 0.0f},
    {0.7843f, 0.0f, 1.0f},    {0.3765f, 0.2863f, 0.5294f},
};

void app_sv_animcmd_ATTACK_replace(u64 a1);
void AttackModule_clear_all_replace(u64 module_accessor);
void AttackModule_clear_replace(u64 module_accessor, int id, bool unk);

void hitbox_vis_main() {
    AttackModule_set_attack_lua_state =
      (void (*)(u64, u64))SaltySDCore_FindSymbol("_ZN3app10sv_animcmd6ATTACKEP9lua_State") + 0xD0 - 0x70;

    SaltySD_function_replace_sym(
      "_ZN3app10sv_animcmd6ATTACKEP9lua_State",
      (u64)&app_sv_animcmd_ATTACK_replace);

    SaltySD_function_replace_sym(
      "_ZN3app8lua_bind28AttackModule__clear_all_implEPNS_26BattleObjectModuleAccessorE",
      (u64)&AttackModule_clear_all_replace);
}

void AttackModule_clear_all_replace(u64 module_accessor) {
  u64 attack_module = LOAD64(module_accessor + 0xA0);
  u64 attack_module_clear_all = LOAD64(attack_module) + 0x50LL;
  u64 (*attack_module_clear_all_impl)(u64) =
    (u64(*)(u64))(LOAD64(attack_module_clear_all));

  attack_module_clear_all_impl(attack_module);

  if (is_training_mode()) {
    // Clear graphics every time we clear all hitboxes.
    // Only if we're not shielding.
    int status_kind = StatusModule::status_kind(module_accessor);
    if (!(status_kind >= 0x1b && status_kind <= 0x1d)) {
      Hash40 shieldEffectHash = {.hash = 0xAFAE75F05LL};
      EffectModule::kill_kind(module_accessor, shieldEffectHash.hash, 0, 1);
    }
  }
}

void push_color(L2CAgent *l2c_agent, Vector3f color) {
  L2CValue red = {.type = L2C_number, .raw_float = color.x};
  L2CValue green = {.type = L2C_number, .raw_float = color.y};
  L2CValue blue = {.type = L2C_number, .raw_float = color.z};

  l2c_agent->push_lua_stack(&red);
  l2c_agent->push_lua_stack(&green);
  l2c_agent->push_lua_stack(&blue);
}

void generate_hitbox_effects(L2CAgent *l2c_agent, L2CValue *id, L2CValue *bone,
                             L2CValue *size, L2CValue *x, L2CValue *y,
                             L2CValue *z, L2CValue *x2, L2CValue *y2,
                             L2CValue *z2) {
  float sizeMult = 19.0 / 200.0;
  Hash40 shieldEffectHash = {.hash = 0xAFAE75F05LL};

  L2CValue shieldEffect = {.type = L2C_hash, .raw = shieldEffectHash.hash};
  L2CValue xRot = {.type = L2C_number, .raw_float = 0.0};
  L2CValue yRot = {.type = L2C_number, .raw_float = 0.0};
  L2CValue zRot = {.type = L2C_number, .raw_float = 0.0};
  L2CValue terminate = {.type = L2C_bool, .raw = 1};
  L2CValue effectSize = {.type = L2C_number, .raw_float = (float)size->raw_float * sizeMult};

  L2CValue rate = {.type = L2C_number, .raw_float = 8.0f};

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
        .type = L2C_number,
        .raw_float = x->raw_float + ((x2->raw_float - x->raw_float) / 3 * i)};
    L2CValue currY = {
        .type = L2C_number,
        .raw_float = y->raw_float + ((y2->raw_float - y->raw_float) / 3 * i)};
    L2CValue currZ = {
        .type = L2C_number,
        .raw_float = z->raw_float + ((z2->raw_float - z->raw_float) / 3 * i)};

    l2c_agent->clear_lua_stack();
    l2c_agent->push_lua_stack(&shieldEffect);
    l2c_agent->push_lua_stack(bone);
    l2c_agent->push_lua_stack(&currX);
    l2c_agent->push_lua_stack(&currY);
    l2c_agent->push_lua_stack(&currZ);
    l2c_agent->push_lua_stack(&xRot);
    l2c_agent->push_lua_stack(&yRot);
    l2c_agent->push_lua_stack(&zRot);
    l2c_agent->push_lua_stack(&effectSize);
    l2c_agent->push_lua_stack(&terminate);
    EFFECT_FOLLOW_NO_SCALE(l2c_agent->lua_state_agent);

    // Set to hitbox ID color
    // LAST_EFFECT_SET_COLOR(Red, Green, Blue)
    l2c_agent->clear_lua_stack();
    push_color(l2c_agent, id_colors[id->raw % 8]);
    LAST_EFFECT_SET_COLOR(l2c_agent->lua_state_agent);

    // Speed up animation by rate to remove pulsing effect
    // LAST_EFFECT_SET_RATE(Rate)
    l2c_agent->clear_lua_stack();
    l2c_agent->push_lua_stack(&rate);
    LAST_EFFECT_SET_RATE(l2c_agent->lua_state_agent);
  }
}

void app_sv_animcmd_ATTACK_replace(u64 a1) {
  u64 v1;  // x19
  u64 v2;  // x9
  u64 i;   // x8

  // Instantiate our own L2CAgent with the given lua_State
  L2CAgent l2c_agent;
  l2c_agent.L2CAgent_constr(a1);

  // Get all necessary hitbox params
  L2CValue id, bone, damage, angle, kbg, wkb, bkb, size, x, y, z, x2, y2, z2;
  l2c_agent.get_lua_stack(1, &id);
  l2c_agent.get_lua_stack(3, &bone);
  l2c_agent.get_lua_stack(4, &damage);
  l2c_agent.get_lua_stack(5, &angle);
  l2c_agent.get_lua_stack(6, &kbg);
  l2c_agent.get_lua_stack(7, &wkb);
  l2c_agent.get_lua_stack(8, &bkb);
  l2c_agent.get_lua_stack(9, &size);
  l2c_agent.get_lua_stack(10, &x);
  l2c_agent.get_lua_stack(11, &y);
  l2c_agent.get_lua_stack(12, &z);
  l2c_agent.get_lua_stack(13, &x2);
  l2c_agent.get_lua_stack(14, &y2);
  l2c_agent.get_lua_stack(15, &z2);

  // original code: parse lua stack and call AttackModule::set_attack()
  v1 = a1;
  AttackModule_set_attack_lua_state(LOAD64(LOAD64(a1 - 8) + 416LL), a1);

  if (HITBOX_VIS && is_training_mode()) {
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