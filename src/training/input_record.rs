use crate::common::button_config;
use crate::common::consts::{FighterId, HitstunPlayback, OnOff};
use crate::common::input::*;
use crate::common::{get_module_accessor, is_in_hitstun, is_in_shieldstun, MENU};
use crate::training::mash;
use crate::training::ui::notifications::{clear_notifications, color_notification};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use skyline::nn::ui2d::ResColor;
use smash::app::{lua_bind::*, utility, BattleObjectModuleAccessor};
use smash::lib::lua_const::*;
use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
pub enum InputRecordState {
    None,
    Pause,
    Record,
    Playback,
}

#[derive(PartialEq, Debug)]
pub enum PossessionState {
    Player,
    Cpu,
    Lockout,
    Standby,
}

#[derive(PartialEq, Copy, Clone)]
pub enum StartingStatus {
    Aerial, // FIGHTER_STATUS_KIND_ATTACK_AIR TODO: This shouldn't happen without starting input recording in the air - when would we want this?
    // Probably should lock input recordings to either the ground or the air
    Airdodge, // FIGHTER_STATUS_KIND_ESCAPE_AIR, FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
    // Other statuses cannot be used to hitstun cancel via damage_fly_attack_frame/damage_fly_escape_frame
    // Some statuses can leave shield earlier though, so we should check for this
    SpecialHi,  // Up B: FIGHTER_STATUS_KIND_SPECIAL_HI,
    Jump,       // FIGHTER_STATUS_KIND_JUMP_SQUAT
    DoubleJump, //FIGHTER_STATUS_KIND_JUMP_AERIAL
    Spotdodge,  // FIGHTER_STATUS_KIND_ESCAPE
    Roll,       // FIGHTER_STATUS_KIND_ESCAPE_F, FIGHTER_STATUS_KIND_ESCAPE_B
    Grab,       // FIGHTER_STATUS_KIND_CATCH
    Other,
}

use InputRecordState::*;
use PossessionState::*;

const STICK_NEUTRAL: f32 = 0.2;
const STICK_CLAMP_MULTIPLIER: f32 = 1.0 / 120.0; // 120.0 = CLAMP_MAX
const FINAL_RECORD_MAX: usize = 600; // Maximum length for input recording sequences (capacity)
const TOTAL_SLOT_COUNT: usize = 5; // Total number of input recording slots
pub static mut INPUT_RECORD: InputRecordState = InputRecordState::None;
pub static mut INPUT_RECORD_FRAME: usize = 0;
pub static mut POSSESSION: PossessionState = PossessionState::Player;
pub static mut LOCKOUT_FRAME: usize = 0;
pub static mut BUFFER_FRAME: usize = 0;
pub static mut RECORDED_LR: f32 = 1.0; // The direction the CPU was facing before the current recording was recorded
pub static mut CURRENT_LR: f32 = 1.0; // The direction the CPU was facing at the beginning of this playback
pub static mut STARTING_STATUS: i32 = 0; // The first status entered in the recording outside of waits
                                         //     used to calculate if the input playback should begin before hitstun would normally end (hitstun cancel, monado art?)
pub static mut CURRENT_RECORD_SLOT: usize = 0; // Which slot is being used for recording right now? Want to make sure this is synced with menu choices, maybe just use menu instead
pub static mut CURRENT_PLAYBACK_SLOT: usize = 0; // Which slot is being used for playback right now?
pub static mut CURRENT_FRAME_LENGTH: usize = 60;

lazy_static! {
    static ref P1_FINAL_MAPPING: Mutex<[[MappedInputs; FINAL_RECORD_MAX]; TOTAL_SLOT_COUNT]> =
        Mutex::new([[{ MappedInputs::empty() }; FINAL_RECORD_MAX]; TOTAL_SLOT_COUNT]);
    static ref P1_FRAME_LENGTH_MAPPING: Mutex<[usize; TOTAL_SLOT_COUNT]> =
        Mutex::new([60usize; TOTAL_SLOT_COUNT]);
    static ref P1_STARTING_STATUSES: Mutex<[StartingStatus; TOTAL_SLOT_COUNT]> =
        Mutex::new([{ StartingStatus::Other }; TOTAL_SLOT_COUNT]);
}

unsafe fn can_transition(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let transition_term = into_transition_term(into_starting_status(STARTING_STATUS));
    WorkModule::is_enable_transition_term(module_accessor, transition_term)
}

