use crate::common::consts::*;
use crate::common::*;
use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;

pub unsafe fn force_option(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if StatusModule::status_kind(module_accessor) as i32 != *FIGHTER_STATUS_KIND_CLIFF_WAIT {
        return;
    }

    if !WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB,
    ) {
        return;
    }

    let random_frame = app::sv_math::rand(
        hash40("fighter"),
        MotionModule::end_frame(module_accessor) as i32,
    ) as f32;

    let frame = MotionModule::frame(module_accessor) as f32;
    if !(frame == random_frame || frame > 30.0) {
        return;
    }

    let mut status = 0;
    let ledge_case: LedgeOption;

    if MENU.ledge_state == LedgeOption::Random {
        ledge_case = (app::sv_math::rand(hash40("fighter"), 4) + 2).into();
    } else {
        ledge_case = MENU.ledge_state;
    }

    if let Some(new_status) = ledge_case.into_status() {
        status = new_status;
    }

    StatusModule::change_status_request_from_script(module_accessor, status, true);
}

pub unsafe fn should_perform_defensive_option(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    prev_status: i32,
    status: i32,
) -> bool {
    ([
        *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
        *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
    ]
    .iter()
    .any(|i| i == &status || i == &prev_status))
        && (WorkModule::is_enable_transition_term(
            module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        ) || CancelModule::is_enable_cancel(module_accessor))
}

pub unsafe fn defensive_option(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    _category: i32,
    flag: &mut i32,
) {
    let status = StatusModule::status_kind(module_accessor) as i32;
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0) as i32;

    if !should_perform_defensive_option(module_accessor, prev_status, status) {
        return;
    }

    perform_defensive_option(module_accessor, flag);
}

pub unsafe fn check_button_on(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> Option<bool> {
    if !is_training_mode() {
        return None;
    }

    if !is_operation_cpu(module_accessor) {
        return None;
    }

    if ![*CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_GUARD].contains(&button) {
        return None;
    }

    if MENU.defensive_state != Defensive::Shield {
        return None;
    }

    let prev_status = StatusModule::prev_status_kind(module_accessor, 0) as i32;
    let status = StatusModule::status_kind(module_accessor) as i32;
    if !should_perform_defensive_option(module_accessor, prev_status, status) {
        return None;
    }

    Some(true)
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
    flag: &mut i32,
) {
    if !is_training_mode() {
        return;
    }

    if !is_operation_cpu(module_accessor) {
        return;
    }

    if MENU.ledge_state == LedgeOption::None {
        return;
    }

    force_option(module_accessor);
    defensive_option(module_accessor, category, flag);
}
