use smash::app::{self};

mod bowser;

/**
 * Checks if the current status matches the expected status
 */
pub fn check_status(module_accessor: &mut app::BattleObjectModuleAccessor, current_status: i32, expected_status: i32) -> bool {
    if bowser::check_up_b(module_accessor, current_status, expected_status) {
        return true;
    }

    false
}
