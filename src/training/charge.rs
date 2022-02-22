use smash::app::{self, lua_bind::*, ArticleOperationTarget, FighterUtil, FighterFacial};
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};

#[derive(Copy, Clone)]
pub struct ChargeState {
    pub int_x: Option<i32>,
    pub int_y: Option<i32>,
    pub float_x: Option<f32>,
    pub float_y: Option<f32>,
    pub float_z: Option<f32>,
    pub has_charge: Option<bool>
}

impl ChargeState {
    fn int_x(mut self, int_x: i32) -> Self {
        self.int_x = Some(int_x);
        self
    }

    fn int_y(mut self, int_y: i32) -> Self {
        self.int_y = Some(int_y);
        self
    }

    fn float_x(mut self, float_x: f32) -> Self {
        self.float_x = Some(float_x);
        self
    }

    fn float_y(mut self, float_y: f32) -> Self {
        self.float_y = Some(float_y);
        self
    }

    fn float_z(mut self, float_z: f32) -> Self {
        self.float_z = Some(float_z);
        self
    }

    fn has_charge(mut self, has_charge: bool) -> Self {
        self.has_charge = Some(has_charge);
        self
    }

}

impl Default for ChargeState {
    fn default() -> Self {
        Self {
            int_x: None,
            int_y: None,
            float_x: None,
            float_y: None,
            float_z: None,
            has_charge: None
        }
    }
}

pub unsafe fn get_charge(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> ChargeState { // ADD ELSES
    // Mario FLUDD
    if fighter_kind == FIGHTER_KIND_MARIO {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
        return ChargeState::default().int_x(my_charge); // why can't I do the no semicolon return here?
    }

    // Donkey Kong Giant Punch
    if fighter_kind == FIGHTER_KIND_DONKEY {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        return ChargeState::default().int_x(my_charge);
    }

    // Samus/Dark Samus Charge Shot
    if fighter_kind == FIGHTER_KIND_SAMUS || fighter_kind == FIGHTER_KIND_SAMUSD {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        return ChargeState::default().int_x(my_charge);
    }

    // Sheik Needles
    if fighter_kind == FIGHTER_KIND_SHEIK {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT);
        return ChargeState::default().int_x(my_charge);
    }

    // Mewtwo Shadowball
    if fighter_kind == FIGHTER_KIND_MEWTWO {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME);
        let prev_frame = WorkModule::get_int(module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_PREV_SHADOWBALL_CHARGE_FRAME);
        let ball_had = WorkModule::is_flag(module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SHADOWBALL_HAD);
        return ChargeState::default().int_x(my_charge).int_y(prev_frame).has_charge(ball_had);
    }

    // Game and Watch Bucket
    if fighter_kind == FIGHTER_KIND_GAMEWATCH {
        let my_charge = WorkModule::get_float(module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE);
        let my_attack = WorkModule::get_float(module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK);
        return ChargeState::default().float_x(my_charge).float_y(my_attack);
    }

    // Wario Waft
    if fighter_kind == FIGHTER_KIND_WARIO {
        let my_charge = WorkModule::get_int(module_accessor, 0x100000BF); // DOCUMENT THISSSSSSSSSSSSS
        return ChargeState::default().int_x(my_charge);
    }

    // Squirtle Water Gun
    if fighter_kind == FIGHTER_KIND_PZENIGAME {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE);
        return ChargeState::default().int_x(my_charge);
    }

    // Lucario Aura Sphere
    if fighter_kind == FIGHTER_KIND_LUCARIO {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
        let prev_frame = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME);
        let ball_had = WorkModule::is_flag(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD);
        return ChargeState::default().int_x(my_charge).int_y(prev_frame).has_charge(ball_had);
    }

    // ROB Gyro/Laser/Fuel
    if fighter_kind == FIGHTER_KIND_ROBOT {
        let laser_charge = WorkModule::get_float(module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE);
        let gyro_charge = WorkModule::get_float(module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_GYRO_CHARGE_VALUE);
        let fuel_charge = WorkModule::get_float(module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
        return ChargeState::default().float_x(laser_charge).float_y(gyro_charge).float_z(fuel_charge);
    }

    // Wii Fit Sun Salutation
    if fighter_kind == FIGHTER_KIND_WIIFIT {
        let my_charge = WorkModule::get_float(module_accessor, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO);
        return ChargeState::default().float_x(my_charge);
    }

    // Pac-Man Bonus Fruit
    if fighter_kind == FIGHTER_KIND_PACMAN {
        let my_charge = WorkModule::get_int(module_accessor, 0x100000C1); // FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK
        let max_have = WorkModule::is_flag(module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_HAVE_ITEM);
        return ChargeState::default().int_x(my_charge).has_charge(max_have);
    }

    // Robin Thunder Tome Spells
    if fighter_kind == FIGHTER_KIND_REFLET {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
        return ChargeState::default().int_x(my_charge);
    }

    // Plant Poison Breath
    if fighter_kind == FIGHTER_KIND_PACKUN {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT);
        return ChargeState::default().int_x(my_charge);
    }

    // Hero (Ka)frizz(le)
    if fighter_kind == FIGHTER_KIND_BRAVE {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
        return ChargeState::default().int_x(my_charge);
    }

    // Banjo Wonderwing
    if fighter_kind == FIGHTER_KIND_BUDDY {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
        return ChargeState::default().int_x(my_charge);
    }

    // Mii Gunner Charge Blast
    if fighter_kind == FIGHTER_KIND_MIIGUNNER {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
        return ChargeState::default().int_x(my_charge);
    }

    return ChargeState::default();
}

