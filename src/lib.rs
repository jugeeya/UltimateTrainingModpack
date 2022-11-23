#![feature(proc_macro_hygiene)]
#![feature(const_mut_refs)]
#![feature(exclusive_range_pattern)]
#![feature(once_cell)]
#![feature(c_variadic)]
#![allow(
    clippy::borrow_interior_mutable_const,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::missing_safety_doc,
    clippy::wrong_self_convention,
    clippy::option_map_unit_fn,
    clippy::float_cmp
)]

pub mod common;
mod hazard_manager;
mod hitbox_visualizer;
mod training;

#[cfg(test)]
mod test;

use crate::common::*;
use crate::events::{Event, EVENT_QUEUE};

use skyline::libc::mkdir;
use skyline::nro::{self, NroInfo};
use skyline::{hooks::InlineCtx};
use std::fs;

use crate::menu::quick_menu_loop;
#[cfg(feature = "web_session_preload")]
use crate::menu::web_session_loop;
use owo_colors::OwoColorize;
use training_mod_consts::{MenuJsonStruct, OnOff};

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

#[derive(Debug)]
pub struct TValue {
    value: u64,
    tt: i32
}

#[skyline::hook(offset = 0x38f3d60)]
pub unsafe fn handle_lua_setfield(
    lua_state: u64,
    lua_tvalue: *const TValue,
    field_name: *const skyline::libc::c_char
) {
    if skyline::from_c_str(field_name) == "LayoutRootList" {
        println!("In LayoutRootList");
    }
    original!()(lua_state, lua_tvalue, field_name);
}

#[skyline::hook(offset = 0x3777130)]
pub unsafe fn handle_play_animation(
    layout_view: u64,
    animation_name: *const skyline::libc::c_char
) -> u64 {
    println!("play_animation: {}", skyline::from_c_str(animation_name));
    original!()(layout_view, animation_name)
}

#[skyline::hook(offset = 0x3776cd0)]
pub unsafe fn handle_play_animation_at_speed(
    speed: f32,
    unk: u64,
    animation_name: *const skyline::libc::c_char
) -> u64 {
    println!("play_animation_at_speed: {}", skyline::from_c_str(animation_name));
    original!()(speed, unk, animation_name)
}

#[skyline::hook(offset = 0x3777000)]
pub unsafe fn handle_play_animation_at_speed2(
    speed: f32,
    unk: u64,
    animation_name: *const skyline::libc::c_char
) -> u64 {
    println!("play_animation_at_speed2: {}", skyline::from_c_str(animation_name));
    original!()(speed, unk, animation_name)
}



#[skyline::hook(offset = 0x3776ab0, inline)]
pub unsafe fn handle_get_pane_animation(ctx: &mut InlineCtx) {
    println!("get_pane_animation: {}", skyline::from_c_str(*ctx.registers[1].x.as_ref() as *const u8));
}


#[skyline::hook(offset = 0x4b120)]
pub unsafe fn handle_bind_animation(
    layout_view: u64,
    animation_name: *const skyline::libc::c_char
) -> u64 {
    println!("bind_animation: {}", skyline::from_c_str(animation_name));
    original!()(layout_view, animation_name)
}

#[skyline::hook(offset = 0x0595d0)]
pub unsafe fn handle_bind_animation2(
    layout_view: u64,
    animation_name: *const skyline::libc::c_char,
    unk1: u32,
    unk2: u32
) -> u64 {
    println!("bind_animation: {}", skyline::from_c_str(animation_name));
    original!()(layout_view, animation_name, unk1, unk2)
}

#[repr(C)]
#[derive(Debug)]
pub struct LayoutPaneUi2d {
    unk_addresses: [u64; 6],
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    scale_x: f32,
    scale_y: f32,
    size_x: f32,
    size_y: f32,
    flags: u8,
    alpha: u8
}

#[repr(C)]

#[derive(Debug)]

pub struct LayoutPane {
    layout_pane_ui2d: *mut LayoutPaneUi2d,
    picture: u64,
    sub_layout_pane_user_data_unk: u64,
    sub_layout_pane: *mut LayoutPane,
}

