use crate::{common::menu::QUICK_MENU_ACTIVE};
use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};
use training_mod_tui::AppPage;
use training_mod_tui::gauge::GaugeState;
use crate::training::ui;

pub static NUM_MENU_TEXT_OPTIONS: usize = 27;
pub static NUM_MENU_TEXT_SLIDERS: usize = 2;
pub static NUM_MENU_TABS: usize = 3;

pub static mut HAS_SORTED_MENU_CHILDREN: bool = false;

pub static mut MENU_PANE_PTR: u64 = 0;
const MENU_POS : ResVec3 = ResVec3 {
    x: -360.0,
    y: 440.0,
    z: 0.0
};


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

const BLACK: ResColor = ResColor {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

pub static mut MENU_NAME : &str = "trMod_menu";
pub static mut SLIDER_MENU_NAME : &str = "slider_menu";
pub static mut SLIDER_TITLE_NAME : &str = "slider_title";
pub static mut SLIDER_UI_CONTAINER_NAME : &str = "slider_ui_container";

macro_rules! menu_text_name_fmt {
    ($x:ident, $y:ident) => {
        format!("trMod_menu_opt_{}_{}", $x, $y).as_str()
    };
}

macro_rules! menu_text_check_fmt {
    ($x:ident, $y:ident) => {
        format!("trMod_menu_check_{}_{}", $x, $y).as_str()
    };
}

macro_rules! menu_text_bg_left_fmt {
    ($x:ident, $y:ident) => {
        format!("trMod_menu_bg_left_{}_{}", $x, $y).as_str()
    };
}

macro_rules! menu_text_bg_back_fmt {
    ($x:ident, $y:ident) => {
        format!("trMod_menu_bg_back_{}_{}", $x, $y).as_str()
    };
}

macro_rules! menu_tab_fmt {
    ($x:ident) => {
        format!("trMod_menu_tab_{}", $x).as_str()
    };
}

macro_rules! menu_tab_help_fmt {
    ($x:ident) => {
        format!("trMod_menu_tab_help_{}", $x).as_str()
    };
}

macro_rules! menu_text_slider_fmt {
    ($x:ident) => {
        format!("trMod_menu_slider_{}", $x).as_str()
    };
}

macro_rules! menu_slider_label_fmt {
    ($x:ident) => {
        format!("trMod_menu_slider_{}_lbl", $x).as_str()
    };
}

macro_rules! menu_slider_button_label_fmt {
    ($x:ident) => {
        format!("slider_item_btn_{}_lbl", $x).as_str()
    };
}

macro_rules! menu_slider_button_fg_fmt {
    ($x:ident) => {
        format!("slider_btn_fg_{}_lbl", $x).as_str()
    };
}

// Sort all panes in under menu pane such that text and check options
// are last
pub unsafe fn all_menu_panes_sorted(root_pane: &Pane) -> Vec<&mut Pane> {
    let mut panes = (0..NUM_MENU_TEXT_OPTIONS)
        .flat_map(|idx| {
            let x = idx % 3;
            let y = idx / 3;
            [
                root_pane
                    .find_pane_by_name_recursive(menu_text_name_fmt!(x, y))
                    .unwrap(),
                root_pane
                    .find_pane_by_name_recursive(menu_text_check_fmt!(x, y))
                    .unwrap(),
                root_pane
                    .find_pane_by_name_recursive(menu_text_bg_left_fmt!(x, y))
                    .unwrap(),
                root_pane
                    .find_pane_by_name_recursive(menu_text_bg_back_fmt!(x, y))
                    .unwrap(),
            ]
        })
        .collect::<Vec<&mut Pane>>();

    panes.append(
        &mut (0..NUM_MENU_TEXT_SLIDERS)
            .map(|idx| {
                root_pane
                    .find_pane_by_name_recursive(menu_text_slider_fmt!(idx))
                    .unwrap()
            })
            .collect::<Vec<&mut Pane>>(),
    );

    panes.append(
        &mut (0..NUM_MENU_TEXT_SLIDERS)
            .map(|idx| {
                root_pane
                    .find_pane_by_name_recursive(menu_slider_label_fmt!(idx))
                    .unwrap()
            })
            .collect::<Vec<&mut Pane>>(),
    );

    panes.sort_by(|a, _| {
        if a.get_name().contains("opt") || a.get_name().contains("check") {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });

    panes
}

pub unsafe fn draw(root_pane: &mut Pane) {
    // Update menu display
    // Grabbing lock as read-only, essentially
    let app = &*crate::common::menu::QUICK_MENU_APP.data_ptr();

    if let Some(quit_button) = root_pane.find_pane_by_name_recursive("btn_finish") {
        // Normally at (-804, 640)
        // Comes down to (-804, 514)
        if QUICK_MENU_ACTIVE {
            quit_button.pos_y = 514.0;
        }

        for quit_txt_s in &["set_txt_00", "set_txt_01"] {
            if let Some(quit_txt) = quit_button.find_pane_by_name_recursive(quit_txt_s) {
                quit_txt.as_textbox().set_text_string(if QUICK_MENU_ACTIVE {
                    "Modpack Menu"
                } else {
                    // Awkward. We should get the o.g. translation for non-english games
                    // Or create our own textbox here so we don't step on their toes.
                    "Quit Training"
                });
            }
        }
    }

    let menu_pane = root_pane.find_pane_by_name_recursive(MENU_NAME).unwrap();
    menu_pane.set_visible(QUICK_MENU_ACTIVE);

    if !HAS_SORTED_MENU_CHILDREN {
        let sorted_panes = all_menu_panes_sorted(root_pane);
        // Place in sorted order such that backings are behind, etc.
        sorted_panes.iter().for_each(|p| menu_pane.remove_child(p));
        sorted_panes.iter().for_each(|p| menu_pane.append_child(p));

        HAS_SORTED_MENU_CHILDREN = true;
    }

    // Make all invisible first
    (0..NUM_MENU_TEXT_OPTIONS).for_each(|idx| {
        let x = idx % 3;
        let y = idx / 3;
        root_pane
            .find_pane_by_name_recursive(menu_text_name_fmt!(x, y))
            .map(|text| text.set_visible(false));
        root_pane
            .find_pane_by_name_recursive(menu_text_check_fmt!(x, y))
            .map(|text| text.set_visible(false));
        root_pane
            .find_pane_by_name_recursive(menu_text_bg_left_fmt!(x, y))
            .map(|text| text.set_visible(false));
        root_pane
            .find_pane_by_name_recursive(menu_text_bg_back_fmt!(x, y))
            .map(|text| text.set_visible(false));
    });
    (0..NUM_MENU_TEXT_SLIDERS).for_each(|idx| {
        root_pane
            .find_pane_by_name_recursive(menu_text_slider_fmt!(idx))
            .map(|text| text.set_visible(false));

        root_pane
            .find_pane_by_name_recursive(menu_slider_label_fmt!(idx))
            .map(|text| text.set_visible(false));
    });

    root_pane
            .find_pane_by_name_recursive(SLIDER_MENU_NAME)
            .map(|pane| pane.set_visible(false));

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
    let tab_titles = [prev_tab, tab_selected, next_tab].map(|idx| app_tabs[idx]);

    (0..NUM_MENU_TABS).for_each(|idx| {
        root_pane
            .find_pane_by_name_recursive(format!("trMod_menu_tab_{idx}").as_str())
            .map(|text| text.as_textbox().set_text_string(tab_titles[idx]));
    });

    if app.page == AppPage::SUBMENU {
        let tab_selected = app.tab_selected();
        let tab = app.menu_items.get(tab_selected).unwrap();

        (0..NUM_MENU_TEXT_OPTIONS)
            // Valid options in this submenu
            .filter_map(|idx| tab.idx_to_list_idx_opt(idx))
            .map(|(list_section, list_idx)| {
                (
                    list_section,
                    list_idx,
                    root_pane
                        .find_pane_by_name_recursive(menu_text_name_fmt!(
                            list_section,
                            list_idx
                        ))
                        .unwrap(),
                    root_pane
                        .find_pane_by_name_recursive(menu_text_bg_left_fmt!(
                            list_section,
                            list_idx
                        ))
                        .unwrap(),
                    root_pane
                        .find_pane_by_name_recursive(menu_text_bg_back_fmt!(
                            list_section,
                            list_idx
                        ))
                        .unwrap(),
                )
            })
            .for_each(|(list_section, list_idx, text, bg_left, bg_back)| {
                let list = &tab.lists[list_section];
                let submenu = &list.items[list_idx];
                let is_selected = list.state.selected().filter(|s| *s == list_idx).is_some();
                let text = text.as_textbox();
                text.set_text_string(submenu.submenu_title);
                text.set_visible(true);
                let bg_left_material = &mut *bg_left.as_picture().material;
                if is_selected {
                    if let Some(footer) =
                        root_pane.find_pane_by_name_recursive("trMod_menu_footer_txt")
                    {
                        footer.as_textbox().set_text_string(submenu.help_text);
                    }
                    bg_left_material.set_white_res_color(BG_LEFT_ON_WHITE_COLOR);
                    bg_left_material.set_black_res_color(BG_LEFT_ON_BLACK_COLOR);
                    text.text_shadow_enable(true);
                    text.text_outline_enable(true);
                    text.set_color(255, 255, 255, 255);
                } else {
                    bg_left_material.set_white_res_color(BG_LEFT_OFF_WHITE_COLOR);
                    bg_left_material.set_black_res_color(BG_LEFT_OFF_BLACK_COLOR);
                    text.text_shadow_enable(false);
                    text.text_outline_enable(false);
                    text.set_color(85, 89, 92, 255);
                }

                bg_left.set_visible(true);
                bg_back.set_visible(true);
            });
    } else if matches!(app.selected_sub_menu_slider.state, GaugeState::None) {
        let (_title, _help_text, mut sub_menu_str_lists) = app.sub_menu_strs_and_states();
        (0..sub_menu_str_lists.len()).for_each(|list_section| {
            let sub_menu_str = sub_menu_str_lists[list_section].0.clone();
            let sub_menu_state = &mut sub_menu_str_lists[list_section].1;
            sub_menu_str
                .iter()
                .enumerate()
                .for_each(|(idx, (checked, name))| {
                    let is_selected = sub_menu_state.selected().filter(|s| *s == idx).is_some();
                    if let Some(text) = root_pane
                        .find_pane_by_name_recursive(menu_text_name_fmt!(list_section, idx))
                    {
                        let text = text.as_textbox();
                        text.set_text_string(name);

                        if is_selected {
                            text.text_shadow_enable(true);
                            text.text_outline_enable(true);
                            text.set_color(255, 255, 255, 255);
                        } else {
                            text.text_shadow_enable(false);
                            text.text_outline_enable(false);
                            text.set_color(85, 89, 92, 255);
                        }

                        text.set_visible(true);
                    }

                    if let Some(bg_left) = root_pane
                        .find_pane_by_name_recursive(menu_text_bg_left_fmt!(list_section, idx))
                    {
                        let bg_left_material = &mut *bg_left.as_picture().material;
                        if is_selected {
                            bg_left_material.set_white_res_color(BG_LEFT_ON_WHITE_COLOR);
                            bg_left_material.set_black_res_color(BG_LEFT_ON_BLACK_COLOR);
                        } else {
                            bg_left_material.set_white_res_color(BG_LEFT_OFF_WHITE_COLOR);
                            bg_left_material.set_black_res_color(BG_LEFT_OFF_BLACK_COLOR);
                        }
                        bg_left.set_visible(true);
                    }

                    if let Some(bg_back) = root_pane
                        .find_pane_by_name_recursive(menu_text_bg_back_fmt!(list_section, idx))
                    {
                        bg_back.set_visible(true);
                    }

                    if let Some(check) = root_pane
                        .find_pane_by_name_recursive(menu_text_check_fmt!(list_section, idx))
                    {
                        if *checked {
                            let check = check.as_textbox();

                            check.set_text_string("+");
                            check.set_visible(true);
                        }
                    }
                });
        });
    } else {
        let (title, _help_text, gauge_vals) = app.sub_menu_strs_for_slider();
        let selected_min = gauge_vals.selected_min;
        let selected_max = gauge_vals.selected_max;

        if let Some(pane) = root_pane.find_pane_by_name_recursive(SLIDER_MENU_NAME) {
            pane.set_visible(true);
        }

        if let Some(text) = root_pane.find_pane_by_name_recursive(SLIDER_TITLE_NAME) {
            let text = text.as_textbox();
            text.set_text_string(title);
        }

        (0..NUM_MENU_TEXT_SLIDERS).for_each(|index| {
            if let Some(text_pane) = root_pane.find_pane_by_name_recursive(
                menu_slider_label_fmt!(index),
            ) {
                let text_pane = text_pane.as_textbox();
                text_pane.set_visible(true);

                match index {
                    0 => {
                        text_pane.set_text_string("Min");

                        match gauge_vals.state {
                            GaugeState::MinHover | GaugeState::MinSelected => {
                                text_pane.text_shadow_enable(true);
                                text_pane.text_outline_enable(true);
                                text_pane.set_color(255, 255, 255, 255);
                            }
                            _ => {
                                text_pane.text_shadow_enable(false);
                                text_pane.text_outline_enable(false);
                                text_pane.set_color(85, 89, 92, 255);
                            }
                        }
                    }
                    1 => {
                        text_pane.set_text_string("Max");

                        match gauge_vals.state {
                            GaugeState::MaxHover | GaugeState::MaxSelected => {
                                text_pane.text_shadow_enable(true);
                                text_pane.text_outline_enable(true);
                                text_pane.set_color(255, 255, 255, 255);
                            }
                            _ => {
                                text_pane.text_shadow_enable(false);
                                text_pane.text_outline_enable(false);
                                text_pane.set_color(85, 89, 92, 255);
                            }
                        }
                    }
                    _ => panic!("Unexpected slider label index {}!", index),
                }
            }

            if let Some(text_pane) = root_pane
                .find_pane_by_name_recursive(menu_text_slider_fmt!(index))
            {
                let text_pane = text_pane.as_textbox();
                text_pane.set_visible(true);

                match index {
                    0 => text_pane.set_text_string(&format!("{selected_min}")),
                    1 => text_pane.set_text_string(&format!("{selected_max}")),
                    _ => panic!("Unexpected slider label index {}!", index),
                }
            }

            if let Some(bg_left) = root_pane
                .find_pane_by_name_recursive(menu_slider_button_fg_fmt!(index))
            {
                let bg_left_material = &mut *bg_left.as_picture().material;

                match index {
                    0 => match gauge_vals.state {
                        GaugeState::MinHover => {
                            bg_left_material.set_white_res_color(BG_LEFT_ON_WHITE_COLOR);
                            bg_left_material.set_black_res_color(BG_LEFT_ON_BLACK_COLOR);
                        }
                        GaugeState::MinSelected => {
                            bg_left_material.set_white_res_color(BG_LEFT_SELECTED_WHITE_COLOR);
                            bg_left_material.set_black_res_color(BG_LEFT_SELECTED_BLACK_COLOR);
                        }
                        _ => {
                            bg_left_material.set_white_res_color(BG_LEFT_OFF_WHITE_COLOR);
                            bg_left_material.set_black_res_color(BG_LEFT_OFF_BLACK_COLOR);
                        }
                    },
                    1 => match gauge_vals.state {
                        GaugeState::MaxHover => {
                            bg_left_material.set_white_res_color(BG_LEFT_ON_WHITE_COLOR);
                            bg_left_material.set_black_res_color(BG_LEFT_ON_BLACK_COLOR);
                        }
                        GaugeState::MaxSelected => {
                            bg_left_material.set_white_res_color(BG_LEFT_SELECTED_WHITE_COLOR);
                            bg_left_material.set_black_res_color(BG_LEFT_SELECTED_BLACK_COLOR);
                        }
                        _ => {
                            bg_left_material.set_white_res_color(BG_LEFT_OFF_WHITE_COLOR);
                            bg_left_material.set_black_res_color(BG_LEFT_OFF_BLACK_COLOR);
                        }
                    },
                    _ => panic!("Unexpected slider label index {}!", index),
                }
                bg_left.set_visible(true);
            }
        });
    }
}

pub static BUILD_CONTAINER_PANE: ui::PaneCreationCallback = |_, root_pane, original_build, layout, out_build_result_information, device, _block, parts_build_data_set, build_arg_set, build_res_set, _kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    // Let's create our parent display pane here.
    let menu_pane_kind = u32::from_le_bytes([b'p', b'a', b'n', b'1']);
    let mut menu_pane_block = ResPane::new(MENU_NAME);
    // Overall menu pane @ 0,0 to reason about positions globally
    menu_pane_block.set_pos(ResVec3::default());
    let menu_pane = build!(menu_pane_block, ResPane, menu_pane_kind, Pane);
    menu_pane.detach();

    root_pane.append_child(menu_pane);
    if MENU_PANE_PTR != menu_pane as *mut Pane as u64 {
        MENU_PANE_PTR = menu_pane as *mut Pane as u64;
        HAS_SORTED_MENU_CHILDREN = false;
    }

    ui::reset_creation();
};

pub static BUILD_FOOTER_BG: ui::PaneCreationCallback = |_, root_pane, original_build, layout, out_build_result_information, device, block, parts_build_data_set, build_arg_set, build_res_set, kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    let menu_pane = root_pane.find_pane_by_name(MENU_NAME, true).unwrap();
    let block = block as *mut ResPictureWithTex<1>;
    // For menu backing
    let mut pic_menu_block = *block;
    pic_menu_block.set_name("trMod_menu_footer_bg");
    let pic_menu_pane = build!(pic_menu_block, ResPictureWithTex<1>, kind, Picture);
    pic_menu_pane.detach();

    menu_pane.append_child(pic_menu_pane);
};

