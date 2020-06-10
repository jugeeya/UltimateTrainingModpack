#![feature(proc_macro_hygiene)]
#![feature(with_options)]
#![feature(const_mut_refs)]

mod common;
mod hitbox_visualizer;
mod training;

use crate::common::*;

use skyline::c_str;
use skyline::libc::{c_void, fclose, fopen, fwrite, mkdir};
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
        mkdir("sd:/TrainingModpack/\u{0}".as_bytes().as_ptr(), 0777);
        let f = fopen(
            "sd:/TrainingModpack/training_modpack.log\u{0}"
                .as_bytes()
                .as_ptr(),
            "w\u{0}".as_bytes().as_ptr(),
        );

        if !f.is_null() {
            fwrite(c_str(&buffer) as *const c_void, 1, buffer.len(), f);
            fclose(f);
        }
    }
}
