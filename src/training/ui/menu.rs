use std::collections::HashMap;

use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};
use training_mod_tui::{
    App, AppPage, ConfirmationState, SliderState, NX_SUBMENU_COLUMNS, NX_SUBMENU_ROWS,
};

use crate::common::menu::{
    MENU_CLOSE_FRAME_COUNTER, MENU_CLOSE_WAIT_FRAMES, MENU_RECEIVED_INPUT, P1_CONTROLLER_STYLE,
    QUICK_MENU_ACTIVE, QUICK_MENU_APP,
};
use crate::input::*;
use crate::training::frame_counter;
use training_mod_consts::TOGGLE_MAX;
use training_mod_sync::*;

use super::fade_out;
use super::set_icon_text;

const BG_LEFT_ON_WHITE_COLOR: ResColor = ResColor {
    r: 0,
    g: 28,
    b: 118,
    a: 255,
};

const BG_LEFT_ON_BLACK_COLOR: ResColor = ResColor {
    r: 0,
    g: 22,
    b: 112,
    a: 0,
};

const BG_LEFT_OFF_WHITE_COLOR: ResColor = ResColor {
    r: 8,
    g: 13,
    b: 17,
    a: 255,
};

const BG_LEFT_OFF_BLACK_COLOR: ResColor = ResColor {
    r: 5,
    g: 10,
    b: 14,
    a: 0,
};

const BG_LEFT_SELECTED_BLACK_COLOR: ResColor = ResColor {
    r: 240,
    g: 154,
    b: 7,
    a: 0,
};

const BG_LEFT_SELECTED_WHITE_COLOR: ResColor = ResColor {
    r: 255,
    g: 166,
    b: 7,
    a: 255,
};

pub static VANILLA_MENU_ACTIVE: RwLock<bool> = RwLock::new(false);

static GCC_BUTTON_MAPPING: LazyLock<HashMap<&'static str, u16>> = LazyLock::new(|| {
    HashMap::from([
        ("L", 0xE204),
        ("R", 0xE205),
        ("X", 0xE206),
        ("Y", 0xE207),
        ("Z", 0xE208),
    ])
});
static PROCON_BUTTON_MAPPING: LazyLock<HashMap<&'static str, u16>> = LazyLock::new(|| {
    HashMap::from([
        ("L", 0xE0E4),
        ("R", 0xE0E5),
        ("X", 0xE0E2),
        ("Y", 0xE0E3),
        ("ZL", 0xE0E6),
        ("ZR", 0xE0E7),
    ])
});

