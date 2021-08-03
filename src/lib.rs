#![feature(proc_macro_hygiene)]
#![feature(with_options)]
#![feature(const_mut_refs)]

pub mod common;
mod hazard_manager;
mod hitbox_visualizer;
mod training;

#[cfg(test)]
mod test;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate num_derive;

use crate::common::*;
use training::combo::FRAME_ADVANTAGE;

use skyline::libc::{c_void, fclose, fopen, fwrite, mkdir};
use std::fs;
use skyline::nro::{self, NroInfo};

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

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

#[cfg(not(test))]
#[skyline::main(name = "training_modpack")]
pub fn main() {
    macro_rules! log {
        ($($arg:tt)*) => {
            print!("{}", "[Training Modpack] ".green());
            println!($($arg)*);
        };
    }

    log!("Initialized.");
    hitbox_visualizer::hitbox_visualization();
    hazard_manager::hazard_manager();
    training::training_mods();
    nro::add_hook(nro_main).unwrap();

    unsafe {
        let mut buffer = format!("{:x}", &MENU as *const _ as u64);
        log!(
            "Writing training_modpack.log with {}...",
            buffer
        );
        mkdir(c_str!("sd:/TrainingModpack/"), 777);

        // Only necessary upon version upgrade.
        // log!("[Training Modpack] Removing training_modpack_menu.conf...");
        // remove(c_str!("sd:/TrainingModpack/training_modpack_menu.conf"));

        let mut f = fopen(
            c_str!("sd:/TrainingModpack/training_modpack.log"),
            c_str!("w"),
        );

        if !f.is_null() {
            fwrite(c_str!(buffer) as *const c_void, 1, buffer.len(), f);
            fclose(f);
        }

        buffer = format!("{:x}", &FRAME_ADVANTAGE as *const _ as u64);
        log!(
            "Writing training_modpack_frame_adv.log with {}...",
            buffer
        );

        f = fopen(
            c_str!("sd:/TrainingModpack/training_modpack_frame_adv.log"),
            c_str!("w"),
        );

        if !f.is_null() {
            fwrite(c_str!(buffer) as *const c_void, 1, buffer.len(), f);
            fclose(f);
        }
    }

    let ovl_path = "sd:/switch/.overlays/ovlTrainingModpack.ovl";
    if !fs::metadata(ovl_path).is_err() {
        log!("Removing ovlTrainingModpack.ovl...");
        fs::remove_file(ovl_path).unwrap();
    }

    log!("Performing version check...");
    release::version_check();
}