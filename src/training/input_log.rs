use std::collections::VecDeque;

use crate::common::input::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    pub static ref P1_INPUT_MAPPINGS: Mutex<VecDeque<MappedInputs>> = Mutex::new(VecDeque::new());
}

// TODO: how many
const NUM_INPUTS: usize = 120;

pub fn handle_final_input_mapping(player_idx: i32, out: *mut MappedInputs) {
    unsafe {
        if player_idx == 0 {
            let mut mappings = P1_INPUT_MAPPINGS.lock();

            mappings.push_front(*out);
            mappings.truncate(NUM_INPUTS);
        }
    }
}
