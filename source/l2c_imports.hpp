#ifndef L2C_IMPORTS_H
#define L2C_IMPORTS_H

#include <switch.h>

#include <math.h>

#include "l2c.hpp"
#include "lua_bind_hash.hpp"

#define LOAD64 *(u64 *)

u64 is_training_mode(void) asm("_ZN3app9smashball16is_training_modeEv") LINKABLE;

namespace lib {
	enum L2CVarType {
		L2C_void = 0,
		L2C_bool = 1,
		L2C_integer = 2,
		L2C_number = 3,
		L2C_pointer = 4,
		L2C_table = 5,
		L2C_inner_function = 6,
		L2C_hash = 7,
		L2C_string = 8,
	};

	struct L2CTable_meta {
		uint64_t a;
		uint64_t b;
		uint64_t c;
		uint64_t d;
	};
  
	struct L2CTable {
		uint32_t refcnt;
		uint32_t unk;

		uint64_t begin; // L2CValue*
		uint64_t end; // L2CValue*
		uint64_t also_end; // L2CValue*
		struct L2CTable_meta meta;
		uint64_t unk_ptr;
	};

	struct L2CInnerFunctionBase {
		uint64_t unk;
		uint32_t refcnt;
	} L2CInnerFunctionBase;

	struct L2CValue {
		uint32_t type;
		uint32_t unk;
		union {
			uint64_t raw;
			float raw_float;
			// void* raw_pointer;
			// struct L2CTable* raw_table;
			// struct L2CInnerFunctionBase* raw_innerfunc;
			//std::string* raw_string;
		};

		L2CValue() {
			type = L2C_void;
		}

		L2CValue(bool val) {
			type = L2C_bool;
			raw = val;
		}

		L2CValue(int val) {
			type = L2C_integer;
			raw = val;
		}

		L2CValue(u64 val) {
			type = L2C_integer;
			raw = val;
		}

		L2CValue(float val) {
			if (isnan(val)) {
				type = L2C_void;
			} else {
				type = L2C_number;
				raw_float = val;
			}
		}

		L2CValue(double val) {
			if (isnan(val)) {
				type = L2C_void;
			} else {
				type = L2C_number;
				raw_float = val;
			}
		}

		operator bool() asm("_ZNK3lib8L2CValuecvbEv") LINKABLE;

		void push_variadic(u64, const char*, void*) asm("_ZN3lib8L2CValue13push_variadicEmPKcRNS_7utility8VariadicE") LINKABLE;
	};

	struct L2CAgent {
		uint64_t vtable;
		uint64_t lua_state_agent;
		uint64_t unk10;
		uint64_t unk18;
		uint64_t unk20;
		uint64_t unk28;
		uint64_t unk30;
		uint64_t unk38;
		uint64_t lua_state_agentbase;

		L2CAgent* L2CAgent_constr(u64 lua_state) asm("_ZN3lib8L2CAgentC2EP9lua_State") LINKABLE;
		u64 push_lua_stack(L2CValue* l2c_value) asm("_ZN3lib8L2CAgent14push_lua_stackERKNS_8L2CValueE") LINKABLE;
		// pop_lua_stack
		// Notes: 
		// Actually takes three arguments, but the third is given through X8 due to how 
		// AArch64 treats struct pointers that are used as pass by reference to get the value.
		// Thus, my current solution is to use inline ASM before using this to pass the 
		// last arg. This is done using asm("mov x8, %x0" : : "r"(&popped) : "x8" );, where
		// popped is an L2CValue that will be populated by the function.
		// FURTHERMORE, this function does NOT actually pop the stack, it only returns the value at the
		// position indicated by the second argument.
		// This index is either positive, meaning absolute position in the stack, or negative,
		// which is more traditional, i.e. -1 is the top of the stack.
		//__int64_t (*lib_L2CAgent_pop_lua_stack)(__int64_t, int);
		u64 pop_lua_stack(int index) asm("_ZN3lib8L2CAgent13pop_lua_stackEi") LINKABLE;

		void get_lua_stack(int index, lib::L2CValue* l2c_val) {
			asm("mov x8, %x0" : : "r"(l2c_val) : "x8" );
			pop_lua_stack(index);
		}

		u64 sv_set_function_hash(u64 (*func)(L2CAgent*, void*), u64 hash) asm("_ZN3lib8L2CAgent20sv_set_function_hashEPvN3phx6Hash40E") LINKABLE;
		u64 clear_lua_stack() asm("_ZN3lib8L2CAgent15clear_lua_stackEv") LINKABLE;
	};

	bool lua_bind_get_value(u64, int*) asm("_ZN3lib18lua_bind_get_valueIiEEbmRT_") LINKABLE;

	int lua_const(const char* str) {
		int val;
		if (lua_bind_get_value(lua_bind_hash_str(str), &val))
			return val;
		else
			return -1;
	}
}

#endif // L2C_IMPORTS_H
