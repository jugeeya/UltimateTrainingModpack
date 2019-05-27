#ifndef LUA_BIND_HASH_H
#define LUA_BIND_HASH_H

#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>

uint64_t lua_bind_hash(const void* data_, size_t len) {
	int64_t *data = (int64_t*)data_;
	size_t hash = len;
	int64_t hash_add = 0x27d4eb2f165667c4;
	int64_t* data_end = (int64_t *)((int64_t)data + len);

	uint64_t hash_vals[4] = {
		0x60ea27eeadc0b5d5,
		0xc2b2ae3d27d4eb4e,
		0xffffffffffffffff,
		0x61c8864e7a143578,
	};

	uint64_t hash_vals_mid[4];

	const uint64_t hash_vals_end[4] = {
		0x3c6ef3630bd7950e,
		0x1bbcd8c2f5e54380,
		0x779b185ebca87000,
		0xe6c617af2a1c0000
	};

	const uint64_t hash_vals_end_2[4] = {
		0x3f,
		0x39,
		0x34,
		0x2e
	};

	const uint64_t MULT1 = 0x87bcb65480000000; // -0x784349ab80000000
	const uint64_t MULT2 = 0xdef35b010f796ca9; // -0x210ca4fef0869357
	const uint64_t MULT3 = 0x9e3779b185ebca87; // -0x61c8864e7a143579
	const uint64_t MULT4 = 0xc2b2ae3d27d4eb4f; // -0x3d4d51c2d82b14b1
	const uint64_t MULT5 = 0x93ea75a780000000; // -0x6c158a5880000000
	const uint64_t MULT6 = 0x27d4eb2f165667c5;
	const uint64_t ADD1 = 0x85ebca77c2b2ae63;
	const uint64_t ADD2 = 0x165667b19e3779f9;
	
	if (len >= 32) {
		do {
			for (int i = 0; i < 4; i++) {
				hash_vals[i] += data[i] * MULT4;
				hash_vals_mid[i] = hash_vals[i] >> 0x21 | hash_vals[i] * 0x80000000;
				hash_vals[i] = hash_vals_mid[i] * MULT3;
			}
		  
		  data = data + 4;
		} while (data <= data_end + -4);
		uint64_t val = 0;
		for (int i = 0; i < 4; i++) {
			val += (hash_vals_mid[i] * hash_vals_end[i] | hash_vals[i] >> hash_vals_end_2[i]);
		}

		val = (val ^ (hash_vals_mid[0] * MULT1 | hash_vals_mid[0] * MULT2 >> 0x21) * MULT3) * MULT3;
		for (int i = 1; i < 4; i++) {
			val = (val + ADD1 ^ (hash_vals_mid[i] * MULT1 | hash_vals_mid[i] * MULT2 >> 0x21) * MULT3) * MULT3;
		}
		val += ADD1;

		hash_add = val;
	}

	hash += hash_add;
	for (; data + 1 <= data_end; data++) {
	  hash = (*data * MULT5 | (*data * MULT4) >> 0x21) * MULT3 ^ hash;
	  hash = (hash >> 0x25 | hash << 0x1b) * MULT3 + ADD1;
	}

	int32_t* data_32 = (int32_t*) data; 
	if ((int64_t *)(data_32 + 1) <= data_end) {
		hash = (uint64_t)*data_32 * MULT3 ^ hash;
		hash = (hash >> 0x29 | hash << 0x17) * MULT4 + ADD2;
		data = (int64_t *)(data_32 + 1);
	}

	int8_t* data_8 = (int8_t*) data;

	for (; data != data_end; data = (int64_t*)data_8) {
		hash = (uint64_t)(*data_8) * MULT6 ^ hash;
		hash = (hash >> 0x35 | hash << 0xb) * MULT3;
		data_8++;
	}
	
	uint64_t final_hash = (hash ^ hash >> 0x21) * MULT4;
	final_hash = (final_hash ^ final_hash >> 0x1d) * ADD2;
	return final_hash ^ final_hash >> 0x20;
}

uint64_t lua_bind_hash_str(const char* str) {
	return lua_bind_hash(str, strlen(str));
}

#endif // LUA_BIND_HASH_H
