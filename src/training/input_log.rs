use crate::common::input::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;

pub const NUM_LOGS: usize = 10;

#[derive(Copy, Clone, Default)]
pub struct InputLog {
    pub frames: u64,
    pub raw_inputs: Controller,
    pub smash_inputs: MappedInputs,
}

impl InputLog {
    pub fn is_different(&self, other: &InputLog) -> bool {
        // TODO: Vary raw vs smash based on menu option

        // TODO: Include direction binning checks
        self.smash_inputs.buttons != other.smash_inputs.buttons
    }
}

fn insert_in_place<T>(array: &mut [T], value: T, index: usize) {
    array[index..].rotate_right(1);
    array[index] = value;
}

fn insert_in_front<T>(array: &mut [T], value: T) {
    insert_in_place(array, value, 0);
}

lazy_static! {
    pub static ref P1_INPUT_LOGS: Mutex<[InputLog; NUM_LOGS]> =
        Mutex::new([InputLog::default(); NUM_LOGS]);
}

pub fn handle_final_input_mapping(
    player_idx: i32,
    controller_struct: &SomeControllerStruct,
    out: *mut MappedInputs,
) {
    unsafe {
        if player_idx == 0 {
            // TODO: Use frame counter to determine frame value

            let potential_input_log = InputLog {
                frames: 1,
                raw_inputs: *controller_struct.controller,
                smash_inputs: *out,
            };

            let input_logs = &mut *P1_INPUT_LOGS.lock();
            let latest_input_log = input_logs.first_mut();
            if latest_input_log.is_none() {
                insert_in_front(input_logs, potential_input_log);
                return;
            }

            let latest_input_log = latest_input_log.unwrap();
            if latest_input_log.is_different(&potential_input_log) {
                insert_in_front(input_logs, potential_input_log);
            } else {
                latest_input_log.frames = std::cmp::min(latest_input_log.frames + 1, 99);
            }
        }
    }
}
