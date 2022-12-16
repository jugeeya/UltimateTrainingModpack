use crate::common::consts::*;
use crate::common::*;
use crate::training::{frame_counter, mash};
use smash::app::{self, lua_bind::*, sv_system, BattleObjectModuleAccessor};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterBase;

static mut TECH_ROLL_DIRECTION: Direction = Direction::empty();
static mut MISS_TECH_ROLL_DIRECTION: Direction = Direction::empty();
static mut FRAME_COUNTER: usize = 0;

pub fn init() {
    unsafe {
        FRAME_COUNTER = frame_counter::register_counter();
    }
}

unsafe fn is_enable_passive(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    let fighter = get_fighter_common_from_accessor(module_accessor);
    fighter.is_enable_passive().get_bool()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterBase_change_status)]
pub unsafe fn handle_change_status(
    fighter: &mut L2CFighterBase,
    status_kind: L2CValue,
    unk: L2CValue,
) -> L2CValue {
    let mut status_kind = status_kind;
    let mut unk = unk;

    if is_training_mode() {
        mod_handle_change_status(fighter, &mut status_kind, &mut unk);
    }

    original!()(fighter, status_kind, unk)
}

unsafe fn mod_handle_change_status(
    fighter: &mut L2CFighterBase,
    status_kind: &mut L2CValue,
    unk: &mut L2CValue,
) {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if !is_operation_cpu(module_accessor) {
        return;
    }

    let status_kind_int = status_kind
        .try_get_int()
        .unwrap_or(*FIGHTER_STATUS_KIND_WAIT as u64) as i32;

    let state: TechFlags = MENU.tech_state.get_random();

    if handle_grnd_tech(module_accessor, status_kind, unk, status_kind_int, state) {
        return;
    }

    if handle_wall_tech(module_accessor, status_kind, unk, status_kind_int, state) {
        return;
    }

    handle_ceil_tech(module_accessor, status_kind, unk, status_kind_int, state);
}

unsafe fn handle_grnd_tech(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    status_kind: &mut L2CValue,
    unk: &mut L2CValue,
    status_kind_int: i32,
    state: TechFlags,
) -> bool {
    if status_kind_int != *FIGHTER_STATUS_KIND_DOWN
        && status_kind_int != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
    {
        return false;
    }

    // prev_status_kind(module_accessor, 0) gets the 1st previous status,
    // which is FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON for both aerial/grounded sideb
    // prev_status_kind(module_accessor, 1) gets the 2nd previous status,
    // which is FIGHTER_STATUS_KIND_CATCHED_GANON for grounded sideb
    // and FIGHTER_STATUS_KIND_CATCHED_AIR_GANON for aerial sideb
    let second_prev_status = StatusModule::prev_status_kind(module_accessor, 1);
    let can_tech = WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE,
    ) && (second_prev_status != FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON)
        && is_enable_passive(module_accessor);

    if !can_tech {
        return false;
    }

    let do_tech: bool = match state {
        TechFlags::IN_PLACE => {
            *status_kind = FIGHTER_STATUS_KIND_PASSIVE.as_lua_int();
            *unk = LUA_TRUE;
            true
        }
        TechFlags::ROLL_F => {
            *status_kind = FIGHTER_STATUS_KIND_PASSIVE_FB.as_lua_int();
            *unk = LUA_TRUE;
            TECH_ROLL_DIRECTION = Direction::IN; // = In
            true
        }
        TechFlags::ROLL_B => {
            *status_kind = FIGHTER_STATUS_KIND_PASSIVE_FB.as_lua_int();
            *unk = LUA_TRUE;
            TECH_ROLL_DIRECTION = Direction::OUT; // = Away
            true
        }
        _ => false,
    };
    if do_tech && MENU.mash_triggers.contains(MashTrigger::TECH) {
        if MENU.tech_action_state == Action::empty() {
            mash::buffer_menu_mash(MENU.mash_state.get_random())
        } else {
            mash::buffer_menu_mash(MENU.tech_action_state.get_random())
        }
    }

    true
}

unsafe fn handle_wall_tech(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    status_kind: &mut L2CValue,
    unk: &mut L2CValue,
    status_kind_int: i32,
    state: TechFlags,
) -> bool {
    let can_tech = WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
    ) && is_enable_passive(module_accessor);

    if ![
        *FIGHTER_STATUS_KIND_STOP_WALL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
    ]
    .contains(&status_kind_int)
        || state == TechFlags::NO_TECH
        || !can_tech
    {
        return false;
    }

    let do_tech: bool = match state {
        TechFlags::IN_PLACE => {
            *status_kind = FIGHTER_STATUS_KIND_PASSIVE_WALL.as_lua_int();
            *unk = LUA_TRUE;
            true
        }
        TechFlags::ROLL_F => {
            *status_kind = FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.as_lua_int();
            *unk = LUA_TRUE;
            true
        }
        _ => false,
    };
    if do_tech && MENU.mash_triggers.contains(MashTrigger::TECH) {
        if MENU.tech_action_state == Action::empty() {
            mash::buffer_menu_mash(MENU.mash_state.get_random())
        } else {
            mash::buffer_menu_mash(MENU.tech_action_state.get_random())
        }
    }
    true
}

