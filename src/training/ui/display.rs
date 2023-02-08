use crate::training::ui;
use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};

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

pub unsafe fn draw(root_pane: &mut Pane) {
    let notification_idx = 0;

    let queue = &mut ui::notifications::QUEUE;
    let notification = queue.first_mut();

    if let Some(parent) = root_pane.find_pane_by_name_recursive(display_parent_fmt!(notification_idx)) {
        parent.set_visible(notification.is_some());
        if notification.is_none() {
            return;
        }
    }

    let notification = notification.unwrap();
    let header_txt = notification.header();
    let message = notification.message();
    let color = notification.color();
    let has_completed = notification.tick();
    if has_completed {
        queue.remove(0);
    }

    if let Some(header) = root_pane.find_pane_by_name_recursive(display_header_fmt!(notification_idx)) {
        header.as_textbox().set_text_string(header_txt);
    }

    if let Some(text) = root_pane.find_pane_by_name_recursive(display_txt_fmt!(notification_idx)) {
        let text = text.as_textbox();
        text.set_text_string(message);
        text.set_default_material_colors();
        text.set_color(color.r, color.g, color.b, color.a);
    }
}