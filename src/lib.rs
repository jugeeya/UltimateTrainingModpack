#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(with_options)]

mod hitbox_visualizer;
mod training;
mod common;

use smash::hash40;
use smash::lib::lua_const::{*};
use smash::lib::{self, L2CAgent, L2CValue};
use smash::app::{self};
use smash::app::lua_bind::{*};
use smash::app::sv_animcmd::{self};
use smash::app::sv_system::{self};
use skyline::libc::{size_t, c_int, c_void, strlen};
use smash::Result;
use skyline::nn;
use skyline::patching::patch_data_from_text;
use skyline::{from_c_str, c_str};
use std::fs;

#[allow(unused_unsafe)]
#[skyline::hook(replace = nn::ro::LoadModule)]
pub unsafe fn handle_load_module(
    p_out_module: *mut skyline::nn::ro::Module, 
    p_image: *const c_void, 
    buffer: *mut c_void, 
    buffer_size: size_t, 
    flag: c_int) -> Result {

    let ret = original!()(p_out_module, p_image, buffer, buffer_size, flag);

    let name = from_c_str(&(*p_out_module).Name as *const u8);
    println!("[handleLoadModule] NRO name: {}\n", name);
    let text_start = (*(*p_out_module).ModuleObject).module_base;
    println!("Module base: {}\n", text_start);
    if name.starts_with("common") {
        println!("Is common!");
        // raw const_value_table is at : 0x635b70
        let fighter_status_kind_fall : u64 = 0x8ee6c39e9be4f0b5;
        let res = match patch_data_from_text(text_start as *const u8, 0x6362b8, &fighter_status_kind_fall) {
            Ok(v) => format!("Patched!"),
            Err(e) => format!("Error patching with e: {}", e)
        };

        println!("{}", res);
    }

    ret
}

#[skyline::main(name = "test")]
pub fn main() {
    println!("Training modpack initialized.");
    hitbox_visualizer::hitbox_visualization();
    training::training_mods();

    println!("OpenMode_Write: {} {}", nn::fs::OpenMode_OpenMode_Write, nn::fs::OpenMode_OpenMode_Write as i32);
    let buffer = format!("{:x}", &common::menu as *const _ as u64);
    println!("Writing training_modpack.log with {}...\n", buffer);

    // skyline::install_hook!(handle_load_module);
}
