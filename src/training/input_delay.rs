use std::collections::VecDeque;

use crate::common::input::*;

use training_mod_sync::*;

use crate::common::MENU;

static P1_DELAYED_INPUT_MAPPINGS: RwLock<VecDeque<MappedInputs>> = RwLock::new(VecDeque::new());

pub unsafe fn handle_final_input_mapping(player_idx: i32, out: *mut MappedInputs) {
    if player_idx == 0 {
        let mut delayed_mappings = lock_write(&P1_DELAYED_INPUT_MAPPINGS);
        let actual_mapping = *out;
        if delayed_mappings.len() < read(&MENU).input_delay.into_delay() as usize {
            *out = MappedInputs::empty();
        } else if let Some(delayed_mapping) = delayed_mappings.back() {
            *out = *delayed_mapping;
        }
        delayed_mappings.push_front(actual_mapping);
        delayed_mappings.truncate(read(&MENU).input_delay.into_delay() as usize);
    }
}
