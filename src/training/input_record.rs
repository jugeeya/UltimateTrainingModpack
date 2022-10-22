use skyline::nn::hid::NpadGcState;
use smash::app::{BattleObjectModuleAccessor, lua_bind::*};
use smash::lib::lua_const::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use skyline::logging::print_stack_trace;
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
    //debug: // TODO: comment call to see if impl always is running (shouldn't be)
    ControlModule::get_attack_air_kind(module_accessor);
    // is this too early? maybe try moving this later on; get attack is probably used to go from small structure to bigger?
    // no; above is untrue; it checks the same spot in controlmodule we're trying to get to

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
                //INPUT_RECORD_FRAME += 1;
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
    if INPUT_RECORD_FRAME < P1_FINAL_MAPPING.lock().len() - 1 {
        INPUT_RECORD_FRAME += 1;
    }
  }
}

#[skyline::hook(offset = 0x3f7220)] // Used by HDR to implement some of their control changes
unsafe fn parse_internal_controls(current_control_internal: &mut ControlModuleInternal) {
    let control_index = (*current_control_internal).controller_index;
    // go through the original parsing function first (this may be wrong?)
    call_original!(current_control_internal);

    if control_index == 0 { // if player 1 (need to check if it works this way docked)
        if INPUT_RECORD == Record {
            P1_FINAL_MAPPING.lock()[INPUT_RECORD_FRAME] = (*current_control_internal).construct_stored(); // am I hard copying this correctly?
            current_control_internal.clear() // don't control player while recording
        }
    } 
}

/*#[skyline::hook(offset = 0x06bdc40)]
pub unsafe fn handle_get_atk_air(
    module_accessor: *mut BattleObjectModuleAccessor,
) -> i32 {
    let ori = original!()(module_accessor);
    // + 0x48 = Control Module location 
    // -------
    // 0x0 = vtable ( 0x4F83DE0 , which + 0x3b0 = GetAttackAirKind addr)
    // 0x8 = Boma pointer
    // 0x624 = AttackAirKind?
    
    // let control_module_addr: *const u64 = ((module_accessor as u64) + 0x48) as *const u64; // mod_accessor pointer + 0x48 should be the reference to the control module
    // let vtable_addr: *const u64 = (*control_module_addr) as *const u64; // so this shouldn't be zero???
    // let atk_air_kind_dec_addr: *const i32 = ((vtable_addr as u64) + 624) as *const i32; // this 624 is dec?
    // let atk_air_kind_hex_addr: *const i32 = ((vtable_addr as u64) + 0x624) as *const i32; // this 624 is dec?

    // let boma_val_atk_air_kind_dec_addr: *const i32 = ((*module_accessor)._address + 624) as *const i32; // this 624 is dec?
    // let boma_val_atk_air_kind_hex_addr: *const i32 = ((*module_accessor)._address + 0x624) as *const i32; // this 624 is dec?


    //println!("boma: {:p}, boma-_addr: {}, boma_val: {}, control: {:p}, vtable: {:p}, atk_air_kind: {}, actual: {}",module_accessor,(*module_accessor)._address,*(module_accessor as *const u64),control_module_addr,vtable_addr,*atk_air_kind_addr,ori);
    //skyline::logging::print_stack_trace();


    // above prob wrong
    // BOMA + 0x16 = module accessor (size 400) + 48 = StatusModule???? (ControlModule is 56 into module accessor?)
    //16 + 56 = 0x48

    //let opt_a_h = (*(((module_accessor as u64) + 0x48) as *const u64) + 0x624) as *const i32;
    //let opt_a_d = (*(((module_accessor as u64) + 0x48) as *const u64) + 624) as *const i32;

    //let opt_b_h = (*(((module_accessor as u64) + 56) as *const u64) + 0x624) as *const i32;
    //let opt_b_d = (*(((module_accessor as u64) + 56) as *const u64) + 624) as *const i32;

    //let opt_c_h = (*((((*module_accessor)._address as u64) + 0x48) as *const u64) + 0x624) as *const i32;
    //let opt_c_d = (*((((*module_accessor)._address as u64) + 0x48) as *const u64) + 624) as *const i32;

    //let opt_d_h = (*((((*module_accessor)._address as u64) + 56) as *const u64) + 0x624) as *const i32;
    //let opt_d_d = (*((((*module_accessor)._address as u64) + 56) as *const u64) + 624) as *const i32;

    //let opt_f_h = ((module_accessor as u64) + 0x624) as *const i32;
    //let opt_f_d = ((module_accessor as u64) + 624) as *const i32;

    //println!("A-h: {}, A-d: {}, B-h: {}, B-d: {}, C-h: {}, C-d: {}, D-h: {}, D-d: {}, F-h: {}, F-d: {}, ori: {}",*opt_a_h,*opt_a_d,*opt_b_h,*opt_b_d,0,0,0,0,*opt_f_h,*opt_f_d,ori);
    //println!("A-h: {}, A-d: {}, B-h: {}, B-d: {}, C-h: {}, C-d: {}, D-h: {}, D-d: {}, F-h: {}, F-d: {}, ori: {}",*opt_a_h,*opt_a_d,0,0,0,0,0,0,0,0,ori);
    //println!("A-h: {}, A-d: {}, ori: {}",*opt_a_h,*opt_a_d,ori);
    //println!("help");

    //unknown issue with opt a causing crash; maybe need an "as *const i32"? only thing missing of it vs first ver; maybe can't do it all in one line?


    ori
    //TODO: Figure out what the address of the boma is really referncing; RAM? Or an actual correct address?
}*/

/*#[skyline::hook(offset = 0x02000c10)]
pub unsafe fn handle_get_atk_air(
    boma: *mut BattleObjectModuleAccessor2,
) -> i32 {
    let ori = original!()(boma);
    let module_accessor = (*boma).module_accessor;
    let control_module = module_accessor.control_module;
    let atk_air_kind = (*control_module).atk_air_kind;
    
    //let atk_air_kind_d = ((*control_module) + 624) as *const i32; // should be vtable + 624?
    //let atk_air_kind_d = *((module_accessor as *const i32).add(624));
    //println!("A-d: {}, ori: {}",atk_air_kind,ori);
    ori
    // If setting attack_air_kind directly in cpu func isn't working it's prob because it's getting set back to 0
    // just store attack_air_kind and fastfall in padding? prob not best idea but maybe worth????
}*/

pub fn init() {
    skyline::install_hooks!(
        set_cpu_controls,
        parse_internal_controls,
        handle_get_atk_air,
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