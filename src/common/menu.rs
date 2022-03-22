use crate::common::*;
use crate::events::{Event, EVENT_QUEUE};
use crate::training::frame_counter;
use crate::common::consts::get_menu_from_url;
use ramhorns::Template;
use skyline::info::get_program_id;
use skyline_web::{Background, BootDisplay, Webpage};
use smash::lib::lua_const::*;
use std::fs;
use std::path::Path;
use crate::mkdir;

static mut FRAME_COUNTER_INDEX: usize = 0;
const MENU_LOCKOUT_FRAMES: u32 = 15;
// TODO: Revert
pub static mut QUICK_MENU_ACTIVE: bool = true;

pub fn init() {
    unsafe {
        FRAME_COUNTER_INDEX = frame_counter::register_counter();
        write_menu();
    }
}

pub unsafe fn menu_condition(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    // Only check for button combination if the counter is 0 (not locked out)
    match frame_counter::get_frame_count(FRAME_COUNTER_INDEX) {
        0 => {
            ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                && ControlModule::check_button_on_trriger(
                    module_accessor,
                    *CONTROL_PAD_BUTTON_APPEAL_HI,
                )
        }
        1..MENU_LOCKOUT_FRAMES => false,
        _ => {
            // Waited longer than the lockout time, reset the counter so the menu can be opened again
            frame_counter::full_reset(FRAME_COUNTER_INDEX);
            false
        }
    }
}

pub unsafe fn write_menu() {
    let tpl = Template::new(include_str!("../templates/menu.html")).unwrap();

    let overall_menu = get_menu();

    let data = tpl.render(&overall_menu);

    // Now that we have the html, write it to file
    // From skyline-web
    let program_id = get_program_id();
    let htdocs_dir = "training_modpack";
    let menu_dir_path = Path::new("sd:/atmosphere/contents")
        .join(&format!("{:016X}", program_id))
        .join(&format!("manual_html/html-document/{}.htdocs/", htdocs_dir));

    let menu_html_path = menu_dir_path
        .join("training_menu.html");

    mkdir(menu_dir_path.to_str().unwrap().as_bytes().as_ptr(), 777);
    let write_resp = fs::write(menu_html_path, data);
    if write_resp.is_err() {
        println!("Error!: {}", write_resp.err().unwrap());
    }
}

const MENU_CONF_PATH: &str = "sd:/TrainingModpack/training_modpack_menu.conf";

pub fn set_menu_from_url(orig_last_url: &str) {
    let last_url = &orig_last_url.replace("&save_defaults=1", "");
    unsafe {
        MENU = get_menu_from_url(MENU, last_url);

        if MENU.quick_menu == OnOff::Off {
            if is_emulator() {
                skyline::error::show_error(
                    0x69,
                    "Cannot use web menu on emulator.\n\0",
                    "Only the quick menu is runnable via emulator currently.\n\0",
                );
                MENU.quick_menu = OnOff::On;
            }
        }
    }

    if last_url.len() != orig_last_url.len() {
        // Save as default
        unsafe {
            DEFAULT_MENU = MENU;
            write_menu();
        }
        let menu_defaults_conf_path = "sd:/TrainingModpack/training_modpack_menu_defaults.conf";
        std::fs::write(menu_defaults_conf_path, last_url)
            .expect("Failed to write default menu conf file");
    }

    std::fs::write(MENU_CONF_PATH, last_url).expect("Failed to write menu conf file");
    unsafe {
        EVENT_QUEUE.push(Event::menu_open(last_url.to_string()));
    }
}

pub fn spawn_menu() {
    unsafe {
        frame_counter::reset_frame_count(FRAME_COUNTER_INDEX);
        frame_counter::start_counting(FRAME_COUNTER_INDEX);
    }

    let mut quick_menu = false;
    unsafe {
        if MENU.quick_menu == OnOff::On {
            quick_menu = true;
        }
    }

    if !quick_menu {
        let fname = "training_menu.html";
        let params = unsafe { MENU.to_url_params() };
        let page_response = Webpage::new()
            .background(Background::BlurredScreenshot)
            .htdocs_dir("training_modpack")
            .boot_display(BootDisplay::BlurredScreenshot)
            .boot_icon(true)
            .start_page(&format!("{}{}", fname, params))
            .open()
            .unwrap();

        let orig_last_url = page_response.get_last_url().unwrap();

        set_menu_from_url(orig_last_url);
    } else {
        unsafe {
            QUICK_MENU_ACTIVE = true;
        }
    }
}

use skyline::nn::hid::NpadGcState;

pub struct ButtonPresses {
    pub a: ButtonPress,
    pub b: ButtonPress,
    pub zr: ButtonPress,
    pub zl: ButtonPress,
    pub left: ButtonPress,
    pub right: ButtonPress,
    pub up: ButtonPress,
    pub down: ButtonPress
}

