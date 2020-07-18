use smash::lib::lua_const::*;

pub fn check_up_b(current_status: i32,expected_status: i32) -> bool {
    if expected_status != *FIGHTER_STATUS_KIND_SPECIAL_HI {
        return false;
    }

    // Grounded up B
    if current_status == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G {
        return true;
    }

    // Aerial up B
    if current_status == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A {
        return true;
    }

    false
}