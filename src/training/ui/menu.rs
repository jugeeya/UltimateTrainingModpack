use std::collections::HashMap;

use lazy_static::lazy_static;
use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};
use training_mod_tui::gauge::GaugeState;
use training_mod_tui::{App, AppPage, NUM_LISTS};

use crate::{common, common::menu::QUICK_MENU_ACTIVE, input::*};

pub static NUM_MENU_TEXT_OPTIONS: usize = 32;
pub static _NUM_MENU_TABS: usize = 3;

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

lazy_static! {
    static ref GCC_BUTTON_MAPPING: HashMap<&'static str, u16> = HashMap::from([
        ("L", 0xE204),
        ("R", 0xE205),
        ("X", 0xE206),
        ("Y", 0xE207),
        ("Z", 0xE208)
    ]);
    static ref PROCON_BUTTON_MAPPING: HashMap<&'static str, u16> = HashMap::from([
        ("L", 0xE0E4),
        ("R", 0xE0E5),
        ("X", 0xE0E2),
        ("Y", 0xE0E3),
        ("ZL", 0xE0E6),
        ("ZR", 0xE0E7)
    ]);
}

unsafe fn render_submenu_page(app: &App, root_pane: &Pane) {
    let tab_selected = app.tab_selected();
    let tab = app.menu_items.get(tab_selected).unwrap();
    let submenu_ids = app.submenu_ids();

    (0..NUM_MENU_TEXT_OPTIONS)
        // Valid options in this submenu
        .filter_map(|idx| tab.idx_to_list_idx_opt(idx))
        .for_each(|(list_section, list_idx)| {
            let menu_button_row = root_pane
                .find_pane_by_name_recursive(format!("TrModMenuButtonRow{list_idx}").as_str())
                .unwrap();
            menu_button_row.set_visible(true);

            let menu_button = menu_button_row
                .find_pane_by_name_recursive(format!("Button{list_section}").as_str())
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

            let list = &tab.lists[list_section];
            let submenu = &list.items[list_idx];
            let is_selected = list.state.selected().filter(|s| *s == list_idx).is_some();

            title_text.set_text_string(submenu.submenu_title.as_str());

            // In the actual 'layout.arc' file, every icon image is stacked
            // into a single container pane, with each image directly on top of another.
            // Hide all icon images, and strategically mark the icon that
            // corresponds with a particular button to be visible.
            submenu_ids.iter().for_each(|id| {
                let pane = menu_button.find_pane_by_name_recursive(id);
                if let Some(p) = pane {
                    p.set_visible(id == &submenu.submenu_id);
                }
            });

            menu_button
                .find_pane_by_name_recursive("check")
                .unwrap()
                .set_visible(false);

            if is_selected {
                root_pane
                    .find_pane_by_name_recursive("FooterTxt")
                    .unwrap()
                    .as_textbox()
                    .set_text_string(submenu.help_text.as_str());

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
        });
}

unsafe fn render_toggle_page(app: &App, root_pane: &Pane) {
    let (_title, _help_text, mut sub_menu_str_lists) = app.sub_menu_strs_and_states();
    (0..sub_menu_str_lists.len()).for_each(|list_section| {
        let sub_menu_str = sub_menu_str_lists[list_section].0.clone();
        let sub_menu_state = &mut sub_menu_str_lists[list_section].1;
        sub_menu_str
            .iter()
            .enumerate()
            .for_each(|(list_idx, (checked, name))| {
                let menu_button_row = root_pane
                    .find_pane_by_name_recursive(format!("TrModMenuButtonRow{list_idx}").as_str())
                    .unwrap();
                menu_button_row.set_visible(true);

                let menu_button = menu_button_row
                    .find_pane_by_name_recursive(format!("Button{list_section}").as_str())
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

                let is_selected = sub_menu_state
                    .selected()
                    .filter(|s| *s == list_idx)
                    .is_some();

                let submenu_ids = app.submenu_ids();

                submenu_ids.iter().for_each(|id| {
                    let pane = menu_button.find_pane_by_name_recursive(id);
                    if let Some(p) = pane {
                        p.set_visible(false);
                    }
                });

                title_text.set_text_string(name);
                menu_button
                    .find_pane_by_name_recursive("check")
                    .unwrap()
                    .set_visible(true);

                menu_button
                    .find_pane_by_name_recursive("Icon")
                    .unwrap()
                    .set_visible(*checked);

                let title_bg_material = &mut *title_bg.material;

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
            });
    });
}

unsafe fn render_slider_page(app: &App, root_pane: &Pane) {
    let (title, _help_text, gauge_vals) = app.sub_menu_strs_for_slider();
    let selected_min = gauge_vals.selected_min;
    let selected_max = gauge_vals.selected_max;

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
    header.set_text_string(title.as_str());
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
    match gauge_vals.state {
        GaugeState::MinHover | GaugeState::MinSelected => {
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
    match gauge_vals.state {
        GaugeState::MaxHover | GaugeState::MaxSelected => {
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
    let min_colors = match gauge_vals.state {
        GaugeState::MinHover => (BG_LEFT_ON_WHITE_COLOR, BG_LEFT_ON_BLACK_COLOR),
        GaugeState::MinSelected => (BG_LEFT_SELECTED_WHITE_COLOR, BG_LEFT_SELECTED_BLACK_COLOR),
        _ => (BG_LEFT_OFF_WHITE_COLOR, BG_LEFT_OFF_BLACK_COLOR),
    };

    min_title_bg_material.set_white_res_color(min_colors.0);
    min_title_bg_material.set_black_res_color(min_colors.1);

    let max_title_bg_material = &mut *max_title_bg.as_picture().material;
    let max_colors = match gauge_vals.state {
        GaugeState::MaxHover => (BG_LEFT_ON_WHITE_COLOR, BG_LEFT_ON_BLACK_COLOR),
        GaugeState::MaxSelected => (BG_LEFT_SELECTED_WHITE_COLOR, BG_LEFT_SELECTED_BLACK_COLOR),
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

pub unsafe fn draw(root_pane: &Pane) {
    // Update menu display
    // Grabbing lock as read-only, essentially
    let app = &*crate::common::menu::QUICK_MENU_APP.data_ptr();
    if let Some(quit_button) = root_pane.find_pane_by_name_recursive("TrModTitle") {
        for quit_txt_s in &["set_txt_00", "set_txt_01"] {
            if let Some(quit_txt) = quit_button.find_pane_by_name_recursive(quit_txt_s) {
                quit_txt.as_textbox().set_text_string("Modpack Menu");
            }
        }
    }

    root_pane
        .find_pane_by_name_recursive("TrModMenu")
        .unwrap()
        .set_visible(QUICK_MENU_ACTIVE);
    if QUICK_MENU_ACTIVE {
        common::menu::FRAME_COUNTER += 1;
    }

    // Make all invisible first
    (0..NUM_MENU_TEXT_OPTIONS).for_each(|idx| {
        let col_idx = idx % NUM_LISTS;
        let row_idx = idx / NUM_LISTS;

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
    });

    // Make normal training panes invisible if we're active
    // InfluencedAlpha means "Should my children panes' alpha be influenced by mine, as the parent?"
    let status_r_pane = root_pane
        .find_pane_by_name_recursive("status_R")
        .expect("Unable to find status_R pane");
    // status_r_pane.flags |= 1 << PaneFlag::InfluencedAlpha as u8;
    status_r_pane.set_visible(!QUICK_MENU_ACTIVE);

    root_pane
        .find_pane_by_name_recursive("TrModSlider")
        .unwrap()
        .set_visible(false);

    let app_tabs = &app.tabs.items;
    let tab_selected = app.tabs.state.selected().unwrap();
    let prev_tab = if tab_selected == 0 {
        app_tabs.len() - 1
    } else {
        tab_selected - 1
    };
    let next_tab = if tab_selected == app_tabs.len() - 1 {
        0
    } else {
        tab_selected + 1
    };
    let tab_titles = [prev_tab, tab_selected, next_tab].map(|idx| app_tabs[idx].clone());

    let is_gcc = (*common::menu::P1_CONTROLLER_STYLE.data_ptr()) == ControllerStyle::GCController;
    let button_mapping = if is_gcc {
        GCC_BUTTON_MAPPING.clone()
    } else {
        PROCON_BUTTON_MAPPING.clone()
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

    let (left_tab_key, right_tab_key, save_defaults_key, reset_current_key, reset_all_key) =
        if is_gcc {
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
            let it = icon_pane.text_buf as *mut u16;
            icon_pane.text_len = 1;
            *it = **key;
            *(it.add(1)) = 0x0;
        }

        if *name == "CurrentTab" {
            icon_pane.set_text_string("");
            // Center tab should be highlighted
            help_pane.set_default_material_colors();
            help_pane.set_color(255, 255, 0, 255);
        }
        help_pane.set_text_string(tab_titles[idx].as_str());
    });
    [
        (save_defaults_key, "SaveDefaults", "Save Defaults"),
        (reset_current_key, "ResetCurrentDefaults", "Reset Current"),
        (reset_all_key, "ResetAllDefaults", "Reset All"),
    ]
    .iter()
    .for_each(|(key, name, title)| {
        let key_help_pane = root_pane.find_pane_by_name_recursive(name).unwrap();

        let icon_pane = key_help_pane
            .find_pane_by_name_recursive("set_txt_icon")
            .unwrap()
            .as_textbox();
        icon_pane.set_text_string("");
        let it = icon_pane.text_buf as *mut u16;
        icon_pane.text_len = 1;
        *it = *key.unwrap();
        *(it.add(1)) = 0x0;

        key_help_pane
            .find_pane_by_name_recursive("set_txt_help")
            .unwrap()
            .as_textbox()
            .set_text_string(title);
    });

    match app.page {
        AppPage::SUBMENU => render_submenu_page(app, root_pane),
        AppPage::SLIDER => render_slider_page(app, root_pane),
        AppPage::TOGGLE => render_toggle_page(app, root_pane),
        AppPage::CONFIRMATION => todo!(),
    }
}