unsafe fn should_mash_playback() {
    // Don't want to interrupt recording
    if is_recording() {
        return;
    }
    if !mash::is_playback_queued() {
        return;
    }
    // playback is queued, so we might want to begin this frame
    // if we're currently playing back, we don't want to interrupt (this may change for layered multislot playback, but for now this is fine)
    if is_playback() {
        return;
    }
    let mut should_playback = false;
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    // depending on our current status, we want to wait for different timings to begin playback

    // TODO: This isn't the best way to write this I'm sure, want to rewrite

    if is_in_hitstun(&mut *cpu_module_accessor) {
        // if we're in hitstun and want to enter the frame we start hitstop for SDI, start if we're in any damage status instantly
        if MENU.hitstun_playback == HitstunPlayback::Instant {
            should_playback = true;
        }
        // if we want to wait until we exit hitstop and begin flying away for shield art etc, start if we're not in hitstop
        if MENU.hitstun_playback == HitstunPlayback::Hitstop
            && !StopModule::is_stop(cpu_module_accessor)
        {
            should_playback = true;
        }
        // if we're in hitstun and want to wait till FAF to act, then we want to match our starting status to the correct transition term to see if we can hitstun cancel
        if MENU.hitstun_playback == HitstunPlayback::Hitstun && can_transition(cpu_module_accessor)
        {
            should_playback = true;
        }
    } else if is_in_shieldstun(&mut *cpu_module_accessor) {
        // TODO: Add instant shieldstun toggle for shield art out of electric hitstun? Idk that's so specific
        if can_transition(cpu_module_accessor) {
            should_playback = true;
        }
    } else if can_transition(cpu_module_accessor) {
        should_playback = true;
    }

    // how do we deal with buffering motion inputs out of shield? You can't complete them in one frame, but they can definitely be buffered during shield drop
    // probably need a separate standby setting for grounded, aerial, shield, where shield starts once you let go of shield, and aerial keeps you in the air?

    if should_playback {
        playback(Some(mash::queued_playback_slot()));
    }
}

// TODO: set up a better match later and make this into one func

fn into_starting_status(status: i32) -> StartingStatus {
    if status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        return StartingStatus::Aerial;
    } else if (*FIGHTER_STATUS_KIND_ESCAPE_AIR..*FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE)
        .contains(&status)
    {
        return StartingStatus::Airdodge;
    } else if status == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        return StartingStatus::SpecialHi;
    } else if status == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        return StartingStatus::Jump;
    } else if status == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
        return StartingStatus::DoubleJump;
    } else if status == *FIGHTER_STATUS_KIND_ESCAPE {
        return StartingStatus::Spotdodge;
    } else if (*FIGHTER_STATUS_KIND_ESCAPE_F..*FIGHTER_STATUS_KIND_ESCAPE_B).contains(&status) {
        return StartingStatus::Roll;
    } else if status == *FIGHTER_STATUS_KIND_CATCH {
        return StartingStatus::Grab;
    }
    StartingStatus::Other
}

fn into_transition_term(starting_status: StartingStatus) -> i32 {
    match starting_status {
        StartingStatus::Aerial => *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR,
        StartingStatus::Airdodge => *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR,
        StartingStatus::Other => *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, // placeholder - most likely don't want to use this in final build, and have a different set of checks
        StartingStatus::SpecialHi => *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        StartingStatus::Jump => *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
        StartingStatus::DoubleJump => *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL,
        StartingStatus::Spotdodge => *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        StartingStatus::Roll => *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
        StartingStatus::Grab => *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
    }
}

