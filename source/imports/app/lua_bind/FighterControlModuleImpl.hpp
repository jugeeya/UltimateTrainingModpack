#pragma once

namespace app::lua_bind {
    namespace FighterControlModuleImpl {
        u64 check_hit_stop_delay_command(u64,Vector2f *) asm("_ZN3app8lua_bind59FighterControlModuleImpl__check_hit_stop_delay_command_implEPNS_26BattleObjectModuleAccessorERN3phx8Vector2fE") LINKABLE;
        u64 get_param_attack_hi4_flick_y(u64) asm("_ZN3app8lua_bind59FighterControlModuleImpl__get_param_attack_hi4_flick_y_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_param_attack_lw4_flick_y(u64) asm("_ZN3app8lua_bind59FighterControlModuleImpl__get_param_attack_lw4_flick_y_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 get_param_dash_s4_frame(u64) asm("_ZN3app8lua_bind54FighterControlModuleImpl__get_param_dash_s4_frame_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        bool is_enable_hit_stop_delay(u64) asm("_ZN3app8lua_bind55FighterControlModuleImpl__is_enable_hit_stop_delay_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        bool is_enable_hit_stop_delay_life(u64) asm("_ZN3app8lua_bind60FighterControlModuleImpl__is_enable_hit_stop_delay_life_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 reserve_on_attack_button(u64) asm("_ZN3app8lua_bind55FighterControlModuleImpl__reserve_on_attack_button_implEPNS_26BattleObjectModuleAccessorE") LINKABLE;
        u64 update_attack_air_kind(u64,bool) asm("_ZN3app8lua_bind53FighterControlModuleImpl__update_attack_air_kind_implEPNS_26BattleObjectModuleAccessorEb") LINKABLE;
    }
}