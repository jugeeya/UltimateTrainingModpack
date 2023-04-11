#![feature(proc_macro_hygiene)]
#![feature(const_mut_refs)]
#![feature(exclusive_range_pattern)]
#![feature(c_variadic)]
#![allow(
    clippy::borrow_interior_mutable_const,
    clippy::declare_interior_mutable_const,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::missing_safety_doc,
    clippy::wrong_self_convention,
    clippy::option_map_unit_fn,
    clippy::fn_null_check,
    clippy::transmute_num_to_bytes
)]

use std::fs;
use std::path::PathBuf;

use skyline::nro::{self, NroInfo};
use training_mod_consts::LEGACY_TRAINING_MODPACK_ROOT;

use crate::common::events::events_loop;
use crate::common::*;
use crate::consts::TRAINING_MODPACK_ROOT;
use crate::events::{Event, EVENT_QUEUE};
use crate::logging::*;
use crate::menu::quick_menu_loop;
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
            err_msg.as_str(),
        );
    }));
    init_logger().unwrap();

    info!("Initialized.");
    unsafe {
        EVENT_QUEUE.push(Event::smash_open());
        notification("Training Modpack".to_string(), "Welcome!".to_string(), 60);
        notification(
            "Open Menu".to_string(),
            "Special + Uptaunt".to_string(),
            120,
        );
        notification(
            "Save State".to_string(),
            "Shield + Downtaunt".to_string(),
            120,
        );
        notification("Load State".to_string(), "Shield + Uptaunt".to_string(), 120);
    }

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
        fs::remove_dir_all(LEGACY_TRAINING_MODPACK_ROOT)
            .unwrap_or_else(|e| error!("Could not delete legacy Training Modpack folder with error {e}"));
    }

    info!("Performing version check...");
    release::version_check();

    menu::load_from_file();
    button_config::load_from_file();

    std::thread::spawn(events_loop);

    std::thread::spawn(|| unsafe { quick_menu_loop() });
}
