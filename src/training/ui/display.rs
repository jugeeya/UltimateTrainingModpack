use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};

use crate::training::ui;

macro_rules! display_parent_fmt {
    ($x:ident) => {
        format!("TrModDisp{}", $x).as_str()
    };
}

macro_rules! display_header_fmt {
    ($x:ident) => {
        format!("TrModDisp{}Header", $x).as_str()
    };
}

macro_rules! display_txt_fmt {
    ($x:ident) => {
        format!("TrModDisp{}Txt", $x).as_str()
    };
}

pub unsafe fn draw(root_pane: &Pane) {
    let notification_idx = 0;

    let queue = &mut ui::notifications::QUEUE;
    let notification = queue.first();

    root_pane
        .find_pane_by_name_recursive(display_parent_fmt!(notification_idx))
        .unwrap()
        .set_visible(notification.is_some());
    if notification.is_none() {
        return;
    }

    let notification = notification.unwrap();
    let color = notification.color;

    root_pane
        .find_pane_by_name_recursive(display_header_fmt!(notification_idx))
        .unwrap()
        .as_textbox()
        .set_text_string(&notification.header);

    let text = root_pane
        .find_pane_by_name_recursive(display_txt_fmt!(notification_idx))
        .unwrap()
        .as_textbox();
    text.set_text_string(&notification.message);
    text.set_default_material_colors();
    text.set_color(color.r, color.g, color.b, color.a);

    let notification = queue.first_mut().unwrap();
    let has_completed = notification.check_completed();
    if has_completed {
        queue.remove(0);
    }
}
