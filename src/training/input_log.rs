use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::VecDeque;

use crate::common::{input::*, menu::QUICK_MENU_ACTIVE, try_get_module_accessor};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use skyline::nn::ui2d::ResColor;
use smash::app::{lua_bind::*, utility};
use training_mod_consts::{FighterId, InputDisplay, MENU};

use super::{frame_counter, input_record::STICK_CLAMP_MULTIPLIER};

const GREEN: ResColor = ResColor {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};

const RED: ResColor = ResColor {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

const CYAN: ResColor = ResColor {
    r: 0,
    g: 255,
    b: 255,
    a: 255,
};

const BLUE: ResColor = ResColor {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};

const PURPLE: ResColor = ResColor {
    r: 255,
    g: 0,
    b: 255,
    a: 255,
};

pub const YELLOW: ResColor = ResColor {
    r: 255,
    g: 255,
    b: 0,
    a: 255,
};

pub const WHITE: ResColor = ResColor {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

pub static PER_LOG_FRAME_COUNTER: Lazy<usize> =
    Lazy::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGameNoReset));
pub static OVERALL_FRAME_COUNTER: Lazy<usize> =
    Lazy::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGameNoReset));

pub const NUM_LOGS: usize = 10;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum DirectionStrength {
    None,
    Weak,
    // Strong,
}

#[derive(Copy, Clone, Default)]
pub struct InputLog {
    pub ttl: u32,
    pub frames: u32,
    pub overall_frame: u32,
    pub raw_inputs: Controller,
    pub smash_inputs: MappedInputs,
    pub status: i32,
    pub fighter_kind: i32,
}

const WALK_THRESHOLD_X: i8 = 20;
const _DASH_THRESHOLD_X: i8 = 102;
const DEADZONE_THRESHOLD_Y: i8 = 30;
const _TAP_JUMP_THRESHOLD_Y: i8 = 90;

fn bin_stick_values(x: i8, y: i8) -> (DirectionStrength, f32) {
    (
        // TODO
        DirectionStrength::Weak,
        match (x, y) {
            // X only
            (x, y) if y.abs() < DEADZONE_THRESHOLD_Y => match x {
                x if x > WALK_THRESHOLD_X => 0.0,
                x if x < -WALK_THRESHOLD_X => 180.0,
                _ => return (DirectionStrength::None, 0.0)
            },
            // Y only
            (x, y) if x.abs() < WALK_THRESHOLD_X => match x {
                x if x > DEADZONE_THRESHOLD_Y => 90.0,
                x if x < -DEADZONE_THRESHOLD_Y => 270.0,
                _ => return (DirectionStrength::None, 0.0)
            },
            // Positive Y
            (x, y) if y > DEADZONE_THRESHOLD_Y => match x {
                x if x > WALK_THRESHOLD_X => 45.0,
                x if x < -WALK_THRESHOLD_X => 135.0,
                _ => return (DirectionStrength::Weak, 90.0)
            },
            // Negative Y
            (x, y) if y < DEADZONE_THRESHOLD_Y => match x {
                x if x > WALK_THRESHOLD_X => 315.0,
                x if x < -WALK_THRESHOLD_X => 225.0,
                _ => return (DirectionStrength::Weak, 270.0)
            },
            _ => return (DirectionStrength::None, 0.0)
        },
    )
}

impl InputLog {
    pub fn is_different(&self, other: &InputLog) -> bool {
        unsafe {
            match MENU.input_display {
                InputDisplay::Smash => self.is_smash_different(other),
                InputDisplay::Raw => self.is_raw_different(other),
                InputDisplay::None => false,
            }
        }
    }

    pub fn binned_lstick(&self) -> (DirectionStrength, f32) {
        unsafe {
            match MENU.input_display {
                InputDisplay::Smash => self.smash_binned_lstick(),
                InputDisplay::Raw => self.raw_binned_lstick(),
                InputDisplay::None => panic!("Invalid input display to log"),
            }
        }
    }

    pub fn binned_rstick(&self) -> (DirectionStrength, f32) {
        unsafe {
            match MENU.input_display {
                InputDisplay::Smash => self.smash_binned_rstick(),
                InputDisplay::Raw => self.raw_binned_rstick(),
                InputDisplay::None => panic!("Invalid input display to log"),
            }
        }
    }

    pub fn button_icons(&self) -> VecDeque<(&str, ResColor)> {
        unsafe {
            match MENU.input_display {
                InputDisplay::Smash => self.smash_button_icons(),
                InputDisplay::Raw => self.raw_button_icons(),
                InputDisplay::None => panic!("Invalid input display to log"),
            }
        }
    }

    fn smash_button_icons(&self) -> VecDeque<(&str, ResColor)> {
        self.smash_inputs
            .buttons
            .to_vec()
            .iter()
            .filter_map(|button| {
                Some(match *button {
                    Buttons::ATTACK | Buttons::ATTACK_RAW => ("A", GREEN),
                    Buttons::SPECIAL | Buttons::SPECIAL_RAW | Buttons::SPECIAL_RAW2 => ("B", RED),
                    Buttons::JUMP => ("X", CYAN),
                    Buttons::GUARD | Buttons::GUARD_HOLD => ("L", BLUE),
                    Buttons::CATCH => ("ZR", PURPLE),
                    Buttons::STOCK_SHARE => ("+", WHITE),
                    Buttons::APPEAL_HI => ("^", WHITE),
                    Buttons::APPEAL_LW => ("v", WHITE),
                    Buttons::APPEAL_SL => (">", WHITE),
                    Buttons::APPEAL_SR => ("<", WHITE),
                    _ => return None,
                })
            })
            .unique_by(|(s, _)| *s)
            .collect::<VecDeque<(&str, ResColor)>>()
    }

