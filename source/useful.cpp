#include "useful.h"

#include <math.h>

#include "l2c.hpp"

float round_to(float val, float to) {
	return roundf(val / to) * to;
}

float lerp(float min, float max, float t) {
	return min + (max - min) * t;
}

float unlerp(float min, float max, float val) {
	return (val - min) / (max - min);
}

float lerpBounded(float min, float max, float t) {
	return t <= 0 ? min : t >= 1 ? max : lerp(min, max, t);
}

float unlerpBounded(float min, float max, float val) {
	return val <= min ? 0 : val >= max ? 1 : unlerp(min, max, val);
}

Vector3f colorLerp(Vector3f minColor, Vector3f maxColor, float t, float gamma) {
	float gammaInv = 1.0f / gamma;
	float roundTo = 1.0f / 255.0f; // color components must be a multiple of 1/255
	return {
		round_to(powf(lerpBounded(powf(minColor.x, gamma), powf(maxColor.x, gamma), t), gammaInv), roundTo),
		round_to(powf(lerpBounded(powf(minColor.y, gamma), powf(maxColor.y, gamma), t), gammaInv), roundTo),
		round_to(powf(lerpBounded(powf(minColor.z, gamma), powf(maxColor.z, gamma), t), gammaInv), roundTo)
	};
}