#[skyline::hook(offset = 0x3775480, inline)]
pub unsafe fn handle_get_pane_by_name(
    ctx: &mut InlineCtx
) {
    // Grabbing stuff off the stack is interesting. 
    let pane_name = skyline::from_c_str(
        (ctx as *const InlineCtx as *const u8).add(0x100).add(0xD8)
    );
    println!("get_pane_by_name: {}", pane_name); 
    if pane_name == "set_dmg_p" || true {
        let layout_pane = (*ctx.registers[0].x.as_ref()) as *mut LayoutPane;
        if !layout_pane.is_null() {
            println!("pane: {:#?}", *layout_pane);
            // pane_set_text_string(layout_pane, c_str!("Test!"));
            let sublayout_pane = (*layout_pane).sub_layout_pane;
            if !sublayout_pane.is_null() {
                println!("sublayout_pane: {:#?}", *sublayout_pane);
                // pane_set_text_string(layout_pane, c_str!("Test!"));
            }
            let layout_pane_ui2d = (*layout_pane).layout_pane_ui2d;
            if !layout_pane_ui2d.is_null() {
                println!("pane_ui2d: {:#?}", *layout_pane_ui2d);
                // Turn invisible
                (*layout_pane_ui2d).scale_x = (*layout_pane_ui2d).scale_x * 2.0;
                (*layout_pane_ui2d).scale_y = (*layout_pane_ui2d).scale_y * 2.0;
                (*layout_pane_ui2d).flags = (*layout_pane_ui2d).flags | 0x10;

            }
        }
    }
}


#[skyline::hook(offset = 0x3774ac0)]
pub unsafe fn handle_set_enable_input(
    layout_root: u64,
    enable: bool
) -> u64 {
    println!("set_enable_input");
    original!()(layout_root, enable)
}

pub struct AnimTransform {
    vtable: u64,
    unk: [u64; 2],
    enabled: bool
}

pub struct AnimTransformNode {
    data: *mut AnimTransform,
    next: *mut AnimTransformNode
}

pub struct RawLayout {
    anim_trans_list: AnimTransformNode,
    root_pane: *const LayoutPaneUi2d,
    group_container: u64,
    layout_size: f64,
    layout_name: *const skyline::libc::c_char
}

pub struct Layout {
    vtable: u64,
    raw_layout: RawLayout
}

#[skyline::hook(offset = 0x4b620)]
pub unsafe fn handle_draw(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    let layout_name = skyline::from_c_str((*layout).raw_layout.layout_name);
    let layout_root_pane = (*layout).raw_layout.root_pane;
    let mut curr = &mut (*layout).raw_layout.anim_trans_list as *mut AnimTransformNode;

    if layout_name == "info_training" {
        for s in vec![
            "txt_cap_00",
            "set_txt_num_00",
            "set_txt_num_01",
        ] {
            let txt_pane = find_pane_by_name_recursive(layout_root_pane, c_str!(s));
            // println!("Replacing {}/{}...", layout_name, s);
            pane_set_text_string(txt_pane, c_str!("Hello!"));            
            // println!("Txt Pane: {:#X?}", *txt_pane);
        }
    }

    if layout_name == "info_melee" {
        for s in vec![
            "p1"
        ] {
            let dmg_pane = find_pane_by_name_recursive(layout_root_pane, c_str!(s));
            (*dmg_pane).pos_y = (*dmg_pane).pos_y + 300.0;
            for anim_search_name in vec![
                // "dig_3",
                // "dig_3_anim",
                // "dig_3_reach",
                // "set_dmg_num_3",
                // "dig_2",
                // "dig_2_anim",
                // "dig_2_reach",
                // "set_dmg_num_2",
                // "dig_1",
                // "dig_1_anim",
                // "dig_1_reach",
                // "set_dmg_num_1",
                // "dig_0",
                // "dig_0_anim",
                // "dig_0_reach",
                // "set_dmg_num_0",
                // "set_dmg_num_dec",
                // "dig_dec",
                // "dig_dec_reach_0",
                // "dig_dec_anim_00",
                // "dig_dec_reach_1",
                // "dig_dec_reach_01",
                "set_dmg_p",
                "changedig"
            ] {
                let dmg_pane_p1 = find_pane_by_name_recursive(dmg_pane, c_str!(anim_search_name));
                if !dmg_pane_p1.is_null() {
                    println!("Found pane by {}::find_pane_by_name({}): {:X?}", layout_name, anim_search_name, *dmg_pane_p1);
                }
            }
        }
    }


    
    original!()(layout, draw_info, cmd_buffer);
}

