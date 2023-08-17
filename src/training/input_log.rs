use crate::common::input::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;

use super::frame_counter;

static mut FRAME_COUNTER: usize = 0;

pub fn init() {
    unsafe {
        FRAME_COUNTER = frame_counter::register_counter();
        frame_counter::start_counting(FRAME_COUNTER);
    }
}

pub const NUM_LOGS: usize = 10;

#[derive(PartialEq, Eq)]
pub enum DirectionStrength {
    None,
    Weak,
    Strong,
}

#[derive(Copy, Clone, Default)]
pub struct InputLog {
    pub ttl: u32,
    pub frames: u32,
    pub raw_inputs: Controller,
    pub smash_inputs: MappedInputs,
}

fn bin_stick_values(x: f32, y: f32) -> (DirectionStrength, f32) {
    let angle = y.atan2(x).to_degrees();
    let length = (x * x + y * y).sqrt();
    (
        match length.abs() {
            x if x > 0.5 => DirectionStrength::Strong,
            x if x > 0.2 => DirectionStrength::Weak,
            _ => DirectionStrength::None,
        },
        angle,
    )
}

impl InputLog {
    pub fn is_smash_different(&self, other: &InputLog) -> bool {
        self.smash_inputs.buttons != other.smash_inputs.buttons
            || self.smash_binned_lstick() != other.smash_binned_lstick()
            || self.smash_binned_rstick() != other.smash_binned_rstick()
    }

    pub fn smash_binned_lstick(&self) -> (DirectionStrength, f32) {
        let x = self.smash_inputs.lstick_x as f32;
        let y = self.smash_inputs.lstick_y as f32;

        bin_stick_values(x, y)
    }

    pub fn smash_binned_rstick(&self) -> (DirectionStrength, f32) {
        let x = self.smash_inputs.rstick_x as f32;
        let y = self.smash_inputs.rstick_y as f32;

        bin_stick_values(x, y)
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
            let current_frame = frame_counter::get_frame_count(FRAME_COUNTER);

            let potential_input_log = InputLog {
                ttl: 600,
                frames: 1,
                raw_inputs: *controller_struct.controller,
                smash_inputs: *out,
            };

            let input_logs = &mut *P1_INPUT_LOGS.lock();
            let latest_input_log = input_logs.first_mut().unwrap();
            // Use different "different" function depending on menu option
            if latest_input_log.is_smash_different(&potential_input_log) {
                frame_counter::reset_frame_count(FRAME_COUNTER);
                // We should count this frame already
                frame_counter::tick_idx(FRAME_COUNTER);
                insert_in_front(input_logs, potential_input_log);
            } else {
                latest_input_log.frames = std::cmp::min(current_frame, 99);
            }

            // For the remainder, decrease TTL
            for input_log in input_logs.iter_mut().take(NUM_LOGS).skip(1) {
                if input_log.ttl > 0 {
                    input_log.ttl -= 1;
                }
            }
        }
    }
}
