#ifndef LUA_HELPER_H
#define LUA_HELPER_H
#include <switch.h>
#include "l2c_imports.hpp"

void get_lua_stack(lib::L2CAgent* l2c_agent, int index, lib::L2CValue* l2c_val) {
    asm("mov x8, %x0" : : "r"(l2c_val) : "x8" );
    l2c_agent->pop_lua_stack(index);
}

#endif // LUA_HELPER_H
