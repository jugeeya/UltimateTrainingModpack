use lazy_static::lazy_static;
use parking_lot::Mutex;
use skyline::nn::hid::NpadHandheldState;
use std::collections::VecDeque;

lazy_static! {
    static ref P1_DELAYED_NPAD_STATES: Mutex<VecDeque<NpadHandheldState>> =
        Mutex::new(VecDeque::new());
}

pub unsafe fn p1_controller_id() -> u32 {
    use skyline::nn::hid::*;
    let min_controller_id = (0..8)
        .filter(|i| GetNpadStyleSet(i as *const _).flags != 0)
        .min()
        .unwrap_or(0);

    let handheld_id = 0x20;
    if GetNpadStyleSet(&handheld_id as *const _).flags != 0 {
        handheld_id
    } else {
        min_controller_id
    }
}

pub unsafe fn handle_get_npad_state(
    state: *mut skyline::nn::hid::NpadHandheldState,
    controller_id: *const u32,
) {
    if *controller_id == p1_controller_id() {
        let mut delayed_states = P1_DELAYED_NPAD_STATES.lock();
        let actual_state = *state;

        // if INPUT_RECORD == 1 {
        //     P1_NPAD_STATES[INPUT_RECORD_FRAME] = actual_state;
        // }

        if delayed_states.len() < crate::common::MENU.input_delay as usize {
            (*state).Buttons = 0;
            (*state).LStickX = 0;
            (*state).LStickY = 0;
            (*state).RStickX = 0;
            (*state).RStickY = 0;
            (*state).Flags = 0;
        } else if let Some(delayed_state) = delayed_states.back() {
            (*state).Buttons = delayed_state.Buttons;
            (*state).LStickX = delayed_state.LStickX;
            (*state).LStickY = delayed_state.LStickY;
            (*state).RStickX = delayed_state.RStickX;
            (*state).RStickY = delayed_state.RStickY;
            (*state).Flags = delayed_state.Flags;
        }

        delayed_states.push_front(actual_state);
        delayed_states.truncate(crate::common::MENU.input_delay as usize);
    } else {
        // if INPUT_RECORD == 1 || INPUT_RECORD == 2 {
        //     (*state).Buttons = P1_NPAD_STATES[INPUT_RECORD_FRAME].Buttons;
        //     (*state).LStickX = P1_NPAD_STATES[INPUT_RECORD_FRAME].LStickX;
        //     (*state).LStickY = P1_NPAD_STATES[INPUT_RECORD_FRAME].LStickY;
        //     (*state).RStickX = P1_NPAD_STATES[INPUT_RECORD_FRAME].RStickX;
        //     (*state).RStickY = P1_NPAD_STATES[INPUT_RECORD_FRAME].RStickY;
        //     (*state).Flags = P1_NPAD_STATES[INPUT_RECORD_FRAME].Flags;
        // }
    }
}
