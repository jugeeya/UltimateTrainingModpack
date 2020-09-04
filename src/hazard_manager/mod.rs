#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
use crate::common::{consts::*, *};
use skyline::hooks::{getRegionAddress, Region};
use skyline::hooks::A64InlineHook;
use skyline::hook;
use skyline::error::show_error;
use smash::app::smashball::is_training_mode;
use std::{iter::StepBy, ops::Range};

struct TextIter<InnerIter: Iterator<Item = usize> + Sized> {
    inner: InnerIter
}

impl TextIter<StepBy<Range<usize>>> {
    fn new() -> Self {
        unsafe {
            let text = getRegionAddress(Region::Text) as usize;
            let rodata = getRegionAddress(Region::Rodata) as usize;

            Self {
                inner: (text..rodata).step_by(4)
            }
        }
    }
}

impl<InnerIter: Iterator<Item = usize> + Sized> Iterator for TextIter<InnerIter> {
    type Item = (usize, Instruction);

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = self.inner.next()? as *const u32;
        let raw_instr = unsafe { *ptr };
        Some((ptr as usize, Instruction::from_u32(raw_instr)))
    }
}
 
const LDR_MASK: u32     = 0b1111111111_000000000000_00000_00000; // 64-bit LDR unsigned offset
const LDR_MASKED: u32   = 0b1111100101_000000000000_00000_00000;
 
const ADD_MASK: u32     = 0b11111111_00_000000000000_00000_00000; // 64-bit ADD immediate
const ADD_MASKED: u32   = 0b10010001_00_000000000000_00000_00000; 
 
const ADRP_MASK: u32    = 0b1_00_11111_0000000000000000000_00000; 
const ADRP_MASKED: u32  = 0b1_00_10000_0000000000000000000_00000;
 
const LDUR_MASK: u32    = 0b11_111_1_11_11_1_000000000_00_00000_00000;
const LDUR_MASKED: u32  = 0b11_111_0_00_01_0_000000000_00_00000_00000;
 
const LDRB_MASK: u32    = 0b11_111_1_11_11_0_00000_000_0_00_00000_00000; // LDRB immediate Unsigned offset
const LDRB_MASKED: u32  = 0b00_111_0_01_01_0_00000_000_0_00_00000_00000;
 
const SUB_MASK: u32     = 0b1_1_1_11111_00_0_00000_000000_00000_00000; // 32-bit SUB Immediate
const SUB_MASKED: u32   = 0b0_1_0_10001_00_0_00000_000000_00000_00000;
 
const AND_MASK: u32     = 0b1_11_111111_1_000000_000000_00000_00000; // 32-bit AND immediate
const AND_MASKED: u32   = 0b0_00_100100_0_000000_000000_00000_00000;
 
const LDRSW_MASK: u32   = 0b11_111_1_11_11_000000000000_00000_00000; // 64-bit LDRSW unsigned offset
const LDRSW_MASKED: u32 = 0b10_111_0_01_10_000000000000_00000_00000;

const CBZ_MASK: u32     = 0b1_111111_1_0000000000000000000_00000; // 32-bit CBZ
const CBZ_MASKED: u32   = 0b0_011010_0_0000000000000000000_00000;

const CMP_MASK: u32     = 0b1_1_1_11111_00_000000000000_00000_00000; // 32-bit CMP immediate
const CMP_MASKED: u32   = 0b0_1_1_10001_00_000000000000_00000_00000;

const BCS_MASK: u32    = 0b1111111_1_0000000000000000000_1_1111; // B.cond jump
const BCS_MASKED: u32  = 0b0101010_0_0000000000000000000_0_0010;

enum Instruction {
    Ldr {
        imm: u16,
        rn: u8,
        rt: u8
    },
    Add {
        shift: u8,
        imm: u16,
        rn: u8,
        rd: u8
    },
    Adrp {
        imm: u32,
        rd: u8
    },
    Ldur {
        imm: u16,
        rn: u8,
        rt: u8
    },
    Ldrb {
        imm: u16,
        rn: u8,
        rt: u8
    },
    Sub {
        shift: u8,
        imm: u16,
        rn: u8,
        rd: u8
    },
    And {
        imm: u16,
        rn: u8,
        rd: u8
    },
    Mov {
        imm: u8,
        rm: u8,
        rn: u8,
        rd: u8
    },
    Bl {
        imm: u32
    },
    Ldrsw {
        imm: u16,
        rn: u8,
        rt: u8
    },
    Cbz {
        imm: u32,
        rt: u8
    },
    Cmp {
        shift: u8,
        imm: u16,
        rn: u8
    },
    BCs {
        imm: u32,
        cond: u8
    },
    Unk(u32),
}

