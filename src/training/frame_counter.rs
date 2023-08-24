pub enum FrameCounterType {
    InGame,
    // "Reset" occurs when we enter training mode and when we run L+R+A or save state load
    // Some frame counters need in-game frames that do not reset when this occurs
    InGameNoReset,
    Real
}

pub struct FrameCounter {
    count: u32,
    should_count: bool,
    counter_type: FrameCounterType,
}

static mut COUNTERS: Vec<FrameCounter> = vec![];

pub fn register_counter(counter_type: FrameCounterType) -> usize {
    unsafe {
        let index = COUNTERS.len();

        COUNTERS.push(FrameCounter{
            count: 0,
            should_count: false,
            counter_type: counter_type
        })

        index
    }
}

pub fn start_counting(index: usize) {
    unsafe {
        COUNTERS[index].should_count = true;
    }
}

pub fn stop_counting(index: usize) {
    unsafe {
        COUNTERS[index].should_count = false;
    }
}

pub fn reset_frame_count(index: usize) {
    unsafe {
        COUNTERS[index].count = 0;
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
    unsafe { COUNTERS[index].count }
}

pub fn tick_idx(index: usize) {
    unsafe {
        COUNTERS[index].count += 1;
    }
}

pub fn tick_ingame() {
    unsafe {
        for (index, counter) in COUNTERS.iter().enumerate() {
            if !counter.should_count || counter.counter_type == FrameCounterType::Real {
                continue;
            }
            tick_idx(index);
        }
    }
}

pub fn tick_real() {
    unsafe {
        for (index, counter) in COUNTERS.iter().enumerate() {
            if !counter.should_count || (counter.counter_type == FrameCounterType::InGame || counter.counter_type == FrameCounterType::InGameNoReset)  {
                continue;
            }
            tick_idx(index);
        }
    }
}

pub fn reset_all() {
    unsafe {
        for (index, counter) in COUNTERS.iter().enumerate() {
            if counter.counter_type != FrameCounterType::InGame {
                continue;
            }
            full_reset(index);
        }
    }
}
