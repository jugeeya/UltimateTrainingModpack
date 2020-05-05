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
use skyline::{from_c_str, c_str, hooks::A64HookFunction};
use std::fs;
use skyline::nro::{self, NroInfo};

extern "C" {
    #[link_name = "\u{1}_ZN7lua2cpp16L2CFighterCommon28status_AttackAir_Main_commonEv"]
    pub fn status_AttackAirMain_common(
        arg1: *mut L2CAgent
    ) -> u64;

    #[link_name = "\u{1}_ZN7lua2cpp27L2CFighterAnimcmdGameCommon21game_BatSwing4Common1Ev"]
    pub fn game_BatSwing4Common1(
        arg1: *mut L2CAgent
    ) -> u64;

    #[link_name = "\u{1}_ZN7lua2cpp27L2CFighterAnimcmdGameCommon21game_BatSwing4Common2Ev"]
    pub fn game_BatSwing4Common2(
        arg1: *mut L2CAgent
    ) -> u64;
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = status_AttackAirMain_common)]
pub unsafe fn handle_AttackAirMain(fighter: *mut L2CAgent) {
    CancelModule::enable_cancel(sv_system::battle_object_module_accessor((*fighter).lua_state_agent));
    original!()(fighter);
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = game_BatSwing4Common1)]
pub unsafe fn handle_game_BatSwing4Common1(fighter: *mut L2CAgent) {
    println!("[handle_game_BatSwing4Common1]");
    original!()(fighter);
}

#[allow(unused_unsafe)]
#[skyline::hook(replace = game_BatSwing4Common2)]
pub unsafe fn handle_game_BatSwing4Common2(fighter: *mut L2CAgent) {
    println!("[handle_game_BatSwing4Common2]");
    sv_animcmd::frame((*fighter).lua_state_agent, 0x30 as f32);
    if sv_animcmd::is_excute((*fighter).lua_state_agent) {
        hitbox_visualizer::wrap(sv_animcmd::EFFECT_FOLLOW_NO_SCALE,
            &mut (*fighter), 
            &mut [
            L2CValue::new_int(hash40("sys_shield")), L2CValue::new_int(hash40("top")), 
            L2CValue::new_num(0.0), L2CValue::new_num(0.0), L2CValue::new_num(0.0), L2CValue::new_num(0.0), L2CValue::new_num(0.0), L2CValue::new_num(0.0), 
            L2CValue::new_num(1.0), L2CValue::new_bool(false)].to_vec());
    }
    original!()(fighter);
}

fn nro_main(nro: &NroInfo) {
    match nro.name {
        "common" => 
                {
                    println!("Loaded common NRO!");
                    unsafe {
                        let text_start = (*(*nro.module).ModuleObject).module_base;
                        println!("Common Text Start: {:x}", text_start);
                        // raw const_value_table is at : 0x635b70
                        let fighter_status_kind_fall : u64 = 0x8ee6c39e9be4f0b5;
                        // let res = match patch_data_from_text(text_start as *const u8, 0x6362b8, &fighter_status_kind_fall) {
                        //     Ok(v) => format!("Patched!"),
                        //     Err(e) => format!("Error patching with e: {}", e)
                        // };
                        let fighter_kind_popo : u64 = 0x04aa7a0e945950d5;
                        let res = match patch_data_from_text(text_start as *const u8, 0x638d78, &fighter_kind_popo) {
                            Ok(v) => format!("Patched!"),
                            Err(e) => format!("Error patching with e: {}", e)
                        };
                        println!("Result: {}", res);

                        // skyline::install_hook!(handle_AttackAirMain);
                        // skyline::install_hook!(handle_game_BatSwing4Common1);
                        // skyline::install_hook!(handle_game_BatSwing4Common2);
                        println!("Hooked!");
                    }
                },
        "item" => println!("Loaded item NRO!"),
        _ => ()
    }
}

#[skyline::main(name = "test")]
pub fn main() {
    println!("Training modpack initialized.");
    hitbox_visualizer::hitbox_visualization();
    training::training_mods();
    nro::add_hook(nro_main).unwrap();

    // println!("OpenMode_Write: {} {}", nn::fs::OpenMode_OpenMode_Write, nn::fs::OpenMode_OpenMode_Write as i32);
    // let buffer = format!("{:x}", &common::menu as *const _ as u64);
    // println!("Writing training_modpack.log with {}...\n", buffer);

    // fs::File::create("sd:/test.log").unwrap();
}
