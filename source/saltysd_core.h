#ifndef SALTYSD_CORE_H
#define SALTYSD_CORE_H

#include <switch.h>

#include "useful.h"

extern u64 SaltySDCore_getCodeStart() LINKABLE;
extern u64 SaltySDCore_getCodeSize() LINKABLE;
extern u64 SaltySDCore_findCode(u8* code, size_t size) LINKABLE;

#endif // SALTYSD_CORE_H