pub unsafe fn get_command_flag_cat(module_accessor: &mut BattleObjectModuleAccessor) {
    // Allow this because sometimes we want to make sure our NNSDK doesn't have
    // an erroneous definition
    #[allow(clippy::unnecessary_cast)]
    let entry_id_int =
        WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
    let fighter_kind = utility::get_kind(module_accessor);
    let fighter_is_nana = fighter_kind == *FIGHTER_KIND_NANA;

    CURRENT_RECORD_SLOT = MENU.recording_slot.into_idx();

    if entry_id_int == 0 && !fighter_is_nana {
        if button_config::combo_passes_exclusive(button_config::ButtonCombo::InputPlayback) {
            playback(MENU.playback_button_combination.get_random().into_idx());
        } else if MENU.record_trigger == OnOff::On
            && button_config::combo_passes_exclusive(button_config::ButtonCombo::InputRecord)
        {
            lockout_record();
        }

        // may need to move this to another func
        if (INPUT_RECORD == Record || INPUT_RECORD == Playback)
            && INPUT_RECORD_FRAME >= CURRENT_FRAME_LENGTH - 1
        {
            INPUT_RECORD = None;
            POSSESSION = Player;
            INPUT_RECORD_FRAME = 0;
            if mash::is_playback_queued() {
                mash::reset();
            }
            if MENU.playback_loop == OnOff::On {
                playback(Some(CURRENT_PLAYBACK_SLOT));
            }
        }
    }

    // Handle Possession Coloring
    if entry_id_int == 1 && POSSESSION == Lockout {
        clear_notifications("Input Recording");
        color_notification(
            "Input Recording".to_string(),
            "Lockout".to_owned(),
            60,
            ResColor {
                r: 200,
                g: 8,
                b: 8,
                a: 255,
            },
        );
        set_color_rgb_2(
            module_accessor,
            0.0,
            0.0,
            1.0,
            *MODEL_COLOR_TYPE_COLOR_BLEND,
        );
    } else if entry_id_int == 1 && POSSESSION == Standby {
        clear_notifications("Input Recording");
        color_notification(
            "Input Recording".to_string(),
            "Standby".to_owned(),
            60,
            ResColor {
                r: 200,
                g: 8,
                b: 200,
                a: 255,
            },
        );
        set_color_rgb_2(
            module_accessor,
            1.0,
            0.0,
            1.0,
            *MODEL_COLOR_TYPE_COLOR_BLEND,
        );
    } else if entry_id_int == 1 && POSSESSION == Cpu {
        clear_notifications("Input Recording");
        color_notification(
            "Input Recording".to_string(),
            "Recording".to_owned(),
            60,
            ResColor {
                r: 200,
                g: 8,
                b: 8,
                a: 255,
            },
        );
        set_color_rgb_2(
            module_accessor,
            1.0,
            0.0,
            0.0,
            *MODEL_COLOR_TYPE_COLOR_BLEND,
        );
    }
}

pub unsafe fn lockout_record() {
    INPUT_RECORD = Pause;
    INPUT_RECORD_FRAME = 0;
    POSSESSION = Lockout;
    P1_FINAL_MAPPING.lock()[CURRENT_RECORD_SLOT]
        .iter_mut()
        .for_each(|mapped_input| {
            *mapped_input = MappedInputs::empty();
        });
    CURRENT_FRAME_LENGTH = MENU.recording_frames.into_frames();
    P1_FRAME_LENGTH_MAPPING.lock()[CURRENT_RECORD_SLOT] = CURRENT_FRAME_LENGTH;
    LOCKOUT_FRAME = 30; // This needs to be this high or issues occur dropping shield - but does this cause problems when trying to record ledge?
    BUFFER_FRAME = 0;
    // Store the direction the CPU is facing when we initially record, so we can turn their inputs around if needed
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    RECORDED_LR = PostureModule::lr(cpu_module_accessor);
    CURRENT_LR = RECORDED_LR;
}

