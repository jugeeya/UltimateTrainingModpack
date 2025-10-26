use skyline::nn::ui2d::*;
use smash::ui2d::SmashTextBox;

use crate::common::menu::QUICK_MENU_ACTIVE;
use crate::common::TRAINING_MENU_ADDR;
use crate::training::ui::notifications::*;
use crate::training::ui::PaneExt;
use training_mod_sync::*;

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
    if (*TRAINING_MENU_ADDR).combo_display_toggle == 0 {
        // User has turned off the "combo display" option in the vanilla menu
        // Remove all notifications from the queue so we don't show them
        // This will also set the pane's visibility to false
        clear_all_notifications();
    }

    let notification_idx = 0;
    let mut queue_lock = lock_write(&NOTIFICATIONS_QUEUE);

    let notification = (*queue_lock).first_mut();

    root_pane
        .find_pane_by_name_recursive_expect(display_parent_fmt!(notification_idx))
        .set_visible(notification.is_some() && !read(&QUICK_MENU_ACTIVE));

    if notification.is_none() {
        return;
    }

    let notification = notification.expect("notification not none in draw()");
    notification.tick();
    let color = notification.color;

    if !notification.has_drawn() {
        notification.set_drawn();
        root_pane
            .find_pane_by_name_recursive_expect(display_header_fmt!(notification_idx))
            .as_textbox()
            .set_text_string(&notification.header);

        let text = root_pane
            .find_pane_by_name_recursive_expect(display_txt_fmt!(notification_idx))
            .as_textbox();
        text.set_text_string(&notification.message);
        text.set_default_material_colors();
        text.set_color(color.r, color.g, color.b, color.a);
    }

    if notification.has_completed() {
        (*queue_lock).remove(0);
    }
    drop(queue_lock);
}
