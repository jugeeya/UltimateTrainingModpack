use smash::app::{self, lua_bind::*, smashball::is_training_mode};
use smash::lib::lua_const::*;

#[repr(C)]
struct TroopsManager {
    _x0: u64,
    max_pikmin_count: usize, // always 3
    current_pikmin_count: usize,
    pikmin_objects: *mut *mut app::BattleObject,
    pikmin: [*mut app::BattleObject; 3],
    padding_0: u64,
    padding_1: u64,
    padding_2: u64,
    padding_3: u64,
    padding_4: u64,
    padding_5: u64,
    padding_6: u64,
    padding_7: u64,
    padding_8: u64,
    held_pikmin_count: usize,
    maybe_more_pikmin_objects: *mut *mut app::BattleObject,
    held_pikmin: [*mut app::BattleObject; 3], // @ 0x90
}

#[repr(C)]
pub struct WeaponWorkModule {
    vtable: u64,
    owner: *mut app::BattleObjectModuleAccessor,
}

// Prevent Order Loss
static ACTIVATE_AUTONOMY_OFFSET: usize = 0x034b5cf0;
#[skyline::hook(offset = ACTIVATE_AUTONOMY_OFFSET)]
pub unsafe fn autonomy_handle(weapon: *mut app::Weapon, work_module: *mut WeaponWorkModule) {
    if !is_training_mode() {
        return original!()(weapon, work_module);
    }
    let pikmin_boma = (*work_module).owner;
    // If the Pikmin is in a status where we want this behavior, execute the original process
    let pikmin_status = StatusModule::status_kind(pikmin_boma);
    let is_pikmin_thrown = (*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_ATTACK_AIR
        ..=*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_ATTACK_HI4_LANDING)
        .contains(&pikmin_status);
    if is_pikmin_thrown {
        original!()(weapon, work_module)
    }
}

pub fn init() {
    skyline::install_hooks!(autonomy_handle,);
}

fn get_pikmin_prev(variation: i32) -> i32 {
    if variation > 0 {
        return variation - 1;
    }
    4
}

pub unsafe fn follow(module_accessor: &mut app::BattleObjectModuleAccessor) {
    let troops_manager = WorkModule::get_int64(module_accessor, 0x100000C0) as *mut TroopsManager;
    let following_count = (*troops_manager).current_pikmin_count;
    let held_count = (*troops_manager).held_pikmin_count;

    let mut pikmin_boid_following_vec = Vec::new();
    let mut pikmin_boid_held_vec = Vec::new();

    // First, we get the order of held pikmin, since they'll be in front if we save state during a move or grab
    for held_index in 0..held_count {
        let held_boid = (*((*troops_manager).held_pikmin[held_index])).battle_object_id;
        pikmin_boid_held_vec.push(held_boid);
    }
    // Next, we get the order of the following pikmin
    for following_index in 0..following_count {
        let following_boid = (*((*troops_manager).pikmin[following_index])).battle_object_id;
        pikmin_boid_following_vec.push(following_boid);
    }

    for pikmin_boid in pikmin_boid_following_vec {
        let pikmin_boma = app::sv_battle_object::module_accessor(pikmin_boid);
        StatusModule::change_status_request(
            pikmin_boma,
            *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_AIR_FOLLOW,
            false,
        );
    }
}

pub unsafe fn spawn_pikmin(module_accessor: &mut app::BattleObjectModuleAccessor, variation: i32) {
    WorkModule::set_int(
        module_accessor,
        get_pikmin_prev(variation),
        *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PRE_PIKMIN_VARIATION,
    );
    WorkModule::set_int(
        module_accessor,
        get_pikmin_prev(get_pikmin_prev(variation)),
        *FIGHTER_PIKMIN_INSTANCE_WORK_INT_BEFORE_PRE_PIKMIN_VARIATION,
    );
    ArticleModule::generate_article(
        module_accessor as *mut app::BattleObjectModuleAccessor,
        0,
        false,
        -1,
    );
}