unsafe fn handle_ceil_tech(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    status_kind: &mut L2CValue,
    unk: &mut L2CValue,
    status_kind_int: i32,
    state: TechFlags,
) -> bool {
    let can_tech = WorkModule::is_enable_transition_term(
        module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
    ) && is_enable_passive(module_accessor);

    if ![
        *FIGHTER_STATUS_KIND_STOP_CEIL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
    ]
    .contains(&status_kind_int)
        || state == TechFlags::NO_TECH
        || !can_tech
    {
        return false;
    }

    *status_kind = FIGHTER_STATUS_KIND_PASSIVE_CEIL.as_lua_int();
    *unk = LUA_TRUE;
    if MENU.mash_triggers.contains(MashTrigger::TECH) {
        if MENU.tech_action_state == Action::empty() {
            mash::buffer_menu_mash(MENU.mash_state.get_random())
        } else {
            mash::buffer_menu_mash(MENU.tech_action_state.get_random())
        }
    }
    true
}

pub unsafe fn get_command_flag_cat(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) || MENU.tech_state == TechFlags::empty() {
        return;
    }

    let status = StatusModule::status_kind(module_accessor) as i32;
    let mut requested_status: i32 = 0;
    if [
        *FIGHTER_STATUS_KIND_DOWN_WAIT,
        *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE,
    ]
    .contains(&status)
    {
        // Mistech
        requested_status = match MENU.miss_tech_state.get_random() {
            MissTechFlags::GETUP => *FIGHTER_STATUS_KIND_DOWN_STAND,
            MissTechFlags::ATTACK => *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
            MissTechFlags::ROLL_F => {
                MISS_TECH_ROLL_DIRECTION = Direction::IN; // = In
                *FIGHTER_STATUS_KIND_DOWN_STAND_FB
            }
            MissTechFlags::ROLL_B => {
                MISS_TECH_ROLL_DIRECTION = Direction::OUT; // = Away
                *FIGHTER_STATUS_KIND_DOWN_STAND_FB
            }
            _ => return,
        };
    } else if status == *FIGHTER_STATUS_KIND_LAY_DOWN {
        // Snake down throw
        let lockout_time = get_snake_laydown_lockout_time(module_accessor);
        if frame_counter::should_delay(lockout_time, FRAME_COUNTER) {
            return;
        };
        requested_status = match MENU.miss_tech_state.get_random() {
            MissTechFlags::GETUP => *FIGHTER_STATUS_KIND_DOWN_STAND,
            MissTechFlags::ATTACK => *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
            MissTechFlags::ROLL_F => {
                MISS_TECH_ROLL_DIRECTION = Direction::IN; // = In
                *FIGHTER_STATUS_KIND_DOWN_STAND_FB
            }
            MissTechFlags::ROLL_B => {
                MISS_TECH_ROLL_DIRECTION = Direction::OUT; // = Away
                *FIGHTER_STATUS_KIND_DOWN_STAND_FB
            }
            _ => return,
        };
    } else if status == *FIGHTER_STATUS_KIND_SLIP_WAIT {
        // Handle slips (like Diddy banana)
        requested_status = match MENU.miss_tech_state.get_random() {
            MissTechFlags::GETUP => *FIGHTER_STATUS_KIND_SLIP_STAND,
            MissTechFlags::ATTACK => *FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK,
            MissTechFlags::ROLL_F => *FIGHTER_STATUS_KIND_SLIP_STAND_F,
            MissTechFlags::ROLL_B => *FIGHTER_STATUS_KIND_SLIP_STAND_B,
            _ => return,
        };
    } else {
        // Not in a tech situation, make sure the snake dthrow counter is fully reset.
        frame_counter::full_reset(FRAME_COUNTER);
    };

    if requested_status != 0 {
        StatusModule::change_status_request_from_script(module_accessor, requested_status, false);
        if MENU.mash_triggers.contains(MashTrigger::MISTECH) {
            if MENU.tech_action_state == Action::empty() {
                mash::buffer_menu_mash(MENU.mash_state.get_random())
            } else {
                mash::buffer_menu_mash(MENU.tech_action_state.get_random())
            }
        }
    }
}

pub unsafe fn change_motion(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    motion_kind: u64,
) -> Option<u64> {
    if !is_operation_cpu(module_accessor) {
        return None;
    }

    if MENU.tech_state == TechFlags::empty() {
        return None;
    }

    if [hash40("passive_stand_f"), hash40("passive_stand_b")].contains(&motion_kind) {
        if TECH_ROLL_DIRECTION == Direction::IN {
            return Some(hash40("passive_stand_f"));
        } else {
            return Some(hash40("passive_stand_b"));
        }
    } else if [hash40("down_forward_u"), hash40("down_back_u")].contains(&motion_kind) {
        if MISS_TECH_ROLL_DIRECTION == Direction::IN {
            return Some(hash40("down_forward_u"));
        } else {
            return Some(hash40("down_back_u"));
        }
    } else if [hash40("down_forward_d"), hash40("down_back_d")].contains(&motion_kind) {
        if MISS_TECH_ROLL_DIRECTION == Direction::IN {
            return Some(hash40("down_forward_d"));
        } else {
            return Some(hash40("down_back_d"));
        }
    }

    None
}

unsafe fn get_snake_laydown_lockout_time(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> u32 {
    let base_lockout_time: f32 = WorkModule::get_param_float(
        module_accessor,
        hash40("common"),
        hash40("laydown_no_action_frame"),
    );
    let max_lockout_time: f32 = WorkModule::get_param_float(
        module_accessor,
        hash40("common"),
        hash40("laydown_no_action_frame_max"),
    );
    let max_lockout_damage: f32 = WorkModule::get_param_float(
        module_accessor,
        hash40("common"),
        hash40("laydown_damage_max"),
    );
    let damage: f32 = DamageModule::damage(module_accessor, 0);
    std::cmp::min(
        (base_lockout_time + (damage / max_lockout_damage) * (max_lockout_time - base_lockout_time))
            as u32,
        max_lockout_time as u32,
    )
}