pub static BUILD_FOOTER_TXT: ui::PaneCreationCallback = |_, root_pane, original_build, layout, out_build_result_information, device, block, parts_build_data_set, build_arg_set, build_res_set, kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    let menu_pane = root_pane.find_pane_by_name(MENU_NAME, true).unwrap();

    let block = block as *mut ResTextBox;
    let mut text_block = *block;
    text_block.set_name("trMod_menu_footer_txt");

    let text_pane = build!(text_block, ResTextBox, kind, TextBox);
    text_pane.set_text_string("Footer!");
    // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
    text_pane.set_default_material_colors();
    text_pane.set_color(255, 255, 255, 255);
    text_pane.detach();

    menu_pane.append_child(text_pane);
};

pub static BUILD_TAB_TXTS: ui::PaneCreationCallback = |_, root_pane, original_build, layout, out_build_result_information, device, block, parts_build_data_set, build_arg_set, build_res_set, kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    (0..NUM_MENU_TABS).for_each(|txt_idx| {
        let menu_pane = root_pane.find_pane_by_name(MENU_NAME, true).unwrap();

        let block = block as *mut ResTextBox;
        let mut text_block = *block;
        text_block.enable_shadow();
        text_block.text_alignment(TextAlignment::Center);

        let x = txt_idx;
        text_block.set_name(menu_tab_fmt!(x));

        let mut x_offset = x as f32 * 300.0;
        // Center current tab since we don't have a help key
        if x == 1 {
            x_offset -= 25.0;
        }
        text_block.set_pos(ResVec3::new(
            MENU_POS.x - 25.0 + x_offset,
            MENU_POS.y + 75.0,
            0.0,
        ));
        let text_pane = build!(text_block, ResTextBox, kind, TextBox);
        text_pane.set_text_string(format!("Tab {txt_idx}!").as_str());
        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        text_pane.set_default_material_colors();
        text_pane.set_color(255, 255, 255, 255);
        if txt_idx == 1 {
            text_pane.set_color(255, 255, 0, 255);
        }
        text_pane.detach();
        menu_pane.append_child(text_pane);

        let mut help_block = *block;
        // Font Idx 2 = nintendo64 which contains nice symbols
        help_block.font_idx = 2;

        let x = txt_idx;
        help_block.set_name(menu_tab_help_fmt!(x));

        let x_offset = x as f32 * 300.0;
        help_block.set_pos(ResVec3::new(
            MENU_POS.x - 250.0 + x_offset,
            MENU_POS.y + 75.0,
            0.0,
        ));
        let help_pane = build!(help_block, ResTextBox, kind, TextBox);
        help_pane.set_text_string("Help Buttons");
        let it = help_pane.text_buf as *mut u16;
        match txt_idx {
            // Left Tab: ZL
            0 => {
                *it = 0xE0E6;
                *(it.add(1)) = 0x0;
                help_pane.text_len = 1;
            }
            1 => {
                *it = 0x0;
                help_pane.text_len = 0;
            }
            // Right Tab: ZR
            2 => {
                *it = 0xE0E7;
                *(it.add(1)) = 0x0;
                help_pane.text_len = 1;
            }
            _ => {}
        }

        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        help_pane.set_default_material_colors();
        help_pane.set_color(255, 255, 255, 255);
        help_pane.detach();
        menu_pane.append_child(help_pane);
    });
};

