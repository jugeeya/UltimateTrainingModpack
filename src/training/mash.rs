use crate::common::consts::*;
use crate::common::*;
use crate::training::character_specific;
use crate::training::fast_fall;
use crate::training::shield;
use smash::app::{self, lua_bind::*};
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

    return 0;
}

pub fn handle_mash(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        if !is_operation_cpu(module_accessor) {
            return;
        }

        perform_action(module_accessor);
    }
}

unsafe fn check_buffer(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if QUEUE.len() > 0 {
        /*
         Reset when CPU is idle to prevent deadlocks
         and to reset when using the training mode reset
        */
        if should_reset(module_accessor) {
            reset();
        }

        return;
    }

    if !should_buffer(module_accessor) {
        return;
    }

    buffer_menu_mash(module_accessor);
}

fn should_reset(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if !is_idle(module_accessor) {
        return false;
    }

    let prev_status;

    unsafe {
        prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    }

    // Don't reset after teching
    if prev_status == *FIGHTER_STATUS_KIND_DOWN {
        return false;
    }

    if prev_status == *FIGHTER_STATUS_KIND_PASSIVE {
        return false;
    }

    if prev_status == *FIGHTER_STATUS_KIND_PASSIVE_FB {
        return false;
    }

    return true;
}

fn should_buffer(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    unsafe{
        if MENU.mash_in_neutral == OnOff::On {
            return true;
        }
    }

    if is_in_hitstun(module_accessor) {
        return true;
    }

    if is_in_footstool(module_accessor) {
        return true;
    }

    return false;
}

// Temp Translation
pub fn buffer_menu_mash(module_accessor: &mut app::BattleObjectModuleAccessor) -> Action {
    unsafe {
        let action;
        if MENU.mash_state == Mash::Random {
            action = get_random_action(module_accessor);
        } else {
            action = mash_to_action(MENU.mash_state);
        }
        buffer_action(action);

        action
    }
}

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

fn get_random_action(module_accessor: &mut app::BattleObjectModuleAccessor) -> Action {
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

        let random_cmd_index = get_random_int(random_cmds.len() as i32) as usize;

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
        Attack::DashAttack => DashAttack,
        Attack::Nothing => Nothing,
    }
}

unsafe fn perform_action(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    use Action::*;

    let action = get_current_buffer();

    match action {
        Airdodge => {
            let expected_status;
            let transition_flag;
            // Shield if grounded instead
            if is_grounded(module_accessor) {
                expected_status = *FIGHTER_STATUS_KIND_GUARD_ON;
                transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE;
            } else {
                expected_status = *FIGHTER_STATUS_KIND_ESCAPE_AIR;
                transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR;
            }

            return get_flag(module_accessor, expected_status, transition_flag);
        }
        Jump => {
            return update_jump_flag(module_accessor);
        }
        Spotdodge => {
            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_KIND_ESCAPE,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
            );
        }
        RollForward => {
            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_KIND_ESCAPE_F,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
            );
        }
        RollBack => {
            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_KIND_ESCAPE_B,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B,
            );
        }
        Shield => {
            /*
            Doesn't actually cause the shield, but will clear the buffer once shield is possible.
            Shield hold is performed trough shield::should_hold_shield
            */
            return get_flag(
                module_accessor,
                *FIGHTER_STATUS_KIND_GUARD_ON,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON,
            );
        }
        _ => return get_attack_flag(module_accessor, action),
    }
}

unsafe fn update_jump_flag(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    let check_flag: i32;
    let transition_flag: i32;

    if is_grounded(module_accessor) {
        check_flag = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
        transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT;
    } else if is_airborne(module_accessor) {
        check_flag = *FIGHTER_STATUS_KIND_JUMP_AERIAL;
        transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL;
    } else {
        check_flag = *FIGHTER_STATUS_KIND_JUMP;
        transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL;
    }

    return get_flag(module_accessor, check_flag, transition_flag);
}

