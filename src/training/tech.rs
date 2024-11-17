use std::collections::HashMap;

use skyline::hooks::{getRegionAddress, Region};
use smash::app::{lua_bind::*, sv_system, BattleObjectModuleAccessor};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterBase;
use smash::phx::{Hash40, Vector3f};

use crate::common::consts::*;
use crate::common::offsets::{
    OFFSET_CHANGE_ACTIVE_CAMERA, OFFSET_SET_TRAINING_FIXED_CAMERA_VALUES,
};
use crate::common::*;
use crate::training::{frame_counter, mash, save_states};
use training_mod_sync::*;

static TECH_ROLL_DIRECTION: RwLock<Direction> = RwLock::new(Direction::empty());
static MISS_TECH_ROLL_DIRECTION: RwLock<Direction> = RwLock::new(Direction::empty());
static NEEDS_VISIBLE: RwLock<bool> = RwLock::new(false);
static DEFAULT_FIXED_CAM_CENTER: RwLock<Vector3f> = RwLock::new(Vector3f {
    x: 0.0,
    y: 0.0,
    z: 0.0,
});

static FRAME_COUNTER: LazyLock<usize> =
    LazyLock::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGame));

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

    let state: TechFlags = read(&MENU).tech_state.get_random();

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
            assign(&TECH_ROLL_DIRECTION, Direction::IN);
            true
        }
        TechFlags::ROLL_B => {
            *status_kind = FIGHTER_STATUS_KIND_PASSIVE_FB.as_lua_int();
            *unk = LUA_TRUE;
            assign(&TECH_ROLL_DIRECTION, Direction::OUT);
            true
        }
        _ => false,
    };
    if do_tech && read(&MENU).mash_triggers.contains(&MashTrigger::TECH) {
        if read(&MENU).tech_action_override == Action::empty() {
            mash::external_buffer_menu_mash(read(&MENU).mash_state.get_random())
        } else {
            mash::external_buffer_menu_mash(read(&MENU).tech_action_override.get_random())
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
    if do_tech && read(&MENU).mash_triggers.contains(&MashTrigger::TECH) {
        if read(&MENU).tech_action_override == Action::empty() {
            mash::external_buffer_menu_mash(read(&MENU).mash_state.get_random())
        } else {
            mash::external_buffer_menu_mash(read(&MENU).tech_action_override.get_random())
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
    if read(&MENU).mash_triggers.contains(&MashTrigger::TECH) {
        if read(&MENU).tech_action_override == Action::empty() {
            mash::external_buffer_menu_mash(read(&MENU).mash_state.get_random())
        } else {
            mash::external_buffer_menu_mash(read(&MENU).tech_action_override.get_random())
        }
    }
    true
}

pub unsafe fn get_command_flag_cat(module_accessor: &mut BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) || read(&MENU).tech_state == TechFlags::empty() {
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
        requested_status = match read(&MENU).miss_tech_state.get_random() {
            MissTechFlags::GETUP => *FIGHTER_STATUS_KIND_DOWN_STAND,
            MissTechFlags::ATTACK => *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
            MissTechFlags::ROLL_F => {
                assign(&MISS_TECH_ROLL_DIRECTION, Direction::IN);
                *FIGHTER_STATUS_KIND_DOWN_STAND_FB
            }
            MissTechFlags::ROLL_B => {
                assign(&MISS_TECH_ROLL_DIRECTION, Direction::OUT);
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
        requested_status = match read(&MENU).miss_tech_state.get_random() {
            MissTechFlags::GETUP => *FIGHTER_STATUS_KIND_DOWN_STAND,
            MissTechFlags::ATTACK => *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
            MissTechFlags::ROLL_F => {
                assign(&MISS_TECH_ROLL_DIRECTION, Direction::IN);
                *FIGHTER_STATUS_KIND_DOWN_STAND_FB
            }
            MissTechFlags::ROLL_B => {
                assign(&MISS_TECH_ROLL_DIRECTION, Direction::OUT);
                *FIGHTER_STATUS_KIND_DOWN_STAND_FB
            }
            _ => return,
        };
    } else if status == *FIGHTER_STATUS_KIND_SLIP_WAIT {
        // Handle slips (like Diddy banana)
        requested_status = match read(&MENU).miss_tech_state.get_random() {
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
        if read(&MENU).mash_triggers.contains(&MashTrigger::MISTECH) {
            if read(&MENU).tech_action_override == Action::empty() {
                mash::external_buffer_menu_mash(read(&MENU).mash_state.get_random())
            } else {
                mash::external_buffer_menu_mash(read(&MENU).tech_action_override.get_random())
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

    if read(&MENU).tech_state == TechFlags::empty() {
        return None;
    }

    if [hash40("passive_stand_f"), hash40("passive_stand_b")].contains(&motion_kind) {
        if read(&TECH_ROLL_DIRECTION) == Direction::IN {
            return Some(hash40("passive_stand_f"));
        } else {
            return Some(hash40("passive_stand_b"));
        }
    } else if [hash40("down_forward_u"), hash40("down_back_u")].contains(&motion_kind) {
        if read(&MISS_TECH_ROLL_DIRECTION) == Direction::IN {
            return Some(hash40("down_forward_u"));
        } else {
            return Some(hash40("down_back_u"));
        }
    } else if [hash40("down_forward_d"), hash40("down_back_d")].contains(&motion_kind) {
        if read(&MISS_TECH_ROLL_DIRECTION) == Direction::IN {
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
    if !is_training_mode() || read(&MENU).tech_hide == OnOff::OFF {
        return;
    }
    let module_accessor = get_module_accessor(FighterId::CPU);
    // Handle invisible tech animations
    let status = StatusModule::status_kind(module_accessor);
    let teching_statuses = [
        *FIGHTER_STATUS_KIND_DOWN,       // Miss tech
        *FIGHTER_STATUS_KIND_PASSIVE,    // Tech in Place
        *FIGHTER_STATUS_KIND_PASSIVE_FB, // Tech Roll
    ];
    if teching_statuses.contains(&status) {
        // Force hide the cursor with fixed camera
        WorkModule::set_float(
            module_accessor,
            800.0,
            *FIGHTER_INSTANCE_WORK_ID_FLOAT_CURSOR_OFFSET_Y,
        );
        // Disable visibility
        if MotionModule::frame(module_accessor) >= 6.0 {
            assign(&NEEDS_VISIBLE, true);
            VisibilityModule::set_whole(module_accessor, false);
            EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_nopassive"), false);
            EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_down_smoke"), false);
            EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_passive"), false);
            EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_crown"), false);
            EffectModule::set_visible_kind(
                module_accessor,
                Hash40::new("sys_crown_collision"),
                false,
            );
        }
        if MotionModule::end_frame(module_accessor) - MotionModule::frame(module_accessor) <= 5.0 {
            // Re-enable visibility
            assign(&NEEDS_VISIBLE, false);
            VisibilityModule::set_whole(module_accessor, true);
        }
    } else {
        // If the CPU's tech status was interrupted, make them visible again
        let mut needs_visible_lock = lock_write(&NEEDS_VISIBLE);
        if *needs_visible_lock {
            *needs_visible_lock = false;
            VisibilityModule::set_whole(module_accessor, true);
        }
        drop(needs_visible_lock);
    }
}

// Prevent Mistech Quake
#[skyline::hook(replace = CameraModule::req_quake_pos)]
pub unsafe fn handle_fighter_req_quake_pos(
    module_accessor: &mut BattleObjectModuleAccessor,
    quake_kind: i32,
) -> u64 {
    if !is_training_mode() || !is_operation_cpu(&mut *module_accessor) {
        return original!()(module_accessor, quake_kind);
    }
    let status = StatusModule::status_kind(module_accessor);
    if status == FIGHTER_STATUS_KIND_DOWN && read(&MENU).tech_hide == OnOff::ON {
        // We're hiding techs, prevent mistech quake from giving away missed tech
        return original!()(module_accessor, *CAMERA_QUAKE_KIND_NONE);
    }
    original!()(module_accessor, quake_kind)
}

// Zoom in the Fixed Camera view while this is on to set up a good situation for practice
#[skyline::hook(offset = *OFFSET_CHANGE_ACTIVE_CAMERA)]
pub unsafe fn handle_change_active_camera(
    camera_manager: *mut u64,
    camera_mode: i32,
    int_1: i32,
    pointer: *mut u64,
    bool_1: bool,
) -> bool {
    if !is_training_mode() || camera_mode != 4 {
        return original!()(camera_manager, camera_mode, int_1, pointer, bool_1);
    }
    // Determine which Fixed Camera Values to have set in Camera Manager based on tech_hide toggle
    set_fixed_camera_values();
    original!()(camera_manager, camera_mode, int_1, pointer, bool_1)
}

pub struct CameraValuesForTraining {
    fixed_camera_center: Vector3f,
    _unk_fixed_camera_horiz_angle: f32, // ?
    _unk_fixed_camera_vert_angle: f32,  // ?
    _unk_3: f32,
    _unk_4: f32,
    _unk_5: f32,
    _unk_6: Vector3f,
    // ^ maybe not even a Vector, but this is where Angle would
    // be stored in the FixedParam CameraParam
}

pub struct CameraManager {
    _padding: [u8; 0xbd0], // Don't need this info for our setup, TNN has this documented if you need
    fixed_camera_center: Vector3f,
}

unsafe fn set_fixed_camera_values() {
    let camera_manager = get_camera_manager();
    if read(&MENU).tech_hide == OnOff::OFF {
        // Use Stage's Default Values for fixed Camera
        camera_manager.fixed_camera_center = read(&DEFAULT_FIXED_CAM_CENTER);
    } else {
        // We're in CameraMode 4, which is Fixed, and we are hiding tech chases, so we want a better view of the stage
        if let Some(camera_vector) = get_stage_camera_values(save_states::stage_id()) {
            camera_manager.fixed_camera_center = camera_vector;
        }
        // Otherwise, the stage doesn't have custom values for the fixed camera for tech chasing, so don't override it
    }
}

pub unsafe fn get_camera_manager() -> &'static mut CameraManager {
    // CameraManager pointer is located here
    let on_cam_mgr_ptr = (getRegionAddress(Region::Text) as u64) + 0x52b6f00;
    let pointer_arith = on_cam_mgr_ptr as *const *mut *mut CameraManager;
    &mut ***pointer_arith
}

fn get_stage_camera_values(stage_id: i32) -> Option<Vector3f> {
    // Used for FD, BF, SBF, Town, and PS2
    let default_vec = Vector3f {
        x: 0.0,
        y: 50.0,
        z: 230.0,
    };
    // Hollow Bastion Values
    let hollow_vec = Vector3f {
        x: 0.0,
        y: 50.0,
        z: 210.0,
    };
    // Smashville Values
    let smashville_vec = Vector3f {
        x: 0.0,
        y: 35.0,
        z: 350.0,
    };

    let stage_vecs: HashMap<i32, Vector3f> = HashMap::from([
        (*StageID::End, default_vec),
        (*StageID::BattleField, default_vec),
        (*StageID::BattleField_S, default_vec),
        (*StageID::Animal_City, default_vec),
        (*StageID::Poke_Stadium2, default_vec),
        (*StageID::Animal_Village, smashville_vec),
        (*StageID::Trail_Castle, hollow_vec),
        // All FD Variants of Stages:
        (*StageID::End_BattleField, default_vec),
        (*StageID::End_Mario_Castle64, default_vec),
        (*StageID::End_DK_Jungle, default_vec),
        (*StageID::End_Zelda_Hyrule, default_vec),
        (*StageID::End_Yoshi_Story, default_vec),
        (*StageID::End_Kirby_Pupupu64, default_vec),
        (*StageID::End_Poke_Yamabuki, default_vec),
        (*StageID::End_Mario_Past64, default_vec),
        (*StageID::End_Mario_CastleDx, default_vec),
        (*StageID::End_Mario_Rainbow, default_vec),
        (*StageID::End_DK_WaterFall, default_vec),
        (*StageID::End_DK_Lodge, default_vec),
        (*StageID::End_Zelda_Greatbay, default_vec),
        (*StageID::End_Zelda_Temple, default_vec),
        (*StageID::End_Yoshi_CartBoard, default_vec),
        (*StageID::End_Yoshi_Yoster, default_vec),
        (*StageID::End_Kirby_Fountain, default_vec),
        (*StageID::End_Kirby_Greens, default_vec),
        (*StageID::End_Fox_Corneria, default_vec),
        (*StageID::End_Fox_Venom, default_vec),
        (*StageID::End_Metroid_ZebesDx, default_vec),
        (*StageID::End_Mother_Onett, default_vec),
        (*StageID::End_Poke_Stadium, default_vec),
        (*StageID::End_Metroid_Kraid, default_vec),
        (*StageID::End_Mother_Fourside, default_vec),
        (*StageID::End_Fzero_Bigblue, default_vec),
        (*StageID::End_Mario_PastUsa, default_vec),
        (*StageID::End_Mario_Dolpic, default_vec),
        (*StageID::End_Yoshi_Island, default_vec),
        (*StageID::End_Fox_LylatCruise, default_vec),
        (*StageID::End_Zelda_Oldin, default_vec),
        (*StageID::End_Animal_Village, default_vec),
        (*StageID::End_Icarus_SkyWorld, default_vec),
        (*StageID::End_FE_Siege, default_vec),
        (*StageID::End_Wario_Madein, default_vec),
        (*StageID::End_Poke_Stadium2, default_vec),
        (*StageID::End_Kirby_Halberd, default_vec),
        (*StageID::End_MG_Shadowmoses, default_vec),
        (*StageID::End_Mother_Newpork, default_vec),
        (*StageID::End_Ice_Top, default_vec),
        (*StageID::End_Metroid_Norfair, default_vec),
        (*StageID::End_Kart_CircuitX, default_vec),
        (*StageID::End_Metroid_Orpheon, default_vec),
        (*StageID::End_Pikmin_Planet, default_vec),
        (*StageID::End_Mario_PastX, default_vec),
        (*StageID::End_Fzero_Porttown, default_vec),
        (*StageID::End_LuigiMansion, default_vec),
        (*StageID::End_Zelda_Pirates, default_vec),
        (*StageID::End_Poke_Tengam, default_vec),
        (*StageID::End_75m, default_vec),
        (*StageID::End_MarioBros, default_vec),
        (*StageID::End_Plankton, default_vec),
        (*StageID::End_Sonic_Greenhill, default_vec),
        (*StageID::End_Mario_3DLand, default_vec),
        (*StageID::End_Mario_NewBros2, default_vec),
        (*StageID::End_Mario_Paper, default_vec),
        (*StageID::End_Zelda_Gerudo, default_vec),
        (*StageID::End_Zelda_Train, default_vec),
        (*StageID::End_Poke_Unova, default_vec),
        (*StageID::End_Poke_Tower, default_vec),
        (*StageID::End_FE_Arena, default_vec),
        (*StageID::End_Icarus_Uprising, default_vec),
        (*StageID::End_Animal_Island, default_vec),
        (*StageID::End_PunchOutSB, default_vec),
        (*StageID::End_PunchOutW, default_vec),
        (*StageID::End_Xeno_Gaur, default_vec),
        (*StageID::End_Nintendogs, default_vec),
        (*StageID::End_StreetPass, default_vec),
        (*StageID::End_Tomodachi, default_vec),
        (*StageID::End_Pictochat2, default_vec),
        (*StageID::End_Rock_Wily, default_vec),
        (*StageID::End_Mother_Magicant, default_vec),
        (*StageID::End_Kirby_Gameboy, default_vec),
        (*StageID::End_BalloonFight, default_vec),
        (*StageID::End_Fzero_Mutecity3DS, default_vec),
        (*StageID::End_Mario_Uworld, default_vec),
        (*StageID::End_Mario_Galaxy, default_vec),
        (*StageID::End_Kart_CircuitFor, default_vec),
        (*StageID::End_Zelda_Skyward, default_vec),
        (*StageID::End_Kirby_Cave, default_vec),
        (*StageID::End_Poke_Kalos, default_vec),
        (*StageID::End_FE_Colloseum, default_vec),
        (*StageID::End_Icarus_Angeland, default_vec),
        (*StageID::End_Wario_Gamer, default_vec),
        (*StageID::End_Pikmin_Garden, default_vec),
        (*StageID::End_Animal_City, default_vec),
        (*StageID::End_WiiFit, default_vec),
        (*StageID::End_WreckingCrew, default_vec),
        (*StageID::End_Pilotwings, default_vec),
        (*StageID::End_WufuIsland, default_vec),
        (*StageID::End_Sonic_Windyhill, default_vec),
        (*StageID::End_Pac_Land, default_vec),
        (*StageID::End_FlatZoneX, default_vec),
        (*StageID::End_DuckHunt, default_vec),
        (*StageID::End_SF_Suzaku, default_vec),
        (*StageID::End_Mario_Maker, default_vec),
        (*StageID::End_FF_Midgar, default_vec),
        (*StageID::End_Bayo_Clock, default_vec),
        (*StageID::End_Spla_Parking, default_vec),
        (*StageID::End_Dracula_Castle, default_vec),
        (*StageID::End_Zelda_Tower, default_vec),
        (*StageID::End_Mario_Odyssey, default_vec),
        (*StageID::End_Jack_Mementoes, default_vec),
        (*StageID::End_Brave_Altar, default_vec),
        (*StageID::End_Buddy_Spiral, default_vec),
        (*StageID::End_Dolly_Stadium, default_vec),
        (*StageID::End_FE_Shrine, default_vec),
        (*StageID::End_Tantan_Spring, default_vec),
        (*StageID::End_Pickel_World, default_vec),
        (*StageID::End_FF_Cave, default_vec),
        (*StageID::End_Xeno_Alst, default_vec),
        (*StageID::End_Demon_Dojo, default_vec),
        (*StageID::End_Trail_Castle, default_vec),
        // All BF Variants of Stages:
        (*StageID::Battle_End, default_vec),
        (*StageID::Battle_Mario_Castle64, default_vec),
        (*StageID::Battle_DK_Jungle, default_vec),
        (*StageID::Battle_Zelda_Hyrule, default_vec),
        (*StageID::Battle_Yoshi_Story, default_vec),
        (*StageID::Battle_Kirby_Pupupu64, default_vec),
        (*StageID::Battle_Poke_Yamabuki, default_vec),
        (*StageID::Battle_Mario_Past64, default_vec),
        (*StageID::Battle_Mario_CastleDx, default_vec),
        (*StageID::Battle_Mario_Rainbow, default_vec),
        (*StageID::Battle_DK_WaterFall, default_vec),
        (*StageID::Battle_DK_Lodge, default_vec),
        (*StageID::Battle_Zelda_Greatbay, default_vec),
        (*StageID::Battle_Zelda_Temple, default_vec),
        (*StageID::Battle_Yoshi_CartBoard, default_vec),
        (*StageID::Battle_Yoshi_Yoster, default_vec),
        (*StageID::Battle_Kirby_Fountain, default_vec),
        (*StageID::Battle_Kirby_Greens, default_vec),
        (*StageID::Battle_Fox_Corneria, default_vec),
        (*StageID::Battle_Fox_Venom, default_vec),
        (*StageID::Battle_Metroid_ZebesDx, default_vec),
        (*StageID::Battle_Mother_Onett, default_vec),
        (*StageID::Battle_Poke_Stadium, default_vec),
        (*StageID::Battle_Metroid_Kraid, default_vec),
        (*StageID::Battle_Mother_Fourside, default_vec),
        (*StageID::Battle_Fzero_Bigblue, default_vec),
        (*StageID::Battle_Mario_PastUsa, default_vec),
        (*StageID::Battle_Mario_Dolpic, default_vec),
        (*StageID::Battle_Yoshi_Island, default_vec),
        (*StageID::Battle_Fox_LylatCruise, default_vec),
        (*StageID::Battle_Zelda_Oldin, default_vec),
        (*StageID::Battle_Animal_Village, default_vec),
        (*StageID::Battle_Icarus_SkyWorld, default_vec),
        (*StageID::Battle_FE_Siege, default_vec),
        (*StageID::Battle_Wario_Madein, default_vec),
        (*StageID::Battle_Poke_Stadium2, default_vec),
        (*StageID::Battle_Kirby_Halberd, default_vec),
        (*StageID::Battle_MG_Shadowmoses, default_vec),
        (*StageID::Battle_Mother_Newpork, default_vec),
        (*StageID::Battle_Ice_Top, default_vec),
        (*StageID::Battle_Metroid_Norfair, default_vec),
        (*StageID::Battle_Kart_CircuitX, default_vec),
        (*StageID::Battle_Metroid_Orpheon, default_vec),
        (*StageID::Battle_Pikmin_Planet, default_vec),
        (*StageID::Battle_Mario_PastX, default_vec),
        (*StageID::Battle_Fzero_Porttown, default_vec),
        (*StageID::Battle_LuigiMansion, default_vec),
        (*StageID::Battle_Zelda_Pirates, default_vec),
        (*StageID::Battle_Poke_Tengam, default_vec),
        (*StageID::Battle_75m, default_vec),
        (*StageID::Battle_MarioBros, default_vec),
        (*StageID::Battle_Plankton, default_vec),
        (*StageID::Battle_Sonic_Greenhill, default_vec),
        (*StageID::Battle_Mario_3DLand, default_vec),
        (*StageID::Battle_Mario_NewBros2, default_vec),
        (*StageID::Battle_Mario_Paper, default_vec),
        (*StageID::Battle_Zelda_Gerudo, default_vec),
        (*StageID::Battle_Zelda_Train, default_vec),
        (*StageID::Battle_Poke_Unova, default_vec),
        (*StageID::Battle_Poke_Tower, default_vec),
        (*StageID::Battle_FE_Arena, default_vec),
        (*StageID::Battle_Icarus_Uprising, default_vec),
        (*StageID::Battle_Animal_Island, default_vec),
        (*StageID::Battle_PunchOutSB, default_vec),
        (*StageID::Battle_PunchOutW, default_vec),
        (*StageID::Battle_Xeno_Gaur, default_vec),
        (*StageID::Battle_Nintendogs, default_vec),
        (*StageID::Battle_StreetPass, default_vec),
        (*StageID::Battle_Tomodachi, default_vec),
        (*StageID::Battle_Pictochat2, default_vec),
        (*StageID::Battle_Rock_Wily, default_vec),
        (*StageID::Battle_Mother_Magicant, default_vec),
        (*StageID::Battle_Kirby_Gameboy, default_vec),
        (*StageID::Battle_BalloonFight, default_vec),
        (*StageID::Battle_Fzero_Mutecity3DS, default_vec),
        (*StageID::Battle_Mario_Uworld, default_vec),
        (*StageID::Battle_Mario_Galaxy, default_vec),
        (*StageID::Battle_Kart_CircuitFor, default_vec),
        (*StageID::Battle_Zelda_Skyward, default_vec),
        (*StageID::Battle_Kirby_Cave, default_vec),
        (*StageID::Battle_Poke_Kalos, default_vec),
        (*StageID::Battle_FE_Colloseum, default_vec),
        (*StageID::Battle_Icarus_Angeland, default_vec),
        (*StageID::Battle_Wario_Gamer, default_vec),
        (*StageID::Battle_Pikmin_Garden, default_vec),
        (*StageID::Battle_Animal_City, default_vec),
        (*StageID::Battle_WiiFit, default_vec),
        (*StageID::Battle_WreckingCrew, default_vec),
        (*StageID::Battle_Pilotwings, default_vec),
        (*StageID::Battle_WufuIsland, default_vec),
        (*StageID::Battle_Sonic_Windyhill, default_vec),
        (*StageID::Battle_Pac_Land, default_vec),
        (*StageID::Battle_FlatZoneX, default_vec),
        (*StageID::Battle_DuckHunt, default_vec),
        (*StageID::Battle_SF_Suzaku, default_vec),
        (*StageID::Battle_Mario_Maker, default_vec),
        (*StageID::Battle_FF_Midgar, default_vec),
        (*StageID::Battle_Bayo_Clock, default_vec),
        (*StageID::Battle_Spla_Parking, default_vec),
        (*StageID::Battle_Dracula_Castle, default_vec),
        (*StageID::Battle_Zelda_Tower, default_vec),
        (*StageID::Battle_Mario_Odyssey, default_vec),
        (*StageID::Battle_Jack_Mementoes, default_vec),
        (*StageID::Battle_Brave_Altar, default_vec),
        (*StageID::Battle_Buddy_Spiral, default_vec),
        (*StageID::Battle_Dolly_Stadium, default_vec),
        (*StageID::Battle_FE_Shrine, default_vec),
        (*StageID::Battle_Tantan_Spring, default_vec),
        (*StageID::BattleField_S, default_vec),
        (*StageID::Battle_Pickel_World, default_vec),
        (*StageID::Battle_FF_Cave, default_vec),
        (*StageID::Battle_Xeno_Alst, default_vec),
        (*StageID::Battle_Demon_Dojo, default_vec),
        (*StageID::Battle_Trail_Castle, default_vec),
    ]);
    stage_vecs.get(&stage_id).copied()
}

// We hook where the training fixed camera fields are initially set, so we can change them later if necessary
#[skyline::hook(offset = *OFFSET_SET_TRAINING_FIXED_CAMERA_VALUES)]
pub unsafe fn handle_set_training_fixed_camera_values(
    camera_manager: *mut u64, // not actually camera manager - is this even used?????
    fixed_camera_values: &mut CameraValuesForTraining,
) {
    if !is_training_mode() {
        return original!()(camera_manager, fixed_camera_values);
    }
    assign(
        &DEFAULT_FIXED_CAM_CENTER,
        fixed_camera_values.fixed_camera_center,
    );
    original!()(camera_manager, fixed_camera_values);
    // Set Fixed Camera Values now, since L + R + A reset switches without calling ChangeActiveCamera
    set_fixed_camera_values();
}

pub fn init() {
    skyline::install_hooks!(
        handle_fighter_req_quake_pos,
        handle_change_active_camera,
        handle_set_training_fixed_camera_values,
    );
}
