#![feature(proc_macro_hygiene)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(with_options)]

mod common;
mod hitbox_visualizer;
mod training;

use crate::common::consts::*;
use crate::common::*;

use skyline::c_str;
use skyline::libc::{c_void, fclose, fopen, fwrite};
use skyline::nro::{self, NroInfo};
use smash::app::lua_bind::*;
use smash::app::sv_system::{self};
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

#[allow(unused_unsafe)]
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_guard_cont)]
pub unsafe fn handle_sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if (*menu).MASH_STATE == MASH_ATTACK && (*menu).ATTACK_STATE == MASH_GRAB {
        if StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_STATUS_KIND_GUARD_DAMAGE {
            if WorkModule::get_int(
                module_accessor,
                *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME,
            ) == 0
            {
                if WorkModule::is_enable_transition_term(
                    module_accessor,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
                ) {
                    fighter.fighter_base.change_status(
                        L2CValue::new_int(*FIGHTER_STATUS_KIND_CATCH as u64),
                        L2CValue::new_bool(true),
                    );
                }
            }
        }
    }
    original!()(fighter)
}

fn nro_main(nro: &NroInfo) {
    match nro.name {
        "common" => {
            println!("Loaded common NRO!");
            skyline::install_hook!(handle_sub_guard_cont);
        }
        _ => (),
    }
}

#[skyline::main(name = "test")]
pub fn main() {
    println!("Training modpack initialized.");
    hitbox_visualizer::hitbox_visualization();
    training::training_mods();
    nro::add_hook(nro_main).unwrap();

    unsafe {
        common::menu = &mut common::menu_struct;
        let buffer = format!("{:x}", common::menu as u64);
        println!("Writing training_modpack.log with {}...\n", buffer);
        let f = fopen(
            "sd:/SaltySD/training_modpack.log\u{0}".as_bytes().as_ptr(),
            "w\u{0}".as_bytes().as_ptr(),
        );

        println!("File pointer: {:#?}", f);
        if !f.is_null() {
            fwrite(c_str(&buffer) as *const c_void, 1, buffer.len(), f);
            fclose(f);
        }
    }
}
