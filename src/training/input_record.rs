use skyline::nn::hid::NpadHandheldState;
use smash::app::{BattleObjectModuleAccessor, lua_bind::*};
use smash::lib::lua_const::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use crate::training::input_delay::p1_controller_id;

lazy_static! {
    static ref P1_NPAD_STATES: Mutex<[NpadHandheldState; 90]> =
        Mutex::new([{
            NpadHandheldState::default()
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
            playback();
        }
        // Attack + Dpad Left: Record
        else if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH)
            && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
        {
           record();
        }



        if INPUT_RECORD == Record || INPUT_RECORD == Playback {
            if INPUT_RECORD_FRAME >= P1_NPAD_STATES.lock().len() - 1 {
                if INPUT_RECORD == Record {
                    INPUT_RECORD = Playback;
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
    P1_NPAD_STATES.lock().iter_mut().for_each(|state| {
        *state = NpadHandheldState::default();
    });
    INPUT_RECORD_FRAME = 0;
}

pub unsafe fn playback() {
    INPUT_RECORD = Playback;
    INPUT_RECORD_FRAME = 0;
}

pub unsafe fn handle_get_npad_state(
    state: *mut NpadHandheldState,
    controller_id: *const u32,
) {
    if *controller_id == p1_controller_id() {
        if INPUT_RECORD == Record {
            P1_NPAD_STATES.lock()[INPUT_RECORD_FRAME] = *state;
        }
    } else if INPUT_RECORD == Record || INPUT_RECORD == Playback {
        let update_count = (*state).updateCount;
        *state = P1_NPAD_STATES.lock()[INPUT_RECORD_FRAME];
        (*state).updateCount = update_count;
    }
}