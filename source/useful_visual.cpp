#include "useful_visual.h"

#include <math.h>

#include "useful.h"

float round_to(float val, float align) {
	return roundf(val / align) * align;
}

float lerp(float min, float max, float t) {
	return min + (max - min) * t;
}

float unlerp(float min, float max, float val) {
	return (val - min) / (max - min);
}

float lerp_bounded(float min, float max, float t) {
	return t <= 0 ? min : t >= 1 ? max : lerp(min, max, t);
}

float unlerp_bounded(float min, float max, float val) {
	return val <= min ? 0 : val >= max ? 1 : unlerp(min, max, val);
}

Vector3f color_lerp(Vector3f min_color, Vector3f max_color, float t, float gamma) {
	float gamma_inv = 1.0f / gamma;
	float align = 1.0f / 255.0f; // color components must be a multiple of 1/255
	return {
		round_to(powf(lerp_bounded(powf(min_color.x, gamma), powf(max_color.x, gamma), t), gamma_inv), align),
		round_to(powf(lerp_bounded(powf(min_color.y, gamma), powf(max_color.y, gamma), t), gamma_inv), align),
		round_to(powf(lerp_bounded(powf(min_color.z, gamma), powf(max_color.z, gamma), t), gamma_inv), align)
	};
}
