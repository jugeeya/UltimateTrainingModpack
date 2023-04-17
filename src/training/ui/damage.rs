use skyline::nn::ui2d::*;
use smash::ui2d::SmashPane;

use crate::common::{get_player_dmg_digits, is_ready_go, is_training_mode};
use crate::consts::FighterId;

use std::ffi::CStr;
use std::slice;

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

pub fn parse_anim_transform(anim_transform: &mut AnimTransform, layout_name: Option<&str>) {
    let res_animation_block_data_start = anim_transform.res_animation_block.cast::<u8>();
    let res_animation_block = unsafe {
        anim_transform
            .res_animation_block
            .as_ref()
            .expect("Invalid AnimationBlock pointer!")
    };

    let anim_cont_offsets: &[u32] = unsafe {
        slice::from_raw_parts(
            res_animation_block_data_start
                .add(res_animation_block.anim_cont_offsets_offset as usize)
                .cast::<u32>(),
            res_animation_block.anim_cont_count as usize,
        )
    };

    for &offset in anim_cont_offsets {
        let res_animation_cont = unsafe {
            res_animation_block_data_start
                .add(offset as usize)
                .cast::<ResAnimationContent>()
                .as_ref()
                .expect("Invalid AnimationContent pointer!")
        };

        let name = CStr::from_bytes_with_nul(&res_animation_cont.name)
            .ok()
            .and_then(|s| CStr::to_str(s).ok());
        let anim_type = res_animation_cont.anim_content_type;

        if let Some(name) = name {
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
                    } else if name == "set_dmg_num_2" {
                        anim_transform.frame = tens as f32;
                    } else if name == "set_dmg_num_1" {
                        anim_transform.frame = ones as f32;
                    } else if name == "set_dmg_num_dec" {
                        anim_transform.frame = dec as f32;
                    }
                }
            }
        }
    }
}

pub unsafe fn draw(root_pane: &mut Pane, layout_name: &str) {
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
                            dmg_num.alpha = 255;
                            dmg_num.global_alpha = 255;
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
                        death_explosion.alpha = 0;
                        death_explosion.global_alpha = 0;
                    }
                }
            }
        }
    }
}
