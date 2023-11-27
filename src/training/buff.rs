use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector3f};

use crate::common::consts::*;
use crate::is_operation_cpu;
use crate::training::frame_counter;
use crate::training::handle_add_limit;

use once_cell::sync::Lazy;

static mut BUFF_REMAINING_PLAYER: i32 = 0;
static mut BUFF_REMAINING_CPU: i32 = 0;

static mut IS_BUFFING_PLAYER: bool = false;
static mut IS_BUFFING_CPU: bool = false;

static BUFF_DELAY_COUNTER: Lazy<usize> =
    Lazy::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGame));

pub unsafe fn restart_buff(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if is_operation_cpu(module_accessor) {
        IS_BUFFING_CPU = false;
        return;
    }
    IS_BUFFING_PLAYER = false;
}

pub unsafe fn start_buff(module_accessor: &mut app::BattleObjectModuleAccessor) {
    if is_operation_cpu(module_accessor) {
        IS_BUFFING_CPU = true;
        return;
    }
    IS_BUFFING_PLAYER = true;
}

pub unsafe fn is_buffing(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if is_operation_cpu(module_accessor) {
        return IS_BUFFING_CPU;
    }
    IS_BUFFING_PLAYER
}

pub unsafe fn is_buffing_any() -> bool {
    IS_BUFFING_CPU || IS_BUFFING_PLAYER
}

pub unsafe fn set_buff_rem(module_accessor: &mut app::BattleObjectModuleAccessor, new_value: i32) {
    if is_operation_cpu(module_accessor) {
        BUFF_REMAINING_CPU = new_value;
        return;
    }
    BUFF_REMAINING_PLAYER = new_value;
}

pub unsafe fn get_buff_rem(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    if is_operation_cpu(module_accessor) {
        return BUFF_REMAINING_CPU;
    }
    BUFF_REMAINING_PLAYER
}

pub unsafe fn handle_buffs(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    fighter_kind: i32,
    status: i32,
) -> bool {
    // Future Enhancements:
    // - Remove startup effects on buffs (Flash of Limit, Wii Fit's flash, Shulk's occasional Jump Art smoke, etc.)
    // - Ensure IS_BUFFING_CPU && IS_BUFFING_PLAYER are set to false on leaving training mode
    // - Deal With Arsene!/Shulk Art SFX with a different function than below to keep the reset SFX on state load
    SoundModule::stop_all_sound(module_accessor); // silences buff sfx other than KO Punch
    ControlModule::stop_rumble(module_accessor, false);
    MotionAnimcmdModule::set_sleep(module_accessor, false);
    CameraModule::stop_quake(module_accessor, *CAMERA_QUAKE_KIND_M); // stops Psyche-Up quake
    CameraModule::stop_quake(module_accessor, *CAMERA_QUAKE_KIND_S); // stops Monado Art quake

    let menu_vec = MENU.buff_state;

    if fighter_kind == *FIGHTER_KIND_BRAVE {
        return buff_hero(module_accessor, status);
    } else if fighter_kind == *FIGHTER_KIND_JACK && menu_vec.contains(&BuffOption::ARSENE) {
        return buff_joker(module_accessor);
    } else if fighter_kind == *FIGHTER_KIND_WIIFIT && menu_vec.contains(&BuffOption::BREATHING) {
        return buff_wiifit(module_accessor, status);
    } else if fighter_kind == *FIGHTER_KIND_CLOUD && menu_vec.contains(&BuffOption::LIMIT) {
        return buff_cloud(module_accessor);
    } else if fighter_kind == *FIGHTER_KIND_LITTLEMAC && menu_vec.contains(&BuffOption::KO) {
        return buff_mac(module_accessor);
    } else if fighter_kind == *FIGHTER_KIND_EDGE && menu_vec.contains(&BuffOption::WING) {
        return buff_sepiroth(module_accessor);
    } else if fighter_kind == *FIGHTER_KIND_SHULK {
        return buff_shulk(module_accessor, status);
    } else if fighter_kind == *FIGHTER_KIND_TANTAN && menu_vec.contains(&BuffOption::POWER_DRAGON) {
        return buff_minmin(module_accessor);
    } else if fighter_kind == *FIGHTER_KIND_WARIO {
        return buff_wario(module_accessor);
    }
    true
}

unsafe fn buff_hero(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    let buff_vec = MENU.buff_state.hero_buffs().to_vec();
    if !is_buffing(module_accessor) {
        // Initial set up for spells
        start_buff(module_accessor);
        set_buff_rem(module_accessor, buff_vec.len() as i32);
        // Since it's the first step of buffing, we need to set up how many buffs there are
    }
    if get_buff_rem(module_accessor) <= 0 {
        // If there are no buffs selected/left, we're done
        if frame_counter::should_delay(3_u32, *BUFF_DELAY_COUNTER) {
            // Need to wait 3 frames to make sure we stop the spell SFX, since it's a bit delayed
            return false;
        }
        return true;
    }
    buff_hero_single(module_accessor, status, buff_vec);
    false
}

