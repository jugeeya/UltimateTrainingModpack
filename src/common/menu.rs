use crate::common::consts::get_menu_from_url;
use crate::common::*;
use crate::events::{Event, EVENT_QUEUE};
use crate::training::frame_counter;

use owo_colors::OwoColorize;
use ramhorns::Template;
use skyline::info::get_program_id;
use skyline::nn::hid::NpadGcState;
use skyline::nn::web::WebSessionBootMode;
use skyline_web::{Background, BootDisplay, WebSession, Webpage};
use smash::lib::lua_const::*;
use std::fs;
use std::path::Path;
use training_mod_consts::WebAppletResponse;
use training_mod_tui::Color;

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
    let menu_html_path = Path::new("sd:/atmosphere/contents")
        .join(&format!("{:016X}", program_id))
        .join(&format!("manual_html/html-document/{}.htdocs/", htdocs_dir))
        .join("training_menu.html");

    let write_resp = fs::write(menu_html_path, data);
    if write_resp.is_err() {
        println!("Error!: {}", write_resp.err().unwrap());
    }
}

const MENU_CONF_PATH: &str = "sd:/TrainingModpack/training_modpack_menu.conf";

pub unsafe fn set_menu_from_url(last_url: &str) {
    // TODO: Remove this function
    // Or maybe keep it in for compatibility with existing settings files upon upgrade?
    MENU = get_menu_from_url(MENU, last_url, false);
    DEFAULTS_MENU = get_menu_from_url(MENU, last_url, true);

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
    println!(
        "serialized menu:\n{}",
        serde_json::to_string_pretty(&MENU).unwrap()
    );
    std::fs::write(MENU_CONF_PATH, last_url).expect("Failed to write menu conf file");
    EVENT_QUEUE.push(Event::menu_open(last_url.to_string()));
}

pub unsafe fn set_menu_from_json(message: &str) {
    if let Ok(message_json) = serde_json::from_str::<WebAppletResponse>(message) {
        MENU = message_json.menu;
        DEFAULTS_MENU = message_json.defaults_menu;
    } else {
        panic!("Could not read the menu response!\n{}", message);
    };
}

