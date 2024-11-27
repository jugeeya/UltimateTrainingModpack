#![feature(proc_macro_hygiene)]
#![feature(iter_intersperse)]
#![feature(const_mut_refs)]
#![feature(exclusive_range_pattern)]
#![feature(c_variadic)]
#![allow(stable_features)]
#![feature(pointer_byte_offsets)]
#![allow(
    clippy::borrow_interior_mutable_const,
    clippy::declare_interior_mutable_const,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::missing_safety_doc,
    clippy::wrong_self_convention,
    clippy::option_map_unit_fn,
    clippy::transmute_num_to_bytes,
    clippy::missing_transmute_annotations
)]

#[macro_use]
extern crate rust_i18n;
i18n!(fallback = "en_us");

use std::fs;
use std::path::PathBuf;

use skyline::nro::{self, NroInfo};
use training_mod_consts::{OnOff, LEGACY_TRAINING_MODPACK_ROOT};
use training_mod_sync::*;

use crate::common::button_config::DEFAULT_OPEN_MENU_CONFIG;
use crate::common::events::events_loop;
use crate::common::*;
use crate::consts::TRAINING_MODPACK_ROOT;
use crate::events::{Event, EVENT_QUEUE};
use crate::logging::*;
use crate::training::ui::notifications::notification;

pub mod common;
mod hazard_manager;
mod hitbox_visualizer;
mod training;

mod logging;

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

        let err_msg = format!("SSBU Training Modpack has panicked at '{msg}', {location}");
        skyline::error::show_error(
            69,
            "SSBU Training Modpack has panicked! Please open the details and send a screenshot to the developer, then close the game.\n",
            &err_msg,
        );
    }));
    init_logger().unwrap();

    info!("Initialized.");

    let mut event_queue = lock_write(&EVENT_QUEUE);
    (*event_queue).push(Event::smash_open());
    drop(event_queue);
    notification(
        t!("common.plugin_title").to_string(),
        "Welcome!".to_string(),
        60,
    );

    hitbox_visualizer::hitbox_visualization();
    hazard_manager::hazard_manager();
    training::training_mods();
    nro::add_hook(nro_main).unwrap();

    fs::create_dir_all(TRAINING_MODPACK_ROOT)
        .expect("Could not create Training Modpack root folder!");

    // Migrate legacy if exists
    if fs::metadata(LEGACY_TRAINING_MODPACK_ROOT).is_ok() {
        for entry in fs::read_dir(LEGACY_TRAINING_MODPACK_ROOT).unwrap() {
            let entry = entry.unwrap();
            let src_path = &entry.path();
            let dest_path = &PathBuf::from(TRAINING_MODPACK_ROOT).join(entry.file_name());
            fs::rename(src_path, dest_path).unwrap_or_else(|e| {
                error!("Could not move file from {src_path:#?} to {dest_path:#?} with error {e}")
            });
        }
        fs::remove_dir_all(LEGACY_TRAINING_MODPACK_ROOT).unwrap_or_else(|e| {
            error!("Could not delete legacy Training Modpack folder with error {e}")
        });
    }

    info!("Performing saved data check...");
    let data_loader = std::thread::Builder::new()
        .stack_size(0x20000)
        .spawn(move || {
            menu::load_from_file();
        })
        .unwrap();
    let _result = data_loader.join();

    if !is_emulator() {
        info!("Performing version check...");
        let _updater = std::thread::Builder::new()
            .stack_size(0x20000)
            .spawn(move || {
                release::perform_version_check();
            })
            .unwrap();
        let _result = _updater.join();
    } else {
        info!("Skipping version check because we are using an emulator");
    }

    localization::init();

    notification(
        t!("common.open_menu").to_string(),
        if read(&MENU).menu_open_start_press == OnOff::ON {
            t!("common.hold_button", button = t!("buttons.start")).to_string()
        } else {
            DEFAULT_OPEN_MENU_CONFIG.to_string()
        },
        120,
    );

    notification(
        "Save State".to_string(),
        read(&MENU).save_state_save.to_string(),
        120,
    );
    notification(
        "Load State".to_string(),
        read(&MENU).save_state_load.to_string(),
        120,
    );
    notification(
        "Input Record".to_string(),
        read(&MENU).input_record.to_string(),
        120,
    );
    notification(
        "Input Playback".to_string(),
        read(&MENU).input_playback.to_string(),
        120,
    );

    std::thread::spawn(events_loop);
}
