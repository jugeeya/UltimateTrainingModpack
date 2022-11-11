use lazy_static::lazy_static;
use parking_lot::Mutex;
use skyline::hooks::InlineCtx;
use std::collections::{HashMap, VecDeque};

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
        let mut queue = &mut QUEUE;
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

// MSBT strings without arguments
#[skyline::hook(offset = 0x3778bf4, inline)]
unsafe fn handle_msbt_get_by_label(ctx: &mut InlineCtx) {
    // Label is in SP+0xE0
    let msbt_label =
        skyline::from_c_str((ctx as *const InlineCtx as *const u8).add(0x100).add(0xE0));

    println!("MSBT Label: {msbt_label}");
    if MSBT_LABEL_REPLACE.contains_key(&msbt_label) {
        let mut text = MSBT_LABEL_REPLACE.get(&msbt_label).unwrap().to_string();
        text.push('\0');

        let text_vec: Vec<u16> = text.encode_utf16().collect();
        *ctx.registers[0].x.as_mut() = text_vec.as_ptr() as u64;
    }
}

// MSBT strings with templates
#[skyline::hook(offset = 0x37a1244, inline)]
unsafe fn handle_msbt_get_by_label_with_arguments(ctx: &mut InlineCtx) {
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
        let mut bytes: Vec<u16> = text.encode_utf16().collect();
        *ctx.registers[1].x.as_mut() = bytes.as_ptr() as u64;
        *ctx.registers[2].w.as_mut() = bytes.len() as u32;
    }

    if MSBT_LABEL_REPLACE.contains_key(&msbt_label) {
        let mut text = MSBT_LABEL_REPLACE.get(&msbt_label).unwrap().to_string();
        let mut bytes: Vec<u16> = text.encode_utf16().collect();
        *ctx.registers[1].x.as_mut() = bytes.as_ptr() as u64;
        *ctx.registers[2].w.as_mut() = bytes.len() as u32;
    }
}

pub fn init() {
    skyline::install_hooks!(
        handle_msbt_get_by_label,
        handle_msbt_get_by_label_with_arguments
    );
}
