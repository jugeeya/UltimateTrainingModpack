use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};

use crate::common::{menu::QUICK_MENU_ACTIVE, PauseMenu, TRAINING_MENU_ADDR};
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
    let menu = TRAINING_MENU_ADDR as *const PauseMenu;
    let cc_displayed = (*menu).combo_display_toggle != 0;
    let notification_idx = 0;

    let queue = &mut ui::notifications::QUEUE;
    let notification = queue.first_mut();

    root_pane
        .find_pane_by_name_recursive(display_parent_fmt!(notification_idx))
        .unwrap()
        .set_visible(notification.is_some() && !QUICK_MENU_ACTIVE && cc_displayed); // TODO: Add check for combo counter on here
    if notification.is_none() {
        return;
    }

    let notification = notification.unwrap();
    let color = notification.color;

    if !cc_displayed {
        // Set the notification to drawn so we don't draw it
        notification.set_drawn();
        notification.force_complete();
    }

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
        queue.remove(0);
    }
}
