use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

pub unsafe fn get_charge(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> (f32, f32, f32) {
    // Looks like I'm in the if else dimension again here, since we can't match with these pointers. We could always use the numbers directly and match, up to y'all.
    if fighter_kind == FIGHTER_KIND_MARIO {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // DK Punch, AttackPower thing? Unsure

    if fighter_kind == FIGHTER_KIND_SAMUS || fighter_kind == FIGHTER_KIND_SAMUSD {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT) as f32;
        return (my_charge, -1.0, -1.0);
    }

    if fighter_kind == FIGHTER_KIND_SHEIK {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // Mewtwo Shadowball

    // GnW Bucket

    if fighter_kind == FIGHTER_KIND_GAMEWATCH {
        let my_charge = WorkModule::get_float(module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE);
        let my_attack = WorkModule::get_float(module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK);
        return (my_charge, my_attack, -1.0);
    }

    // Wario Waft
    if fighter_kind == FIGHTER_KIND_WARIO {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL) as f32;
        println!("Waft Get: {}", my_charge);
        return (my_charge, -1.0, -1.0);
    }

    // Squirtle Water Gun

    // Olimar Pikmin .-.

    // Lucario Aura Sphere

    // ROB Gyro/Laser (maybe just Gyro?)

    // Wii Fit Sun Salutation

    if fighter_kind == FIGHTER_KIND_WIIFIT {
        let my_charge = WorkModule::get_float(module_accessor, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO);
        return (my_charge, -1.0, -1.0);
    }

    // Pac-Man Bonus Fruit

    // Robin Thunder Tome Spells

    // Incineroar Revenge

    // Plant Poison

    // Hero (Ka)frizz(le)

    if fighter_kind == FIGHTER_KIND_BRAVE {
        let my_charge = WorkModule::get_int(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME) as f32;
        return (my_charge, -1.0, -1.0);
    }

    // Banjo Wonderwing

    // Steve Tools

    // Sora Spell

    // Mii Gunner Charge Blast

    return (-1.0, -1.0, -1.0);
}

pub unsafe fn handle_charge(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, charge: (f32, f32, f32)) {
    if charge.0 < 0.0 {
        return;
    }

    if fighter_kind == FIGHTER_KIND_MARIO { // 0 to 80, flash
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE)
    }

    // DK Punch, AttackPower thing? Unsure

    if fighter_kind == FIGHTER_KIND_SAMUS || fighter_kind == FIGHTER_KIND_SAMUSD { // 0 to 112, flash, gun sparks
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT)
    }

    if fighter_kind == FIGHTER_KIND_SHEIK { // 0 to 6, flash, needles in hand
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT);
    }

    // Mewtwo Shadowball

    // GnW Bucket

    if fighter_kind == FIGHTER_KIND_GAMEWATCH {
        WorkModule::set_float(module_accessor, charge.0, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE);
        WorkModule::set_float(module_accessor, charge.1, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK);
    }

    // Wario Waft

    if fighter_kind == FIGHTER_KIND_WARIO {
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL); // need other vars
    }

    // Squirtle Water Gun

    // Olimar Pikmin .-.

    // Lucario Aura Sphere

    // ROB Gyro/Laser (maybe just Gyro?)

    // Wii Fit Sun Salutation

    if fighter_kind == FIGHTER_KIND_WIIFIT { // 0 to 1, all effects handled already
        WorkModule::set_float(module_accessor, charge.0, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_LEVEL_RATIO)
    }

    // Pac-Man Bonus Fruit

    // Robin Thunder Tome Spells

    // Incineroar Revenge

    // Plant Poison

    // Hero (Ka)frizz(le)

    if fighter_kind == FIGHTER_KIND_BRAVE { // 0 to 81, flash, fire on hand all handled already
        WorkModule::set_int(module_accessor, charge.0 as i32, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME)
    }

    // Banjo Wonderwing

    // Steve Tools

    // Sora Spell

    // Mii Gunner Charge Blast

    return;
}
