

static mut CURRENT_FRAME : u8 = 0;
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

pub unsafe fn get_frame_count() -> u8 {
    CURRENT_FRAME
}

pub unsafe fn tick(){
    if SHOULD_COUNT{
        CURRENT_FRAME += 1;
    }
}