#![feature(proc_macro_hygiene)]
#![feature(with_options)]
#![feature(const_mut_refs)]

mod common;
mod hazard_manager;
mod hitbox_visualizer;
mod training;

#[macro_use]
extern crate bitflags;

use crate::common::*;
use crate::common::consts::*;
use training::combo::FRAME_ADVANTAGE;

use skyline::libc::{c_void, fclose, fopen, fwrite, mkdir};
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
            training::tech::handle_change_status
        );
    }
}

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

use skyline_web::{Background, BootDisplay, Webpage};
use ramhorns::{Template, Content};

#[derive(Content)]
pub struct Dialog {
    #[md]
    text: String,
    left_button: String,
    right_button: String,
}

use std::thread;
use std::time::Duration;

use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

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
        let mut buffer = format!("{:x}", MENU as *const _ as u64);
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

    thread::spawn(||{
        loop {
            unsafe {
                thread::sleep(Duration::from_secs(5));

                // Grab + Dpad up: reset state
                let mut state = skyline::nn::hid::NpadHandheldState::default();
                let id = 0x20;

                skyline::nn::hid::GetNpadHandheldState(&mut state, &id);

                println!("{:#?}", state.Buttons);

                if true {
                    let tpl = Template::new(include_str!("templates/menu.html")).unwrap();

                    let dialog = Dialog {
                        text: "".into(),
                        left_button: "".into(),
                        right_button: "".into()
                    };

                    let response = Webpage::new()
                        .background(Background::BlurredScreenshot)
                        .file("index.html", include_str!("templates/menu.html"))
                        .htdocs_dir("contents")
                        .boot_display(BootDisplay::BlurredScreenshot)
                        .open()
                        .unwrap();

                    let last_url = response.get_last_url().unwrap();
                    println!("Response last url: {:#?}", last_url);

                    if last_url.contains("Infinite") {
                        MENU_STRUCT.shield_state = Shield::Infinite;
                    } else if last_url.contains("Hold") {
                        MENU_STRUCT.shield_state = Shield::Hold;
                    } else {
                        MENU_STRUCT.shield_state = Shield::None;
                    }
                }
            }
            thread::sleep(Duration::from_secs(5));
        }
    });
}
