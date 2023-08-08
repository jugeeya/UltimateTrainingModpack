use std::fs;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use skyline::nn::hid::GetNpadStyleSet;
use training_mod_consts::MenuJsonStruct;

use training_mod_tui::AppPage;

use crate::common::*;
use crate::consts::MENU_OPTIONS_PATH;
use crate::events::{Event, EVENT_QUEUE};
use crate::input::*;
use crate::logging::*;

// This is a special frame counter that will tick on draw()
// We'll count how long the menu has been open
pub static mut FRAME_COUNTER: u32 = 0;
const MENU_CLOSE_WAIT_FRAMES: u32 = 60;
pub static mut QUICK_MENU_ACTIVE: bool = false;

pub unsafe fn menu_condition() -> bool {
    button_config::combo_passes_exclusive(button_config::ButtonCombo::OpenMenu)
}

pub fn load_from_file() {
    info!("Checking for previous menu in {MENU_OPTIONS_PATH}...");
    if fs::metadata(MENU_OPTIONS_PATH).is_ok() {
        let menu_conf = fs::read_to_string(MENU_OPTIONS_PATH)
            .unwrap_or_else(|_| panic!("Could not remove {}", MENU_OPTIONS_PATH));
        if let Ok(menu_conf_json) = serde_json::from_str::<MenuJsonStruct>(&menu_conf) {
            unsafe {
                MENU = menu_conf_json.menu;
                DEFAULTS_MENU = menu_conf_json.defaults_menu;
                info!("Previous menu found. Loading...");
            }
        } else {
            warn!("Previous menu found but is invalid. Deleting...");
            fs::remove_file(MENU_OPTIONS_PATH).unwrap_or_else(|_| {
                panic!(
                    "{} has invalid schema but could not be deleted!",
                    MENU_OPTIONS_PATH
                )
            });
        }
    } else {
        info!("No previous menu file found.");
    }
}

pub unsafe fn set_menu_from_json(message: &str) {
    let response = serde_json::from_str::<MenuJsonStruct>(message);
    info!("Received menu message: {message}");
    if let Ok(message_json) = response {
        // Includes both MENU and DEFAULTS_MENU
        MENU = message_json.menu;
        DEFAULTS_MENU = message_json.defaults_menu;
        fs::write(
            MENU_OPTIONS_PATH,
            serde_json::to_string_pretty(&message_json).unwrap(),
        )
        .expect("Failed to write menu settings file");
    } else {
        skyline::error::show_error(
            0x70,
            "Could not parse the menu response!\nPlease send a screenshot of the details page to the developers.\n\0",
            &format!("{message:#?}\0"),
        );
    };
}

pub fn spawn_menu() {
    unsafe {
        FRAME_COUNTER = 0;
        QUICK_MENU_ACTIVE = true;
    }
}

lazy_static! {
    pub static ref QUICK_MENU_APP: Mutex<training_mod_tui::App<'static>> = Mutex::new(
        training_mod_tui::App::new(unsafe { ui_menu(MENU) }, unsafe {
            (
                ui_menu(DEFAULTS_MENU),
                serde_json::to_string(&DEFAULTS_MENU).unwrap(),
            )
        })
    );
    pub static ref P1_CONTROLLER_STATE: Mutex<Controller> = Mutex::new(Controller::default());
}

pub fn handle_final_input_mapping(
    player_idx: i32,
    controller_struct: &mut SomeControllerStruct,
    out: *mut MappedInputs,
) {
    unsafe {
        if player_idx == 0 {
            *P1_CONTROLLER_STATE.lock() = *controller_struct.controller;
            if QUICK_MENU_ACTIVE {
                // If we're here, remove all other presses
                *out = MappedInputs::default();
            }
        }
    }
}

pub unsafe fn quick_menu_loop() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let mut received_input = true;
        loop {
            std::thread::sleep(std::time::Duration::from_millis(16));

            if !QUICK_MENU_ACTIVE {
                continue;
            }

            // Check for all controllers unplugged
            let mut potential_controller_ids = (0..8).collect::<Vec<u32>>();
            potential_controller_ids.push(0x20);
            if potential_controller_ids
                .iter()
                .all(|i| GetNpadStyleSet(i as *const _).flags == 0)
            {
                QUICK_MENU_ACTIVE = false;
                continue;
            }

            let p1_controller_state = *P1_CONTROLLER_STATE.data_ptr();
            let is_gcc = p1_controller_state.style == ControllerStyle::GCController;
            let button_presses = p1_controller_state.just_down;

            let app = &mut *QUICK_MENU_APP.data_ptr();
            button_presses.a().then(|| {
                app.on_a();
                received_input = true;
            });
            button_presses.b().then(|| {
                received_input = true;
                if app.page != AppPage::SUBMENU {
                    app.on_b()
                } else if FRAME_COUNTER > MENU_CLOSE_WAIT_FRAMES {
                    // Leave menu.
                    QUICK_MENU_ACTIVE = false;
                    FRAME_COUNTER = 0;
                    let menu_json = app.get_menu_selections();
                    set_menu_from_json(&menu_json);
                    EVENT_QUEUE.push(Event::menu_open(menu_json));
                }
            });
            button_presses.x().then(|| {
                app.save_defaults();
                received_input = true;
            });
            button_presses.y().then(|| {
                app.reset_all_submenus();
                received_input = true;
            });
            (button_presses.l() || button_presses.real_digital_l()).then(|| {
                if is_gcc {
                    app.previous_tab();
                }
                received_input = true;
            });
            (button_presses.r() || button_presses.real_digital_r()).then(|| {
                if is_gcc {
                    app.next_tab();
                } else {
                    app.reset_current_submenu();
                }
                received_input = true;
            });
            button_presses.zl().then(|| {
                if !is_gcc {
                    app.previous_tab();
                }
                received_input = true;
            });
            button_presses.zr().then(|| {
                if !is_gcc {
                    app.next_tab();
                } else {
                    app.reset_current_submenu();
                }
                received_input = true;
            });
            button_presses.l_left().then(|| {
                app.on_left();
                received_input = true;
            });
            button_presses.l_right().then(|| {
                app.on_right();
                received_input = true;
            });
            button_presses.l_up().then(|| {
                app.on_up();
                received_input = true;
            });
            button_presses.l_down().then(|| {
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
