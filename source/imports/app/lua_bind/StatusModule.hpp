#pragma once

namespace app::lua_bind {
    namespace StatusModule {
        u64 change_status_request(u64,int,bool) asm("_ZN3app8lua_bind40StatusModule__change_status_request_implEPNS_26BattleObjectModuleAccessorEib") LINKABLE;
        u64 change_status_request_from_script(u64,int,bool) asm("_ZN3app8lua_bind52StatusModule__change_status_request_from_script_implEPNS_26BattleObjectModuleAccessorEib") LINKABLE;
        u64 delete_status_request_from_script(u64) asm("_ZN3app8lua_bind52StatusModule__delete_status_request_from_script_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 init_settings(u64,int situationKind,int,u64,int groundCliffCheckKind,bool,int,int,int,int) asm("_ZN3app8lua_bind32StatusModule__init_settings_implEPNS_26BattleObjectModuleAccessorENS_13SituationKindEijNS_20GroundCliffCheckKindEbiiii") LINKABLE;
        u64 is_changing(u64) asm("_ZN3app8lua_bind30StatusModule__is_changing_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 is_situation_changed(u64) asm("_ZN3app8lua_bind39StatusModule__is_situation_changed_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        int prev_situation_kind(u64) asm("_ZN3app8lua_bind38StatusModule__prev_situation_kind_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        int prev_status_kind(u64,u64) asm("_ZN3app8lua_bind35StatusModule__prev_status_kind_implEPNS_26BattleObjectModuleAccessorEj") LINKABLE;
        u64 set_keep_situation_air(u64,bool) asm("_ZN3app8lua_bind41StatusModule__set_keep_situation_air_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_situation_kind(u64,int situationKind,bool) asm("_ZN3app8lua_bind37StatusModule__set_situation_kind_implEPNS_26BattleObjectModuleAccessorENS_13SituationKindEb") LINKABLE;
        u64 set_status_kind_interrupt(u64,int) asm("_ZN3app8lua_bind44StatusModule__set_status_kind_interrupt_implEPNS_26BattleObjectModuleAccessorEi") LINKABLE;
        int situation_kind(u64) asm("_ZN3app8lua_bind33StatusModule__situation_kind_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        int status_kind(u64) asm("_ZN3app8lua_bind30StatusModule__status_kind_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 status_kind_interrupt(u64) asm("_ZN3app8lua_bind40StatusModule__status_kind_interrupt_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 status_kind_next(u64) asm("_ZN3app8lua_bind35StatusModule__status_kind_next_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 status_kind_que_from_script(u64) asm("_ZN3app8lua_bind46StatusModule__status_kind_que_from_script_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
    }
}