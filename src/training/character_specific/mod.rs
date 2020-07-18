mod bowser;

/**
 * Checks if the current status matches the expected status
 */
pub fn check_status(current_status: i32, expected_status: i32) -> bool {
    if bowser::check_up_b(current_status, expected_status) {
        return true;
    }

    false
}
