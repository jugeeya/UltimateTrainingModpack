use skyline::hooks::InlineCtx;

pub static mut QUEUE: Vec<Notification<'static>> = vec![];

#[derive(Copy, Clone)]
pub struct Notification<'a> {
    message: &'a str,
    length: u32,
}

impl<'a> Notification<'a> {
    pub fn new(msg: &'a str, len: u32) -> Notification {
        Notification {
            message: msg,
            length: len,
        }
    }

    // Returns: has_completed
    pub fn tick(&mut self) -> bool {
        if self.length <= 1 {
            return true;
        }
        self.length -= 1;
        false
    }

    pub fn message(self) -> &'a str {
        self.message
    }

    pub fn length(self) -> u32 {
        self.length
    }
}

pub fn new_notification(msg: &'static str, len: u32) {
    unsafe {
        let queue = &mut QUEUE;
        // We have to account for the fact that we only tick every 5 frames
        queue.push(Notification::new(msg, len / 5));
    }
}

use phf::phf_map;

static MSBT_LABEL_REPLACE: phf::Map<&'static str, &'static str> = phf_map! {
    "mel_training_dmg_sum" => "Save State",
    "mel_training_hit" => "Input Recording",
    "mel_training_dmg" => "Frame Advantage",
};

// MSBT Label -> Pane pointer
static mut LABEL_POINTER_DMG_SUM_N : *mut u64 = 0 as *mut u64;
static mut LABEL_POINTER_DMG_SUM_DECIMAL_N : *mut u64 = 0 as *mut u64; 
static mut LABEL_POINTER_DMG_HIT_N : *mut u64 = 0 as *mut u64; 
static mut LABEL_POINTER_DMG_DMG_N : *mut u64 = 0 as *mut u64; 
static mut LABEL_POINTER_DMG_DMG_DECIMAL_N : *mut u64 = 0 as *mut u64; 

// MSBT strings without arguments
#[skyline::hook(offset = 0x3778bf4, inline)]
unsafe fn intercept_msbt_get_by_label(ctx: &mut InlineCtx) {
    // Label is in SP+0xE0
    let msbt_label =
        skyline::from_c_str((ctx as *const InlineCtx as *const u8).add(0x100).add(0xE0));

    if MSBT_LABEL_REPLACE.contains_key(&msbt_label) {
        let mut text = MSBT_LABEL_REPLACE.get(&msbt_label).unwrap().to_string();
        text.push('\0');

        let text_vec: Vec<u16> = text.encode_utf16().collect();
        *ctx.registers[0].x.as_mut() = text_vec.as_ptr() as u64;
    }
}

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr()
    };
}

pub unsafe fn update_text(msbt_label: &str, new_text: &str) {
    let ptr = match msbt_label {
        "mel_training_dmg_sum_n" => LABEL_POINTER_DMG_SUM_N,
        "mel_training_dmg_sum_decimal_n" => LABEL_POINTER_DMG_SUM_DECIMAL_N,
        "mel_training_hit_n" => LABEL_POINTER_DMG_HIT_N,
        "mel_training_dmg_n" => LABEL_POINTER_DMG_DMG_N,
        "mel_training_dmg_decimal_n" => LABEL_POINTER_DMG_DMG_DECIMAL_N,
        _ => 0 as *mut u64
    };

    println!("Updating pane for label {msbt_label}");
    if ptr.is_null() {
        println!("Pointer for {msbt_label} is null!");
    } else {
        *ptr = c_str!(new_text) as u64;
    }
}

// // This is actually a varargs function. Inline hooks please save me.
// #[skyline::from_offset(0x3778c50)]
// extern "C" fn msbt_get_by_label_with_arguments(pane: u64, msbt_label: *const u8, num_args: u32, arg1: u32);

