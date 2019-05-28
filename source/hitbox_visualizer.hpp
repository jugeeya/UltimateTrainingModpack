#include <math.h>

#include "l2c.hpp"
#include "l2c_imports.hpp"
#include "acmd_wrapper.hpp"
#include "saltysd_helper.hpp"
#include "const_value_table.h"
#include "taunt_toggles.h"
#include "useful.h"

using namespace lib;
using namespace app::lua_bind;
using namespace app::sv_animcmd;

void (*AttackModule_set_attack_lua_state)(u64, u64);

Vector3f ID_COLORS[8] = { // used to tint the hitbox effects -- make sure that at least one component is equal to 1.0
	{ 1.0f, 0.0f, 0.0f }, // #ff0000 (red)
	{ 1.0f, 0.4f, 0.0f }, // #ff9900 (orange)
	{ 0.8f, 1.0f, 0.0f }, // #ccff00 (yellow)
	{ 0.2f, 1.0f, 0.2f }, // #00ff33 (green)
	{ 0.0f, 0.8f, 1.0f }, // #00ccff (sky blue)
	{ 0.4f, 0.4f, 1.0f }, // #6666ff (blue)
	{ 0.8f, 0.0f, 1.0f }, // #cc00ff (purple)
	{ 1.0f, 0.2f, 0.8f }, // #ff33cc (pink)
};
int MAX_EFFECTS_PER_HITBOX = 16; // max # of circles drawn for an extended hitbox

namespace app::lua_bind::AttackModule {
	// clear graphics every time we clear all hitboxes
	void clear_all_replace(u64 module_accessor) {
		if (is_training_mode()) {
			// only if we're not shielding
			int status_kind = StatusModule::status_kind(module_accessor);
			if (!(status_kind >= FIGHTER_STATUS_KIND_GUARD_ON && status_kind <= FIGHTER_STATUS_KIND_GUARD_OFF)) {
				Hash40 shieldEffectHash = { .hash = 0xAFAE75F05LL };
				EffectModule::kill_kind(module_accessor, shieldEffectHash.hash, 0, 1);
			}
		}

		// call original AttackModule::clear_all_impl
		u64 attack_module = load_module(module_accessor, 0xA0);
		void (*clear_all)(u64) = (void(*)(u64))(load_module_impl(attack_module, 0x50));

		return clear_all(attack_module);
	}
}

void generate_hitbox_effects(L2CAgent *l2c_agent, L2CValue *bone, L2CValue *size,
		L2CValue *x, L2CValue *y, L2CValue *z, L2CValue *x2, L2CValue *y2, L2CValue *z2,
		Vector3f *color) {
	L2CValue red(color->x);
	L2CValue green(color->y);
	L2CValue blue(color->z);

	float sizeMult = 19.0f / 200.0f;
	Hash40 shieldEffectHash = { .hash = 0xAFAE75F05LL };

	L2CValue shieldEffect(shieldEffectHash.hash);
	L2CValue xRot(0.0f);
	L2CValue yRot(0.0f);
	L2CValue zRot(0.0f);
	L2CValue terminate(true);
	L2CValue effectSize((float)size->raw_float * sizeMult);

	L2CValue rate(8.0f);

	float xDist, yDist, zDist;
	int nEffects;
	if (x2->type != L2C_void && y2->type != L2C_void && z2->type != L2C_void) { // extended hitbox
		xDist = x2->raw_float - x->raw_float;
		yDist = y2->raw_float - y->raw_float;
		zDist = z2->raw_float - z->raw_float;
		float dist = sqrtf(xDist * xDist + yDist * yDist + zDist * zDist);
		nEffects = (int)ceilf(dist / (size->raw_float * 1.75f)) + 1; // just enough effects to form a continuous line
		if (nEffects < 2)
		    nEffects = 2;
		if (nEffects > MAX_EFFECTS_PER_HITBOX)
		    nEffects = MAX_EFFECTS_PER_HITBOX;
	} else { // non-extended hitbox
		xDist = yDist = zDist = 0;
		nEffects = 1;
	}

	for (int i = 0; i < nEffects; i++) {
		float t = nEffects <= 1 ? 0 : (float)i / (nEffects - 1);
		L2CValue currX(x->raw_float + xDist * t);
		L2CValue currY(y->raw_float + yDist * t);
		L2CValue currZ(z->raw_float + zDist * t);

		ACMD acmd(l2c_agent);
		acmd.wrap(EFFECT_FOLLOW_NO_SCALE, { shieldEffect, *bone, currX, currY, currZ, xRot, yRot, zRot, effectSize, terminate });

		// set to hitbox ID color
		acmd.wrap(LAST_EFFECT_SET_COLOR, { red, green, blue });

		// speed up animation by rate to remove pulsing effect
		acmd.wrap(LAST_EFFECT_SET_RATE, { rate });
	}
}

