use crate::common::button_config;
use crate::common::consts::get_random_int;
use crate::common::consts::FighterId;
use crate::common::consts::OnOff;
use crate::common::consts::SaveStateMirroring;
use crate::common::consts::RecordTrigger;
//TODO: Cleanup above
use crate::common::is_dead;
use crate::common::MENU;
use crate::is_operation_cpu;
use crate::training::input_record;
use crate::training::buff;
use crate::training::character_specific::steve;
use crate::training::charge::{self, ChargeState};
use crate::training::items::apply_item;
use crate::training::reset;
use smash::app::{self, lua_bind::*, Item};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};
use training_mod_consts::CharacterItem;

#[derive(PartialEq)]
enum SaveState {
    Save,
    NoAction,
    KillPlayer,
    PosMove,
    NanaPosMove,
    ApplyBuff,
}

struct SavedState {
    x: f32,
    y: f32,
    percent: f32,
    lr: f32,
    situation_kind: i32,
    state: SaveState,
    fighter_kind: i32,
    charge: ChargeState,
    steve_state: Option<steve::SteveState>,
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
            charge: ChargeState {
                int_x: None,
                int_y: None,
                float_x: None,
                float_y: None,
                float_z: None,
                has_charge: None,
            },
            steve_state: Some(steve::SteveState {
                mat_g1: 36,
                mat_wood: 18,
                mat_stone: 0,
                mat_iron: 3,
                mat_gold: 0,
                mat_redstone: 2,
                mat_diamond: 0,
                sword_mat: 1 as char,
                sword_durability: 25.0,
                axe_mat: 1 as char,
                axe_durability: 70.0,
                pick_mat: 1 as char,
                pick_durability: 70.0,
                shovel_mat: 1 as char,
                shovel_durability: 70.0,
            }),
        }
    };
}

use crate::{is_ptrainer, ITEM_MANAGER_ADDR};
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
    false
}

pub unsafe fn should_mirror() -> f32 {
    match MENU.save_state_mirroring {
        SaveStateMirroring::None => 1.0,
        SaveStateMirroring::Alternate => -1.0 * MIRROR_STATE,
        SaveStateMirroring::Random => ([-1.0, 1.0])[get_random_int(2) as usize],
    }
}

