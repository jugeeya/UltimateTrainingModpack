#ifndef SALTYSD_DYNAMIC_H
#define SALTYSD_DYNAMIC_H

#include <stdint.h>

#include "../useful/useful.h"

extern "C" {
	uint64_t SaltySDCore_GetSymbolAddr(void* base, char* name) LINKABLE;
	uint64_t SaltySDCore_FindSymbol(char* name) LINKABLE;
	uint64_t SaltySDCore_FindSymbolBuiltin(char* name) LINKABLE;
	void SaltySDCore_RegisterModule(void* base) LINKABLE;
	void SaltySDCore_RegisterBuiltinModule(void* base) LINKABLE;
	void SaltySDCore_DynamicLinkModule(void* base) LINKABLE;
	void SaltySDCore_ReplaceModuleImport(void* base, char* name, void* new_replace) LINKABLE;
	void SaltySDCore_ReplaceImport(char* name, void* new_replace) LINKABLE;
}

#endif // SALTYSD_DYNAMIC_H
