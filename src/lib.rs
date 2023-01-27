#![feature(proc_macro_hygiene)]
#![feature(const_mut_refs)]
#![feature(exclusive_range_pattern)]
#![feature(once_cell)]
#![feature(c_variadic)]
#![allow(
    clippy::borrow_interior_mutable_const,
    clippy::declare_interior_mutable_const,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::missing_safety_doc,
    clippy::wrong_self_convention,
    clippy::option_map_unit_fn,
    clippy::float_cmp,
    clippy::fn_null_check,
    // Look into why for this one
    clippy::transmute_num_to_bytes
)]

pub mod common;
mod hazard_manager;
mod hitbox_visualizer;
mod training;

#[cfg(test)]
mod test;
mod logging;

use crate::common::*;
use crate::events::{Event, EVENT_QUEUE};

use skyline::libc::mkdir;
use skyline::nro::{self, NroInfo};
use std::fs;

use crate::menu::quick_menu_loop;
#[cfg(feature = "web_session_preload")]
use crate::menu::web_session_loop;
use training_mod_consts::{MenuJsonStruct, OnOff};
use crate::logging::*;

fn nro_main(nro: &NroInfo<'_>) {
    if nro.module.isLoaded {
        return;
    }

    if nro.name == "common" {
        skyline::install_hooks!(
            training::shield::handle_sub_guard_cont,
            training::directional_influence::handle_correct_damage_vector_common,
            training::tech::handle_change_status,
        );
    }
}

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr()
    };
}

#[skyline::main(name = "training_modpack")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let err_msg = format!("thread has panicked at '{msg}', {location}");
        skyline::error::show_error(
            69,
            "SSBU Training Modpack has panicked! Please open the details and send a screenshot to the developer, then close the game.\n",
            err_msg.as_str(),
        );
    }));
    init_logger().unwrap();

    info!("Initialized.");
    unsafe {
        EVENT_QUEUE.push(Event::smash_open());
    }

    training::ui_hacks::install_hooks();

    hitbox_visualizer::hitbox_visualization();
    hazard_manager::hazard_manager();
    training::training_mods();
    nro::add_hook(nro_main).unwrap();

    unsafe {
        mkdir(c_str!("sd:/TrainingModpack/"), 777);
    }

    let ovl_path = "sd:/switch/.overlays/ovlTrainingModpack.ovl";
    if fs::metadata(ovl_path).is_ok() {
        warn!("Removing ovlTrainingModpack.ovl...");
        fs::remove_file(ovl_path).expect(&format!("Could not remove {}", ovl_path));
    }

    info!("Performing version check...");
    release::version_check();

    let menu_conf_path = "sd:/TrainingModpack/training_modpack_menu.json";
    info!("Checking for previous menu in training_modpack_menu.json...");
    if fs::metadata(menu_conf_path).is_ok() {
        let menu_conf = fs::read_to_string(menu_conf_path).expect(&format!("Could not remove {}", menu_conf_path));
        if let Ok(menu_conf_json) = serde_json::from_str::<MenuJsonStruct>(&menu_conf) {
            unsafe {
                MENU = menu_conf_json.menu;
                DEFAULTS_MENU = menu_conf_json.defaults_menu;
                info!("Previous menu found. Loading...");
            }
        } else {
            warn!("Previous menu found but is invalid. Deleting...");
            fs::remove_file(menu_conf_path).expect(&format!("{} has invalid schema but could not be deleted!", menu_conf_path));
        }
    } else {
        info!("No previous menu file found.");
    }

    let combo_path = "sd:/TrainingModpack/training_modpack.toml";
    info!("Checking for previous button combo settings in training_modpack.toml...");
    if fs::metadata(combo_path).is_ok() {
        info!("Previous button combo settings found. Loading...");
        let combo_conf = fs::read_to_string(combo_path).expect(&format!("Could not read {}", combo_path));
        if button_config::validate_config(&combo_conf) {
            button_config::save_all_btn_config_from_toml(&combo_conf);
        } else {
            button_config::save_all_btn_config_from_defaults();
        }
    } else {
        info!("No previous button combo file found. Creating...");
        fs::write(combo_path, button_config::DEFAULT_BTN_CONFIG)
            .expect("Failed to write button config conf file");
        button_config::save_all_btn_config_from_defaults();
    }

    if is_emulator() {
        unsafe {
            DEFAULTS_MENU.quick_menu = OnOff::On;
            MENU.quick_menu = OnOff::On;
            BASE_MENU.quick_menu = OnOff::On;
        }
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

                let url = format!("{host}{path}");
                minreq::post(url).with_json(&event).expect("Failed to send info to firebase").send().ok();
            }
        }
    });

    std::thread::spawn(|| unsafe { quick_menu_loop() });

    #[cfg(feature = "web_session_preload")]
    if !is_emulator() {
        std::thread::spawn(|| unsafe { web_session_loop() });
    }
}
