use crate::common::*;
use crate::events::{Event, EVENT_QUEUE};
use crate::training::frame_counter;
use crate::logging::*;

use ramhorns::Template;
use skyline::info::get_program_id;
use skyline::nn::hid::NpadGcState;
use skyline::nn::web::WebSessionBootMode;
use skyline_web::{Background, BootDisplay, WebSession, Webpage};
use std::fs;
use std::path::Path;
use training_mod_consts::{MenuJsonStruct, TrainingModpackMenu};

static mut FRAME_COUNTER_INDEX: usize = 0;
pub static mut QUICK_MENU_FRAME_COUNTER_INDEX: usize = 0;
const MENU_LOCKOUT_FRAMES: u32 = 15;
pub static mut QUICK_MENU_ACTIVE: bool = false;

pub fn init() {
    unsafe {
        FRAME_COUNTER_INDEX = frame_counter::register_counter();
        QUICK_MENU_FRAME_COUNTER_INDEX = frame_counter::register_counter();
        write_menu();
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

pub unsafe fn write_menu() {
    let tpl = Template::new(include_str!("../templates/menu.html")).unwrap();

    let overall_menu = get_menu();

    let data = tpl.render(&overall_menu);

    // Now that we have the html, write it to file
    // From skyline-web
    let program_id = get_program_id();
    let htdocs_dir = "training_modpack";
    let menu_html_path = Path::new("sd:/atmosphere/contents")
        .join(format!("{program_id:016X}"))
        .join(format!("manual_html/html-document/{htdocs_dir}.htdocs/"))
        .join("training_menu.html");
    fs::write(menu_html_path, data).expect("Failed to write menu HTML file");
}

const MENU_CONF_PATH: &str = "sd:/TrainingModpack/training_modpack_menu.json";

pub unsafe fn set_menu_from_json(message: &str) {
    let web_response = serde_json::from_str::<MenuJsonStruct>(message);
    let tui_response = serde_json::from_str::<TrainingModpackMenu>(message);
    info!("Received menu message: {message}");
    if let Ok(message_json) = web_response {
        // Includes both MENU and DEFAULTS_MENU
        // From Web Applet
        MENU = message_json.menu;
        DEFAULTS_MENU = message_json.defaults_menu;
        std::fs::write(
            MENU_CONF_PATH,
            serde_json::to_string_pretty(&message_json).unwrap(),
        )
        .expect("Failed to write menu settings file from web response");
    } else if let Ok(message_json) = tui_response {
        // Only includes MENU
        // From TUI
        MENU = message_json;

        let conf = MenuJsonStruct {
            menu: MENU,
            defaults_menu: DEFAULTS_MENU,
        };
        std::fs::write(MENU_CONF_PATH, serde_json::to_string_pretty(&conf).unwrap())
            .expect("Failed to write menu settings file from tui response");
    } else {
        skyline::error::show_error(
            0x70,
            "Could not parse the menu response!\nPlease send a screenshot of the details page to the developers.\n\0",
            &format!("{message:#?}\0")
        );
    };
    if MENU.quick_menu == OnOff::Off && is_emulator() {
        skyline::error::show_error(
            0x69,
            "Cannot use web menu on emulator.\n\0",
            "Only the quick menu is runnable via emulator currently.\n\0",
        );
        MENU.quick_menu = OnOff::On;
    }
    EVENT_QUEUE.push(Event::menu_open(message.to_string()));
}

pub fn spawn_menu() {
    unsafe {
        frame_counter::reset_frame_count(FRAME_COUNTER_INDEX);
        frame_counter::start_counting(FRAME_COUNTER_INDEX);
        frame_counter::reset_frame_count(QUICK_MENU_FRAME_COUNTER_INDEX);
        frame_counter::start_counting(QUICK_MENU_FRAME_COUNTER_INDEX);

        if MENU.quick_menu == OnOff::Off {
            #[cfg(feature = "web_session_preload")]
            {
                WEB_MENU_ACTIVE = true;
            }

            #[cfg(not(feature = "web_session_preload"))]
            {
                spawn_web_session(new_web_session(false));
            }
        } else {
            let mut app = QUICK_MENU_APP.lock();
            *app = training_mod_tui::App::new(get_menu());
            drop(app);
            QUICK_MENU_ACTIVE = true;
        }
    }
}

pub struct ButtonPresses {
    pub a: ButtonPress,
    pub b: ButtonPress,
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

use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    pub static ref QUICK_MENU_APP: Mutex<training_mod_tui::App<'static>> =
        Mutex::new(training_mod_tui::App::new(unsafe { get_menu() }));
}

pub unsafe fn quick_menu_loop() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let backend = training_mod_tui::TestBackend::new(75, 15);
        let mut terminal = training_mod_tui::Terminal::new(backend).unwrap();
        let mut has_slept_millis = 0;
        let render_frames = 5;
        let mut json_response = String::new();
        let button_presses = &mut BUTTON_PRESSES;
        let mut received_input = true;
        loop {
            std::thread::sleep(std::time::Duration::from_millis(16));
            has_slept_millis += 16;
            if has_slept_millis < 16 * render_frames {
                continue;
            }
            has_slept_millis = 0;

            if !QUICK_MENU_ACTIVE {
                continue;
            }

            let mut app = QUICK_MENU_APP.lock();
            button_presses.a.read_press().then(|| {
                app.on_a();
                received_input = true;
            });
            let b_press = &mut button_presses.b;
            b_press.read_press().then(|| {
                received_input = true;
                if !app.outer_list {
                    app.on_b()
                } else if frame_counter::get_frame_count(QUICK_MENU_FRAME_COUNTER_INDEX) == 0
                    && !json_response.is_empty()
                {
                    // Leave menu.
                    QUICK_MENU_ACTIVE = false;
                    set_menu_from_json(&json_response);
                }
            });
            button_presses.zl.read_press().then(|| {
                app.on_l();
                received_input = true;
            });
            button_presses.zr.read_press().then(|| {
                app.on_r();
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
                terminal
                    .draw(|f| json_response = training_mod_tui::ui(f, &mut app))
                    .unwrap();
                received_input = false;
            }
            drop(app);
        }
    }
}

static mut WEB_MENU_ACTIVE: bool = false;

unsafe fn spawn_web_session(session: WebSession) {
    info!("Opening menu session...");
    let loaded_msg = session.recv();
    info!(
        "Received loaded message from web: {}",
        &loaded_msg
    );
    let message_send = MenuJsonStruct {
        menu: MENU,
        defaults_menu: DEFAULTS_MENU,
    };
    session.send_json(&message_send);
    let message_recv = session.recv();
    info!("Tearing down Training Modpack menu session");
    session.exit();
    session.wait_for_exit();
    set_menu_from_json(&message_recv);
}

unsafe fn new_web_session(hidden: bool) -> WebSession {
    Webpage::new()
        .background(Background::BlurredScreenshot)
        .boot_icon(true)
        .boot_display(BootDisplay::BlurredScreenshot)
        .htdocs_dir("training_modpack")
        .start_page("training_menu.html")
        .open_session(if hidden {
            WebSessionBootMode::InitiallyHidden
        } else {
            WebSessionBootMode::Default
        })
        .unwrap()
}

pub unsafe fn web_session_loop() {
    // Don't query the FighterManager too early otherwise it will crash...
    std::thread::sleep(std::time::Duration::new(30, 0)); // sleep for 30 secs on bootup
    let mut web_session: Option<WebSession> = None;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        if (is_ready_go() || entry_count() > 0) && is_training_mode() {
            if web_session.is_some() {
                if WEB_MENU_ACTIVE {
                    spawn_web_session(web_session.unwrap());
                    web_session = None;
                    WEB_MENU_ACTIVE = false;
                }
            } else {
                // TODO
                // Starting a new session causes some ingame lag.
                // Investigate whether we can minimize this lag by
                // waiting until the player is idle or using CPU boost mode
                info!("Starting new menu session...");
                web_session = Some(new_web_session(true));
            }
        } else {
            // No longer in training mode, tear down the session.
            // This will avoid conflicts with other web plugins, and helps with stability.
            // Having the session open too long, especially if the switch has been put to sleep, can cause freezes
            if let Some(web_session_to_kill) = web_session {
                info!("Tearing down Training Modpack menu session");
                web_session_to_kill.exit();
                web_session_to_kill.wait_for_exit();
            }
            web_session = None;
        }
    }
}
