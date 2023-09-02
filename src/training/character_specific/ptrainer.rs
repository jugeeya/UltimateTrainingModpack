use crate::training::frame_counter;
use crate::training::save_states;
use once_cell::sync::Lazy;
use skyline::hooks::InlineCtx;
use smash::app::{self, lua_bind::*, smashball::is_training_mode};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::phx::Hash40;

static SWITCH_DELAY_COUNTER: Lazy<usize> =
    Lazy::new(|| frame_counter::register_counter(frame_counter::FrameCounterType::InGame));

pub unsafe fn is_switched(ptrainer_module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(ptrainer_module_accessor);
    let situ_kind = StatusModule::situation_kind(ptrainer_module_accessor);
    if status_kind == *WEAPON_PTRAINER_PTRAINER_STATUS_KIND_RUN {
        MotionModule::set_rate(ptrainer_module_accessor, 1.0);
    }
    if frame_counter::should_delay(5_u32, *SWITCH_DELAY_COUNTER) {
        // Need to wait to make sure we stop the flash effect
        return false;
    }
    // If you're trying to fix PT getting locked up, maybe try
    //  to run FUN_71000106c0 in lua2cpp_ptrainer to get her wandering correct
    // Also worth trying is figuring out how to prevent PT from entering WEAPON_PTRAINER_PTRAINER_STATUS_KIND_RUN_STOP
    //  instead of run below after that status change
    if situ_kind == SITUATION_KIND_AIR {
        StatusModule::set_situation_kind(
            ptrainer_module_accessor,
            app::SituationKind(*SITUATION_KIND_GROUND),
            true,
        );
        StatusModule::change_status_force(
            ptrainer_module_accessor,
            *WEAPON_PTRAINER_PTRAINER_STATUS_KIND_RUN,
            false,
        );
    }
    true
}

pub unsafe fn change_motion(
    ptrainer_module_accessor: &mut app::BattleObjectModuleAccessor,
    motion_kind: u64,
) {
    if app::utility::get_kind(ptrainer_module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER
        && hash40("restart") == motion_kind
        && save_states::is_loading()
    {
        MotionModule::set_rate(ptrainer_module_accessor, 1000.0);
    }
}

pub unsafe fn get_ptrainer_mball_module_accessor(
    ptrainer_module_accessor: &mut app::BattleObjectModuleAccessor,
) -> Option<&mut app::BattleObjectModuleAccessor> {
    if ArticleModule::is_exist(
        ptrainer_module_accessor,
        *WEAPON_PTRAINER_PTRAINER_GENERATE_ARTICLE_MBALL,
    ) {
        let ptrainer_masterball: *mut app::Article = ArticleModule::get_article(
            ptrainer_module_accessor,
            *WEAPON_PTRAINER_PTRAINER_GENERATE_ARTICLE_MBALL,
        );
        let ptrainer_masterball_id = Article::get_battle_object_id(ptrainer_masterball);
        return Some(&mut *app::sv_battle_object::module_accessor(
            ptrainer_masterball_id as u32,
        ));
    }
    None
}

pub unsafe fn get_ptrainer_module_accessor(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> &mut app::BattleObjectModuleAccessor {
    let ptrainer_object_id =
        LinkModule::get_parent_object_id(module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER);
    &mut *app::sv_battle_object::module_accessor(ptrainer_object_id as u32)
}

pub unsafe fn get_pokemon_module_accessor(
    ptrainer_module_accessor: *mut app::BattleObjectModuleAccessor,
) -> *mut app::BattleObjectModuleAccessor {
    let pokemon_object_id = LinkModule::get_node_object_id(
        ptrainer_module_accessor,
        *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON,
    );
    &mut *app::sv_battle_object::module_accessor(pokemon_object_id as u32)
}

pub unsafe fn handle_pokemon_effect(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    hash: Hash40,
    size: f32,
) -> f32 {
    let kind = app::utility::get_kind(module_accessor);
    if ![
        *FIGHTER_KIND_PZENIGAME,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_PLIZARDON,
        *WEAPON_KIND_PTRAINER_PTRAINER,
        *WEAPON_KIND_PTRAINER_MBALL,
        -1,
    ]
    .contains(&kind)
    {
        return size;
    }

    let is_ptrainer_switch_hash = [
        Hash40::new("sys_flying_plate"),      // for Req
        Hash40::new("ptrainer_change_light"), // for ReqOnJoint
    ]
    .contains(&hash)
        || hash.hash == 0x10e3fac8d9;

    // We never want the flying plate, and otherwise we allow outside of savestates
    if (is_ptrainer_switch_hash && save_states::is_loading())
        || Hash40::new("sys_flying_plate") == hash
    {
        // Making the size 0 prevents these effects from being displayed. Fixes Pokemon Trainer Angel Platform Effect.
        return 0.0;
    }
    size
}

pub unsafe fn handle_pokemon_sound_effect(hash: Hash40) -> Hash40 {
    let is_ptrainer_switch_sound_hash = [
        Hash40::new("se_ptrainer_change_appear"),
        Hash40::new("se_ptrainer_ball_open"),
        Hash40::new("se_ptrainer_ball_swing"),
    ]
    .contains(&hash);
    if is_ptrainer_switch_sound_hash && save_states::is_loading() {
        return Hash40::new("se_silent");
    }
    hash
}

// Choose which pokemon to switch to!
static POKEMON_DECIDE_OFFSET: usize = 0x34cdc64;

#[skyline::hook(offset = POKEMON_DECIDE_OFFSET, inline)]
unsafe fn handle_pokemon_decide(ctx: &mut InlineCtx) {
    if !is_training_mode() || !save_states::is_loading() {
        return;
    }
    let x20 = ctx.registers[20].x.as_mut();
    let fighter = *x20 as *mut u64 as *mut app::Fighter;
    let module_accessor = (*fighter).battle_object.module_accessor;
    let pokemon_value = save_states::get_state_pokemon(module_accessor);
    if pokemon_value <= 2 {
        let w8 = ctx.registers[8].w.as_mut();
        *w8 = pokemon_value;
    }
}

pub fn init() {
    skyline::install_hooks!(handle_pokemon_decide,);
}
