#ifndef SALTYSD_IPC_H
#define SALTYSD_IPC_H

#include <switch.h>

#include "../useful/useful.h"

extern "C" {
	void SaltySD_Init() LINKABLE;
	Result SaltySD_Deinit() LINKABLE;
	Result SaltySD_Term() LINKABLE;
	Result SaltySD_Restore() LINKABLE;
	Result SaltySD_LoadELF(u64 heap, u64* elf_addr, u64* elf_size, char* name) LINKABLE;
	Result SaltySD_Memcpy(u64 to, u64 from, u64 size) LINKABLE;
	Result SaltySD_GetSDCard(Handle *retrieve) LINKABLE;
	Result SaltySD_printf(const char* format, ...) LINKABLE;
}

#endif //SALTYSD_IPC_H
