use crate::common::FIGHTER_MANAGER_ADDR;
use crate::hitbox_visualizer;
use skyline::nn::ro::LookupSymbol;
use smash::app::{self, lua_bind::*};
use smash::lib::{lua_const::*};
use crate::common::*;

pub mod directional_influence;
pub mod shield;
pub mod tech;

mod ledge;
mod mash;
mod save_states;

#[skyline::hook(replace = WorkModule::get_param_float)]
pub unsafe fn handle_get_param_float(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    param_type: u64,
    param_hash: u64,
) -> f32 {
    shield::get_param_float(module_accessor, param_type, param_hash)
        .unwrap_or_else(|| original!()(module_accessor, param_type, param_hash))
}

#[skyline::hook(replace = ControlModule::get_attack_air_kind)]
pub unsafe fn handle_get_attack_air_kind(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> i32 {
    // bool replace;
    // int kind = InputRecorder::get_attack_air_kind(module_accessor, replace);
    // if (replace) return kind;

    mash::get_attack_air_kind(module_accessor).unwrap_or_else(|| original!()(module_accessor))
}

static mut FRAME_COUNTER : u64 = 0;
static mut PLAYER_ACTIONABLE : bool = false;
static mut CPU_ACTIONABLE : bool = false;
static mut PLAYER_ACTIVE_FRAME : u64 = 0;
static mut CPU_ACTIVE_FRAME : u64 = 0;
static mut FRAME_ADVANTAGE_CHECK : bool = false;

pub unsafe fn was_in_hitstun(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    (*FIGHTER_STATUS_KIND_DAMAGE..=*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&prev_status)
}

pub unsafe fn was_in_shieldstun(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    prev_status == FIGHTER_STATUS_KIND_GUARD_DAMAGE
}

pub unsafe fn get_module_accessor(entry_id_int: i32) -> *mut app::BattleObjectModuleAccessor {
    let entry_id = app::FighterEntryID(entry_id_int);
    let mgr = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
    let fighter_information =
        FighterManager::get_fighter_information(mgr, entry_id) as *mut app::FighterInformation;
    let fighter_entry =
        FighterManager::get_fighter_entry(mgr, entry_id) as *mut app::FighterEntry;
    let current_fighter_id = FighterEntry::current_fighter_id(fighter_entry);
    app::sv_battle_object::module_accessor(current_fighter_id as u32)

}

pub unsafe fn is_actionable(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    WorkModule::is_enable_transition_term(
    module_accessor, 
    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) ||
    WorkModule::is_enable_transition_term(
        module_accessor, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) ||
    WorkModule::is_enable_transition_term(
        module_accessor, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) ||
    WorkModule::is_enable_transition_term(
        module_accessor, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) ||
    CancelModule::is_enable_cancel(module_accessor)
}

#[skyline::hook(replace = ControlModule::get_command_flag_cat)]
pub unsafe fn handle_get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) -> i32 {
    save_states::save_states(module_accessor);

    let mut flag = original!()(module_accessor, category);

    if is_training_mode() {
        if category == 0 {
            let entry_id_int =
            WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
            // do only once.
            if entry_id_int == 0 {
                let player_module_accessor = get_module_accessor(0);
                let cpu_module_accessor = get_module_accessor(1);

                // Use to factor in that we should only update frame advantage if 
                // there's been a hit that connects
                // if AttackModule::is_infliction(
                //     player_module_accessor, 
                //     *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {

                // }

                // the frame the fighter *becomes* actionable
                if !CPU_ACTIONABLE && is_actionable(cpu_module_accessor) {
                    CPU_ACTIVE_FRAME = FRAME_COUNTER;
                }

                if !PLAYER_ACTIONABLE && is_actionable(player_module_accessor) {
                    PLAYER_ACTIVE_FRAME = FRAME_COUNTER;
                }
                
                CPU_ACTIONABLE = is_actionable(cpu_module_accessor);
                PLAYER_ACTIONABLE = is_actionable(player_module_accessor);

                // if neither are active
                if !CPU_ACTIONABLE && !PLAYER_ACTIONABLE {
                    FRAME_ADVANTAGE_CHECK = true;
                }

                // if both are now active
                if PLAYER_ACTIONABLE && CPU_ACTIONABLE {
                    if FRAME_ADVANTAGE_CHECK {
                        if was_in_hitstun(cpu_module_accessor) || was_in_shieldstun(cpu_module_accessor) {
                            let frame_advantage : i64;
                            if PLAYER_ACTIVE_FRAME > CPU_ACTIVE_FRAME {
                                frame_advantage = (PLAYER_ACTIVE_FRAME - CPU_ACTIVE_FRAME) as i64 * -1;
                            } else {
                                frame_advantage = (CPU_ACTIVE_FRAME - PLAYER_ACTIVE_FRAME) as i64;
                            }
                            println!("Frame advantage: {}", frame_advantage);
                        }
                        
                        FRAME_ADVANTAGE_CHECK = false;
                    }
                }

                FRAME_COUNTER += 1;
            }
        }
    }

    // bool replace;
    // int ret = InputRecorder::get_command_flag_cat(module_accessor, category, flag, replace);
    // if (replace) return ret;

    shield::get_command_flag_cat(module_accessor);
    mash::get_command_flag_cat(module_accessor, category, &mut flag);
    ledge::get_command_flag_cat(module_accessor, category, &mut flag);
    tech::get_command_flag_cat(module_accessor, category, &mut flag);
    hitbox_visualizer::get_command_flag_cat(module_accessor, category);

    flag
}

