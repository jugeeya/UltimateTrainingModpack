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
use skyline::libc::{size_t, c_int, c_void, strlen, fopen, fwrite, fclose};
use smash::Result;
use skyline::nn;
use skyline::patching::patch_data_from_text;
use skyline::{from_c_str, c_str, hooks::A64HookFunction, logging::hex_dump_ptr};
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
    // hitbox_visualizer::hitbox_visualization();
    training::training_mods();
    nro::add_hook(nro_main).unwrap();

    let buffer = format!("{:x}", &common::menu as *const _ as u64);
    println!("Writing training_modpack.log with {}...\n", buffer);
    unsafe {
        // let f = fopen(c_str("sd:/test.log"), c_str("w"));
        let f = fopen("sd:/SaltySD/training_modpack.log\u{0}".as_bytes().as_ptr(), "w\u{0}".as_bytes().as_ptr());

        println!("File pointer: {:#?}", f);
        if !f.is_null() {
            fwrite(c_str(&buffer) as *const c_void, 1, buffer.len(), f);
            fclose(f);
        }
    }

    // fs::File::create("sd:/test.log").unwrap();
}

// #![feature(proc_macro_hygiene)]

// use smash::hash40;
// use smash::lib::lua_const::{*};
// use smash::lib::{self, L2CAgent, L2CValue};
// use smash::app;
// use smash::app::{lua_bind::*, sv_animcmd, sv_system};
// use skyline::libc::{size_t, c_int, c_void, strlen};
// use smash::Result;
// use skyline::nro::{self, NroInfo};
// use skyline::logging::HexDump;

// extern "C" {
//     #[link_name = "\u{1}_ZN7lua2cpp16L2CFighterCommon17status_Catch_MainEv"]
//     pub fn status_Catch_Main(
//         arg1: *mut L2CAgent
//     ) -> u64;

//     #[link_name = "\u{1}_ZN7lua2cpp16L2CFighterCommon28sub_wait_ground_check_commonEN3lib8L2CValueE"]
//     pub fn sub_wait_ground_check_common(
//         arg1: *mut L2CAgent,
//         arg2: L2CValue
//     ) -> L2CValue;

//     #[link_name = "\u{1}_ZN7lua2cpp16L2CFighterCommon25sub_air_check_fall_commonEv"]
//     pub fn sub_air_check_fall_common(
//         arg1: *mut L2CAgent
//     ) -> L2CValue;

//     #[link_name = "\u{1}_ZN7lua2cpp14L2CFighterBase13change_statusEN3lib8L2CValueES2_"]
//     pub fn change_status(
//         arg1: *mut L2CAgent,
//         arg2: L2CValue,
//         arg3: L2CValue
//     ) -> u64;
// }

// #[allow(unused_unsafe)]
// #[skyline::hook(replace = status_Catch_Main)]
// pub unsafe fn handle_status_Catch_Main(fighter: *mut L2CAgent) {
//     let module_accessor = sv_system::battle_object_module_accessor((*fighter).lua_state_agent);
//     // if CancelModule::is_enable_cancel(module_accessor) {
//     //     let ret = sub_wait_ground_check_common(fighter, L2CValue::new_bool(false));
//     //     println!("{}", HexDump(&ret));
//     //     // sub_air_check_fall_common(fighter);
//     // }
    
//     let situation_kind = StatusModule::situation_kind(module_accessor) as i32;
//     if situation_kind == SITUATION_KIND_AIR {
//         // change_status(fighter,L2CValue::new_int(*FIGHTER_STATUS_KIND_FALL as u64), L2CValue::new_bool(false));
//         StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
//         return;
//     }
//     if WorkModule::is_enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
//         if MotionModule::is_end(module_accessor) {
//             if situation_kind != SITUATION_KIND_GROUND {
//                 return;
//             }
//             // change_status(fighter,L2CValue::new_int(*FIGHTER_STATUS_KIND_WAIT as u64), L2CValue::new_bool(false));
//             StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
//             return;
//         }
//     }

//     // original!()(fighter); // to call original
// }

// fn nro_main(nro: &NroInfo) {
//     match nro.name {
//         "common" => 
//                 {
//                     println!("Loaded common NRO!");
//                     skyline::install_hook!(handle_status_Catch_Main);
//                     println!("change_status: {:p}", change_status as *const());
//                     println!("sub_air_check_fall_common: {:p}", sub_air_check_fall_common as *const());
//                     println!("sub_wait_ground_check_common: {:p}", sub_wait_ground_check_common as *const());
//                 },
//         _ => ()
//     }
// }

// #[skyline::main(name = "test")]
// pub fn main() {
//     println!("Hello from Skyline plugin!");
//     nro::add_hook(nro_main).unwrap();
// }
