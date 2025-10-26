use crate::common::try_get_battle_object;
use crate::offsets::OFFSET_KIRBY_OPFF;
use crate::training::charge::ChargeState;
use crate::training::save_states;
use smash::app::{self, lua_bind::*, smashball::is_training_mode};
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};

#[repr(C)]
pub struct CopyModule {
    _vtable: u64, // maybe, no clue
    padding: [u8; 0x17390],
    copied_fighter_kind: i32,
}

// Wait to set up copy ability variables until after CopyStart runs;
#[skyline::hook(offset = *OFFSET_KIRBY_OPFF)]
pub unsafe fn handle_copy_start(param1: u64, kirby_fighter: *mut app::Fighter) -> u64 {
    if !is_training_mode() || !save_states::is_loading() {
        return original!()(param1, kirby_fighter);
    }
    // Need to check copy start before the function runs, since it will turn it off if it was on
    let module_accessor = (*kirby_fighter).battle_object.module_accessor;
    let on_copy_start = WorkModule::is_flag(module_accessor, 0x20000104); // *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY_ON_START
                                                                          // Run optional initial copy setup (it depends on fighter kind)
    let ori = original!()(param1, kirby_fighter);
    // Now try to set save state variables
    if on_copy_start {
        let copy_module =
            WorkModule::get_int64(module_accessor, 0x10000106) as *const i64 as *const CopyModule; //*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_MODULE_ADDRESS
        let opponent_fighter_kind = (*copy_module).copied_fighter_kind;
        handle_kirby_hat_charge(
            &mut *module_accessor,
            opponent_fighter_kind,
            save_states::get_charge_state(module_accessor),
        );
        save_states::end_copy_ability(module_accessor);
    }
    ori
}

pub unsafe fn is_kirby_hat_okay(
    opponent_module_accessor: &mut app::BattleObjectModuleAccessor,
    save_state_fighter_option: Option<i32>,
) -> Option<bool> {
    let mut opponent_fighter_kind = app::utility::get_kind(opponent_module_accessor);
    let save_state_fighter_kind = save_state_fighter_option?;
    if opponent_fighter_kind == save_state_fighter_kind {
        return Some(true);
    }
    // We have a fighter but they don't match - see if it's an accepted transformation
    let trainer_kinds = [
        *FIGHTER_KIND_PZENIGAME,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_PLIZARDON,
        -1, // Fighter Kind while switching pokemon
    ];
    let element_kinds = [*FIGHTER_KIND_EFLAME, *FIGHTER_KIND_ELIGHT];
    if opponent_fighter_kind == -1 {
        let trainer_boid = LinkModule::get_parent_object_id(
            opponent_module_accessor,
            *FIGHTER_POKEMON_LINK_NO_PTRAINER,
        ) as u32;
        if trainer_boid != *BATTLE_OBJECT_ID_INVALID as u32
            && app::sv_battle_object::is_active(trainer_boid)
        {
            opponent_fighter_kind = *FIGHTER_KIND_PZENIGAME; // ptrainer is in the match, so assume we have a ptrainer fighter
        }
    }
    let both_trainer = trainer_kinds.contains(&opponent_fighter_kind)
        && trainer_kinds.contains(&save_state_fighter_kind);
    let both_element = element_kinds.contains(&opponent_fighter_kind)
        && element_kinds.contains(&save_state_fighter_kind);
    Some(both_trainer || both_element)
}

