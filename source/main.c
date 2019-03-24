#include <switch.h>

#include <string.h>
#include <stdio.h>
#include <dirent.h>
#include <sys/iosupport.h>
#include <sys/reent.h>
#include <switch/kernel/ipc.h>

#include "useful.h"

#include "saltysd_core.h"
#include "saltysd_ipc.h"
#include "saltysd_dynamic.h"

#include "lua/lua.h"
#include "lua/lstate.h"

#include "l2c.h"

u32 __nx_applet_type = AppletType_None;

static char g_heap[0x8000];

Handle orig_main_thread;
void* orig_ctx;
void* orig_saved_lr;

void __libnx_init(void* ctx, Handle main_thread, void* saved_lr)
{
    extern char* fake_heap_start;
    extern char* fake_heap_end;

    fake_heap_start = &g_heap[0];
    fake_heap_end   = &g_heap[sizeof g_heap];
    
    orig_ctx = ctx;
    orig_main_thread = main_thread;
    orig_saved_lr = saved_lr;
    
    // Call constructors.
    void __libc_init_array(void);
    __libc_init_array();
}

void __attribute__((weak)) NORETURN __libnx_exit(int rc)
{
    // Call destructors.
    void __libc_fini_array(void);
    __libc_fini_array();

    SaltySD_printf("SaltySD Plugin: jumping to %p\n", orig_saved_lr);

    __nx_exit(0, orig_saved_lr);
}

extern uint64_t _ZN2nn2fs8ReadFileEPmNS0_10FileHandleElPvm(uint64_t idk1, uint64_t idk2, uint64_t idk3, uint64_t idk4, uint64_t idk5) LINKABLE;
extern uint64_t _ZN2nn2fs8ReadFileENS0_10FileHandleElPvm(uint64_t handle, uint64_t offset, uint64_t out, uint64_t size) LINKABLE;

extern uint64_t _ZN2nn4util14DecompressZlibEPvmPKvmS1_m(void * idk1, unsigned long idk2, void const* idk3, unsigned long idk4, void * idk5, unsigned long idk6) LINKABLE;
extern uint64_t _ZN2nn2ro10LoadModuleEPNS0_6ModuleEPKvPvmi(uint64_t *module, void const* idk1, void * idk2, unsigned long idk3, int idk4) LINKABLE;

uint64_t _ZN2nn4util14DecompressZlibEPvmPKvmS1_m_intercept(void * idk1, unsigned long idk2, void const* idk3, unsigned long idk4, void * idk5, unsigned long idk6) {
	uint64_t ret = _ZN2nn4util14DecompressZlibEPvmPKvmS1_m(idk1, idk2, idk3, idk4, idk5, idk6);
	SaltySD_printf("SaltySD Plugin: DecompressZlib(%llx, %llx, %llx, %llx, %llx, %llx) -> %llx\n", idk1, idk2, idk3, idk4, idk5, idk6, ret);
	return ret;
}

uint64_t _ZN2nn2ro10LoadModuleEPNS0_6ModuleEPKvPvmi_intercept(uint64_t *module, void const* idk1, void * idk2, unsigned long idk3, int idk4) {
	uint64_t ret = _ZN2nn2ro10LoadModuleEPNS0_6ModuleEPKvPvmi(module, idk1, idk2, idk3, idk4);
	SaltySD_printf("SaltySD Plugin: nn::ro::LoadModule(%llx, %llx, %llx, %llx, %llx) -> %llx\n", module, idk1, idk2, idk3, idk4, ret);
	return ret;
}

uint64_t ReadFile_intercept(uint64_t idk1, uint64_t idk2, uint64_t idk3, uint64_t idk4, uint64_t idk5)
{
    uint64_t ret = _ZN2nn2fs8ReadFileEPmNS0_10FileHandleElPvm(idk1, idk2, idk3, idk4, idk5);
    SaltySD_printf("SaltySD Plugin: ReadFile(%llx, %llx, %llx, %llx, %llx) -> %llx\n", idk1, idk2, idk3, idk4, idk5, ret);
    return ret;
}

