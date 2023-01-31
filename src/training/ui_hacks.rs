use crate::common::{get_player_dmg_digits, is_ready_go, is_training_mode};
use crate::consts::FighterId;
use crate::{common::menu::QUICK_MENU_ACTIVE, training::combo::FRAME_ADVANTAGE};
use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};
use training_mod_consts::{OnOff, MENU};
use training_mod_tui::gauge::GaugeState;

pub unsafe fn iterate_anim_list(
    anim_transform_node: &mut AnimTransformNode,
    layout_name: Option<&str>,
) {
    let mut curr = anim_transform_node as *mut AnimTransformNode;
    let mut _anim_idx = 0;
    while !curr.is_null() {
        // Only if valid
        if curr != (*curr).next {
            let anim_transform = (curr as *mut u64).add(2) as *mut AnimTransform;

            parse_anim_transform(anim_transform.as_mut().unwrap(), layout_name);
        }

        curr = (*curr).next;
        _anim_idx += 1;
        if curr == anim_transform_node as *mut AnimTransformNode || curr == (*curr).next {
            break;
        }
    }
}

pub unsafe fn parse_anim_transform(anim_transform: &mut AnimTransform, layout_name: Option<&str>) {
    let res_animation_block_data_start = anim_transform.res_animation_block as u64;
    let res_animation_block = &*anim_transform.res_animation_block;
    let mut anim_cont_offsets = (res_animation_block_data_start
        + res_animation_block.anim_cont_offsets_offset as u64)
        as *const u32;
    for _anim_cont_idx in 0..res_animation_block.anim_cont_count {
        let anim_cont_offset = *anim_cont_offsets;
        let res_animation_cont = (res_animation_block_data_start + anim_cont_offset as u64)
            as *const ResAnimationContent;

        let name = skyline::try_from_c_str((*res_animation_cont).name.as_ptr())
            .unwrap_or("UNKNOWN".to_string());
        let anim_type = (*res_animation_cont).anim_content_type;

        // AnimContentType 1 == MATERIAL
        if name.starts_with("set_dmg_num") && anim_type == 1 {
            if let Some(layout_name) = layout_name {
                let (hundreds, tens, ones, dec) = get_player_dmg_digits(match layout_name {
                    "p1" => FighterId::Player,
                    "p2" => FighterId::CPU,
                    _ => panic!("Unknown layout name: {}", layout_name),
                });

                if name == "set_dmg_num_3" {
                    anim_transform.frame = hundreds as f32;
                }
                if name == "set_dmg_num_2" {
                    anim_transform.frame = tens as f32;
                }
                if name == "set_dmg_num_1" {
                    anim_transform.frame = ones as f32;
                }
                if name == "set_dmg_num_dec" {
                    anim_transform.frame = dec as f32;
                }
            }
        }

        anim_cont_offsets = anim_cont_offsets.add(1);
    }
}

pub static NUM_DISPLAY_PANES: usize = 1;
pub static NUM_MENU_TEXT_OPTIONS: usize = 27;
pub static NUM_MENU_TEXT_SLIDERS: usize = 2;
pub static NUM_MENU_TABS: usize = 3;

pub static mut HAS_SORTED_MENU_CHILDREN: bool = false;

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
    r: 80,
    g: 0,
    b: 0,
    a: 0,
};

const BG_LEFT_SELECTED_WHITE_COLOR: ResColor = ResColor {
    r: 118,
    g: 0,
    b: 0,
    a: 255,
};

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
                    .find_pane_by_name_recursive(
                        menu_slider_label_fmt!(idx)
                    )
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