pub static BUILD_OPT_TXTS: ui::PaneCreationCallback = |_, root_pane, original_build, layout, out_build_result_information, device, block, parts_build_data_set, build_arg_set, build_res_set, kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    (0..NUM_MENU_TEXT_OPTIONS).for_each(|txt_idx| {
        let x = txt_idx % 3;
        let y = txt_idx / 3;

        let menu_pane = root_pane.find_pane_by_name(MENU_NAME, true).unwrap();

        let block = block as *mut ResTextBox;
        let mut text_block = *block;
        text_block.text_alignment(TextAlignment::Center);

        text_block.set_name(menu_text_name_fmt!(x, y));

        let x_offset = x as f32 * 500.0;
        let y_offset = y as f32 * 85.0;
        text_block.set_pos(ResVec3::new(
            MENU_POS.x - 480.0 + x_offset,
            MENU_POS.y - 50.0 - y_offset,
            0.0,
        ));
        let text_pane = build!(text_block, ResTextBox, kind, TextBox);
        text_pane.set_text_string(format!("Opt {txt_idx}!").as_str());
        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        text_pane.set_default_material_colors();
        text_pane.set_color(85, 89, 92, 255);
        text_pane.set_text_shadow(ResVec2::new(4.0, -3.0), ResVec2::new(1.0, 1.0), [BLACK, BLACK], 0.0);
        text_pane.set_text_alignment(HorizontalPosition::Center, VerticalPosition::Center);
        text_pane.detach();
        menu_pane.append_child(text_pane);

        let mut check_block = *block;
        // Font Idx 2 = nintendo64 which contains nice symbols
        check_block.font_idx = 2;

        check_block.set_name(menu_text_check_fmt!(x, y));
        check_block.set_pos(ResVec3::new(
            MENU_POS.x - 375.0 + x_offset,
            MENU_POS.y - 50.0 - y_offset,
            0.0,
        ));
        let check_pane = build!(check_block, ResTextBox, kind, TextBox);
        check_pane.set_text_string(format!("Check {txt_idx}!").as_str());
        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        check_pane.set_default_material_colors();
        check_pane.set_color(0, 0, 0, 255);
        check_pane.detach();
        menu_pane.append_child(check_pane);
    });
};

