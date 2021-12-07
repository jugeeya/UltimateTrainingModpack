use crate::common::*;
use crate::training::*;

static mut SHOULD_COUNT: Vec<bool> = vec![];
static mut COUNTERS: Vec<u32> = vec![];

pub fn register_counter() -> usize {
    unsafe {
        let index = COUNTERS.len();

        COUNTERS.push(0);
        SHOULD_COUNT.push(false);

        index
    }
}

pub fn start_counting(index: usize) {
    unsafe {
        SHOULD_COUNT[index] = true;
    }
}

pub fn stop_counting(index: usize) {
    unsafe {
        SHOULD_COUNT[index] = false;
    }
}

pub fn reset_frame_count(index: usize) {
    unsafe {
        COUNTERS[index] = 0;
    }
}

pub fn full_reset(index: usize) {
    frame_counter::reset_frame_count(index);
    frame_counter::stop_counting(index);
}

/**
 * Returns true until a certain number of frames have passed
 */
pub fn should_delay(delay: u32, index: usize) -> bool {
    if delay == 0 {
        return false;
    }

    let current_frame = frame_counter::get_frame_count(index);

    if current_frame == 0 {
        frame_counter::start_counting(index);
    }

    if current_frame >= delay {
        full_reset(index);
        return false;
    }

    true
}

pub fn get_frame_count(index: usize) -> u32 {
    unsafe { COUNTERS[index] }
}

fn tick() {
    unsafe {
        for (index, _frame) in COUNTERS.iter().enumerate() {
            if !SHOULD_COUNT[index] {
                continue;
            }
            COUNTERS[index] += 1;
        }
    }
}

pub fn reset_all() {
    unsafe {
        for (index, _frame) in COUNTERS.iter().enumerate() {
            full_reset(index);
        }
    }
}

pub fn get_command_flag_cat(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if !is_operation_cpu(module_accessor) {
        return;
    }

    tick();
}
