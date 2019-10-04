#pragma once
#include "common.hpp"

#define SAVE_STATE 1
#define DEFAULT 2
#define CAMERA_MOVE 3
#define POS_MOVE 4

int save_state_player_state = DEFAULT;
int save_state_cpu_state = DEFAULT;
bool save_state_move_alert = false;

float save_state_x_player = 0;
float save_state_y_player = 0;
float save_state_percent_player = 0;
float save_state_lr_player = 1.0;
int save_state_situation_kind_player = 0;

float save_state_x_cpu = 0;
float save_state_y_cpu = 0;
float save_state_percent_cpu = 0;
float save_state_lr_cpu = 1.0;
int save_state_situation_kind_cpu = 0;

void save_states(u64 module_accessor) {
    int status = StatusModule::status_kind(module_accessor);
    if (is_training_mode()) {
        float* save_state_x;
        float* save_state_y;
        float* save_state_percent;
        float* save_state_lr;
        int* save_state_situation_kind;
        int* save_state;
        if (is_operation_cpu(module_accessor)) {
            save_state_x = &save_state_x_cpu;
            save_state_y = &save_state_y_cpu;
            save_state_percent = &save_state_percent_cpu;
            save_state_lr = &save_state_lr_cpu;
            save_state_situation_kind = &save_state_situation_kind_cpu;
            save_state = &save_state_cpu_state;
        } else {
            save_state_x = &save_state_x_player;
            save_state_y = &save_state_y_player;
            save_state_percent = &save_state_percent_player;
            save_state_lr = &save_state_lr_player;
            save_state_situation_kind = &save_state_situation_kind_player;
            save_state = &save_state_player_state;
        }

        // Grab + Dpad up: reset state
        if (ControlModule::check_button_on(module_accessor, CONTROL_PAD_BUTTON_CATCH) &&
            ControlModule::check_button_trigger(module_accessor, CONTROL_PAD_BUTTON_APPEAL_HI)) {
            if (*save_state == DEFAULT) {
                save_state_player_state = CAMERA_MOVE;
                save_state_cpu_state = CAMERA_MOVE;
            }
        }

        // move to camera bounds
        if (*save_state == CAMERA_MOVE) {
            *save_state = POS_MOVE;

            float left_right = (*save_state_x > 0) - (*save_state_x < 0);
            float y_pos = 0;
            if (*save_state_situation_kind == SITUATION_KIND_GROUND)
                y_pos = -50;

            Vector3f pos = {.x = left_right * 50, .y = y_pos, .z = 0};
            PostureModule::set_pos(module_accessor, &pos);
            StatusModule::set_situation_kind(module_accessor, SITUATION_KIND_AIR, 0);
        }

        // move to correct pos
        if (*save_state == POS_MOVE) {
            Vector3f pos = {.x = *save_state_x, .y = *save_state_y, .z = 0};
            PostureModule::set_pos(module_accessor, &pos);
            PostureModule::set_lr(module_accessor, *save_state_lr);
            DamageModule::add_damage(
                module_accessor,
                -1.0 * DamageModule::damage(module_accessor, 0), 0);
            DamageModule::add_damage(module_accessor, *save_state_percent, 0);

            StatusModule::set_situation_kind(module_accessor, *save_state_situation_kind, 0);

            // Doesn't work, and I don't know why yet.
            if (*save_state_situation_kind == SITUATION_KIND_GROUND && status != FIGHTER_STATUS_KIND_WAIT)
                    StatusModule::change_status_request(module_accessor, FIGHTER_STATUS_KIND_WAIT, 0); 
            else if (*save_state_situation_kind == SITUATION_KIND_AIR && status != FIGHTER_STATUS_KIND_FALL)
                    StatusModule::change_status_request(module_accessor, FIGHTER_STATUS_KIND_FALL, 0); 
            else if (*save_state_situation_kind == SITUATION_KIND_CLIFF && status != FIGHTER_STATUS_KIND_CLIFF_CATCH)
                    StatusModule::change_status_request(module_accessor, FIGHTER_STATUS_KIND_CLIFF_CATCH, 0);
            
            *save_state = DEFAULT;
        }

        // Grab + Dpad down: Save state
        if (ControlModule::check_button_on(module_accessor, CONTROL_PAD_BUTTON_CATCH) &&
            ControlModule::check_button_trigger(module_accessor, CONTROL_PAD_BUTTON_APPEAL_LW)) {
            save_state_player_state = SAVE_STATE;
            save_state_cpu_state = SAVE_STATE;
        }

        if (*save_state == SAVE_STATE) {
            *save_state = DEFAULT;

            *save_state_x = PostureModule::pos_x(module_accessor);
            *save_state_y = PostureModule::pos_y(module_accessor);
            *save_state_lr = PostureModule::lr(module_accessor);
            *save_state_percent = DamageModule::damage(module_accessor, 0);
            *save_state_situation_kind = StatusModule::situation_kind(module_accessor);
        }
    }
}