use smash::app::{self, lua_bind::*, smashball::is_training_mode};
use smash::lib::lua_const::*;
use crate::training::buff;
use skyline::hooks::InlineCtx;

// On Hit Function for Min Min, we call it to force power dragon
static ON_HIT_OFFSET: usize = 0x034b5cf0;
#[skyline::from_offset(ON_HIT_OFFSET)]
pub fn minmin_on_hit(ulong: u64, fighter: *mut app::Fighter, long: i64 ) -> bool;

// Enable Power Dragon
static POWER_DRAGON_OFFSET: usize = 0x122a398;

// Override check in on hit function for minmin to give her power dragon
#[skyline::hook(offset = POWER_DRAGON_OFFSET, inline)]
unsafe fn handle_enable_power_dragon(ctx: &mut InlineCtx) {
    let w0 = ctx.registers[0].w.as_mut();
    let x19 = ctx.registers[19].x.as_ref();
    // the Fighter * is moved to x19 at 710122a36c
    let fighter = (*x19) as *mut app::Fighter;
    let module_accessor = (*fighter).battle_object.module_accessor;
    let should_power_up = buff::is_buffing(&mut *module_accessor);
    if !is_training_mode() {
        return;
    }
    if should_power_up {
        *w0 = 1;
    }
}

pub fn init() {
    skyline::install_hooks!(handle_enable_power_dragon,);
}