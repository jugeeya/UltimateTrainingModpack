use std::collections::VecDeque;
use lazy_static::lazy_static;
use parking_lot::Mutex;

pub static mut QUEUE: Vec<Notification<'static>> = vec![];

#[derive(Copy, Clone)]
pub struct Notification<'a> {
    message: &'a str,
    length: u32
}

impl<'a> Notification<'a> {
    pub fn new(msg: &'a str, len: u32) -> Notification {
        Notification {
            message: msg,
            length: len
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