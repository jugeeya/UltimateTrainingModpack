use std::collections::HashMap;

use log::info;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use smash::app::{self, lua_bind::*, ArticleOperationTarget, Item};
use smash::cpp::l2c_value::LuaConst;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};
use std::ptr;
use training_mod_consts::{CharacterItem, SaveDamage, SaveStateSlot};

use SaveState::*;

use crate::common::button_config;
use crate::common::consts::get_random_float;
use crate::common::consts::get_random_int;
use crate::common::consts::FighterId;
use crate::common::consts::OnOff;
use crate::common::consts::PlaybackSlot;
use crate::common::consts::RecordTrigger;
use crate::common::consts::SaveStateMirroring;
//TODO: Cleanup above
use crate::common::consts::SAVE_STATES_TOML_PATH;
use crate::common::get_module_accessor;
use crate::common::is_dead;
use crate::common::MENU;
use crate::is_operation_cpu;
use crate::training::buff;
use crate::training::character_specific::{ptrainer, steve};
use crate::training::charge::{self, ChargeState};
use crate::training::input_record;
use crate::training::items::apply_item;
use crate::training::reset;
use crate::training::ui::notifications;
use crate::{is_ptrainer, ITEM_MANAGER_ADDR};

// Don't remove Mii hats, Pikmin, Luma, or crafting table
const ARTICLE_ALLOWLIST: [(LuaConst, LuaConst); 9] = [
    (
        FIGHTER_KIND_MIIFIGHTER,
        FIGHTER_MIIFIGHTER_GENERATE_ARTICLE_HAT,
    ),
    (
        FIGHTER_KIND_MIISWORDSMAN,
        FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_HAT,
    ),
    (
        FIGHTER_KIND_MIIGUNNER,
        FIGHTER_MIIGUNNER_GENERATE_ARTICLE_HAT,
    ),
    (FIGHTER_KIND_ROSETTA, FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO),
    (FIGHTER_KIND_PICKEL, FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE),
    (FIGHTER_KIND_ELIGHT, FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD),
    (FIGHTER_KIND_EFLAME, FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD),
    (FIGHTER_KIND_PIKMIN, FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN),
    (
        FIGHTER_KIND_DUCKHUNT,
        FIGHTER_DUCKHUNT_GENERATE_ARTICLE_RETICLE,
    ),
];

extern "C" {
    #[link_name = "\u{1}_ZN3app14sv_information8stage_idEv"]
    pub fn stage_id() -> i32;
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
pub enum SaveState {
    Save,
    NoAction,
    KillPlayer,
    WaitForAlive,
    PosMove,
    ApplyBuff,
    NanaPosMove,
    WaitForPokemonSwitch,
    WaitForCopyAbility,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct SavedState {
    pub x: f32,
    pub y: f32,
    pub percent: f32,
    pub lr: f32,
    pub situation_kind: i32,
    pub state: SaveState,
    pub fighter_kind: i32,
    pub charge: ChargeState,
    pub steve_state: Option<steve::SteveState>,
}

#[macro_export]
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
                int_z: None,
                float_x: None,
                float_y: None,
                float_z: None,
                has_charge: None,
                robin_charges: None,
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

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct SaveStateSlots {
    player: [SavedState; NUM_SAVE_STATE_SLOTS],
    cpu: [SavedState; NUM_SAVE_STATE_SLOTS],
}

const NUM_SAVE_STATE_SLOTS: usize = 5;
// I actually had to do it this way, a simple load-from-file in main() caused crashes.
lazy_static::lazy_static! {
    static ref SAVE_STATE_SLOTS : Mutex<SaveStateSlots> = Mutex::new({
        info!("Initialized lazy_static: SAVE_STATE_SLOTS");
        load_from_file()
    });
}

pub fn load_from_file() -> SaveStateSlots {
    let defaults = SaveStateSlots {
        player: [default_save_state!(); NUM_SAVE_STATE_SLOTS],
        cpu: [default_save_state!(); NUM_SAVE_STATE_SLOTS],
    };

    info!("Checking for previous save state settings in {SAVE_STATES_TOML_PATH}...");
    if std::fs::metadata(SAVE_STATES_TOML_PATH).is_err() {
        return defaults;
    }

    info!("Previous save state settings found. Loading...");
    if let Ok(data) = std::fs::read_to_string(SAVE_STATES_TOML_PATH) {
        let input_slots = toml::from_str::<SaveStateSlots>(&data);
        if let Ok(input_slots) = input_slots {
            return input_slots;
        }
    }

    defaults
}

pub unsafe fn save_to_file() {
    let save_states_str = toml::to_string_pretty(&*SAVE_STATE_SLOTS.data_ptr())
        .expect("Error serializing save state information");
    std::fs::write(SAVE_STATES_TOML_PATH, save_states_str)
        .expect("Could not write save state information to file");
}

unsafe fn save_state_player(slot: usize) -> &'static mut SavedState {
    &mut (*SAVE_STATE_SLOTS.data_ptr()).player[slot]
}

unsafe fn save_state_cpu(slot: usize) -> &'static mut SavedState {
    &mut (*SAVE_STATE_SLOTS.data_ptr()).cpu[slot]
}

