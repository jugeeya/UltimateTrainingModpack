use crate::common::*;
use crate::training::frame_counter;
use crate::training::ledge;
use crate::training::mash;
use crate::training::sdi;
use crate::training::shield_tilt;
use crate::training::throw;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

pub fn check_reset(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) {
        return;
    }

    if !should_reset(module_accessor) {
        return;
    }

    on_reset();
}

fn should_reset(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if !is_idle(module_accessor) {
        return false;
    }

    let prev_status;

    unsafe {
        prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    }

    // Only reset automatically on training mode reset
    if prev_status != *FIGHTER_STATUS_KIND_NONE {
        return false;
    }

    true
}

pub fn on_reset() {
    mash::full_reset();
    sdi::roll_direction();
    frame_counter::reset_all();
    ledge::reset_ledge_delay();
    throw::reset_throw_delay();
    shield_tilt::roll_direction();
}
