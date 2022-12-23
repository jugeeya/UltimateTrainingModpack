use crate::common::consts::OnOff;
use crate::common::consts::*;
use crate::common::*;
use crate::training::attack_angle;
use crate::training::character_specific;
use crate::training::fast_fall;
use crate::training::frame_counter;
use crate::training::full_hop;
use crate::training::input_record;
use crate::training::shield;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

const DISTANCE_CLOSE_THRESHOLD: f32 = 16.0;
const DISTANCE_MID_THRESHOLD: f32 = 37.0;
const DISTANCE_FAR_THRESHOLD: f32 = 64.0;

static mut CURRENT_AERIAL: Action = Action::NAIR;
static mut QUEUE: Vec<Action> = vec![];

static mut FALLING_AERIAL: bool = false;

static mut AERIAL_DELAY_COUNTER: usize = 0;
static mut AERIAL_DELAY: u32 = 0;

pub fn buffer_action(action: Action) {
    unsafe {
        if !QUEUE.is_empty() {
            return;
        }
    }

    if action == Action::empty() {
        return;
    }
   
    // We want to allow for triggering a mash to end playback for neutral playbacks, but not for SDI/disadv playbacks
    unsafe { 
         // exit playback if we want to perform mash actions out of it
        if MENU.playback_mash == OnOff::On && !input_record::is_recording() {
            input_record::stop_playback();
        }
        // if we don't want to leave playback on mash actions, then don't perform the mash
        if input_record::is_playback() {
            return;
        }
    }

    attack_angle::roll_direction();

    roll_aerial_delay(action);

    unsafe {
        QUEUE.insert(0, action);
        buffer_follow_up();
    }
}

pub fn buffer_follow_up() {
    let action;

    unsafe {
        action = MENU.follow_up.get_random();
    }

    if action == Action::empty() {
        return;
    }

    roll_aerial_delay(action);

    unsafe {
        QUEUE.insert(0, action);
    }
}

pub fn get_current_buffer() -> Action {
    unsafe {
        if QUEUE.is_empty() {
            return Action::empty();
        }

        *QUEUE.last().unwrap()
    }
}

fn reset() {
    unsafe {
        QUEUE.pop();
    }

    shield::suspend_shield(get_current_buffer());

    unsafe {
        frame_counter::full_reset(AERIAL_DELAY_COUNTER);
        AERIAL_DELAY = 0;
    }
}

pub fn full_reset() {
    unsafe {
        while !QUEUE.is_empty() {
            reset();
        }
    }
}

pub fn clear_queue() {
    unsafe { QUEUE.clear() }
}

pub fn set_aerial(attack: Action) {
    unsafe {
        CURRENT_AERIAL = attack;
    }
}

