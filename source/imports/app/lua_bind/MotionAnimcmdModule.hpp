#pragma once

namespace app::lua_bind {
    namespace MotionAnimcmdModule {
        u64 call_script_single(u64,int,u64,int) asm("_ZN3app8lua_bind44MotionAnimcmdModule__call_script_single_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Ei") LINKABLE;
        u64 change_script_motion_line_single(u64,int,u64,int) asm("_ZN3app8lua_bind58MotionAnimcmdModule__change_script_motion_line_single_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Ei") LINKABLE;
        u64 change_script_motion_lines(u64,u64,float,bool,bool,float,bool) asm("_ZN3app8lua_bind52MotionAnimcmdModule__change_script_motion_lines_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efbbfb") LINKABLE;
        u64 change_script_motion_partial_lines(u64,u64,float,bool,float,bool) asm("_ZN3app8lua_bind60MotionAnimcmdModule__change_script_motion_partial_lines_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efbfb") LINKABLE;
        u64 enable_skip_delay_update(u64) asm("_ZN3app8lua_bind50MotionAnimcmdModule__enable_skip_delay_update_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 exec_motion_lines_initialize(u64,float,bool) asm("_ZN3app8lua_bind54MotionAnimcmdModule__exec_motion_lines_initialize_implEPNS_26BattleObjectModuleAccessorEfb") LINKABLE;
        u64 flush(u64,bool) asm("_ZN3app8lua_bind31MotionAnimcmdModule__flush_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 flush_current_motion(u64) asm("_ZN3app8lua_bind46MotionAnimcmdModule__flush_current_motion_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 set_sleep(u64,bool) asm("_ZN3app8lua_bind35MotionAnimcmdModule__set_sleep_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_sleep_effect(u64,bool) asm("_ZN3app8lua_bind42MotionAnimcmdModule__set_sleep_effect_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_sleep_game(u64,bool) asm("_ZN3app8lua_bind40MotionAnimcmdModule__set_sleep_game_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
        u64 set_sleep_sound(u64,bool) asm("_ZN3app8lua_bind41MotionAnimcmdModule__set_sleep_sound_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
    }
}