pub fn spawn_menu() {
    unsafe {
        frame_counter::reset_frame_count(FRAME_COUNTER_INDEX);
        frame_counter::start_counting(FRAME_COUNTER_INDEX);
        frame_counter::reset_frame_count(QUICK_MENU_FRAME_COUNTER_INDEX);
        frame_counter::start_counting(QUICK_MENU_FRAME_COUNTER_INDEX);

        if MENU.quick_menu == OnOff::Off {
            SHOULD_SHOW_MENU = true;
        } else {
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
    pub fn default() -> ButtonPress {
        ButtonPress {
            prev_frame_is_pressed: false,
            is_pressed: false,
            lockout_frames: 0,
        }
    }

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

extern "C" {
    #[link_name = "render_text_to_screen"]
    pub fn render_text_to_screen_cstr(str: *const skyline::libc::c_char);

    #[link_name = "set_should_display_text_to_screen"]
    pub fn set_should_display_text_to_screen(toggle: bool);
}

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr()
    };
}

pub fn render_text_to_screen(s: &str) {
    unsafe {
        render_text_to_screen_cstr(c_str!(s));
    }
}

pub unsafe fn quick_menu_loop() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let menu = consts::get_menu();

        let mut app = training_mod_tui::App::new(menu);

        let backend = training_mod_tui::TestBackend::new(75, 15);
        let mut terminal = training_mod_tui::Terminal::new(backend).unwrap();

        let mut has_slept_millis = 0;
        let render_frames = 5;
        let mut url = String::new();
        let button_presses = &mut menu::BUTTON_PRESSES;
        let mut received_input = true;
        loop {
            button_presses.a.read_press().then(|| {
                app.on_a();
                received_input = true;
            });
            let b_press = &mut button_presses.b;
            b_press.read_press().then(|| {
                received_input = true;
                if !app.outer_list {
                    app.on_b()
                } else if frame_counter::get_frame_count(menu::QUICK_MENU_FRAME_COUNTER_INDEX) == 0
                {
                    // Leave menu.
                    menu::QUICK_MENU_ACTIVE = false;
                    menu::set_menu_from_url(url.as_str()); // TODO update to json
                    println!("URL: {}", url.as_str());
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

            std::thread::sleep(std::time::Duration::from_millis(16));
            has_slept_millis += 16;
            if has_slept_millis < 16 * render_frames {
                continue;
            }
            has_slept_millis = 16;
            if !menu::QUICK_MENU_ACTIVE {
                app = training_mod_tui::App::new(consts::get_menu());
                set_should_display_text_to_screen(false);
                continue;
            }
            if !received_input {
                continue;
            }
            let mut view = String::new();

            let frame_res = terminal
                .draw(|f| url = training_mod_tui::ui(f, &mut app))
                .unwrap();

            use std::fmt::Write;
            for (i, cell) in frame_res.buffer.content().iter().enumerate() {
                match cell.fg {
                    Color::Black => write!(&mut view, "{}", &cell.symbol.black()),
                    Color::Blue => write!(&mut view, "{}", &cell.symbol.blue()),
                    Color::LightBlue => write!(&mut view, "{}", &cell.symbol.bright_blue()),
                    Color::Cyan => write!(&mut view, "{}", &cell.symbol.cyan()),
                    Color::LightCyan => write!(&mut view, "{}", &cell.symbol.cyan()),
                    Color::Red => write!(&mut view, "{}", &cell.symbol.red()),
                    Color::LightRed => write!(&mut view, "{}", &cell.symbol.bright_red()),
                    Color::LightGreen => write!(&mut view, "{}", &cell.symbol.bright_green()),
                    Color::Green => write!(&mut view, "{}", &cell.symbol.green()),
                    Color::Yellow => write!(&mut view, "{}", &cell.symbol.yellow()),
                    Color::LightYellow => write!(&mut view, "{}", &cell.symbol.bright_yellow()),
                    Color::Magenta => write!(&mut view, "{}", &cell.symbol.magenta()),
                    Color::LightMagenta => {
                        write!(&mut view, "{}", &cell.symbol.bright_magenta())
                    }
                    _ => write!(&mut view, "{}", &cell.symbol),
                }
                .unwrap();
                if i % frame_res.area.width as usize == frame_res.area.width as usize - 1 {
                    writeln!(&mut view).unwrap();
                }
            }
            writeln!(&mut view).unwrap();

            render_text_to_screen(view.as_str());
            received_input = false;
        }
    }
}

static mut SHOULD_SHOW_MENU: bool = false;

pub unsafe fn web_session_loop() {
    let mut web_session: Option<WebSession> = None;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        if is_training_mode() {
            if web_session.is_some() {
                if SHOULD_SHOW_MENU {
                    println!("[Training Modpack] Opening menu session...");
                    let session = web_session.unwrap();
                    let message_send = WebAppletResponse {
                        menu: MENU,
                        defaults_menu: DEFAULTS_MENU
                    };
                    session.send_json(&message_send);
                    println!("[Training Modpack] Sending message:\n{}", serde_json::to_string_pretty(&message_send).unwrap());
                    session.show();
                    let message_recv = session.recv();
                    println!("[Training Modpack] Received menu from web:\n{}", &message_recv);
                    println!("[Training Modpack] Tearing down Training Modpack menu session");
                    session.exit();
                    session.wait_for_exit();
                    web_session = None;
                    set_menu_from_json(&message_recv);
                    SHOULD_SHOW_MENU = false;
                }
            } else {
                println!("[Training Modpack] Starting new menu session...");
                web_session = Some(
                    Webpage::new()
                        .background(Background::BlurredScreenshot)
                        .htdocs_dir("training_modpack")
                        .start_page("training_menu.html")
                        .open_session(WebSessionBootMode::InitiallyHidden)
                        .unwrap(),
                );

            }
        } else {
            // This isn't working...I think that is_training_mode() isn't becoming false once the training match exits.
            if web_session.is_some() {
                println!("[Training Modpack] Tearing down Training Modpack menu session");
                web_session.unwrap().exit();
            }
            web_session = None;
        }
    }
}
