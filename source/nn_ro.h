#ifndef NN_RO_H
#define NN_RO_H

#include <switch.h>

#define nn_ro_LoadModule _ZN2nn2ro10LoadModuleEPNS0_6ModuleEPKvPvmi
extern uint64_t _ZN2nn2ro10LoadModuleEPNS0_6ModuleEPKvPvmi(void* module, void const* unk_1, void* unk_2, unsigned long unk_3, int unk_4) LINKABLE;

typedef unsigned char      undefined;
typedef unsigned char      byte;
typedef unsigned char      dwfenc;
typedef unsigned int       dword;
typedef long long          longlong;
typedef unsigned long long qword;
typedef unsigned char      uchar;
typedef unsigned int       uint;
typedef unsigned long      ulong;
typedef unsigned long long ulonglong;
typedef unsigned char      undefined1;
typedef unsigned short     undefined2;
typedef unsigned int       undefined3;
typedef unsigned int       undefined4;
typedef unsigned long long undefined5;
typedef unsigned long long undefined6;
typedef unsigned long long undefined7;
typedef unsigned long long undefined8;
typedef unsigned short     ushort;
typedef unsigned short     word;

enum module_state {module_unloaded, module_loaded};

typedef struct RoModule_t {
	struct RoModule_t *next;
	struct RoModule_t *prev;
	union {
		void *rel;
		void *rela;
		void *raw;
	} rela_or_rel_plt;
	union {
		void *rel;
		void *rela;
	} rela_or_rel;
	uint64_t module_base;
	void *dyanmic;
	bool is_rela;
	uint64_t rela_or_rel_plt_size;
	void (*dt_init)(void);
	void (*dt_fini)(void);
	uint32_t *hash_bucket;
	uint32_t *hash_chain;
	char *dynstr;
	void *dynsym;
	uint64_t dynstr_size;
	void **got;
	uint64_t rela_dyn_size;
	uint64_t rel_dyn_size;
	uint64_t rel_count;
	uint64_t rela_count;
	uint64_t hash_nchain_value;
	uint64_t hash_nbucket_value;
	uint64_t got_stub_ptr;
} RoModule;

typedef struct Module_t {
	RoModule *module;
	enum module_state state;
	uintptr_t module_address;
	uintptr_t bss_address;
} Module;

typedef struct SmashModule { /* PlaceHolder Structure */
	Module module;
	void *field_0x20;
	void *src_buffer;
	char name[256]; /* Created by retype action */
	undefined field_0x130;
	undefined field_0x131;
	undefined4 is_loaded; // bool
} SmashModule;

# endif // NN_RO_H
