#![feature(proc_macro_hygiene)]
#![feature(with_options)]
#![feature(const_mut_refs)]

mod common;
mod hitbox_visualizer;
mod training;

use crate::common::*;
use training::combo::FRAME_ADVANTAGE;

use skyline::libc::{c_void, fclose, fopen, fwrite, mkdir, remove};
use skyline::nro::{self, NroInfo};

fn nro_main(nro: &NroInfo<'_>) {
    match nro.name {
        "common" => {
            skyline::install_hooks!(
                training::shield::handle_sub_guard_cont,
                training::directional_influence::handle_correct_damage_vector_common,
                training::tech::handle_change_status,
                sys_line_system_control_fighter_hook
            );
        }
        _ => (),
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sys_line_system_control_fighter)]
pub unsafe fn sys_line_system_control_fighter_hook(fighter: &mut smash::lua2cpp::L2CFighterCommon) -> smash::lib::L2CValue {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let ret = original!()(fighter);
    hitbox_visualizer::get_command_flag_cat(module_accessor, 0);
    ret
}

#[skyline::hook(replace = smash::app::lua_bind::ModelModule::set_joint_srt)]
pub unsafe fn handle_set_joint_srt(
    module_accessor: &mut smash::app::BattleObjectModuleAccessor,
    joint: smash::phx::Hash40, 
    x: *const smash::phx::Vector3f, 
    y: *const smash::phx::Vector3f, 
    z: *const smash::phx::Vector3f
) -> u64 {
    println!("[set_joint_srt] Joint: {}", joint.hash);
    println!("x: {:#?}", *x);
    println!("y: {:#?}", *y);
    println!("z: {:#?}", *z);

    original!()(module_accessor, joint, x, y, z)
}

#[skyline::hook(replace = smash::lua2cpp::L2CAgentBase_clean_coroutine)]
pub unsafe fn clean_coroutine_hook(
    agent: &mut smash::lua2cpp::L2CAgentBase,
    index: i32
) -> u64 {
    // println!("[clean_coroutine] lua_state: {:?}, module_accessor: {:?}, index: {}",
    //     agent.lua_state_agent,
    //     agent.lua_state_agentbase,
    //     index);
    if agent.lua_state_agentbase != 0 {
        // let frame = smash::app::lua_bind::MotionModule::frame(agent.lua_state_agentbase as *mut smash::cpp::root::app::BattleObjectModuleAccessor);
        // let entry_id_int =
        // smash::app::lua_bind::WorkModule::get_int(agent.lua_state_agentbase as *mut smash::cpp::root::app::BattleObjectModuleAccessor, *smash::lib::lua_const::FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
        // println!("EntryID: {}, Frame: {}", entry_id_int, frame);
        hitbox_visualizer::get_command_flag_cat(&mut *(agent.lua_state_agentbase as *mut smash::cpp::root::app::BattleObjectModuleAccessor), 0);
    }
    original!()(agent, index)
}

macro_rules! c_str {
    ($l:tt) => { [$l.as_bytes(), "\u{0}".as_bytes()]
                .concat()
                .as_ptr(); }
}

#[skyline::main(name = "training_modpack")]
pub fn main() {
    println!("[Training Modpack] Initialized.");
    skyline::install_hooks!(handle_set_joint_srt);
    hitbox_visualizer::hitbox_visualization();
    training::training_mods();
    nro::add_hook(nro_main).unwrap();

    unsafe {
        let mut buffer = format!("{:x}", MENU as *const _ as u64);
        println!(
            "[Training Modpack] Writing training_modpack.log with {}...",
            buffer
        );
        mkdir(c_str!("sd:/TrainingModpack/"), 0777);

        println!("[Training Modpack] Removing training_modpack_menu.conf...");
        remove(c_str!("sd:/TrainingModpack/training_modpack_menu.conf"));

        let mut f = fopen(
            c_str!("sd:/TrainingModpack/training_modpack.log"),
            c_str!("w"),
        );

        if !f.is_null() {
            fwrite(c_str!(buffer) as *const c_void, 1, buffer.len(), f);
            fclose(f);
        }

        buffer = format!("{:x}", &FRAME_ADVANTAGE as *const _ as u64);
        println!(
            "[Training Modpack] Writing training_modpack_frame_adv.log with {}...",
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
}
