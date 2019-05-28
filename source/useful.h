#ifndef USEFUL_H
#define USEFUL_H

#include <switch.h>
#include <string.h>
#include <stdio.h>

#define LINKABLE __attribute__ ((weak))

#define LOAD64 *(u64 *)

#define debug_log(...) {\
	char log_buf[0x200]; snprintf(log_buf, 0x200, __VA_ARGS__); \
	svcOutputDebugString(log_buf, strlen(log_buf)); }

typedef struct Hash40 {
	uint64_t hash : 40;
} Hash40;

typedef struct Vector3f {
	float x;
	float y;
	float z;
} Vector3f;
	
#endif // USEFUL_H