pub unsafe fn get_current_pikmin(
    module_accessor: &mut app::BattleObjectModuleAccessor,
) -> [Option<i32>; 3] {
    let troops_manager = WorkModule::get_int64(module_accessor, 0x100000C0) as *mut TroopsManager;

    let following_count = (*troops_manager).current_pikmin_count;
    let held_count = (*troops_manager).held_pikmin_count;

    let mut pikmin_boid_vec = Vec::new();
    //let mut pikmin_boma_vec: [*mut app::BattleObjectModuleAccessor; 3] = [0 as *mut app::BattleObjectModuleAccessor; 3];
    let mut ordered_pikmin_variation: [Option<i32>; 3] = [None; 3];

    // First, we get the order of held pikmin, since they'll be in front if we save state during a move or grab
    for held_index in 0..held_count {
        let held_work_var = match held_index {
            0 => *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0,
            1 => *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_1,
            2 => *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_2,
            _ => {
                panic!("Pikmin Held Out of Bounds!");
            }
        };
        let held_boid = WorkModule::get_int(module_accessor, held_work_var) as u32;
        println!(", boid: {}", held_boid);
        pikmin_boid_vec.push(held_boid);
    }
    // Next, we get the order of the following pikmin
    for following_index in 0..following_count {
        let following_boid = (*((*troops_manager).pikmin[following_index])).battle_object_id;
        pikmin_boid_vec.push(following_boid);
    }
    // Now, we have all pikmin boids, and want to get their bomas (if they exist) so we can check their color
    for (idx, pikmin_boid) in pikmin_boid_vec.iter().enumerate() {
        if *pikmin_boid != *BATTLE_OBJECT_ID_INVALID as u32
            && app::sv_battle_object::is_active(*pikmin_boid)
        {
            let pikmin_boma = app::sv_battle_object::module_accessor(*pikmin_boid);
            let pikmin_variation = WorkModule::get_int(
                pikmin_boma,
                *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION,
            );
            ordered_pikmin_variation[idx] = Some(pikmin_variation);
        }
    }

    ordered_pikmin_variation
}

//#[cfg(debug_assertions)]
#[allow(dead_code)]
pub unsafe fn pretty_print(module_accessor: &mut app::BattleObjectModuleAccessor) {
    let troops_manager = WorkModule::get_int64(module_accessor, 0x100000C0) as *mut TroopsManager;

    let following_count = (*troops_manager).current_pikmin_count;
    let held_count = (*troops_manager).held_pikmin_count;

    let mut pikmin_following_boid_vec = Vec::new();
    let mut pikmin_held_boid_vec = Vec::new();

    // First, we get the order of held pikmin, since they'll be in front if we save state during a move or grab
    for held_index in 0..held_count {
        let held_boid = (*((*troops_manager).held_pikmin[held_index])).battle_object_id;
        pikmin_held_boid_vec.push(held_boid);
        print(held_boid, true);
    }
    // Next, we get the order of the following pikmin
    for following_index in 0..following_count {
        let following_boid = (*((*troops_manager).pikmin[following_index])).battle_object_id;
        pikmin_following_boid_vec.push(following_boid);
        print(following_boid, false);
    }
    println!("----------------------------------------")
}

//#[cfg(debug_assertions)]
#[allow(dead_code)]
unsafe fn print(boid: u32, held: bool) {
    if boid != *BATTLE_OBJECT_ID_INVALID as u32 && app::sv_battle_object::is_active(boid) {
        let pikmin_boma = app::sv_battle_object::module_accessor(boid);
        let pikmin_variation = WorkModule::get_int(
            pikmin_boma,
            *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION,
        );
        let pikmin_status = StatusModule::status_kind(pikmin_boma);
        let pikmin_autonomy: bool = WorkModule::is_flag(
            pikmin_boma,
            *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_AUTONOMY,
        );
        // solution:
        //WorkModule::off_flag(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_AUTONOMY);
        let owner_cond = WorkModule::get_int(
            pikmin_boma,
            *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_OWNER_CONDITION_CURRENT,
        );
        let owner_cond_follow = WorkModule::get_int(
            pikmin_boma,
            *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_OWNER_CONDITION_FOLLOW,
        );
        let owner_opt_flag_follow = WorkModule::get_int(
            pikmin_boma,
            *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_OWNER_OPTION_FLAG_FOLLOW,
        );
        println!("Color: {}, Status: {}, Held {}, Autonomy: {}, owner_cond: {}, owner_cond_follow: {}, owner_opt_flag_follow: {}",
            pikmin_variation, pikmin_status, held, pikmin_autonomy, owner_cond, owner_cond_follow, owner_opt_flag_follow
        );
    }
}
