use skyline::nn::ui2d::ResColor;

pub static mut QUEUE: Vec<Notification> = vec![];

#[derive(Clone)]
pub struct Notification {
    pub header: String,
    pub message: String,
    length: u32,
    pub color: ResColor,
    has_drawn: bool,
}

impl Notification {
    pub fn new(header: String, message: String, length: u32, color: ResColor) -> Notification {
        Notification {
            header,
            message,
            length,
            color,
            has_drawn: false,
        }
    }

    pub fn set_drawn(&mut self) {
        self.has_drawn = true;
    }

    pub fn has_drawn(&mut self) -> bool {
        self.has_drawn
    }

    pub fn tick(&mut self) {
        self.length -= 1;
    }

    // Returns: has_completed
    pub fn check_completed(&mut self) -> bool {
        if self.length <= 1 {
            return true;
        }
        false
    }
}

pub fn notification(header: String, message: String, len: u32) {
    unsafe {
        let queue = &mut QUEUE;
        queue.push(Notification::new(
            header,
            message,
            len,
            ResColor {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
        ));
    }
}

pub fn color_notification(header: String, message: String, len: u32, color: ResColor) {
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