uint64_t ReadFile_intercept2(uint64_t handle, uint64_t offset, uint64_t out, uint64_t size)
{
    uint64_t ret = _ZN2nn2fs8ReadFileENS0_10FileHandleElPvm(handle, offset, out, size);
    SaltySD_printf("SaltySD Plugin: ReadFile2(%llx, %llx, %llx, %llx) -> %llx\n", handle, offset, out, size, ret);
    return ret;
}

float _ZN3app11peachdaikon32PEACH_PEACHDAIKON_DAIKON_1_POWEREv_replace() {
	float (*getStitchDamage)() = (float (*)(void)) SaltySDCore_FindSymbol("_ZN3app11peachdaikon32PEACH_PEACHDAIKON_DAIKON_8_POWEREv");
	float stitchDamage = getStitchDamage();
	return stitchDamage*10;
}

uint64_t attack_code_replace(uint64_t*** arg0, uint64_t arg1) {
	/*
	arg0 = *(_QWORD *)(v5 + 160)
	arg1 = v7
	func = (**(_QWORD **)(v5 + 160) + 104LL)
	(*func)(arg0, arg1);
	*/
	uint64_t (*given_func)(uint64_t, uint64_t) = (uint64_t (*)(uint64_t, uint64_t))((*(*(arg0+160)) + 104));
	return given_func(*(arg0+160), arg1);
}

