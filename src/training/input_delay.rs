use crate::common::MENU;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use skyline::nn::hid::{GetNpadStyleSet, NpadGcState};
use std::collections::VecDeque;

lazy_static! {
    static ref P1_DELAYED_NPAD_STATES: Mutex<VecDeque<NpadGcState>> = Mutex::new(VecDeque::new());
}

pub unsafe fn p1_controller_id() -> u32 {
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

pub fn handle_get_npad_state(state: *mut NpadGcState, controller_id: *const u32) {
    unsafe {
        if crate::common::is_training_mode() {
            if *controller_id == p1_controller_id() {
                let mut delayed_states = P1_DELAYED_NPAD_STATES.lock();
                let actual_state = *state;

                if delayed_states.len() < MENU.input_delay.into_delay() as usize {
                    let update_count = (*state).updateCount;
                    let attributes = (*state).Flags;
                    *state = NpadGcState::default();
                    (*state).updateCount = update_count;
                    (*state).Flags = attributes;
                } else if let Some(delayed_state) = delayed_states.back() {
                    let update_count = (*state).updateCount;
                    let attributes = (*state).Flags;
                    *state = *delayed_state;
                    (*state).updateCount = update_count;
                    (*state).Flags = attributes;
                }

                delayed_states.push_front(actual_state);
                delayed_states.truncate(MENU.input_delay.into_delay() as usize);
            }
        }
    }
}
