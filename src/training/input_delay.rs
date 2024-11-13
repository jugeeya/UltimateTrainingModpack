use std::collections::VecDeque;

use crate::common::input::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;

use training_mod_sync::*;

use crate::common::MENU;

lazy_static! {
    static ref P1_DELAYED_INPUT_MAPPINGS: Mutex<VecDeque<MappedInputs>> =
        Mutex::new(VecDeque::new());
}

pub fn handle_final_input_mapping(player_idx: i32, out: *mut MappedInputs) {
    unsafe {
        if player_idx == 0 {
            let mut delayed_mappings = P1_DELAYED_INPUT_MAPPINGS.lock();
            let actual_mapping = *out;

            if delayed_mappings.len() < get(&MENU).input_delay.into_delay() as usize {
                *out = MappedInputs::empty();
            } else if let Some(delayed_mapping) = delayed_mappings.back() {
                *out = *delayed_mapping;
            }

            delayed_mappings.push_front(actual_mapping);
            delayed_mappings.truncate(get(&MENU).input_delay.into_delay() as usize);
        }
    }
}
