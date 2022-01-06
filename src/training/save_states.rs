use crate::common::consts::FighterId;
use crate::common::consts::OnOff;
use crate::common::consts::SaveStateMirroring;
use crate::common::MENU;
use crate::common::{get_random_int, is_dead};
use crate::training::reset;
use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};

#[derive(PartialEq)]
enum SaveState {
    Save,
    NoAction,
    KillPlayer,
    PosMove,
    NanaPosMove,
}

struct SavedState {
    x: f32,
    y: f32,
    percent: f32,
    lr: f32,
    situation_kind: i32,
    state: SaveState,
    fighter_kind: i32,
}

macro_rules! default_save_state {
    () => {
        SavedState {
            x: 0.0,
            y: 0.0,
            percent: 0.0,
            lr: 1.0,
            situation_kind: 0,
            state: NoAction,
            fighter_kind: -1,
        }
    };
}

use SaveState::*;

static mut SAVE_STATE_PLAYER: SavedState = default_save_state!();
static mut SAVE_STATE_CPU: SavedState = default_save_state!();
static mut MIRROR_STATE: f32 = 1.0;
// MIRROR_STATE == 1 -> Do not mirror
// MIRROR_STATE == -1 -> Do Mirror

pub unsafe fn is_killing() -> bool {
    if SAVE_STATE_PLAYER.state == KillPlayer || SAVE_STATE_CPU.state == KillPlayer {
        return true;
    }
    return false;
}

pub unsafe fn should_mirror() -> f32 {
    match MENU.save_state_mirroring {
        SaveStateMirroring::None => 1.0,
        SaveStateMirroring::Alternate => -1.0 * MIRROR_STATE,
        SaveStateMirroring::Random => ([-1.0, 1.0])[get_random_int(2) as usize],
    }
}

pub unsafe fn get_param_int(
    _module_accessor: &mut app::BattleObjectModuleAccessor,
    param_type: u64,
    param_hash: u64,
) -> Option<i32> {
    if param_type == hash40("common") {
        if param_hash == hash40("dead_rebirth_wait_frame") {
            return Some(1);
        }
        if param_hash == hash40("rebirth_move_frame") {
            return Some(0);
        }
        if param_hash == hash40("rebirth_wait_frame") {
            return Some(0);
        }
        if param_hash == hash40("rebirth_invincible_frame") {
            return Some(0);
        }
        if param_hash == hash40("rebirth_invincible_add_frame") {
            return Some(0);
        }
    }

    None
}

fn set_damage(module_accessor: &mut app::BattleObjectModuleAccessor, damage: f32) {
    let overwrite_damage;

    unsafe {
        overwrite_damage = MENU.save_damage == OnOff::On;
    }

    if !overwrite_damage {
        return;
    }

    unsafe {
        DamageModule::heal(
            module_accessor,
            -1.0 * DamageModule::damage(module_accessor, 0),
            0,
        );
        DamageModule::add_damage(module_accessor, damage, 0);
    }
}

