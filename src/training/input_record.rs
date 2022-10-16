use skyline::nn::hid::NpadGcState;
use smash::app::{BattleObjectModuleAccessor, lua_bind::*};
use smash::lib::lua_const::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use crate::training::input_recording::structures::*;
use crate::common::consts::*;
use crate::common::*;

lazy_static! {
    static ref P1_FINAL_MAPPING: Mutex<[ControlModuleStored; 90]> =
        Mutex::new([{
            ControlModuleStored::default()
        }; 90]);
}

pub static mut INPUT_RECORD: InputRecordState = InputRecordState::None;
pub static mut INPUT_RECORD_FRAME: usize = 0;

#[derive(PartialEq)]
pub enum InputRecordState {
    None,
    Record,
    Playback,
}

use InputRecordState::*;

pub unsafe fn get_command_flag_cat(module_accessor: &mut BattleObjectModuleAccessor) {
    let entry_id_int =
            WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;

    if entry_id_int == 0 {
        // Attack + Dpad Right: Playback
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            crate::common::raygun_printer::print_string(&mut *module_accessor, "PLAYBACK");
            playback();
            println!("Playback Command Received!"); //debug
        }
        // Attack + Dpad Left: Record
        else if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
        {
           crate::common::raygun_printer::print_string(&mut *module_accessor, "RECORDING");
           record();
           println!("Record Command Received!"); //debug
        }


        // may need to move this to another func
        if INPUT_RECORD == Record || INPUT_RECORD == Playback {
            if INPUT_RECORD_FRAME >= P1_FINAL_MAPPING.lock().len() - 1 {
                if INPUT_RECORD == Record {
                    //INPUT_RECORD = Playback; // shouldn't do this, causes it to play twice. TODO: replace with line below once other things tested
                    INPUT_RECORD = None;
                } else if INPUT_RECORD == Playback {
                    INPUT_RECORD = None;
                }
                INPUT_RECORD_FRAME = 0;
            } else {
                INPUT_RECORD_FRAME += 1;
            }
        }
    }
}

pub unsafe fn record() {
    INPUT_RECORD = Record;
    // Reset mappings to nothing, and then start recording. Maybe this resetting is unnecessary? Unsure
    P1_FINAL_MAPPING.lock().iter_mut().for_each(|mapped_input| {
        *mapped_input = ControlModuleStored::default();
    });
    INPUT_RECORD_FRAME = 0;
}

pub unsafe fn playback() {
    INPUT_RECORD = Playback;
    INPUT_RECORD_FRAME = 0;
}

#[skyline::hook(offset = 0x2da180)] // After cpu controls are assigned from ai calls
unsafe fn set_cpu_controls(p_data: *mut *mut u8) {
  call_original!(p_data);
  let controller_data = *p_data.add(1) as *mut ControlModuleInternal;
  let controller_no  = (*controller_data).controller_index;

  if INPUT_RECORD == Record || INPUT_RECORD == Playback {
    //println!("Overriding Cpu Player: {}", controller_no); // cpu is normally 1, at least on handheld
    if INPUT_RECORD_FRAME > 0 {
        let saved_stored_inputs = P1_FINAL_MAPPING.lock()[INPUT_RECORD_FRAME-1];
        let saved_internal_inputs = saved_stored_inputs.construct_internal((*controller_data).vtable, controller_no);
        *controller_data = saved_internal_inputs;
    }
  }
}

#[skyline::hook(offset = 0x3f7220)] // Used by HDR to implement some of their control changes
unsafe fn parse_internal_controls(current_control_internal: &mut ControlModuleInternal) {
    let control_index = current_control_internal.controller_index;
    // go through the original parsing function first (this may be wrong?)
    call_original!(current_control_internal);

    if control_index == 0 { // if player 1 (need to check if it works this way docked)
        if INPUT_RECORD == Record {
            P1_FINAL_MAPPING.lock()[INPUT_RECORD_FRAME] = (*current_control_internal).construct_stored(); // am I hard copying this correctly?
            //current_control_internal.clear() // don't control player while recording TODO: uncomment
        }
    } 
}

pub fn init() {
    skyline::install_hooks!(
        set_cpu_controls,
        parse_internal_controls,
    );
}

/*
    // debug:
    let input_type;
    if INPUT_RECORD == Record {
        input_type = "Record";
    } else if INPUT_RECORD == Playback {
        input_type = "Playback";
    } else {
        input_type = "Other";
    }
    //println!("{} PLAYER, Frame: {}", input_type, INPUT_RECORD_FRAME); //debug
*/