#[skyline::hook(offset = 0x4b620)]
pub unsafe fn handle_draw(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    let layout_name = skyline::from_c_str((*layout).layout_name);
    let root_pane = &mut *(*layout).root_pane;

    if is_training_mode() && is_ready_go() && layout_name != "info_training" {
        root_pane.flags |= 1 << PaneFlag::InfluencedAlpha as u8;
        root_pane.set_visible(MENU.hud == OnOff::On);
    }

    // Update percentage display as soon as possible on death
    if is_training_mode() && is_ready_go() && layout_name == "info_melee" {
        for player_name in &["p1", "p2"] {
            if let Some(parent) = root_pane.find_pane_by_name_recursive(player_name) {
                let _p1_layout_name = skyline::from_c_str((*parent.as_parts().layout).layout_name);
                let anim_list = &mut (*parent.as_parts().layout).anim_trans_list;

                let mut has_altered_anim_list = false;
                let (hundreds, tens, _, _) = get_player_dmg_digits(match *player_name {
                    "p1" => FighterId::Player,
                    "p2" => FighterId::CPU,
                    _ => panic!("Unknown player name: {}", player_name),
                });

                for dmg_num_s in &[
                    "set_dmg_num_3",
                    "dig_3",
                    "dig_3_anim",
                    "set_dmg_num_2",
                    "dig_2",
                    "dig_2_anim",
                    "set_dmg_num_1",
                    "dig_1",
                    "dig_1_anim",
                    "set_dmg_num_p",
                    "dig_dec",
                    "dig_dec_anim_00",
                    "set_dmg_num_dec",
                    "dig_dec_anim_01",
                    "dig_0_anim",
                    "set_dmg_p",
                ] {
                    if let Some(dmg_num) = parent.find_pane_by_name_recursive(dmg_num_s) {
                        if (dmg_num_s.contains('3') && hundreds == 0)
                            || (dmg_num_s.contains('2') && hundreds == 0 && tens == 0)
                        {
                            continue;
                        }

                        if *dmg_num_s == "set_dmg_p" {
                            dmg_num.pos_y = 0.0;
                        } else if *dmg_num_s == "set_dmg_num_p" {
                            dmg_num.pos_y = -4.0;
                        } else if *dmg_num_s == "dig_dec" {
                            dmg_num.pos_y = -16.0;
                        } else {
                            dmg_num.pos_y = 0.0;
                        }

                        if dmg_num.alpha != 255 || dmg_num.global_alpha != 255 {
                            dmg_num.set_visible(true);
                            if !has_altered_anim_list {
                                iterate_anim_list(anim_list, Some(player_name));
                                has_altered_anim_list = true;
                            }
                        }
                    }
                }

                for death_explosion_s in &[
                    "set_fxui_dead1",
                    "set_fxui_dead2",
                    "set_fxui_dead3",
                    "set_fxui_fire",
                ] {
                    if let Some(death_explosion) =
                        parent.find_pane_by_name_recursive(death_explosion_s)
                    {
                        death_explosion.set_visible(false);
                    }
                }
            }
        }
    }

    // Update training mod displays
    if layout_name == "info_training" {
        // Update frame advantage
        if let Some(parent) = root_pane.find_pane_by_name_recursive("trMod_disp_0") {
            parent.set_visible(crate::common::MENU.frame_advantage == OnOff::On);
        }

        if let Some(header) = root_pane.find_pane_by_name_recursive("trMod_disp_0_header") {
            header.as_textbox().set_text_string("Frame Advantage");
        }

        if let Some(text) = root_pane.find_pane_by_name_recursive("trMod_disp_0_txt") {
            let text = text.as_textbox();
            text.set_text_string(format!("{FRAME_ADVANTAGE}").as_str());
            match FRAME_ADVANTAGE {
                x if x < 0 => text.set_color(200, 8, 8, 255),
                x if x == 0 => text.set_color(0, 0, 0, 255),
                _ => text.set_color(31, 198, 0, 255),
            };
        }

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

        let menu_pane = root_pane.find_pane_by_name_recursive("trMod_menu").unwrap();
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
                .find_pane_by_name_recursive(format!("trMod_menu_slider_{}_lbl", idx).as_str())
                .map(|text| text.set_visible(false));
        });

        root_pane
            .find_pane_by_name_recursive("slider_menu")
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

        if app.outer_list {
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
                    } else {
                        bg_left_material.set_white_res_color(BG_LEFT_OFF_WHITE_COLOR);
                        bg_left_material.set_black_res_color(BG_LEFT_OFF_BLACK_COLOR);
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
            let (_title, _help_text, gauge_vals) = app.sub_menu_strs_for_slider();
            let selected_min = gauge_vals.selected_min;
            let selected_max = gauge_vals.selected_max;

            if let Some(pane) = root_pane.find_pane_by_name_recursive("slider_menu") {
                pane.set_visible(true);
            }

            if let Some(text) = root_pane.find_pane_by_name_recursive("slider_title") {
                let text = text.as_textbox();
                text.set_text_string(&format!("{_title}"));
            }

            (0..NUM_MENU_TEXT_SLIDERS).for_each(|index| {
                if let Some(text_pane) = root_pane.find_pane_by_name_recursive(
                    format!("trMod_menu_slider_{}_lbl", index).as_str(),
                ) {
                    let text_pane = text_pane.as_textbox();
                    text_pane.set_visible(true);

                    match index {
                        0 => text_pane.set_text_string("Min"),
                        1 => text_pane.set_text_string("Max"),
                        _ => text_pane.set_text_string(""),
                    }
                }

                if let Some(text_pane) = root_pane
                    .find_pane_by_name_recursive(format!("trMod_menu_slider_{}", index).as_str())
                {
                    let text_pane = text_pane.as_textbox();
                    text_pane.set_visible(true);

                    match index {
                        0 => text_pane.set_text_string(&format!("{selected_min}")),
                        1 => text_pane.set_text_string(&format!("{selected_max}")),
                        _ => text_pane.set_text_string(""),
                    }
                }

                if let Some(bg_left) = root_pane
                    .find_pane_by_name_recursive(format!("slider_btn_fg_{}", index).as_str())
                {
                    let bg_left_material = &mut *bg_left.as_picture().material;

                    match index {
                        0 => {
                            match gauge_vals.state {
                                GaugeState::MinHover => {
                                    bg_left_material.set_white_res_color(BG_LEFT_ON_WHITE_COLOR);
                                    bg_left_material.set_black_res_color(BG_LEFT_ON_BLACK_COLOR);
                                },
                                GaugeState::MinSelected => {
                                    bg_left_material.set_white_res_color(BG_LEFT_SELECTED_WHITE_COLOR);
                                    bg_left_material.set_black_res_color(BG_LEFT_SELECTED_BLACK_COLOR);
                                },
                                _ => {
                                    bg_left_material.set_white_res_color(BG_LEFT_OFF_WHITE_COLOR);
                                    bg_left_material.set_black_res_color(BG_LEFT_OFF_BLACK_COLOR);
                                }
                            }
                        },
                        1 => {
                            match gauge_vals.state {
                                GaugeState::MaxHover => {
                                    bg_left_material.set_white_res_color(BG_LEFT_ON_WHITE_COLOR);
                                    bg_left_material.set_black_res_color(BG_LEFT_ON_BLACK_COLOR);
                                },
                                GaugeState::MaxSelected => {
                                    bg_left_material.set_white_res_color(BG_LEFT_SELECTED_WHITE_COLOR);
                                    bg_left_material.set_black_res_color(BG_LEFT_SELECTED_BLACK_COLOR);
                                },
                                _ => {
                                    bg_left_material.set_white_res_color(BG_LEFT_OFF_WHITE_COLOR);
                                    bg_left_material.set_black_res_color(BG_LEFT_OFF_BLACK_COLOR);
                                }
                            }
                        },
                        _ => {
                            bg_left_material.set_white_res_color(BG_LEFT_OFF_WHITE_COLOR);
                            bg_left_material.set_black_res_color(BG_LEFT_OFF_BLACK_COLOR);
                        }
                    }
                    bg_left.set_visible(true);
                }
            });
        }
    }

    original!()(layout, draw_info, cmd_buffer);
}