// #[skyline::hook(offset = 0x37a1130, inline)]
// // LayoutPane *param_1,undefined8 param_2,int param_3,undefined8 *param_4
// unsafe fn handle_msbt_get_by_label_with_arguments(ctx: &mut InlineCtx) {
//     let pane = *ctx.registers[0].x.as_ref() as u64;
//     let msbt_label_str = skyline::from_c_str(*ctx.registers[1].x.as_ref() as *const u8);
//     match msbt_label_str.as_str() {
//         "mel_training_dmg_sum_n" => LABEL_POINTER_DMG_SUM_N = pane,
//         "mel_training_dmg_sum_decimal_n" => LABEL_POINTER_DMG_SUM_DECIMAL_N = pane,
//         "mel_training_hit_n" => LABEL_POINTER_DMG_HIT_N = pane,
//         "mel_training_dmg_n" => LABEL_POINTER_DMG_DMG_N = pane,
//         "mel_training_dmg_decimal_n" => LABEL_POINTER_DMG_DMG_DECIMAL_N = pane,
//         _ => ()
//     };
// }


pub unsafe fn update_combo_counters() {
    // println!("Updating pane pointer {TRAINING_COMBO_PANE_POINTER:x}");
    // update_combo_counters_internal(TRAINING_COMBO_PANE_POINTER);
}

// #[skyline::from_offset(0x13e8d60)]
// extern "C" fn update_combo_counters_internal(pane: u64);

// static mut TRAINING_COMBO_PANE_POINTER : u64 = 0;
// #[skyline::hook(offset = 0x13e8d60)]
// unsafe fn intercept_update_combo_counters_internal(ctx: &mut InlineCtx) {
    // println!("Grabbing pane pointer {TRAINING_COMBO_PANE_POINTER:x}");
    // TRAINING_COMBO_PANE_POINTER = *ctx.registers[0].x.as_ref() as u64;
// }


// MSBT strings with templates
#[skyline::hook(offset = 0x37a1244, inline)]
unsafe fn intercept_msbt_get_by_label_with_arguments(ctx: &mut InlineCtx) {
    let msbt_label = skyline::from_c_str(*ctx.registers[22].x.as_ref() as *const u8);
    let frame_advantage = format!("{}", crate::training::combo::FRAME_ADVANTAGE);
    let new_text = match msbt_label.as_str() {
        "mel_training_dmg_sum_n" => Some("I used to be Total Damage!"),
        "mel_training_dmg_sum_decimal_n" => Some(""),
        "mel_training_hit_n" => Some("I used to be Combo!"),
        "mel_training_dmg_n" => Some(frame_advantage.as_str()),
        "mel_training_dmg_decimal_n" => Some(""),
        _ => None,
    };

    if let Some(text) = new_text {
        let bytes: Vec<u16> = text.encode_utf16().collect();
        let ptr = ctx.registers[1].x.as_mut() as *mut u64;
        match msbt_label.as_str() {
            "mel_training_dmg_sum_n" => LABEL_POINTER_DMG_SUM_N = ptr,
            "mel_training_dmg_sum_decimal_n" => LABEL_POINTER_DMG_SUM_DECIMAL_N = ptr,
            "mel_training_hit_n" => LABEL_POINTER_DMG_HIT_N = ptr,
            "mel_training_dmg_n" => LABEL_POINTER_DMG_DMG_N = ptr,
            "mel_training_dmg_decimal_n" => LABEL_POINTER_DMG_DMG_DECIMAL_N = ptr,
            _ => ()
        };
        *ptr = bytes.as_ptr() as u64;
        *ctx.registers[2].w.as_mut() = bytes.len() as u32;
    }
}

// Some other place used for Combo boxes
#[skyline::hook(offset = 0x37a0580, inline)]
unsafe fn intercept_msbt_text_something(ctx: &mut InlineCtx) {
    let msbt_label = skyline::from_c_str(*((*ctx.registers[21].x.as_ref() as *const u8).add(0xE0) as *const *const u8) as *const u8);
    
    if MSBT_LABEL_REPLACE.contains_key(&msbt_label) {
        let mut text = MSBT_LABEL_REPLACE.get(&msbt_label).unwrap().to_string();
        text.push('\0');

        let text_vec: Vec<u16> = text.encode_utf16().collect();
        *ctx.registers[1].x.as_mut() = text_vec.as_ptr() as u64;
    }
}

pub fn init() {
    skyline::install_hooks!(
        intercept_msbt_get_by_label,
        intercept_msbt_get_by_label_with_arguments,
        // handle_msbt_get_by_label_with_arguments,
        intercept_update_combo_counters_internal,
        intercept_msbt_text_something
    );
}