pub unsafe fn get_kirby_hat_charge(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    opponent_fighter_kind: i32,
    charge_state: ChargeState,
) -> ChargeState {
    if opponent_fighter_kind == FIGHTER_KIND_SAMUS || opponent_fighter_kind == FIGHTER_KIND_SAMUSD {
        let shot_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT,
        );
        charge_state.int_x(shot_charge)
    }
    // Sheik Needles
    else if opponent_fighter_kind == FIGHTER_KIND_SHEIK {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT,
        );
        charge_state.int_x(my_charge)
    }
    // Mewtwo Shadowball
    else if opponent_fighter_kind == FIGHTER_KIND_MEWTWO {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME,
        );
        let prev_frame = WorkModule::get_int(
            module_accessor,
            *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_PREV_SHADOWBALL_CHARGE_FRAME,
        );
        let ball_had = WorkModule::is_flag(
            module_accessor,
            *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SHADOWBALL_HAD,
        );
        charge_state
            .int_x(my_charge)
            .int_y(prev_frame)
            .has_charge(ball_had)
    }
    // Squirtle Water Gun
    else if opponent_fighter_kind == FIGHTER_KIND_PZENIGAME {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE,
        );
        charge_state.int_x(my_charge)
    }
    // Olimar Pikmin
    else if opponent_fighter_kind == FIGHTER_KIND_PIKMIN {
        let pre_pikmin_variation = WorkModule::get_int(
            module_accessor,
            *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PRE_PIKMIN_VARIATION,
        );
        let before_pre_pikmin_variation = WorkModule::get_int(
            module_accessor,
            *FIGHTER_PIKMIN_INSTANCE_WORK_INT_BEFORE_PRE_PIKMIN_VARIATION,
        );
        charge_state
            .int_x(pre_pikmin_variation)
            .int_y(before_pre_pikmin_variation)
    }
    // Lucario Aura Sphere
    else if opponent_fighter_kind == FIGHTER_KIND_LUCARIO {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME,
        );
        let prev_frame = WorkModule::get_int(
            module_accessor,
            *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME,
        );
        let ball_had = WorkModule::is_flag(
            module_accessor,
            *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD,
        );
        charge_state
            .int_x(my_charge)
            .int_y(prev_frame)
            .has_charge(ball_had)
    }
    // ROB Gyro/Laser/Fuel
    else if opponent_fighter_kind == FIGHTER_KIND_ROBOT {
        let laser_charge = WorkModule::get_float(
            module_accessor,
            *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE,
        );
        charge_state.float_x(laser_charge)
    }
    // Wii Fit Sun Salutation
    else if opponent_fighter_kind == FIGHTER_KIND_WIIFIT {
        let my_charge = WorkModule::get_float(
            module_accessor,
            *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO,
        );
        charge_state.float_x(my_charge)
    }
    // Pac-Man Bonus Fruit
    else if opponent_fighter_kind == FIGHTER_KIND_PACMAN {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK,
        );
        let fruit_have = WorkModule::is_flag(
            module_accessor,
            *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW,
        );
        charge_state.int_x(my_charge).has_charge(fruit_have)
    }
    // Robin Thunder Tome Spells
    else if opponent_fighter_kind == FIGHTER_KIND_REFLET {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND,
        );
        charge_state.int_x(my_charge)
    }
    // Hero (Ka)frizz(le)
    else if opponent_fighter_kind == FIGHTER_KIND_BRAVE {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME,
        );
        charge_state.int_x(my_charge)
    }
    // Sora Spell
    else if opponent_fighter_kind == FIGHTER_KIND_TRAIL {
        let my_charge = WorkModule::get_int(
            module_accessor,
            *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND,
        );
        charge_state.int_x(my_charge)
    }
    // No charge for this character's copy ability
    else {
        charge_state
    }
}

