use crate::sync::*;

static COUNTERS: RwLock<Vec<FrameCounter>> = RwLock::new(vec![]);
#[derive(PartialEq, Eq)]
pub enum FrameCounterType {
    InGame,
    // "Reset" occurs when we enter training mode and when we run L+R+A or save state load
    // Some frame counters need in-game frames that do not reset when this occurs
    InGameNoReset,
    Real,
}

pub struct FrameCounter {
    count: u32,
    should_count: bool,
    counter_type: FrameCounterType,
}

pub fn register_counter(counter_type: FrameCounterType) -> usize {
    let mut counters_guard = lock_write_rwlock(&COUNTERS);
    let index = (*counters_guard).len();
    (*counters_guard).push(FrameCounter {
        count: 0,
        should_count: false,
        counter_type,
    });
    index
}

pub fn start_counting(index: usize) {
    let mut counters_guard = lock_write_rwlock(&COUNTERS);
    (*counters_guard)[index].should_count = true;
}

pub fn stop_counting(index: usize) {
    let mut counters_guard = lock_write_rwlock(&COUNTERS);
    (*counters_guard)[index].should_count = false;
}

pub fn is_counting(index: usize) -> bool {
    let counters_guard = lock_read_rwlock(&COUNTERS);
    (*counters_guard)[index].should_count

}

pub fn reset_frame_count(index: usize) {
    let mut counters_guard = lock_write_rwlock(&COUNTERS);
    (*counters_guard)[index].count = 0;
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
    let counters_guard = lock_read_rwlock(&COUNTERS);
    (*counters_guard)[index].count
}

pub fn tick_idx(index: usize) {
    let mut counters_guard = lock_write_rwlock(&COUNTERS);
    (*counters_guard)[index].count += 1;
}

pub fn tick_ingame() {
    let mut counters_guard = lock_write_rwlock(&COUNTERS);
    for counter in (*counters_guard).iter_mut() {
        if !counter.should_count || counter.counter_type == FrameCounterType::Real {
            continue;
        }
        // same as full_reset, but we already have the lock so we can't lock again
        counter.count += 1;
    }
}

pub fn tick_real() {
    let mut counters_guard = lock_write_rwlock(&COUNTERS);
    for counter in (*counters_guard).iter_mut() {
        if !counter.should_count
            || (counter.counter_type == FrameCounterType::InGame
                || counter.counter_type == FrameCounterType::InGameNoReset)
        {
            continue;
        }
        // same as full_reset, but we already have the lock so we can't lock again
        counter.count += 1;
    }
}

pub fn reset_all() {
    let mut counters_guard = lock_write_rwlock(&COUNTERS);
    for counter in (*counters_guard).iter_mut() {
        if counter.counter_type != FrameCounterType::InGame {
            continue;
        }
        // same as full_reset, but we already have the lock so we can't lock again
        counter.count = 0;
        counter.should_count = false;
    }
}