pub static BUILD_SLIDER_CONTAINER_PANE: ui::PaneCreationCallback = |_, root_pane, original_build, layout, out_build_result_information, device, block, parts_build_data_set, build_arg_set, build_res_set, kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    let menu_pane = root_pane.find_pane_by_name(MENU_NAME, true).unwrap();
    let slider_ui_root_pane_kind = u32::from_le_bytes([b'p', b'a', b'n', b'1']);
    let mut slider_ui_root_block = ResPane::new(SLIDER_MENU_NAME);

    slider_ui_root_block.set_pos(ResVec3::default());

    let slider_ui_root = build!(
        slider_ui_root_block,
        ResPane,
        slider_ui_root_pane_kind,
        Pane
    );

    slider_ui_root.detach();
    menu_pane.append_child(slider_ui_root);

    let block = block as *mut ResPictureWithTex<1>;

    let mut picture_block = *block;

    picture_block.set_name(SLIDER_UI_CONTAINER_NAME);
    picture_block.set_size(ResVec2::new(675.0, 300.0));
    picture_block.set_pos(ResVec3::new(-530.0, 180.0, 0.0));
    picture_block.tex_coords = [
        [ResVec2::new(0.0, 0.0)],
        [ResVec2::new(1.0, 0.0)],
        [ResVec2::new(0.0, 1.5)],
        [ResVec2::new(1.0, 1.5)],
    ];

    let picture_pane = build!(picture_block, ResPictureWithTex<1>, kind, Picture);
    picture_pane.detach();
    slider_ui_root.append_child(picture_pane);
};