void _ZN3app10sv_animcmd6ATTACKEP9lua_State_replace(__int64_t a1) {
  // Get necessary functions
  // lib::L2CAgent::L2CAgent(L2CAgent*, lua_State *)
  __int64_t (*lib_L2CAgent)(__int64_t*, __int64_t) = 
	(__int64_t (*)(__int64_t*, __int64_t))(SaltySDCore_FindSymbol("_ZN3lib8L2CAgentC2EP9lua_State"));
	
  // lib::L2CAgent::~L2CAgent(L2CAgent*)
  __int64_t (*lib_L2CAgent_del)(__int64_t*) = 
	(__int64_t (*)(__int64_t*))(SaltySDCore_FindSymbol("_ZN3lib8L2CAgentD2Ev"));

  // L2CAgent *__fastcall lib::L2CAgent::push_lua_stack(L2CAgent *result, const lib::L2CValue *a2)
  __int64_t (*lib_L2CAgent_push_lua_stack)(__int64_t, const __int64_t*) = 
    (__int64_t (*)(__int64_t, const __int64_t*))(SaltySDCore_FindSymbol("_ZN3lib8L2CAgent14push_lua_stackERKNS_8L2CValueE"));
	
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
  __int64_t (*lib_L2CAgent_pop_lua_stack)(__int64_t, int) = 
    (__int64_t (*)(__int64_t, int))(SaltySDCore_FindSymbol("_ZN3lib8L2CAgent13pop_lua_stackEi"));
	
  // L2CAgent *__fastcall lib::L2CAgent::clear_lua_stack(L2CAgent *result)
  __int64_t (*lib_L2CAgent_clear_lua_stack)(__int64_t) = 
    (__int64_t (*)(__int64_t))(SaltySDCore_FindSymbol("_ZN3lib8L2CAgent15clear_lua_stackEv"));
	
  // lib::L2CValue::L2CValue(lib::L2CValue *__hidden this, int)
  __int64_t (*lib_L2CValue_L2CValue_int)(__int64_t, __int64_t) = 
    (__int64_t (*)(__int64_t, __int64_t))(SaltySDCore_FindSymbol("_ZN3lib8L2CValueC1Ei"));
	
  // lib::L2CValue::L2CValue(lib::L2CValue *__hidden this, float)
  __int64_t (*lib_L2CValue_L2CValue_float)(__int64_t, float) = 
    (__int64_t (*)(__int64_t, float))(SaltySDCore_FindSymbol("_ZN3lib8L2CValueC1Ef"));
	
  // lib::L2CValue::L2CValue(lib::L2CValue *__hidden this, phx::Hash40)
  __int64_t (*_ZN3lib8L2CValueC1EN3phx6Hash40E)(__int64_t, __int64_t) = 
    (__int64_t (*)(__int64_t, __int64_t))(SaltySDCore_FindSymbol("_ZN3lib8L2CValueC1EN3phx6Hash40E"));
	
  // lib::L2CValue::L2CValue(lib::L2CValue *__hidden this, bool)
  __int64_t (*lib_L2CValue_L2CValue_bool)(__int64_t, bool) = 
	(__int64_t (*)(__int64_t, bool))(SaltySDCore_FindSymbol("_ZN3lib8L2CValueC1Eb"));
	
  // lib::L2CValue::~L2CValue(lib::L2CValue *__hidden this)
  __int64_t (*lib_L2CValue_L2CValue_del)(__int64_t) = 
	(__int64_t (*)(__int64_t))(SaltySDCore_FindSymbol("_ZN3lib8L2CValueD1Ev"));
	
  // app::sv::animcmd::FLASH(lua_State* a1)
  __int64_t (*app_sv_animcmd_FLASH)(__int64_t) = 
	(__int64_t (*)(__int64_t))(SaltySDCore_FindSymbol("_ZN3app10sv_animcmd5FLASHEP9lua_State"));
	
	// app::sv::animcmd::EFFECT(lua_State* a1)
  __int64_t (*app_sv_animcmd_EFFECT)(__int64_t) = 
	(__int64_t (*)(__int64_t))(SaltySDCore_FindSymbol("_ZN3app10sv_animcmd6EFFECTEP9lua_State"));
	
  __int64_t v1; // x19
  uint64_t v2; // x9
  uint64_t i; // x8
  
  // Instantiate our own L2CAgent with the given lua_State
  struct L2CAgent l2c_agent;
  lib_L2CAgent(&l2c_agent, a1);
  
  lua_State* l2c_state = (lua_State*) a1;
  
  int num_elems = lua_gettop(l2c_state);
  /*
  struct L2CValue hitboxParams[36];
  for (int i = 35; i >= 0; i--) {
	lib_L2CAgent_pop_lua_stack(&l2c_agent, 1, &hitboxParams[i]);
  }
  
  for (int i = 0; i < 36; i++) {
	if (i == 3) {
		struct L2CValue inject_dmg = {.raw_float = (float)20, .type = L2C_number};
		lib_L2CAgent_push_lua_stack(&l2c_agent, &inject_dmg);
	}
    else if (i == 4) {
		struct L2CValue inject_angle = {.raw = (int)90, .type = L2C_integer};
		lib_L2CAgent_push_lua_stack(&l2c_agent, &inject_angle); 
    }
	else
		lib_L2CAgent_push_lua_stack(&l2c_agent, &hitboxParams[i]);
  }*/
  
  // Replacing a value on the stack:
  // Works! :D
  /*
  struct L2CValue inject_dmg = {.raw_float = (float)20, .type = L2C_number};
  lib_L2CAgent_push_lua_stack(&l2c_agent, &inject_dmg);
  lua_replace((lua_State*) l2c_agent.lua_state_agent, 4); 
  */
  
  // Getting and replacing a value on the stack?
  struct L2CValue damage;
  asm("mov x8, %x0" : : "r"(&damage) : "x8" );
  lib_L2CAgent_pop_lua_stack(&l2c_agent, 4);
  struct L2CValue inject_dmg = {.raw_float = ((float)(damage.raw_float) + 10), .type = L2C_number};
  lib_L2CAgent_push_lua_stack(&l2c_agent, &inject_dmg);
  lua_replace((lua_State*) l2c_agent.lua_state_agent, 4); 
  
  // PLAN: 
  // - Iterate through lua_State stack, store all values (in an array?)
  // - Pop all values off of the stack
  // - Push our own values on the stack 
  // - Call simple animcmd function with the stack
  // - Pop all values off the stack
  // - Push the stored values back on the stack in the proper order

  v1 = a1;
  u64 attack_code_addr = SaltySDCore_FindSymbol("_ZN3app10sv_animcmd6ATTACKEP9lua_State");
  void (*sub_71019420D0)(__int64_t, __int64_t) = (void (*)(__int64_t, __int64_t))(attack_code_addr + 96);
  sub_71019420D0(*(__int64_t *)(*(__int64_t *)(a1 - 8) + 416LL), a1);
  
  // Test:
  
  struct Hash40 effectHash1 = {.hash = 0x1446E1363ALL};
  struct Hash40 effectHash2 = {.hash = 0x31ED91FCALL};
  struct L2CValue v33 = {.raw = effectHash1.hash, .type = L2C_hash};
  struct L2CValue v31 = {.raw = effectHash2.hash, .type = L2C_hash};
  struct L2CValue v28 = {.raw = (int)0, .type = L2C_integer};
  struct L2CValue v27 = {.raw_float = (float)8.5, .type = L2C_number};
  struct L2CValue v26 = {.raw = (int)-13, .type = L2C_integer};
  struct L2CValue v25 = {.raw =  (int)0, .type = L2C_integer};
  struct L2CValue v24 = {.raw =  (int)0, .type = L2C_integer};
  struct L2CValue v23 = {.raw =  (int)0, .type = L2C_integer};
  struct L2CValue v22 = {.raw =  (int)1, .type = L2C_integer};
  struct L2CValue v21 = {.raw =  (int)0, .type = L2C_integer};
  struct L2CValue v20 = {.raw =  (int)0, .type = L2C_integer};
  struct L2CValue v19 = {.raw =  (int)0, .type = L2C_integer};
  struct L2CValue v18 = {.raw =  (int)0, .type = L2C_integer};
  struct L2CValue v17 = {.raw =  (int)0, .type = L2C_integer};
  struct L2CValue v16 = {.raw =  (int)0, .type = L2C_integer};
  struct L2CValue v15 = {.raw = (bool) 1, .type = L2C_bool};
  lib_L2CAgent_clear_lua_stack(&l2c_agent);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v33);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v31);
  
  // Test popping then pushing two values.
  struct L2CValue v31Popped;
  asm("mov x8, %x0" : : "r"(&v31Popped) : "x8" );
  lib_L2CAgent_pop_lua_stack(&l2c_agent, 2);
  struct L2CValue v33Popped;
  asm("mov x8, %x0" : : "r"(&v33Popped) : "x8" );
  lib_L2CAgent_pop_lua_stack(&l2c_agent, 1);
  
  //lib_L2CAgent_clear_lua_stack(&l2c_agent);
  //lib_L2CAgent_push_lua_stack(&l2c_agent, &v33Popped);
  //lib_L2CAgent_push_lua_stack(&l2c_agent, &v31Popped);
  
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v28);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v27);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v26);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v25);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v24);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v23);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v22);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v21);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v20);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v19);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v18);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v17);  
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v16);
  lib_L2CAgent_push_lua_stack(&l2c_agent, &v15);
  
  app_sv_animcmd_EFFECT(l2c_agent.lua_state_agent);
  
  // Push an extra to test popping... Works!
  /*struct L2CValue vToPop = {.raw = (bool) 1, .type = L2C_bool};
  lib_L2CAgent_push_lua_stack(&l2c_agent, &vToPop);
  struct L2CValue vPopped;
  asm("mov x8, %x0" : : "r"(&vPopped) : "x8" );
  lib_L2CAgent_pop_lua_stack(&l2c_agent, 1, &vPopped);
  */
  
  // clear_lua_stack section
  v2 = *(__int64_t *)(v1 + 16);
  for ( i = **(__int64_t **)(v1 + 32) + 16LL; v2 < i; v2 = *(__int64_t *)(v1 + 16) )
  {
    *(__int64_t *)(v1 + 16) = v2 + 16;
    *(__int32_t *)(v2 + 8) = 0;
  }
  *(__int64_t *)(v1 + 16) = i;
  
}

