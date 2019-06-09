#ifndef SALTYSD_CORE_H
#define SALTYSD_CORE_H

#include <switch.h>

#include "../useful/useful.h"

extern "C" {
	u64 SaltySDCore_getCodeStart() LINKABLE;
	u64 SaltySDCore_getCodeSize() LINKABLE;
	u64 SaltySDCore_findCode(u8* code, size_t size) LINKABLE;
}

#endif // SALTYSD_CORE_H