pub unsafe fn save_states(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if MENU.save_state_enable == OnOff::Off {
        return;
    }

    let status = StatusModule::status_kind(module_accessor) as i32;
    let is_cpu = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID)
        == FighterId::CPU as i32;
    let save_state = if is_cpu {
        &mut SAVE_STATE_CPU
    } else {
        &mut SAVE_STATE_PLAYER
    };

    let fighter_kind = app::utility::get_kind(module_accessor);
    let fighter_is_ptrainer = [
        *FIGHTER_KIND_PZENIGAME,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_PLIZARDON,
    ]
    .contains(&fighter_kind);
    let fighter_is_popo = fighter_kind == *FIGHTER_KIND_POPO; // For making sure Popo doesn't steal Nana's PosMove
    let fighter_is_nana = fighter_kind == *FIGHTER_KIND_NANA; // Don't want Nana to reopen savestates etc.

    // Grab + Dpad up: reset state
    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
        && !fighter_is_nana
    {
        if save_state.state == NoAction {
            SAVE_STATE_PLAYER.state = KillPlayer;
            SAVE_STATE_CPU.state = KillPlayer;
        }
        MIRROR_STATE = should_mirror();
        reset::on_reset();
        return;
    }

    // move to camera bounds
    if save_state.state == KillPlayer {
        SoundModule::stop_all_sound(module_accessor);
        if status == FIGHTER_STATUS_KIND_REBIRTH {
            if !(fighter_is_ptrainer
                && save_state.fighter_kind > 0
                && fighter_kind != save_state.fighter_kind)
            {
                // For ptrainer, don't move on unless we're cycled back to the right pokemon
                save_state.state = PosMove;
            }
        } else if !is_dead(module_accessor) && !fighter_is_nana {
            // Don't kill Nana again, since she already gets killed by the game from Popo's death
            let jostle_frame = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_IGNORE_JOSTLE_FRAME);
            if jostle_frame > 1 {
                // Allow jostle to stop being ignored before we die
                WorkModule::set_int(module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_IGNORE_JOSTLE_FRAME);
            }
            // Try moving off-screen so we don't see effects.
            let pos = Vector3f {
                x: -300.0,
                y: -100.0,
                z: 0.0,
            };
            PostureModule::set_pos(module_accessor, &pos);

            MotionAnimcmdModule::set_sleep(module_accessor, true);
            SoundModule::pause_se_all(module_accessor, true);
            ControlModule::stop_rumble(module_accessor, true);
            SoundModule::stop_all_sound(module_accessor);

            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_DEAD, false);
        }

        return;
    }

    // move to correct pos
    if save_state.state == PosMove || save_state.state == NanaPosMove {
        let status_kind = StatusModule::status_kind(module_accessor) as i32;
        if save_state.state == NanaPosMove
            && (!fighter_is_nana || (status_kind == FIGHTER_STATUS_KIND_STANDBY))
        {
            return;
        }
        SoundModule::stop_all_sound(module_accessor);
        MotionAnimcmdModule::set_sleep(module_accessor, false);
        SoundModule::pause_se_all(module_accessor, false);
        ControlModule::stop_rumble(module_accessor, false);
        KineticModule::clear_speed_all(module_accessor);

        let pos = Vector3f {
            x: MIRROR_STATE * save_state.x,
            y: save_state.y,
            z: 0.0,
        };
        let lr = MIRROR_STATE * save_state.lr;
        PostureModule::set_pos(module_accessor, &pos);
        PostureModule::set_lr(module_accessor, lr);

        if save_state.situation_kind == SITUATION_KIND_GROUND {
            if status != FIGHTER_STATUS_KIND_WAIT {
                StatusModule::change_status_request(
                    module_accessor,
                    *FIGHTER_STATUS_KIND_WAIT,
                    false,
                );
            } else {
                save_state.state = NoAction;
            }
        } else if save_state.situation_kind == SITUATION_KIND_AIR {
            if status != FIGHTER_STATUS_KIND_FALL {
                StatusModule::change_status_request(
                    module_accessor,
                    *FIGHTER_STATUS_KIND_FALL,
                    false,
                );
            } else {
                save_state.state = NoAction;
            }
        } else if save_state.situation_kind == SITUATION_KIND_CLIFF {
            if status != FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE
                && status != FIGHTER_STATUS_KIND_CLIFF_CATCH
            {
                StatusModule::change_status_request(
                    module_accessor,
                    *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
                    false,
                );
            } else {
                save_state.state = NoAction;
            }
        } else {
            save_state.state = NoAction;
        }

        // if we're done moving, reset percent
        if save_state.state == NoAction {
            set_damage(module_accessor, save_state.percent);
        }

        // if the fighter is Popo, change the state to one where only Nana can move
        // This is needed because for some reason, outside of frame by frame mode,
        // Popo will keep trying to move instead of letting Nana move if you just
        // change the state back to PosMove
        let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
        if prev_status_kind == FIGHTER_STATUS_KIND_REBIRTH && fighter_is_popo {
            save_state.state = NanaPosMove;
        }

        return;
    }

    // Grab + Dpad down: Save state
    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
        && !fighter_is_nana
    // Don't begin saving state if Nana's delayed input is captured
    {
        MIRROR_STATE = 1.0;
        SAVE_STATE_PLAYER.state = Save;
        SAVE_STATE_CPU.state = Save;
    }

    if save_state.state == Save && !fighter_is_nana {
        // Don't save states with Nana. Should already be fine, just a safety.
        save_state.state = NoAction;

        save_state.x = PostureModule::pos_x(module_accessor);
        save_state.y = PostureModule::pos_y(module_accessor);
        save_state.lr = PostureModule::lr(module_accessor);
        save_state.percent = DamageModule::damage(module_accessor, 0);
        save_state.situation_kind = StatusModule::situation_kind(module_accessor);
        if fighter_is_ptrainer {
            // Only store the fighter_kind for pokemon trainer
            save_state.fighter_kind = app::utility::get_kind(module_accessor);
        } else {
            save_state.fighter_kind = -1;
        }

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
