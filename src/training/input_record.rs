use skyline::nn::hid::NpadHandheldState;
use smash::app::{BattleObjectModuleAccessor, lua_bind::*};
use smash::lib::lua_const::*;
use crate::training::input_delay::p1_controller_id;

pub static mut P1_NPAD_STATES: &mut [NpadHandheldState; 90] = &mut [{
    NpadHandheldState {
        updateCount: 0,
        Buttons: 0,
        LStickX: 0,
        LStickY: 0,
        RStickX: 0,
        RStickY: 0,
        Flags: 0,
    }
}; 90];

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
        else if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
        {
           record();
        }



        if INPUT_RECORD == Record || INPUT_RECORD == Playback {
            if INPUT_RECORD_FRAME >= P1_NPAD_STATES.len() - 1 {
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
    P1_NPAD_STATES.iter_mut().for_each(|state| {
        *state = NpadHandheldState {
            updateCount: 0,
            Buttons: 0,
            LStickX: 0,
            LStickY: 0,
            RStickX: 0,
            RStickY: 0,
            Flags: 0,
        }
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
            P1_NPAD_STATES[INPUT_RECORD_FRAME] = *state;
        }
    } else {
        if INPUT_RECORD == Record || INPUT_RECORD == Playback {
            (*state).Buttons = P1_NPAD_STATES[INPUT_RECORD_FRAME].Buttons;
            (*state).LStickX = P1_NPAD_STATES[INPUT_RECORD_FRAME].LStickX;
            (*state).LStickY = P1_NPAD_STATES[INPUT_RECORD_FRAME].LStickY;
            (*state).RStickX = P1_NPAD_STATES[INPUT_RECORD_FRAME].RStickX;
            (*state).RStickY = P1_NPAD_STATES[INPUT_RECORD_FRAME].RStickY;
            (*state).Flags = P1_NPAD_STATES[INPUT_RECORD_FRAME].Flags;
        }
    }
}
