#pragma once

namespace app::lua_bind {
    namespace FighterStatusModuleImpl {
        u64 reset_log_action_info(u64,u64) asm("_ZN3app8lua_bind51FighterStatusModuleImpl__reset_log_action_info_implEPNS_26BattleObjectModuleAccessorEm") LINKABLE;
        u64 set_fighter_status_data(u64,bool,int,bool,bool,bool,u64,u64,u64,u64) asm("_ZN3app8lua_bind53FighterStatusModuleImpl__set_fighter_status_data_implEPNS_26BattleObjectModuleAccessorEbibbbmjjj") LINKABLE;
    }
}
