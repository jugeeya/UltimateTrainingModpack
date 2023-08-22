static mut SHOULD_COUNT: Vec<bool> = vec![];
static mut NO_RESET: Vec<bool> = vec![];
static mut COUNTERS: Vec<u32> = vec![];

fn _register_counter(no_reset: bool) -> usize {
    unsafe {
        let index = COUNTERS.len();

        COUNTERS.push(0);
        SHOULD_COUNT.push(false);
        NO_RESET.push(no_reset);

        index
    }
}

pub fn register_counter_no_reset() -> usize {
    _register_counter(true)
}

pub fn register_counter() -> usize {
    _register_counter(false)
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
    reset_frame_count(index);
    stop_counting(index);
}

/**
 * Returns true until a certain number of frames have passed
 */
pub fn should_delay(delay: u32, index: usize) -> bool {
    if delay == 0 {
        return false;
    }

    let current_frame = get_frame_count(index);

    if current_frame == 0 {
        start_counting(index);
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

pub fn tick_idx(index: usize) {
    unsafe {
        COUNTERS[index] += 1;
    }
}

pub fn tick() {
    unsafe {
        for (index, _frame) in COUNTERS.iter().enumerate() {
            if !SHOULD_COUNT[index] {
                continue;
            }
            tick_idx(index);
        }
    }
}

pub fn reset_all() {
    unsafe {
        for (index, _frame) in COUNTERS.iter().enumerate() {
            if NO_RESET[index] {
                continue;
            }
            full_reset(index);
        }
    }
}
