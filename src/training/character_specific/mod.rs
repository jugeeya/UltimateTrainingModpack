use smash::app::{self};
use smash::lib::lua_const::*;

pub mod items;
pub mod kirby;
pub mod pikmin;
pub mod ptrainer;
pub mod steve;

use std::collections::HashMap;
use training_mod_sync::LazyLock;

static CHARACTER_SPECIFIC_STATUS_MAP: LazyLock<HashMap<(i32, i32), Vec<i32>>> =
    LazyLock::new(|| {
        HashMap::from([
            (
                // Bowser Up b
                (*FIGHTER_KIND_KOOPA, *FIGHTER_STATUS_KIND_SPECIAL_HI),
                vec![
                    *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G,
                    *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A,
                ],
            ),
            (
                // Sora Neutral B
                (*FIGHTER_KIND_TRAIL, *FIGHTER_STATUS_KIND_SPECIAL_N),
                vec![
                    *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N1,
                    *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N1_SHOOT,
                    *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N1_END,
                    *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N2,
                    *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N3,
                ],
            ),
            (
                // Sora Nair / Fair
                (*FIGHTER_KIND_TRAIL, *FIGHTER_STATUS_KIND_ATTACK_AIR),
                vec![
                    *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N,
                    *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F,
                ],
            ),
            (
                // Krool Sideb
                (*FIGHTER_KIND_KROOL, *FIGHTER_STATUS_KIND_SPECIAL_S),
                vec![
                    *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_THROW,
                    *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_CATCH,
                    *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_FAILURE,
                    *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_GET,
                ],
            ),
        ])
    });

/**
 * Checks if the current status matches the expected status
 * Returns true if the current_status matches a character-specific status for the expected_status
 * Returns false if it does not match or if there are no character-specific statuses for the expected_status
 */
pub unsafe fn check_character_specific_status(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    current_status: i32,
    expected_status: i32,
) -> bool {
    let fighter_kind = app::utility::get_kind(module_accessor);
    if let Some(statuses) = CHARACTER_SPECIFIC_STATUS_MAP.get(&(fighter_kind, expected_status)) {
        if statuses.contains(&current_status) {
            return true;
        }
    }

    false
}
