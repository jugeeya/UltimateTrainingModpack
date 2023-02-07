use crate::common::*;
use crate::events::{Event, EVENT_QUEUE};
use crate::logging::*;
use crate::training::frame_counter;

use skyline::nn::hid::NpadGcState;
use training_mod_consts::MenuJsonStruct;

static mut FRAME_COUNTER_INDEX: usize = 0;
pub static mut QUICK_MENU_FRAME_COUNTER_INDEX: usize = 0;
const MENU_LOCKOUT_FRAMES: u32 = 15;
pub static mut QUICK_MENU_ACTIVE: bool = false;

pub fn init() {
    unsafe {
        FRAME_COUNTER_INDEX = frame_counter::register_counter();
        QUICK_MENU_FRAME_COUNTER_INDEX = frame_counter::register_counter();
    }
}

pub unsafe fn menu_condition(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    // also ensure quick menu is reset
    if frame_counter::get_frame_count(QUICK_MENU_FRAME_COUNTER_INDEX) > 60 {
        frame_counter::full_reset(QUICK_MENU_FRAME_COUNTER_INDEX);
    }

    // Only check for button combination if the counter is 0 (not locked out)
    match frame_counter::get_frame_count(FRAME_COUNTER_INDEX) {
        0 => button_config::combo_passes(module_accessor, button_config::ButtonCombo::OpenMenu),
        1..MENU_LOCKOUT_FRAMES => false,
        _ => {
            // Waited longer than the lockout time, reset the counter so the menu can be opened again
            frame_counter::full_reset(FRAME_COUNTER_INDEX);
            false
        }
    }
}

const MENU_CONF_PATH: &str = "sd:/TrainingModpack/training_modpack_menu.json";

pub unsafe fn set_menu_from_json(message: &str) {
    let response = serde_json::from_str::<MenuJsonStruct>(message);
    info!("Received menu message: {message}");
    if let Ok(message_json) = response {
        // Includes both MENU and DEFAULTS_MENU
        MENU = message_json.menu;
        DEFAULTS_MENU = message_json.defaults_menu;
        std::fs::write(
            MENU_CONF_PATH,
            serde_json::to_string_pretty(&message_json).unwrap(),
        )
        .expect("Failed to write menu settings file");
    } else {
        skyline::error::show_error(
            0x70,
            "Could not parse the menu response!\nPlease send a screenshot of the details page to the developers.\n\0",
            &format!("{message:#?}\0")
        );
    };
}

pub fn spawn_menu() {
    unsafe {
        frame_counter::reset_frame_count(FRAME_COUNTER_INDEX);
        frame_counter::start_counting(FRAME_COUNTER_INDEX);
        frame_counter::reset_frame_count(QUICK_MENU_FRAME_COUNTER_INDEX);
        frame_counter::start_counting(QUICK_MENU_FRAME_COUNTER_INDEX);

        let mut app = QUICK_MENU_APP.lock();
        *app = training_mod_tui::App::new(
            ui_menu(MENU),
            (ui_menu(DEFAULTS_MENU), serde_json::to_string(&DEFAULTS_MENU).unwrap()));
        drop(app);
        QUICK_MENU_ACTIVE = true;
    }
}

pub struct ButtonPresses {
    pub a: ButtonPress,
    pub b: ButtonPress,
    pub x: ButtonPress,
    pub r: ButtonPress,
    pub l: ButtonPress,
    pub zr: ButtonPress,
    pub zl: ButtonPress,
    pub left: ButtonPress,
    pub right: ButtonPress,
    pub up: ButtonPress,
    pub down: ButtonPress,
}

pub struct ButtonPress {
    pub prev_frame_is_pressed: bool,
    pub is_pressed: bool,
    pub lockout_frames: usize,
}

impl ButtonPress {
    pub fn read_press(&mut self) -> bool {
        let is_pressed = self.is_pressed;
        if self.is_pressed {
            self.is_pressed = false;
            if self.lockout_frames == 0 {
                self.prev_frame_is_pressed = true;
                self.lockout_frames = 10;
                return true;
            }
        }

        if self.lockout_frames > 0 {
            self.lockout_frames -= 1;
        }

        self.prev_frame_is_pressed = is_pressed;
        false
    }
}