impl Instruction {
    fn u32_as_ldr(val: u32) -> Option<Self> {
        if val & LDR_MASK == LDR_MASKED {
            Some(Instruction::Ldr {
                imm: ((val >> 10) & 0xFFF) as u16,
                rn: ((val >> 5) & 0x1F) as u8,
                rt: (val & 0x1F) as u8
            })
        }
        else {
            None
        }
    }

    fn u32_as_add(val: u32) -> Option<Self> {
        if val & ADD_MASK == ADD_MASKED {
            Some(Instruction::Add {
                shift: ((val >> 22) & 0x3) as u8,
                imm: ((val >> 10) & 0xFFF) as u16,
                rn: ((val >> 5) & 0x1F) as u8,
                rd: (val & 0x1F) as u8
            })
        }
        else {
            None
        }
    }

    fn u32_as_adrp(val: u32) -> Option<Self> {
        if val & ADRP_MASK == ADRP_MASKED {
            let immhi = (val >> 5) & 0x7FFFF;
            let immlo = (val >> 29) & 0x3;
            Some(Instruction::Adrp {
                imm: (immhi << 14) + (immlo << 12),
                rd: (val & 0x1F) as u8
            })
        }
        else {
            None
        }
    }

    fn u32_as_ldur(val: u32) -> Option<Self> {
        if val & LDUR_MASK == LDUR_MASKED {
            Some(Instruction::Ldur {
                imm: ((val >> 12) & 0x1FF) as u16,
                rn: ((val >> 5) & 0x1F) as u8,
                rt: ((val >> 5) & 0x1F) as u8
            })
        }
        else {
            None
        }
    }

    fn u32_as_ldrb(val: u32) -> Option<Self> {
        if val & LDRB_MASK == LDRB_MASKED {
            Some(Instruction::Ldrb {
                imm: ((val >> 10) & 0xFFF) as u16,
                rn: ((val >> 5) & 0x1F) as u8,
                rt: (val & 0x1F) as u8
            })
        }
        else {
            None
        }
    }

    fn u32_as_sub(val: u32) -> Option<Self> {
        if val & SUB_MASK == SUB_MASKED {
            Some(Instruction::Sub {
                shift: ((val >> 22) & 0x3) as u8,
                imm: ((val >> 10) & 0xFFF) as u16,
                rn: ((val >> 5) & 0x1F) as u8,
                rd: (val & 0x1F) as u8
            })
        }
        else {
            None
        }
    }

    fn u32_as_and(val: u32) -> Option<Self> {
        if val & AND_MASK == AND_MASKED {
            let immr = (val >> 16) & 0x3F;
            let imms = (val >> 10) & 0x3F;
            Some(Instruction::And {
                imm: ((imms << 6) + immr) as u16,
                rn: ((val >> 5) & 0x1F) as u8,
                rd: (val & 0x1F) as u8
            })
        }
        else {
            None
        }
    }

    fn u32_as_cbz(val: u32) -> Option<Self> {
        if val & CBZ_MASK == CBZ_MASKED {
            Some(Instruction::Cbz {
                imm: ((val >> 5) & 0x7FFFF),
                rt: (val & 0x1F) as u8
            })
        }
        else {
            None
        }
    }

    fn u32_as_ldrsw(val: u32) -> Option<Self> {
        if val & LDRSW_MASK == LDRSW_MASKED {
            Some(Instruction::Ldrsw {
                imm: ((val >> 10) & 0xFFF) as u16,
                rn: ((val >> 5) & 0x1F) as u8,
                rt: (val & 0x1F) as u8
            })
        }
        else {
            None
        }
    }

    fn u32_as_cmp(val: u32) -> Option<Self> {
        if val & CMP_MASK == CMP_MASKED {
            Some(Instruction::Cmp {
                shift: ((val >> 22) & 0x3) as u8,
                imm: ((val >> 10) & 0xFFF) as u16,
                rn: ((val >> 5) & 0x1F) as u8
            })
        }
        else {
            None
        }
    }

