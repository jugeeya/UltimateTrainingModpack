use skyline::nn::ui2d::ResColor;

pub static mut QUEUE: Vec<Notification> = vec![];

#[derive(Clone)]
pub struct Notification {
    pub header: String,
    pub message: String,
    length: u32,
    pub color: ResColor,
}

impl Notification {
    pub fn new(header: String, message: String, length: u32, color: ResColor) -> Notification {
        Notification {
            header,
            message,
            length,
            color,
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
