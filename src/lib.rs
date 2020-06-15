#![feature(proc_macro_hygiene)]
#![feature(with_options)]
#![feature(const_mut_refs)]

mod common;
mod hitbox_visualizer;
mod training;

use crate::common::*;

use skyline::libc::{fclose, fopen, fwrite, mkdir, remove, access};
use skyline::nro::{self, NroInfo};

fn nro_main(nro: &NroInfo<'_>) {
    match nro.name {
        "common" => {
            skyline::install_hooks!(
                training::shield::handle_sub_guard_cont,
                training::directional_influence::handle_correct_damage_vector_common,
                training::tech::handle_change_status
            );
        }
        _ => (),
    }
}

macro_rules! c_str {
    ($l:tt) => { [$l.as_bytes(), "\u{0}".as_bytes()]
                .concat()
                .as_ptr(); }
}

#[skyline::main(name = "training_modpack")]
pub fn main() {
    println!("[Training Modpack] Initialized.");
    hitbox_visualizer::hitbox_visualization();
    training::training_mods();
    nro::add_hook(nro_main).unwrap();

    unsafe {
        let buffer = format!("{:x}", MENU as *const _ as u64);
        println!(
            "[Training Modpack] Writing training_modpack.log with {}...",
            buffer
        );
        mkdir(c_str!("sd:/TrainingModpack/"), 0777);

        if access(c_str!("sd:/TrainingModpack/training_modpack.conf"), 0) != -1 {
            remove(c_str!("sd:/TrainingModpack/training_modpack.conf"));
        }

        let f = fopen(
            c_str!("sd:/TrainingModpack/training_modpack.log"),
            c_str!("w"),
        );

        if !f.is_null() {
            fwrite(c_str!(buffer), 1, buffer.len(), f);
            fclose(f);
        }
    }
}
