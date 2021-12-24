use smash::app::{self};
use smash::lib::lua_const::*;

pub fn check_up_b(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    current_status: i32,
    expected_status: i32,
) -> bool {
    if !is_bowser(module_accessor) {
        return false;
    }

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

fn is_bowser(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let fighter_id;
    unsafe {
        fighter_id = app::utility::get_kind(module_accessor);
    }

    fighter_id == *FIGHTER_KIND_KOOPA
}
