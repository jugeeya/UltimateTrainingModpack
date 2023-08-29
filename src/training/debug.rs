#![allow(dead_code)] // For Debug
#![allow(unused_imports)]
use smash::app::{self, lua_bind::*, smashball::is_training_mode};
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
pub unsafe fn handle_on_flag(
    work_module: &mut WorkModule2,
    address: i32,
) {
    if !is_training_mode() {
        original!()(work_module, address);
    }
    if address == *FIGHTER_POKEMON_INSTANCE_WORK_ID_FLAG_RESTART_FROM_MBALL
        && app::utility::get_kind((*work_module).owner) != *FIGHTER_KIND_SHEIK
    {
        is_visible_backshield(work_module.owner);
    }
    original!()(work_module, address);
}

static SET_INT_OFFSET: usize = 0x4e4600;
#[skyline::hook(offset = SET_INT_OFFSET)]
pub unsafe fn handle_set_int(
    work_module: &mut WorkModule2,
    value: u32,
    address: i32,
) {
    if !is_training_mode() {
        original!()(work_module, value, address);
    }
    if address == *WEAPON_PTRAINER_MBALL_INSTANCE_WORK_ID_INT_PLATE_EFF_ID
        && app::utility::get_kind((*work_module).owner) != *FIGHTER_KIND_SHEIK
    {
        is_visible_backshield(work_module.owner);
    }
    original!()(work_module, value, address);
}

static SET_INT64_OFFSET: usize = 0x4e4680;
#[skyline::hook(offset = SET_INT64_OFFSET)]
pub unsafe fn handle_set_int_64(
    work_module: &mut WorkModule2,
    value: u64,
    address: i32,
) {
    if !is_training_mode() {
        original!()(work_module, value, address);
    }
    original!()(work_module, value, address);
}

static SET_FLOAT_OFFSET: usize = 0x4e4420;
#[skyline::hook(offset = SET_FLOAT_OFFSET)]
pub unsafe fn handle_set_float(
    work_module: &mut WorkModule2,
    value: f32,
    address: i32,
) {
    if !is_training_mode() {
        original!()(work_module, value, address);
    }
    original!()(work_module, value, address);
}

static IS_FLAG_OFFSET: usize = 0x4e48e0;
#[skyline::hook(offset = IS_FLAG_OFFSET)]
pub unsafe fn handle_is_flag(
    work_module: &mut WorkModule2,
    address: i32,
) {
    if !is_training_mode() {
        original!()(work_module, address);
    }
    if address == 0x20000104 //*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY_ON_START
        && app::utility::get_kind((*work_module).owner) == *FIGHTER_KIND_KIRBY
    {
        is_visible_backshield(work_module.owner);
    }
    original!()(work_module, address);
}

static GET_INT_OFFSET: usize = 0x4e45e0;
#[skyline::hook(offset = GET_INT_OFFSET)]
pub unsafe fn handle_get_int(
    work_module: &mut WorkModule2,
    address: i32,
) {
    if !is_training_mode() {
        original!()(work_module, address);
    }
    original!()(work_module, address);
}



pub fn init() {
    skyline::install_hooks!(
        handle_on_flag,
        handle_set_int,
        // handle_set_int_64,
        // handle_set_float,
        // handle_get_int,
        // handle_is_flag,
    );
}

// Copy Setup Args: SomeModuleAccessor, 1, 0x55 (Terry?), true, false - in training, swallowed terry
