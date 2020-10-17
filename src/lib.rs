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

use std::thread;
use std::time::Duration;

use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

#[derive(Content)]
struct Toggle<'a> {
    title: &'a str,
    checked: &'a str,
    index: usize,
    value: usize,
}

impl<'a> Toggle<'a> {
    pub fn new(title: &'a str, checked: bool, value: usize) -> Toggle<'a> {
        Toggle{
            title: title,
            checked: if checked { "is-appear "} else { "is-hidden" },
            index: 0,
            value
        }
    }
}

#[derive(Content)]
struct SubMenu<'a> {
    title: &'a str,
    id: &'a str,
    toggles: Vec<Toggle<'a>>,
    index: usize,
    check_against: usize
}

impl<'a> SubMenu<'a> {
    pub fn max_idx(&self) -> usize {
        self.toggles
            .iter()
            .max_by(|t1,t2| t1.index.cmp(&t2.index))
            .map(|t| t.index)
            .unwrap_or(self.index)
    }

    pub fn add_toggle(&mut self, title: &'a str, checked: bool, value: usize) {
        self.toggles.push(Toggle{
            title: title,
            checked: if checked { "is-appear "} else { "is-hidden" },
            index: self.max_idx() + 1,
            value
        });
    }
}

#[derive(Content)]
struct Menu<'a> {
    sub_menus: Vec<SubMenu<'a>>
}

impl<'a> Menu<'a> {
    pub fn max_idx(&self) -> usize {
        self.sub_menus
            .iter()
            .max_by(|x, y| x.max_idx().cmp(&y.max_idx()))
            .map(|sub_menu| sub_menu.max_idx())
            .unwrap_or(0)
    }

    pub fn add_sub_menu(&mut self, title: &'a str, id: &'a str, check_against: usize, toggles: Vec<(&'a str, usize)>) {
        let mut sub_menu = SubMenu {
            title: title,
            id: id,
            toggles: Vec::new(),
            index: self.max_idx() + 1,
            check_against: check_against
        };

        for toggle in toggles {
            sub_menu.add_toggle(toggle.0, (check_against & toggle.1) != 0, toggle.1)
        }

        self.sub_menus.push(sub_menu);
    }

    pub fn add_sub_menu_sep(&mut self, title: &'a str, id: &'a str, check_against: usize, strs: Vec<&'a str>, vals: Vec<usize>) {
        let mut sub_menu = SubMenu {
            title: title,
            id: id,
            toggles: Vec::new(),
            index: self.max_idx() + 1,
            check_against: check_against
        };

        for i in 0..strs.len() {
            sub_menu.add_toggle(strs[i], (check_against & vals[i]) != 0, vals[i])
        }

        self.sub_menus.push(sub_menu);
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
                // thread::sleep(Duration::from_secs(5));

                // // Grab + Dpad up: reset state
                // let mut state = skyline::nn::hid::NpadHandheldState::default();
                // let id = 0x20;

                // skyline::nn::hid::GetNpadHandheldState(&mut state, &id);

                // println!("{:#?}", state.Buttons);

                macro_rules! add_bitflag_submenu {
                    ($menu:ident, $title:literal, $id:ident, $e:ty) => {
                        paste::paste!{
                            let [<$id _strs>] = <$e>::to_toggle_strs();
                            let [<$id _vals>] = <$e>::to_toggle_vals();

                            $menu.add_sub_menu_sep(
                                $title, 
                                stringify!($id), 
                                MENU_STRUCT.$id.bits() as usize,
                                [<$id _strs>].iter().map(|i| i.as_str()).collect(),
                                [<$id _vals>]
                            );
                        }
                    }
                }

                if true {
                    let tpl = Template::new(include_str!("templates/menu.html")).unwrap();

                    let mut overall_menu = Menu {
                        sub_menus: Vec::new()
                    };

                    let input_delay_vals = 0..=10;
                    let input_delay_strs : Vec<String> = (0..=10).map(|i| i.to_string()).collect();
                    let mut toggles = Vec::new();
                    for i in input_delay_vals {
                        toggles.push((input_delay_strs[i].as_str(), i));
                    }

                    add_bitflag_submenu!(overall_menu, "Mash Toggles", mash_state, Action);
                    add_bitflag_submenu!(overall_menu, "Followup Toggles", follow_up, Action);

                    add_bitflag_submenu!(overall_menu, "Ledge Options", ledge_state, LedgeOption);
                    add_bitflag_submenu!(overall_menu, "Ledge Delay", ledge_delay, Delay);
                    add_bitflag_submenu!(overall_menu, "Tech Options", tech_state, TechFlags);
                    add_bitflag_submenu!(overall_menu, "Miss Tech Options", miss_tech_state, MissTechFlags);
                    add_bitflag_submenu!(overall_menu, "Defensive Options", defensive_state, Defensive);

                    add_bitflag_submenu!(overall_menu, "OoS Offset", oos_offset, Delay);
                    add_bitflag_submenu!(overall_menu, "Reaction Time", reaction_time, Delay);

                    add_bitflag_submenu!(overall_menu, "Fast Fall", fast_fall, BoolFlag);
                    add_bitflag_submenu!(overall_menu, "Fast Fall Delay", fast_fall_delay, Delay);
                    add_bitflag_submenu!(overall_menu, "Falling Aerials", falling_aerials, BoolFlag);
                    add_bitflag_submenu!(overall_menu, "Full Hop", full_hop, BoolFlag);

                    add_bitflag_submenu!(overall_menu, "Shield Tilt", shield_tilt, Direction);

                    add_bitflag_submenu!(overall_menu, "DI", di_state, Direction);
                    add_bitflag_submenu!(overall_menu, "SDI", sdi_state, Direction);
                    add_bitflag_submenu!(overall_menu, "Airdodge Direction", air_dodge_dir, Direction);

                    overall_menu.add_sub_menu(
                        "Shield Toggles", 
                        "shield", 
                        MENU_STRUCT.shield_state as usize,
                        [
                            ("None", Shield::None as usize),
                            ("Hold", Shield::Hold as usize),
                            ("Infinite", Shield::Infinite as usize),
                        ].to_vec()
                    );

                    overall_menu.add_sub_menu(
                        "Input Delay", 
                        "inputDelay", 
                        MENU_STRUCT.input_delay as usize,
                        toggles
                    );

                    let data = &tpl.render(&overall_menu);

                    let response = Webpage::new()
                        .background(Background::BlurredScreenshot)
                        .file("index.html", data)
                        .htdocs_dir("contents")
                        .boot_display(BootDisplay::BlurredScreenshot)
                        .boot_icon(true)
                        .open()
                        .unwrap();

                    let last_url = response.get_last_url().unwrap();
                    println!("Response last url: {:#?}; len: {}", last_url, last_url.len());
                    

                    if last_url.contains("Infinite") {
                        MENU_STRUCT.shield_state = Shield::Infinite;
                    } else if last_url.contains("Hold") {
                        MENU_STRUCT.shield_state = Shield::Hold;
                    } else {
                        MENU_STRUCT.shield_state = Shield::None;
                    }
                }

            }
        }
    });
}