namespace app::sv_animcmd {
	void ATTACK_replace(u64 a1) {
		// instantiate our own L2CAgent with the given lua_State
		L2CAgent l2c_agent;
		l2c_agent.L2CAgent_constr(a1);

		// get all necessary hitbox params
		L2CValue id, bone, damage, /* angle, kbg, wkb, bkb, */ size, x, y, z, x2, y2, z2;
		l2c_agent.get_lua_stack(1, &id);
		l2c_agent.get_lua_stack(3, &bone);
		l2c_agent.get_lua_stack(4, &damage);
		// l2c_agent.get_lua_stack(5, &angle);
		// l2c_agent.get_lua_stack(6, &kbg);
		// l2c_agent.get_lua_stack(7, &wkb);
		// l2c_agent.get_lua_stack(8, &bkb);
		l2c_agent.get_lua_stack(9, &size);
		l2c_agent.get_lua_stack(10, &x);
		l2c_agent.get_lua_stack(11, &y);
		l2c_agent.get_lua_stack(12, &z);
		l2c_agent.get_lua_stack(13, &x2);
		l2c_agent.get_lua_stack(14, &y2);
		l2c_agent.get_lua_stack(15, &z2);

		// original code: parse lua stack and call AttackModule::set_attack()
		AttackModule_set_attack_lua_state(LOAD64(LOAD64(a1 - 8) + 416LL), a1);

		if (HITBOX_VIS && is_training_mode()) { // generate hitbox effect(s)
			float colorT = 0.375f + 0.625f * unlerpBounded(1.0f, 17.0f, damage.raw_float);
			Vector3f color = colorLerp({ 1.0f, 1.0f, 1.0f }, ID_COLORS[id.raw % 8], colorT, 0.875f); // color saturation scales with hitbox damage
		    generate_hitbox_effects(&l2c_agent, &bone, &size, &x, &y, &z, &x2, &y2, &z2, &color);
		}

		u64 v1, v2, i;
		v1 = a1;

		// original code: clear_lua_stack section
		v2 = LOAD64(v1 + 16);
		for (i = **(u64 **)(v1 + 32) + 16LL; v2 < i; v2 = LOAD64(v1 + 16)) {
		    LOAD64(v1 + 16) = v2 + 16;
		    *(__int32_t *)(v2 + 8) = 0;
		}
		LOAD64(v1 + 16) = i;
	}
}

void hitbox_vis_main() {
	AttackModule_set_attack_lua_state = (void (*)(u64, u64))SaltySDCore_FindSymbol("_ZN3app10sv_animcmd6ATTACKEP9lua_State") + 0xD0 - 0x70;
	SaltySD_function_replace_sym(
		"_ZN3app10sv_animcmd6ATTACKEP9lua_State",
		(u64)&ATTACK_replace);
	SaltySD_function_replace_sym(
		"_ZN3app8lua_bind28AttackModule__clear_all_implEPNS_26BattleObjectModuleAccessorE",
		(u64)&AttackModule::clear_all_replace);
}
