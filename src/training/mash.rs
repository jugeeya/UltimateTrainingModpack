use crate::common::consts::*;
use crate::common::*;
use crate::training::fast_fall;
use crate::training::shield;
use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;

static mut CURRENT_AERIAL: Action = Action::Nair;
static mut QUEUE: Vec<Action> = vec![];

pub fn buffer_action(action: Action) {
    unsafe {
        if QUEUE.len() > 0 {
            return;
        }
    }

    unsafe {
        QUEUE.insert(0, action);

        if shield::is_aerial(action) {
            set_aerial(action);
        }

        buffer_follow_up();
    }
}

pub fn buffer_follow_up() {
    let action;

    unsafe {
        action = MENU.follow_up;
    }

    if action == Action::Nothing {
        return;
    }

    unsafe {
        QUEUE.insert(0, action);
    }
}

pub fn get_current_buffer() -> Action {
    unsafe {
        let current = QUEUE.last().unwrap_or(&Action::Nothing);
        *current
    }
}

pub fn reset() {
    unsafe {
        QUEUE.pop();
    }

    shield::suspend_shield(get_current_buffer());
}

pub fn set_aerial(attack: Action) {
    if !shield::is_aerial(attack) {
        return;
    }

    unsafe {
        CURRENT_AERIAL = attack;
        println!("Setting Attack {}", attack as i32);
    }
}

pub unsafe fn get_attack_air_kind(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> Option<i32> {
    if !is_training_mode() {
        return None;
    }

    if !is_operation_cpu(module_accessor) {
        return None;
    }

    CURRENT_AERIAL.into_attack_air_kind()
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
) -> i32 {
    // Only do once per frame
    if category != FIGHTER_PAD_COMMAND_CATEGORY1 {
        return 0;
    }

    if !is_training_mode() {
        return 0;
    }

    if !is_operation_cpu(module_accessor) {
        return 0;
    }

    check_buffer(module_accessor);

    perform_action(module_accessor)
}

unsafe fn check_buffer(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if QUEUE.len() > 0 {
        return;
    }

    if !is_in_hitstun(module_accessor) && MENU.mash_in_neutral != OnOff::On {
        return;
    }

    if MENU.mash_state == Mash::Random {
        let action = get_random_action(module_accessor);
        buffer_action(action);
        return;
    }

    let action = mash_to_action(MENU.mash_state);
    buffer_action(action);
}

// Temp Translation
pub fn mash_to_action(mash: Mash) -> Action {
    use Action::*;
    match mash {
        Mash::Airdodge => Airdodge,
        Mash::Jump => Jump,
        Mash::Spotdodge => Spotdodge,
        Mash::RollForward => RollForward,
        Mash::RollBack => RollBack,
        Mash::Shield => Shield,
        Mash::Attack => unsafe { attack_to_action(MENU.mash_attack_state) },
        _ => Nothing,
    }
}

pub fn get_random_action(module_accessor: &mut app::BattleObjectModuleAccessor) -> Action {
    let mut random_cmds = vec![Mash::Jump, Mash::Attack];
    unsafe {
        if is_airborne(module_accessor) {
            random_cmds.push(Mash::Airdodge);
        }

        if is_grounded(module_accessor) {
            random_cmds.push(Mash::RollBack);
            random_cmds.push(Mash::RollForward);
            random_cmds.push(Mash::Spotdodge);
        }

        let random_cmd_index =
            app::sv_math::rand(hash40("fighter"), random_cmds.len() as i32) as usize;

        mash_to_action(random_cmds[random_cmd_index])
    }
}

fn attack_to_action(attack: Attack) -> Action {
    use Action::*;
    match attack {
        Attack::Nair => Nair,
        Attack::Fair => Fair,
        Attack::Bair => Bair,
        Attack::UpAir => UpAir,
        Attack::Dair => Dair,
        Attack::NeutralB => NeutralB,
        Attack::SideB => SideB,
        Attack::UpB => UpB,
        Attack::DownB => DownB,
        Attack::UpSmash => UpSmash,
        Attack::FSmash => FSmash,
        Attack::DSmash => DSmash,
        Attack::Grab => Grab,
        Attack::Jab => Jab,
        Attack::Ftilt => Ftilt,
        Attack::Utilt => Utilt,
        Attack::Dtilt => Dtilt,
        Attack::Nothing => Nothing,
    }
}

unsafe fn perform_action(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    use Action::*;

    let action = get_current_buffer();

    match action {
        Airdodge => {
            // Shield if grounded instead
            if is_grounded(module_accessor) {
                reset();
                buffer_action(Shield);
                return 0;
            }

            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_KIND_ESCAPE_AIR,
                *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE,
            );
        }
        Jump => {
            return update_jump_flag(module_accessor);
        }
        Spotdodge => {
            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_KIND_ESCAPE,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE,
            );
        }
        RollForward => {
            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_KIND_ESCAPE_F,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F,
            );
        }
        RollBack => {
            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_KIND_ESCAPE_B,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B,
            );
        }
        Shield => {
            /*
            Doesn't actually cause the shield, but will clear the buffer once shield is possible.
            Shield hold is performed trough shield::should_hold_shield
            */
            // return get_flag(
            //     module_accessor,
            //     *FIGHTER_STATUS_KIND_GUARD_ON,
            //     *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE,
            // );
            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_KIND_GUARD_ON,
                *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE,
            );
        }
        _ => return get_attack_flag(module_accessor, action),
    }
}

