#pragma once
#include "acmd_wrapper.h"
#include "useful/const_value_table.h"
#include "../taunt_toggles.h"

// Uncomment the line below to debug using syslogger.
// #define DEBUG

#ifdef DEBUG
struct LogPacket
{
    bool dirty = false;
    bool to_sd = false;
    char buffer[256];
} logger;

#define syslog(...) snprintf(logger.buffer, 256, __VA_ARGS__); logger.dirty = true; logger.to_sd = true;
#else
#define syslog(...) 
#endif

using namespace app::lua_bind;

int major, minor, patch;

u64 fighter_manager_addr;
u64 is_training_mode(void) asm("_ZN3app9smashball16is_training_modeEv") LINKABLE;

u8 get_category(u64 module_accessor) {
	return (u8)(*(u32*)(module_accessor + 8) >> 28);
}

bool is_operation_cpu(u64 module_accessor) {
    if (get_category(module_accessor) != BATTLE_OBJECT_CATEGORY_FIGHTER)
        return false;

    int entry_id = WorkModule::get_int(module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    u64 fighter_information = FighterManager::get_fighter_information(LOAD64(fighter_manager_addr), entry_id);

    return FighterInformation::is_operation_cpu(fighter_information);
}

bool is_in_hitstun(u64 module_accessor) {
    int status_kind = StatusModule::status_kind(module_accessor);
    return status_kind >= FIGHTER_STATUS_KIND_DAMAGE &&
           status_kind <= FIGHTER_STATUS_KIND_DAMAGE_FALL;
}

bool is_in_shieldstun(u64 module_accessor) {
    int status_kind = StatusModule::status_kind(module_accessor);
    int prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    // If we are taking shield damage or we are droping shield from taking shield damage we are in hitstun
    if(status_kind == FIGHTER_STATUS_KIND_GUARD_DAMAGE || 
        (prev_status == FIGHTER_STATUS_KIND_GUARD_DAMAGE && status_kind == FIGHTER_STATUS_KIND_GUARD_OFF)) {
        return true;
    }

    return false;
}


bool is_in_landing(u64 module_accessor) {
    int status_kind = StatusModule::status_kind(module_accessor);
    return status_kind >= FIGHTER_STATUS_KIND_LANDING &&
           status_kind <= FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT;
}


void perform_defensive_option(u64 module_accessor, int& flag) {
    if (menu.DEFENSIVE_STATE == RANDOM_DEFENSIVE) {
        const int NUM_DEFENSIVE_CMDS = 4;
        int random_cmds[NUM_DEFENSIVE_CMDS] = {
            FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE,
            FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F,
            FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B,
            FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N
        };

        int random_cmd_index = app::sv_math::rand(hash40("fighter"), NUM_DEFENSIVE_CMDS);
        flag |= random_cmds[random_cmd_index];
    } else if (menu.DEFENSIVE_STATE == DEFENSIVE_ROLL) {
        if (app::sv_math::rand(hash40("fighter"), 2))
            flag |= FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F;
        else
            flag |= FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B;
    }
    else if (menu.DEFENSIVE_STATE == DEFENSIVE_SPOTDODGE)
        flag |= FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE;
    else if (menu.DEFENSIVE_STATE == DEFENSIVE_JAB)
        flag |= FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N;
}