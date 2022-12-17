use crate::training::combo::FRAME_ADVANTAGE;
use crate::training::ui::*;
use skyline::hooks::InlineCtx;

#[skyline::hook(offset = 0x4b620)]
pub unsafe fn handle_draw(layout: *mut Layout, draw_info: u64, cmd_buffer: u64) {
    let layout_name = skyline::from_c_str((*layout).raw_layout.layout_name);
    let layout_root_pane = &*(*layout).raw_layout.root_pane;
    let _anim_list = &mut (*layout).raw_layout.anim_trans_list;
    // anim_list.iterate_anim_list();

    if layout_name == "info_training" {
        for s in ["txt_cap_00", "set_txt_num_00", "set_txt_num_01"] {
            let txt_pane = layout_root_pane.find_pane_by_name_recursive(s);
            if let Some(txt_pane) = txt_pane {
                txt_pane.set_text_string("Hello!");
            }
        }

        if let Some(header) = layout_root_pane.find_pane_by_name_recursive("trMod_disp_1_header") {
            header.set_text_string("Frame Advantage");
        }

        if let Some(text) = layout_root_pane.find_pane_by_name_recursive("trMod_disp_1_txt") {
            text.set_text_string(format!("{FRAME_ADVANTAGE}").as_str());
        }
    }

    original!()(layout, draw_info, cmd_buffer);
}

#[skyline::hook(offset = 0x3794e80)]
pub unsafe fn handle_find_animation_by_name(
    layout_view: *const u64,
    s: *const skyline::libc::c_char,
) -> u64 {
    let ret = original!()(layout_view, s);
    if skyline::from_c_str(s) == "changedig" {
        let ret = ret as *mut AnimTransform;
        if !ret.is_null() {
            ret.as_mut().unwrap().parse_anim_transform();
        }
    }

    ret
}

#[skyline::hook(offset = 0x37ac310, inline)]
pub unsafe fn general_number_formatter(_ctx: &mut InlineCtx) {}

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
    let layout_name = skyline::from_c_str((*layout).raw_layout.layout_name);
    let _kind_str: String = kind.to_le_bytes().map(|b| b as char).iter().collect();

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

    let root_pane = (*layout).raw_layout.root_pane;

    let block = data as *mut ResPane;
    (0..4).for_each(|idx| {
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
            let pic_pane = original!()(
                layout,
                out_build_result_information,
                device,
                &mut pic_block as *mut ResPictureWithTex<1> as *mut u8,
                parts_build_data_set,
                build_arg_set,
                build_res_set,
                kind,
            );
            (*(*pic_pane).parent).remove_child(&*pic_pane);

            // pic is loaded first, we can create our parent pane here.
            let disp_pane_kind = u32::from_le_bytes([b'p', b'a', b'n', b'1']);
            let mut disp_pane_block = ResPane::new(parent_name.as_str());
            disp_pane_block.set_pos(ResVec3::new(70.0 + (idx as f32 * 250.0), -440.0, 0.0));
            let disp_pane = original!()(
                layout,
                out_build_result_information,
                device,
                &mut disp_pane_block as *mut ResPane as *mut u8,
                parts_build_data_set,
                build_arg_set,
                build_res_set,
                disp_pane_kind,
            );
            (*(*disp_pane).parent).remove_child(&*disp_pane);
            (*root_pane).append_child(&*disp_pane);
            (*disp_pane).append_child(&*pic_pane);
        }

        if (*block).name_matches("set_txt_num_01") {
            let disp_pane = (*root_pane)
                .find_pane_by_name(parent_name.as_str(), true)
                .unwrap();

            let block = data as *mut ResTextBox;
            let mut text_block = (*block).clone();
            text_block.pane.set_name(txt_name.as_str());
            text_block.pane.set_pos(ResVec3::new(-10.0, -25.0, 0.0));
            let text_pane = original!()(
                layout,
                out_build_result_information,
                device,
                &mut text_block as *mut ResTextBox as *mut u8,
                parts_build_data_set,
                build_arg_set,
                build_res_set,
                kind,
            );
            (*text_pane).set_text_string(format!("Pane {idx}!").as_str());
            println!("Pane {idx}: {:#?}", *(text_pane as *mut TextBox));
            (*(text_pane as *mut TextBox)).set_color(240 / (idx + 1), 0, (idx + 1) * 60, 255);
            (*((*(text_pane as *mut TextBox)).m_pMaterial)).set_white_color(240.0 / (idx as f32 + 1.0), 0.0, (idx as f32 + 1.0) * 60.0, 255.0);
            (*(*text_pane).parent).remove_child(&*text_pane);
            (*disp_pane).append_child(&*text_pane);
        }

        if (*block).name_matches("txt_cap_01") {
            let disp_pane = (*root_pane)
                .find_pane_by_name(parent_name.as_str(), true)
                .unwrap();

            let block = data as *mut ResTextBox;
            let mut header_block = (*block).clone();
            header_block.pane.set_name(header_name.as_str());
            header_block.pane.set_pos(ResVec3::new(0.0, 25.0, 0.0));
            let header_pane = original!()(
                layout,
                out_build_result_information,
                device,
                &mut header_block as *mut ResTextBox as *mut u8,
                parts_build_data_set,
                build_arg_set,
                build_res_set,
                kind,
            );
            (*header_pane).set_text_string(format!("Header {idx}").as_str());
            (*(*header_pane).parent).remove_child(&*header_pane);
            (*disp_pane).append_child(&*header_pane);
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

#[skyline::hook(offset = 0x47db0, inline)]
pub unsafe fn layout_build_pane_obj(ctx: &mut InlineCtx) {
    println!("Layout BuildPaneObj:\n{}", ctx);
}

pub fn install_hooks() {
    skyline::install_hooks!(
        handle_draw,
        handle_find_animation_by_name,
        general_number_formatter,
        layout_build_parts_impl,
        layout_build_pane_obj
    );
}
