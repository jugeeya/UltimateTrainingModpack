use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};

pub unsafe fn get_charge(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> (f32, f32, f32) {
    // Looks like I'm in the if else dimension again here, since we can't match with these pointers. We could always use the numbers directly and match, up to y'all.
    // TODO: add elses so we spend less time in this function
    // Create all the hashes on startup so we can just call them later and save time?
    // Make a function to handle effects?

    // Mario FLUDD
    
    if fighter_kind == FIGHTER_KIND_MARIO {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // DK Punch, AttackPower thing? Unsure

    if fighter_kind == FIGHTER_KIND_DONKEY {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // Samus/Dark Samus Charge Shot

    if fighter_kind == FIGHTER_KIND_SAMUS || fighter_kind == FIGHTER_KIND_SAMUSD {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // Sheik Needles

    if fighter_kind == FIGHTER_KIND_SHEIK {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // Mewtwo Shadowball

    if fighter_kind == FIGHTER_KIND_MEWTWO {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) as f32;
        let prev_frame = WorkModule::get_int(module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_PREV_SHADOWBALL_CHARGE_FRAME) as f32;
        let ball_had = WorkModule::is_flag(module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SHADOWBALL_HAD);
        if ball_had {
            return (my_charge, prev_frame, 1.0);
        }
        return (my_charge, prev_frame, -1.0);
    }

    // GnW Bucket

    if fighter_kind == FIGHTER_KIND_GAMEWATCH {
        let my_charge = WorkModule::get_float(module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE);
        let my_attack = WorkModule::get_float(module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK);
        return (my_charge, my_attack, -1.0);
    }

    // Wario Waft
    if fighter_kind == FIGHTER_KIND_WARIO {
        let my_charge = WorkModule::get_int(module_accessor, 0x100000BF) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // Squirtle Water Gun

    if fighter_kind == FIGHTER_KIND_PZENIGAME {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // Lucario Aura Sphere

    if fighter_kind == FIGHTER_KIND_LUCARIO {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME) as f32;
        let prev_frame = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME) as f32;
        let ball_had = WorkModule::is_flag(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD);
        if ball_had {
            return (my_charge, prev_frame, 1.0);
        }
        return (my_charge, prev_frame, -1.0);
    }

    // ROB Gyro/Laser/Fuel

    if fighter_kind == FIGHTER_KIND_ROBOT {
        let laser_charge = WorkModule::get_float(module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE);
        let gyro_charge = WorkModule::get_float(module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_GYRO_CHARGE_VALUE);
        let fuel_charge = WorkModule::get_float(module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
        return (laser_charge, gyro_charge, fuel_charge);
    }

    // Wii Fit Sun Salutation

    if fighter_kind == FIGHTER_KIND_WIIFIT {
        let my_charge = WorkModule::get_float(module_accessor, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO);
        return (my_charge, -1.0, -1.0);
    }

    // Pac-Man Bonus Fruit

    if fighter_kind == FIGHTER_KIND_PACMAN {
        let my_charge = WorkModule::get_int(module_accessor, 0x100000C1) as f32;
        let max_have = WorkModule::is_flag(module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_HAVE_ITEM);
        let max_effect = WorkModule::is_flag(module_accessor, 0x200000E3);
        if max_have && max_effect {
            return (my_charge, 1.0, 1.0);
        }
        if max_have {
            return (my_charge, 1.0, -1.0);
        }
        if max_effect {
            return (my_charge, -1.0, 1.0);
        }
        return (my_charge, -1.0, -1.0);
    }

    // Robin Thunder Tome Spells

    if fighter_kind == FIGHTER_KIND_REFLET {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // Incineroar Revenge

    if fighter_kind == FIGHTER_KIND_GAOGAEN {
        let attack_no = WorkModule::get_int(module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_ATTACK_HIT_REVENGE_ATTACK_NO) as f32;
        let revenge_damage = WorkModule::get_float(module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_TOTAL_DAMAGE);
        let revenge_rate = WorkModule::get_float(module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE);
        let is_revenge = WorkModule::is_flag(module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE);
        println!("Attack No: {}, Revenge Damage: {}, Rate: {}, Is Rev: {}",attack_no,revenge_damage,revenge_rate,is_revenge);
        if is_revenge {
            return (revenge_rate, 1.0, -1.0);
        }
        return (revenge_rate, -1.0, -1.0);
    }

    // Plant Poison

    if fighter_kind == FIGHTER_KIND_PACKUN {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // Hero (Ka)frizz(le)

    if fighter_kind == FIGHTER_KIND_BRAVE {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // Banjo Wonderwing

    if fighter_kind == FIGHTER_KIND_BUDDY {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // Steve Tools

    if fighter_kind == FIGHTER_KIND_PICKEL {
        let extend_buffer = WorkModule::get_int64(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_EXTEND_BUFFER);
        //let buffer_pointer = extend_buffer;
        let sword_mat: char = *(extend_buffer as *const char);
        let axe_mat: char = *((extend_buffer + 0xC) as *const char);
        let pick_mat: char = *((extend_buffer + 0xC + 0xC) as *const char);
        println!("Sword: {}, Axe: {}, Pick: {}", sword_mat, axe_mat, pick_mat);
        return (sword_mat as i32 as f32, axe_mat as i32 as f32, pick_mat as i32 as f32);
    }

    // Mii Gunner Charge Blast

    if fighter_kind == FIGHTER_KIND_MIIGUNNER {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT) as f32;
        return (my_charge, -1.0, -1.0);
    }

    return (-1.0, -1.0, -1.0);
}

pub unsafe fn handle_charge(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, charge: (f32, f32, f32)) { // add return;s?
    if charge.0 < 0.0 {
        return;
    }

    // Mario Fludd

    if fighter_kind == FIGHTER_KIND_MARIO { // 0 to 80, flash
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
        if charge.0 as i32 == 80 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
        }
    }

    // DK Punch

    if fighter_kind == FIGHTER_KIND_DONKEY { // ? to ?, flash handled, need to do angry expression
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        /*if charge.0 as i32 == 110 {
            // This prevents the flash and smoke from happening
            WorkModule::on_flag(module_accessor,*FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_N_EFFECT);
        }*/
    }

    // Samus (D) Charge Shot

    if fighter_kind == FIGHTER_KIND_SAMUS || fighter_kind == FIGHTER_KIND_SAMUSD { // 0 to 112, flash, gun sparks
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

    // Sheik Needles

    if fighter_kind == FIGHTER_KIND_SHEIK { // 0 to 6, flash, needles in hand
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT);
        if charge.0 as i32 == 6 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
        }
    }

    // Mewtwo Shadowball

    if fighter_kind == FIGHTER_KIND_MEWTWO { // 0 to 120, 0 to 120, true/false. Flash, hand aura
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

    // GnW Bucket

    if fighter_kind == FIGHTER_KIND_GAMEWATCH { // 0 to 3, unk
        WorkModule::set_float(module_accessor, charge.0, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE);
        WorkModule::set_float(module_accessor, charge.1, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK);
        if charge.0 == 3.0 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
        } else {
            // GnW flashes when successfully bucketing, and it will persist if state is loaded during that time, so we remove it here
            EffectModule::remove_common(module_accessor, Hash40::new("charge_max"));
        }
    }

    // Wario Waft

    if fighter_kind == FIGHTER_KIND_WARIO { // 0 to 6600. Frames?
        WorkModule::set_int(module_accessor, charge.0 as i32, 0x100000BF);
    }

    // Squirtle Water Gun

    if fighter_kind == FIGHTER_KIND_PZENIGAME { // 0 to ?, flash
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE);
        if charge.0 as i32 == 45 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
        }
    }

    // Lucario Aura Sphere

    if fighter_kind == FIGHTER_KIND_LUCARIO { // ?, ?, true/false. Flash, hand aura. I think you can use Mewtwo's vars and it still works, changed anyway tho
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
        WorkModule::set_int(module_accessor, charge.1 as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME);
        if charge.2 > 0.0 {
            WorkModule::on_flag(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD);
        }

        if charge.1 == 90.0 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            
            let effect_hash_1 = Hash40::new("lucario_hadoudan_max_l");
            let effect_hash_2 = Hash40::new("lucario_hadoudan_max_r");
            //let effect_hash_2 = Hash40::new("lucario_hadoudan_max_hold");

            let joint_hash_1 = Hash40::new("handl");
            let joint_hash_2 = Hash40::new("handr");
            //let joint_hash_3 = Hash40::new("top");
            
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
            /*let efh_3 = EffectModule::req_follow(
                module_accessor, effect_hash, 
                joint_hash_2, &pos, &rot, 1.0, false,
                49512, 0, -1, 0, 0,
                false, false
            );*/
            WorkModule::set_int(module_accessor, efh_1 as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_L);
            WorkModule::set_int(module_accessor, efh_2 as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_R);
        }
    }

    // ROB Gyro/Laser/Fuel

    if fighter_kind == FIGHTER_KIND_ROBOT { // all ?, didn't check
        WorkModule::set_float(module_accessor, charge.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE);
        WorkModule::set_float(module_accessor, charge.1, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_GYRO_CHARGE_VALUE);
        WorkModule::set_float(module_accessor, charge.2, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);

        if charge.1 as i32 == 90 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
        }
    }

    // Wii Fit Sun Salutation

    if fighter_kind == FIGHTER_KIND_WIIFIT { // 0 to 1, all effects handled already
        WorkModule::set_float(module_accessor, charge.0, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO)
    }

    // Pac-Man Bonus Fruit

    if fighter_kind == FIGHTER_KIND_PACMAN { // 0 to 12, this only keeps the flash once so still needs more work
        WorkModule::set_int(module_accessor, charge.0 as i32, 0x100000C1);
        if charge.1 > 0.0 {
            WorkModule::on_flag(module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_HAVE_ITEM);
        }
        if charge.0 as i32 == 12 {
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
        }
    }

    // Robin Thunder Tome Spells

    if fighter_kind == FIGHTER_KIND_REFLET { // 0 to 3, flash effect and hand lightning
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

    // Plant Poison

    if fighter_kind == FIGHTER_KIND_PACKUN { // 0 to 75 (didn't check), just flashing?  
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

    // Hero (Ka)frizz(le)

    if fighter_kind == FIGHTER_KIND_BRAVE { // 0 to 81, flash, fire on hand all handled already
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME)
    }

    // Banjo Wonderwing

    if fighter_kind == FIGHTER_KIND_BUDDY {
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN)
    }

    // Steve Tools

    if fighter_kind == FIGHTER_KIND_PICKEL {
        let extend_buffer = WorkModule::get_int64(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_EXTEND_BUFFER);
        
        let new_sword_mat = charge.0 as u8 as char;
        let new_axe_mat = charge.1 as u8 as char;
        let new_pick_mat = charge.2 as u8 as char;


        *(extend_buffer as *mut char) = new_sword_mat;
        *((extend_buffer + 0xC) as *mut char) = new_axe_mat;
        *((extend_buffer + 0xC + 0xC) as *mut char) = new_pick_mat;
    }

    // Mii Gunner Charge Blast

    if fighter_kind == FIGHTER_KIND_MIIGUNNER { // 0 to 120 Flash, Gun sparks
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
        if charge.0 as i32 == 120 { // check
            EffectModule::req_common(module_accessor, Hash40::new("charge_max"), 0.0);
            
            let gunner_hash = Hash40::new("miigunner_cshot_max");
            
            let joint_hash = Hash40::new("armr"); // ? could be incorrect
            
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

    return;
}