pub unsafe fn get_attack_air_kind(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> Option<i32> {
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

    if !is_operation_cpu(module_accessor) {
        return 0;
    }

    check_buffer(module_accessor);

    perform_action(module_accessor)
}

unsafe fn check_buffer(module_accessor: &mut app::BattleObjectModuleAccessor) {
    // Different situations mean we want to change our buffered option, so we check what to buffer every frame
    let buffered_action = get_buffered_action(module_accessor);
    match buffered_action {
        Some(action) => {
            full_reset(); 
            // we need to clear the queue when adding a mash to the queue, but not necessarily a follow-up.
            // We need to clear the queue since it'll be trying to buffer that action until it's completed, but now we want
            //  different things to happen.
            buffer_menu_mash(action);
        },
        None => return,
    }
}

unsafe fn get_buffered_action(module_accessor: &mut app::BattleObjectModuleAccessor) -> Option<Action> {
    let fighter_distance = get_fighter_distance();
    if MENU.mash_triggers.contains(MashTrigger::HIT) && is_in_hitstun(module_accessor) {
        if MENU.hitstun_override == Action::empty() {
            Some(MENU.mash_state.get_random())
        } else {
            Some(MENU.hitstun_override.get_random())
        }
    } else if MENU.mash_triggers.contains(MashTrigger::PARRY) && is_in_parry(module_accessor) {
        if MENU.parry_override == Action::empty() {
            Some(MENU.mash_state.get_random())
        } else {
            Some(MENU.parry_override.get_random())
        }
    } else if MENU.mash_triggers.contains(MashTrigger::BLOCK) && is_in_shieldstun(module_accessor) {
        if MENU.block_override == Action::empty() {
            Some(MENU.mash_state.get_random())
        } else {
            Some(MENU.block_override.get_random())
        }
    } else if MENU.mash_triggers.contains(MashTrigger::FOOTSTOOL) && is_in_footstool(module_accessor) {
        if MENU.footstool_override == Action::empty() {
            Some(MENU.mash_state.get_random())
        } else {
            Some(MENU.footstool_override.get_random())
        }
    } else if MENU.mash_triggers.contains(MashTrigger::TRUMP) && is_in_ledgetrump(module_accessor) {
        if MENU.trump_override == Action::empty() {
            Some(MENU.mash_state.get_random())
        } else {
            Some(MENU.trump_override.get_random())
        }
    } else if MENU.mash_triggers.contains(MashTrigger::LANDING) && is_in_landing(module_accessor) {
        if MENU.landing_override == Action::empty() {
            Some(MENU.mash_state.get_random())
        } else {
            Some(MENU.landing_override.get_random())
        }
    }
    // LEDGE handled in ledge.rs
    // TECH handled in tech.rs
    // MISTECH handled in tech.rs
    else if (MENU.mash_triggers.contains(MashTrigger::TUMBLE) && is_in_tumble(module_accessor))
        || (MENU.mash_triggers.contains(MashTrigger::CLATTER) && is_in_clatter(module_accessor)) 
        || (MENU.mash_triggers.contains(MashTrigger::GROUNDED) && is_grounded(module_accessor))
        || (MENU.mash_triggers.contains(MashTrigger::AIRBORNE) && is_airborne(module_accessor))
        || (MENU.mash_triggers.contains(MashTrigger::DISTANCE_CLOSE) && fighter_distance < DISTANCE_CLOSE_THRESHOLD)
        || (MENU.mash_triggers.contains(MashTrigger::DISTANCE_MID) && fighter_distance < DISTANCE_MID_THRESHOLD)
        || (MENU.mash_triggers.contains(MashTrigger::DISTANCE_FAR) && fighter_distance < DISTANCE_FAR_THRESHOLD)
        || MENU.mash_triggers.contains(MashTrigger::ALWAYS)
    {
        Some(MENU.mash_state.get_random())
    } else {
        None
    }
}

fn buffer_menu_mash(action: Action) {
    unsafe {
        buffer_action(action);
        full_hop::roll_full_hop();
        fast_fall::roll_fast_fall();
        FALLING_AERIAL = MENU.falling_aerials.get_random().into_bool();
    }
}

pub fn external_buffer_menu_mash(action: Action) {
    full_reset();
    buffer_menu_mash(action);
}

unsafe fn perform_action(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    let action = get_current_buffer();
    match action {
        Action::AIR_DODGE => {
            let (expected_status, command_flag) = if is_grounded(module_accessor) {
                // Shield if grounded instead
                /*
                Doesn't actually cause the shield, but will clear the buffer once shield is possible.
                Shield hold is performed through shield::should_hold_shield and request_shield
                */
                (
                    *FIGHTER_STATUS_KIND_GUARD_ON,
                    *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE,
                )
            } else {
                (
                    *FIGHTER_STATUS_KIND_ESCAPE_AIR,
                    *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE,
                )
            };

            get_flag(module_accessor, expected_status, command_flag)
        }
        Action::JUMP => update_jump_flag(module_accessor),
        Action::SPOT_DODGE => get_flag(
            module_accessor,
            *FIGHTER_STATUS_KIND_ESCAPE,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE,
        ),
        Action::ROLL_F => get_flag(
            module_accessor,
            *FIGHTER_STATUS_KIND_ESCAPE_F,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F,
        ),
        Action::ROLL_B => get_flag(
            module_accessor,
            *FIGHTER_STATUS_KIND_ESCAPE_B,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B,
        ),
        Action::SHIELD => {
            if !is_grounded(module_accessor) {
                return 0;
            }
            /*
            Doesn't actually cause the shield, but will clear the buffer once shield is possible.
            Shield hold is performed through shield::should_hold_shield and request_shield
            */
            get_flag(
                module_accessor,
                *FIGHTER_STATUS_KIND_GUARD_ON,
                *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE,
            )
        }
        Action::DASH => {
            let dash_transition = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH;
            let dash_status = *FIGHTER_STATUS_KIND_DASH;

            try_change_status(module_accessor, dash_status, dash_transition);

            get_flag(module_accessor, *FIGHTER_STATUS_KIND_DASH, 0)
        }
        _ => get_attack_flag(module_accessor, action),
    }
}

pub fn request_shield(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    match get_current_buffer() {
        Action::SHIELD => true,
        Action::AIR_DODGE => is_grounded(module_accessor),
        _ => false,
    }
}

unsafe fn update_jump_flag(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    let check_flag = if is_grounded(module_accessor) {
        *FIGHTER_STATUS_KIND_JUMP_SQUAT
    } else if is_airborne(module_accessor) {
        *FIGHTER_STATUS_KIND_JUMP_AERIAL
    } else {
        *FIGHTER_STATUS_KIND_JUMP
    };
    let command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON;

    get_flag(module_accessor, check_flag, command_flag)
}

unsafe fn get_attack_flag(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    action: Action,
) -> i32 {
    let command_flag: i32;
    let status: i32;

    match action {
        Action::NAIR | Action::FAIR | Action::BAIR | Action::UAIR | Action::DAIR => {
            return get_aerial_flag(module_accessor, action);
        }
        Action::NEUTRAL_B => {
            command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N;
            status = *FIGHTER_STATUS_KIND_SPECIAL_N;
        }
        Action::SIDE_B => {
            command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S;
            status = *FIGHTER_STATUS_KIND_SPECIAL_S;
        }
        Action::UP_B => {
            command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI;
            status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
        }
        Action::DOWN_B => {
            command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW;
            status = *FIGHTER_STATUS_KIND_SPECIAL_LW;
        }
        Action::U_SMASH => {
            command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4;
            status = *FIGHTER_STATUS_KIND_ATTACK_HI4_START;
        }
        Action::F_SMASH => {
            command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4;
            status = *FIGHTER_STATUS_KIND_ATTACK_S4_START;
        }
        Action::D_SMASH => {
            command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4;
            status = *FIGHTER_STATUS_KIND_ATTACK_LW4_START;
        }
        Action::GRAB => {
            let cannot_grab = WorkModule::get_int(
                module_accessor,
                *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME,
            ) != 0;
            if cannot_grab {
                return 0;
            }

            command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH;
            status = *FIGHTER_STATUS_KIND_CATCH;
        }
        Action::JAB => {
            // Prevent nair when airborne
            if !is_grounded(module_accessor) {
                return 0;
            }

            command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N;
            status = *FIGHTER_STATUS_KIND_ATTACK;
        }
        Action::F_TILT => {
            command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3;
            status = *FIGHTER_STATUS_KIND_ATTACK_S3;
        }
        Action::U_TILT => {
            command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3;
            status = *FIGHTER_STATUS_KIND_ATTACK_HI3;
        }
        Action::D_TILT => {
            command_flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3;
            status = *FIGHTER_STATUS_KIND_ATTACK_LW3;
        }
        // TODO: Make it work, without being 1 frame late
        Action::DASH_ATTACK => {
            let current_status = StatusModule::status_kind(module_accessor);
            let dash_status = *FIGHTER_STATUS_KIND_DASH;
            let is_dashing = current_status == dash_status;

            // Start Dash First
            if !is_dashing {
                let dash_transition = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH;

                try_change_status(module_accessor, dash_status, dash_transition);
                return 0;
            }

            status = *FIGHTER_STATUS_KIND_ATTACK_DASH;

            let transition = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH;
            try_change_status(module_accessor, status, transition);

            //@TODO find out how to properly reset, since the status just returns FIGHTER_STATUS_KIND_DASH

            return 0;
        }
        _ => return 0,
    }

    get_flag(module_accessor, status, command_flag)
}

unsafe fn get_aerial_flag(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    action: Action,
) -> i32 {
    let mut flag: i32 = 0;

    // If we are grounded we also need to jump
    if is_grounded(module_accessor) {
        // let jump_flag = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
        // let jump_transition = *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT;
        // try_change_status(module_accessor, jump_flag, jump_transition);
        flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON;

        // Delay attack until we are airborne to get a full hop
        if full_hop::should_full_hop() {
            return flag;
        }
    }

    let status = *FIGHTER_STATUS_KIND_ATTACK_AIR;

    if FALLING_AERIAL && !fast_fall::is_falling(module_accessor) {
        return flag;
    }

    if should_delay_aerial(module_accessor) {
        return flag;
    }

    /*
     * We always trigger attack and change it later into the correct aerial
     * @see get_attack_air_kind()
     */
    let command_flag: i32 = match action {
        Action::NAIR | Action::FAIR | Action::BAIR | Action::UAIR | Action::DAIR => {
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N
        }
        _ => 0,
    };

    set_aerial(action);

    flag |= get_flag(module_accessor, status, command_flag);

    flag
}

pub fn init() {
    unsafe {
        AERIAL_DELAY_COUNTER = frame_counter::register_counter();
    }
}

fn roll_aerial_delay(action: Action) {
    if !shield::is_aerial(action) {
        return;
    }
    unsafe {
        AERIAL_DELAY = MENU.aerial_delay.get_random().into_delay();
    }
}

fn should_delay_aerial(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        if AERIAL_DELAY == 0 {
            return false;
        }

        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            return false;
        }

        if !WorkModule::is_enable_transition_term(
            module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR,
        ) {
            return true;
        }

        frame_counter::should_delay(AERIAL_DELAY, AERIAL_DELAY_COUNTER)
    }
}

/**
 * Returns the flag and resets, once the action is performed
 */
unsafe fn get_flag(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    expected_status: i32,
    command_flag: i32,
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

    command_flag
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

    true
}