pub struct ButtonPress {
    pub prev_frame_is_pressed: bool,
    pub is_pressed: bool,
    pub lockout_frames: usize
}

impl ButtonPress {
    pub fn default() -> ButtonPress {
        ButtonPress{
            prev_frame_is_pressed: false,
            is_pressed: false,
            lockout_frames: 0
        }
    }

    pub fn read_press(&mut self) -> bool {
        if self.is_pressed {
            self.is_pressed = false;
            if !self.prev_frame_is_pressed && self.lockout_frames == 0 {
                self.lockout_frames = 10;
                self.prev_frame_is_pressed = true;
                return true;
            }
        }

        if self.lockout_frames > 0 {
            self.lockout_frames -= 1;
        }

        self.prev_frame_is_pressed = self.is_pressed;
        false
    }
}

impl ButtonPresses {
    pub fn default() -> ButtonPresses {
        ButtonPresses{
            a: ButtonPress::default(),
            b: ButtonPress::default(),
            zr: ButtonPress::default(),
            zl: ButtonPress::default(),
            left: ButtonPress::default(),
            right: ButtonPress::default(),
            up: ButtonPress::default(),
            down: ButtonPress::default()
        }
    }
}

pub static mut BUTTON_PRESSES : ButtonPresses = ButtonPresses{
    a: ButtonPress{prev_frame_is_pressed: false, is_pressed: false, lockout_frames: 0},
    b: ButtonPress{prev_frame_is_pressed: false, is_pressed: false, lockout_frames: 0},
    zr: ButtonPress{prev_frame_is_pressed: false, is_pressed: false, lockout_frames: 0},
    zl: ButtonPress{prev_frame_is_pressed: false, is_pressed: false, lockout_frames: 0},
    left: ButtonPress{prev_frame_is_pressed: false, is_pressed: false, lockout_frames: 0},
    right: ButtonPress{prev_frame_is_pressed: false, is_pressed: false, lockout_frames: 0},
    up: ButtonPress{prev_frame_is_pressed: false, is_pressed: false, lockout_frames: 0},
    down: ButtonPress{prev_frame_is_pressed: false, is_pressed: false, lockout_frames: 0},
};

pub fn handle_get_npad_state(state: *mut NpadGcState, _controller_id: *const u32) {
    unsafe {
        if menu::QUICK_MENU_ACTIVE {
            // TODO: This should make more sense, look into.
            // BUTTON_PRESSES.a.is_pressed = (*state).Buttons & (1 << 0) > 0;
            // BUTTON_PRESSES.b.is_pressed = (*state).Buttons & (1 << 1) > 0;
            // BUTTON_PRESSES.zl.is_pressed = (*state).Buttons & (1 << 8) > 0;
            // BUTTON_PRESSES.zr.is_pressed = (*state).Buttons & (1 << 9) > 0;
            // BUTTON_PRESSES.left.is_pressed = (*state).Buttons & ((1 << 12) | (1 << 16)) > 0;
            // BUTTON_PRESSES.right.is_pressed = (*state).Buttons & ((1 << 14) | (1 << 18)) > 0;
            // BUTTON_PRESSES.down.is_pressed = (*state).Buttons & ((1 << 15) | (1 << 19)) > 0;
            // BUTTON_PRESSES.up.is_pressed = (*state).Buttons & ((1 << 13) | (1 << 17)) > 0;

            if frame_counter::get_frame_count(FRAME_COUNTER_INDEX) != 0 {
                return;
            }

            if (*state).Buttons & (1 << 0) > 0 {
                BUTTON_PRESSES.a.is_pressed = true;
            }
            if (*state).Buttons & (1 << 1) > 0 {
                BUTTON_PRESSES.b.is_pressed = true;
            }
            if (*state).Buttons & (1 << 8) > 0 {
                BUTTON_PRESSES.zl.is_pressed = true;
            }
            if (*state).Buttons & (1 << 9) > 0 {
                BUTTON_PRESSES.zr.is_pressed = true;
            }
            if (*state).Buttons & ((1 << 12) | (1 << 16)) > 0 {
                BUTTON_PRESSES.left.is_pressed = true;
            }
            if (*state).Buttons & ((1 << 14) | (1 << 18)) > 0 {
                BUTTON_PRESSES.right.is_pressed = true;
            }
            if (*state).Buttons & ((1 << 15) | (1 << 19)) > 0 {
                BUTTON_PRESSES.down.is_pressed = true;
            }
            if (*state).Buttons & ((1 << 13) | (1 << 17)) > 0 {
                BUTTON_PRESSES.up.is_pressed = true;
            }

            // If we're here, remove all other Npad presses...
            // Should we exclude the home button?
            (*state) = NpadGcState::default();
        }
    }
}

