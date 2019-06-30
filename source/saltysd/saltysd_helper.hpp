#ifndef SALTYSD_HELPER_H
#define SALTYSD_HELPER_H

#include <switch.h>

#include "saltysd_core.h"
#include "saltysd_ipc.h"
#include "saltysd_dynamic.h"

#define ANCHOR_REL 0x70ffffc000
u64 ANCHOR_ABS;
#define IMPORT(x) (x - ANCHOR_REL + ANCHOR_ABS)

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

#endif // SALTYSD_HELPER_H
