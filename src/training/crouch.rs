use smash::app::{lua_bind::StatusModule, BattleObjectModuleAccessor};
use smash::lib::lua_const::*;

use crate::common::consts::OnOff;
use crate::common::*;

use training_mod_sync::*;

pub unsafe fn mod_get_stick_y(module_accessor: &mut BattleObjectModuleAccessor) -> Option<f32> {
    if !is_operation_cpu(module_accessor) {
        return None;
    }
    let fighter_status_kind = StatusModule::status_kind(module_accessor);

    if get(&MENU).crouch == OnOff::ON
        && [
            *FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_SQUAT,
            *FIGHTER_STATUS_KIND_SQUAT_B,
            *FIGHTER_STATUS_KIND_SQUAT_F,
            *FIGHTER_STATUS_KIND_SQUAT_RV,
            *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        ]
        .contains(&fighter_status_kind)
    {
        Some(-1.0)
    } else {
        None
    }
}
