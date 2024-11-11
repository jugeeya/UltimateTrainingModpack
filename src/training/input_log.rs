use itertools::Itertools;
use std::collections::VecDeque;

use crate::common::input::*;
use crate::menu::QUICK_MENU_ACTIVE;
use training_mod_sync::*;
use crate::try_get_module_accessor;
use skyline::nn::ui2d::ResColor;
use smash::app::{lua_bind::*, utility};
use training_mod_consts::{FighterId, InputDisplay, MENU};

use super::{frame_counter, input_record::STICK_CLAMP_MULTIPLIER};

const GREEN: ResColor = ResColor {
    r: 22,
    g: 156,
    b: 0,
    a: 0,
};

const RED: ResColor = ResColor {
    r: 153,
    g: 10,
    b: 10,
    a: 0,
};

const CYAN: ResColor = ResColor {
    r: 0,
    g: 255,
    b: 255,
    a: 0,
};

const BLUE: ResColor = ResColor {
    r: 0,
    g: 40,
    b: 108,
    a: 0,
};

const PURPLE: ResColor = ResColor {
    r: 100,
    g: 66,
    b: 202,
    a: 0,
};

pub const YELLOW: ResColor = ResColor {
    r: 230,
    g: 180,
    b: 14,
    a: 0,
};

pub const WHITE: ResColor = ResColor {
    r: 255,
    g: 255,
    b: 255,
    a: 0,
};

pub static PER_LOG_FRAME_COUNTER: LazyLock<usize> = LazyLock::new(|| {
    frame_counter::register_counter(frame_counter::FrameCounterType::InGameNoReset)
});
pub static OVERALL_FRAME_COUNTER: LazyLock<usize> = LazyLock::new(|| {
    frame_counter::register_counter(frame_counter::FrameCounterType::InGameNoReset)
});

pub const NUM_LOGS: usize = 15;
pub static DRAW_LOG_BASE_IDX: RwLock<usize> = RwLock::new(0);

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

impl PartialEq for InputLog {
    fn eq(&self, other: &Self) -> bool {
        self.frames == other.frames && !self.is_different(other)
    }
}
impl Eq for InputLog {}

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
                _ => return (DirectionStrength::None, 0.0),
            },
            // Y only
            (x, y) if x.abs() < WALK_THRESHOLD_X => match y {
                y if y > DEADZONE_THRESHOLD_Y => 90.0,
                y if y < -DEADZONE_THRESHOLD_Y => 270.0,
                _ => return (DirectionStrength::None, 0.0),
            },
            // Positive Y
            (x, y) if y > DEADZONE_THRESHOLD_Y => match x {
                x if x > WALK_THRESHOLD_X => 45.0,
                x if x < -WALK_THRESHOLD_X => 135.0,
                _ => return (DirectionStrength::Weak, 90.0),
            },
            // Negative Y
            (x, y) if y < DEADZONE_THRESHOLD_Y => match x {
                x if x > WALK_THRESHOLD_X => 315.0,
                x if x < -WALK_THRESHOLD_X => 225.0,
                _ => return (DirectionStrength::Weak, 270.0),
            },
            _ => return (DirectionStrength::None, 0.0),
        },
    )
}

impl InputLog {
    pub fn is_different(&self, other: &InputLog) -> bool {
        unsafe {
            match MENU.input_display {
                InputDisplay::SMASH => self.is_smash_different(other),
                InputDisplay::RAW => self.is_raw_different(other),
                InputDisplay::STATUS => self.is_status_different(other),
                InputDisplay::NONE => false,
                _ => panic!("Invalid value in is_different: {}", MENU.input_display),
            }
        }
    }

    pub fn binned_lstick(&self) -> (DirectionStrength, f32) {
        unsafe {
            match MENU.input_display {
                InputDisplay::SMASH => self.smash_binned_lstick(),
                InputDisplay::RAW => self.raw_binned_lstick(),
                InputDisplay::STATUS => (DirectionStrength::None, 0.0),
                InputDisplay::NONE => panic!("Invalid input display to log"),
                _ => panic!("Invalid value in binned_lstick: {}", MENU.input_display),
            }
        }
    }

    pub fn binned_rstick(&self) -> (DirectionStrength, f32) {
        unsafe {
            match MENU.input_display {
                InputDisplay::SMASH => self.smash_binned_rstick(),
                InputDisplay::RAW => self.raw_binned_rstick(),
                InputDisplay::STATUS => (DirectionStrength::None, 0.0),
                InputDisplay::NONE => panic!("Invalid input display to log"),
                _ => panic!("Invalid value in binned_rstick: {}", MENU.input_display),
            }
        }
    }