pub static mut MENU_PANE_PTR: u64 = 0;
pub static mut HAS_CREATED_OPT_BG: bool = false;
pub static mut HAS_CREATED_OPT_BG_BACK: bool = false;
pub static mut HAS_CREATED_SLIDER_BG: bool = false;
pub static mut HAS_CREATED_SLIDER_BG_BACK: bool = false;

#[skyline::hook(offset = 0x493a0)]
pub unsafe fn layout_build_parts_impl(
    layout: *mut Layout,
    out_build_result_information: *mut u8,
    device: *const u8,
    data: *mut u8,
    parts_build_data_set: *const u8,
    build_arg_set: *const u8,
    build_res_set: *const u8,
    kind: u32,
) -> *mut Pane {
    let layout_name = skyline::from_c_str((*layout).layout_name);
    let _kind_str: String = kind.to_le_bytes().map(|b| b as char).iter().collect();

    macro_rules! build {
        ($block: ident, $resTyp: ty, $kind:ident, $typ: ty) => {
            paste::paste! {
                &mut *(original!()(
                    layout,
                    out_build_result_information,
                    device,
                    &mut $block as *mut $resTyp as *mut u8,
                    parts_build_data_set,
                    build_arg_set,
                    build_res_set,
                    $kind,
                ) as *mut $typ)
            }
        };
    }

    let root_pane = &mut *(*layout).root_pane;
    let block = data as *mut ResPane;
    let menu_pos = ResVec3::new(-360.0, 440.0, 0.0);

    if layout_name == "info_training_btn0_00_item" {
        if !HAS_CREATED_OPT_BG && (*block).name_matches("icn_bg_main") {
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
                    menu_pos.x - 400.0 - 195.0 + x_offset,
                    menu_pos.y - 50.0 - y_offset,
                    0.0,
                ));
                let pic_menu_pane = build!(pic_menu_block, ResPictureWithTex<2>, kind, Picture);
                pic_menu_pane.detach();
                if MENU_PANE_PTR != 0 {
                    (*(MENU_PANE_PTR as *mut Pane)).append_child(pic_menu_pane);
                    HAS_CREATED_OPT_BG = true;
                }
            });
        }

        if !HAS_CREATED_OPT_BG_BACK && (*block).name_matches("btn_bg") {
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
                    menu_pos.x - 400.0 + x_offset,
                    menu_pos.y - 50.0 - y_offset,
                    0.0,
                ));
                let bg_pane = build!(bg_block, ResWindowWithTexCoordsAndFrames<1,4>, kind, Window);
                bg_pane.detach();
                if MENU_PANE_PTR != 0 {
                    (*(MENU_PANE_PTR as *mut Pane)).append_child(bg_pane);
                    HAS_CREATED_OPT_BG_BACK = true;
                }
            });
        }

        if !HAS_CREATED_SLIDER_BG && (*block).name_matches("icn_bg_main") {
            (0..NUM_MENU_TEXT_SLIDERS).for_each(|index| {
                let x = index % 2;
            
                if MENU_PANE_PTR != 0 {
                    let slider_root = (*(MENU_PANE_PTR as *mut Pane)).find_pane_by_name("slider_menu", true).unwrap();
                    let slider_bg = (*(MENU_PANE_PTR as *mut Pane)).find_pane_by_name("slider_ui_container", true).unwrap();
                    let x_offset = x as f32 * 345.0;
                    

                    let block = block as *mut ResPictureWithTex<2>;
                    let mut pic_menu_block = *block;

                    pic_menu_block.set_name(format!("slider_btn_fg_{}", index).as_str());

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
                    HAS_CREATED_SLIDER_BG = true;
                }
            });
        }
        
        if !HAS_CREATED_SLIDER_BG_BACK && (*block).name_matches("btn_bg") {
            (0..NUM_MENU_TEXT_SLIDERS).for_each(|index| {
                let x = index % 2;

                if MENU_PANE_PTR != 0 {
                    let slider_root = (*(MENU_PANE_PTR as *mut Pane)).find_pane_by_name("slider_menu", true).unwrap();
                    let slider_bg = (*(MENU_PANE_PTR as *mut Pane)).find_pane_by_name("slider_ui_container", true).unwrap();

                    let size_y = 90.0;

                    let x_offset = x as f32 * 345.0;

                    let block = block as *mut ResWindowWithTexCoordsAndFrames<1, 4>;
                    let mut bg_block = *block;

                    bg_block.set_name(format!("slider_item_btn_{}", index).as_str());
                    bg_block.scale_x /= 2.0;

                    bg_block.set_size(ResVec2::new(
                        605.0,
                        size_y
                    ));

                    bg_block.set_pos(ResVec3::new(
                        slider_root.pos_x - 700.0 + x_offset,
                        slider_root.pos_y + slider_bg.size_y * 0.458,
                        0.0,
                    ));

                    let bg_pane = build!(bg_block, ResWindowWithTexCoordsAndFrames<1,4>, kind, Window);
                    bg_pane.detach();

                    slider_root.append_child(bg_pane);
                    HAS_CREATED_SLIDER_BG_BACK = true;
                }
            });
        }
    }

    if layout_name != "info_training" {
        return original!()(
            layout,
            out_build_result_information,
            device,
            data,
            parts_build_data_set,
            build_arg_set,
            build_res_set,
            kind,
        );
    }

    // Menu creation
    if (*block).name_matches("pic_numbase_01") {
        // pic is loaded first, we can create our parent pane here.
        let menu_pane_kind = u32::from_le_bytes([b'p', b'a', b'n', b'1']);
        let mut menu_pane_block = ResPane::new("trMod_menu");
        // Overall menu pane @ 0,0 to reason about positions globally
        menu_pane_block.set_pos(ResVec3::default());
        let menu_pane = build!(menu_pane_block, ResPane, menu_pane_kind, Pane);
        menu_pane.detach();
        root_pane.append_child(menu_pane);
        if MENU_PANE_PTR != menu_pane as *mut Pane as u64 {
            MENU_PANE_PTR = menu_pane as *mut Pane as u64;
            HAS_CREATED_OPT_BG = false;
            HAS_CREATED_OPT_BG_BACK = false;
            HAS_SORTED_MENU_CHILDREN = false;
            HAS_CREATED_SLIDER_BG = false;
            HAS_CREATED_SLIDER_BG_BACK = false;
        }
    }

    // Menu footer background
    if (*block).name_matches("pic_help_bg_00") {
        let menu_pane = root_pane.find_pane_by_name("trMod_menu", true).unwrap();
        let block = block as *mut ResPictureWithTex<1>;
        // For menu backing
        let mut pic_menu_block = *block;
        pic_menu_block.set_name("trMod_menu_footer_bg");
        let pic_menu_pane = build!(pic_menu_block, ResPictureWithTex<1>, kind, Picture);
        pic_menu_pane.detach();

        menu_pane.append_child(pic_menu_pane);
    }

    // Menu footer text
    if (*block).name_matches("set_txt_help_00") {
        let menu_pane = root_pane.find_pane_by_name("trMod_menu", true).unwrap();

        let block = data as *mut ResTextBox;
        let mut text_block = *block;
        text_block.set_name("trMod_menu_footer_txt");

        let text_pane = build!(text_block, ResTextBox, kind, TextBox);
        text_pane.set_text_string("Footer!");
        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        text_pane.set_default_material_colors();
        text_pane.set_color(255, 255, 255, 255);
        text_pane.detach();
        menu_pane.append_child(text_pane);
    }

    (0..NUM_MENU_TABS).for_each(|txt_idx| {
        if (*block).name_matches("set_txt_num_01") {
            let menu_pane = root_pane.find_pane_by_name("trMod_menu", true).unwrap();

            let block = data as *mut ResTextBox;
            let mut text_block = *block;
            text_block.enable_shadow();
            text_block.text_alignment(TextAlignment::Center);

            let x = txt_idx;
            text_block.set_name(format!("trMod_menu_tab_{x}").as_str());

            let mut x_offset = x as f32 * 300.0;
            // Center current tab since we don't have a help key
            if x == 1 {
                x_offset -= 25.0;
            }
            text_block.set_pos(ResVec3::new(
                menu_pos.x - 25.0 + x_offset,
                menu_pos.y + 75.0,
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
            help_block.set_name(format!("trMod_menu_tab_help_{x}").as_str());

            let x_offset = x as f32 * 300.0;
            help_block.set_pos(ResVec3::new(
                menu_pos.x - 250.0 + x_offset,
                menu_pos.y + 75.0,
                0.0,
            ));
            let help_pane = build!(help_block, ResTextBox, kind, TextBox);
            help_pane.set_text_string("abcd");
            let it = help_pane.m_text_buf as *mut u16;
            match txt_idx {
                // Left Tab: ZL
                0 => {
                    *it = 0xE0E6;
                    *(it.add(1)) = 0x0;
                    help_pane.m_text_len = 1;
                }
                1 => {
                    *it = 0x0;
                    help_pane.m_text_len = 1;
                }
                // Right Tab: ZR
                2 => {
                    *it = 0xE0E7;
                    *(it.add(1)) = 0x0;
                    help_pane.m_text_len = 1;
                }
                _ => {}
            }

            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            help_pane.set_default_material_colors();
            help_pane.set_color(255, 255, 255, 255);
            help_pane.detach();
            menu_pane.append_child(help_pane);
        }
    });

    (0..NUM_MENU_TEXT_OPTIONS).for_each(|txt_idx| {
        let x = txt_idx % 3;
        let y = txt_idx / 3;

        if (*block).name_matches("set_txt_num_01") {
            let menu_pane = root_pane.find_pane_by_name("trMod_menu", true).unwrap();

            let block = data as *mut ResTextBox;
            let mut text_block = *block;
            text_block.enable_shadow();
            text_block.text_alignment(TextAlignment::Center);

            text_block.set_name(menu_text_name_fmt!(x, y));

            let x_offset = x as f32 * 500.0;
            let y_offset = y as f32 * 85.0;
            text_block.set_pos(ResVec3::new(
                menu_pos.x - 480.0 + x_offset,
                menu_pos.y - 50.0 - y_offset,
                0.0,
            ));
            let text_pane = build!(text_block, ResTextBox, kind, TextBox);
            text_pane.set_text_string(format!("Opt {txt_idx}!").as_str());
            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            text_pane.set_default_material_colors();
            text_pane.set_color(255, 255, 255, 255);
            text_pane.detach();
            menu_pane.append_child(text_pane);

            let mut check_block = *block;
            // Font Idx 2 = nintendo64 which contains nice symbols
            check_block.font_idx = 2;

            check_block.set_name(menu_text_check_fmt!(x, y));
            check_block.set_pos(ResVec3::new(
                menu_pos.x - 375.0 + x_offset,
                menu_pos.y - 50.0 - y_offset,
                0.0,
            ));
            let check_pane = build!(check_block, ResTextBox, kind, TextBox);
            check_pane.set_text_string(format!("Check {txt_idx}!").as_str());
            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            check_pane.set_default_material_colors();
            check_pane.set_color(0, 0, 0, 255);
            check_pane.detach();
            menu_pane.append_child(check_pane);
        }
    });

    // Slider visualization

    // UI Backing
    let slider_root_name = "slider_menu";
    let slider_container_name = "slider_ui_container";

    if (*block).name_matches("pic_numbase_01") {
        let menu_pane = root_pane.find_pane_by_name("trMod_menu", true).unwrap();
        let slider_ui_root_pane_kind = u32::from_le_bytes([b'p', b'a', b'n', b'1']);
        let mut slider_ui_root_block = ResPane::new(slider_root_name);

        slider_ui_root_block.set_pos(ResVec3::default());

        let slider_ui_root = build!(
            slider_ui_root_block,
            ResPane,
            slider_ui_root_pane_kind,
            Pane
        );

        slider_ui_root.detach();
        menu_pane.append_child(slider_ui_root);

        let block = data as *mut ResPictureWithTex<1>;

        let mut picture_block = *block;

        picture_block.set_name(slider_container_name);
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
    }

    if (*block).name_matches("txt_cap_01") {
        let container_pane = root_pane.find_pane_by_name(slider_root_name, true).unwrap();

        let block = data as *mut ResTextBox;
        let mut title_block = *block;

        title_block.set_name("slider_title");
        title_block.set_pos(ResVec3::new(-530.0, 285.0, 0.0));
        title_block.set_size(ResVec2::new(550.0, 100.0));
        title_block.font_size = ResVec2::new(50.0, 100.0);

        let title_pane = build!(title_block, ResTextBox, kind, TextBox);

        title_pane.set_text_string(format!("Slider Title").as_str());

        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        title_pane.set_default_material_colors();

        // Header should be white text
        title_pane.set_color(255, 255, 255, 255);
        title_pane.detach();
        container_pane.append_child(title_pane);
    }

    (0..NUM_MENU_TEXT_SLIDERS).for_each(|idx| {
        let x = idx % 2;

        let label_x_offset = x as f32 * 345.0;

        if (*block).name_matches("set_txt_num_01") {
            let slider_root_pane = root_pane.find_pane_by_name(slider_root_name, true).unwrap();
            let slider_container = root_pane
                .find_pane_by_name(slider_container_name, true)
                .unwrap();

            let block = data as *mut ResTextBox;

            let mut text_block = *block;

            text_block.enable_shadow();
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

            label_block.enable_shadow();
            label_block.text_alignment(TextAlignment::Center);
            label_block.set_name(menu_slider_label_fmt!(idx));
            label_block.set_pos(ResVec3::new(
                slider_root_pane.pos_x - 750.0 + label_x_offset,
                slider_root_pane.pos_y + slider_container.size_y * 0.458 + 5.0,
                0.0,
            ));
            label_block.font_size = ResVec2::new(25.0, 50.0);

            // Aligns text to the center horizontally
            label_block.text_position = 4;

            let label_pane = build!(label_block, ResTextBox, kind, TextBox);

            label_pane.set_text_string(format!("Slider opt {idx}!").as_str());
            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            label_pane.set_default_material_colors();
            label_pane.set_color(250, 250, 250, 255);
            label_pane.detach();

            slider_root_pane.append_child(label_pane);
        }
    });

    // Display panes
    (0..NUM_DISPLAY_PANES).for_each(|idx| {
        let mod_prefix = "trMod_disp_";
        let parent_name = format!("{mod_prefix}{idx}");
        let pic_name = format!("{mod_prefix}{idx}_base");
        let header_name = format!("{mod_prefix}{idx}_header");
        let txt_name = format!("{mod_prefix}{idx}_txt");

        if (*block).name_matches("pic_numbase_01") {
            let block = block as *mut ResPictureWithTex<1>;
            let mut pic_block = *block;
            pic_block.set_name(pic_name.as_str());
            pic_block.set_pos(ResVec3::default());
            let pic_pane = build!(pic_block, ResPictureWithTex<1>, kind, Picture);
            pic_pane.detach();

            // pic is loaded first, we can create our parent pane here.
            let disp_pane_kind = u32::from_le_bytes([b'p', b'a', b'n', b'1']);
            let mut disp_pane_block = ResPane::new(parent_name.as_str());
            disp_pane_block.set_pos(ResVec3::new(806.0, 390.0 - (idx as f32 * 110.0), 0.0));
            let disp_pane = build!(disp_pane_block, ResPane, disp_pane_kind, Pane);
            disp_pane.detach();
            root_pane.append_child(disp_pane);
            disp_pane.append_child(pic_pane);
        }

        if (*block).name_matches("set_txt_num_01") {
            let disp_pane = root_pane
                .find_pane_by_name(parent_name.as_str(), true)
                .unwrap();

            let block = data as *mut ResTextBox;
            let mut text_block = *block;
            text_block.set_name(txt_name.as_str());
            text_block.set_pos(ResVec3::new(-10.0, -25.0, 0.0));
            let text_pane = build!(text_block, ResTextBox, kind, TextBox);
            text_pane.set_text_string(format!("Pane {idx}!").as_str());
            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            text_pane.set_default_material_colors();
            text_pane.detach();
            disp_pane.append_child(text_pane);
        }

        if (*block).name_matches("txt_cap_01") {
            let disp_pane = root_pane
                .find_pane_by_name(parent_name.as_str(), true)
                .unwrap();

            let block = data as *mut ResTextBox;
            let mut header_block = *block;
            header_block.set_name(header_name.as_str());
            header_block.set_pos(ResVec3::new(0.0, 25.0, 0.0));
            let header_pane = build!(header_block, ResTextBox, kind, TextBox);
            header_pane.set_text_string(format!("Header {idx}").as_str());
            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            header_pane.set_default_material_colors();
            // Header should be white text
            header_pane.set_color(255, 255, 255, 255);
            header_pane.detach();
            disp_pane.append_child(header_pane);
        }
    });

    original!()(
        layout,
        out_build_result_information,
        device,
        data,
        parts_build_data_set,
        build_arg_set,
        build_res_set,
        kind,
    )
}

pub fn install_hooks() {
    skyline::install_hooks!(handle_draw, layout_build_parts_impl,);
}
