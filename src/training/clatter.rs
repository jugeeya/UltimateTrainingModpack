use crate::common::consts::*;
use crate::common::*;
use crate::training::mash;
use smash::app::lua_bind::{ControlModule, EffectModule};
use smash::app::BattleObjectModuleAccessor;
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};

static mut COUNTER: u32 = 0;
static mut WAS_IN_CLATTER_FLAG: bool = false;

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
        Hash40::new("hip"),
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
        if WAS_IN_CLATTER_FLAG && MENU.mash_triggers.contains(MashTrigger::CLATTER) {
            mash::buffer_menu_mash();
        }
        WAS_IN_CLATTER_FLAG = false;
        return;
    }
    WAS_IN_CLATTER_FLAG = true;
    let repeat = MENU.clatter_strength.into_u32();

    COUNTER = (COUNTER + 1) % repeat;
    if COUNTER == repeat - 1 {
        do_clatter_input(module_accessor);
    }
}
