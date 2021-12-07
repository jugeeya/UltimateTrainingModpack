use crate::common::consts::*;
use crate::common::*;
use crate::training::mash;
use smash::app::sv_system;
use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterBase;

static mut TECH_ROLL_DIRECTION: Direction = Direction::empty();
static mut MISS_TECH_ROLL_DIRECTION: Direction = Direction::empty();

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
fn handle_grnd_tech(
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

    unsafe {
        let can_tech = WorkModule::is_enable_transition_term(
            module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE,
        );

        if !can_tech {
            return false;
        }
    }

    match state {
        TechFlags::IN_PLACE => {
            *status_kind = FIGHTER_STATUS_KIND_PASSIVE.as_lua_int();
            *unk = LUA_TRUE;
            unsafe {
                mash::perform_defensive_option();
            }
        }
        TechFlags::ROLL_F => {
            *status_kind = FIGHTER_STATUS_KIND_PASSIVE_FB.as_lua_int();
            *unk = LUA_TRUE;
            unsafe {
                TECH_ROLL_DIRECTION = Direction::IN; // = In
                mash::perform_defensive_option();
            }
        }
        TechFlags::ROLL_B => {
            *status_kind = FIGHTER_STATUS_KIND_PASSIVE_FB.as_lua_int();
            *unk = LUA_TRUE;
            unsafe {
                TECH_ROLL_DIRECTION = Direction::OUT; // = Away
                mash::perform_defensive_option();
            }
        }
        _ => (),
    }

    true
}

fn handle_wall_tech(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    status_kind: &mut L2CValue,
    unk: &mut L2CValue,
    status_kind_int: i32,
    state: TechFlags,
) -> bool {
    if status_kind_int != *FIGHTER_STATUS_KIND_STOP_WALL
        && status_kind_int != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
    {
        return false;
    }

    if state == TechFlags::NO_TECH {
        return false;
    }

    unsafe {
        let can_tech = WorkModule::is_enable_transition_term(
            module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        );

        if !can_tech {
            return false;
        }
    }

    match state {
        TechFlags::IN_PLACE => {
            *status_kind = FIGHTER_STATUS_KIND_PASSIVE_WALL.as_lua_int();
            *unk = LUA_TRUE;
        }
        TechFlags::ROLL_F => {
            *status_kind = FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.as_lua_int();
            *unk = LUA_TRUE;
        }
        _ => (),
    }

    true
}

fn handle_ceil_tech(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    status_kind: &mut L2CValue,
    unk: &mut L2CValue,
    status_kind_int: i32,
    state: TechFlags,
) -> bool {
    if status_kind_int != *FIGHTER_STATUS_KIND_STOP_CEIL
        && status_kind_int != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
    {
        return false;
    }

    if state == TechFlags::NO_TECH {
        return false;
    }

    unsafe {
        let can_tech = WorkModule::is_enable_transition_term(
            module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
        );

        if !can_tech {
            return false;
        }
    }

    *status_kind = FIGHTER_STATUS_KIND_PASSIVE_CEIL.as_lua_int();
    *unk = LUA_TRUE;
    true
}

pub unsafe fn get_command_flag_cat(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) {
        return;
    }

    if MENU.tech_state == TechFlags::empty() {
        return;
    }

    let status = StatusModule::status_kind(module_accessor) as i32;

    if [
        *FIGHTER_STATUS_KIND_DOWN_WAIT,
        *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE,
    ]
    .contains(&status)
    {
        let status: i32 = match MENU.miss_tech_state.get_random() {
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

        StatusModule::change_status_request_from_script(module_accessor, status, false);

        mash::perform_defensive_option();
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
