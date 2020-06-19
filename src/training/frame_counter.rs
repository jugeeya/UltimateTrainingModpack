use crate::common::*;
use crate::training::*;

static mut SHOULD_COUNT: Vec<bool> = vec![];
static mut COUNTERS: Vec<u32> = vec![];

pub unsafe fn register_counter() -> usize {
    let index = COUNTERS.len();

    COUNTERS.push(0);
    SHOULD_COUNT.push(false);

    index
}

pub unsafe fn start_counting(index:usize) {
    SHOULD_COUNT[index] = true;
}

pub unsafe fn stop_counting(index:usize) {
    SHOULD_COUNT[index] =  false;
}

pub unsafe fn reset_frame_count(index:usize) {
    COUNTERS[index] = 0;
}

pub unsafe fn get_frame_count(index:usize) -> u32 {
    COUNTERS[index]
}

pub unsafe fn tick() {
    for (index, _frame) in COUNTERS.iter().enumerate() {
        if !SHOULD_COUNT[index]{
            continue;
        }
        COUNTERS[index] += 1;
        println!("Tick {}", COUNTERS[index]);
    }
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) {
    if !is_training_mode() {
        return;
    }

    if !once_per_frame(module_accessor, category) {
        return;
    }

    tick();
}
