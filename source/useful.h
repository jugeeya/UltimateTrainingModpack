#ifndef USEFUL_H
#define USEFUL_H

#include <switch.h>
#include <string.h>
#include <stdio.h>

#define LOAD64(x) *(u64 *)(x)

#define LINKABLE __attribute__ ((weak))

#define debug_log(...) \
    {char log_buf[0x200]; snprintf(log_buf, 0x200, __VA_ARGS__); \
    svcOutputDebugString(log_buf, strlen(log_buf));}
    
#endif // USEFUL_H
