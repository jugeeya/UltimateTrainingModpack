#ifndef USEFUL_VISUAL_H
#define USEFUL_VISUAL_H

#include <math.h>
#include "useful.h"

/**
 * Rounds a number to the nearest multiple of another number.
 */
float round_to(float val, float align) { return roundf(val / align) * align; }

/**
 * Linearly interpolates between two numbers, without bounds checking.
 */
float lerp(float min, float max, float t) { return min + (max - min) * t; }

float unlerp(float min, float max, float val) {
    return (val - min) / (max - min);
}

/**
 * Linearly interpolates between two numbers, with bounds checking.
 */
float lerp_bounded(float min, float max, float t) {
    return t <= 0 ? min : t >= 1 ? max : lerp(min, max, t);
}

float unlerp_bounded(float min, float max, float val) {
    return val <= min ? 0 : val >= max ? 1 : unlerp(min, max, val);
}

/**
 * Linearly nterpolates between two colors, with bounds checking, accounting for
 * gamma. arguments:
 * - min_color (Vector3f) -- xyz maps to rgb, components are usually in the
 * range [0.0f, 1.0f] but can go beyond to account for super-bright or
 * super-dark colors
 * - max_Color (Vector3f) -- same as minColor
 * - t (float) -- how far to interpolate between the colors
 * - gamma (float = 2.0f) -- used for color correction, helps avoid ugly dark
 * colors when interpolating b/t bright colors
 */

Vector3f color_lerp(Vector3f min_color, Vector3f max_color, float t,
                    float gamma = 2.0f) {
    float gamma_inv = 1.0f / gamma;
    float align =
        1.0f / 255.0f;  // color components must be a multiple of 1/255
    return {round_to(powf(lerp_bounded(powf(min_color.x, gamma),
                                       powf(max_color.x, gamma), t),
                          gamma_inv),
                     align),
            round_to(powf(lerp_bounded(powf(min_color.y, gamma),
                                       powf(max_color.y, gamma), t),
                          gamma_inv),
                     align),
            round_to(powf(lerp_bounded(powf(min_color.z, gamma),
                                       powf(max_color.z, gamma), t),
                          gamma_inv),
                     align)};
}

#endif  // USEFUL_VISUAL_H