pub unsafe fn get_state_pokemon(
    ptrainer_module_accessor: *mut app::BattleObjectModuleAccessor,
) -> u32 {
    let selected_slot = get_slot();
    let pokemon_module_accessor = ptrainer::get_pokemon_module_accessor(ptrainer_module_accessor);
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    let fighter_kind = if !ptr::eq(pokemon_module_accessor, cpu_module_accessor) {
        save_state_player(selected_slot).fighter_kind
    } else {
        save_state_cpu(selected_slot).fighter_kind
    };
    (fighter_kind - *FIGHTER_KIND_PZENIGAME) as u32
}

pub unsafe fn get_charge_state(
    module_accessor: *mut app::BattleObjectModuleAccessor,
) -> ChargeState {
    let selected_slot = get_slot();
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    if !ptr::eq(module_accessor, cpu_module_accessor) {
        save_state_player(selected_slot).charge
    } else {
        save_state_cpu(selected_slot).charge
    }
}

pub unsafe fn end_copy_ability(module_accessor: *mut app::BattleObjectModuleAccessor) {
    let selected_slot = get_slot();
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    let save_state = if !ptr::eq(module_accessor, cpu_module_accessor) {
        save_state_player(selected_slot)
    } else {
        save_state_cpu(selected_slot)
    };
    if save_state.state == WaitForCopyAbility {
        save_state.state = NoAction;
    }
}

// MIRROR_STATE == 1 -> Do not mirror
// MIRROR_STATE == -1 -> Do Mirror
static mut MIRROR_STATE: f32 = 1.0;

static mut RANDOM_SLOT: usize = 0;

unsafe fn get_slot() -> usize {
    let random_slot = MENU.randomize_slots.get_random();
    if random_slot != SaveStateSlot::empty() {
        RANDOM_SLOT
    } else {
        MENU.save_state_slot.into_idx().unwrap_or(0)
    }
}

pub unsafe fn is_killing() -> bool {
    let selected_slot = get_slot();
    (save_state_player(selected_slot).state == KillPlayer
        || save_state_player(selected_slot).state == WaitForAlive)
        || (save_state_cpu(selected_slot).state == KillPlayer
            || save_state_cpu(selected_slot).state == WaitForAlive)
}

pub unsafe fn is_loading() -> bool {
    let selected_slot = get_slot();
    save_state_player(selected_slot).state != NoAction
        || save_state_cpu(selected_slot).state != NoAction
}

