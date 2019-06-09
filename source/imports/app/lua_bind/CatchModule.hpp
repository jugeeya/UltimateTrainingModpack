#pragma once

namespace app::lua_bind {
    namespace CatchModule {
        u64 capture_object_id(u64,bool) asm("_ZN3app8lua_bind35CatchModule__capture_object_id_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 capture_pos_x_diff(u64) asm("_ZN3app8lua_bind36CatchModule__capture_pos_x_diff_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 catch_cut(u64,bool,bool) asm("_ZN3app8lua_bind27CatchModule__catch_cut_implEPNS_26BattleObjectModuleAccessorEbb") LINKABLE;
        u64 check_damage(u64) asm("_ZN3app8lua_bind30CatchModule__check_damage_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 cling_cut(u64,bool) asm("_ZN3app8lua_bind27CatchModule__cling_cut_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 is_catch(u64) asm("_ZN3app8lua_bind26CatchModule__is_catch_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set_catch(u64,u64) asm("_ZN3app8lua_bind27CatchModule__set_catch_implEPNS_26BattleObjectModuleAccessorEj") LINKABLE;
        u64 set_send_cut_event(u64,bool) asm("_ZN3app8lua_bind36CatchModule__set_send_cut_event_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 update_pos_cling(u64) asm("_ZN3app8lua_bind34CatchModule__update_pos_cling_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
    }
}