pub static BUILD_SLIDER_HEADER_TXT: ui::PaneCreationCallback = |_, root_pane, original_build, layout, out_build_result_information, device, block, parts_build_data_set, build_arg_set, build_res_set, kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    let container_pane = root_pane.find_pane_by_name(SLIDER_MENU_NAME, true).unwrap();

    let block = block as *mut ResTextBox;
    let mut title_block = *block;

    title_block.set_name(SLIDER_TITLE_NAME);
    title_block.set_pos(ResVec3::new(-530.0, 285.0, 0.0));
    title_block.set_size(ResVec2::new(550.0, 100.0));
    title_block.font_size = ResVec2::new(50.0, 100.0);

    let title_pane = build!(title_block, ResTextBox, kind, TextBox);

    title_pane.set_text_string("Slider title!");

    // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
    title_pane.set_default_material_colors();

    // Header should be white text
    title_pane.set_color(255, 255, 255, 255);
    title_pane.detach();
    container_pane.append_child(title_pane);
};

pub static BUILD_SLIDER_TXTS: ui::PaneCreationCallback = |_, root_pane, original_build, layout, out_build_result_information, device, block, parts_build_data_set, build_arg_set, build_res_set, kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    (0..NUM_MENU_TEXT_SLIDERS).for_each(|idx| {
        let x = idx % 2;

        let label_x_offset = x as f32 * 345.0;

        let slider_root_pane = root_pane.find_pane_by_name(SLIDER_MENU_NAME, true).unwrap();
        let slider_container = root_pane
            .find_pane_by_name(SLIDER_UI_CONTAINER_NAME, true)
            .unwrap();

        let block = block as *mut ResTextBox;

        let mut text_block = *block;

        text_block.text_alignment(TextAlignment::Center);

        text_block.set_name(menu_text_slider_fmt!(idx));

        let value_x_offset = x as f32 * 345.0;

        text_block.set_pos(ResVec3::new(
            slider_root_pane.pos_x - 675.0 + value_x_offset,
            slider_root_pane.pos_y + (slider_container.size_y * 0.458),
            0.0,
        ));

        let text_pane = build!(text_block, ResTextBox, kind, TextBox);
        text_pane.set_text_string(format!("Slider opt {idx}!").as_str());
        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        text_pane.set_default_material_colors();
        text_pane.set_color(0, 0, 0, 255);
        text_pane.detach();
        slider_root_pane.append_child(text_pane);

        let mut label_block = *block;

        label_block.text_alignment(TextAlignment::Center);
        label_block.set_name(menu_slider_label_fmt!(idx));
        label_block.set_pos(ResVec3::new(
            slider_root_pane.pos_x - 750.0 + label_x_offset,
            slider_root_pane.pos_y + slider_container.size_y * 0.458 + 5.0,
            0.0,
        ));
        label_block.font_size = ResVec2::new(25.0, 50.0);

        let label_pane = build!(label_block, ResTextBox, kind, TextBox);

        label_pane.set_text_string(format!("Slider opt {idx}!").as_str());
        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        label_pane.set_default_material_colors();
        label_pane.set_color(85, 89, 92, 255);
        label_pane.text_outline_enable(true);
        label_pane.set_text_shadow(ResVec2::new(4.0, -3.0), ResVec2::new(1.0, 1.0), [BLACK, BLACK], 0.0);
        label_pane.set_text_alignment(HorizontalPosition::Left, VerticalPosition::Top);
        label_pane.detach();

        slider_root_pane.append_child(label_pane);
        
    });
};

