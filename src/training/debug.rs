#![allow(dead_code)] // For Debug
#![allow(unused_imports)]
//#![cfg(debug_assertions)]
use crate::common::is_operation_cpu;
use smash::app::{self, lua_bind::*, smashball::is_training_mode, utility};
use smash::lib::lua_const::*;

#[skyline::from_offset(0x1655400)]
fn is_visible_backshield(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool;

#[repr(C)]
pub struct WorkModule2 {
    vtable: u64,
    owner: &'static mut app::BattleObjectModuleAccessor,
}

static ON_FLAG_OFFSET: usize = 0x4e4910;
#[skyline::hook(offset = ON_FLAG_OFFSET)]
pub unsafe fn handle_on_flag(work_module: &mut WorkModule2, address: i32) {
    if address == *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_FLAG_OUTFIELD_INVISIBLE
        && app::utility::get_kind(work_module.owner) != *FIGHTER_KIND_SHEIK
    {
        is_visible_backshield(work_module.owner);
    }
    original!()(work_module, address);
}

static SET_INT_OFFSET: usize = 0x4e4600;
#[skyline::hook(offset = SET_INT_OFFSET)]
pub unsafe fn handle_set_int(work_module: &mut WorkModule2, value: u32, address: i32) {
    if !is_training_mode() {
        original!()(work_module, value, address);
    }
    if address == *WEAPON_PTRAINER_MBALL_INSTANCE_WORK_ID_INT_PLATE_EFF_ID
        && app::utility::get_kind(work_module.owner) == *WEAPON_KIND_PTRAINER_MBALL
    {
        is_visible_backshield(work_module.owner);
    }
    original!()(work_module, value, address);
}

static SET_INT64_OFFSET: usize = 0x4e4680;
#[skyline::hook(offset = SET_INT64_OFFSET)]
pub unsafe fn handle_set_int_64(work_module: &mut WorkModule2, value: u64, address: i32) {
    if !is_training_mode() {
        original!()(work_module, value, address);
    }
    original!()(work_module, value, address);
}

static SET_FLOAT_OFFSET: usize = 0x4e4420;
#[skyline::hook(offset = SET_FLOAT_OFFSET)]
pub unsafe fn handle_set_float(work_module: &mut WorkModule2, value: f32, address: i32) {
    if !is_training_mode() {
        original!()(work_module, value, address);
    }
    if address == *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO //*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY_ON_START
        && app::utility::get_kind(work_module.owner) == FIGHTER_KIND_KIRBY
    {
        is_visible_backshield(work_module.owner);
    }
    original!()(work_module, value, address);
}

static IS_FLAG_OFFSET: usize = 0x4e48e0;
#[skyline::hook(offset = IS_FLAG_OFFSET)]
pub unsafe fn handle_is_flag(work_module: &mut WorkModule2, address: i32) -> bool {
    if !is_training_mode() {
        original!()(work_module, address);
    }
    if address == *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_FLAG_ENABLE_CHANGE_POKEMON //*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY_ON_START
        && app::utility::get_kind(work_module.owner) != *FIGHTER_KIND_SHEIK
        && original!()(work_module, address)
    {
        is_visible_backshield(work_module.owner);
    }
    original!()(work_module, address)
}

static GET_INT_OFFSET: usize = 0x4e45e0;
#[skyline::hook(offset = GET_INT_OFFSET)]
pub unsafe fn handle_get_int(work_module: &mut WorkModule2, address: i32) {
    if !is_training_mode() {
        original!()(work_module, address);
    }
    original!()(work_module, address);
}

pub fn init() {
    skyline::install_hooks!(
        //handle_on_flag,
        //handle_set_int,
        // handle_set_int_64,
        handle_set_float,
        // handle_get_int,
        //handle_is_flag,
    );
}

// Example Call:

// print_fighter_info(
//     module_accessor,
//     "DebugTest",
//     true,
//     false,
//     true,
//     true,
//     vec![
//         ("FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT", FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT),
//     ],
//     Vec::new(),
//     vec![
//         ("FIGHTER_STATUS_CLIFF_FLAG_TO_FALL", FIGHTER_STATUS_CLIFF_FLAG_TO_FALL),
//     ],
// );
#[allow(clippy::too_many_arguments)] // This function has so many arguments so it's easy to quickly fill them in when debugging with the analyzer
pub fn print_fighter_info(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    title: &str,
    player_only: bool,
    cpu_only: bool,
    print_fighter_kind: bool,
    print_status: bool,
    work_int_pairs: Vec<(&str, i32)>,
    work_float_pairs: Vec<(&str, i32)>,
    work_flag_pairs: Vec<(&str, i32)>,
) {
    unsafe {
        // Don't print for fighters we don't want to
        let is_cpu = is_operation_cpu(module_accessor);
        if (player_only && is_cpu) || (cpu_only && !is_cpu) {
            return;
        }
        // Print Title
        print!("{}: ", title);
        // Print Fighter Kind:
        if print_fighter_kind {
            print!("FIGHTER_KIND: {}, ", utility::get_kind(module_accessor));
        }
        // Print Status:
        if print_status {
            print!(
                "FIGHTER_STATUS: {}, ",
                StatusModule::status_kind(module_accessor)
            );
        }

        // Print Work Ints:
        for work_int_pair in work_int_pairs {
            print!(
                "{}: {}, ",
                work_int_pair.0,
                WorkModule::get_int(module_accessor, work_int_pair.1)
            );
        }

        // Print Work Floats:
        for work_float_pair in work_float_pairs {
            print!(
                "{}: {}, ",
                work_float_pair.0,
                WorkModule::get_float(module_accessor, work_float_pair.1)
            );
        }

        // Print Work Flags:
        for work_flag_pair in work_flag_pairs {
            print!(
                "{}: {}, ",
                work_flag_pair.0,
                WorkModule::is_flag(module_accessor, work_flag_pair.1)
            );
        }

        // End Line
        println!("|");
    }
}