unsafe fn render_submenu_page(app: &mut App, root_pane: &Pane) {
    let tabs_clone = app.tabs.clone(); // Need this to avoid double-borrow later on
    let tab = app.selected_tab();
    for row in 0..NX_SUBMENU_ROWS {
        let menu_button_row = root_pane
            .find_pane_by_name_recursive(format!("TrModMenuButtonRow{row}").as_str())
            .unwrap();
        menu_button_row.set_visible(true);
        for col in 0..NX_SUBMENU_COLUMNS {
            if let Some(submenu) = tab.submenus.get(row, col) {
                // Find all the panes we need to modify
                let menu_button = menu_button_row
                    .find_pane_by_name_recursive(format!("Button{col}").as_str())
                    .unwrap();
                let title_text = menu_button
                    .find_pane_by_name_recursive("TitleTxt")
                    .unwrap()
                    .as_textbox();
                let title_bg = menu_button
                    .find_pane_by_name_recursive("TitleBg")
                    .unwrap()
                    .as_picture();
                let title_bg_material = &mut *title_bg.material;
                let is_selected = row == tab.submenus.state.selected_row().unwrap()
                    && col == tab.submenus.state.selected_col().unwrap();

                // Set Pane Visibility
                title_text.set_text_string(&t!(submenu.title));

                // In the actual 'layout.arc' file, every icon image is stacked
                // into a single container pane, with each image directly on top of another.
                // Hide all icon images, and strategically mark the icon that
                // corresponds with a particular button to be visible.

                for t in tabs_clone.iter() {
                    for s in t.submenus.iter() {
                        let pane = menu_button.find_pane_by_name_recursive(s.id);
                        if let Some(p) = pane {
                            p.set_visible(s.id == submenu.id);
                        }
                    }
                }

                menu_button
                    .find_pane_by_name_recursive("check")
                    .unwrap()
                    .set_visible(false);

                for value in 1..=TOGGLE_MAX {
                    if let Some(pane) =
                        menu_button.find_pane_by_name_recursive(format!("{}", value).as_str())
                    {
                        pane.set_visible(false);
                    } else {
                        break;
                    }
                }

                if is_selected {
                    // Help text
                    root_pane
                        .find_pane_by_name_recursive("FooterTxt")
                        .unwrap()
                        .as_textbox()
                        .set_text_string(&t!(submenu.help_text));

                    title_bg_material.set_white_res_color(BG_LEFT_ON_WHITE_COLOR);
                    title_bg_material.set_black_res_color(BG_LEFT_ON_BLACK_COLOR);
                    title_text.text_shadow_enable(true);
                    title_text.text_outline_enable(true);
                    title_text.set_color(255, 255, 255, 255);
                } else {
                    title_bg_material.set_white_res_color(BG_LEFT_OFF_WHITE_COLOR);
                    title_bg_material.set_black_res_color(BG_LEFT_OFF_BLACK_COLOR);
                    title_text.text_shadow_enable(false);
                    title_text.text_outline_enable(false);
                    title_text.set_color(178, 199, 211, 255);
                }
                menu_button.set_visible(true);
                menu_button
                    .find_pane_by_name_recursive("Icon")
                    .unwrap()
                    .set_visible(true);
            }
        }
    }
}

unsafe fn render_toggle_page(app: &mut App, root_pane: &Pane) {
    let tabs_clone = app.tabs.clone(); // Need this to avoid double-borrow later on
    let submenu = app.selected_submenu();
    // If the options can only be toggled on or off, then use the check icon
    // instead of the number icons
    let use_check_icon = submenu.toggles.get(0, 0).unwrap().max == 1;
    for row in 0..NX_SUBMENU_ROWS {
        let menu_button_row = root_pane
            .find_pane_by_name_recursive(format!("TrModMenuButtonRow{row}").as_str())
            .unwrap();
        menu_button_row.set_visible(true);
        for col in 0..NX_SUBMENU_COLUMNS {
            if let Some(toggle) = submenu.toggles.get(row, col) {
                let menu_button = menu_button_row
                    .find_pane_by_name_recursive(format!("Button{col}").as_str())
                    .unwrap();
                menu_button.set_visible(true);
                let title_text = menu_button
                    .find_pane_by_name_recursive("TitleTxt")
                    .unwrap()
                    .as_textbox();
                let title_bg = menu_button
                    .find_pane_by_name_recursive("TitleBg")
                    .unwrap()
                    .as_picture();
                let title_bg_material = &mut *title_bg.material;
                let is_selected = row == submenu.toggles.state.selected_row().unwrap()
                    && col == submenu.toggles.state.selected_col().unwrap();

                if is_selected {
                    title_text.text_shadow_enable(true);
                    title_text.text_outline_enable(true);
                    title_text.set_color(255, 255, 255, 255);
                    title_bg_material.set_white_res_color(BG_LEFT_ON_WHITE_COLOR);
                    title_bg_material.set_black_res_color(BG_LEFT_ON_BLACK_COLOR);
                } else {
                    title_text.text_shadow_enable(false);
                    title_text.text_outline_enable(false);
                    title_text.set_color(178, 199, 211, 255);
                    title_bg_material.set_white_res_color(BG_LEFT_OFF_WHITE_COLOR);
                    title_bg_material.set_black_res_color(BG_LEFT_OFF_BLACK_COLOR);
                }

                // Hide all submenu icons, since we're not on the submenu page
                for t in tabs_clone.iter() {
                    for s in t.submenus.iter() {
                        let pane = menu_button.find_pane_by_name_recursive(s.id);
                        if let Some(p) = pane {
                            p.set_visible(false);
                        }
                    }
                }

                title_text.set_text_string(&t!(toggle.title));

                if use_check_icon {
                    menu_button
                        .find_pane_by_name_recursive("check")
                        .unwrap()
                        .set_visible(true);

                    menu_button
                        .find_pane_by_name_recursive("Icon")
                        .unwrap()
                        .set_visible(toggle.value > 0);
                } else {
                    menu_button
                        .find_pane_by_name_recursive("check")
                        .unwrap()
                        .set_visible(false);
                    menu_button
                        .find_pane_by_name_recursive("Icon")
                        .unwrap()
                        .set_visible(toggle.value > 0);

                    // Note there's no pane for 0
                    for value in 1..=toggle.max {
                        let err_msg = format!("Could not find pane with name {}", value);
                        menu_button
                            .find_pane_by_name_recursive(format!("{}", value).as_str())
                            .expect(&err_msg)
                            .set_visible(value == toggle.value);
                    }
                }
            }
        }
    }
}

