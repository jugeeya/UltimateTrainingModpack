#ifndef SALTYSD_IPC_H
#define SALTYSD_IPC_H

#include <switch.h>

#include "useful.h"

extern void SaltySD_Init() LINKABLE;
extern Result SaltySD_Deinit() LINKABLE;
extern Result SaltySD_Term() LINKABLE;
extern Result SaltySD_Restore() LINKABLE;
extern Result SaltySD_LoadELF(u64 heap, u64* elf_addr, u64* elf_size, char* name) LINKABLE;
extern Result SaltySD_Memcpy(u64 to, u64 from, u64 size) LINKABLE;
extern Result SaltySD_GetSDCard(Handle *retrieve) LINKABLE;
extern Result SaltySD_printf(const char* format, ...) LINKABLE;

#endif //SALTYSD_IPC_H