unsafe fn get_attack_flag(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    action: Action,
) -> i32 {
    use Action::*;

    let transition_flag: i32;
    let status: i32;

    match action {
        Nair | Fair | Bair | UpAir | Dair => {
            return get_aerial_flag(module_accessor, action);
        }
        NeutralB => {
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N;
            status = *FIGHTER_STATUS_KIND_SPECIAL_N;
        }
        SideB => {
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S;
            status = *FIGHTER_STATUS_KIND_SPECIAL_S;
        }
        UpB => {
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI;
            status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
        }
        DownB => {
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW;
            status = *FIGHTER_STATUS_KIND_SPECIAL_LW;
        }
        UpSmash => {
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START;
            status = *FIGHTER_STATUS_KIND_ATTACK_HI4_START;
        }
        FSmash => {
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START;
            status = *FIGHTER_STATUS_KIND_ATTACK_S4_START;
        }
        DSmash => {
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START;
            status = *FIGHTER_STATUS_KIND_ATTACK_LW4_START;
        }
        Grab => {
            let cannot_grab = WorkModule::get_int(
                module_accessor,
                *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME,
            ) != 0;
            if cannot_grab {
                return 0;
            }

            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH;
            status = *FIGHTER_STATUS_KIND_CATCH;
        }
        Jab => {
            // Prevent nair when airborne
            if !is_grounded(module_accessor) {
                return 0;
            }

            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK;
            status = *FIGHTER_STATUS_KIND_ATTACK;
        }
        Ftilt => {
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3;
            status = *FIGHTER_STATUS_KIND_ATTACK_S3;
        }
        Utilt => {
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3;
            status = *FIGHTER_STATUS_KIND_ATTACK_HI3;
        }
        Dtilt => {
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3;
            status = *FIGHTER_STATUS_KIND_ATTACK_LW3;
        }
        DashAttack => {
            let current_status = StatusModule::status_kind(module_accessor);
            let is_dashing = current_status == *FIGHTER_STATUS_KIND_DASH;

            // Start Dash First
            if !is_dashing {
                let dash_transition = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH;
                let dash_status = *FIGHTER_STATUS_KIND_DASH;

                try_change_status(module_accessor, dash_status, dash_transition);
            }

            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH;
            status = *FIGHTER_STATUS_KIND_ATTACK_DASH;

            return get_flag(module_accessor, status, transition_flag);
        }
        _ => return 0,
    }

    return get_flag(module_accessor, status, transition_flag);
}

unsafe fn get_aerial_flag(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    action: Action,
) -> i32 {
    let mut flag: i32 = 0;

    // If we are grounded we also need to jump
    if is_grounded(module_accessor) {
        let jump_flag = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
        let transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT;
        try_change_status(module_accessor, jump_flag, transition_flag);

        // Delay attack until we are airborne to get a full hop
        if MENU.full_hop == OnOff::On {
            return flag;
        }
    }

    let status = *FIGHTER_STATUS_KIND_ATTACK_AIR;

    if MENU.falling_aerials == OnOff::On && !fast_fall::is_falling(module_accessor) {
        return flag;
    }

    let transition_flag: i32;
    use Action::*;

    /*
     * We always trigger attack and change it later into the correct aerial
     * @see get_attack_air_kind()
     */
    match action {
        Nair | Fair | Bair | UpAir | Dair => {
            transition_flag = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR;
        }
        _ => {
            transition_flag = 0;
        }
    }

    set_aerial(action);

    flag |= get_flag(module_accessor, status, transition_flag);

    flag
}

/**
 * Returns the flag and resets, once the action is performed
 */
unsafe fn get_flag(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    expected_status: i32,
    transition_flag: i32,
) -> i32 {
    // let current_status = StatusModule::prev_status_kind(module_accessor,0);
    let current_status = StatusModule::status_kind(module_accessor);
    if current_status == expected_status {
        // Reset Buffer
        reset();
    }

    // Workaround for Character specific status
    if character_specific::check_status(module_accessor, current_status, expected_status) {
        reset();
    }

    try_change_status(module_accessor, expected_status, transition_flag);

    return 0;
}

fn try_change_status(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    expected_status: i32,
    transition_flag: i32,
) -> bool {
    let allow_transition;
    unsafe {
        allow_transition = WorkModule::is_enable_transition_term(module_accessor, transition_flag);
    }

    if !allow_transition {
        return false;
    }

    unsafe {
        StatusModule::change_status_request_from_script(module_accessor, expected_status, true);
    }

    return true;
}

pub unsafe fn perform_defensive_option() {
    reset();

    let action;

    match MENU.defensive_state {
        Defensive::Random => {
            let random_cmds = vec![
                Mash::Spotdodge,
                Mash::RollBack,
                Mash::RollForward,
                Mash::Attack,
            ];

            let random_cmd_index =get_random_int(random_cmds.len() as i32) as usize;

            action = mash_to_action(random_cmds[random_cmd_index]);
        }
        Defensive::Roll => {
            if get_random_int(2) == 0 {
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
