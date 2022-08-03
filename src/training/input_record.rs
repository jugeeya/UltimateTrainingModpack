use skyline::nn::hid::NpadGcState;
use smash::app::{BattleObjectModuleAccessor, lua_bind::*};
use smash::lib::lua_const::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use crate::training::input_recording::structures::*;
use crate::common::consts::*;
use crate::common::*;

lazy_static! {
    static ref P1_FINAL_MAPPING: Mutex<[MappedInputs; 90]> =
        Mutex::new([{
            MappedInputs::default()
        }; 90]);
}

pub static mut INPUT_RECORD: InputRecordState = InputRecordState::None;
pub static mut INPUT_RECORD_FRAME: usize = 0;
pub static mut CPU_CONTROL_ADDR: *mut ControlModuleInternal = 0 as *mut ControlModuleInternal;

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
        *mapped_input = MappedInputs::default();
    });
    INPUT_RECORD_FRAME = 0;
}

pub unsafe fn playback() {
    INPUT_RECORD = Playback;
    INPUT_RECORD_FRAME = 0;
}

pub fn handle_get_npad_state( // Shouldn't need this hook anymore, keeping for reference for now
    _state: *mut NpadGcState,
    _controller_id: *const u32,
) {
    /*unsafe {
        if *controller_id == p1_controller_id() {
            if INPUT_RECORD == Record {
                P1_NPAD_STATES.lock()[INPUT_RECORD_FRAME] = *state;
            }
        } else if INPUT_RECORD == Record || INPUT_RECORD == Playback {
            let update_count = (*state).updateCount;
            *state = P1_NPAD_STATES.lock()[INPUT_RECORD_FRAME];
            (*state).updateCount = update_count;
        }
    }*/
}

// TODO: Explain
static FIM_OFFSET: usize = 0x17504a0; 
// TODO: Should we define all of our offsets in one file, like HDR? Should at least be a good start for changing to be based on ASM instructions
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
    //println!("Player: {}, Out Addr: {:p}", player_idx, out);
    if player_idx == 0 { // if player 1 (what is going on here? switching from handheld to docked seems to make this change to 1 and 2 instead of 0)
        if INPUT_RECORD == Record {
            P1_FINAL_MAPPING.lock()[INPUT_RECORD_FRAME] = *out;
            *out = MappedInputs::default() // don't control player while recording TODO: Change this for later, want off during dev and testing
        }
    } 
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
}

#[skyline::hook(offset = 0x2da180)] // After cpu controls are assigned from ai calls
unsafe fn set_cpu_controls(p_data: *mut *mut u8) {
  call_original!(p_data);
  let controller_data = *p_data.add(1) as *mut ControlModuleInternal;
  let _controller_no  = (*controller_data).controller_index;
  let input_type;
  if INPUT_RECORD == Record {
    input_type = "Record";
  } else if INPUT_RECORD == Playback {
    input_type = "Playback";
  } else {
    input_type = "Other";
  }
  if INPUT_RECORD == Record || INPUT_RECORD == Playback {
    //println!("Overriding Cpu Player: {}", controller_no); // cpu is normally 1, at least on handheld
    if INPUT_RECORD_FRAME > 0 {
        let saved_mapped_inputs = P1_FINAL_MAPPING.lock()[INPUT_RECORD_FRAME-1];
        (*controller_data).buttons = saved_mapped_inputs.buttons;
        (*controller_data).stick_x = (saved_mapped_inputs.lstick_x as f32) / (i8::MAX as f32);
        (*controller_data).stick_y = (saved_mapped_inputs.lstick_y as f32) / (i8::MAX as f32);
        println!("{} CPU, Frame: {}", input_type, INPUT_RECORD_FRAME); //debug
        /*println!("Saved stick x: {}, new stick x: {}", saved_mapped_inputs.lstick_x, *(controller_data.add(0x40) as *mut f32));
        println!("Saved stick y: {}, new stick y: {}", saved_mapped_inputs.lstick_y, *(controller_data.add(0x44) as *mut f32));
        */
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
        println!("CPU Buttons: {:#018b}", (*controller_data).buttons);
    }
    
    CPU_CONTROL_ADDR = controller_data;
    //println!("Saving CPU Addr as {:p}", controller_data);
    let cpu_module_accessor = get_module_accessor(FighterId::CPU);
    let cpu_control_module_reference_location = (cpu_module_accessor as  *mut *mut u64).add(0x48 / 8); // boma + 72, value here points to controlmodule
    let cpu_control_module = *(cpu_control_module_reference_location); // we're saying the value at this address is the address of the controlmodule
    // We dereference once to go from the pointer to the address that points to the control module, to the pointer to the control module
    // IMPORTANT! Above should not be u64, it should be ControlModule. But ControlModule is a module, not a type, so it doesn't work for now. ControlModule is size 4096.
    // println!("CPU BOMA: {:p}, CM_Ref: {:p}, ControlModule: {:p}, ControlModuleInternal: {:p}", cpu_module_accessor, cpu_control_module_reference_location, cpu_control_module, controller_data);

  } 
}

pub fn init() {
    skyline::install_hooks!(
        handle_final_input_mapping,
        set_cpu_controls
    );
}