// int get_pad_flag(u64 module_accessor) {
//     u64 control_module = load_module(module_accessor, 0x48);
//     int (*get_pad_flag)(u64) = (int (*)(u64)) load_module_impl(control_module, 0x348);
//     int pad_flag = get_pad_flag(control_module);

//     bool replace;
//     int ret = InputRecorder::get_pad_flag(module_accessor, replace);
//     if (replace) return ret;

//     return pad_flag;
// }

// float get_stick_x_replace(u64 module_accessor) {
//     u64 control_module = load_module(module_accessor, 0x48);
//     float (*get_stick_x)(u64) = (float (*)(u64)) load_module_impl(control_module, 0x178);
//     float stick_x = get_stick_x(control_module);

//     bool replace;
//     float ret = InputRecorder::get_stick_x(module_accessor, replace);
//     if (replace) return ret;

//     return stick_x;
// }

// float get_stick_y_replace(u64 module_accessor) {
//     u64 control_module = load_module(module_accessor, 0x48);
//     float (*get_stick_y)(u64) = (float (*)(u64)) load_module_impl(control_module, 0x188);
//     float stick_y = get_stick_y(control_module);

//     bool replace;
//     float ret = InputRecorder::get_stick_y(module_accessor, replace);
//     if (replace) return ret;

//     return stick_y;
// }

#[skyline::hook(replace = ControlModule::check_button_on)]
pub unsafe fn handle_check_button_on(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> bool {
    shield::check_button_on(module_accessor, button).unwrap_or_else(|| {
        mash::check_button_on(module_accessor, button).unwrap_or_else(|| {
            tech::check_button_on(module_accessor, button).unwrap_or_else(|| {
                ledge::check_button_on(module_accessor, button)
                    .unwrap_or_else(|| original!()(module_accessor, button))
            })
        })
    })
}

#[skyline::hook(replace = ControlModule::check_button_off)]
pub unsafe fn handle_check_button_off(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> bool {
    shield::check_button_off(module_accessor, button)
        .unwrap_or_else(|| original!()(module_accessor, button))
}

#[skyline::hook(replace = MotionModule::change_motion)]
pub unsafe fn handle_change_motion(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    motion_kind: u64,
    unk1: f32,
    unk2: f32,
    unk3: bool,
    unk4: f32,
    unk5: bool,
    unk6: bool,
) -> u64 {
    let motion_kind = tech::change_motion(module_accessor, motion_kind).unwrap_or(motion_kind);

    original!()(
        module_accessor,
        motion_kind,
        unk1,
        unk2,
        unk3,
        unk4,
        unk5,
        unk6,
    )
}

pub fn training_mods() {
    println!("[Training Modpack] Applying training mods.");
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
        );
    }

    skyline::install_hooks!(
        // Mash airdodge/jump
        handle_get_command_flag_cat,
        // Hold/Infinite shield
        handle_check_button_on,
        handle_check_button_off,
        handle_get_param_float,
        // Mash attack
        handle_get_attack_air_kind,
        // Tech options
        handle_change_motion,
    );

    // // Input recorder
    // SaltySD_function_replace_sym(
    //     "_ZN3app8lua_bind31ControlModule__get_stick_x_implEPNS_26BattleObjectModuleAccessorE",
    //     (u64)&ControlModule::get_stick_x_replace);
    // SaltySD_function_replace_sym(
    //     "_ZN3app8lua_bind31ControlModule__get_stick_y_implEPNS_26BattleObjectModuleAccessorE",
    //     (u64)&ControlModule::get_stick_y_replace);
}