    fn raw_button_icons(&self) -> VecDeque<(&str, ResColor)> {
        let buttons = self.raw_inputs.current_buttons;
        let mut icons = VecDeque::new();
        if buttons.a() {
            icons.push_front(("A", GREEN));
        }
        if buttons.b() {
            icons.push_front(("B", RED));
        }
        if buttons.x() {
            icons.push_front(("X", CYAN));
        }
        if buttons.y() {
            icons.push_front(("Y", CYAN));
        }
        if buttons.l() || buttons.real_digital_l() {
            icons.push_front(("L", BLUE));
        }
        if buttons.r() || buttons.real_digital_r() {
            icons.push_front(("R", BLUE));
        }
        if buttons.zl() {
            icons.push_front(("ZL", PURPLE));
        }
        if buttons.zr() {
            icons.push_front(("ZR", PURPLE));
        }
        if buttons.plus() {
            icons.push_front(("+", WHITE));
        }
        if buttons.minus() {
            icons.push_front(("-", WHITE));
        }

        icons
    }

    fn is_smash_different(&self, other: &InputLog) -> bool {
        self.smash_inputs.buttons != other.smash_inputs.buttons
            || self.smash_binned_lstick() != other.smash_binned_lstick()
            || self.smash_binned_rstick() != other.smash_binned_rstick()
            || (MENU.input_display_status.as_bool() && self.status != other.status)
    }

    fn smash_binned_lstick(&self) -> (DirectionStrength, f32) {
        bin_stick_values(self.smash_inputs.lstick_x, self.smash_inputs.lstick_y)
    }

    fn smash_binned_rstick(&self) -> (DirectionStrength, f32) {
        bin_stick_values(self.smash_inputs.rstick_x, self.smash_inputs.rstick_y)
    }

    fn is_raw_different(&self, other: &InputLog) -> bool {
        self.raw_inputs.current_buttons != other.raw_inputs.current_buttons
            || self.raw_binned_lstick() != other.raw_binned_lstick()
            || self.raw_binned_rstick() != other.raw_binned_rstick()
            || (MENU.input_display_status.as_bool() && self.status != other.status)
    }

    fn raw_binned_lstick(&self) -> (DirectionStrength, f32) {
        let x = (self.raw_inputs.left_stick_x / STICK_CLAMP_MULTIPLIER) as i8;
        let y = (self.raw_inputs.left_stick_y / STICK_CLAMP_MULTIPLIER) as i8;
        bin_stick_values(x, y)
    }

    fn raw_binned_rstick(&self) -> (DirectionStrength, f32) {
        let x = (self.raw_inputs.left_stick_x / STICK_CLAMP_MULTIPLIER) as i8;
        let y = (self.raw_inputs.left_stick_y / STICK_CLAMP_MULTIPLIER) as i8;
        bin_stick_values(x, y)
    }
}

fn insert_in_place<T>(array: &mut [T], value: T, index: usize) {
    array[index..].rotate_right(1);
    array[index] = value;
}

fn insert_in_front<T>(array: &mut [T], value: T) {
    insert_in_place(array, value, 0);
}

lazy_static! {
    pub static ref P1_INPUT_LOGS: Mutex<[InputLog; NUM_LOGS]> =
        Mutex::new([InputLog::default(); NUM_LOGS]);
}

pub fn handle_final_input_mapping(
    player_idx: i32,
    controller_struct: &SomeControllerStruct,
    out: *mut MappedInputs,
) {
    unsafe {
        if MENU.input_display == InputDisplay::None {
            return;
        }

        if QUICK_MENU_ACTIVE {
            return;
        }

        if player_idx == 0 {
            let module_accessor = try_get_module_accessor(FighterId::Player);
            if module_accessor.is_none() {
                return;
            }
            let module_accessor = module_accessor.unwrap();

            let current_frame = frame_counter::get_frame_count(*PER_LOG_FRAME_COUNTER);
            let current_overall_frame = frame_counter::get_frame_count(*OVERALL_FRAME_COUNTER);
            // We should always be counting
            frame_counter::start_counting(*PER_LOG_FRAME_COUNTER);
            frame_counter::start_counting(*OVERALL_FRAME_COUNTER);

            let potential_input_log = InputLog {
                ttl: 600,
                frames: 1,
                overall_frame: current_overall_frame,
                raw_inputs: *controller_struct.controller,
                smash_inputs: *out,
                status: StatusModule::status_kind(module_accessor),
                fighter_kind: utility::get_kind(&mut *module_accessor),
            };

            let input_logs = &mut *P1_INPUT_LOGS.lock();
            let latest_input_log = input_logs.first_mut().unwrap();
            let prev_overall_frames = latest_input_log.overall_frame;
            let prev_ttl = latest_input_log.ttl;
            // Only update if we are on a new frame according to the latest log
            let is_new_frame = prev_overall_frames != current_overall_frame;
            if is_new_frame && latest_input_log.is_different(&potential_input_log) {
                frame_counter::reset_frame_count(*PER_LOG_FRAME_COUNTER);
                // We should count this frame already
                frame_counter::tick_idx(*PER_LOG_FRAME_COUNTER);
                insert_in_front(input_logs, potential_input_log);
            } else if is_new_frame {
                *latest_input_log = potential_input_log;
                latest_input_log.frames = std::cmp::min(current_frame, 99);
                latest_input_log.ttl = prev_ttl;
            }

            // Decrease TTL
            for input_log in input_logs.iter_mut().take(NUM_LOGS) {
                if input_log.ttl > 0 && is_new_frame {
                    input_log.ttl -= 1;
                }
            }
        }
    }
}
