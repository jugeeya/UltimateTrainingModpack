use log::info;
use smash::app;

use crate::common::*;
use crate::training::combo;
use crate::training::frame_counter;
use crate::training::ledge;
use crate::training::mash;
use crate::training::sdi;
use crate::training::shield_tilt;
use crate::training::throw;

pub fn check_reset(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) {
        return;
    }

    info!("Resetting Training Mode...");

    on_reset();
}

pub fn on_reset() {
    mash::full_reset();
    sdi::roll_direction();
    frame_counter::reset_all();
    combo::reset();
    ledge::reset_ledge_delay();
    throw::reset_throw_delay();
    shield_tilt::roll_direction();
}
