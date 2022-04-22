use crate::common::{is_in_clatter, is_operation_cpu, is_training_mode, MENU};
use smash::app::lua_bind::{ControlModule, EffectModule};
use smash::app::BattleObjectModuleAccessor;
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};

static mut COUNTER: u32 = 0;

unsafe fn do_clatter_input(module_accessor: &mut BattleObjectModuleAccessor) {
    ControlModule::add_clatter_time(module_accessor, -8.0, 0);
    let zeros = Vector3f {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    EffectModule::req_on_joint(
        module_accessor,
        Hash40::new("sys_clatter"),
        Hash40::new("top"),
        &zeros,
        &zeros,
        1.0,
        &zeros,
        &zeros,
        true,
        *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32
            | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32
            | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32,
        0,
        0,
    );
}

pub unsafe fn handle_clatter(module_accessor: &mut BattleObjectModuleAccessor) {
    if !is_training_mode() || !is_operation_cpu(module_accessor) {
        return;
    }

    if !is_in_clatter(module_accessor) {
        return;
    }

    COUNTER = (COUNTER + 1) % MENU.clatter_strength.into_u32();
    if COUNTER == 1 {
        do_clatter_input(module_accessor);
    }
}