unsafe fn buff_hero_single(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    status: i32,
    buff_vec: Vec<BuffOption>,
) {
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    if prev_status_kind == FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START {
        // If we just applied a buff successfully, subtract from buffs remaining
        let new_rem_value = get_buff_rem(module_accessor) - 1;
        set_buff_rem(module_accessor, new_rem_value);
    }
    let spell_index = get_buff_rem(module_accessor) - 1;
    // Used to get spell from our vector
    let spell_option = buff_vec.get(spell_index as usize);
    if spell_option.is_none() {
        // There are no spells selected, or something went wrong with making the vector
        return;
    }
    let real_spell_value = spell_option.unwrap().into_int().unwrap();
    if status != FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START {
        WorkModule::set_int(
            module_accessor,
            real_spell_value,
            *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_DECIDE_COMMAND,
        );
        StatusModule::change_status_force(
            module_accessor,
            *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START,
            true,
            // True to prevent Shielding over the spells
        );
    }
    if status == FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START {
        MotionModule::set_rate(module_accessor, 50.0);
    }
}

unsafe fn buff_cloud(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if !is_buffing(module_accessor) {
        // Only need to add to the limit gauge once
        start_buff(module_accessor);
        handle_add_limit(100.0, module_accessor, 0);
    }
    if frame_counter::should_delay(2_u32, *BUFF_DELAY_COUNTER) {
        // Need to wait 2 frames to make sure we stop the limit SFX, since it's a bit delayed
        return false;
    }
    true
}

unsafe fn buff_joker(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if !is_buffing(module_accessor) {
        // Only need to add to the rebel gauge once
        start_buff(module_accessor);
        let entry_id = app::FighterEntryID(FighterId::CPU as i32);
        // Strangely, this doesn't actually matter and works for both fighters
        app::FighterSpecializer_Jack::add_rebel_gauge(module_accessor, entry_id, 120.0);
    }
    if frame_counter::should_delay(2_u32, *BUFF_DELAY_COUNTER) {
        // Need to wait 2 frames to make sure we stop the voice call, since it's a bit delayed
        return false;
    }
    true
}

unsafe fn buff_mac(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if !is_buffing(module_accessor) {
        // Only need to set KO gauge once
        start_buff(module_accessor);
        WorkModule::set_float(
            module_accessor,
            100.0,
            *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE,
        );
    }
    if frame_counter::should_delay(2_u32, *BUFF_DELAY_COUNTER) {
        // Need to wait 2 frames to make sure we stop the KO sound, since it's a bit delayed
        return false;
    }
    true
}

unsafe fn buff_sepiroth(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if !is_buffing(module_accessor) {
        // To ensure Sephiroth gains Wing, we set flags for Sephiroth being in a Stamina Mode Sudden Death match.
        // The function that checks whether to start giving Sephiroth Wing every frame also checks for this exact
        //  scenario. We do this because inline hooking it with the current version of skyline crashes the game,
        //  likely due to the hook clobbering some of the floating point registers that we need for later.
        WorkModule::on_flag(
            module_accessor,
            *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_IS_RULE_HP,
        );
        WorkModule::on_flag(
            module_accessor,
            *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SUDDEN_DEATH,
        );
    }
    start_buff(module_accessor);
    if WorkModule::is_flag(
        module_accessor,
        *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED,
    ) {
        // Wing is activated, so we turn off these flags so future deaths don't spawn Sephiroth in with Wing
        WorkModule::off_flag(
            module_accessor,
            *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_IS_RULE_HP,
        );
        WorkModule::off_flag(
            module_accessor,
            *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SUDDEN_DEATH,
        );
        return true;
    }
    false
}

unsafe fn buff_wario(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if !is_buffing(module_accessor) {
        let waft_level: BuffOption = MENU.buff_state.wario_buffs().get_random();
        let waft_count_secs = match waft_level {
            BuffOption::WAFT_MINI => WorkModule::get_param_float(
                module_accessor,
                hash40("param_special_lw"),
                hash40("gass_middle_time"),
            ) as i32,
            BuffOption::WAFT_HALF => WorkModule::get_param_float(
                module_accessor,
                hash40("param_special_lw"),
                hash40("gass_large_time"),
            ) as i32,
            BuffOption::WAFT_FULL => WorkModule::get_param_float(
                module_accessor,
                hash40("param_special_lw"),
                hash40("gass_max_time"),
            ) as i32,
            _ => return true,
        };
        let waft_count_frames = waft_count_secs * 60;
        WorkModule::set_int(
            module_accessor,
            waft_count_frames,
            *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_COUNT,
        );
        WorkModule::set_int(
            module_accessor,
            waft_level.into_int().unwrap(),
            *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL,
        );
    }
    start_buff(module_accessor);
    true
}

