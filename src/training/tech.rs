use smash::app::{lua_bind::*, sv_system, BattleObjectModuleAccessor};
use smash::hash40;
use smash::phx::{Hash40, Vector3f};
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterBase;

use crate::common::consts::*;
use crate::common::*;
use crate::training::{frame_counter, mash};

use once_cell::sync::Lazy;

static mut TECH_ROLL_DIRECTION: Direction = Direction::empty();
static mut MISS_TECH_ROLL_DIRECTION: Direction = Direction::empty();
static mut NEEDS_VISIBLE: bool = false;

static FRAME_COUNTER: Lazy<usize> =
    Lazy::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGame));

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
    fighter: &L2CFighterBase,
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
    module_accessor: &mut BattleObjectModuleAccessor,
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
        if MENU.tech_action_override == Action::empty() {
            mash::external_buffer_menu_mash(MENU.mash_state.get_random())
        } else {
            mash::external_buffer_menu_mash(MENU.tech_action_override.get_random())
        }
    }

    true
}

unsafe fn handle_wall_tech(
    module_accessor: &mut BattleObjectModuleAccessor,
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
        if MENU.tech_action_override == Action::empty() {
            mash::external_buffer_menu_mash(MENU.mash_state.get_random())
        } else {
            mash::external_buffer_menu_mash(MENU.tech_action_override.get_random())
        }
    }
    true
}

unsafe fn handle_ceil_tech(
    module_accessor: &mut BattleObjectModuleAccessor,
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
        if MENU.tech_action_override == Action::empty() {
            mash::external_buffer_menu_mash(MENU.mash_state.get_random())
        } else {
            mash::external_buffer_menu_mash(MENU.tech_action_override.get_random())
        }
    }
    true
}

pub unsafe fn get_command_flag_cat(module_accessor: &mut BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) || MENU.tech_state == TechFlags::empty() {
        return;
    }

    let status = StatusModule::status_kind(module_accessor);
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
        if frame_counter::should_delay(lockout_time, *FRAME_COUNTER) {
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
        frame_counter::full_reset(*FRAME_COUNTER);
    };

    if requested_status != 0 {
        StatusModule::change_status_force(module_accessor, requested_status, true);
        if MENU.mash_triggers.contains(MashTrigger::MISTECH) {
            if MENU.tech_action_override == Action::empty() {
                mash::external_buffer_menu_mash(MENU.mash_state.get_random())
            } else {
                mash::external_buffer_menu_mash(MENU.tech_action_override.get_random())
            }
        }
    }
}