pub static mut BUTTON_PRESSES: ButtonPresses = ButtonPresses {
    a: ButtonPress {
        prev_frame_is_pressed: false,
        is_pressed: false,
        lockout_frames: 0,
    },
    b: ButtonPress {
        prev_frame_is_pressed: false,
        is_pressed: false,
        lockout_frames: 0,
    },
    x: ButtonPress {
        prev_frame_is_pressed: false,
        is_pressed: false,
        lockout_frames: 0,
    },
    r: ButtonPress {
        prev_frame_is_pressed: false,
        is_pressed: false,
        lockout_frames: 0,
    },
    l: ButtonPress {
        prev_frame_is_pressed: false,
        is_pressed: false,
        lockout_frames: 0,
    },
    zr: ButtonPress {
        prev_frame_is_pressed: false,
        is_pressed: false,
        lockout_frames: 0,
    },
    zl: ButtonPress {
        prev_frame_is_pressed: false,
        is_pressed: false,
        lockout_frames: 0,
    },
    left: ButtonPress {
        prev_frame_is_pressed: false,
        is_pressed: false,
        lockout_frames: 0,
    },
    right: ButtonPress {
        prev_frame_is_pressed: false,
        is_pressed: false,
        lockout_frames: 0,
    },
    up: ButtonPress {
        prev_frame_is_pressed: false,
        is_pressed: false,
        lockout_frames: 0,
    },
    down: ButtonPress {
        prev_frame_is_pressed: false,
        is_pressed: false,
        lockout_frames: 0,
    },
};

pub fn handle_get_npad_state(state: *mut NpadGcState, _controller_id: *const u32) {
    unsafe {
        let update_count = (*state).updateCount;
        let flags = (*state).Flags;
        if QUICK_MENU_ACTIVE {
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
            if (*state).Buttons & (1 << 2) > 0 {
                BUTTON_PRESSES.x.is_pressed = true;
            }
            if (*state).Buttons & (1 << 6) > 0 {
                BUTTON_PRESSES.l.is_pressed = true;
            }
            if (*state).Buttons & (1 << 7) > 0 {
                BUTTON_PRESSES.r.is_pressed = true;
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
            (*state).updateCount = update_count;
            (*state).Flags = flags;
        }
    }
}

use lazy_static::lazy_static;
use parking_lot::Mutex;
use training_mod_tui::AppPage;

lazy_static! {
    pub static ref QUICK_MENU_APP: Mutex<training_mod_tui::App<'static>> =
        Mutex::new(training_mod_tui::App::new(
            unsafe { ui_menu(MENU) },
            unsafe { (ui_menu(DEFAULTS_MENU), serde_json::to_string(&DEFAULTS_MENU).unwrap())}
            )
        );
}

pub unsafe fn quick_menu_loop() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let button_presses = &mut BUTTON_PRESSES;
        let mut received_input = true;
        loop {
            std::thread::sleep(std::time::Duration::from_millis(16));

            if !QUICK_MENU_ACTIVE {
                continue;
            }

            let app = &mut *QUICK_MENU_APP.data_ptr();
            button_presses.a.read_press().then(|| {
                app.on_a();
                received_input = true;
            });
            let b_press = &mut button_presses.b;
            b_press.read_press().then(|| {
                received_input = true;
                if app.page != AppPage::SUBMENU {
                    app.on_b()
                } else if frame_counter::get_frame_count(QUICK_MENU_FRAME_COUNTER_INDEX) == 0
                {
                    // Leave menu.
                    QUICK_MENU_ACTIVE = false;
                    let menu_json = app.get_menu_selections();
                    set_menu_from_json(&menu_json);
                    EVENT_QUEUE.push(Event::menu_open(menu_json));
                }
            });
            button_presses.x.read_press().then(|| {
                app.on_x();
                received_input = true;
            });
            button_presses.l.read_press().then(|| {
                app.on_l();
                received_input = true;
            });
            button_presses.r.read_press().then(|| {
                app.on_r();
                received_input = true;
            });
            button_presses.zl.read_press().then(|| {
                app.on_zl();
                received_input = true;
            });
            button_presses.zr.read_press().then(|| {
                app.on_zr();
                received_input = true;
            });
            button_presses.left.read_press().then(|| {
                app.on_left();
                received_input = true;
            });
            button_presses.right.read_press().then(|| {
                app.on_right();
                received_input = true;
            });
            button_presses.up.read_press().then(|| {
                app.on_up();
                received_input = true;
            });
            button_presses.down.read_press().then(|| {
                app.on_down();
                received_input = true;
            });

            if received_input {
                received_input = false;
                set_menu_from_json(&app.get_menu_selections());
            }
        }
    }
}