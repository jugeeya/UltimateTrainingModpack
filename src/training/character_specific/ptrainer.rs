use smash::app::{self, smashball::is_training_mode}; //lua_bind::*,
//use smash::lib::lua_const::*;
use crate::training::save_states;
use skyline::hooks::InlineCtx;

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
    let w8 = ctx.registers[8].w.as_mut();
    *w8 = pokemon_value;
}

pub fn init() {
    skyline::install_hooks!(pokemon_decide_handle,);
}