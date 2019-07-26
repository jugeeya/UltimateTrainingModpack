#pragma once
#include "common.hpp"

typedef struct FrameInput {
    int cat1_flag;
    int cat2_flag;
    int cat3_flag;
    int cat4_flag;
    int pad_flag;
    float stick_x;
    float stick_y;
} FrameInput;

#define NUM_FRAME_INPUTS 60
FrameInput frame_inputs[NUM_FRAME_INPUTS];
int curr_frame = 0;

#define NUM_PRE_FRAME 60
int curr_pre_frame = 0;

#define INPUT_PRE_RECORDING 1
#define INPUT_RECORDING 2
#define INPUT_PLAYBACK 3
int INPUT_RECORD_STATE = NONE;

namespace InputRecorder {

int get_command_flag_cat(u64 module_accessor, int category, int flag, bool& replace) {
    if (is_training_mode()) {
        if (is_operation_cpu(module_accessor)) {
            if (INPUT_RECORD_STATE == INPUT_PRE_RECORDING)
                ; // Set color overlay to red
            
            if (INPUT_RECORD_STATE == INPUT_RECORDING || 
                INPUT_RECORD_STATE == INPUT_PLAYBACK) {
                if (INPUT_RECORD_STATE == INPUT_RECORDING)
                    ; // Set color overlay to blue
                else {
                    ; // Reset color overlay
                    replace = true;
                    if (category == 0)
                        return frame_inputs[curr_frame].cat1_flag;
                    else if (category == 1)
                        return frame_inputs[curr_frame].cat2_flag;
                    else if (category == 2)
                        return frame_inputs[curr_frame].cat3_flag;
                    else if (category == 3)
                        return frame_inputs[curr_frame].cat4_flag;
                }
            }
        } else {
            if (INPUT_RECORD_STATE == NONE) {
                if (ControlModule::check_button_on(module_accessor, CONTROL_PAD_BUTTON_CATCH) &&
                    ControlModule::check_button_trigger(module_accessor, CONTROL_PAD_BUTTON_APPEAL_S_L)) {
                    print_string(module_accessor, "PRERECORD");
                    INPUT_RECORD_STATE = INPUT_PRE_RECORDING;
                }
            } else if (INPUT_RECORD_STATE == INPUT_PRE_RECORDING) {
                if (category == FIGHTER_PAD_COMMAND_CATEGORY1) {
                    curr_pre_frame++;
                    if (curr_pre_frame == NUM_PRE_FRAME - 1) {
                        print_string(module_accessor, "RECORDING");
                        INPUT_RECORD_STATE = INPUT_RECORDING;
                        curr_pre_frame = 0;
                    }
                }
            } else if (INPUT_RECORD_STATE == INPUT_RECORDING) {
                if (category == FIGHTER_PAD_COMMAND_CATEGORY1) {
                    curr_frame++;
                    frame_inputs[curr_frame] = FrameInput{
                        flag,
                        ControlModule::get_command_flag_cat(module_accessor, 1),
                        ControlModule::get_command_flag_cat(module_accessor, 2),
                        ControlModule::get_command_flag_cat(module_accessor, 3),
                        ControlModule::get_pad_flag(module_accessor),
                        ControlModule::get_stick_x(module_accessor),
                        ControlModule::get_stick_y(module_accessor),
                    };

                    if (curr_frame == NUM_FRAME_INPUTS - 1) {
                        print_string(module_accessor, "PLAYBACK");
                        INPUT_RECORD_STATE = INPUT_PLAYBACK;
                        curr_frame = 0;
                    }
                }
            } else if (INPUT_RECORD_STATE == INPUT_PLAYBACK) {
                if (ControlModule::check_button_on(module_accessor, CONTROL_PAD_BUTTON_CATCH) &&
                    ControlModule::check_button_trigger(module_accessor, CONTROL_PAD_BUTTON_APPEAL_S_R)) {
                    print_string(module_accessor, "STOP");
                    INPUT_RECORD_STATE = NONE;
                    for (size_t i = 0; i < NUM_FRAME_INPUTS; i++)
                        frame_inputs[i] = FrameInput{};
                    curr_frame = 0;
                }

                if (category == FIGHTER_PAD_COMMAND_CATEGORY1) {
                    curr_frame = (curr_frame + 1) % NUM_FRAME_INPUTS;
                }
            }
        }
    }

    replace = false;
    return 0;
}

int get_pad_flag(u64 module_accessor, bool& replace) {
    if (is_training_mode() && is_operation_cpu(module_accessor)) {
        if (INPUT_RECORD_STATE == INPUT_RECORDING ||
            INPUT_RECORD_STATE == INPUT_PLAYBACK) {
            replace = true;
            return frame_inputs[curr_frame].pad_flag;
        }
    }

    replace = false;
    return 0;
}

float get_stick_x(u64 module_accessor, bool& replace) {
    if (is_training_mode() && is_operation_cpu(module_accessor)) {
        if (INPUT_RECORD_STATE == INPUT_RECORDING || 
            INPUT_RECORD_STATE == INPUT_PLAYBACK) {
            replace = true;
            return frame_inputs[curr_frame].stick_x;
        }
    }

    replace = false;
    return 0;
}

float get_stick_y(u64 module_accessor, bool& replace) {
    if (is_training_mode() && is_operation_cpu(module_accessor)) {
        if (INPUT_RECORD_STATE == INPUT_RECORDING ||
            INPUT_RECORD_STATE == INPUT_PLAYBACK) {
            replace = true;
            return frame_inputs[curr_frame].stick_y;
        }
    }

    replace = false;
    return 0;
}
}