unsafe fn update_jump_flag(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    let check_flag: i32;

    if is_grounded(module_accessor) {
        check_flag = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
    } else if is_airborne(module_accessor) {
        check_flag = *FIGHTER_STATUS_KIND_JUMP_AERIAL;
    } else {
        check_flag = *FIGHTER_STATUS_KIND_JUMP;
    }

    return get_flag(
        module_accessor,
        check_flag,
        *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON,
    );
}

unsafe fn get_attack_flag(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    action: Action,
) -> i32 {
    use Action::*;

    let action_flag: i32;
    let status: i32;

    match action {
        Nair | Fair | Bair | UpAir | Dair => {
            return get_aerial_flag(module_accessor, action);
        }
        NeutralB => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N;
            status = *FIGHTER_STATUS_KIND_SPECIAL_N;
        }
        SideB => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S;
            status = *FIGHTER_STATUS_KIND_SPECIAL_S;
        }
        UpB => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI;
            status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
        }
        DownB => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW;
            status = *FIGHTER_STATUS_KIND_SPECIAL_LW;
        }
        UpSmash => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4;
            status = *FIGHTER_STATUS_KIND_ATTACK_HI4_START;
        }
        FSmash => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4;
            status = *FIGHTER_STATUS_KIND_ATTACK_S4_START;
        }
        DSmash => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4;
            status = *FIGHTER_STATUS_KIND_ATTACK_LW4_START;
        }
        Grab => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH;
            status = *FIGHTER_STATUS_KIND_CATCH;
        }
        Jab => {
            // Prevent nair when airborne
            if !is_grounded(module_accessor) {
                return 0;
            }

            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N;
            status = *FIGHTER_STATUS_KIND_ATTACK;
        }
        Ftilt => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3;
            status = *FIGHTER_STATUS_KIND_ATTACK_S3;
        }
        Utilt => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3;
            status = *FIGHTER_STATUS_KIND_ATTACK_HI3;
        }
        Dtilt => {
            action_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3;
            status = *FIGHTER_STATUS_KIND_ATTACK_LW3;
        }
        _ => return 0,
    }

    return get_flag(module_accessor, status, action_flag);
}

unsafe fn get_aerial_flag(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    action: Action,
) -> i32 {
    let mut flag: i32 = 0;

    // If we are grounded we also need to jump
    if is_grounded(module_accessor) {
        flag += *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON;

        // Delay attack until we are airborne to get a full hop
        if MENU.full_hop == OnOff::On {
            return flag;
        }
    }

    let status = *FIGHTER_STATUS_KIND_ATTACK_AIR;

    if MENU.falling_aerials == OnOff::On && !fast_fall::is_falling(module_accessor) {
        return flag;
    }

    let action_flag: i32;
    use Action::*;

    match action {
        Nair => {
            action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_N;
        }
        Fair => {
            // For some reason the game doesn't trigger the fair correctly
            // action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_F;
            action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_N;
        }
        Bair => {
            action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_B;
        }
        UpAir => {
            // For some reason the game doesn't trigger the uair correctly
            // action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI;
            action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_N;
        }
        Dair => {
            action_flag = *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW;
        }
        _ => {
            action_flag = 0;
        }
    }

    flag |= get_flag(module_accessor, status, action_flag);

    flag
}

/**
 * Returns the flag and resets, once the action is performed
 */
unsafe fn get_flag(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    status: i32,
    action_flag: i32,
) -> i32 {
    if StatusModule::status_kind(module_accessor) == status {
        // Reset Buffer
        reset();
    }

    return action_flag;
}

pub unsafe fn perform_defensive_option() {
    reset();

    let mut suspend_shield = true;
    let action;

    match MENU.defensive_state {
        Defensive::Random => {
            let random_cmds = vec![
                Mash::Spotdodge,
                Mash::RollBack,
                Mash::RollForward,
                Mash::Attack,
            ];

            let random_cmd_index =
                app::sv_math::rand(hash40("fighter"), random_cmds.len() as i32) as usize;

            action = mash_to_action(random_cmds[random_cmd_index]);
        }
        Defensive::Roll => {
            if app::sv_math::rand(hash40("fighter"), 2) == 0 {
                action = Action::RollForward;
            } else {
                action = Action::RollBack;
            }
        }
        Defensive::Spotdodge => action = Action::Spotdodge,
        Defensive::Jab => {
            action = Action::Jab;
        }
        Defensive::Shield => {
            action = Action::Shield;
        }
        _ => return,
    }

    buffer_action(action);

    // Suspend shield hold to allow for other defensive options
    shield::suspend_shield(action);
}
