#ifndef L2C_H
#define L2C_H

//#include "useful.h"
//#include "crc32.h"

typedef struct Hash40
{
    uint64_t hash : 40;
} Hash40;

enum L2CVarType
{
    L2C_void = 0,
    L2C_bool = 1,
    L2C_integer = 2,
    L2C_number = 3,
    L2C_pointer = 4,
    L2C_table = 5,
    L2C_inner_function = 6,
    L2C_hash = 7,
    L2C_string = 8,
};

typedef struct L2CTable_meta
{
    uint64_t a;
    uint64_t b;
    uint64_t c;
    uint64_t d;
} L2CTable_meta;

typedef struct L2CTable
{
    uint32_t refcnt;
    uint32_t unk;
    
    uint64_t begin; // L2CValue*
    uint64_t end; // L2CValue*
    uint64_t also_end; // L2CValue*
    struct L2CTable_meta meta;
    uint64_t unk_ptr;
} L2CTable;

typedef struct L2CInnerFunctionBase
{
    uint64_t unk;
    uint32_t refcnt;
} L2CInnerFunctionBase;

typedef struct L2CValue
{
    uint32_t type;
    uint32_t unk;
    union
    {
        uint64_t raw;
        float raw_float;
        void* raw_pointer;
        struct L2CTable* raw_table;
        struct L2CInnerFunctionBase* raw_innerfunc;
        //std::string* raw_string;
    };
} L2CValue;

typedef struct L2CAgent
{
    uint64_t vtable;
    uint64_t lua_state_agent;
    uint64_t unk10;
    uint64_t unk18;
    uint64_t unk20;
    uint64_t unk28;
    uint64_t unk30;
    uint64_t unk38;
    uint64_t lua_state_agentbase;
} L2CAgent;

typedef struct lua_State_smash
{
    uint64_t unk0;
    uint64_t unk8;
    uint64_t unk10;
    uint64_t unk18;
    uint64_t unk20;
    uint64_t unk28;
    uint64_t unk30;
    uint64_t unk38;
    uint64_t unk40;
    uint64_t unkptr48;
    uint64_t unkptr50;
    uint64_t unk58;
    uint64_t unk60;
    uint64_t unk68;
    uint64_t unk70;
    uint64_t unk78;
    uint64_t unk80;
    uint64_t unk88;
    uint64_t unk90;
    uint64_t unkptr98;
    uint64_t unkptrA0;
    uint64_t unkA8;
    uint64_t unkB0;
    uint64_t unkB8;
    uint64_t unkC0;
    uint64_t unkptrC8;
    uint64_t unkD0;
    uint64_t unkD8;
    uint64_t unkE0;
    uint64_t unkE8;
    uint64_t unkF0;
    uint64_t unkF8;
    uint64_t unk100;
    uint64_t unk108;
    uint64_t unk110;
    uint64_t unkptr118;
    uint64_t unk120;
    uint64_t unk128;
    uint64_t unk130;
    uint64_t unk138;
    uint64_t unk140;
    uint64_t unk148;
    uint64_t unk150;
    uint64_t unkptr158;
    uint64_t unk160;
    uint64_t unk168;
    uint64_t unk170;
    uint64_t unk178;
    uint64_t unk180;
    uint64_t unk188;
    uint64_t unk190;
    uint64_t unk198;
    uint64_t unk1A0;
    uint64_t unk1A8;
    uint64_t unk1B0;
    uint64_t unk1B8;
    uint64_t unk1C0;
    uint64_t unk1C8;
    uint64_t unk1D0;
    uint64_t unk1D8;
    uint64_t unk1E0;
    uint64_t unk1E8;
    uint64_t unk1F0;
    uint64_t unk1F8;
    uint64_t unk200;
    uint64_t unk208;
    uint64_t unk210;
    uint64_t unk218;
} lua_State_smash;

struct lua_Stateptr48
{
    uint64_t vtable;
};

struct lua_Stateptr50
{
    uint64_t vtable;
};

struct lua_Stateptr98
{
    uint64_t vtable;
};

struct lua_StateptrA0
{
    uint64_t vtable;
};

struct lua_StateptrC8
{
    uint64_t vtable;
};

struct lua_Stateptr118
{
    uint64_t vtable;
};

struct lua_Stateptr158
{
    uint64_t vtable;
};

struct lua_Stateptr50Vtable
{
    uint64_t unk0;
    uint64_t unk8;
    uint64_t unk10;
    uint64_t unk18;
    uint64_t unk20;
    uint64_t unk28;
    uint64_t unk30;
    uint64_t unk38;
    uint64_t unk40;
    uint64_t unk48;
    uint64_t unk50;
    uint64_t unk58;
    uint64_t unk60;
    uint64_t unk68;
    uint64_t unk70;
    uint64_t unk78;
    uint64_t unk80;
    uint64_t unk88;
    uint64_t unk90;
    uint64_t unk98;
    uint64_t unkA0;
    uint64_t unkA8;
    uint64_t unkB0;
    uint64_t unkB8;
    uint64_t unkC0;
    uint64_t unkC8;
    uint64_t unkD0;
    uint64_t unkD8;
    uint64_t unkE0;
    uint64_t unkE8;
    uint64_t unkF0;
    uint64_t unkF8;
    uint64_t unk100;
    uint64_t unk108;
    uint64_t unk110;
    uint64_t unk118;
    uint64_t unk120;
    uint64_t unk128;
    uint64_t unk130;
    uint64_t unk138;
    uint64_t unk140;
    uint64_t unk148;
    uint64_t unk150;
    uint64_t unk158;
    uint64_t unk160;
    uint64_t unk168;
    uint64_t unk170;
    uint64_t unk178;
    uint64_t unk180;
    uint64_t unk188;
    uint64_t unk190;
    uint64_t unk198;
    uint64_t unk1A0;
    uint64_t unk1A8;
    uint64_t unk1B0;
    uint64_t unk1B8;
    uint64_t unk1C0;
    uint64_t unk1C8;
    uint64_t unk1D0;
    uint64_t unk1D8;
    uint64_t unk1E0;
    uint64_t unk1E8;
    uint64_t unk1F0;
    uint64_t unk1F8;
};

#endif // L2C_H