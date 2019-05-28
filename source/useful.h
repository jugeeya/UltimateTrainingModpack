#ifndef USEFUL_H
#define USEFUL_H

#include <switch.h>
#include <string.h>
#include <stdio.h>

#include "l2c.hpp"

#define LINKABLE __attribute__ ((weak))

#define debug_log(...) \
	{char log_buf[0x200]; snprintf(log_buf, 0x200, __VA_ARGS__); \
	svcOutputDebugString(log_buf, strlen(log_buf));}

/**
 * Rounds a number to the nearest multiple of another number.
 */
float round_to(float val, float to);

/**
 * Linearly interpolates between two numbers, without bounds checking.
 */
float lerp(float min, float max, float t);
float unlerp(float min, float max, float val);
/**
 * Linearly interpolates between two numbers, with bounds checking.
 */
float lerpBounded(float min, float max, float t);
float unlerpBounded(float min, float max, float val);

/**
 * Linearly nterpolates between two colors, with bounds checking, accounting for gamma.
 */
Vector3f colorLerp(Vector3f minColor, Vector3f maxColor, float t, float gamma = 2.2f);
	
#endif // USEFUL_H
