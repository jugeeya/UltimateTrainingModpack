#pragma once
#include "../acmd_wrapper.hpp"
#include "../useful/const_value_table.h"
#include "../taunt_toggles.h"

u64 fighter_manager_addr;

bool is_operation_cpu(u64 module_accessor) {
    int entry_id = WorkModule::get_int(module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    u64 fighter_information = FighterManager::get_fighter_information(LOAD64(fighter_manager_addr), entry_id);

    return FighterInformation::is_operation_cpu(fighter_information);
}

bool is_in_hitstun(u64 module_accessor) {
    int status_kind = StatusModule::status_kind(module_accessor);
    return status_kind >= FIGHTER_STATUS_KIND_DAMAGE &&
           status_kind <= FIGHTER_STATUS_KIND_DAMAGE_FALL;
}

bool is_in_landing(u64 module_accessor) {
    int status_kind = StatusModule::status_kind(module_accessor);
    return status_kind >= FIGHTER_STATUS_KIND_LANDING &&
           status_kind <= FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT;
}