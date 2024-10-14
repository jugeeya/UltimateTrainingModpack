use std::sync::LazyLock;

use skyline::error::show_error;
use skyline::hook;
use skyline::hooks::A64InlineHook;
use skyline::text_iter::{add_get_imm, adrp_get_imm, Instruction::*, TextIter};
use smash::app::smashball::is_training_mode;

use crate::common::consts::*;
use crate::logging::*;

use HazardState::*;
use HookState::*;

enum HazardState {
    Begin,
    Adrp1,
    Add2,
    Ldur3,
    Ldrb4,
    Ldr5,
}

enum HookState {
    Begin,
    Adrp1,
    Ldrsw2,
}

static HAZARD_FLAG_ADDRESS: LazyLock<usize> = LazyLock::new(get_hazard_flag_address);
static LOAD_ADDRESS: LazyLock<usize> = LazyLock::new(get_hazard_hook_address);

fn get_hazard_flag_address() -> usize {
    let mut state = HazardState::Begin;
    let mut flag_pos = 0;
    for (pos, instr) in TextIter::new() {
        state = match (state, instr) {
            (HazardState::Begin, Adrp { .. }) => {
                flag_pos = pos;
                HazardState::Adrp1
            }
            (HazardState::Adrp1, Add { .. }) => Add2,
            (Add2, Ldur { .. }) => Ldur3,
            (Ldur3, Ldrb { .. }) => Ldrb4,
            (Ldrb4, Ldr { .. }) => Ldr5,
            (Ldr5, Sub { .. }) => {
                break;
            }
            _ => {
                flag_pos = 0;
                HazardState::Begin
            }
        }
    }
    if flag_pos == 0 {
        return 0x0;
    }
    let program_counter = flag_pos & !0xFFF; // Need program counter to mimic ADRP
    let adrp = unsafe { adrp_get_imm(*(flag_pos as *mut u32)) as usize };
    let add = unsafe { add_get_imm(*((flag_pos + 4) as *mut u32)) as usize };
    program_counter + adrp + add + 0x9
}

fn get_hazard_hook_address() -> usize {
    let mut state = HookState::Begin;
    let mut flag_pos = 0;
    for (pos, instr) in TextIter::new() {
        state = match (state, instr) {
            (HookState::Begin, Adrp { .. }) => HookState::Adrp1,
            (HookState::Adrp1, Ldrsw { .. }) => Ldrsw2,
            (Ldrsw2, Cbz { .. }) => {
                flag_pos = pos;
                break;
            }
            _ => {
                flag_pos = 0;
                HookState::Begin
            }
        }
    }

    flag_pos
}

#[hook(offset = *LOAD_ADDRESS, inline)]
fn hazard_intercept(_ctx: &skyline::hooks::InlineCtx) {
    unsafe {
        if is_training_mode() {
            mod_handle_hazards();
        }
    }
}

fn mod_handle_hazards() {
    unsafe {
        let address = *HAZARD_FLAG_ADDRESS as *mut u8;
        *address = (MENU.stage_hazards == OnOff::ON) as u8;
    }
}

unsafe fn validate_hazards_addrs() -> Result<(), ()> {
    let mut error_string: String = String::new();
    let mut error_id = 0;

    if *HAZARD_FLAG_ADDRESS == 0 {
        error_string += &String::from("The Ultimate Training Modpack was unable to locate stage loading code in your version of the game.\n\n");
        error_id += 1000;
    }
    if *LOAD_ADDRESS == 0 {
        error_string += &String::from("The Ultimate Training Modpack was unable to locate the global hazard address in your version of the game.\n\n");
        error_id += 1000;
    }

    if error_id != 0 {
        error_string += "Please report this along with your game version.\n\nHazard control will be disabled for this launch.\n\n";
        show_error(
            error_id,
            "Failed to apply stage hazard control mods.\n",
            error_string.as_str(),
        );
        return Err(());
    }

    Ok(())
}

pub fn hazard_manager() {
    info!("Applying hazard control mods.");
    unsafe {
        if let Ok(()) = validate_hazards_addrs() {
            A64InlineHook(
                (*LOAD_ADDRESS) as *const skyline::libc::c_void,
                hazard_intercept as *const skyline::libc::c_void,
            );
        }
    }
}