unsafe fn buff_shulk(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    let current_art = MENU.buff_state.shulk_buffs().get_random();
    if current_art == BuffOption::empty() {
        // No Monado Arts selected in the buff menu, so we don't need to buff
        return true;
    }
    start_buff(module_accessor);
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    if prev_status_kind == FIGHTER_SHULK_STATUS_KIND_SPECIAL_N_ACTION {
        if frame_counter::should_delay(3_u32, *BUFF_DELAY_COUNTER) {
            // Need to continue to be buffing to make sure we stop "JUMP!" voice line
            return false;
        }
        return true;
    }
    if status != FIGHTER_SHULK_STATUS_KIND_SPECIAL_N_ACTION {
        WorkModule::set_int(
            module_accessor,
            current_art.into_int().unwrap(),
            *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE_SELECT,
        );
        WorkModule::set_int(
            module_accessor,
            29,
            *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_SELECT_TIMER,
        );
        WorkModule::on_flag(
            module_accessor,
            *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_SELECT,
        );
        StatusModule::change_status_force(
            module_accessor,
            *FIGHTER_SHULK_STATUS_KIND_SPECIAL_N_ACTION,
            true,
        );
    } else {
        MotionModule::set_rate(module_accessor, 40.0);
    }
    false
}

unsafe fn buff_wiifit(module_accessor: &mut app::BattleObjectModuleAccessor, status: i32) -> bool {
    if !is_buffing(module_accessor) {
        start_buff(module_accessor);
    }
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    if prev_status_kind == FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS {
        return true;
    }
    if status != FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS {
        StatusModule::change_status_force(
            module_accessor,
            *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS,
            false,
        );
    } else {
        MotionModule::set_rate(module_accessor, 40.0);
    }
    false
}

unsafe fn buff_minmin(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    // Handle all of Min Min's effect setup, copied from 710122a340
    VisibilityModule::set_status_default(
        module_accessor,
        Hash40 { hash: 0x59a6ef56c },
        Hash40 { hash: 0x9b7cb3e40 },
    );
    VisibilityModule::set_status_default(
        module_accessor,
        Hash40 { hash: 0xa9ffaf181 },
        Hash40 { hash: 0xef190b4e8 },
    );
    let article_spiralleft =
        ArticleModule::get_article(module_accessor, *FIGHTER_TANTAN_GENERATE_ARTICLE_SPIRALLEFT);
    if !article_spiralleft.is_null() {
        VisibilityModule::set_status_default(
            module_accessor,
            Hash40 { hash: 0x6ec1f4d21 },
            Hash40 { hash: 0xa9aba8db6 },
        );
        VisibilityModule::set(
            module_accessor,
            Hash40 { hash: 0x59a6ef56c },
            Hash40 { hash: 0xadd214353 },
        );
    }
    let arm_l_big_frame = WorkModule::get_param_int(module_accessor, 0xdf05c072b, 0xf4fd45d48);
    WorkModule::set_int(
        module_accessor,
        arm_l_big_frame,
        *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME,
    );
    WorkModule::on_flag(
        module_accessor,
        *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L,
    );
    let mut reinforce_l_effect_handle_l = WorkModule::get_int(
        module_accessor,
        *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_REINFORCE_L_EFFECT_HANDLE_L,
    );
    if reinforce_l_effect_handle_l != 0 {
        EffectModule::kill(
            module_accessor,
            reinforce_l_effect_handle_l as u32,
            false,
            true,
        );
    }
    let pos_and_rot = Vector3f {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let scale_vec = Vector3f {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    reinforce_l_effect_handle_l = EffectModule::req(
        module_accessor,
        Hash40 { hash: 0x12600df9d4 },
        &pos_and_rot,
        &pos_and_rot,
        1.0,
        0,
        -1,
        false,
        0,
    ) as i32;
    EffectModule::set_scale(
        module_accessor,
        reinforce_l_effect_handle_l as u32,
        &scale_vec,
    );
    WorkModule::set_int(
        module_accessor,
        reinforce_l_effect_handle_l,
        *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_REINFORCE_L_EFFECT_HANDLE_L,
    );
    MotionModule::add_motion_partial(
        module_accessor,
        *FIGHTER_TANTAN_MOTION_PART_SET_KIND_DRAGON,
        Hash40 { hash: 0xc86296416 },
        0.0,
        1.0,
        false,
        false,
        0.0,
        true,
        true,
        false,
    );
    true
}