unsafe fn render_slider_page(app: &mut App, root_pane: &Pane) {
    let submenu = app.selected_submenu();
    let slider = submenu.slider.unwrap();
    let selected_min = slider.lower;
    let selected_max = slider.upper;

    let slider_pane = root_pane
        .find_pane_by_name_recursive("TrModSlider")
        .unwrap();
    slider_pane.set_visible(true);

    let _background = slider_pane
        .find_pane_by_name_recursive("Background")
        .unwrap()
        .as_picture();
    let header = slider_pane
        .find_pane_by_name_recursive("Header")
        .unwrap()
        .as_textbox();
    header.set_text_string(submenu.title);
    let min_button = slider_pane
        .find_pane_by_name_recursive("MinButton")
        .unwrap()
        .as_picture();
    let max_button = slider_pane
        .find_pane_by_name_recursive("MaxButton")
        .unwrap()
        .as_picture();
    let min_title_text = min_button
        .find_pane_by_name_recursive("TitleTxt")
        .unwrap()
        .as_textbox();
    let min_title_bg = min_button
        .find_pane_by_name_recursive("TitleBg")
        .unwrap()
        .as_picture();
    let min_value_text = min_button
        .find_pane_by_name_recursive("ValueTxt")
        .unwrap()
        .as_textbox();
    let max_title_text = max_button
        .find_pane_by_name_recursive("TitleTxt")
        .unwrap()
        .as_textbox();
    let max_title_bg = max_button
        .find_pane_by_name_recursive("TitleBg")
        .unwrap()
        .as_picture();
    let max_value_text = max_button
        .find_pane_by_name_recursive("ValueTxt")
        .unwrap()
        .as_textbox();

    min_title_text.set_text_string("Min");
    match slider.state {
        SliderState::LowerHover | SliderState::LowerSelected => {
            min_title_text.text_shadow_enable(true);
            min_title_text.text_outline_enable(true);
            min_title_text.set_color(255, 255, 255, 255);
        }
        _ => {
            min_title_text.text_shadow_enable(false);
            min_title_text.text_outline_enable(false);
            min_title_text.set_color(178, 199, 211, 255);
        }
    }

    max_title_text.set_text_string("Max");
    match slider.state {
        SliderState::UpperHover | SliderState::UpperSelected => {
            max_title_text.text_shadow_enable(true);
            max_title_text.text_outline_enable(true);
            max_title_text.set_color(255, 255, 255, 255);
        }
        _ => {
            max_title_text.text_shadow_enable(false);
            max_title_text.text_outline_enable(false);
            max_title_text.set_color(178, 199, 211, 255);
        }
    }

    min_value_text.set_text_string(&format!("{selected_min}"));
    max_value_text.set_text_string(&format!("{selected_max}"));

    let min_title_bg_material = &mut *min_title_bg.as_picture().material;
    let min_colors = match slider.state {
        SliderState::LowerHover => (BG_LEFT_ON_WHITE_COLOR, BG_LEFT_ON_BLACK_COLOR),
        SliderState::LowerSelected => (BG_LEFT_SELECTED_WHITE_COLOR, BG_LEFT_SELECTED_BLACK_COLOR),
        _ => (BG_LEFT_OFF_WHITE_COLOR, BG_LEFT_OFF_BLACK_COLOR),
    };

    min_title_bg_material.set_white_res_color(min_colors.0);
    min_title_bg_material.set_black_res_color(min_colors.1);

    let max_title_bg_material = &mut *max_title_bg.as_picture().material;
    let max_colors = match slider.state {
        SliderState::UpperHover => (BG_LEFT_ON_WHITE_COLOR, BG_LEFT_ON_BLACK_COLOR),
        SliderState::UpperSelected => (BG_LEFT_SELECTED_WHITE_COLOR, BG_LEFT_SELECTED_BLACK_COLOR),
        _ => (BG_LEFT_OFF_WHITE_COLOR, BG_LEFT_OFF_BLACK_COLOR),
    };

    max_title_bg_material.set_white_res_color(max_colors.0);
    max_title_bg_material.set_black_res_color(max_colors.1);

    min_value_text.set_visible(true);
    max_value_text.set_visible(true);

    // Hide the Icon pane for MinButton and MaxButton
    [min_button, max_button].iter().for_each(|button| {
        let icon = button.find_pane_by_name_recursive("Icon").unwrap();
        icon.set_visible(false);
    });
}

