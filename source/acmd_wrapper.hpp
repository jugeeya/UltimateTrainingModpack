#ifndef ACMD_WRAPPER_H
#define ACMD_WRAPPER_H

#include <switch.h>

#include "imports/app/sv_animcmd.hpp"
#include "imports/app/sv_system.hpp"
#include "imports/app/sv_math.hpp"
#include "imports/app/lua_bind.hpp"
#include "imports/lib/l2c.hpp"

#include <initializer_list>

using namespace lib;

u64 load_module(u64 module_accessor, u64 module_offset) {
	return LOAD64(module_accessor + module_offset);
}

void* load_module_impl(u64 module, u64 function_offset) {
	u64 function_impl = LOAD64(module) + function_offset;
	return (void*) LOAD64(function_impl);
}

bool is_before_frame(u64 lua_state, float f) {
	u64 acmd_frame_obj = LOAD64(LOAD64(lua_state - 8) + 432LL);
	return *(float*)((*((u32 *)acmd_frame_obj + 64) + 15) & 0xFFFFFFF0) < f; 
}

struct ACMD {
	L2CAgent* l2c_agent;
	u64 module_accessor;

	ACMD(L2CAgent* agent) {
		l2c_agent = agent;
		module_accessor = app::sv_system::battle_object_module_accessor(l2c_agent->lua_state_agent);
	}

	void frame(float f) {
		l2c_agent->clear_lua_stack();
		L2CValue frame_val(f);
		l2c_agent->push_lua_stack(&frame_val);
		app::sv_animcmd::frame(l2c_agent->lua_state_agent, f);
		l2c_agent->clear_lua_stack();
	}

	// attempted reimplementation of sv_animcmd::frame
	bool _frame(float f) {
        u64 acmd_obj = LOAD64(LOAD64(l2c_agent->lua_state_agent - 8) + 432);
        if ( !is_before_frame(l2c_agent->lua_state_agent, f) )
            return true;
        
        *(u8*)(acmd_obj + 47) = 3;
        LOAD64(acmd_obj + 48) = (u64)&is_before_frame;
        *(float*)(acmd_obj + 56) = f;
        
        u64 acmd_obj_other = LOAD64(acmd_obj);
        if (*(u8*)(acmd_obj_other + 664)) {
            void (*some_func)(u64) = (void (*)(u64)) LOAD64(LOAD64(LOAD64(acmd_obj_other + 656)) + 16);
            some_func(LOAD64(acmd_obj_other + 656));
            return true;
        }

        bool doThing = false;

        if ( !*(u16*)(l2c_agent->lua_state_agent + 196)) {
            u64 v4 = LOAD64(l2c_agent->lua_state_agent + 32);
            *(u8*)(l2c_agent->lua_state_agent + 12) = 1;
            LOAD64(v4 + 56) = LOAD64(v4) - LOAD64(l2c_agent->lua_state_agent + 56);
            if ( *(u8*)(v4 + 66) & 2)
                return true;
            doThing = true;
        }


        if ( doThing || LOAD64(LOAD64(l2c_agent->lua_state_agent + 24) + 200) == l2c_agent->lua_state_agent) {
            // throw 
            u64 v4 = LOAD64(l2c_agent->lua_state_agent + 32);
            LOAD64(v4 + 32) = 0;
            LOAD64(v4) = LOAD64(l2c_agent->lua_state_agent + 16) - 16;
            
        }

        return false;

        // throw
    }

	void wait(float f) {
		l2c_agent->clear_lua_stack();
		L2CValue frame_val(f);
		l2c_agent->push_lua_stack(&frame_val);
		app::sv_animcmd::wait(l2c_agent->lua_state_agent, f);
		l2c_agent->clear_lua_stack();
	}

	bool is_excute() {
		l2c_agent->clear_lua_stack();
		app::sv_animcmd::is_excute(l2c_agent->lua_state_agent);
		L2CValue is_excute;
		l2c_agent->get_lua_stack(1, &is_excute);
		bool excute = (bool)(is_excute);
		l2c_agent->clear_lua_stack();
		return excute;
	}

	void wrap(u64 (*acmd_func)(u64), std::initializer_list<L2CValue> list) {
		l2c_agent->clear_lua_stack(); 
		for (L2CValue elem : list)
			l2c_agent->push_lua_stack(&elem);

		acmd_func(l2c_agent->lua_state_agent);
		l2c_agent->clear_lua_stack();    
	}

