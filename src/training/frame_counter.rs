use crate::common::*;
use crate::training::*;

static mut COUNTERS: Vec<FrameCounter> = vec![];

#[derive(Clone, Copy)]
pub struct FrameCounter {
    counter: u32,
    should_count: bool
}

impl FrameCounter {
    pub fn new() -> FrameCounter {
        let counter = FrameCounter {
            counter: 0,
            should_count: false
        };
        
        unsafe {
            COUNTERS.push(counter);
        }
        counter
    }

    pub fn get_frame_count(&self) -> u32 {
        self.counter
    }

    pub fn tick(&mut self) {
        if self.should_count {
            self.counter += 1;
        }
    }

    pub fn start_counting(&mut self) {
        self.should_count = true;
    }

    pub fn stop_counting(&mut self) {
        self.should_count = false;
    }

    pub fn reset_frame_count(&mut self) {
        self.counter = 0
    }

    pub fn full_reset(&mut self) {
        self.reset_frame_count();
        self.stop_counting();
    }

    /**
    * Returns true until a certain number of frames have passed
    */
    pub fn should_delay(&mut self, delay: u32) -> bool {
        if delay == 0 {
            return false;
        }

        let current_frame = self.get_frame_count();

        if current_frame == 0 {
            self.start_counting();
        }

        if current_frame >= delay {
            self.full_reset();
            return false;
        }

        true
    }
}

fn tick() {
    unsafe {
        for counter in COUNTERS.iter() {
            counter.tick();
        }
    }
}

pub fn reset_all(){
    unsafe {
        for counter in COUNTERS.iter() {
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