pub unsafe fn handle_kirby_hat_charge(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    opponent_fighter_kind: i32,
    charge: ChargeState,
) {
    // Samus/Dark Samus Charge Shot - 0 to 112
    if opponent_fighter_kind == FIGHTER_KIND_SAMUS || opponent_fighter_kind == FIGHTER_KIND_SAMUSD {
        if let Some(shot_charge) = charge.int_x {
            WorkModule::set_int(
                module_accessor,
                shot_charge,
                *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT,
            );
            if shot_charge == 112 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let samus_cshot_hash = if opponent_fighter_kind == FIGHTER_KIND_SAMUS {
                    Hash40::new("samus_cshot_max")
                } else {
                    Hash40::new("samusd_cshot_max")
                };
                let joint_hash = Hash40::new("handr");
                let pos = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let rot = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let efh = EffectModule::req_follow(
                    module_accessor,
                    samus_cshot_hash,
                    joint_hash,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    0,
                    0,
                    0,
                    false,
                    false,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh as i32,
                    *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX,
                );
            }
        }
    }
    // Sheik Needles - 0 to 6
    else if opponent_fighter_kind == FIGHTER_KIND_SHEIK {
        if let Some(needle_charge) = charge.int_x {
            WorkModule::set_int(
                module_accessor,
                needle_charge,
                *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT,
            );
            if needle_charge == 6 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            }
        }
    }
    // Mewtwo Shadowball - 0 to 120, Boolean
    else if opponent_fighter_kind == FIGHTER_KIND_MEWTWO {
        if let Some(charge_frame) = charge.int_x {
            WorkModule::set_int(
                module_accessor,
                charge_frame,
                *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME,
            );
        }
        if let Some(prev_frame) = charge.int_y {
            WorkModule::set_int(
                module_accessor,
                prev_frame,
                *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_PREV_SHADOWBALL_CHARGE_FRAME,
            );
            if prev_frame == 120 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let pos = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let rot = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let eff_hash = Hash40 { hash: 0x1ac6d446d8 };
                let joint_hash_l = Hash40 { hash: 0x5e008fd84 };
                let efh_l = EffectModule::req_follow(
                    module_accessor,
                    eff_hash,
                    joint_hash_l,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false,
                );
                let joint_hash_r = Hash40 { hash: 0x51a07c0e7 };
                let efh_r = EffectModule::req_follow(
                    module_accessor,
                    eff_hash,
                    joint_hash_r,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh_l as i32,
                    *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_EF_ID_SHADOWBALL_MAX_L,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh_r as i32,
                    *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_EF_ID_SHADOWBALL_MAX_R,
                );
            }
        }
        if let Some(has_shadowball) = charge.has_charge {
            WorkModule::set_flag(
                module_accessor,
                has_shadowball,
                *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SHADOWBALL_HAD,
            );
        }
    }
    // Squirtle Water Gun - 0 to 45
    else if opponent_fighter_kind == FIGHTER_KIND_PZENIGAME {
        if let Some(water_charge) = charge.int_x {
            WorkModule::set_int(
                module_accessor,
                water_charge,
                *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE,
            );
            if water_charge == 45 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            }
        }
    }
    // Olimar Pikmin - 0 to 4
    else if opponent_fighter_kind == FIGHTER_KIND_PIKMIN {
        if let Some(pre) = charge.int_x {
            WorkModule::set_int(
                module_accessor,
                pre,
                *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PRE_PIKMIN_VARIATION,
            );
        }
        if let Some(before_pre) = charge.int_y {
            WorkModule::set_int(
                module_accessor,
                before_pre,
                *FIGHTER_PIKMIN_INSTANCE_WORK_INT_BEFORE_PRE_PIKMIN_VARIATION,
            );
        }
    }
    // Lucario Aura Sphere - 0 to 90
    else if opponent_fighter_kind == FIGHTER_KIND_LUCARIO {
        if let Some(charge_frame) = charge.int_x {
            WorkModule::set_int(
                module_accessor,
                charge_frame,
                *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME,
            );
        }
        if let Some(prev_frame) = charge.int_y {
            WorkModule::set_int(
                module_accessor,
                prev_frame,
                *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME,
            );
            if prev_frame == 90 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let pos = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let rot = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let eff_hash_l = Hash40 { hash: 0x164bf96ca1 };
                let joint_hash_l = Hash40 { hash: 0x5e008fd84 };
                let efh_l = EffectModule::req_follow(
                    module_accessor,
                    eff_hash_l,
                    joint_hash_l,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false,
                );
                let eff_hash_r = Hash40 { hash: 0x16b1f651c2 };
                let joint_hash_r = Hash40 { hash: 0x51a07c0e7 };
                let efh_r = EffectModule::req_follow(
                    module_accessor,
                    eff_hash_r,
                    joint_hash_r,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh_l as i32,
                    *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_L,
                );
                WorkModule::set_int(
                    module_accessor,
                    efh_r as i32,
                    *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_R,
                );
            }
        }
        if let Some(has_shadowball) = charge.has_charge {
            WorkModule::set_flag(
                module_accessor,
                has_shadowball,
                *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD,
            );
        }
    }
    // ROB Laser
    else if opponent_fighter_kind == FIGHTER_KIND_ROBOT {
        if let Some(beam_energy) = charge.float_x {
            WorkModule::set_float(
                module_accessor,
                beam_energy,
                *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE,
            );
        }
    }
    // Wii Fit Sun Salutation - 0 to 1
    else if opponent_fighter_kind == FIGHTER_KIND_WIIFIT {
        if let Some(sun_ratio) = charge.float_x {
            WorkModule::set_float(
                module_accessor,
                sun_ratio,
                *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO,
            )
        }
    }
    // Pac-Man Bonus Fruit - 0 to 12
    else if opponent_fighter_kind == FIGHTER_KIND_PACMAN {
        let mut has_key = false;
        if let Some(charge_rank) = charge.int_x {
            WorkModule::set_int(
                module_accessor,
                charge_rank,
                *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK,
            );

            if charge_rank == 12 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                has_key = true;
            }
        }
        if let Some(has_fruit) = charge.has_charge {
            WorkModule::set_flag(
                module_accessor,
                has_fruit,
                *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW,
            );
            if has_key {
                WorkModule::set_flag(
                    module_accessor,
                    has_key,
                    *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_HAVE_ITEM,
                );
            }
        }
    }
    // Robin Thunder Tome Spells - 0 to 3
    else if opponent_fighter_kind == FIGHTER_KIND_REFLET {
        if let Some(thunder_kind) = charge.int_x {
            WorkModule::set_int(
                module_accessor,
                thunder_kind,
                *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND,
            );
            if thunder_kind == 3 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
                let eff_hash = Hash40 { hash: 0x12db3e4172 };
                let joint_hash = Hash40 { hash: 0x5eb263e0d };
                let pos = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let rot = Vector3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                EffectModule::req_follow(
                    module_accessor,
                    eff_hash,
                    joint_hash,
                    &pos,
                    &rot,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false,
                );
            }
        }
    }
    // Hero (Ka)frizz(le) - 0 to 81
    else if opponent_fighter_kind == FIGHTER_KIND_BRAVE {
        EffectModule::remove_common(module_accessor, Hash40::new("charge_max"));
        WorkModule::off_flag(
            module_accessor,
            *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_EFFECT,
        );
        if let Some(frizz_charge) = charge.int_x {
            WorkModule::set_int(
                module_accessor,
                frizz_charge,
                *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME,
            );
        }
    }
    // Sora Spell - 0 to 2
    else if opponent_fighter_kind == FIGHTER_KIND_TRAIL {
        if let Some(spell_kind) = charge.int_x {
            let prev_spell_kind = (spell_kind + 1) % 3;
            WorkModule::set_int(
                module_accessor,
                prev_spell_kind,
                *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND,
            );
            // app::FighterSpecializer_Trail::change_magic() doesn't actually run if the below flag isn't set
            WorkModule::on_flag(
                module_accessor,
                *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CHANGE_MAGIC,
            );
            if let Some(battle_object) = try_get_battle_object(module_accessor.battle_object_id) {
                let fighter = battle_object as *const app::BattleObject as *mut app::Fighter;
                app::FighterSpecializer_Trail::change_magic(fighter);
            }
        }
    }
}

pub fn init() {
    skyline::install_hooks!(handle_copy_start,);
}