#[skyline::hook(offset = 0x4b120)] 
pub unsafe fn handle_pane_bind_animation(layout: *mut Layout, anim: *const u64) {
    println!("Bind Animation");
    original!()(layout, anim)
}

#[skyline::from_offset(0x59970)]
pub unsafe fn find_pane_by_name_recursive(
    pane: *const LayoutPaneUi2d,
    s: *const skyline::libc::c_char
) -> *mut LayoutPaneUi2d;

#[skyline::from_offset(0x583c0)]
pub unsafe fn find_pane_by_name(
    pane: *const LayoutPaneUi2d,
    s: *const skyline::libc::c_char,
    some_bool_maybe: bool
) -> *mut LayoutPaneUi2d;

#[skyline::from_offset(0x37a1270)]
pub unsafe fn pane_set_text_string(
    pane: *const LayoutPaneUi2d,
    s: *const skyline::libc::c_char
);


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

        let err_msg = format!("thread has panicked at '{}', {}", msg, location);
        skyline::error::show_error(
            69,
            "Skyline plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n",
            err_msg.as_str(),
        );
    }));

    macro_rules! log {
        ($($arg:tt)*) => {
            println!("{}{}", "[Training Modpack] ".green(), format!($($arg)*));
        };
    }

    log!("Initialized.");
    unsafe {
        EVENT_QUEUE.push(Event::smash_open());
    }

    skyline::install_hooks!(
        // handle_lua_setfield,
        // handle_play_animation,
        // handle_play_animation_at_speed,
        // handle_get_pane_animation,
        // handle_play_animation_at_speed2,
        // handle_bind_animation,
        // handle_bind_animation2,
        // handle_set_enable_input,
        // handle_get_pane_by_name,
        handle_draw,
        handle_pane_bind_animation
    );

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

    let menu_conf_path = "sd:/TrainingModpack/training_modpack_menu.json";
    log!("Checking for previous menu in training_modpack_menu.json...");
    if fs::metadata(menu_conf_path).is_ok() {
        let menu_conf = fs::read_to_string(&menu_conf_path).unwrap();
        if let Ok(menu_conf_json) = serde_json::from_str::<MenuJsonStruct>(&menu_conf) {
            unsafe {
                MENU = menu_conf_json.menu;
                DEFAULTS_MENU = menu_conf_json.defaults_menu;
                log!("Previous menu found. Loading...");
            }
        } else if menu_conf.starts_with("http://localhost") {
            log!("Previous menu found, with URL schema. Deleting...");
            fs::remove_file(menu_conf_path).expect("Could not delete menu conf file!");
        } else {
            log!("Previous menu found but is invalid. Deleting...");
            fs::remove_file(menu_conf_path).expect("Could not delete menu conf file!");
        }
    } else {
        log!("No previous menu file found.");
    }

    let combo_path = "sd:/TrainingModpack/training_modpack.toml";
    log!("Checking for previous button combo settings in training_modpack.toml...");
    if fs::metadata(combo_path).is_ok() {
        log!("Previous button combo settings found. Loading...");
        let combo_conf = fs::read_to_string(&combo_path).unwrap();
        if button_config::validate_config(&combo_conf) {
            button_config::save_all_btn_config_from_toml(&combo_conf);
        } else {
            button_config::save_all_btn_config_from_defaults();
        }
    } else {
        log!("No previous button combo file found. Creating...");
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

                let url = format!("{}{}", host, path);
                minreq::post(url).with_json(&event).unwrap().send().ok();
            }
        }
    });

    std::thread::spawn(|| unsafe { quick_menu_loop() });

    #[cfg(feature = "web_session_preload")]
    if !is_emulator() {
        std::thread::spawn(|| unsafe { web_session_loop() });
    }
}
