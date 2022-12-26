use crate::training::combo::FRAME_ADVANTAGE;
use crate::training::ui::*;
use training_mod_consts::OnOff;

pub static NUM_DISPLAY_PANES : usize = 1;
pub static NUM_MENU_TEXT_OPTIONS : usize = 15;

#[skyline::hook(offset = 0x4b620)]
pub unsafe fn handle_draw(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    let layout_name = skyline::from_c_str((*layout).layout_name);
    let layout_root_pane = &*(*layout).root_pane;

    if crate::common::is_training_mode() && layout_name == "info_melee" {
        if let Some(parent) = layout_root_pane.find_pane_by_name_recursive("p1") {
            let p1_layout_name = skyline::from_c_str((*(*parent.as_parts()).layout).layout_name);
            let anim_list = &mut (*(*parent.as_parts()).layout).anim_trans_list;
            anim_list.iterate_anim_list();

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
                "dig_dec_anim_01"
            ] {
                if let Some(dmg_num) = parent.find_pane_by_name_recursive(dmg_num_s) {
                    dmg_num.alpha = 255;
                    dmg_num.global_alpha = 255;
                }
            }
        }
    }

    if layout_name == "info_training" {
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


        // Grabbing lock as read-only, essentially
        let app = &*crate::common::menu::QUICK_MENU_APP.data_ptr();
        if app.outer_list {
            let tab_selected = app.tab_selected();
            // let mut item_help = None;
            let tab = app.menu_items.get(tab_selected).unwrap();
            
            (0..NUM_MENU_TEXT_OPTIONS)
                // Valid options in this submenu
                .filter_map(|idx| tab.idx_to_list_idx_opt(idx))
                .map(|(list_section, list_idx)| (list_section, list_idx, 
                    layout_root_pane.find_pane_by_name_recursive(
                        &format!("trMod_menu_opt_{list_idx}_{list_section}").to_owned()).unwrap()))
                .for_each(|(list_section, list_idx, text)| {
                    let list = &tab.lists[list_section];
                    let submenu = &list.items[list_idx];
                    let is_selected = list.state.selected().filter(|s| *s == list_idx).is_some();
                    text.set_text_string(submenu.submenu_title);
                    text.alpha = 255;
                    text.global_alpha = 255;
                    let text = text.as_textbox();
                    if is_selected {
                        text.set_color(31, 198, 0, 255);
                        if let Some(footer) = layout_root_pane.find_pane_by_name_recursive(&format!("trMod_menu_footer").to_owned()) {
                            footer.set_text_string(submenu.help_text);
                        }
                    } else {
                        text.set_color(0, 0, 0, 255);
                    }
                });
        
            (0..NUM_MENU_TEXT_OPTIONS)
                // Invalid options in this submenu
                .filter(|idx| tab.idx_to_list_idx_opt(*idx).is_none())
                .for_each(|idx| {
                    let x = idx % 3;
                    let y = idx / 3;
                    layout_root_pane.find_pane_by_name_recursive(&format!("trMod_menu_opt_{y}_{x}").to_owned())
                        .map(|text| {
                            text.alpha = 0;
                            text.global_alpha = 0;
                        });
                });
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

    // Menu creation
    if (*block).name_matches("pic_numbase_01") {
        let block = block as *mut ResPictureWithTex<1>;
        // For menu backing
        let mut pic_menu_block = (*block).clone();
        pic_menu_block.picture.pane.set_name("trMod_menu_base");
        pic_menu_block.picture.pane.set_pos(ResVec3::default());
        pic_menu_block.picture.pane.set_size(ResVec2::new(1200.0, 1600.0));
        let pic_menu_pane = build!(pic_menu_block, ResPictureWithTex<1>, kind, Picture);
        pic_menu_pane.detach();

        // pic is loaded first, we can create our parent pane here.
        let menu_pane_kind = u32::from_le_bytes([b'p', b'a', b'n', b'1']);
        let mut menu_pane_block = ResPane::new("trMod_menu");
        // X should be -960.0 + 600.0
        menu_pane_block.set_pos(ResVec3::new(600.0, 540.0 - 100.0, 0.0));
        let menu_pane = build!(menu_pane_block, ResPane, menu_pane_kind, Pane);
        menu_pane.detach();
        root_pane.append_child(menu_pane);
        menu_pane.append_child(pic_menu_pane);
    }

    // Menu header, footer
    if (*block).name_matches("set_txt_num_01") {
        let menu_pane = root_pane
            .find_pane_by_name("trMod_menu", true)
            .unwrap();

        let block = data as *mut ResTextBox;

        // Header
        let mut text_block = (*block).clone();
        text_block.pane.size_x = text_block.pane.size_x * 2.0;
        text_block.pane.set_name("trMod_menu_header");

        text_block.pane.set_pos(ResVec3::new(-350.0, 75.0, 0.0));
        let text_pane = build!(text_block, ResTextBox, kind, TextBox);
        text_pane.pane.set_text_string("Ultimate Training Modpack Menu");
        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        text_pane.set_default_material_colors();
        text_pane.set_color(200, 8, 8, 255);
        text_pane.detach();
        menu_pane.append_child(text_pane);

        // Footer
        let mut text_block = (*block).clone();
        text_block.pane.size_x = text_block.pane.size_x * 4.0;
        text_block.pane.set_name("trMod_menu_footer");

        text_block.pane.set_pos(ResVec3::new(-150.0, -300.0, 0.0));
        let text_pane = build!(text_block, ResTextBox, kind, TextBox);
        text_pane.pane.set_text_string("Footer");
        // Ensure Material Colors are not hardcoded so we can just use SetTextColor.
        text_pane.set_default_material_colors();
        text_pane.set_color(8, 8, 200, 255);
        text_pane.detach();
        menu_pane.append_child(text_pane);
    }

    (0..NUM_MENU_TEXT_OPTIONS).for_each(|txt_idx| {
        if (*block).name_matches("set_txt_num_01") {
            let menu_pane = root_pane
                .find_pane_by_name("trMod_menu", true)
                .unwrap();
    
            let block = data as *mut ResTextBox;
            let mut text_block = (*block).clone();
            let x = txt_idx % 3;
            let y = txt_idx / 3;
            text_block.pane.set_name(format!("trMod_menu_opt_{y}_{x}").as_str());

            let x_offset = x as f32 * 300.0;
            let y_offset = y as f32 * 50.0;
            text_block.pane.set_pos(ResVec3::new(-450.0 + x_offset, -25.0 - y_offset, 0.0));
            let text_pane = build!(text_block, ResTextBox, kind, TextBox);
            text_pane.pane.set_text_string(format!("Opt {txt_idx}!").as_str());
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
