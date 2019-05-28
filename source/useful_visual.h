#ifndef USEFUL_VISUAL_H
#define USEFUL_VISUAL_H

#include "useful.h"
/**
 * Rounds a number to the nearest multiple of another number.
 */
float round_to(float val, float align);

/**
 * Linearly interpolates between two numbers, without bounds checking.
 */
float lerp(float min, float max, float t);
float unlerp(float min, float max, float val);
/**
 * Linearly interpolates between two numbers, with bounds checking.
 */
float lerp_bounded(float min, float max, float t);
float unlerp_bounded(float min, float max, float val);

/**
 * Linearly nterpolates between two colors, with bounds checking, accounting for gamma.
 * arguments:
 * - min_color (Vector3f) -- xyz maps to rgb, components are usually in the range [0.0f, 1.0f] but can go beyond to account for super-bright or super-dark colors
 * - max_Color (Vector3f) -- same as minColor
 * - t (float) -- how far to interpolate between the colors
 * - gamma (float = 2.0f) -- used for color correction, helps avoid ugly dark colors when interpolating b/t bright colors
 */
Vector3f color_lerp(Vector3f min_color, Vector3f max_color, float t, float gamma = 2.0f);
	
#endif // USEFUL_VISUAL_H