// Returns whether we did playback
pub unsafe fn playback(slot: Option<usize>) -> bool {
    if INPUT_RECORD == Pause {
        println!("Tried to playback during lockout!");
        return false;
    }
    if slot.is_none() {
        println!("Tried to playback without a slot selected!");
        return false;
    }
    let slot = slot.unwrap();

    clear_notifications("Input Recording");
    color_notification(
        "Input Recording".to_string(),
        "Playback".to_owned(),
        60,
        ResColor {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
    );

    CURRENT_PLAYBACK_SLOT = slot;
    CURRENT_FRAME_LENGTH = P1_FRAME_LENGTH_MAPPING.lock()[CURRENT_PLAYBACK_SLOT];
    INPUT_RECORD = Playback;
    POSSESSION = Player;
    INPUT_RECORD_FRAME = 0;
    BUFFER_FRAME = 0;
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    CURRENT_LR = PostureModule::lr(cpu_module_accessor);

    true
}

pub unsafe fn playback_ledge(slot: Option<usize>) {
    let did_playback = playback(slot);
    if did_playback {
        BUFFER_FRAME = 5; // So we can make sure the option is buffered and won't get ledge trumped if delay is 0
                          // drop down from ledge can't be buffered on the same frame as jump/attack/roll/ngu so we have to do this
                          // Need to buffer 1 less frame for non-lassos
        let cpu_module_accessor = get_module_accessor(FighterId::CPU);
        let status_kind = StatusModule::status_kind(cpu_module_accessor) as i32;
        if status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH {
            BUFFER_FRAME -= 1;
        }
    }
}

pub unsafe fn stop_playback() {
    INPUT_RECORD = None;
    INPUT_RECORD_FRAME = 0;
}

pub unsafe fn is_end_standby() -> bool {
    // Returns whether we should be done with standby this frame (if any significant controller input has been made)
    let first_frame_input = P1_FINAL_MAPPING.lock()[CURRENT_RECORD_SLOT][0];

    let clamped_lstick_x =
        ((first_frame_input.lstick_x as f32) * STICK_CLAMP_MULTIPLIER).clamp(-1.0, 1.0);
    let clamped_lstick_y =
        ((first_frame_input.lstick_y as f32) * STICK_CLAMP_MULTIPLIER).clamp(-1.0, 1.0);
    let clamped_rstick_x =
        ((first_frame_input.rstick_x as f32) * STICK_CLAMP_MULTIPLIER).clamp(-1.0, 1.0);
    let clamped_rstick_y =
        ((first_frame_input.rstick_y as f32) * STICK_CLAMP_MULTIPLIER).clamp(-1.0, 1.0);

    // No buttons pressed or just flick jump-- if they really did a stick jump, we'd have lstick movement as well
    let buttons_pressed =
        !(first_frame_input.buttons.is_empty() || first_frame_input.buttons == Buttons::FLICK_JUMP);
    let lstick_movement =
        clamped_lstick_x.abs() >= STICK_NEUTRAL || clamped_lstick_y.abs() >= STICK_NEUTRAL;
    let rstick_movement =
        clamped_rstick_x.abs() >= STICK_NEUTRAL || clamped_rstick_y.abs() >= STICK_NEUTRAL;
    lstick_movement || rstick_movement || buttons_pressed
}

pub unsafe fn handle_final_input_mapping(player_idx: i32, out: *mut MappedInputs) {
    if player_idx == 0 {
        // if player 1
        if INPUT_RECORD == Record {
            // check for standby before starting action:
            if POSSESSION == Standby && is_end_standby() {
                // last input made us start an action, so start recording and end standby.
                INPUT_RECORD_FRAME += 1;
                POSSESSION = Cpu;
            }

            if INPUT_RECORD_FRAME == 1 {
                // We're on the second frame of recording, grabbing the status should give us the status that resulted from the first frame of input
                // We'll want to save this status so that we use the correct TRANSITION TERM for hitstun cancelling out of damage fly
                let cpu_module_accessor = get_module_accessor(FighterId::CPU);
                P1_STARTING_STATUSES.lock()[CURRENT_PLAYBACK_SLOT] =
                    into_starting_status(StatusModule::status_kind(cpu_module_accessor));
                STARTING_STATUS = StatusModule::status_kind(cpu_module_accessor);
                // TODO: Handle this based on slot later instead
            }

            P1_FINAL_MAPPING.lock()[CURRENT_RECORD_SLOT][INPUT_RECORD_FRAME] = *out;
            *out = MappedInputs::empty(); // don't control player while recording
            println!("Stored Player Input! Frame: {}", INPUT_RECORD_FRAME);
        }
    }
}

#[skyline::hook(offset = 0x2da180)] // After cpu controls are assigned from ai calls
unsafe fn set_cpu_controls(p_data: *mut *mut u8) {
    call_original!(p_data);
    let controller_data = *p_data.add(1) as *mut ControlModuleInternal;
    let controller_no = (*controller_data).controller_index;

    // Check if we need to begin playback this frame due to a mash toggle
    // TODO: Setup STARTING_STATUS based on current playback slot here

    // This check prevents out of shield if mash exiting is on
    if INPUT_RECORD == None {
        should_mash_playback();
    }

    if INPUT_RECORD == Pause {
        match LOCKOUT_FRAME.cmp(&0) {
            Ordering::Greater => LOCKOUT_FRAME -= 1,
            Ordering::Equal => {
                INPUT_RECORD = Record;
                POSSESSION = Standby;
            }
            Ordering::Less => println!("LOCKOUT_FRAME OUT OF BOUNDS"),
        }
    }

    if INPUT_RECORD == Record || INPUT_RECORD == Playback {
        let mut x_input_multiplier = RECORDED_LR * CURRENT_LR; // if we aren't facing the way we were when we initially recorded, we reverse horizontal inputs
                                                               // Don't flip Shulk's dial inputs
        let cpu_module_accessor = get_module_accessor(FighterId::CPU);
        let fighter_kind = utility::get_kind(&mut *cpu_module_accessor);
        if fighter_kind == *FIGHTER_KIND_SHULK {
            let circle_menu_flag = WorkModule::is_flag(
                &mut *cpu_module_accessor,
                *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CIRCLE_MENU,
            );
            if circle_menu_flag {
                // While in dial, don't flip horizontal inputs
                x_input_multiplier = 1.0;
            }
            // If we have issues with the frame after the dial comes out, change condition to
            //  circle_menu_flag && FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_DECIDE_INTERVAL_FRAME > 1
        }
        println!("Overriding Cpu Player: {}, Frame: {}, BUFFER_FRAME: {}, STARTING_STATUS: {}, INPUT_RECORD: {:#?}, POSSESSION: {:#?}", controller_no, INPUT_RECORD_FRAME, BUFFER_FRAME, STARTING_STATUS, INPUT_RECORD, POSSESSION);

        let mut saved_mapped_inputs = P1_FINAL_MAPPING.lock()[if INPUT_RECORD == Record {
            CURRENT_RECORD_SLOT
        } else {
            CURRENT_PLAYBACK_SLOT
        }][INPUT_RECORD_FRAME];

        if BUFFER_FRAME <= 3 && BUFFER_FRAME > 0 {
            // Our option is already buffered, now we need to 0 out inputs to make sure our future controls act like flicks/presses instead of holding the button
            saved_mapped_inputs = MappedInputs::empty();
        }

        (*controller_data).buttons = saved_mapped_inputs.buttons;
        (*controller_data).stick_x =
            x_input_multiplier * ((saved_mapped_inputs.lstick_x as f32) / (i8::MAX as f32));
        (*controller_data).stick_y = (saved_mapped_inputs.lstick_y as f32) / (i8::MAX as f32);
        // Clamp stick inputs for separate part of structure
        let mut clamped_lstick_x = x_input_multiplier
            * ((saved_mapped_inputs.lstick_x as f32) * STICK_CLAMP_MULTIPLIER).clamp(-1.0, 1.0);
        let mut clamped_lstick_y =
            ((saved_mapped_inputs.lstick_y as f32) * STICK_CLAMP_MULTIPLIER).clamp(-1.0, 1.0);
        clamped_lstick_x = if clamped_lstick_x.abs() >= STICK_NEUTRAL {
            clamped_lstick_x
        } else {
            0.0
        };
        clamped_lstick_y = if clamped_lstick_y.abs() >= STICK_NEUTRAL {
            clamped_lstick_y
        } else {
            0.0
        };
        (*controller_data).clamped_lstick_x = clamped_lstick_x;
        (*controller_data).clamped_lstick_y = clamped_lstick_y;
        //println!("CPU Buttons: {:#018b}", (*controller_data).buttons);

        // Keep counting frames, unless we're in standby waiting for an input, or are buffering an option
        // When buffering an option, we keep inputting the first frame of input during the buffer window
        if BUFFER_FRAME > 0 {
            BUFFER_FRAME -= 1;
        } else if INPUT_RECORD_FRAME < CURRENT_FRAME_LENGTH - 1 && POSSESSION != Standby {
            INPUT_RECORD_FRAME += 1;
        }
    }
}

pub unsafe fn is_playback() -> bool {
    INPUT_RECORD == Record || INPUT_RECORD == Playback
}

pub unsafe fn is_recording() -> bool {
    INPUT_RECORD == Record
}

pub unsafe fn is_standby() -> bool {
    POSSESSION == Standby || POSSESSION == Lockout
}

extern "C" {
    // TODO: we should be using this from skyline
    #[link_name = "\u{1}_ZN3app8lua_bind31ModelModule__set_color_rgb_implEPNS_26BattleObjectModuleAccessorEfffNS_16MODEL_COLOR_TYPEE"]
    pub fn set_color_rgb_2(
        arg1: *mut BattleObjectModuleAccessor,
        arg2: f32,
        arg3: f32,
        arg4: f32,
        arg5: i32,
    );
}

pub fn init() {
    skyline::install_hooks!(set_cpu_controls);
}
