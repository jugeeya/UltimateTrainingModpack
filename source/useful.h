#ifndef USEFUL_H
#define USEFUL_H

#include <switch.h>
#include <string.h>
#include <stdio.h>

#define LINKABLE __attribute__ ((weak))

#define debug_log(...) \
	{char log_buf[0x200]; snprintf(log_buf, 0x200, __VA_ARGS__); \
	svcOutputDebugString(log_buf, strlen(log_buf));}
	
#endif // USEFUL_H
