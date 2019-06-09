#ifndef APP_SV_MATH
#define APP_SV_MATH

#include <switch.h>

namespace app::sv_math {
	int rand(u64, int) asm("_ZN3app7sv_math4randEN3phx6Hash40Ei") LINKABLE;
}

#endif // APP_SV_MATH