pub unsafe fn should_mirror() -> f32 {
    match MENU.save_state_mirroring {
        SaveStateMirroring::NONE => 1.0,
        SaveStateMirroring::ALTERNATE => -1.0 * MIRROR_STATE,
        SaveStateMirroring::RANDOM => ([-1.0, 1.0])[get_random_int(2) as usize],
        _ => panic!(
            "Invalid value in should_mirror: {}",
            MENU.save_state_mirroring
        ),
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
            // Remove Shulk Monado Art Damage Effects
            WorkModule::set_int(
                module_accessor,
                0,
                *FIGHTER_INSTANCE_WORK_ID_INT_SHULK_MONAD_ARTS_DAMAGE_FLASH_FRAME,
            );
            EffectModule::remove_common(module_accessor, Hash40::new("monad_arts_damage_buster"));
            EffectModule::remove_common(module_accessor, Hash40::new("monad_arts_damage_smash"));
            // TODO: We should try to remove all effects here
            return Some(1);
        }
        if param_hash == hash40("rebirth_move_frame") {
            return Some(0);
        }
        if param_hash == hash40("rebirth_move_frame_trainer") && is_loading() {
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
    if param_type == hash40("param_mball")
        && param_hash == hash40("change_fly_frame")
        && is_loading()
    {
        return Some(0);
    }

    None
}

fn get_stage_offset(stage_id: i32) -> f32 {
    let offsets: HashMap<i32, f32> = HashMap::from([
        (*StageID::Animal_Village, 1.195),
        (*StageID::Animal_City, 1.448),
        (*StageID::Yoshi_Island, -1.053),
    ]);

    *offsets.get(&stage_id).unwrap_or(&0.0)
}

fn set_damage(module_accessor: &mut app::BattleObjectModuleAccessor, damage: f32) {
    unsafe {
        DamageModule::heal(
            module_accessor,
            -1.0 * DamageModule::damage(module_accessor, 0),
            0,
        );
        DamageModule::add_damage(module_accessor, damage, 0);
    }
}

unsafe fn on_ptrainer_death(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_ptrainer(module_accessor) {
        return;
    }
    let ptrainer_module_accessor = ptrainer::get_ptrainer_module_accessor(module_accessor);
    WorkModule::off_flag(
        ptrainer_module_accessor,
        *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_FLAG_ENABLE_CHANGE_POKEMON,
    );
    MotionModule::set_rate(ptrainer_module_accessor, 1000.0);
    if let Some(ptrainer_masterball_module_accessor) =
        ptrainer::get_ptrainer_mball_module_accessor(ptrainer_module_accessor)
    {
        MotionModule::set_rate(ptrainer_masterball_module_accessor, 1000.0);
        ArticleModule::set_visibility_whole(
            ptrainer::get_ptrainer_module_accessor(module_accessor),
            *WEAPON_PTRAINER_PTRAINER_GENERATE_ARTICLE_MBALL,
            false,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
    }
}

pub unsafe fn on_death(fighter_kind: i32, module_accessor: &mut app::BattleObjectModuleAccessor) {
    SoundModule::stop_all_sound(module_accessor);
    // All articles have ID <= 0x25
    (0..=0x25)
        .filter(|article_idx| {
            !ARTICLE_ALLOWLIST.iter().any(|article_allowed| {
                article_allowed.0 == fighter_kind && article_allowed.1 == *article_idx
            })
        })
        .for_each(|article_idx| {
            if ArticleModule::is_exist(module_accessor, article_idx) {
                let article: *mut app::Article =
                    ArticleModule::get_article(module_accessor, article_idx);
                let article_object_id = Article::get_battle_object_id(article);
                ArticleModule::remove_exist_object_id(module_accessor, article_object_id as u32);
            }
        });
    let item_mgr = *(ITEM_MANAGER_ADDR as *mut *mut app::ItemManager);
    (0..ItemManager::get_num_of_active_item_all(item_mgr)).for_each(|item_idx| {
        let item = ItemManager::get_active_item(item_mgr, item_idx);
        if item != 0 {
            let item = item as *mut Item;
            let item_battle_object_id = app::lua_bind::Item::get_battle_object_id(item) as u32;
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
}

pub unsafe fn save_states(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if MENU.save_state_enable == OnOff::OFF {
        return;
    }

    let selected_slot = get_slot();
    let status = StatusModule::status_kind(module_accessor);
    let is_cpu = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID)
        == FighterId::CPU as i32;
    let save_state = if is_cpu {
        save_state_cpu(selected_slot)
    } else {
        save_state_player(selected_slot)
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
        *FIGHTER_KIND_SHULK,
        *FIGHTER_KIND_TANTAN,
        *FIGHTER_KIND_WARIO,
    ]
    .contains(&fighter_kind);

    // Reset state
    let autoload_reset = MENU.save_state_autoload == OnOff::ON
        && save_state.state == NoAction
        && is_dead(module_accessor);
    let mut triggered_reset: bool = false;
    if !is_operation_cpu(module_accessor) && !fighter_is_nana {
        triggered_reset = button_config::combo_passes(button_config::ButtonCombo::LoadState);
    }
    if (autoload_reset || triggered_reset) && !fighter_is_nana {
        if save_state.state == NoAction {
            let random_slot = MENU.randomize_slots.get_random();
            let slot = if random_slot != SaveStateSlot::empty() {
                RANDOM_SLOT = random_slot.into_idx().unwrap_or(0);
                RANDOM_SLOT
            } else {
                selected_slot
            };

            save_state_player(slot).state = KillPlayer;
            save_state_cpu(slot).state = KillPlayer;
        }
        MIRROR_STATE = should_mirror();
        // end input recording playback
        input_record::stop_playback();
        return;
    }

    // Kill the fighter and move them to camera bounds
    // Note: Nana shouldn't control her state here. Popo will give a signal to have
    // Nana move into NanaPosMove once he moves.
    if save_state.state == KillPlayer && !fighter_is_nana {
        on_ptrainer_death(module_accessor);
        if !is_dead(module_accessor) {
            StatusModule::change_status_force(module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
        }

        save_state.state = WaitForAlive;

        return;
    }

    if save_state.state == WaitForAlive {
        on_ptrainer_death(module_accessor);
        if !is_dead(module_accessor) && !fighter_is_nana {
            on_death(fighter_kind, module_accessor);
            save_state.state = PosMove;
        }
    }

    // move to correct pos
    if save_state.state == PosMove || save_state.state == NanaPosMove {
        let status_kind = StatusModule::status_kind(module_accessor);
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

        let mut pos = if MIRROR_STATE == -1.0 {
            Vector3f {
                x: MIRROR_STATE * (save_state.x - get_stage_offset(stage_id())),
                y: save_state.y,
                z: 0.0,
            }
        } else {
            Vector3f {
                x: save_state.x,
                y: save_state.y,
                z: 0.0,
            }
        };

        // Adjust fighter y position if they won't grab ledge when they should
        adjust_ledge_pos(&mut pos, save_state.fighter_kind, save_state.situation_kind);
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
            if status != FIGHTER_STATUS_KIND_FALL
                && status != FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE
                && status != FIGHTER_STATUS_KIND_CLIFF_CATCH
            {
                StatusModule::change_status_request(
                    module_accessor,
                    *FIGHTER_STATUS_KIND_FALL,
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
            // Set damage of the save state
            if !is_cpu {
                match MENU.save_damage_player {
                    SaveDamage::SAVED => {
                        set_damage(module_accessor, save_state.percent);
                    }
                    SaveDamage::RANDOM => {
                        // Gen random value
                        let pct: f32 = get_random_float(
                            MENU.save_damage_limits_player.0 as f32,
                            MENU.save_damage_limits_player.1 as f32,
                        );
                        set_damage(module_accessor, pct);
                    }
                    SaveDamage::DEFAULT => {}
                    _ => panic!(
                        "Invalid value in save_states()::save_damage_player: {}",
                        MENU.save_damage_player
                    ),
                }
            } else {
                match MENU.save_damage_cpu {
                    SaveDamage::SAVED => {
                        set_damage(module_accessor, save_state.percent);
                    }
                    SaveDamage::RANDOM => {
                        // Gen random value
                        let pct: f32 = get_random_float(
                            MENU.save_damage_limits_cpu.0 as f32,
                            MENU.save_damage_limits_cpu.1 as f32,
                        );
                        set_damage(module_accessor, pct);
                    }
                    SaveDamage::DEFAULT => {}
                    _ => panic!(
                        "Invalid value in save_states()::save_damage_cpu: {}",
                        MENU.save_damage_cpu
                    ),
                }
            }

            // Set to held item
            if !is_cpu && !fighter_is_nana && MENU.character_item != CharacterItem::NONE {
                apply_item(MENU.character_item);
            }

            // Set the charge of special moves if the fighter matches the kind in the save state
            if save_state.fighter_kind == fighter_kind {
                charge::handle_charge(module_accessor, fighter_kind, save_state.charge);
                // If we ever want to buff kirby, we have to move this to after apply buff
                if fighter_kind == FIGHTER_KIND_KIRBY {
                    save_state.state = WaitForCopyAbility;
                }
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
                    ptrainer::get_ptrainer_module_accessor(module_accessor),
                    *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_FLAG_ENABLE_CHANGE_POKEMON,
                );
                save_state.state = WaitForPokemonSwitch;
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
        if MENU.record_trigger.contains(&RecordTrigger::SAVESTATE) {
            input_record::lockout_record();
            return;
        }
        // otherwise, begin input recording playback if selected
        // for ledge, don't do this - if you want playback on a ledge, you have to set it as a ledge option,
        // otherwise there too many edge cases here
        else if MENU.save_state_playback.get_random() != PlaybackSlot::empty()
            && save_state.situation_kind != SITUATION_KIND_CLIFF
        {
            input_record::playback(MENU.save_state_playback.get_random().into_idx());
        }

        return;
    }

    if save_state.state == ApplyBuff {
        // needs its own save_state.state since this may take multiple frames, want it to loop
        if buff::handle_buffs(module_accessor, fighter_kind, status) {
            // returns true when done buffing fighter
            buff::restart_buff(module_accessor);
            // set is_buffing back to false when done
            save_state.state = NoAction;
        }
    }

    // If we switched last frame, artifacts and sound should be cleaned up, so transition into NoAction
    if save_state.state == WaitForPokemonSwitch
        && ptrainer::is_switched(ptrainer::get_ptrainer_module_accessor(module_accessor))
    {
        save_state.state = NoAction;
    }

    // Wait for Copy Ability to be applied if needed, exit otherwise
    if save_state.state == WaitForCopyAbility {
        let is_copy_start = WorkModule::is_flag(module_accessor, 0x20000104); // *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY_ON_START
        if !is_copy_start {
            save_state.state = NoAction;
        }
    }

    // Save state
    if button_config::combo_passes(button_config::ButtonCombo::SaveState) {
        // Don't begin saving state if Nana's delayed input is captured
        MIRROR_STATE = 1.0;
        save_state_player(MENU.save_state_slot.into_idx().unwrap_or(0)).state = Save;
        save_state_cpu(MENU.save_state_slot.into_idx().unwrap_or(0)).state = Save;
        notifications::clear_notifications("Save State");
        notifications::notification(
            "Save State".to_string(),
            format!("Saved Slot {}", MENU.save_state_slot),
            120,
        );
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

        // If both chars finished saving by now
        if save_state_player(selected_slot).state != Save
            && save_state_cpu(selected_slot).state != Save
        {
            save_to_file();
        }
    }
}

fn adjust_ledge_pos(pos: &mut Vector3f, fighter_kind: i32, situation_kind: i32) {
    // Adjusts save states for fighters who have grabbed the ledge
    if situation_kind != SITUATION_KIND_CLIFF {
        return;
    }

    let fighter_needs_ledge_adjust_far = [
        *FIGHTER_KIND_DONKEY,
        *FIGHTER_KIND_DEDEDE,
        *FIGHTER_KIND_KROOL,
    ]
    .contains(&fighter_kind);

    let fighter_needs_ledge_adjust_medium = [
        *FIGHTER_KIND_SAMUS,
        *FIGHTER_KIND_SAMUSD,
        *FIGHTER_KIND_MEWTWO,
        *FIGHTER_KIND_DOLLY,
    ]
    .contains(&fighter_kind);

    let fighter_needs_ledge_adjust_slight = [
        *FIGHTER_KIND_KOOPA,
        *FIGHTER_KIND_ELIGHT,
        *FIGHTER_KIND_SNAKE,
    ]
    .contains(&fighter_kind);

    if fighter_needs_ledge_adjust_far {
        pos.y += 6.0;
    } else if fighter_needs_ledge_adjust_medium {
        pos.y += 4.0;
    }
    if fighter_needs_ledge_adjust_slight {
        pos.y += 2.0;
    }
}
