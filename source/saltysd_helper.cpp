#include <switch.h>

#include "saltysd_core.h"
#include "saltysd_ipc.h"
#include "saltysd_dynamic.h"
#include "nn_ro.h"

void (*SaltySD_installed_hook)(char*, u64) = NULL;

int SaltySD_function_replace(u64 addr, u64 new_func) {
	if (addr) {
		SaltySD_Memcpy(addr, (u64) "\x49\x00\x00\x58", 4); // LDR X9, .+8
		SaltySD_Memcpy(addr+4, (u64) "\x20\x01\x1F\xD6", 4); // BR X9
		SaltySD_Memcpy(addr+8, (u64) &new_func, 8); // .dword newaddr

		return 0;
	}
	return -1;
}

int SaltySD_function_replace_sym(char* function_sym, u64 new_func) {
	u64 addr = SaltySDCore_FindSymbol(function_sym);
	return SaltySD_function_replace(addr, new_func);
}

void LoadModule(SmashModule *module, void *param_2, void *param_3, unsigned long param_4, int param_5) {
	nn_ro_LoadModule(module, param_2, param_3, param_4, param_5);
	if (SaltySD_installed_hook != NULL) {
		SaltySD_installed_hook((char*)&module->name, (u64)module->module.module->module_base);
	}        
}

void SaltySD_install_nro_hook(u64 LoadModule_thunk_addr, void hook_main(char*, u64)) {
	SaltySD_installed_hook = hook_main;
	SaltySD_function_replace(LoadModule_thunk_addr, (u64) LoadModule);
}
