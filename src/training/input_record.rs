use smash::app::{BattleObjectModuleAccessor, lua_bind::*, utility};
use smash::lib::lua_const::*;
use smash::phx::{Hash40};
use lazy_static::lazy_static;
use parking_lot::Mutex;
//use skyline::logging::print_stack_trace;
use crate::training::input_recording::structures::*;
use crate::common::consts::{RecordTrigger, FighterId};
use crate::common::{MENU, get_module_accessor};


#[derive(PartialEq)]
pub enum InputRecordState {
    None,
    Pause,
    Record,
    Playback,
}

#[derive(PartialEq)]
pub enum PossessionState {
    Player,
    Cpu,
    Lockout,
    Standby,
}

use InputRecordState::*;
use PossessionState::*;

const FINAL_RECORD_MAX: usize = 150; // Maximum length for input recording sequences (capacity)
pub static mut FINAL_RECORD_FRAME: usize = FINAL_RECORD_MAX; // The final frame to play back of the currently recorded sequence (size)
pub static mut INPUT_RECORD: InputRecordState = InputRecordState::None;
pub static mut INPUT_RECORD_FRAME: usize = 0;
pub static mut POSSESSION: PossessionState = PossessionState::Player;
pub static mut LOCKOUT_FRAME: usize = 0;

lazy_static! {
    static ref P1_FINAL_MAPPING: Mutex<[MappedInputs; FINAL_RECORD_MAX]> =
        Mutex::new([{
            MappedInputs::default()
        }; FINAL_RECORD_MAX]);
}

pub unsafe fn get_command_flag_cat(module_accessor: &mut BattleObjectModuleAccessor) {
    let entry_id_int =
            WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
    let fighter_kind = utility::get_kind(module_accessor);
    let fighter_is_nana = fighter_kind == *FIGHTER_KIND_NANA;

    if entry_id_int == 0 && !fighter_is_nana {
        // Attack + Dpad Right: Playback
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            //crate::common::raygun_printer::print_string(&mut *module_accessor, "PLAYBACK");
            playback();
            println!("Playback Command Received!"); //debug
        }
        // Attack + Dpad Left: Record
        else if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
            && MENU.record_trigger == RecordTrigger::COMMAND
        {
           //crate::common::raygun_printer::print_string(&mut *module_accessor, "RECORDING");
           record();
           println!("Record Command Received!"); //debug
        }


        // may need to move this to another func
        if INPUT_RECORD == Record || INPUT_RECORD == Playback {
            if INPUT_RECORD_FRAME >= P1_FINAL_MAPPING.lock().len() - 1 { // FINAL_RECORD_FRAME - 1 { 
                // Above alternative causes us to stay on last input forever, need to figure out since we want to be able to have shorter playbacks
                INPUT_RECORD = None;
                POSSESSION = Player;
                INPUT_RECORD_FRAME = 0;
            }
        }
    }

    // Handle Possession Effects
    //let highlight_hash = Hash40::new("demon_rage2"); // red faster blink
    let blue_hash = Hash40::new("cloud_limitbreak"); // blue
    let purple_hash = Hash40::new("reflet_special_lw_damage"); // red slight blink
    let red_hash = Hash40::new("demon_rage");
    let is_red = EffectModule::is_exist_common(module_accessor,red_hash);
    let is_blue = EffectModule::is_exist_common(module_accessor,blue_hash);
    let is_purple = EffectModule::is_exist_common(module_accessor,purple_hash);
    if entry_id_int == 1 && POSSESSION == Lockout {
        if !is_blue {
            EffectModule::req_common(module_accessor,blue_hash,0.0);
        }
    } else if entry_id_int == 1 && POSSESSION == Standby {
        if !is_purple {
            EffectModule::req_common(module_accessor,purple_hash,0.0);
        }
    } else if entry_id_int == 1 && POSSESSION == Cpu {
        // if we're controlling the Cpu and we don't have an effect, call the effect
        if !is_red {
            EffectModule::req_common(module_accessor,red_hash,0.0);
        }
    } else if entry_id_int == 1 && POSSESSION == Player {
        // Remove effects since we're controlling the player
        if is_red {
            EffectModule::remove_common(module_accessor,red_hash);
        } else if is_blue {
            EffectModule::remove_common(module_accessor,blue_hash);
        } else if is_purple {
            EffectModule::remove_common(module_accessor,purple_hash);
        }
    }
}

pub unsafe fn lockout_record() {
    INPUT_RECORD = Pause;
    //INPUT_RECORD_FRAME = 0;
    POSSESSION = Lockout;
    LOCKOUT_FRAME = 5;
}

