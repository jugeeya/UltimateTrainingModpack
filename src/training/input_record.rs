use smash::app::{BattleObjectModuleAccessor, lua_bind::*, utility};
use smash::lib::lua_const::*;
use smash::phx::{Hash40};
use lazy_static::lazy_static;
use parking_lot::Mutex;
//use skyline::logging::print_stack_trace;
use crate::training::input_recording::structures::*;
use crate::common::consts::RecordTrigger;
use crate::common::MENU;

#[derive(PartialEq)]
pub enum InputRecordState {
    None,
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

lazy_static! {
    static ref P1_FINAL_MAPPING: Mutex<[ControlModuleStored; FINAL_RECORD_MAX]> =
        Mutex::new([{
            ControlModuleStored::default()
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
                if INPUT_RECORD == Record {
                    //INPUT_RECORD = Playback; // shouldn't do this, causes it to play twice. TODO: replace with line below once other things tested
                    INPUT_RECORD = None;
                    POSSESSION = Player;
                } else if INPUT_RECORD == Playback {
                    INPUT_RECORD = None;
                    POSSESSION = Player;
                }
                INPUT_RECORD_FRAME = 0;
            } else {
                //INPUT_RECORD_FRAME += 1;
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

pub unsafe fn record() {
    INPUT_RECORD = Record;
    POSSESSION = Cpu;
    // Reset mappings to nothing, and then start recording. Likely want to reset in case we cut off recording early.
    P1_FINAL_MAPPING.lock().iter_mut().for_each(|mapped_input| {
        *mapped_input = ControlModuleStored::default();
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

#[skyline::hook(offset = 0x2da180)] // After cpu controls are assigned from ai calls
unsafe fn set_cpu_controls(p_data: *mut *mut u8) {
  call_original!(p_data);
  let controller_data = *p_data.add(1) as *mut ControlModuleInternal;
  let controller_no  = (*controller_data).controller_index;

  if INPUT_RECORD == Record || INPUT_RECORD == Playback {
    if INPUT_RECORD_FRAME > 0 {
        let saved_stored_inputs = P1_FINAL_MAPPING.lock()[INPUT_RECORD_FRAME-1]; // don't think we can start at 0 since this happens before input is set up by player? unsure; like this it seems synced up
        // maybe test if it's actually synced up by not clearing inputs and seeing if one frame moves clank?
        let saved_internal_inputs = saved_stored_inputs.construct_internal((*controller_data).vtable, controller_no);
        *controller_data = saved_internal_inputs;
    }
    if INPUT_RECORD_FRAME < P1_FINAL_MAPPING.lock().len() - 1 {
        INPUT_RECORD_FRAME += 1;
    }
  }
}

#[skyline::hook(offset = 0x3f7220)] // Used by HDR to implement some of their control changes
unsafe fn parse_internal_controls(current_control_internal: &mut ControlModuleInternal) {
    let control_index = (*current_control_internal).controller_index;
    // go through the original parsing function first
    call_original!(current_control_internal);

    if control_index == 0 {
        if INPUT_RECORD == Record {
            P1_FINAL_MAPPING.lock()[INPUT_RECORD_FRAME] = (*current_control_internal).construct_stored();
            current_control_internal.clear() // don't control player while recording
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
        parse_internal_controls,
    );
}