    pub fn button_icons(&self) -> VecDeque<(&str, ResColor)> {
        unsafe {
            match MENU.input_display {
                InputDisplay::SMASH => self.smash_button_icons(),
                InputDisplay::RAW => self.raw_button_icons(),
                InputDisplay::STATUS => VecDeque::new(),
                InputDisplay::NONE => panic!("Invalid input display to log"),
                _ => unreachable!(),
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
                    Buttons::ATTACK | Buttons::ATTACK_RAW => ("a", GREEN),
                    Buttons::SPECIAL | Buttons::SPECIAL_RAW2 => ("b", RED),
                    Buttons::JUMP => ("x", CYAN),
                    Buttons::GUARD | Buttons::GUARD_HOLD => ("lb", BLUE),
                    Buttons::CATCH => ("zr", PURPLE),
                    Buttons::STOCK_SHARE => ("plus", WHITE),
                    Buttons::APPEAL_HI => ("dpad_up", WHITE),
                    Buttons::APPEAL_LW => ("dpad_down", WHITE),
                    Buttons::APPEAL_SL => ("dpad_right", WHITE),
                    Buttons::APPEAL_SR => ("dpad_left", WHITE),
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
            icons.push_front(("a", GREEN));
        }
        if buttons.b() {
            icons.push_front(("b", RED));
        }
        if buttons.x() {
            icons.push_front(("x", CYAN));
        }
        if buttons.y() {
            icons.push_front(("y", CYAN));
        }
        if buttons.l() || buttons.real_digital_l() {
            icons.push_front(("lb", BLUE));
        }
        if buttons.r() || buttons.real_digital_r() {
            icons.push_front(("rb", BLUE));
        }
        if buttons.zl() {
            icons.push_front(("zl", PURPLE));
        }
        if buttons.zr() {
            icons.push_front(("zr", PURPLE));
        }
        if buttons.plus() {
            icons.push_front(("plus", WHITE));
        }
        if buttons.minus() {
            icons.push_front(("minus", WHITE));
        }
        if buttons.dpad_up() {
            icons.push_front(("dpad_up", WHITE));
        }
        if buttons.dpad_down() {
            icons.push_front(("dpad_down", WHITE));
        }
        if buttons.dpad_left() {
            icons.push_front(("dpad_left", WHITE));
        }
        if buttons.dpad_right() {
            icons.push_front(("dpad_right", WHITE));
        }

        icons
    }

    fn is_smash_different(&self, other: &InputLog) -> bool {
        self.smash_inputs.buttons != other.smash_inputs.buttons
            || self.smash_binned_lstick() != other.smash_binned_lstick()
            || self.smash_binned_rstick() != other.smash_binned_rstick()
            || (unsafe { MENU.input_display_status.as_bool() } && self.status != other.status)
    }

    fn is_status_different(&self, other: &InputLog) -> bool {
        unsafe {
            let input_display_status = MENU.input_display_status.as_bool();
            input_display_status && (self.status != other.status)
        }
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
            || (unsafe { MENU.input_display_status.as_bool() } && self.status != other.status)
    }

    fn raw_binned_lstick(&self) -> (DirectionStrength, f32) {
        let x = (self.raw_inputs.left_stick_x / STICK_CLAMP_MULTIPLIER) as i8;
        let y = (self.raw_inputs.left_stick_y / STICK_CLAMP_MULTIPLIER) as i8;
        bin_stick_values(x, y)
    }

    fn raw_binned_rstick(&self) -> (DirectionStrength, f32) {
        let x = (self.raw_inputs.right_stick_x / STICK_CLAMP_MULTIPLIER) as i8;
        let y = (self.raw_inputs.right_stick_y / STICK_CLAMP_MULTIPLIER) as i8;
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

pub static P1_INPUT_LOGS: LazyLock<RwLock<[InputLog; NUM_LOGS]>> =
    LazyLock::new(|| RwLock::new([InputLog::default(); NUM_LOGS]));

pub fn handle_final_input_mapping(
    player_idx: i32,
    controller_struct: &SomeControllerStruct,
    out: *mut MappedInputs,
) {
    unsafe {
        if MENU.input_display == InputDisplay::NONE {
            return;
        }

        if read_rwlock(&QUICK_MENU_ACTIVE) {
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

            let mut input_logs_guard = lock_write_rwlock(&(*P1_INPUT_LOGS));
            let input_logs = &mut *input_logs_guard;
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
                let mut draw_log_base_idx_guard = lock_write_rwlock(&DRAW_LOG_BASE_IDX);
                *draw_log_base_idx_guard = (*draw_log_base_idx_guard + 1) % NUM_LOGS;
                drop(draw_log_base_idx_guard);
            } else if is_new_frame {
                *latest_input_log = potential_input_log;
                latest_input_log.frames = std::cmp::min(current_frame, 99);
                latest_input_log.ttl = prev_ttl;
            }

            // Decrease TTL
            for input_log in input_logs.iter_mut() {
                if input_log.ttl > 0 && is_new_frame {
                    input_log.ttl -= 1;
                }
            }
        }
    }
}