pub unsafe fn record() {
    INPUT_RECORD = Record;
    POSSESSION = Cpu;
    // Reset mappings to nothing, and then start recording. Likely want to reset in case we cut off recording early.
    P1_FINAL_MAPPING.lock().iter_mut().for_each(|mapped_input| {
        *mapped_input = MappedInputs::default();
    });
    INPUT_RECORD_FRAME = 0;
}

pub unsafe fn playback() {
    INPUT_RECORD = Playback;
    INPUT_RECORD_FRAME = 0;
}

pub unsafe fn stop_playback() {
    INPUT_RECORD = None;
    INPUT_RECORD_FRAME = 0;
}

pub unsafe fn is_end_standby() -> bool {
    // Returns whether we should be done with standby this frame (if the fighter is no longer in a waiting status)
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    let status_kind = StatusModule::status_kind(cpu_module_accessor) as i32;
    ![
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_CLIFF_WAIT,
    ]
    .contains(&status_kind)
}

static FIM_OFFSET: usize = 0x17504a0; 
// TODO: Should we define all of our offsets in one file? Should at least be a good start for changing to be based on ASM instructions
#[skyline::hook(offset = FIM_OFFSET)]
unsafe fn handle_final_input_mapping(
    mappings: *mut ControllerMapping,
    player_idx: i32, // Is this the player index, or plugged in controller index? Need to check, assuming player for now - is this 0 indexed or 1?
    out: *mut MappedInputs,
    controller_struct: &mut SomeControllerStruct,
    arg: bool
) {
    // go through the original mapping function first
    let _ret = original!()(mappings, player_idx, out, controller_struct, arg);
    if player_idx == 0 { // if player 1
        if INPUT_RECORD == Record {
            P1_FINAL_MAPPING.lock()[INPUT_RECORD_FRAME] = *out;
            *out = MappedInputs::default(); // don't control player while recording
            println!("Stored Player Input! Frame: {}",INPUT_RECORD_FRAME);
        }
    } 
}

#[skyline::hook(offset = 0x2da180)] // After cpu controls are assigned from ai calls
unsafe fn set_cpu_controls(p_data: *mut *mut u8) {
    call_original!(p_data);
    let controller_data = *p_data.add(1) as *mut ControlModuleInternal;
    let controller_no  = (*controller_data).controller_index;
    if INPUT_RECORD == Record || INPUT_RECORD == Playback {
        println!("Overriding Cpu Player: {}, Frame: {}", controller_no, INPUT_RECORD_FRAME);
        let saved_mapped_inputs = P1_FINAL_MAPPING.lock()[INPUT_RECORD_FRAME];
        (*controller_data).buttons = saved_mapped_inputs.buttons;
        (*controller_data).stick_x = (saved_mapped_inputs.lstick_x as f32) / (i8::MAX as f32);
        (*controller_data).stick_y = (saved_mapped_inputs.lstick_y as f32) / (i8::MAX as f32);
        // Clamp stick inputs for separate part of structure
        const NEUTRAL: f32 = 0.2;
        const CLAMP_MAX: f32 = 120.0;
        let clamp_mul = 1.0 / CLAMP_MAX;
        let mut clamped_lstick_x = ((saved_mapped_inputs.lstick_x as f32) * clamp_mul).clamp(-1.0, 1.0);
        let mut clamped_lstick_y = ((saved_mapped_inputs.lstick_y as f32) * clamp_mul).clamp(-1.0, 1.0);
        clamped_lstick_x = if clamped_lstick_x.abs() >= NEUTRAL { clamped_lstick_x } else { 0.0 };
        clamped_lstick_y = if clamped_lstick_y.abs() >= NEUTRAL { clamped_lstick_y } else { 0.0 };
        (*controller_data).clamped_lstick_x = clamped_lstick_x;
        (*controller_data).clamped_lstick_y = clamped_lstick_y;
        //println!("CPU Buttons: {:#018b}", (*controller_data).buttons);
        //let cpu_module_accessor = get_module_accessor(FighterId::CPU);
        if INPUT_RECORD_FRAME < P1_FINAL_MAPPING.lock().len() - 1 {
            INPUT_RECORD_FRAME += 1;
        }
    } 
}

pub unsafe fn is_playback() -> bool {
    if INPUT_RECORD == Record || INPUT_RECORD == Playback {
        true
    } else {
        false
    }
}

pub unsafe fn is_recording() -> bool {
    if INPUT_RECORD == Record {
        true
    } else {
        false
    }
}

pub fn init() {
    skyline::install_hooks!(
        set_cpu_controls,
        handle_final_input_mapping,
    );
}