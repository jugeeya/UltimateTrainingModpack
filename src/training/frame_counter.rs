use crate::common::*;
use crate::training::*;

static mut COUNTERS: Vec<FrameCounter> = vec![];

pub struct FrameCounter {
    counter: u32,
    should_count: bool
}

impl FrameCounter {
    fn new() -> FrameCounter {
        COUNTERS.push(self);
        self.counter = 0;
        self.should_count = false;
        self
    }

    pub fn get_frame_count(&self) -> u32 {
        self.counter
    }

    pub fn tick(&self) {
        if self.should_count {
            self.counter += 1;
        }
    }

    pub fn start_counting(&self) {
        self.should_count = true;
    }

    pub fn stop_counting(&self) {
        self.should_count = false;
    }

    pub fn reset_frame_count(&self) {
        self.counter = 0
    }

    pub fn full_reset(&self) {
        self.reset_frame_count();
        self.stop_counting();
    }

    /**
    * Returns true until a certain number of frames have passed
    */
    pub fn should_delay(&self, delay: u32) -> bool {
        if delay == 0 {
            return false;
        }

        let current_frame = self.get_frame_count(index);

        if current_frame == 0 {
            self.start_counting(index);
        }

        if current_frame >= delay {
            self.full_reset(index);
            return false;
        }

        true
    }
}

fn tick() {
    unsafe {
        for counter in COUNTERS {
            counter.tick();
        }
    }
}

pub fn reset_all(){
    unsafe {
        for counter in COUNTERS {
            counter.full_reset();
        }
    }
}

pub fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) {
    if !is_operation_cpu(module_accessor) {
        return;
    }

    tick();
}