pub static BUILD_BG_LEFTS: ui::PaneCreationCallback = |_, _, original_build, layout, out_build_result_information, device, block, parts_build_data_set, build_arg_set, build_res_set, kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    (0..NUM_MENU_TEXT_OPTIONS).for_each(|txt_idx| {
        let x = txt_idx % 3;
        let y = txt_idx / 3;

        let x_offset = x as f32 * 500.0;
        let y_offset = y as f32 * 85.0;

        let block = block as *mut ResPictureWithTex<2>;
        let mut pic_menu_block = *block;
        pic_menu_block.set_name(menu_text_bg_left_fmt!(x, y));
        pic_menu_block.picture.scale_x /= 1.5;
        pic_menu_block.picture.set_pos(ResVec3::new(
            MENU_POS.x - 400.0 - 195.0 + x_offset,
            MENU_POS.y - 50.0 - y_offset,
            0.0,
        ));
        let pic_menu_pane = build!(pic_menu_block, ResPictureWithTex<2>, kind, Picture);
        pic_menu_pane.detach();
        if MENU_PANE_PTR != 0 {
            (*(MENU_PANE_PTR as *mut Pane)).append_child(pic_menu_pane);
        }
    });

    (0..NUM_MENU_TEXT_SLIDERS).for_each(|index| {
        let x = index % 2;

        if MENU_PANE_PTR != 0 {
            let slider_root = (*(MENU_PANE_PTR as *mut Pane))
                .find_pane_by_name(SLIDER_MENU_NAME, true)
                .unwrap();
            let slider_bg = (*(MENU_PANE_PTR as *mut Pane))
                .find_pane_by_name(SLIDER_UI_CONTAINER_NAME, true)
                .unwrap();
            let x_offset = x as f32 * 345.0;

            let block = block as *mut ResPictureWithTex<2>;
            let mut pic_menu_block = *block;

            pic_menu_block.set_name(menu_slider_button_fg_fmt!(index));

            pic_menu_block.picture.scale_x /= 1.85;
            pic_menu_block.picture.scale_y /= 1.25;

            pic_menu_block.set_pos(ResVec3::new(
                slider_root.pos_x - 842.5 + x_offset,
                slider_root.pos_y + slider_bg.size_y * 0.458,
                0.0,
            ));

            let pic_menu_pane = build!(pic_menu_block, ResPictureWithTex<2>, kind, Picture);
            pic_menu_pane.detach();

            slider_root.append_child(pic_menu_pane);
        }
    });
};