float _ZN3app11peachdaikon32PEACH_PEACHDAIKON_DAIKON_1_POWEREv_replace_with_arg(float arg1) {
	return arg1;
}

int SaltySD_function_replace_sym(const char* function_sym, u64 new_func) {
	u64 addr = SaltySDCore_FindSymbol(function_sym);
	return SaltySD_function_replace(addr, new_func);
}

int SaltySD_function_replace(u64 addr, u64 new_func) {
	if (addr) {
		SaltySD_Memcpy(addr, "\x49\x00\x00\x58", 4); // LDR X9, .+8
		SaltySD_Memcpy(addr+4, "\x20\x01\x1F\xD6", 4); // BR X9
		SaltySD_Memcpy(addr+8, &new_func, 8); // .dword newaddr
		
		SaltySD_printf("SaltySD Plugin: forcing function at %llx to jump to %11x\n", addr, new_func);
		
		return 0;
	}
	
	return -1;
}

int main(int argc, char *argv[])
{
    SaltySD_printf("SaltySD Plugin: alive\n");
    
    char* ver = "Ver. %d.%d.%d";
    u64 dst_3 = SaltySDCore_findCode(ver, strlen(ver));
    if (dst_3)
    {
        SaltySD_Memcpy(dst_3, "noice v%d%d%d", 13);
    }
	
	u64 code_start = SaltySDCore_getCodeStart();
	u64 code_size = SaltySDCore_getCodeSize();
	
	SaltySD_printf("SaltySD Plugin: code start: %llx, code size: %11x\n", code_start, code_size);
	
	char* dmgCode = "\xB9\xD8\xA9\x41\x94\xE0\x2B";
    u64 dst_dmg = SaltySDCore_findCode(dmgCode, strlen(dmgCode));
    if (dst_dmg)
    {
		dst_dmg = dst_dmg + 1;
        SaltySD_printf("SaltySD Plugin: found attack damage code at: %llx\n", dst_dmg);
		//SaltySD_Memcpy(dst_dmg, "\x40\x0B\x80\x52", 4); // MOV W0, #90
    }
	
	//SaltySD_function_replace_sym("_ZN3app11peachdaikon32PEACH_PEACHDAIKON_DAIKON_1_POWEREv", &_ZN3app11peachdaikon32PEACH_PEACHDAIKON_DAIKON_1_POWEREv_replace);
	u64 addr = SaltySDCore_FindSymbol("_ZN3app11peachdaikon32PEACH_PEACHDAIKON_DAIKON_1_POWEREv");
	if (addr) {
		SaltySD_Memcpy(addr, "\x00\x90\x26\x1e", 4); // FMOV S0, #20.0
		SaltySD_Memcpy(addr+4, "\x49\x00\x00\x58", 4); // LDR X9, .+8
		SaltySD_Memcpy(addr+8, "\x20\x01\x1F\xD6", 4); // BR X9
		u64 new_addr = &_ZN3app11peachdaikon32PEACH_PEACHDAIKON_DAIKON_1_POWEREv_replace_with_arg;
		SaltySD_Memcpy(addr+12, &new_addr, 8); // .dword newaddr
		
		SaltySD_printf("SaltySD Plugin: forcing function at %llx to jump to %llx\n", addr, new_addr);
	}
	
	SaltySD_function_replace_sym("_ZN3app10sv_animcmd6ATTACKEP9lua_State", &_ZN3app10sv_animcmd6ATTACKEP9lua_State_replace);
	
							
    SaltySDCore_ReplaceImport("_ZN2nn2fs8ReadFileEPmNS0_10FileHandleElPvm", ReadFile_intercept);
    SaltySDCore_ReplaceImport("_ZN2nn2fs8ReadFileENS0_10FileHandleElPvm", ReadFile_intercept2);
	SaltySDCore_ReplaceImport("_ZN2nn4util14DecompressZlibEPvmPKvmS1_m", _ZN2nn4util14DecompressZlibEPvmPKvmS1_m_intercept);
	SaltySDCore_ReplaceImport("_ZN2nn2ro10LoadModuleEPNS0_6ModuleEPKvPvmi",  _ZN2nn2ro10LoadModuleEPNS0_6ModuleEPKvPvmi_intercept);
	
    __libnx_exit(0);
}

