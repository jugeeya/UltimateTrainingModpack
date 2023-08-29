use smash::app::{self, lua_bind::*, smashball::is_training_mode};
use smash::lib::lua_const::*;
use crate::training::save_states;
use skyline::hooks::InlineCtx;
use smash::phx::Hash40;

pub unsafe fn get_ptrainer_module_accessor(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> &mut app::BattleObjectModuleAccessor {
    let ptrainer_object_id =
        LinkModule::get_parent_object_id(module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER);
    &mut *app::sv_battle_object::module_accessor(ptrainer_object_id as u32)
}

pub unsafe fn check_effect_pokemon_state(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    hash: Hash40,
    size: f32,
) -> f32 {
    let kind = app::utility::get_kind(module_accessor);
    if ![*FIGHTER_KIND_PZENIGAME,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_PLIZARDON,
        *WEAPON_KIND_PTRAINER_PTRAINER,
        *WEAPON_KIND_PTRAINER_MBALL,
    ].contains(&kind) {
        return size;
    }
      
    let is_ptrainer_switch_hash = [
        Hash40::new("sys_flying_plate"), // for Req
        Hash40::new("ptrainer_change_light"), // for ReqOnJoint
    ].contains(&hash) || hash.hash == 0x10e3fac8d9;

    // TODO: check to make sure this is only during save state
    if is_ptrainer_switch_hash {
        // Making the size 0 prevents these effects from being displayed. Fixes Pokemon Trainer Angel Platform Effect.
        return 0.0;
    }
    size
}

pub unsafe fn sound_effect_pokemon_state(
    module_accessor: *mut app::BattleObjectModuleAccessor,
    hash: Hash40,
) -> Hash40 {
    // TODO: properly check for pokemon not being in down b status
    // Supress PT SFX on switch
    let is_ptrainer_switch_sound_hash = [
        Hash40::new("se_ptrainer_change_appear"),
        Hash40::new("se_ptrainer_ball_open"),
        Hash40::new("se_ptrainer_ball_swing"),
    ].contains(&hash);
    if is_ptrainer_switch_sound_hash && save_states::is_loading() {
        // if StatusModule::status_kind(module_accessor) == FIGHTER_STATUS_KIND_WAIT {
        //     my_hash = Hash40::new("se_silent");
        // } 
        return Hash40::new("se_silent");
    }
    hash
}

// Choose which pokemon to switch to!
static POKEMON_DECIDE_OFFSET: usize = 0x34cdc64;

#[skyline::hook(offset = POKEMON_DECIDE_OFFSET, inline)]
unsafe fn pokemon_decide_handle(ctx: &mut InlineCtx) {
    if !is_training_mode() || !save_states::is_loading() {
        return;
    }
    let x20 = ctx.registers[20].x.as_mut();
    let fighter = *x20 as *mut u64 as *mut app::Fighter;
    let module_accessor = (*fighter).battle_object.module_accessor;
    let pokemon_value = save_states::get_state_pokemon(module_accessor);
    if 0 <= pokemon_value <= 2 {
        let w8 = ctx.registers[8].w.as_mut();
        *w8 = pokemon_value;
    }
}

pub fn init() {
    skyline::install_hooks!(pokemon_decide_handle,);
}