pub unsafe fn change_motion(
    module_accessor: &mut BattleObjectModuleAccessor,
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

unsafe fn get_snake_laydown_lockout_time(module_accessor: &mut BattleObjectModuleAccessor) -> u32 {
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

pub unsafe fn hide_tech() {
    if !is_training_mode() || MENU.tech_hide == OnOff::Off {
        return;
    }
    let module_accessor = get_module_accessor(FighterId::CPU);
    // Handle invisible tech animations 
    let status = StatusModule::status_kind(module_accessor);
    let teching_statuses = [
        *FIGHTER_STATUS_KIND_DOWN, // Miss tech
        *FIGHTER_STATUS_KIND_PASSIVE, // Tech in Place
        *FIGHTER_STATUS_KIND_PASSIVE_FB, // Tech Roll
    ];
    if teching_statuses.contains(&status) { 
        // Force hide the cursor with fixed camera
        WorkModule::set_float(module_accessor, 800.0,*FIGHTER_INSTANCE_WORK_ID_FLOAT_CURSOR_OFFSET_Y);
        // Disable visibility
        if MotionModule::frame(module_accessor) >= 6.0
        {
            NEEDS_VISIBLE = true;
            VisibilityModule::set_whole(module_accessor, false);
            EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_nopassive"), false);
            EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_down_smoke"), false);
            EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_passive"), false);
            EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_crown"), false);
            EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_crown_collision"), false);
        }
        if MotionModule::end_frame(module_accessor) - MotionModule::frame(module_accessor) <= 5.0 { // Re-enable visibility
            NEEDS_VISIBLE = false;
            VisibilityModule::set_whole(module_accessor, true);
        }
    } else {
        // If the CPU's tech status was interrupted, make them visible again
        if NEEDS_VISIBLE {
            NEEDS_VISIBLE = false;
            VisibilityModule::set_whole(module_accessor, true);
        }
    }
}

pub struct FighterCameraModule {
    _vtable: u64,
    owner: *mut BattleObjectModuleAccessor,
}

// Prevent Mistech Quake
#[skyline::hook(offset = 0x3ec820)]
pub unsafe fn handle_fighter_req_quake_pos(
    camera_module: &mut FighterCameraModule,
    quake_kind: i32,
) -> u64 {
    let module_accessor = camera_module.owner;
    if !is_training_mode() || !is_operation_cpu(&mut *module_accessor) {
        return original!()(camera_module, quake_kind);
    }
    let status = StatusModule::status_kind(module_accessor);
    if status == FIGHTER_STATUS_KIND_DOWN && MENU.tech_hide == OnOff::On { 
        // We're hiding techs, prevent mistech quake from giving away missed tech
        return original!()(camera_module, *CAMERA_QUAKE_KIND_NONE);
    }
    original!()(camera_module, quake_kind)
}

// Zoom in the Fixed Camera view while this is on to set up a good situation for practice
#[skyline::hook(offset = 0x4ee460)]
pub unsafe fn handle_change_active_camera(
    camera_manager: *mut u64,
    camera_mode: i32,
    int_1: i32,
    pointer: *mut u64,
    bool_1: bool,
) -> bool {
    let ori = original!()(camera_manager, camera_mode, int_1, pointer, bool_1);
    if !is_training_mode() || MENU.tech_hide == OnOff::Off || camera_mode != 4 {
        return ori;
    }
    // We're in CameraMode 4, which is Fixed, and we are hiding tech chases, so we want a better view of the stage
    // Zoom in the camera for a better view for tech chasing
    // TODO: Call handle_set_training_fixed_camera_values somehow, maybe store the CameraManager pointer?
    ori
}

pub struct CameraValuesForTraining {
    fixed_camera_center: Vector3f,
    unk_fixed_camera_horiz_angle: f32, // ?
    unk_fixed_camera_vert_angle: f32, // ?
    unk_3: f32,
    unk_4: f32,
    unk_5: f32,
    unk_6: Vector3f, // maybe not even a Vector, but this is where Angle would be stored in Params
}

pub struct CameraManager {
    padding: [u8; 0xbd0], // Don't need this info for our setup, TNN has this documented if you need
    fixed_camera_center: Vector3f,
}

pub unsafe fn get_camera_manager() -> &mut CameraManager {
    // CameraManager pointer is located here
    let on_cam_mgr_ptr = (getRegionAddress(Region::Text) as u64) + 0x52b6f00;
    let pointer_arith = (on_cam_mgr_ptr as *const *mut *mut CameraManager);
    &mut ***pointer_arith
}
//     stage_id = 
// fn get_stage_camera_values() -> Vector3f {save_states::stage_id();
//     let offsets: HashMap<i32, Vector3f> = HashMap::from([
//         (*StageID::Animal_Village, 1.195),
//         (*StageID::Animal_City, 1.448),
//         (*StageID::Yoshi_Island, -1.053),
//     ]);

//     *offsets.get(&stage_id).unwrap_or(&0.0)
// }

// We hook where the training fixed camera fields are initially set, so we can change them later if necessary
#[skyline::hook(offset = 0x3157bb0)]
pub unsafe fn handle_set_training_fixed_camera_values(
    camera_manager: *mut u64, // not actually camera manager - is this even used?????
    fixed_camera_values: &mut CameraValuesForTraining,
) {
    println!(
        "x: {}, y: {}, z: {}, unk_1: {}, unk_2: {}, unk_3: {}, unk_4: {}, unk_5: {}, unk_6_x: {}, unk_6_y: {}, unk_6_z: {}",
        fixed_camera_values.fixed_camera_center.x,
        fixed_camera_values.fixed_camera_center.y,
        fixed_camera_values.fixed_camera_center.z,
        fixed_camera_values.unk_fixed_camera_horiz_angle,
        fixed_camera_values.unk_fixed_camera_vert_angle,
        fixed_camera_values.unk_3,
        fixed_camera_values.unk_4,
        fixed_camera_values.unk_5,
        fixed_camera_values.unk_6.x,
        fixed_camera_values.unk_6.y,
        fixed_camera_values.unk_6.z,
    );
    original!()(camera_manager, fixed_camera_values);
}

// TODO: zoom_in doesn't work here - need to implement something else for the zoom, like changing the training fixed zoom camera params
// Search "Camera" in Ghidra data bottom left for this

pub fn init() {
    skyline::install_hooks!(
        handle_fighter_req_quake_pos,
        handle_change_active_camera,
        handle_set_training_fixed_camera_values,
    );
}