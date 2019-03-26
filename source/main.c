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

// lib::L2CAgent::L2CAgent(L2CAgent*, lua_State *)
__int64_t (*lib_L2CAgent)(__int64_t*, __int64_t);

// L2CAgent *__fastcall lib::L2CAgent::push_lua_stack(L2CAgent *result, const lib::L2CValue *a2)
__int64_t (*lib_L2CAgent_push_lua_stack)(__int64_t, const __int64_t*);

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
__int64_t (*lib_L2CAgent_pop_lua_stack)(__int64_t, int);

// L2CAgent *__fastcall lib::L2CAgent::clear_lua_stack(L2CAgent *result)
__int64_t (*lib_L2CAgent_clear_lua_stack)(__int64_t);

// app::sv::animcmd::EFFECT(lua_State* a1)
__int64_t (*app_sv_animcmd_EFFECT)(__int64_t);

// app::sv::animcmd::EFFECT_FOLLOW_FLIP_COLOR(lua_State* a1)
__int64_t (*app_sv_animcmd_EFFECT_FOLLOW_FLIP_COLOR)(__int64_t);

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

void get_lua_stack(__int64_t* l2c_agent, int index, __int64_t* l2c_val) {
	 __int64_t (*lib_L2CAgent_pop_lua_stack)(__int64_t, int) = 
    (__int64_t (*)(__int64_t, int))(SaltySDCore_FindSymbol("_ZN3lib8L2CAgent13pop_lua_stackEi"));
	
	asm("mov x8, %x0" : : "r"(l2c_val) : "x8" );
    lib_L2CAgent_pop_lua_stack(l2c_agent, index);
}

void _ZN3app10sv_animcmd6ATTACKEP9lua_State_replace(__int64_t a1) {
  // Stretched bones fix: Scale down by ModelModule::scale() with lua_State arg of bone?
	
  __int64_t v1; // x19
  uint64_t v2; // x9
  uint64_t i; // x8
  
  // Instantiate our own L2CAgent with the given lua_State
  L2CAgent l2c_agent;
  lib_L2CAgent(&l2c_agent, a1);
  
  // Getting and replacing a value on the stack. Works!
  /*
  L2CValue damage;
  get_lua_stack(&l2c_agent, 4, &damage);
  L2CValue inject_dmg = {.raw_float = ((float)(damage.raw_float) + 10.0), .type = L2C_number};
  lib_L2CAgent_push_lua_stack(&l2c_agent, &inject_dmg);
  lua_replace((lua_State*) l2c_agent.lua_state_agent, 4); 
  */
  
  
  // Get all necessary hitbox params
  L2CValue bone;
  get_lua_stack(&l2c_agent, 3, &bone);
  L2CValue damage;
  get_lua_stack(&l2c_agent, 4, &damage);
  L2CValue angle;
  get_lua_stack(&l2c_agent, 5, &angle);
  L2CValue kbg;
  get_lua_stack(&l2c_agent, 6, &kbg);
  L2CValue wkb;
  get_lua_stack(&l2c_agent, 7, &wkb);
  L2CValue bkb;
  get_lua_stack(&l2c_agent, 8, &bkb);
  L2CValue size;
  get_lua_stack(&l2c_agent, 9, &size);
  L2CValue x;
  get_lua_stack(&l2c_agent, 10, &x);
  L2CValue y;
  get_lua_stack(&l2c_agent, 11, &y);
  L2CValue z;
  get_lua_stack(&l2c_agent, 12, &z);
  L2CValue x2;
  get_lua_stack(&l2c_agent, 13, &x2);
  L2CValue y2;
  get_lua_stack(&l2c_agent, 14, &y2);
  L2CValue z2;
  get_lua_stack(&l2c_agent, 15, &z2);

  v1 = a1;
  u64 attack_code_addr = SaltySDCore_FindSymbol("_ZN3app10sv_animcmd6ATTACKEP9lua_State");
  void (*sub_71019420D0)(__int64_t, __int64_t) = (void (*)(__int64_t, __int64_t))(attack_code_addr + 96);
  sub_71019420D0(*(__int64_t *)(*(__int64_t *)(a1 - 8) + 416LL), a1);
  
  // EFFECT_FOLLOW_COLOR(Graphic, Bone, Z, Y, X, ZRot, YRot, XRot, Size, unknown=0x1, Red, Green, Blue)
  // FIRST, to test, let's assume single hitbox, not extended, so ignore x2,y2,z2.
  // EFFECT_FOLLOW_FLIP_COLOR(GFXLeft,GFXRight,Bone, Z, Y, X, ZRot, YRot, XRot, Size,Terminate,unknown,R,G,B)
  float sizeMult = 19.0 / 200.0;
  Hash40 shieldEffectHash = {.hash = 0xAFAE75F05LL};
  
  L2CValue shieldEffect = {.raw = shieldEffectHash.hash, .type = L2C_hash};
  L2CValue xRot = {.raw_float = (float) 0.0, .type = L2C_number};
  L2CValue yRot = {.raw_float = (float) 0.0, .type = L2C_number};
  L2CValue zRot = {.raw_float = (float) 0.0, .type = L2C_number};
  L2CValue unkParam = {.raw = (int) 1, .type = L2C_integer};
  L2CValue unkParam2 = {.raw = (float) 35.0f, .type = L2C_number};
  L2CValue effectSize = {.raw_float = (float) size.raw_float * sizeMult, .type = L2C_number};
  L2CValue red = {.raw_float = (float) 255.0, .type = L2C_number};
  L2CValue green = {.raw_float = (float) 0.0, .type = L2C_number};
  L2CValue blue = {.raw_float = (float) 0.0, .type = L2C_number};
	
  int num_effects;
  if (x2.type != L2C_void && y2.type != L2C_void && z2.type != L2C_void) {
	  num_effects = 4;
  } else {
	x2 = x;
	y2 = y; 
	z2 = z;
	num_effects = 1;
  }
  
  for (int i = 0; i < num_effects; i++) {
	L2CValue currX = {.raw_float = (float) x.raw_float + ((x2.raw_float - x.raw_float) /  3 * i), .type = L2C_number};
	L2CValue currY = {.raw_float = (float) y.raw_float + ((y2.raw_float - y.raw_float) /  3 * i), .type = L2C_number};
	L2CValue currZ = {.raw_float = (float) z.raw_float + ((z2.raw_float - z.raw_float) /  3 * i), .type = L2C_number};
	  
	lib_L2CAgent_clear_lua_stack(&l2c_agent);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &shieldEffect);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &shieldEffect);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &bone);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &currX);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &currY);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &currZ);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &xRot);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &yRot);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &zRot);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &effectSize);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &unkParam);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &unkParam);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &red);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &green);
	lib_L2CAgent_push_lua_stack(&l2c_agent, &blue);
	app_sv_animcmd_EFFECT_FOLLOW_FLIP_COLOR(l2c_agent.lua_state_agent);
  }
  
  // clear_lua_stack section
  v2 = *(__int64_t *)(v1 + 16);
  for ( i = **(__int64_t **)(v1 + 32) + 16LL; v2 < i; v2 = *(__int64_t *)(v1 + 16) )
  {
    *(__int64_t *)(v1 + 16) = v2 + 16;
    *(__int32_t *)(v2 + 8) = 0;
  }
  *(__int64_t *)(v1 + 16) = i;
  
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