unsafe fn render_confirmation_page(app: &mut App, root_pane: &Pane) {
    let show_row = 3; // Row in the middle of the page
    let show_cols = [1, 2]; // Columns in the middle of the page
    let no_col = show_cols[0]; // Left
    let yes_col = show_cols[1]; // Right
    let help_text = match app.confirmation_return_page {
        AppPage::TOGGLE | AppPage::SLIDER => {
            "Are you sure you want to reset the current setting to the defaults?"
        }
        AppPage::SUBMENU => "Are you sure you want to reset ALL settings to the defaults?",
        _ => "", // Shouldn't ever get this case, but don't panic if we do
    };

    // Set help text
    root_pane
        .find_pane_by_name_recursive("FooterTxt")
        .unwrap()
        .as_textbox()
        .set_text_string(help_text);

    // Show only the buttons that we care about
    for row in 0..NX_SUBMENU_ROWS {
        let should_show_row = row == show_row;
        let menu_button_row = root_pane
            .find_pane_by_name_recursive(format!("TrModMenuButtonRow{row}").as_str())
            .unwrap();
        menu_button_row.set_visible(should_show_row);
        if should_show_row {
            for col in 0..NX_SUBMENU_COLUMNS {
                let should_show_col = show_cols.contains(&col);
                let menu_button = menu_button_row
                    .find_pane_by_name_recursive(format!("Button{col}").as_str())
                    .unwrap();
                menu_button.set_visible(should_show_col);
                if should_show_col {
                    let title_text = menu_button
                        .find_pane_by_name_recursive("TitleTxt")
                        .unwrap()
                        .as_textbox();
                    let title_bg = menu_button
                        .find_pane_by_name_recursive("TitleBg")
                        .unwrap()
                        .as_picture();
                    let title_bg_material = &mut *title_bg.material;

                    if col == no_col {
                        title_text.set_text_string("No");
                    } else if col == yes_col {
                        title_text.set_text_string("Yes");
                    }
                    let is_selected = (col == no_col
                        && app.confirmation_state == ConfirmationState::HoverNo)
                        || (col == yes_col
                            && app.confirmation_state == ConfirmationState::HoverYes);

                    if is_selected {
                        title_text.text_shadow_enable(true);
                        title_text.text_outline_enable(true);
                        title_text.set_color(255, 255, 255, 255);
                        title_bg_material.set_white_res_color(BG_LEFT_ON_WHITE_COLOR);
                        title_bg_material.set_black_res_color(BG_LEFT_ON_BLACK_COLOR);
                    } else {
                        title_text.text_shadow_enable(false);
                        title_text.text_outline_enable(false);
                        title_text.set_color(178, 199, 211, 255);
                        title_bg_material.set_white_res_color(BG_LEFT_OFF_WHITE_COLOR);
                        title_bg_material.set_black_res_color(BG_LEFT_OFF_BLACK_COLOR);
                    }

                    // Hide all submenu icons, since we're not on the submenu page
                    // TODO: Do we want to show the check on "Yes" and a red "X" on "No?"
                    for t in app.tabs.iter() {
                        for s in t.submenus.iter() {
                            let pane = menu_button.find_pane_by_name_recursive(s.id);
                            if let Some(p) = pane {
                                p.set_visible(false);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub unsafe fn draw(root_pane: &Pane) {
    // Determine if we're in the menu by seeing if the "help" footer has
    // begun moving upward. It starts at -80 and moves to 0 over 10 frames
    // in info_training_in_menu.bflan
    assign(
        &VANILLA_MENU_ACTIVE,
        root_pane
            .find_pane_by_name_recursive("L_staying_help")
            .unwrap()
            .pos_y
            != -80.0,
    );

    let overall_parent_pane = root_pane.find_pane_by_name_recursive("TrModMenu").unwrap();
    overall_parent_pane.set_visible(read(&QUICK_MENU_ACTIVE) && !read(&VANILLA_MENU_ACTIVE));
    let menu_close_wait_frame = frame_counter::get_frame_count(*MENU_CLOSE_FRAME_COUNTER);
    fade_out(
        overall_parent_pane,
        MENU_CLOSE_WAIT_FRAMES - menu_close_wait_frame,
        MENU_CLOSE_WAIT_FRAMES,
    );

    // Only submit updates if we have received input
    let received_input = read(&MENU_RECEIVED_INPUT);
    if !received_input {
        return;
    } else {
        assign(&MENU_RECEIVED_INPUT, false);
    }

    if let Some(quit_button) = root_pane.find_pane_by_name_recursive("TrModTitle") {
        for quit_txt_s in &["set_txt_00", "set_txt_01"] {
            if let Some(quit_txt) = quit_button.find_pane_by_name_recursive(quit_txt_s) {
                quit_txt
                    .as_textbox()
                    .set_text_string(&t!("common.modpack_menu"));
            }
        }
    }

    // Make all invisible first
    for row_idx in 0..NX_SUBMENU_ROWS {
        for col_idx in 0..NX_SUBMENU_COLUMNS {
            let menu_button_row = root_pane
                .find_pane_by_name_recursive(format!("TrModMenuButtonRow{row_idx}").as_str())
                .unwrap();
            menu_button_row.set_visible(false);

            let menu_button = menu_button_row
                .find_pane_by_name_recursive(format!("Button{col_idx}").as_str())
                .unwrap();
            menu_button.set_visible(false);

            menu_button
                .find_pane_by_name_recursive("ValueTxt")
                .unwrap()
                .set_visible(false);
        }
    }

    // Make normal training panes invisible if we're active
    // InfluencedAlpha means "Should my children panes' alpha be influenced by mine, as the parent?"
    let status_r_pane = root_pane
        .find_pane_by_name_recursive("status_R")
        .expect("Unable to find status_R pane");
    // status_r_pane.flags |= 1 << PaneFlag::InfluencedAlpha as u8;
    status_r_pane.set_visible(!read(&QUICK_MENU_ACTIVE));

    root_pane
        .find_pane_by_name_recursive("TrModSlider")
        .unwrap()
        .set_visible(false);

    // Update menu display
    // Grabbing lock as read-only, essentially
    let mut app = lock_write(&QUICK_MENU_APP);
    // We don't really need to change anything, but get_before_selected requires &mut self

    let tab_titles = [
        app.tabs
            .get_before_selected()
            .expect("No tab selected!")
            .title,
        app.tabs.get_selected().expect("No tab selected!").title,
        app.tabs
            .get_after_selected()
            .expect("No tab selected!")
            .title,
    ];

    let is_gcc = read(&P1_CONTROLLER_STYLE) == ControllerStyle::GCController;
    let button_mapping = if is_gcc {
        &(*GCC_BUTTON_MAPPING)
    } else {
        &(*PROCON_BUTTON_MAPPING)
    };

    let (x_key, y_key, l_key, r_key, zl_key, zr_key, z_key) = (
        button_mapping.get("X"),
        button_mapping.get("Y"),
        button_mapping.get("L"),
        button_mapping.get("R"),
        button_mapping.get("ZL"),
        button_mapping.get("ZR"),
        button_mapping.get("Z"),
    );

    let (left_tab_key, right_tab_key, save_defaults_key, reset_key, clear_toggle_key) = if is_gcc {
        (l_key, r_key, x_key, z_key, y_key)
    } else {
        (zl_key, zr_key, x_key, r_key, y_key)
    };

    [
        (left_tab_key, "LeftTab"),
        (None, "CurrentTab"),
        (right_tab_key, "RightTab"),
    ]
    .iter()
    .enumerate()
    .for_each(|(idx, (key, name))| {
        let key_help_pane = root_pane.find_pane_by_name_recursive(name).unwrap();

        let icon_pane = key_help_pane
            .find_pane_by_name_recursive("set_txt_icon")
            .unwrap()
            .as_textbox();
        let help_pane = key_help_pane
            .find_pane_by_name_recursive("set_txt_help")
            .unwrap()
            .as_textbox();
        icon_pane.set_text_string("");

        // Left/Right tabs have keys
        if let Some(key) = key {
            set_icon_text(icon_pane, &[**key]);
        }

        if *name == "CurrentTab" {
            icon_pane.set_text_string("");
            // Center tab should be highlighted
            help_pane.set_default_material_colors();
            help_pane.set_color(255, 255, 0, 255);
        }
        help_pane.set_text_string(&t!(tab_titles[idx]));
    });

    // Save Defaults Keyhelp
    let name = "SaveDefaults";
    let key = save_defaults_key;
    let title = "Save Defaults";
    let key_help_pane = root_pane.find_pane_by_name_recursive(name).unwrap();
    let icon_pane = key_help_pane
        .find_pane_by_name_recursive("set_txt_icon")
        .unwrap()
        .as_textbox();
    set_icon_text(icon_pane, &[*key.unwrap()]);
    key_help_pane
        .find_pane_by_name_recursive("set_txt_help")
        .unwrap()
        .as_textbox()
        .set_text_string(title);

    // Reset Keyhelp
    let name = "ResetDefaults";
    let key = reset_key;
    let title = match app.page {
        AppPage::SUBMENU => "Reset All",
        AppPage::SLIDER => "Reset Current",
        AppPage::TOGGLE => "Reset Current",
        AppPage::CONFIRMATION => "",
        AppPage::CLOSE => "",
    };
    if !title.is_empty() {
        let key_help_pane = root_pane.find_pane_by_name_recursive(name).unwrap();
        let icon_pane = key_help_pane
            .find_pane_by_name_recursive("set_txt_icon")
            .unwrap()
            .as_textbox();
        set_icon_text(icon_pane, &[*key.unwrap()]);
        key_help_pane
            .find_pane_by_name_recursive("set_txt_help")
            .unwrap()
            .as_textbox()
            .set_text_string(title);
    }

    // Clear Toggle Keyhelp
    let name = "ClearToggle";
    let key_help_pane = root_pane.find_pane_by_name_recursive(name).unwrap();
    let icon_pane = key_help_pane
        .find_pane_by_name_recursive("set_txt_icon")
        .unwrap();
    if app.should_show_clear_keyhelp() {
        // This is only displayed when you're in a multiple selection toggle menu w/ toggle.max > 1
        let key = clear_toggle_key;
        let title = "Clear Toggle";
        set_icon_text(icon_pane.as_textbox(), &[*key.unwrap()]);
        key_help_pane
            .find_pane_by_name_recursive("set_txt_help")
            .unwrap()
            .as_textbox()
            .set_text_string(title);
        icon_pane.set_visible(true);
        key_help_pane.set_visible(true);
    } else {
        icon_pane.set_visible(false);
        key_help_pane.set_visible(false);
    }

    match app.page {
        AppPage::SUBMENU => render_submenu_page(&mut app, root_pane),
        AppPage::SLIDER => render_slider_page(&mut app, root_pane),
        AppPage::TOGGLE => render_toggle_page(&mut app, root_pane),
        AppPage::CONFIRMATION => render_confirmation_page(&mut app, root_pane),
        AppPage::CLOSE => {}
    }
}