	void ATTACK(
		u64 i1,  // ID
		u64 i2,  // Part
		u64 h1,  // Bone
		float f1,  // Damage
		u64 i3,  // Angle
		u64 i4,  // KBG
		u64 i5,  // FKB
		u64 i6,  // BKB
		float f2,  // Size
		float f3,  // X
		float f4,  // Y
		float f5,  // Z
		// X2
		// Y2
		// Z2
		float f6,  // Hitlag
		float f7,  // SDI
		u64 i7,  // Clang/Rebound
		u64 i8,  // Facing Restriction
		u64 i9,  // Fixed Weight
		u64 i10, // Shield Damage
		float f8,  // Trip Chance
		u64 i11, // Rehite Rate
		u64 i12, // Reflectable
		u64 i13, // Absorbable
		u64 i14, // Flinchless
		u64 i15, // Disable Hitlag
		u64 i16, // Direct
		u64 i17, // Ground/Air
		u64 i18, // Hit Bits
		u64 i19, // Collision Bits
		u64 i20, // Friendly Fire
		u64 h2,  // Effect
		u64 i21, // SFX Level
		u64 i22, // SFX Type
		u64 i23) {  // Move Type
		wrap(app::sv_animcmd::ATTACK, {
			L2CValue(i1), L2CValue(i2), L2CValue(h1), L2CValue(f1),
			L2CValue(i3), L2CValue(i4), L2CValue(i5), L2CValue(i6),
			L2CValue(f2), L2CValue(f3), L2CValue(f4), L2CValue(f5),
			L2CValue("void"), L2CValue("void"), L2CValue("void"), L2CValue(f6),
			L2CValue(f7), L2CValue(i7), L2CValue(i8), L2CValue(i9),
			L2CValue(i10), L2CValue(f8), L2CValue(i11), L2CValue(i12),
			L2CValue(i13), L2CValue(i14), L2CValue(i15), L2CValue(i16),
 			L2CValue(i17), L2CValue(i18), L2CValue(i19), L2CValue(i20),
			L2CValue(h2), L2CValue(i21), L2CValue(i22), L2CValue(i23)
		});
	}

	void ATTACK(
		u64 i1,  // ID
		u64 i2,  // Part
		u64 h1,  // Bone
		float f1,  // Damage
		u64 i3,  // Angle
		u64 i4,  // KBG
		u64 i5,  // FKB
		u64 i6,  // BKB
		float f2,  // Size
		float f3,  // X
		float f4,  // Y
		float f5,  // Z
		float fX2, // X2
		float fY2, // Y2
		float fZ2, // Z2
		float f6,  // Hitlag
		float f7,  // SDI
		u64 i7,  // Clang/Rebound
		u64 i8,  // Facing Restriction
		u64 i9,  // Fixed Weight
		u64 i10, // Shield Damage
		float f8,  // Trip Chance
		u64 i11, // Rehite Rate
		u64 i12, // Reflectable
		u64 i13, // Absorbable
		u64 i14, // Flinchless
		u64 i15, // Disable Hitlag
		u64 i16, // Direct
		u64 i17, // Ground/Air
		u64 i18, // Hit Bits
		u64 i19, // Collision Bits
		u64 i20, // Friendly Fire
		u64 h2,  // Effect
		u64 i21, // SFX Level
		u64 i22, // SFX Type
		u64 i23) {  // Move Type
		wrap(app::sv_animcmd::ATTACK, {
			L2CValue(i1), L2CValue(i2), L2CValue(h1), L2CValue(f1),
			L2CValue(i3), L2CValue(i4), L2CValue(i5), L2CValue(i6),
			L2CValue(f2), L2CValue(f3), L2CValue(f4), L2CValue(f5),
			L2CValue(fX2), L2CValue(fY2), L2CValue(fZ2), L2CValue(f6),
			L2CValue(f7), L2CValue(i7), L2CValue(i8), L2CValue(i9),
			L2CValue(i10), L2CValue(f8), L2CValue(i11), L2CValue(i12),
			L2CValue(i13), L2CValue(i14), L2CValue(i15), L2CValue(i16),
 			L2CValue(i17), L2CValue(i18), L2CValue(i19), L2CValue(i20),
			L2CValue(h2), L2CValue(i21), L2CValue(i22), L2CValue(i23)
		});
	}
};

#endif // ACMD_WRAPPER_H
