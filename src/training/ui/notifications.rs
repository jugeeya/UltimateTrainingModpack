use skyline::nn::ui2d::ResColor;

pub static mut QUEUE: Vec<Notification<'static>> = vec![];

#[derive(Copy, Clone)]
pub struct Notification<'a> {
    header: &'a str,
    message: &'a str,
    length: u32,
    color: ResColor
}

impl<'a> Notification<'a> {
    pub fn new(header: &'a str, message: &'a str, length: u32, color: ResColor) -> Notification<'a> {
        Notification {
            header,
            message,
            length,
            color
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

    pub fn header(self) -> &'a str {
        self.header
    }

    pub fn message(self) -> &'a str {
        self.message
    }

    pub fn color(self) -> ResColor {
        self.color
    }
}

pub fn notification(header: &'static str, message: &'static str, len: u32) {
    unsafe {
        let queue = &mut QUEUE;
        queue.push(Notification::new(header, message, len, ResColor {
            r: 0,
            g: 0,
            b: 0,
            a: 255
        }));
    }
}

pub fn color_notification(header: &'static str, message: &'static str, len: u32, color: ResColor) {
    unsafe {
        let queue = &mut QUEUE;
        queue.push(Notification::new(header, message, len, color));
    }
}

pub fn clear_notifications(header: &'static str) {
    unsafe {
        let queue = &mut QUEUE;
        queue.retain(|notif| notif.header != header);
    }
}