pub unsafe fn handle_charge(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, charge: ChargeState) {
    // Don't bother checking the fighter type if we don't have a charge stored
    //if charge == (-1.0, -1.0, -1.0) {
       // return;
    //}

    // Mario Fludd - 0 to 80
    if fighter_kind == FIGHTER_KIND_MARIO {
        if let Some(fludd_charge) = charge.int_x {
            WorkModule::set_int(module_accessor, fludd_charge, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
            if fludd_charge == 80 {
                EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            }
        }
    }
    /*
    // DK Punch - 0 to 110
    else if fighter_kind == FIGHTER_KIND_DONKEY {
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        if charge.0 as i32 == 110 {
            FighterUtil::set_face_motion_by_priority(module_accessor,FighterFacial(*FIGHTER_FACIAL_SPECIAL), Hash40::new("special_n_max_face"));
        }
    }

    // Samus/Dark Samus Charge Shot - 0 to 112
    else if fighter_kind == FIGHTER_KIND_SAMUS || fighter_kind == FIGHTER_KIND_SAMUSD {
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        if charge.0 as i32 == 112 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            let samus_cshot_hash;
            if fighter_kind == FIGHTER_KIND_SAMUS {
                samus_cshot_hash = Hash40::new("samus_cshot_max");
            } else {
                samus_cshot_hash = Hash40::new("samusd_cshot_max");
            }
            let joint_hash = Hash40::new("armr");
            let pos = Vector3f {
                x: 7.98004,
                y: -0.50584,
                z: -0.25092,
            };
            let rot = Vector3f {
                x: -91.2728,
                y: -1.7974,
                z: 176.373,
            };
            let efh = EffectModule::req_follow(
                module_accessor, samus_cshot_hash, 
                joint_hash, &pos, &rot, 1.0, false,
                0, 0, 0, 0, 0,
                false, false
            );
            WorkModule::set_int(module_accessor, efh as i32, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX);
        }
    }

    // Sheik Needles - 0 to 6
    else if fighter_kind == FIGHTER_KIND_SHEIK {
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT);
        ArticleModule::generate_article_enable(module_accessor,*FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE, false, -1);
        let hash_main = Hash40::new("set_main");
        if charge.0 == 6.0 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            ArticleModule::set_visibility(module_accessor,*FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                hash_main, Hash40::new("group_main_default"),ArticleOperationTarget(0));
        } else if charge.0 == 5.0 {
            ArticleModule::set_visibility(module_accessor,*FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                hash_main, Hash40::new("group_main_5"),ArticleOperationTarget(0));
        } else if charge.0 == 4.0 {
            ArticleModule::set_visibility(module_accessor,*FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                hash_main, Hash40::new("group_main_4"),ArticleOperationTarget(0));
        } else if charge.0 == 3.0 {
            ArticleModule::set_visibility(module_accessor,*FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                hash_main, Hash40::new("group_main_3"),ArticleOperationTarget(0));
        } else if charge.0 == 2.0 {
            ArticleModule::set_visibility(module_accessor,*FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                hash_main, Hash40::new("group_main_2"),ArticleOperationTarget(0));
        } else if charge.0 == 1.0 {
            ArticleModule::set_visibility(module_accessor,*FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                hash_main, Hash40::new("group_main_1"),ArticleOperationTarget(0));
        } else if charge.0 == 0.0 {
            ArticleModule::set_visibility(module_accessor,*FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE,
                hash_main, Hash40::new("group_main_0"),ArticleOperationTarget(0));
        }
    }

    // Mewtwo Shadowball - 0 to 120, Boolean
    else if fighter_kind == FIGHTER_KIND_MEWTWO {
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME);
        WorkModule::set_int(module_accessor, charge.1 as i32, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_PREV_SHADOWBALL_CHARGE_FRAME);
        if charge.2 > 0.0 {
            WorkModule::on_flag(module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SHADOWBALL_HAD);
        }

        if charge.1 == 120.0 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            let effect_hash = Hash40::new("mewtwo_shadowball_max_hand");
            let joint_hash_1 = Hash40::new("handl");
            let joint_hash_2 = Hash40::new("handr");
            let pos = Vector3f {
                x: 1.0,
                y: 0.5,
                z: 0.0,
            };
            let rot = Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let efh_1 = EffectModule::req_follow(
                module_accessor, effect_hash, 
                joint_hash_1, &pos, &rot, 1.0, false,
                0, 0, -1, 0, 0,
                false, false
            );
            let efh_2 = EffectModule::req_follow(
                module_accessor, effect_hash, 
                joint_hash_2, &pos, &rot, 1.0, false,
                0, 0, -1, 0, 0,
                false, false
            );
            WorkModule::set_int(module_accessor, efh_1 as i32, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_EF_ID_SHADOWBALL_MAX_L);
            WorkModule::set_int(module_accessor, efh_2 as i32, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_EF_ID_SHADOWBALL_MAX_R);
        }
    }

    // GnW Bucket - 0 to 3, Attack not tested
    else if fighter_kind == FIGHTER_KIND_GAMEWATCH {
        WorkModule::set_float(module_accessor, charge.0, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE);
        WorkModule::set_float(module_accessor, charge.1, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK);
        if charge.0 == 3.0 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
        } else {
            // GnW flashes when successfully bucketing, and it will persist if state is loaded during that time, so we remove it here
            EffectModule::remove_common(module_accessor, Hash40::new("charge_max"));
        }
    }

    // Wario Waft - 0 to 6000
    else if fighter_kind == FIGHTER_KIND_WARIO {
        WorkModule::set_int(module_accessor, charge.0 as i32, 0x100000BF); // FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_COUNT
    }

    // Squirtle Water Gun - 0 to 45
    else if fighter_kind == FIGHTER_KIND_PZENIGAME {
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE);
        if charge.0 as i32 == 45 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
        }
    }

    // Lucario Aura Sphere - 0 to 90, Boolean
    else if fighter_kind == FIGHTER_KIND_LUCARIO {
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
        WorkModule::set_int(module_accessor, charge.1 as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME);
        if charge.2 > 0.0 {
            WorkModule::on_flag(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD);
        }

        if charge.1 == 90.0 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            let effect_hash_1 = Hash40::new("lucario_hadoudan_max_l");
            let effect_hash_2 = Hash40::new("lucario_hadoudan_max_r");
            let joint_hash_1 = Hash40::new("handl");
            let joint_hash_2 = Hash40::new("handr");
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
            let efh_1 = EffectModule::req_follow(
                module_accessor, effect_hash_1, 
                joint_hash_1, &pos, &rot, 1.0, false,
                0, 0, -1, 0, 0,
                false, false
            );
            let efh_2 = EffectModule::req_follow(
                module_accessor, effect_hash_2, 
                joint_hash_2, &pos, &rot, 1.0, false,
                0, 0, -1, 0, 0,
                false, false
            );
            WorkModule::set_int(module_accessor, efh_1 as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_L);
            WorkModule::set_int(module_accessor, efh_2 as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_R);
        }
    }

    // ROB Gyro/Laser/Fuel - Gyro from 0 to 90, rest unchecked
    else if fighter_kind == FIGHTER_KIND_ROBOT {
        WorkModule::set_float(module_accessor, charge.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE);
        WorkModule::set_float(module_accessor, charge.1, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_GYRO_CHARGE_VALUE);
        WorkModule::set_float(module_accessor, charge.2, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);

        if charge.1 as i32 == 90 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
        }
    }

    // Wii Fit Sun Salutation - 0 to 1
    else if fighter_kind == FIGHTER_KIND_WIIFIT {
        WorkModule::set_float(module_accessor, charge.0, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO)
    }

    // Pac-Man Bonus Fruit - 0 to 12
    else if fighter_kind == FIGHTER_KIND_PACMAN {
        WorkModule::set_int(module_accessor, charge.0 as i32, 0x100000C1); // FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK
        if charge.1 > 0.0 {
            WorkModule::on_flag(module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_HAVE_ITEM);
        }
        if charge.0 as i32 == 12 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
        }
    }

    // Robin Thunder Tome Spells - 0 to 3
    else if fighter_kind == FIGHTER_KIND_REFLET {
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
        if charge.0 as i32 == 3 { 
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            let reflet_hash = Hash40::new("reflet_thunder_max");            
            let joint_hash = Hash40::new("handl");
            let pos = Vector3f {
                x: 1.0,
                y: 2.0,
                z: 0.0,
            };
            let rot = Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            EffectModule::req_follow(
                module_accessor, reflet_hash, 
                joint_hash, &pos, &rot, 1.0, false,
                0, 0, -1, 0, 0,
                false, false
            );
        }
    }

    // Mii Gunner Charge Blast - 0 to 120
    else if fighter_kind == FIGHTER_KIND_MIIGUNNER {
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
        if charge.0 as i32 == 120 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            let gunner_hash = Hash40::new("miigunner_cshot_max");
            let joint_hash = Hash40::new("armr");
            let pos = Vector3f {
                x: 6.0,
                y: 0.0,
                z: 0.0,
            };
            let rot = Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let efh = EffectModule::req_follow(
                module_accessor, gunner_hash, 
                joint_hash, &pos, &rot, 1.0, false,
                0, 0, 0, 0, 0,
                false, false
            );
            WorkModule::set_int(module_accessor, efh as i32, *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX);
        }
    }

    // Plant Poison - 0 to 75
    else if fighter_kind == FIGHTER_KIND_PACKUN {
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT);
        if charge.0 as i32 == 75 { 
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            let plant_hash = Hash40::new("packun_poison_max_smoke");            
            let joint_hash = Hash40::new("hip");
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
                module_accessor, plant_hash, 
                joint_hash, &pos, &rot, 1.0, false,
                32768, 0, -1, 0, 0,
                false, false
            );
            WorkModule::set_int(module_accessor, efh as i32, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_CHARGE_MAX_EFFECT_HANDLE);
        }
    }

    // Hero (Ka)frizz(le) - 0 to 81
    else if fighter_kind == FIGHTER_KIND_BRAVE {
        WorkModule::off_flag(module_accessor, 0x200000E8); // FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_EFFECT
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
    }

    // Banjo Wonderwing - 0 to 5
    else if fighter_kind == FIGHTER_KIND_BUDDY {
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
    }
    */
    return;
}
