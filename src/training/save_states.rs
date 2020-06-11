use crate::common::*;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};

#[derive(PartialEq)]
enum SaveState {
    Save,
    NoAction,
    CameraMove,
    PosMove,
}

use SaveState::*;

static mut SAVE_STATE_PLAYER_STATE: SaveState = NoAction;
static mut SAVE_STATE_CPU_STATE: SaveState = NoAction;

static mut SAVE_STATE_X_PLAYER: f32 = 0.0;
static mut SAVE_STATE_Y_PLAYER: f32 = 0.0;
static mut SAVE_STATE_PERCENT_PLAYER: f32 = 0.0;
static mut SAVE_STATE_LR_PLAYER: f32 = -1.0;
static mut SAVE_STATE_SITUATION_KIND_PLAYER: i32 = 0 as i32;

static mut SAVE_STATE_X_CPU: f32 = 0.0;
static mut SAVE_STATE_Y_CPU: f32 = 0.0;
static mut SAVE_STATE_PERCENT_CPU: f32 = 0.0;
static mut SAVE_STATE_LR_CPU: f32 = 1.0;
static mut SAVE_STATE_SITUATION_KIND_CPU: i32 = 0 as i32;

pub unsafe fn save_states(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_training_mode() {
        return;
    }

    let status = StatusModule::status_kind(module_accessor) as i32;
    let save_state_x: *mut f32;
    let save_state_y: *mut f32;
    let save_state_percent: *mut f32;
    let save_state_lr: *mut f32;
    let save_state_situation_kind: *mut i32;
    let save_state: *mut SaveState;
    if is_operation_cpu(module_accessor) {
        save_state_x = &mut SAVE_STATE_X_CPU;
        save_state_y = &mut SAVE_STATE_Y_CPU;
        save_state_percent = &mut SAVE_STATE_PERCENT_CPU;
        save_state_lr = &mut SAVE_STATE_LR_CPU;
        save_state_situation_kind = &mut SAVE_STATE_SITUATION_KIND_CPU;
        save_state = &mut SAVE_STATE_CPU_STATE;
    } else {
        save_state_x = &mut SAVE_STATE_X_PLAYER;
        save_state_y = &mut SAVE_STATE_Y_PLAYER;
        save_state_percent = &mut SAVE_STATE_PERCENT_PLAYER;
        save_state_lr = &mut SAVE_STATE_LR_PLAYER;
        save_state_situation_kind = &mut SAVE_STATE_SITUATION_KIND_PLAYER;
        save_state = &mut SAVE_STATE_PLAYER_STATE;
    }

    // Grab + Dpad up: reset state
    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
    {
        if *save_state == NoAction {
            SAVE_STATE_PLAYER_STATE = CameraMove;
            SAVE_STATE_CPU_STATE = CameraMove;
        }
        return;
    }

    // move to camera bounds
    if *save_state == CameraMove {
        *save_state = PosMove;

        let left_right = (PostureModule::pos_x(module_accessor) > 0.0) as i32 as f32
            - (PostureModule::pos_x(module_accessor) < 0.0) as i32 as f32;
        let y_pos = 20.0;

        let pos = Vector3f {
            x: left_right * 50.0,
            y: y_pos,
            z: 0.0,
        };
        PostureModule::set_pos(module_accessor, &pos);

        // force aerial, because from aerial state we can move anywhere
        if StatusModule::situation_kind(module_accessor) == SITUATION_KIND_GROUND {
            StatusModule::change_status_request(
                module_accessor,
                *FIGHTER_STATUS_KIND_JUMP_SQUAT,
                false,
            );
        }
        return;
    }

    // move to correct pos
    if *save_state == PosMove {
        if StatusModule::situation_kind(module_accessor) == SITUATION_KIND_GROUND {
            return;
        }

        KineticModule::clear_speed_all(module_accessor);

        let pos = Vector3f {
            x: *save_state_x,
            y: *save_state_y,
            z: 0.0,
        };
        PostureModule::set_pos(module_accessor, &pos);
        PostureModule::set_lr(module_accessor, *save_state_lr);
        DamageModule::heal(
            module_accessor,
            -1.0 * DamageModule::damage(module_accessor, 0),
            0,
        );
        DamageModule::add_damage(module_accessor, *save_state_percent, 0);

        StatusModule::set_situation_kind(
            module_accessor,
            app::SituationKind {
                situation_kind: *save_state_situation_kind,
            },
            false,
        );

        if *save_state_situation_kind == SITUATION_KIND_GROUND && status != FIGHTER_STATUS_KIND_WAIT
        {
            StatusModule::change_status_request(
                module_accessor,
                *FIGHTER_STATUS_KIND_CLIFF_WAIT,
                false,
            );
        } else if *save_state_situation_kind == SITUATION_KIND_AIR
            && status != FIGHTER_STATUS_KIND_FALL
        {
            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        } else if *save_state_situation_kind == SITUATION_KIND_CLIFF
            && status != FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE
            && status != FIGHTER_STATUS_KIND_CLIFF_CATCH
        {
            StatusModule::change_status_request(
                module_accessor,
                *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
                false,
            );
        }
        *save_state = NoAction;
        return;
    }

    // Grab + Dpad down: Save state
    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
    {
        SAVE_STATE_PLAYER_STATE = Save;
        SAVE_STATE_CPU_STATE = Save;
    }

    if *save_state == Save {
        *save_state = NoAction;

        *save_state_x = PostureModule::pos_x(module_accessor);
        *save_state_y = PostureModule::pos_y(module_accessor);
        *save_state_lr = PostureModule::lr(module_accessor);
        *save_state_percent = DamageModule::damage(module_accessor, 0);
        *save_state_situation_kind = StatusModule::situation_kind(module_accessor);

        let zeros = Vector3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        EffectModule::req_on_joint(
            module_accessor,
            Hash40::new("sys_deku_flash"),
            Hash40::new("top"),
            &zeros,
            &zeros,
            0.25,
            &zeros,
            &zeros,
            true,
            *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32
                | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32
                | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32,
            0,
            0,
        );
    }
}
