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

    let vis_options = MENU.visualization.to_vec();
    let hitbox_vis = is_training_mode() && vis_options.contains(&VisualizationFlags::HITBOX_VIS);
    let hurtbox_vis = is_training_mode() && vis_options.contains(&VisualizationFlags::HURTBOX_VIS);

    enable_hitbox_vis(hitbox_vis);
    enable_hurtbox_vis(hurtbox_vis);
    enable_special_vis(hitbox_vis);
}
