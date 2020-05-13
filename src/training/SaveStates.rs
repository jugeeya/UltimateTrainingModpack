use crate::common::*;
use smash::app::lua_bind::*;
use smash::app::{self};
use smash::lib::lua_const::*;
use smash::phx::Vector3f;

#[derive(PartialEq)]
enum SaveState {
    Save,
    NoAction,
    CameraMove,
    PosMove,
}

use crate::training::SaveStates::SaveState::*;

static mut save_state_player_state: SaveState = NoAction;
static mut save_state_cpu_state: SaveState = NoAction;
static mut save_state_move_alert: bool = false;

static mut save_state_x_player: f32 = 0.0;
static mut save_state_y_player: f32 = 0.0;
static mut save_state_percent_player: f32 = 0.0;
static mut save_state_lr_player: f32 = 1.0;
static mut save_state_situation_kind_player: i32 = 0 as i32;

static mut save_state_x_cpu: f32 = 0.0;
static mut save_state_y_cpu: f32 = 0.0;
static mut save_state_percent_cpu: f32 = 0.0;
static mut save_state_lr_cpu: f32 = 1.0;
static mut save_state_situation_kind_cpu: i32 = 0 as i32;

pub unsafe fn save_states(module_accessor: &mut app::BattleObjectModuleAccessor) {
    let status = StatusModule::status_kind(module_accessor) as i32;
    if is_training_mode() {
        let save_state_x: *mut f32;
        let save_state_y: *mut f32;
        let save_state_percent: *mut f32;
        let save_state_lr: *mut f32;
        let save_state_situation_kind: *mut i32;
        let save_state: *mut SaveState;
        if is_operation_cpu(module_accessor) {
            save_state_x = &mut save_state_x_cpu;
            save_state_y = &mut save_state_y_cpu;
            save_state_percent = &mut save_state_percent_cpu;
            save_state_lr = &mut save_state_lr_cpu;
            save_state_situation_kind = &mut save_state_situation_kind_cpu;
            save_state = &mut save_state_cpu_state;
        } else {
            save_state_x = &mut save_state_x_player;
            save_state_y = &mut save_state_y_player;
            save_state_percent = &mut save_state_percent_player;
            save_state_lr = &mut save_state_lr_player;
            save_state_situation_kind = &mut save_state_situation_kind_player;
            save_state = &mut save_state_player_state;
        }

        // Grab + Dpad up: reset state
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH) != 0
            && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                != 0
        {
            if *save_state == NoAction {
                save_state_player_state = CameraMove;
                save_state_cpu_state = CameraMove;
            }
        }

        // move to camera bounds
        if *save_state == CameraMove {
            *save_state = PosMove;

            let left_right =
                (*save_state_x > 0.0) as i32 as f32 - (*save_state_x < 0.0) as i32 as f32;
            let mut y_pos = 0.0;
            if *save_state_situation_kind == SITUATION_KIND_GROUND {
                y_pos = -50.0;
            }

            let pos = Vector3f {
                x: left_right * 50.0,
                y: y_pos,
                z: 0.0,
            };
            PostureModule::set_pos(module_accessor, &pos);
            StatusModule::set_situation_kind(
                module_accessor,
                app::SituationKind {
                    situation_kind: *SITUATION_KIND_AIR,
                },
                false,
            );
        }

        // move to correct pos
        if *save_state == PosMove {
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

            // Doesn't work, and I don't know why yet.
            // if *save_state_situation_kind == SITUATION_KIND_GROUND && status != FIGHTER_STATUS_KIND_WAIT {
            //     StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            // }
            // else if (*save_state_situation_kind == SITUATION_KIND_AIR && status != FIGHTER_STATUS_KIND_FALL)
            //         StatusModule::change_status_request(module_accessor, FIGHTER_STATUS_KIND_FALL, 0);
            if *save_state_situation_kind == SITUATION_KIND_CLIFF
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
        }

        // Grab + Dpad down: Save state
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH) != 0
            && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
                != 0
        {
            save_state_player_state = Save;
            save_state_cpu_state = Save;
        }

        if *save_state == Save {
            *save_state = NoAction;

            *save_state_x = PostureModule::pos_x(module_accessor);
            *save_state_y = PostureModule::pos_y(module_accessor);
            *save_state_lr = PostureModule::lr(module_accessor);
            *save_state_percent = DamageModule::damage(module_accessor, 0);
            *save_state_situation_kind = StatusModule::situation_kind(module_accessor);
        }
    }
}
