use crate::{training::combo::FRAME_ADVANTAGE, common::menu::QUICK_MENU_ACTIVE};
use crate::training::ui::*;
use crate::common::get_player_dmg_digits;
use crate::common::MENU;
use crate::consts::FighterId;
use training_mod_consts::{OnOff, SaveDamage};
use training_mod_tui::gauge::GaugeState;

pub static NUM_DISPLAY_PANES : usize = 1;
pub static NUM_MENU_TEXT_OPTIONS : usize = 27;
pub static NUM_MENU_TEXT_SLIDERS : usize = 4;
pub static NUM_MENU_TABS : usize = 3;

#[skyline::hook(offset = 0x4b620)]
pub unsafe fn handle_draw(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    let layout_name = skyline::from_c_str((*layout).layout_name);
    let layout_root_pane = &*(*layout).root_pane;

    // Update percentage display as soon as possible on death,
    // only if we have random save state damage active
    if crate::common::is_training_mode() && 
        (MENU.save_damage_cpu == SaveDamage::RANDOM || MENU.save_damage_player == SaveDamage::RANDOM) && 
        layout_name == "info_melee" {
        for player_name in &["p1", "p2"] {
            if let Some(parent) = layout_root_pane.find_pane_by_name_recursive(player_name) {
                let _p1_layout_name = skyline::from_c_str((*(*parent.as_parts()).layout).layout_name);
                let anim_list = &mut (*(*parent.as_parts()).layout).anim_trans_list;

                let mut has_altered_anim_list = false;
                let (hundreds, tens, _, _) = get_player_dmg_digits(
                    match *player_name {
                        "p1" => FighterId::Player,
                        "p2" => FighterId::CPU,
                        _ => panic!("Unknown player name: {}", player_name)
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
                    "set_dmg_p"
                ] {
                    if let Some(dmg_num) = parent.find_pane_by_name_recursive(dmg_num_s) {
                        if (dmg_num_s.contains('3') && hundreds == 0) || 
                            (dmg_num_s.contains('2') && hundreds == 0 && tens == 0) {
                            continue;
                        }

                        if *dmg_num_s == "set_dmg_p" {
                            println!("{}: {}", dmg_num_s, dmg_num.pos_y);
                            dmg_num.pos_y = 0.0;
                        } else if *dmg_num_s == "set_dmg_num_p" {
                            println!("{}: {}", dmg_num_s, dmg_num.pos_y);
                            dmg_num.pos_y = -4.0;
                        } else if *dmg_num_s == "dig_dec" {
                            println!("{}: {}", dmg_num_s, dmg_num.pos_y);
                            dmg_num.pos_y = -16.0;
                        } else {
                            dmg_num.pos_y = 0.0;
                        }

                        if dmg_num.alpha != 255 || dmg_num.global_alpha != 255 {
                            dmg_num.alpha = 255;
                            dmg_num.global_alpha = 255;
                            if !has_altered_anim_list {
                                anim_list.iterate_anim_list(Some(player_name));
                                has_altered_anim_list = true;
                            }
                        }
                    }
                }

                for death_explosion_s in &["set_fxui_dead1", "set_fxui_dead2", "set_fxui_dead3"] {
                    if let Some(death_explosion) = parent.find_pane_by_name_recursive(death_explosion_s) {
                        death_explosion.alpha = 0;
                        death_explosion.global_alpha = 0;
                    }
                }
            }
        }
    }

    // Update training mod displays
    if layout_name == "info_training" {
        // Update frame advantage
        if let Some(parent) = layout_root_pane.find_pane_by_name_recursive("trMod_disp_0") {
            if crate::common::MENU.frame_advantage == OnOff::On {
                parent.alpha = 255;
                parent.global_alpha = 255;
            } else {
                parent.alpha = 0;
                parent.global_alpha = 0;
            }
        }

        if let Some(header) = layout_root_pane.find_pane_by_name_recursive("trMod_disp_0_header") {
            header.set_text_string("Frame Advantage");
        }

        if let Some(text) = layout_root_pane.find_pane_by_name_recursive("trMod_disp_0_txt") {
            text.set_text_string(format!("{FRAME_ADVANTAGE}").as_str());
            let text = text.as_textbox();
            if FRAME_ADVANTAGE < 0 {
                text.set_color(200, 8, 8, 255);
            } else if FRAME_ADVANTAGE == 0 {
                text.set_color(0, 0, 0, 255);
            } else {
                text.set_color(31, 198, 0, 255);
            }
        }


        // Update menu display
        // Grabbing lock as read-only, essentially
        let app = &*crate::common::menu::QUICK_MENU_APP.data_ptr();

        let menu_pane = layout_root_pane
            .find_pane_by_name_recursive("trMod_menu")
            .unwrap(); 
        if QUICK_MENU_ACTIVE {
            menu_pane.alpha = 255;
            menu_pane.global_alpha = 255;
        } else {
            menu_pane.alpha = 0;
            menu_pane.global_alpha = 0;
        }

        // Make all invisible first
        (0..NUM_MENU_TEXT_OPTIONS)
            .for_each(|idx| {
                let x = idx % 3;
                let y = idx / 3;
                layout_root_pane.find_pane_by_name_recursive(&format!("trMod_menu_opt_{x}_{y}").to_owned())
                    .map(|text| {
                        text.alpha = 0;
                        text.global_alpha = 0;
                    });
                layout_root_pane.find_pane_by_name_recursive(&format!("trMod_menu_check_{x}_{y}").to_owned())
                    .map(|text| {
                        text.alpha = 0;
                        text.global_alpha = 0;
                    });
            });
        (0..NUM_MENU_TEXT_SLIDERS)
            .for_each(|idx| {
                layout_root_pane.find_pane_by_name_recursive(&format!("trMod_menu_slider_{idx}").to_owned())
                    .map(|text| {
                        text.alpha = 0;
                        text.global_alpha = 0;
                    });
            });

        let app_tabs = &app.tabs.items;
        let tab_selected = app.tabs.state.selected().unwrap();
        let prev_tab = if tab_selected == 0 { app_tabs.len() - 1 } else { tab_selected - 1 };
        let next_tab = if tab_selected == app_tabs.len() - 1 { 0 } else { tab_selected + 1 };
        let tab_titles = [prev_tab, tab_selected, next_tab]
            .map(|idx| app_tabs[idx]);
        

        (0..NUM_MENU_TABS).for_each(|idx| {
            layout_root_pane.find_pane_by_name_recursive(&format!("trMod_menu_tab_{idx}").to_owned())
                .map(|text| text.set_text_string(tab_titles[idx]) );
        });

        if app.outer_list {
            let tab_selected = app.tab_selected();
            let tab = app.menu_items.get(tab_selected).unwrap();
            
            (0..NUM_MENU_TEXT_OPTIONS)
                // Valid options in this submenu
                .filter_map(|idx| tab.idx_to_list_idx_opt(idx))
                .map(|(list_section, list_idx)| (list_section, list_idx, 
                    layout_root_pane.find_pane_by_name_recursive(
                        &format!("trMod_menu_opt_{list_section}_{list_idx}").to_owned()).unwrap()))
                .for_each(|(list_section, list_idx, text)| {
                    let list = &tab.lists[list_section];
                    let submenu = &list.items[list_idx];
                    let is_selected = list.state.selected().filter(|s| *s == list_idx).is_some();
                    text.set_text_string(submenu.submenu_title);
                    text.alpha = 255;
                    text.global_alpha = 255;
                    let text = text.as_textbox();
                    if is_selected {
                        text.set_color(0x27, 0x4E, 0x13, 255);
                        if let Some(footer) = layout_root_pane.find_pane_by_name_recursive(&format!("trMod_menu_footer_txt").to_owned()) {
                            footer.set_text_string(submenu.help_text);
                        }
                    } else {
                        text.set_color(0, 0, 0, 255);
                    }
                });
        } else {
            if matches!(app.selected_sub_menu_slider.state, GaugeState::None) {
                let (_title, _help_text, mut sub_menu_str_lists) = app.sub_menu_strs_and_states();
                for list_section in 0..sub_menu_str_lists.len() {
                    let sub_menu_str = sub_menu_str_lists[list_section].0.clone();
                    let sub_menu_state = &mut sub_menu_str_lists[list_section].1;
                    sub_menu_str
                        .iter()
                        .enumerate()
                        .for_each(|(idx, (checked, name))| {
                            let is_selected = sub_menu_state.selected().filter(|s| *s == idx).is_some();
                            if let Some(text) = layout_root_pane.find_pane_by_name_recursive(
                                &format!("trMod_menu_opt_{list_section}_{idx}").to_owned()) {
                                let text = text.as_textbox();
                                text.set_text_string(name);
                                if is_selected {
                                    text.set_color(0x27, 0x4E, 0x13, 255);
                                } else {
                                    text.set_color(0, 0, 0, 255);
                                }
                                text.alpha = 255;
                                text.global_alpha = 255;
                            }

                            if let Some(check) = layout_root_pane.find_pane_by_name_recursive(
                                &format!("trMod_menu_check_{list_section}_{idx}").to_owned()) {
                                if *checked {
                                    let check = check.as_textbox();

                                    check.set_text_string("+");
                                    check.alpha = 255;
                                    check.global_alpha = 255;
                                }
                            }
                        });
                }
            } else {
                let (_title, _help_text, gauge_vals) = app.sub_menu_strs_for_slider();
                let abs_min = gauge_vals.abs_min;
                let abs_max = gauge_vals.abs_max;
                let selected_min = gauge_vals.selected_min;
                let selected_max = gauge_vals.selected_max;
                if let Some(text) = layout_root_pane.find_pane_by_name_recursive("trMod_menu_slider_0") {
                    let text = text.as_textbox();
                    text.alpha = 255;
                    text.global_alpha = 255;
                    text.set_text_string(&format!("{abs_min}"));
                }

                if let Some(text) = layout_root_pane.find_pane_by_name_recursive("trMod_menu_slider_1") {
                    let text = text.as_textbox();
                    text.alpha = 255;
                    text.global_alpha = 255;
                    text.set_text_string(&format!("{selected_min}"));
                    match gauge_vals.state {
                        GaugeState::MinHover => text.set_color(200, 8, 8, 255),
                        GaugeState::MinSelected => text.set_color(8, 200, 8, 255),
                        _ => text.set_color(0, 0, 0, 255)
                    }
                }

                if let Some(text) = layout_root_pane.find_pane_by_name_recursive("trMod_menu_slider_2") {
                    let text = text.as_textbox();
                    text.alpha = 255;
                    text.global_alpha = 255;
                    text.set_text_string(&format!("{selected_max}"));
                    match gauge_vals.state {
                        GaugeState::MaxHover => text.set_color(200, 8, 8, 255),
                        GaugeState::MaxSelected => text.set_color(8, 200, 8, 255),
                        _ => text.set_color(0, 0, 0, 255)
                    }
                }

                if let Some(text) = layout_root_pane.find_pane_by_name_recursive("trMod_menu_slider_3") {
                    let text = text.as_textbox();
                    text.alpha = 255;
                    text.global_alpha = 255;
                    text.set_text_string(&format!("{abs_max}"));
                }
            }
        }
    }

    original!()(layout, draw_info, cmd_buffer);
}

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

    let root_pane = &*(*layout).root_pane;

    let block = data as *mut ResPane;

    let menu_pos = ResVec3::new(-360.0, 440.0, 0.0);

    // Menu creation
    if (*block).name_matches("pic_numbase_01") {
        let block = block as *mut ResPictureWithTex<1>;
        // For menu backing
        let mut pic_menu_block = (*block).clone();
        pic_menu_block.picture.pane.set_name("trMod_menu_base");
        pic_menu_block.picture.pane.set_pos(menu_pos);
        pic_menu_block.picture.pane.set_size(ResVec2::new(1200.0, 1600.0));
        let pic_menu_pane = build!(pic_menu_block, ResPictureWithTex<1>, kind, Picture);
        pic_menu_pane.detach();

        // pic is loaded first, we can create our parent pane here.
        let menu_pane_kind = u32::from_le_bytes([b'p', b'a', b'n', b'1']);
        let mut menu_pane_block = ResPane::new("trMod_menu");
        // Overall menu pane @ 0,0 to reason about positions globally
        menu_pane_block.set_pos(ResVec3::default());
        let menu_pane = build!(menu_pane_block, ResPane, menu_pane_kind, Pane);
        menu_pane.detach();
        root_pane.append_child(menu_pane);
        menu_pane.append_child(pic_menu_pane);
    }

    // Menu header
    // TODO: Copy "Quit Training" window and text
    if (*block).name_matches("set_txt_num_01") {
        let menu_pane = root_pane
            .find_pane_by_name("trMod_menu", true)
            .unwrap();

        let block = data as *mut ResTextBox;

        // Header
        let mut text_block = (*block).clone();
        text_block.pane.size_x = text_block.pane.size_x * 2.0;
        text_block.pane.set_name("trMod_menu_header");

        text_block.pane.set_pos(ResVec3::new(menu_pos.x - 525.0, menu_pos.y + 75.0, 0.0));
        let text_pane = build!(text_block, ResTextBox, kind, TextBox);
        text_pane.pane.set_text_string("Modpack Menu");
        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        text_pane.set_default_material_colors();
        text_pane.set_color(200, 8, 8, 255);
        text_pane.detach();
        menu_pane.append_child(text_pane);
    }

    // Menu footer background
    if (*block).name_matches("pic_help_bg_00") {
        let menu_pane = root_pane
            .find_pane_by_name("trMod_menu", true)
            .unwrap();
        let block = block as *mut ResPictureWithTex<1>;
        // For menu backing
        let mut pic_menu_block = (*block).clone();
        pic_menu_block.picture.pane.set_name("trMod_menu_footer_bg");
        let pic_menu_pane = build!(pic_menu_block, ResPictureWithTex<1>, kind, Picture);
        pic_menu_pane.detach();

        menu_pane.append_child(pic_menu_pane);
    }

    // Menu footer text
    if (*block).name_matches("set_txt_help_00") {
        let menu_pane = root_pane
            .find_pane_by_name("trMod_menu", true)
            .unwrap();

        let block = data as *mut ResTextBox;
        let mut text_block = (*block).clone();
        text_block.pane.set_name(format!("trMod_menu_footer_txt").as_str());

        let text_pane = build!(text_block, ResTextBox, kind, TextBox);
        text_pane.pane.set_text_string(format!("Footer!").as_str());
        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        text_pane.set_default_material_colors();
        text_pane.set_color(255, 255, 255, 255);
        text_pane.detach();
        menu_pane.append_child(text_pane);
    }

    (0..NUM_MENU_TABS).for_each(|txt_idx| {
        if (*block).name_matches("set_txt_num_01") {
            let menu_pane = root_pane
                .find_pane_by_name("trMod_menu", true)
                .unwrap();
    
            let block = data as *mut ResTextBox;
            let mut text_block = (*block).clone();
            text_block.enable_shadow();
            text_block.text_alignment(TextAlignment::Center);

            let x = txt_idx;
            text_block.pane.set_name(format!("trMod_menu_tab_{x}").as_str());

            let mut x_offset = x as f32 * 300.0;
            // Center current tab since we don't have a help key
            if x == 1 {
                x_offset -= 25.0;
            }
            text_block.pane.set_pos(ResVec3::new(menu_pos.x - 125.0 + x_offset, menu_pos.y + 75.0, 0.0));
            let text_pane = build!(text_block, ResTextBox, kind, TextBox);
            text_pane.pane.set_text_string(format!("Tab {txt_idx}!").as_str());
            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            text_pane.set_default_material_colors();
            text_pane.set_color(255, 255, 255, 255);
            if txt_idx == 1 {
                text_pane.set_color(255, 255, 0, 255);
            }
            text_pane.detach();
            menu_pane.append_child(text_pane);

            let mut help_block = (*block).clone();
            // Font Idx 2 = nintendo64 which contains nice symbols
            help_block.font_idx = 2;

            let x = txt_idx;
            help_block.pane.set_name(format!("trMod_menu_tab_help_{x}").as_str());

            let x_offset = x as f32 * 300.0;
            help_block.pane.set_pos(ResVec3::new(menu_pos.x - 350.0 + x_offset, menu_pos.y + 75.0, 0.0));
            let help_pane = build!(help_block, ResTextBox, kind, TextBox);
            help_pane.pane.set_text_string(format!("abcd").as_str());
            let it = help_pane.m_text_buf as *mut u16;
            match txt_idx {
                // Left Tab: ZL
                0 => {
                    *it = 0xE0E6;
                    *(it.add(1)) = 0x0;
                    help_pane.m_text_len = 1;
                },
                1 => {
                    *it = 0x0;
                    help_pane.m_text_len = 1;
                },
                // Right Tab: ZR
                2 => {
                    *it = 0xE0E7;
                    *(it.add(1)) = 0x0;
                    help_pane.m_text_len = 1;
                },
                _ => {},
            }

            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            help_pane.set_default_material_colors();
            help_pane.set_color(255, 255, 255, 255);
            help_pane.detach();
            menu_pane.append_child(help_pane);
        }
    });

    (0..NUM_MENU_TEXT_OPTIONS).for_each(|txt_idx| {
        if (*block).name_matches("set_txt_num_01") {
            let menu_pane = root_pane
                .find_pane_by_name("trMod_menu", true)
                .unwrap();
    
            let block = data as *mut ResTextBox;
            let mut text_block = (*block).clone();
            text_block.enable_shadow();
            text_block.text_alignment(TextAlignment::Center);

            let x = txt_idx % 3;
            let y = txt_idx / 3;
            text_block.pane.set_name(format!("trMod_menu_opt_{x}_{y}").as_str());

            let x_offset = x as f32 * 400.0;
            let y_offset = y as f32 * 75.0;
            text_block.pane.set_pos(ResVec3::new(menu_pos.x - 450.0 + x_offset, menu_pos.y - 25.0 - y_offset, 0.0));
            let text_pane = build!(text_block, ResTextBox, kind, TextBox);
            text_pane.pane.set_text_string(format!("Opt {txt_idx}!").as_str());
            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            text_pane.set_default_material_colors();
            text_pane.set_color(0, 0, 0, 255);
            text_pane.detach();
            menu_pane.append_child(text_pane);

            let mut check_block = (*block).clone();
            // Font Idx 2 = nintendo64 which contains nice symbols
            check_block.font_idx = 2;

            check_block.pane.set_name(format!("trMod_menu_check_{x}_{y}").as_str());
            check_block.pane.set_pos(ResVec3::new(menu_pos.x - 675.0 + x_offset, menu_pos.y - 25.0 - y_offset, 0.0));
            let check_pane = build!(check_block, ResTextBox, kind, TextBox);
            check_pane.pane.set_text_string(format!("Check {txt_idx}!").as_str());
            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            check_pane.set_default_material_colors();
            check_pane.set_color(0, 0, 0, 255);
            check_pane.detach();
            menu_pane.append_child(check_pane);
        }
    });

    // Slider visualization
    (0..NUM_MENU_TEXT_SLIDERS).for_each(|idx| {
        if (*block).name_matches("set_txt_num_01") {
            let menu_pane = root_pane
                .find_pane_by_name("trMod_menu", true)
                .unwrap();
    
            let block = data as *mut ResTextBox;
            let mut text_block = (*block).clone();
            text_block.enable_shadow();
            text_block.text_alignment(TextAlignment::Center);

            text_block.pane.set_name(format!("trMod_menu_slider_{idx}").as_str());

            let x_offset = idx as f32 * 250.0;
            text_block.pane.set_pos(ResVec3::new(menu_pos.x - 450.0 + x_offset, menu_pos.y - 150.0, 0.0));
            let text_pane = build!(text_block, ResTextBox, kind, TextBox);
            text_pane.pane.set_text_string(format!("Slider {idx}!").as_str());
            // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
            text_pane.set_default_material_colors();
            text_pane.set_color(0, 0, 0, 255);
            text_pane.detach();
            menu_pane.append_child(text_pane);
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
            let mut pic_block = (*block).clone();
            pic_block.picture.pane.set_name(pic_name.as_str());
            pic_block.picture.pane.set_pos(ResVec3::default());
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
            let mut text_block = (*block).clone();
            text_block.pane.set_name(txt_name.as_str());
            text_block.pane.set_pos(ResVec3::new(-10.0, -25.0, 0.0));
            let text_pane = build!(text_block, ResTextBox, kind, TextBox);
            text_pane.pane.set_text_string(format!("Pane {idx}!").as_str());
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
            let mut header_block = (*block).clone();
            header_block.pane.set_name(header_name.as_str());
            header_block.pane.set_pos(ResVec3::new(0.0, 25.0, 0.0));
            let header_pane = build!(header_block, ResTextBox, kind, TextBox);
            header_pane.pane.set_text_string(format!("Header {idx}").as_str());
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