    fn u32_as_bcs(val: u32) -> Option<Self> {
        if val & BCS_MASK == BCS_MASKED {
            Some(Instruction::BCs {
                imm: ((val >> 5) & 0x7FFFF),
                cond: (val & 0xF) as u8
            })
        }
        else {
            None
        }
    }

    fn from_u32(val: u32) -> Self {
        Self::u32_as_ldr(val)
            .or_else(|| Self::u32_as_add(val))
            .or_else(|| Self::u32_as_adrp(val))
            .or_else(|| Self::u32_as_ldur(val))
            .or_else(|| Self::u32_as_ldrb(val))
            .or_else(|| Self::u32_as_sub(val))
            .or_else(|| Self::u32_as_and(val))
            .or_else(|| Self::u32_as_cbz(val))
            .or_else(|| Self::u32_as_cmp(val))
            .or_else(|| Self::u32_as_bcs(val))
            .or_else(|| Self::u32_as_ldrsw(val))
            .unwrap_or(Self::Unk(val))
    }
}

enum HazardState {
    Begin,
    Adrp1,
    Add2,
    Ldur3,
    Ldrb4,
    Ldr5
}

enum HookState {
    Begin,
    Adrp1,
    Ldrsw2
}

use HookState::*;
use HazardState::*;
use Instruction::*;

fn adrp_get_imm(instr: u32) -> u32 {
    let immhi = (instr >> 5) & 0x7FFFF;
    let immlo = (instr >> 29) & 0x3;
    return (immhi << 14) + (immlo << 12);
}

fn add_get_imm(instr: u32) -> u32 {
    return (instr >> 10) & 0xFFF;
}

fn get_hazard_flag_address() -> usize {
    let mut state = HazardState::Begin;
    let mut flag_pos = 0;
    for (pos, instr) in TextIter::new() {
        state = match (state, instr) {
            (HazardState::Begin, Adrp { .. }) => {
                flag_pos = pos;
                HazardState::Adrp1
            },
            (HazardState::Adrp1, Add { .. }) => Add2,
            (Add2, Ldur { .. }) => Ldur3,
            (Ldur3, Ldrb { .. }) => Ldrb4,
            (Ldrb4, Ldr { .. }) => Ldr5,
            (Ldr5, Sub { .. }) => {
                break;
            },
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
    return program_counter + adrp + add + 0x9;
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
    return flag_pos;
}

// 8.1.0 Defaults
static mut HAZARD_FLAG_ADDRESS: *mut u8 = 0x4ebbf95 as *mut u8;
static mut LOAD_ADDRESS: usize = 0x214bde8;

#[hook(offset = LOAD_ADDRESS, inline)]
fn hazard_intercept(ctx: &skyline::hooks::InlineCtx) {
    unsafe {
        if is_training_mode() {
            mod_handle_hazards();
        }
    }
}

fn mod_handle_hazards() {
    unsafe {
        if MENU.stage_hazards == StageHazards::On {
            *HAZARD_FLAG_ADDRESS = 0x1;
        }
        else {
            *HAZARD_FLAG_ADDRESS = 0x0;
        }
    }
}

fn display_hazards_error(is_flag_addr: bool) {
    let mut error_string: String = String::new();
    let mut error_id = 0;
    if is_flag_addr {
        error_string = String::from("The Ultimate Training Modpack was unable to locate stage loading code in your version of the game.\n\nPlease report this along with your game version.\n\nHazard control will be disabled for this launch.\n\n");
        error_id     = 1000;
    }
    else {
        error_string = String::from("The Ultimate Training Modpack was unable to locate the global hazard address in your version of the game.\n\nPlease report this along with your game version.\n\nHazard control will be disabled for this launch.\n\n");
        error_id     = 2000;
    }
    show_error(
        error_id,
        "Failed to apply stage hazard control mods.\n",
        error_string.as_str()
    );
}

pub fn hazard_manager() {
    println!("[Training Modpack] Applying hazard control mods.");
    unsafe {
        HAZARD_FLAG_ADDRESS = get_hazard_flag_address() as *mut u8;
        LOAD_ADDRESS = get_hazard_hook_address();
        if HAZARD_FLAG_ADDRESS == 0 as *mut u8 {
            display_hazards_error(true);
            return;
        }
        if LOAD_ADDRESS == 0 {
            display_hazards_error(false);
            return;
        }
        A64InlineHook(LOAD_ADDRESS as *const skyline::libc::c_void, hazard_intercept as *const skyline::libc::c_void);
    }
}