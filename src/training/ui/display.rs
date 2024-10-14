use std::ptr::addr_of_mut;

use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};

use crate::common::menu::QUICK_MENU_ACTIVE;
use crate::common::TRAINING_MENU_ADDR;
use crate::sync::*;
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

    let queue = addr_of_mut!(ui::notifications::QUEUE);
    if (*TRAINING_MENU_ADDR).combo_display_toggle == 0 {
        // User has turned off the "combo display" option in the vanilla menu
        // Remove all notifications from the queue so we don't show them
        // This will also set the pane's visibility to false
        (*queue).clear();
    }

    let notification = (*queue).first_mut();

    root_pane
        .find_pane_by_name_recursive(display_parent_fmt!(notification_idx))
        .unwrap()
        .set_visible(notification.is_some() && !read_rwlock(&QUICK_MENU_ACTIVE));
    if notification.is_none() {
        return;
    }

    let notification = notification.unwrap();
    notification.tick();
    let color = notification.color;

    if !notification.has_drawn() {
        notification.set_drawn();
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
    }

    let has_completed = notification.check_completed();
    if has_completed {
        (*queue).remove(0);
    }
}
