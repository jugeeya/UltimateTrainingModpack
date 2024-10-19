use smash::app::lua_bind::{ControlModule, EffectModule};
use smash::app::BattleObjectModuleAccessor;
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};

use crate::common::consts::*;
use crate::common::*;
use crate::sync::*;

static COUNTER: RwLock<u32> = RwLock::new(0);
static CLATTER_STEP: RwLock<f32> = RwLock::new(8.0);

unsafe fn do_clatter_input(module_accessor: &mut BattleObjectModuleAccessor) {
    let clatter_step = read_rwlock(&CLATTER_STEP);
    ControlModule::add_clatter_time(module_accessor, -1.0 * clatter_step, 0);
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
    // TODO: handle swallowed and cargo carry statuses.
    // Look at set_dec_time/set_dec_time_recovery functions
    if !is_training_mode() || !is_operation_cpu(module_accessor) {
        return;
    }
    if !is_in_clatter(module_accessor) {
        // Don't do clatter inputs if we're not in clatter
        return;
    }
    let repeat = MENU.clatter_strength.into_u32();

    let mut counter_guard = lock_write_rwlock(&COUNTER);
    *counter_guard = ((*counter_guard) + 1) % repeat;
    if *counter_guard == repeat - 1 {
        do_clatter_input(module_accessor);
    }
}

#[skyline::hook(replace = ControlModule::start_clatter)]
pub unsafe fn hook_start_clatter(
    module_accessor: &mut BattleObjectModuleAccessor,
    initial_clatter_time: f32,
    auto_recovery_rate: f32,
    manual_recovery_rate: f32,
    arg5: i8,
    arg6: i32,
    arg7: bool,
    arg8: bool,
) -> u64 {
    // This function is called at the beginning of every clatter situation
    // Grab the manual recovery rate and set that as the amount to reduce
    // the clatter time during each simulated input.
    //
    // Most of the time this is 8 frames, but could be less depending on
    // the status (e.g. freeze is 4 frames / input)
    if is_training_mode() && is_operation_cpu(module_accessor) {
        assign_rwlock(&CLATTER_STEP, manual_recovery_rate);
    }
    original!()(
        module_accessor,
        initial_clatter_time,
        auto_recovery_rate,
        manual_recovery_rate,
        arg5,
        arg6,
        arg7,
        arg8,
    )
}
