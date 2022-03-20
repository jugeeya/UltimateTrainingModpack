#![feature(proc_macro_hygiene)]
#![feature(with_options)]
#![feature(const_mut_refs)]
#![feature(exclusive_range_pattern)]
#![feature(once_cell)]
#![allow(
    clippy::borrow_interior_mutable_const,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::missing_safety_doc,
    clippy::wrong_self_convention
)]

pub mod common;
mod hazard_manager;
mod hitbox_visualizer;
mod training;

#[cfg(test)]
mod test;

use crate::common::*;
use crate::events::{Event, EVENT_QUEUE};
use crate::common::consts::get_menu_from_url;

use skyline::libc::{c_char, mkdir};
use skyline::nro::{self, NroInfo};
use std::fs;

use owo_colors::OwoColorize;

fn nro_main(nro: &NroInfo<'_>) {
    if nro.module.isLoaded {
        return;
    }

    if nro.name == "common" {
        skyline::install_hooks!(
            training::shield::handle_sub_guard_cont,
            training::directional_influence::handle_correct_damage_vector_common,
            training::sdi::process_hit_stop_delay,
            training::tech::handle_change_status,
        );
    }
}

extern "C" {
    #[link_name = "render_text_to_screen"]
    pub fn render_text_to_screen_cstr(str: *const c_char);

    #[link_name = "set_should_display_text_to_screen"]
    pub fn set_should_display_text_to_screen(toggle: bool);
}

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

pub fn render_text_to_screen(s: &str) {
    unsafe {
        render_text_to_screen_cstr(c_str!(s));
    }
}

#[skyline::main(name = "training_modpack")]
pub fn main() {
    macro_rules! log {
        ($($arg:tt)*) => {
            print!("{}", "[Training Modpack] ".green());
            println!($($arg)*);
        };
    }

    log!("Initialized.");
    unsafe {
        EVENT_QUEUE.push(Event::smash_open());
    }

    hitbox_visualizer::hitbox_visualization();
    hazard_manager::hazard_manager();
    training::training_mods();
    nro::add_hook(nro_main).unwrap();


    unsafe {
        mkdir(c_str!("sd:/TrainingModpack/"), 777);
    }

    let ovl_path = "sd:/switch/.overlays/ovlTrainingModpack.ovl";
    if fs::metadata(ovl_path).is_ok() {
        log!("Removing ovlTrainingModpack.ovl...");
        fs::remove_file(ovl_path).unwrap();
    }

    log!("Performing version check...");
    release::version_check();

    let menu_conf_path = "sd:/TrainingModpack/training_modpack_menu.conf";
    log!("Checking for previous menu in training_modpack_menu.conf...");
    if fs::metadata(menu_conf_path).is_ok() {
        let menu_conf = fs::read(menu_conf_path).unwrap();
        if menu_conf.starts_with(b"http://localhost") {
            log!("Previous menu found, loading from training_modpack_menu.conf");
            unsafe {
                MENU = get_menu_from_url(MENU, std::str::from_utf8(&menu_conf).unwrap());
            }
        } else {
            log!("Previous menu found but is invalid.");
        }
    } else {
        log!("No previous menu file found.");
    }

    let menu_defaults_conf_path = "sd:/TrainingModpack/training_modpack_menu_defaults.conf";
    log!("Checking for previous menu defaults in training_modpack_menu_defaults.conf...");
    if fs::metadata(menu_defaults_conf_path).is_ok() {
        let menu_defaults_conf = fs::read(menu_defaults_conf_path).unwrap();
        if menu_defaults_conf.starts_with(b"http://localhost") {
            log!("Menu defaults found, loading from training_modpack_menu_defaults.conf");
            unsafe {
                DEFAULT_MENU = get_menu_from_url(
                    DEFAULT_MENU,
                    std::str::from_utf8(&menu_defaults_conf).unwrap(),
                );
                crate::menu::write_menu();
            }
        } else {
            log!("Previous menu defaults found but are invalid.");
        }
    } else {
        log!("No previous menu defaults found.");
    }

    std::thread::spawn(|| loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        unsafe {
            while let Some(event) = EVENT_QUEUE.pop() {
                let host = "https://my-project-1511972643240-default-rtdb.firebaseio.com";
                let path = format!(
                    "/event/{}/device/{}/{}.json",
                    event.event_name, event.device_id, event.event_time
                );

                let url = format!("{}{}", host, path);
                minreq::post(url).with_json(&event).unwrap().send().ok();
            }
        }
    });

    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let menu;
        unsafe {
            menu = crate::common::consts::get_menu();
        }

        let mut app = training_mod_tui::App::new(menu);

        let backend = training_mod_tui::TestBackend::new(75, 15);
        let mut terminal = training_mod_tui::Terminal::new(backend).unwrap();

        unsafe {
            let mut has_slept_millis = 0;
            let mut url = String::new();
            let button_presses = &mut common::menu::BUTTON_PRESSES;
            loop {
                button_presses.a.read_press().then(|| app.on_a());
                button_presses.b.read_press().then(|| {
                    if app.outer_list == false {
                        app.on_b()
                    } else {
                        // Leave menu.
                        menu::QUICK_MENU_ACTIVE = false;
                        crate::menu::set_menu_from_url(url.as_str());
                    }
                });
                button_presses.zl.read_press().then(|| app.on_l());
                button_presses.zl.read_press().then(|| app.on_r());
                button_presses.left.read_press().then(|| app.on_left());
                button_presses.right.read_press().then(|| app.on_right());
                button_presses.up.read_press().then(|| app.on_up());
                button_presses.down.read_press().then(|| app.on_down());

                std::thread::sleep(std::time::Duration::from_millis(16));
                has_slept_millis += 16;
                let render_frames = 5;
                if has_slept_millis > 16 * render_frames {
                    has_slept_millis = 16;
                    let mut view = String::new();

                    let frame_res = terminal
                        .draw(|f| url = training_mod_tui::ui(f, &mut app))
                        .unwrap();

                    use std::fmt::Write;
                    for (i, cell) in frame_res.buffer.content().into_iter().enumerate() {
                        write!(&mut view, "{}", cell.symbol).unwrap();
                        if i % frame_res.area.width as usize == frame_res.area.width as usize - 1 {
                            write!(&mut view, "\n").unwrap();
                        }
                    }
                    write!(&mut view, "\n").unwrap();

                    if menu::QUICK_MENU_ACTIVE {
                        render_text_to_screen(view.as_str());
                    } else {
                        set_should_display_text_to_screen(false);
                    }
                }
            }
        }
    });
}