int SaltySD_function_replace_sym(char* function_sym, u64 new_func) {
	u64 addr = SaltySDCore_FindSymbol(function_sym);
	return SaltySD_function_replace(addr, new_func);
}

int main(int argc, char *argv[])
{
    SaltySD_printf("SaltySD Plugin: alive\n");
	
	// get necessary functions
	lib_L2CAgent = (__int64_t (*)(__int64_t*, __int64_t))(SaltySDCore_FindSymbol("_ZN3lib8L2CAgentC2EP9lua_State"));
	lib_L2CAgent_push_lua_stack = (__int64_t (*)(__int64_t, const __int64_t*))(SaltySDCore_FindSymbol("_ZN3lib8L2CAgent14push_lua_stackERKNS_8L2CValueE"));
	lib_L2CAgent_pop_lua_stack = (__int64_t (*)(__int64_t, int))(SaltySDCore_FindSymbol("_ZN3lib8L2CAgent13pop_lua_stackEi"));
    lib_L2CAgent_clear_lua_stack = (__int64_t (*)(__int64_t))(SaltySDCore_FindSymbol("_ZN3lib8L2CAgent15clear_lua_stackEv"));
	app_sv_animcmd_EFFECT = (__int64_t (*)(__int64_t))(SaltySDCore_FindSymbol("_ZN3app10sv_animcmd6EFFECTEP9lua_State"));
	app_sv_animcmd_EFFECT_FOLLOW_FLIP_COLOR = (__int64_t (*)(__int64_t))(SaltySDCore_FindSymbol("_ZN3app10sv_animcmd24EFFECT_FOLLOW_FLIP_COLOREP9lua_State"));
	
    
    char* ver = "Ver. %d.%d.%d";
    u64 dst_3 = SaltySDCore_findCode(ver, strlen(ver));
    if (dst_3) {
        SaltySD_Memcpy(dst_3, "noice v%d%d%d", 13);
    }
	
	SaltySD_function_replace_sym("_ZN3app10sv_animcmd6ATTACKEP9lua_State", &_ZN3app10sv_animcmd6ATTACKEP9lua_State_replace);
							
    SaltySDCore_ReplaceImport("_ZN2nn2fs8ReadFileEPmNS0_10FileHandleElPvm", ReadFile_intercept);
    SaltySDCore_ReplaceImport("_ZN2nn2fs8ReadFileENS0_10FileHandleElPvm", ReadFile_intercept2);
	SaltySDCore_ReplaceImport("_ZN2nn4util14DecompressZlibEPvmPKvmS1_m", _ZN2nn4util14DecompressZlibEPvmPKvmS1_m_intercept);
	SaltySDCore_ReplaceImport("_ZN2nn2ro10LoadModuleEPNS0_6ModuleEPKvPvmi",  _ZN2nn2ro10LoadModuleEPNS0_6ModuleEPKvPvmi_intercept);
	
    __libnx_exit(0);
}

