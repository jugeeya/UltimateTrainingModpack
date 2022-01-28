use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

pub unsafe fn get_charge(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> (f32, f32, f32) {
    // Looks like I'm in the if else dimension again here, since we can't match with these pointers. We could always use the numbers directly and match, up to y'all.
    // TODO: add elses so we spend less time in this function
    
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

    // Olimar Pikmin .-.

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
        println!("Rank: {}, max have: {}, max eff: {}",my_charge,max_have,max_effect);
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

    // Sora Spell

    /*if fighter_kind == FIGHTER_KIND_TRAIL {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) as f32;
        return (my_charge, -1.0, -1.0);
    }*/

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
    }

    // DK Punch, AttackPower thing? Unsure

    if fighter_kind == FIGHTER_KIND_DONKEY { // ? to ?, flash handled, need to do angry expression
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
    }

    // Samus (D) Charge Shot

    if fighter_kind == FIGHTER_KIND_SAMUS || fighter_kind == FIGHTER_KIND_SAMUSD { // 0 to 112, flash, gun sparks
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
    }

    // Sheik Needles

    if fighter_kind == FIGHTER_KIND_SHEIK { // 0 to 6, flash, needles in hand
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT);
    }

    // Mewtwo Shadowball

    if fighter_kind == FIGHTER_KIND_MEWTWO { // 0 to 120, 0 to 120, true/false. Flash, hand aura
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME);
        WorkModule::set_int(module_accessor, charge.1 as i32, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_PREV_SHADOWBALL_CHARGE_FRAME);
        if charge.2 > 0.0 {
            WorkModule::on_flag(module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SHADOWBALL_HAD);
        }
    }

    // GnW Bucket

    if fighter_kind == FIGHTER_KIND_GAMEWATCH { // 0 to 3, unk
        WorkModule::set_float(module_accessor, charge.0, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE);
        WorkModule::set_float(module_accessor, charge.1, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK);
    }

    // Wario Waft

    if fighter_kind == FIGHTER_KIND_WARIO { // 0 to 6600. Frames?
        WorkModule::set_int(module_accessor, charge.0 as i32, 0x100000BF);
    }

    // Squirtle Water Gun

    if fighter_kind == FIGHTER_KIND_PZENIGAME { // 0 to ?, flash
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE)
    }

    // Olimar Pikmin .-.

    // Lucario Aura Sphere

    if fighter_kind == FIGHTER_KIND_LUCARIO { // ?, ?, true/false. Flash, hand aura. I think you can use Mewtwo's vars and it still works, changed anyway tho
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
        WorkModule::set_int(module_accessor, charge.1 as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME);
        if charge.2 > 0.0 {
            WorkModule::on_flag(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD);
        }
    }

    // ROB Gyro/Laser/Fuel

    if fighter_kind == FIGHTER_KIND_ROBOT { // all ?, didn't check
        WorkModule::set_float(module_accessor, charge.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE);
        WorkModule::set_float(module_accessor, charge.1, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_GYRO_CHARGE_VALUE);
        WorkModule::set_float(module_accessor, charge.2, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
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
        if charge.2 > 0.0 {
            WorkModule::on_flag(module_accessor, 0x200000E3);
        }
    }

    // Robin Thunder Tome Spells

    if fighter_kind == FIGHTER_KIND_REFLET { // ? to ?, flash effect and hand lightning
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
    }

    // Incineroar Revenge

    if fighter_kind == FIGHTER_KIND_GAOGAEN { // WIP
        //WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_ATTACK_HIT_REVENGE_ATTACK_NO);
        //WorkModule::set_float(module_accessor, charge.0, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_TOTAL_DAMAGE);
        WorkModule::set_float(module_accessor, charge.0, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE);

        if charge.1 > 0.0 {
            WorkModule::on_flag(module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE);
        }
    }

    // Plant Poison

    if fighter_kind == FIGHTER_KIND_PACKUN { // ? to ? (didn't check), just flashing?
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT);
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
        println!("Setting! Sword: {}, Axe: {}, Pick: {}", new_sword_mat, new_axe_mat, new_pick_mat);
    }

    // Sora Spell

    /*if fighter_kind == FIGHTER_KIND_TRAIL { // 
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND)
    }*/

    // Mii Gunner Charge Blast

    if fighter_kind == FIGHTER_KIND_MIIGUNNER { // 0 to ?? Flash, Gun sparks
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT)
    }

    return;
}
