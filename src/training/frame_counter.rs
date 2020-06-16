use crate::common::*;
use crate::training::*;

static mut CURRENT_FRAME : u32 = 0;
static mut SHOULD_COUNT: bool = false;


pub unsafe fn start_counting(){
    SHOULD_COUNT = true;
}

pub unsafe fn stop_counting(){
    SHOULD_COUNT = false;
}

pub unsafe fn reset_frame_count(){
    CURRENT_FRAME = 0;
}

pub unsafe fn get_frame_count() -> u32 {
    CURRENT_FRAME
}

pub unsafe fn tick(){
    if SHOULD_COUNT{
        CURRENT_FRAME += 1;
        println!("Tick {}",CURRENT_FRAME);
    }
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) {
    if !is_training_mode() {
        return;
    }

    if !once_per_frame(module_accessor, category){
        return;
    }

    tick();
}