pub unsafe fn get_param_int(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    param_type: u64,
    param_hash: u64,
) -> Option<i32> {
    if param_type == hash40("common") {
        if param_hash == hash40("dead_rebirth_wait_frame") {
            let jostle_frame = WorkModule::get_int(
                module_accessor,
                *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_IGNORE_JOSTLE_FRAME,
            );
            if jostle_frame > 1 {
                // Allow jostle to stop being ignored before we respawn
                WorkModule::set_int(
                    module_accessor,
                    1,
                    *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_IGNORE_JOSTLE_FRAME,
                );
            }
            return Some(1);
        }
        if param_hash == hash40("rebirth_move_frame") {
            return Some(0);
        }
        if param_hash == hash40("rebirth_move_frame_trainer") {
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
    if param_type == hash40("param_mball") {
        if param_hash == hash40("change_fly_frame") {
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

unsafe fn get_ptrainer_module_accessor(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> &mut app::BattleObjectModuleAccessor {
    let ptrainer_object_id =
        LinkModule::get_parent_object_id(module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER);
    &mut *app::sv_battle_object::module_accessor(ptrainer_object_id as u32)
}

unsafe fn on_ptrainer_death(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_ptrainer(module_accessor) {
        return;
    }
    WorkModule::off_flag(
        get_ptrainer_module_accessor(module_accessor),
        *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_FLAG_ENABLE_CHANGE_POKEMON,
    );
    let ptrainer_module_accessor = get_ptrainer_module_accessor(module_accessor);
    MotionModule::set_rate(ptrainer_module_accessor, 1000.0);
    if ArticleModule::is_exist(
        ptrainer_module_accessor,
        *WEAPON_PTRAINER_PTRAINER_GENERATE_ARTICLE_MBALL,
    ) {
        let ptrainer_masterball: u64 = ArticleModule::get_article(
            ptrainer_module_accessor,
            *WEAPON_PTRAINER_PTRAINER_GENERATE_ARTICLE_MBALL,
        );
        let ptrainer_masterball_id =
            Article::get_battle_object_id(ptrainer_masterball as *mut app::Article);
        let ptrainer_masterball_module_accessor =
            &mut *app::sv_battle_object::module_accessor(ptrainer_masterball_id as u32);
        MotionModule::set_rate(ptrainer_masterball_module_accessor, 1000.0);
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
    let fighter_is_ptrainer = is_ptrainer(module_accessor);
    let fighter_is_popo = fighter_kind == *FIGHTER_KIND_POPO; // For making sure Popo doesn't steal Nana's PosMove
    let fighter_is_nana = fighter_kind == *FIGHTER_KIND_NANA; // Don't want Nana to reopen save states etc.
    let fighter_is_buffable = [
        *FIGHTER_KIND_BRAVE,
        *FIGHTER_KIND_CLOUD,
        *FIGHTER_KIND_JACK,
        *FIGHTER_KIND_LITTLEMAC,
        *FIGHTER_KIND_EDGE,
        *FIGHTER_KIND_WIIFIT,
    ]
    .contains(&fighter_kind);

    // Grab + Dpad up: reset state
    let autoload_reset = MENU.save_state_autoload == OnOff::On
        && save_state.state == NoAction
        && is_dead(module_accessor);
    let mut triggered_reset: bool = false;
    if !is_operation_cpu(module_accessor) {
        triggered_reset =
            button_config::combo_passes(module_accessor, button_config::ButtonCombo::LoadState);
    }
    if (autoload_reset || triggered_reset) && !fighter_is_nana {
        if save_state.state == NoAction {
            SAVE_STATE_PLAYER.state = KillPlayer;
            SAVE_STATE_CPU.state = KillPlayer;
        }
        MIRROR_STATE = should_mirror();
        // end input recording playback
        input_record::stop_playback();
        return;
    }

    // move to camera bounds
    if save_state.state == KillPlayer {
        on_ptrainer_death(module_accessor);
        SoundModule::stop_all_sound(module_accessor);
        if status == FIGHTER_STATUS_KIND_REBIRTH {
            save_state.state = PosMove;
        } else if !is_dead(module_accessor) && !fighter_is_nana {
            // Don't kill Nana again, since she already gets killed by the game from Popo's death
            // Try moving off-screen so we don't see effects.
            let pos = Vector3f {
                x: -300.0,
                y: -100.0,
                z: 0.0,
            };
            PostureModule::set_pos(module_accessor, &pos);

            let item_mgr = *(ITEM_MANAGER_ADDR as *mut *mut app::ItemManager);
            (0..ItemManager::get_num_of_active_item_all(item_mgr)).for_each(|item_idx| {
                let item = ItemManager::get_active_item(item_mgr, item_idx);
                if item != 0 {
                    let item = item as *mut Item;
                    let item_battle_object_id =
                        app::lua_bind::Item::get_battle_object_id(item) as u32;
                    ItemManager::remove_item_from_id(item_mgr, item_battle_object_id);
                }
            });
            MotionAnimcmdModule::set_sleep(module_accessor, true);
            SoundModule::pause_se_all(module_accessor, true);
            ControlModule::stop_rumble(module_accessor, true);
            SoundModule::stop_all_sound(module_accessor);
            // Return camera to normal when loading save state
            SlowModule::clear_whole(module_accessor);
            CameraModule::zoom_out(module_accessor, 0);
            // Remove blue effect (but does not remove darkened screen)
            EffectModule::kill_kind(
                module_accessor,
                Hash40::new("sys_bg_criticalhit"),
                false,
                false,
            );
            // Removes the darkened screen from special zooms
            // If there's a crit that doesn't get removed, it's likely bg_criticalhit2.
            EffectModule::remove_screen(module_accessor, Hash40::new("bg_criticalhit"), 0);
            // Remove all quakes to prevent screen shake lingering through load.
            for quake_kind in *CAMERA_QUAKE_KIND_NONE..=*CAMERA_QUAKE_KIND_MAX {
                CameraModule::stop_quake(module_accessor, quake_kind);
            }

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
        reset::on_reset();

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

        // If we're done moving, reset percent, handle charges, and apply buffs
        if save_state.state == NoAction {
            set_damage(module_accessor, save_state.percent);
            // Set to held item
            if !is_cpu && !fighter_is_nana && MENU.character_item != CharacterItem::None {
                apply_item(MENU.character_item);
            }

            // Set the charge of special moves if the fighter matches the kind in the save state
            if save_state.fighter_kind == fighter_kind {
                charge::handle_charge(module_accessor, fighter_kind, save_state.charge);
            }
            // Buff the fighter if they're one of the fighters who can be buffed
            if fighter_is_buffable {
                save_state.state = ApplyBuff;
            }
            // Perform fighter specific loading actions
            save_state.steve_state.map(|load_steve| {
                steve::load_steve_state(module_accessor, load_steve);
            });
            // Play Training Reset SFX, since silence is eerie
            // Only play for the CPU so we don't have 2 overlapping
            if is_cpu {
                SoundModule::play_se_no3d(
                    module_accessor,
                    Hash40::new("se_system_position_reset"),
                    true,
                    true,
                );
            }
            if fighter_is_ptrainer {
                WorkModule::on_flag(
                    get_ptrainer_module_accessor(module_accessor),
                    *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_FLAG_ENABLE_CHANGE_POKEMON,
                );
            }
        }

        // if the fighter is Popo, change the state to one where only Nana can move
        // This is needed because for some reason, outside of frame by frame mode,
        // Popo will keep trying to move instead of letting Nana move if you just
        // change the state back to PosMove
        let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
        if prev_status_kind == FIGHTER_STATUS_KIND_REBIRTH && fighter_is_popo {
            save_state.state = NanaPosMove;
        }

        // if we're recording on state load, record
        if MENU.record_trigger == RecordTrigger::SAVE_STATE {
            input_record::lockout_record();
        }
        // otherwise, begin input recording playback if selected
        else if MENU.save_state_playback == OnOff::On {
            input_record::playback();
        }

        return;
    }

    if save_state.state == ApplyBuff {
        // needs its own save_state.state since this may take multiple frames, want it to loop
        if buff::handle_buffs(module_accessor, fighter_kind, status, save_state.percent) {
            // returns true when done buffing fighter
            buff::restart_buff(module_accessor);
            // set is_buffing back to false when done
            save_state.state = NoAction;
        }
    }

    // Grab + Dpad down: Save state
    if button_config::combo_passes(module_accessor, button_config::ButtonCombo::SaveState) {
        // Don't begin saving state if Nana's delayed input is captured
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
        // Always store fighter kind so that charges are handled properly
        save_state.fighter_kind = app::utility::get_kind(module_accessor);
        save_state.charge = charge::get_charge(module_accessor, fighter_kind);
        // Perform fighter specific saving actions
        save_state.steve_state = steve::save_steve_state(module_accessor);

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