pub static BUILD_BG_BACKS: ui::PaneCreationCallback = |_, _, original_build, layout, out_build_result_information, device, block, parts_build_data_set, build_arg_set, build_res_set, kind| unsafe {
    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original_build(layout, out_build_result_information, device, &mut $block as *mut $resTyp as *mut ResPane, parts_build_data_set, build_arg_set, build_res_set, $kind,) as *mut $typ)
            }
        };
    }

    (0..NUM_MENU_TEXT_OPTIONS).for_each(|txt_idx| {
        let x = txt_idx % 3;
        let y = txt_idx / 3;

        let x_offset = x as f32 * 500.0;
        let y_offset = y as f32 * 85.0;

        let block = block as *mut ResWindowWithTexCoordsAndFrames<1, 4>;

        let mut bg_block = *block;
        bg_block.set_name(menu_text_bg_back_fmt!(x, y));
        bg_block.scale_x /= 2.0;
        bg_block.set_pos(ResVec3::new(
            MENU_POS.x - 400.0 + x_offset,
            MENU_POS.y - 50.0 - y_offset,
            0.0,
        ));
        let bg_pane = build!(bg_block, ResWindowWithTexCoordsAndFrames<1,4>, kind, Window);
        bg_pane.detach();
        if MENU_PANE_PTR != 0 {
            (*(MENU_PANE_PTR as *mut Pane)).append_child(bg_pane);
        }
    });

    (0..NUM_MENU_TEXT_SLIDERS).for_each(|index| {
        let x = index % 2;

        if MENU_PANE_PTR != 0 {
            let slider_root = (*(MENU_PANE_PTR as *mut Pane))
                .find_pane_by_name(SLIDER_MENU_NAME, true)
                .unwrap();
            let slider_bg = (*(MENU_PANE_PTR as *mut Pane))
                .find_pane_by_name(SLIDER_UI_CONTAINER_NAME, true)
                .unwrap();

            let size_y = 90.0;

            let x_offset = x as f32 * 345.0;

            let block = block as *mut ResWindowWithTexCoordsAndFrames<1, 4>;
            let mut bg_block = *block;

            bg_block.set_name(menu_slider_button_label_fmt!(index));
            bg_block.scale_x /= 2.0;

            bg_block.set_size(ResVec2::new(605.0, size_y));

            bg_block.set_pos(ResVec3::new(
                slider_root.pos_x - 700.0 + x_offset,
                slider_root.pos_y + slider_bg.size_y * 0.458,
                0.0,
            ));

            let bg_pane = build!(bg_block, ResWindowWithTexCoordsAndFrames<1,4>, kind, Window);
            bg_pane.detach();

            slider_root.append_child(bg_pane);
        }
    });
};