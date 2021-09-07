#![feature(proc_macro_hygiene)]
#![feature(with_options)]
#![feature(const_mut_refs)]
#![feature(exclusive_range_pattern)]
#![allow(clippy::borrow_interior_mutable_const, clippy::not_unsafe_ptr_arg_deref, clippy::missing_safety_doc, clippy::wrong_self_convention)]

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
use crate::menu::set_menu_from_url;

use skyline::libc::mkdir;
use skyline_web::{Dialog, DialogOk};
use std::fs;

use owo_colors::OwoColorize;

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
    
    let nro_hook_path = "sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libnro_hook.nro";
    if fs::metadata(nro_hook_path).is_ok() {
        let rm_nro_hook = Dialog::yes_no(
            "You are starting Smash with the NRO hook installed.\n\n\
            This file causes instability and should not be installed with the Training Modpack any longer. Would you like to remove it?\n\
            If you don't know what this means and do not use any character moveset-changing mods, please select Yes."
        );
        if rm_nro_hook {
            log!("Removing libnro_hook.nro...");
            fs::remove_file(nro_hook_path).unwrap();
            DialogOk::ok("Thank you! Please restart Smash for a more stable experience.");
        }
    }

    let menu_conf_path = "sd:/TrainingModpack/training_modpack_menu.conf";
    if fs::metadata(menu_conf_path).is_ok() {
        log!("Loading previous menu from training_modpack_menu.conf...");
        let menu_conf = fs::read(menu_conf_path).unwrap();
        if menu_conf.starts_with(b"http://localhost") {
           set_menu_from_url(std::str::from_utf8(&menu_conf).unwrap());
        }
    }
}
