use crate::common::{consts::*, *};
use smash::app;

extern "C" {
    fn enable_hitbox_vis(enable: bool);
    fn enable_hurtbox_vis(enable: bool);
    fn enable_special_vis(enable: bool);
}

pub unsafe fn get_command_flag_cat(_module_accessor: &mut app::BattleObjectModuleAccessor) {
    if (enable_hitbox_vis as *const ()).is_null() {
        panic!("The visualizer plugin is not found. Please check your Skyline plugins directory for visualizer.nro.");
    }

    let visualization = !is_training_mode() && MENU.hitbox_vis == OnOff::On;

    enable_hitbox_vis(visualization);
    enable_hurtbox_vis(visualization);
    enable_special_vis(visualization);
}
